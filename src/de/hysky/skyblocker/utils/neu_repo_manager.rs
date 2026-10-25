//! Ported from de.hysky.skyblocker.utils.NEURepoManager by fabric2seer.
//!
//! # The repository download (replaces JGit)
//!
//! Java keeps `config/skyblocker/item-repo` as a shallow JGit clone of the NotEnoughUpdates-REPO and updates it with
//! `stashCreate` / `fetch +refs/heads/master:refs/remotes/origin/master --depth 1` / `reset --hard` /
//! `stashApply` / `stashDrop`. JGit is not ported; this port reproduces the same EFFECT over HTTP, against the same
//! remote and branch:
//!
//! * the remote's `master` commit is asked of the GitHub API (`GET /repos/NotEnoughUpdates/NotEnoughUpdates-REPO/
//!   commits/master` with `Accept: application/vnd.github.sha`) - Java's `fetch`;
//! * that commit's source archive (`https://github.com/NotEnoughUpdates/NotEnoughUpdates-REPO/archive/<sha>.zip`)
//!   is downloaded and extracted - the archive holds exactly the working tree a clone checks out - into a staging
//!   directory beside `item-repo`, with the archive's top folder stripped, so `item-repo` ends up with the same
//!   files at the same paths as Java's clone (`item-repo/items/*.json`, `item-repo/constants/*.json`, ...);
//! * the commit is recorded in `item-repo/.git/skyblocker-archive-commit` (the clone's `.git` directory is where Java
//!   keeps its HEAD; nothing else is put there), and the staged tree replaces `item-repo` - Java's `reset --hard`;
//! * on the next start, a recorded commit equal to the remote's means nothing is downloaded (Java's fetch of an
//!   up-to-date ref); a different one re-downloads; a missing or unreadable record is Java's
//!   `RepositoryNotFoundException` ("Local NEU Repository not found or corrupted, downloading new one").
//!
//! What has no archive equivalent is the stash: Java re-applies a user's local edits of the repository files on top
//! of the update (`ContentMergeStrategy.THEIRS`); an archive update replaces the tree, so local edits are dropped.
//!
//! The zip is read with the game's own `java.util.zip.ZipFile` port (`mcrust::java::util::zip::zip_file`), and the
//! HTTP goes through the game's `java.net.http` transport (WinHTTP, redirects followed), so the mod adds no
//! dependency.
//!
//! # Threads
//!
//! The load runs on a worker (`supplyAsync`); the repository objects are thread-safe (`LazyLock` + locks). The
//! after-load tasks Java runs on virtual threads read and write the mod's render-thread statics
//! (`compat::MainThread`), so here they run on the render thread (`compat::execute`) once `REPO_LOADING` completes -
//! the same ordering Java's `thenRunAsync` chain gives, on the one thread those statics allow.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::constants;
use crate::de::hysky::skyblocker::utils::file_utils::FileUtils;
use crate::entry::skyblocker_mod;
use crate::io::github::moulberry::repo::data::item_overlays::{ItemOverlayFile, ItemOverlays};
use crate::io::github::moulberry::repo::data::neu_item::NEUItem;
use crate::io::github::moulberry::repo::data::neu_recipe::NEURecipe;
use crate::io::github::moulberry::repo::neu_constants::NEUConstants;
use crate::io::github::moulberry::repo::neu_recipe_cache::{NEURecipeCache, RecipeMap};
use crate::io::github::moulberry::repo::neu_repo_file::NEURepoFile;
use crate::io::github::moulberry::repo::neu_repository::NEURepository;
use crate::io::github::moulberry::repo::neu_repository_exception::NEURepositoryException;
use mcrust::java::util::concurrent::completable_future::{CompletableFuture, Executor};
use mcrust::net::minecraft::chat_formatting::{self, ChatFormatting};
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::component::Component as _;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::world::entity::player::player::Player;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{LazyLock, Mutex, RwLock};

/// java: private static final String REMOTE_REPO_URL = "https://github.com/NotEnoughUpdates/NotEnoughUpdates-REPO.git";
pub const REMOTE_REPO_URL: &str = "https://github.com/NotEnoughUpdates/NotEnoughUpdates-REPO.git";
/// The `owner/name` of `REMOTE_REPO_URL`, for the GitHub API and archive URLs.
const REMOTE_REPO_SLUG: &str = "NotEnoughUpdates/NotEnoughUpdates-REPO";
/// java: `.setBranchesToClone(List.of("refs/heads/master")).setBranch("refs/heads/master")`
const REMOTE_BRANCH: &str = "master";
/// Where the archive's commit is recorded inside the repository directory (see the module doc).
const COMMIT_RECORD: &str = ".git/skyblocker-archive-commit";

/// java: private static final Path LOCAL_REPO_DIR = SkyblockerMod.CONFIG_DIR.resolve("item-repo");
pub static LOCAL_REPO_DIR: LazyLock<PathBuf> = LazyLock::new(|| skyblocker_mod::CONFIG_DIR.get().join("item-repo"));
/// java: private static CompletableFuture<Void> REPO_LOADING = loadRepository().thenAccept(Consumers.nop());
///
/// Java's class initialiser starts the load; here the first reader does (`NEURepoManager.init` runs
/// `runAsyncAfterLoad`, which reads it, at mod initialisation - the same moment Java's `@Init` triggers `<clinit>`).
pub static REPO_LOADING: LazyLock<Mutex<CompletableFuture<()>>> =
    LazyLock::new(|| Mutex::new(NEURepoManager::load_repository().then_apply(|_| ())));
/// java: private static final NEURepository NEU_REPO = NEURepository.of(LOCAL_REPO_DIR);
pub static NEU_REPO: LazyLock<Arc<NEURepository>> = LazyLock::new(|| NEURepository::of(LOCAL_REPO_DIR.clone()));
/// java: private static final NEURecipeCache RECIPE_CACHE = NEURecipeCache.forRepo(NEU_REPO);
pub static RECIPE_CACHE: LazyLock<Arc<NEURecipeCache>> = LazyLock::new(|| NEURecipeCache::for_repo(&NEU_REPO));
/// java: private static final ItemOverlays STACK_OVERLAYS = ItemOverlays.forRepo(NEU_REPO);
pub static STACK_OVERLAYS: LazyLock<Arc<ItemOverlays>> = LazyLock::new(|| ItemOverlays::for_repo(&NEU_REPO));
/// java: private static final List<Runnable> afterLoadTasks = new CopyOnWriteArrayList<>();
pub static AFTER_LOAD_TASKS: compat::MainThread<Vec<Arc<dyn Fn()>>> = compat::MainThread::new(|| Vec::new());
/// The `runAsyncAfterLoad` tasks whose first run is still waiting on `REPO_LOADING` (Java keeps them in the future
/// chain itself): the task and the future `runAsyncAfterLoad` answered, by id.
static PENDING_AFTER_LOAD_TASKS: compat::MainThread<HashMap<u64, (Arc<dyn Fn()>, CompletableFuture<()>)>> =
    compat::MainThread::new(|| HashMap::new());
static NEXT_PENDING_ID: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
/// java: private static Multimap<@Nullable String, @Nullable NEUItem> nameToNEUItem = HashMultimap.create();
pub static NAME_TO_NEU_ITEM: LazyLock<RwLock<HashMap<String, Vec<Arc<NEUItem>>>>> = LazyLock::new(|| RwLock::new(HashMap::new()));

/// `SkyblockerMod.VIRTUAL_THREAD_EXECUTOR` as a `CompletableFuture` executor (a new thread per task).
fn virtual_thread_executor() -> Executor {
    Arc::new(|task: Box<dyn FnOnce() + Send>| compat::executor(task))
}

/// Why a repository acquisition failed - the JGit exception class Java's `catch` clauses tell apart.
enum RepoError {
    /// java: TransportException - the remote could not be reached.
    Transport(String),
    /// java: GitAPIException | RepositoryNotFoundException - the local repository is missing or corrupted.
    NotFoundOrCorrupted(String),
    /// java: Exception - anything else.
    Other(String),
}

/// java: public class NEURepoManager
#[derive(Clone)]
pub struct NEURepoManager {}

impl NEURepoManager {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// Adds command to update the repository manually from ingame.
    ///
    /// java: @Init public static void init()
    pub fn init() {
        client_command_registration_callback::EVENT.register(Arc::new(
            |dispatcher: &mut client_command_registration_callback::ClientCommandDispatcher| {
                // java: dispatcher.register(ClientCommands.literal(SkyblockerMod.NAMESPACE)
                //       .then(ClientCommands.literal("updateRepository").executes(context -> {
                //           deleteAndDownloadRepository(context.getSource().getPlayer()); return Command.SINGLE_SUCCESS; })));
                use mcrust::com::mojang::brigadier::builder::argument_builder::ArgumentBuilder as _;
                use mcrust::com::mojang::brigadier::context::command_context::CommandContext;
                type Source = client_command_registration_callback::FabricClientCommandSource;
                let mut root = client_command_registration_callback::literal(skyblocker_mod::NAMESPACE);
                let mut update_repository = client_command_registration_callback::literal("updateRepository");
                update_repository.executes(Arc::new(|_: &CommandContext<Source>| {
                    // `context.getSource().getPlayer()` is the client player (commands only run in a world)
                    if mcrust::mcrust::modding::game::with_player(|player| NEURepoManager::delete_and_download_repository(&*player)).is_none() {
                        NEURepoManager::delete_and_download_repository_player(false);
                    }
                    Ok(1)
                }));
                root.then(&update_repository);
                dispatcher.dispatcher().register(&root);
            },
        ));
        // Loads the NEUItem name cache after the repository is loaded.
        NEURepoManager::run_async_after_load(Arc::new(move || { NEURepoManager::load_name_to_neu_item_map(); }));
    }

    /// java: public static boolean isLoading()
    pub fn is_loading() -> bool {
        return !(REPO_LOADING.lock().unwrap_or_else(|p| p.into_inner()).is_done());
    }

    /// java: private static CompletableFuture<Boolean> loadRepository()
    pub fn load_repository() -> CompletableFuture<bool> {
        // The loader thread sends `Http.USER_AGENT`, whose initialiser reads the mod's metadata through a render-thread
        // static (`SkyblockerMod.SKYBLOCKER_MOD`); it is built here, on the thread that starts the first load.
        LazyLock::force(&crate::de::hysky::skyblocker::utils::http::USER_AGENT);
        CompletableFuture::supply_async(|| Ok(NEURepoManager::load_repository_body()), compat::executor).then_apply_async(
            |success: &bool| {
                let success: bool = *success;
                // java: afterLoadTasks.forEach(task -> CompletableFuture.runAsync(task, SkyblockerMod.VIRTUAL_THREAD_EXECUTOR).exceptionally(e -> {
                //       LOGGER.error("[Skyblocker NEU Repo] Encountered unknown exception while running after load task", e); return null; }));
                // (on the render thread - see the module doc)
                crate::compat::execute(|| NEURepoManager::run_after_load_tasks());
                return success;
            },
            virtual_thread_executor(),
        )
    }

    /// The `supplyAsync` body of `loadRepository`.
    fn load_repository_body() -> bool {
        let mut success: bool = true;
        // java: try { if (Files.isDirectory(LOCAL_REPO_DIR)) { ..fetch/reset.. } else { ..clone.. } }
        let result: Result<(), RepoError> =
            match std::panic::catch_unwind(|| if LOCAL_REPO_DIR.is_dir() { NEURepoManager::update_repository() } else { NEURepoManager::clone_repository() }) {
                Ok(result) => result,
                Err(panic) => Err(RepoError::Other(crate::io::github::moulberry::repo::neu_repository::gson::panic_message(&panic))),
            };
        match result {
            Ok(()) => {}
            // java: catch (TransportException e)
            Err(RepoError::Transport(e)) => {
                logger().error(&format!(
                    "[Skyblocker NEU Repo] Transport operation failed. Most likely unable to connect to the remote NEU repo on github {}",
                    e
                ));
                success = false;
            }
            // java: catch (GitAPIException | RepositoryNotFoundException e)
            Err(RepoError::NotFoundOrCorrupted(e)) => {
                logger().warn(&format!("[Skyblocker NEU Repo] Local NEU Repository not found or corrupted, downloading new one {}", e));
                // java: client.execute(() -> Scheduler.INSTANCE.schedule(() -> deleteAndDownloadRepositoryInternal(client.player), 1));
                crate::compat::execute(|| {
                    let __r = crate::de::hysky::skyblocker::utils::scheduler::scheduler::INSTANCE.get_mut();
                    let __f = compat::dispatch(__r, |__t| __t.schedule());
                    __f(
                        __r,
                        Arc::new(|| {
                            let has_player = mcrust::mcrust::modding::game::with_player(|_| ()).is_some();
                            NEURepoManager::delete_and_download_repository_player(has_player);
                        }),
                        1,
                    );
                });
                success = false;
            }
            // java: catch (Exception e)
            Err(RepoError::Other(e)) => {
                logger().error(&format!("[Skyblocker NEU Repo] Encountered unknown exception while downloading NEU Repository {}", e));
                success = false;
            }
        }
        // java: try { NEU_REPO.reload(); } catch (Exception e) { LOGGER.error(..); success = false; }
        // (`<clinit>` has registered RECIPE_CACHE and STACK_OVERLAYS with NEU_REPO by now in Java; force them first)
        LazyLock::force(&RECIPE_CACHE);
        LazyLock::force(&STACK_OVERLAYS);
        let reload: Result<Result<(), NEURepositoryException>, _> = std::panic::catch_unwind(|| NEU_REPO.reload());
        match reload {
            Ok(Ok(())) => {}
            Ok(Err(e)) => {
                logger().error(&format!("[Skyblocker NEU Repo] Encountered unknown exception while loading NEU Repository {}", e));
                success = false;
            }
            Err(panic) => {
                logger().error(&format!(
                    "[Skyblocker NEU Repo] Encountered unknown exception while loading NEU Repository {}",
                    crate::io::github::moulberry::repo::neu_repository::gson::panic_message(&panic)
                ));
                success = false;
            }
        }
        return success;
    }

    /// The `Files.isDirectory(LOCAL_REPO_DIR)` branch: Java's `Git.open` + stash + fetch + `reset --hard`.
    fn update_repository() -> Result<(), RepoError> {
        // java: Git localRepo = Git.open(LOCAL_REPO_DIR.toFile()); - RepositoryNotFoundException when it is no repository
        let local_commit: String = NEURepoManager::read_local_commit()
            .ok_or_else(|| RepoError::NotFoundOrCorrupted(format!("repository not found: {}", LOCAL_REPO_DIR.display())))?;
        // java: localRepo.fetch().setRefSpecs("+refs/heads/master:refs/remotes/origin/master").setThin(true).setDepth(1).call();
        let remote_commit: String = NEURepoManager::fetch_remote_commit()?;
        // java: Ref ref = localRepo.reset().setRef("refs/remotes/origin/master").setMode(ResetCommand.ResetType.HARD).call();
        if remote_commit != local_commit {
            NEURepoManager::download_commit(&remote_commit)?;
        }
        // java: LOGGER.info("[Skyblocker NEU Repo] NEU Repository was updated to {}", ref.getObjectId().getName());
        logger().info(&format!("[Skyblocker NEU Repo] NEU Repository was updated to {}", remote_commit));
        Ok(())
    }

    /// The `else` branch: Java's `Git.cloneRepository().setURI(REMOTE_REPO_URL).setDirectory(LOCAL_REPO_DIR)
    /// .setBranchesToClone(List.of("refs/heads/master")).setBranch("refs/heads/master").setDepth(1).call().close();`
    fn clone_repository() -> Result<(), RepoError> {
        let remote_commit: String = NEURepoManager::fetch_remote_commit()?;
        NEURepoManager::download_commit(&remote_commit)?;
        logger().info("[Skyblocker NEU Repo] NEU Repository Downloaded");
        Ok(())
    }

    /// The commit the local repository is at (`HEAD`), `None` when there is no readable record.
    fn read_local_commit() -> Option<String> {
        let commit = std::fs::read_to_string(LOCAL_REPO_DIR.join(COMMIT_RECORD)).ok()?;
        let commit = commit.trim().to_string();
        if commit.len() == 40 && commit.bytes().all(|b| b.is_ascii_hexdigit()) { Some(commit) } else { None }
    }

    /// The remote `refs/heads/master` commit (the fetch).
    fn fetch_remote_commit() -> Result<String, RepoError> {
        let url = format!("https://api.github.com/repos/{}/commits/{}", REMOTE_REPO_SLUG, REMOTE_BRANCH);
        let user_agent: &str = crate::de::hysky::skyblocker::utils::http::USER_AGENT.as_str();
        let response = mcrust::java::net::http::http_client::request_with_timeouts(
            "GET",
            &url,
            &[("Accept", "application/vnd.github.sha"), ("User-Agent", user_agent)],
            None,
            Some((15_000, 60_000)),
        )
        .map_err(|e| RepoError::Transport(format!("{}: {}", url, e)))?;
        if !response.is_success() {
            return Err(RepoError::Transport(format!("{}: HTTP {}", url, response.status)));
        }
        let commit: String = String::from_utf8_lossy(&response.bytes).trim().to_string();
        if commit.len() != 40 || !commit.bytes().all(|b| b.is_ascii_hexdigit()) {
            return Err(RepoError::Transport(format!("{}: not a commit id: {}", url, commit)));
        }
        Ok(commit)
    }

    /// Downloads the tree of `commit` and makes it the content of `LOCAL_REPO_DIR` (the checkout).
    fn download_commit(commit: &str) -> Result<(), RepoError> {
        let config_dir: &Path = LOCAL_REPO_DIR.parent().unwrap_or(Path::new("."));
        std::fs::create_dir_all(config_dir).map_err(|e| RepoError::Other(format!("{}: {}", config_dir.display(), e)))?;
        let archive_path: PathBuf = config_dir.join("item-repo.download.zip");
        let staging_dir: PathBuf = config_dir.join("item-repo.download");

        // The archive, streamed to disk.
        let url = format!("https://github.com/{}/archive/{}.zip", REMOTE_REPO_SLUG, commit);
        let user_agent: &str = crate::de::hysky::skyblocker::utils::http::USER_AGENT.as_str();
        let mut stream = mcrust::java::net::http::http_client::open("GET", &url, &[("User-Agent", user_agent)], None, Some((15_000, 120_000)))
            .map_err(|e| RepoError::Transport(format!("{}: {}", url, e)))?;
        if !stream.is_success() {
            return Err(RepoError::Transport(format!("{}: HTTP {}", url, stream.status)));
        }
        {
            let mut file = std::fs::File::create(&archive_path).map_err(|e| RepoError::Other(format!("{}: {}", archive_path.display(), e)))?;
            std::io::copy(&mut stream, &mut file).map_err(|e| RepoError::Transport(format!("{}: {}", url, e)))?;
        }
        drop(stream);

        // The checkout, staged beside the repository.
        let extracted = NEURepoManager::extract_archive(&archive_path, &staging_dir, commit);
        let _ = std::fs::remove_file(&archive_path);
        if let Err(e) = extracted {
            FileUtils::recursive_delete(&staging_dir);
            return Err(RepoError::Other(e));
        }

        // `reset --hard`: the staged tree replaces the repository.
        if LOCAL_REPO_DIR.exists() {
            FileUtils::recursive_delete(&LOCAL_REPO_DIR);
        }
        std::fs::rename(&staging_dir, &*LOCAL_REPO_DIR)
            .map_err(|e| RepoError::Other(format!("{} -> {}: {}", staging_dir.display(), LOCAL_REPO_DIR.display(), e)))?;
        Ok(())
    }

    /// Extracts the GitHub archive into `target` (its single top folder stripped) and records `commit`.
    fn extract_archive(archive_path: &Path, target: &Path, commit: &str) -> Result<(), String> {
        use mcrust::java::util::zip::zip_file::ZipFile;
        if target.exists() {
            FileUtils::recursive_delete(&target.to_path_buf());
        }
        std::fs::create_dir_all(target).map_err(|e| format!("{}: {}", target.display(), e))?;
        let zip = ZipFile::open(archive_path).map_err(|e| format!("{}: {}", archive_path.display(), e))?;
        for entry in zip.entries() {
            let name: &str = entry.get_name();
            // "NotEnoughUpdates-REPO-<sha>/items/X.json" -> "items/X.json"
            let Some((_, relative)) = name.split_once('/') else {
                continue;
            };
            if relative.is_empty() {
                continue;
            }
            // Only plain relative components (no `..`, no absolute paths) are written.
            let mut out: PathBuf = target.to_path_buf();
            for part in relative.split('/').filter(|p| !p.is_empty()) {
                if part == ".." || part == "." || part.contains(':') || part.contains('\\') {
                    return Err(format!("unsafe archive entry {}", name));
                }
                out.push(part);
            }
            if entry.is_directory() {
                std::fs::create_dir_all(&out).map_err(|e| format!("{}: {}", out.display(), e))?;
            } else {
                if let Some(parent) = out.parent() {
                    std::fs::create_dir_all(parent).map_err(|e| format!("{}: {}", parent.display(), e))?;
                }
                let bytes = zip.get_input_stream(entry).map_err(|e| format!("{}: {}", name, e))?;
                std::fs::write(&out, bytes).map_err(|e| format!("{}: {}", out.display(), e))?;
            }
        }
        let record: PathBuf = target.join(COMMIT_RECORD);
        if let Some(parent) = record.parent() {
            std::fs::create_dir_all(parent).map_err(|e| format!("{}: {}", parent.display(), e))?;
        }
        std::fs::write(&record, format!("{}\n", commit)).map_err(|e| format!("{}: {}", record.display(), e))?;
        Ok(())
    }

    /// Caches NEUItems by display name using the NEU repository.
    ///
    /// java: private static void loadNameToNEUItemMap()
    pub fn load_name_to_neu_item_map() {
        // java: nameToNEUItem = NEU_REPO.getItems().getItems().values().stream()
        //       .collect(Multimaps.toMultimap(item -> ChatFormatting.stripFormatting(item.getDisplayName()), Function.identity(), HashMultimap::create));
        let mut name_to_neu_item: HashMap<String, Vec<Arc<NEUItem>>> = HashMap::new();
        for item in NEU_REPO.get_items().get_items().values() {
            let entry = name_to_neu_item.entry(chat_formatting::strip_formatting(item.get_display_name())).or_default();
            // a HashMultimap holds each (key, value) pair once
            if !entry.iter().any(|i| Arc::ptr_eq(i, item)) {
                entry.push(item.clone());
            }
        }
        *NAME_TO_NEU_ITEM.write().unwrap_or_else(|p| p.into_inner()) = name_to_neu_item;
    }

    /// Differs from `delete_and_download_repository_internal` in that this method checks if the repository is
    /// currently loading to prevent spamming the command.
    ///
    /// java: private static void deleteAndDownloadRepository(Player player)
    pub fn delete_and_download_repository(player: &dyn Player) {
        if NEURepoManager::is_loading() {
            NEURepoManager::send_message(Some(player), &component::translatable("skyblocker.updateRepository.loading").with_style_format(ChatFormatting::Red));
            return;
        }
        NEURepoManager::delete_and_download_repository_internal(Some(player));
    }

    /// `deleteAndDownloadRepository` for a caller that has no player object to hand (the command source without a
    /// player): `has_player` is whether Java's `player` would be non-null.
    fn delete_and_download_repository_player(has_player: bool) {
        if has_player {
            if mcrust::mcrust::modding::game::with_player(|player| NEURepoManager::delete_and_download_repository(&*player)).is_some() {
                return;
            }
        }
        if NEURepoManager::is_loading() {
            NEURepoManager::send_message(None, &component::translatable("skyblocker.updateRepository.loading").with_style_format(ChatFormatting::Red));
            return;
        }
        NEURepoManager::delete_and_download_repository_internal(None);
    }

    /// java: private static void deleteAndDownloadRepositoryInternal(@Nullable Player player)
    pub fn delete_and_download_repository_internal(player: Option<&dyn Player>) {
        // Only whether `player` is null crosses to the worker (`sendMessage` looks the player up on the render thread).
        let has_player: bool = player.is_some();
        // java: Function<Runnable, CompletableFuture<Void>> runner = isLoading() ? REPO_LOADING::thenRunAsync
        //       : task -> CompletableFuture.runAsync(task, SkyblockerMod.VIRTUAL_THREAD_EXECUTOR);
        let is_loading: bool = NEURepoManager::is_loading();
        let task = move || {
            NEURepoManager::send_message_async(has_player, "skyblocker.updateRepository.start", ChatFormatting::Aqua);
            // java: try { .. } catch (Exception e) { LOGGER.error(..); sendMessage(player, ..error..); }
            let result = std::panic::catch_unwind(|| {
                FileUtils::recursive_delete(&LOCAL_REPO_DIR);
                NEURepoManager::send_message_async(has_player, "skyblocker.updateRepository.deleted", ChatFormatting::Aqua);
                // java: loadRepository().join() ? ..success.. : ..failed..
                let loaded: bool = NEURepoManager::load_repository().join_blocking().unwrap_or_else(|e| panic!("CompletionException: {}", e));
                if loaded {
                    NEURepoManager::send_message_async(has_player, "skyblocker.updateRepository.success", ChatFormatting::Green);
                } else {
                    NEURepoManager::send_message_async(has_player, "skyblocker.updateRepository.failed", ChatFormatting::Red);
                }
            });
            if let Err(panic) = result {
                logger().error(&format!(
                    "[Skyblocker NEU Repo] Encountered unknown exception while deleting the NEU repo {}",
                    crate::io::github::moulberry::repo::neu_repository::gson::panic_message(&panic)
                ));
                NEURepoManager::send_message_async(has_player, "skyblocker.updateRepository.error", ChatFormatting::Red);
            }
        };
        // java: REPO_LOADING = runner.apply(..);
        let mut repo_loading = REPO_LOADING.lock().unwrap_or_else(|p| p.into_inner());
        let next: CompletableFuture<()> = if is_loading {
            repo_loading.then_run_async(task, virtual_thread_executor())
        } else {
            CompletableFuture::<()>::run_async(task, compat::executor)
        };
        *repo_loading = next;
    }

    /// `sendMessage(player, Component.translatable(key).withStyle(color))` from a worker: the component is built on
    /// the render thread, where `sendMessage` runs it.
    fn send_message_async(has_player: bool, key: &'static str, color: ChatFormatting) {
        crate::compat::execute(move || {
            let text: MutableComponent = component::translatable(key).with_style_format(color);
            if has_player
                && mcrust::mcrust::modding::game::with_player(|player| NEURepoManager::send_message(Some(&*player), &text)).is_some()
            {
                return;
            }
            NEURepoManager::send_message(None, &text);
        });
    }

    /// java: private static void sendMessage(@Nullable Player player, Component text)
    pub fn send_message(mut player: Option<&dyn Player>, text: &MutableComponent) {
        let Some(mut player) = player else {
            logger().info(&format!("[Skyblocker NEU Repo] {}", text.get_string()));
            return;
        };
        // java: Minecraft.getInstance().execute(() -> player.sendSystemMessage(Constants.PREFIX.get().append(text)));
        let text: String = (constants::PREFIX.get_mut())().append(text.clone()).to_string();
        crate::compat::execute(move || { script_host::send_feedback(&text); });
    }

    /// Runs the given runnable after the NEU repo is initialized: a completable future of the given runnable.
    ///
    /// java: public static CompletableFuture<Void> runAsyncAfterLoad(Runnable runnable)
    pub fn run_async_after_load(runnable: Arc<dyn Fn()>) -> Option<CompletableFuture<()>> {
        // java: return REPO_LOADING.thenRunAsync(runnable, SkyblockerMod.VIRTUAL_THREAD_EXECUTOR).exceptionally(e -> {
        //       LOGGER.error("[Skyblocker NEU Repo] Encountered unknown exception while running after load task", e); return null;
        //       }).thenRun(() -> afterLoadTasks.add(runnable)); // Add to the list after so it doesn't get executed twice.
        let future: CompletableFuture<()> = CompletableFuture::new();
        let id: u64 = NEXT_PENDING_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        PENDING_AFTER_LOAD_TASKS.get_mut().insert(id, (runnable, future.clone()));
        let repo_loading: CompletableFuture<()> = REPO_LOADING.lock().unwrap_or_else(|p| p.into_inner()).clone();
        repo_loading.when_complete(Box::new(move |result: Result<&(), &str>| {
            let completed_normally: bool = result.is_ok();
            crate::compat::execute(move || NEURepoManager::run_pending_after_load_task(id, completed_normally));
        }));
        Some(future)
    }

    /// The `thenRunAsync(runnable).exceptionally(..).thenRun(afterLoadTasks.add(runnable))` chain of one
    /// `runAsyncAfterLoad` call, on the render thread.
    fn run_pending_after_load_task(id: u64, repo_loading_completed_normally: bool) {
        let Some((runnable, future)) = PENDING_AFTER_LOAD_TASKS.get_mut().remove(&id) else {
            return;
        };
        if repo_loading_completed_normally {
            if let Err(panic) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| runnable())) {
                logger().error(&format!(
                    "[Skyblocker NEU Repo] Encountered unknown exception while running after load task {}",
                    crate::io::github::moulberry::repo::neu_repository::gson::panic_message(&panic)
                ));
            }
        } else {
            logger().error("[Skyblocker NEU Repo] Encountered unknown exception while running after load task");
        }
        AFTER_LOAD_TASKS.get_mut().push(runnable);
        future.complete(());
    }

    /// The `afterLoadTasks.forEach(task -> CompletableFuture.runAsync(task, ..).exceptionally(..))` of a (re)load,
    /// on the render thread.
    fn run_after_load_tasks() {
        let tasks: Vec<Arc<dyn Fn()>> = AFTER_LOAD_TASKS.get_mut().clone();
        for task in tasks {
            if let Err(panic) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| task())) {
                logger().error(&format!(
                    "[Skyblocker NEU Repo] Encountered unknown exception while running after load task {}",
                    crate::io::github::moulberry::repo::neu_repository::gson::panic_message(&panic)
                ));
            }
        }
    }

    /// java: public static void forEachItem(Consumer<NEUItem> consumer)
    pub fn for_each_item(consumer: Arc<dyn Fn(Arc<NEUItem>)>) {
        // java: NEU_REPO.getItems().getItems().values().forEach(consumer);
        for item in NEU_REPO.get_items().get_items().values() {
            consumer(item.clone());
        }
    }

    /// java: public static @Nullable NEUItem getItemByNeuId(String neuId)
    pub fn get_item_by_neu_id(neu_id: &str) -> Option<Arc<NEUItem>> {
        // java: return NEU_REPO.getItems().getItemBySkyblockId(neuId);
        NEU_REPO.get_items().get_item_by_skyblock_id(neu_id)
    }

    /// Gets the `NEUItem` by display name.
    ///
    /// java: public static Collection<@Nullable NEUItem> getItemByName(String displayName)
    pub fn get_item_by_name(display_name: &str) -> Vec<Arc<NEUItem>> {
        // java: return nameToNEUItem.get(displayName); - a Multimap answers an empty collection for an absent key
        NAME_TO_NEU_ITEM.read().unwrap_or_else(|p| p.into_inner()).get(display_name).cloned().unwrap_or_default()
    }

    /// java: public static NEUConstants getConstants()
    pub fn get_constants() -> Arc<NEUConstants> {
        NEU_REPO.get_constants().clone()
    }

    /// java: public static @Nullable NEURepoFile file(String path)
    pub fn file(path: &str) -> Option<NEURepoFile> {
        NEU_REPO.file(path)
    }

    /// java: public static Stream<NEURepoFile> tree(String path) throws NEURepositoryException
    pub fn tree(path: &str) -> Result<std::vec::IntoIter<NEURepoFile>, NEURepositoryException> {
        NEU_REPO.tree(path)
    }

    /// java: public static Map<@NEUId String, Set<NEURecipe>> getRecipes()
    pub fn get_recipes() -> Arc<RecipeMap> {
        RECIPE_CACHE.get_recipes()
    }

    /// java: public static Map<@NEUId String, Set<NEURecipe>> getUsages()
    pub fn get_usages() -> Arc<RecipeMap> {
        RECIPE_CACHE.get_usages()
    }

    /// java: public static Map<@NEUId String, ItemOverlayFile> getStackOverlays(int maxSupportedDataVersion)
    pub fn get_stack_overlays(max_supported_data_version: i32) -> HashMap<String, ItemOverlayFile> {
        STACK_OVERLAYS.get_most_up_to_date_compatible_with(max_supported_data_version)
    }
}

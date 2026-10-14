//! Ported from de.hysky.skyblocker.utils.NEURepoManager by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::constants;
use crate::entry::skyblocker_mod;
use mcrust::java::util::concurrent::completable_future::CompletableFuture;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::world::entity::player::player::Player;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final String REMOTE_REPO_URL = "https://github.com/NotEnoughUpdates/NotEnoughUpdates-REPO.git";
pub const REMOTE_REPO_URL: &str = "https://github.com/NotEnoughUpdates/NotEnoughUpdates-REPO.git";
/// java: private static final Path LOCAL_REPO_DIR = SkyblockerMod.CONFIG_DIR.resolve("item-repo");
pub static LOCAL_REPO_DIR: compat::MainThread<std::path::PathBuf> = compat::MainThread::new(|| skyblocker_mod::CONFIG_DIR.get_mut().join("item-repo"));
// TODO(fabric2seer): static call Consumers.nop(..)
//   java: private static CompletableFuture<Void> REPO_LOADING = loadRepository().thenAccept(Consumers.nop());
/// java: private static CompletableFuture<Void> REPO_LOADING = loadRepository().thenAccept(Consumers.nop());
///
/// rust: port gap - `loadRepository()` needs JGit (clone / fetch / reset of the NEU repo) and the NEU repo parser
/// (`NEURepository.reload`), neither of which is in the crate (GAPS_L.md): the load never finishes, so the future
/// stays pending - `isLoading()` answers true and the after-load tasks wait, as they wait in Java until it completes.
pub static REPO_LOADING: compat::MainThread<mcrust::java::util::concurrent::completable_future::CompletableFuture<()>> = compat::MainThread::new(|| CompletableFuture::new());
// TODO(fabric2seer): static call NEURepository.of(..)
//   java: private static final NEURepository NEU_REPO = NEURepository.of(LOCAL_REPO_DIR);
/// java: private static final NEURepository NEU_REPO = NEURepository.of(LOCAL_REPO_DIR);
pub static NEU_REPO: compat::MainThread<crate::unresolved::NEURepository> = compat::MainThread::new(|| crate::unresolved::NEURepository);
// TODO(fabric2seer): static call NEURecipeCache.forRepo(..)
//   java: private static final NEURecipeCache RECIPE_CACHE = NEURecipeCache.forRepo(NEU_REPO);
/// java: private static final NEURecipeCache RECIPE_CACHE = NEURecipeCache.forRepo(NEU_REPO);
pub static RECIPE_CACHE: compat::MainThread<crate::unresolved::NEURecipeCache> = compat::MainThread::new(|| crate::unresolved::NEURecipeCache);
// TODO(fabric2seer): static call ItemOverlays.forRepo(..)
//   java: private static final ItemOverlays STACK_OVERLAYS = ItemOverlays.forRepo(NEU_REPO);
/// java: private static final ItemOverlays STACK_OVERLAYS = ItemOverlays.forRepo(NEU_REPO);
pub static STACK_OVERLAYS: compat::MainThread<crate::unresolved::ItemOverlays> = compat::MainThread::new(|| crate::unresolved::ItemOverlays);
/// java: private static final List<Runnable> afterLoadTasks = new CopyOnWriteArrayList<>();
pub static AFTER_LOAD_TASKS: compat::MainThread<Vec<Arc<dyn Fn()>>> = compat::MainThread::new(|| Vec::new());
// TODO(fabric2seer): static call HashMultimap.create(..)
//   java: private static Multimap<@Nullable String, @Nullable NEUItem> nameToNEUItem = HashMultimap.create();
/// java: private static Multimap<@Nullable String, @Nullable NEUItem> nameToNEUItem = HashMultimap.create();
pub static NAME_TO_NEU_ITEM: compat::MainThread<crate::unresolved::Multimap> = compat::MainThread::new(|| crate::unresolved::Multimap);

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

    /// java: @Init public static void init()
    pub fn init() {
        client_command_registration_callback::EVENT.register(Arc::new(
            |dispatcher: &mut client_command_registration_callback::ClientCommandDispatcher| {
                // TODO(fabric2seer): a command tree this tool cannot read (arguments, redirects, requires) - SeerClient commands receive the words after the name in `ctx.args`
                //   java: dispatcher.register(..);
            },
        ));
        NEURepoManager::run_async_after_load(Arc::new(move || { NEURepoManager::load_name_to_neu_item_map(); }));
    }

    /// java: public static boolean isLoading()
    pub fn is_loading() -> bool {
        return !(REPO_LOADING.get_mut().is_done());
    }

    /// java: private static CompletableFuture<Boolean> loadRepository()
    pub fn load_repository() -> Option<mcrust::java::util::concurrent::completable_future::CompletableFuture<bool>> {
        return Some((CompletableFuture::supply_async(move || Ok((|| {
            let client = compat::minecraft();
            let mut success: bool = true;
            // java: try {..} catch (TransportException e) catch (GitAPIException | RepositoryNotFoundException e) catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
            let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                if LOCAL_REPO_DIR.get_mut().is_dir() {
                    {
                        // TODO(fabric2seer): static call Git.open(..)
                        //   java: Git localRepo = Git.open(..);
                        let mut local_repo: crate::unresolved::Git = todo!("fabric2seer: the initialiser of `localRepo` (TODO above)");
                        // TODO(fabric2seer): `.stashCreate(..)` on a Git
                        //   java: boolean stashed = localRepo.stashCreate().call() != null;
                        let mut stashed: bool = todo!("fabric2seer: the initialiser of `stashed` (TODO above)");
                        // TODO(fabric2seer): `.fetch(..)` on a Git
                        //   java: localRepo.fetch()
                        //   java: 								.setRefSpecs("+refs/heads/master:refs/remotes/origin/master")
                        //   java: 								.setThin(true)
                        //   java: 								.setDepth(1)
                        //   java: 								.call();
                        // TODO(fabric2seer): `.reset(..)` on a Git
                        //   java: Ref ref = localRepo.reset()
                        //   java: 								.setRef("refs/remotes/origin/master")
                        //   java: 								.setMode(ResetCommand.ResetType.HARD)
                        //   java: 								.call();
                        let mut r#ref: crate::unresolved::Ref = todo!("fabric2seer: the initialiser of `ref` (TODO above)");
                        if stashed {
                            // TODO(fabric2seer): `.stashApply(..)` on a Git
                            //   java: localRepo.stashApply()
                            //   java: 									.setContentMergeStrategy(ContentMergeStrategy.THEIRS)
                            //   java: 									.call();
                            // TODO(fabric2seer): `.stashDrop(..)` on a Git
                            //   java: localRepo.stashDrop().call();
                            logger().info("[Skyblocker NEU Repo] Auto stash has been applied to the NEU Repository");
                        }
                        // TODO(fabric2seer): `.getObjectId(..)` on a Ref
                        //   java: LOGGER.info("[Skyblocker NEU Repo] NEU Repository was updated to {}", ref.getObjectId().getName());
                    }
                } else {
                    // TODO(fabric2seer): static call Git.cloneRepository(..)
                    //   java: Git.cloneRepository()
                    //   java: 							.setURI(REMOTE_REPO_URL)
                    //   java: 							.setDirectory(NEURepoManager.LOCAL_REPO_DIR.toFile())
                    //   java: 							.setBranchesToClone(List.of("refs/heads/master"))
                    //   java: 							.setBranch("refs/heads/master")
                    //   java: 							.setDepth(1)
                    //   java: 							.call().close();
                    logger().info("[Skyblocker NEU Repo] NEU Repository Downloaded");
                }
            }));
            if let Err(e) = __result {
                logger().error(&format!("[Skyblocker NEU Repo] Transport operation failed. Most likely unable to connect to the remote NEU repo on github"));
                success = false;
            }
            // TODO(fabric2seer): the second `catch (GitAPIException | RepositoryNotFoundException e)` clause (merged into the first)
            //   java: {
            //   java: 				LOGGER.warn("[Skyblocker NEU Repo] Local NEU Repository not found or corrupted, downloading new one", e);
            //   java: 				client.execute(() ->
            //   java: 						Scheduler.INSTANCE.schedule(() -> deleteAndDownloadRepositoryInternal(client.player), 1)
            //   java: 				);
            //   java: 				success = false;
            //   java: 			}
            // TODO(fabric2seer): the second `catch (Exception e)` clause (merged into the first)
            //   java: {
            //   java: 				LOGGER.error("[Skyblocker NEU Repo] Encountered unknown exception while downloading NEU Repository", e);
            //   java: 				success = false;
            //   java: 			}
            // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
            let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                // TODO(fabric2seer): `.reload(..)` on a NEURepository
                //   java: NEU_REPO.reload();
            }));
            if let Err(e) = __result {
                logger().error(&format!("[Skyblocker NEU Repo] Encountered unknown exception while loading NEU Repository"));
                success = false;
            }
            return success;
        })()), compat::executor)).then_apply_async(|success: &bool| {
            let success: bool = *success;
            for task in AFTER_LOAD_TASKS.get_mut().iter() {
                // TODO(fabric2seer): CompletableFuture.runAsync of a non-lambda
                //   java: CompletableFuture.runAsync(..).exceptionally(..);
            }
            return success;
        }, std::sync::Arc::new(|__t: Box<dyn FnOnce() + Send>| compat::executor(__t))));
    }

    /// java: private static void loadNameToNEUItemMap()
    pub fn load_name_to_neu_item_map() {
        // TODO(fabric2seer): `.getItems(..)` on a NEURepository
        //   java: nameToNEUItem = NEU_REPO.getItems()
        //   java: 				.getItems()
        //   java: 				.values()
        //   java: 				.stream()
        //   java: 				.collect(Multimaps.toMultimap(item -> ChatFormatting.stripFormatting(item.getDisplayName()), Function.identity(), HashMultimap::create));
    }

    /// java: private static void deleteAndDownloadRepository(Player player)
    pub fn delete_and_download_repository(player: &dyn Player) {
        if NEURepoManager::is_loading() {
            NEURepoManager::send_message(Some(player), &component::translatable("skyblocker.updateRepository.loading").with_style_format(ChatFormatting::Red));
            return;
        }
        NEURepoManager::delete_and_download_repository_internal(Some(player));
    }

    /// java: private static void deleteAndDownloadRepositoryInternal(@Nullable Player player)
    pub fn delete_and_download_repository_internal(mut player: Option<&dyn Player>) {
        // TODO(fabric2seer): CompletableFuture.runAsync of a non-lambda
        //   java: Function<Runnable, CompletableFuture<Void>> runner = isLoading() ? REPO_LOADING::thenRunAsync : task -> CompletableFuture.runAsync(task, SkyblockerMod.VIRTUAL_THREAD_EXECUTOR);
        let mut runner: Arc<dyn Fn(Arc<dyn Fn()>) -> mcrust::java::util::concurrent::completable_future::CompletableFuture<()>> = todo!("fabric2seer: the initialiser of `runner` (TODO above)");
        // TODO(fabric2seer): a lambda / method reference where this call does not take one
        //   java: REPO_LOADING = runner.apply(() -> {
        //   java: 			sendMessage(player, Component.translatable("skyblocker.updateRepository.start").withStyle(ChatFormatting.AQUA));
        //   java: 			try {
        //   java: 				FileUtils.recursiveDelete(NEURepoManager.LOCAL_REPO_DIR);
        //   java: 				sendMessage(player, Component.translatable("skyblocker.updateRepository.deleted").withStyle(ChatFormatting.AQUA));
        //   java: 				sendMessage(player, loadRepository().join() ? Component.translatable("skyblocker.updateRepository.success").withStyle(ChatFormatting.GREEN) : Component.translatable("skyblocker.updateRepository.failed").withStyle(ChatFormatting.RED));
        //   java: 			} catch (Exception e) {
        //   java: 				LOGGER.error("[Skyblocker NEU Repo] Encountered unknown exception while deleting the NEU repo", e);
        //   java: 				sendMessage(player, Component.translatable("skyblocker.updateRepository.error").withStyle(ChatFormatting.RED));
        //   java: 			}
        //   java: 		});
    }

    /// java: private static void sendMessage(@Nullable Player player, Component text)
    pub fn send_message(mut player: Option<&dyn Player>, text: &MutableComponent) {
        let Some(mut player) = player else {
            logger().info(&format!("[Skyblocker NEU Repo] {}", text.get_string()));
            return;
        };
        // java: Minecraft.getInstance().execute(() -> player.sendSystemMessage(Constants.PREFIX.get().append(text)));
        let text: MutableComponent = text.clone();
        crate::compat::execute(move || { script_host::send_feedback(&(constants::PREFIX.get_mut())().append(text.clone()).to_string()); });
    }

    /// java: public static CompletableFuture<Void> runAsyncAfterLoad(Runnable runnable)
    pub fn run_async_after_load(runnable: Arc<dyn Fn()>) -> Option<mcrust::java::util::concurrent::completable_future::CompletableFuture<()>> {
        // java: return REPO_LOADING.thenRunAsync(runnable, SkyblockerMod.VIRTUAL_THREAD_EXECUTOR)
        //   .exceptionally(e -> { LOGGER.error(..); return null; }).thenRun(() -> afterLoadTasks.add(runnable));
        // rust: port gap - every stage hangs off REPO_LOADING, which cannot complete here (the NEU repo load is a port
        // gap, see `REPO_LOADING`), so the chain Java builds would stay pending: the task neither runs nor joins
        // `afterLoadTasks`, and the answer is a future that is not done (GAPS_L.md).
        let _ = runnable;
        return Some(CompletableFuture::new());
    }

    /// java: public static void forEachItem(Consumer<NEUItem> consumer)
    pub fn for_each_item(consumer: Arc<dyn Fn(crate::unresolved::NEUItem)>) {
        // TODO(fabric2seer): `.getItems(..)` on a NEURepository
        //   java: NEU_REPO.getItems().getItems().values().forEach(consumer);
    }

    /// java: public static @Nullable NEUItem getItemByNeuId(String neuId)
    pub fn get_item_by_neu_id(neu_id: &str) -> Option<crate::unresolved::NEUItem> {
        // java: return NEU_REPO.getItems().getItemBySkyblockId(neuId);
        // rust: port gap - the NEU repo is never loaded (see `REPO_LOADING`): no item has the id
        let _ = neu_id;
        None
    }

    /// java: public static Collection<@Nullable NEUItem> getItemByName(String displayName)
    pub fn get_item_by_name(display_name: &str) -> Vec<Option<crate::unresolved::NEUItem>> {
        // java: return nameToNEUItem.get(displayName); - a Multimap answers an empty collection for an absent key
        // rust: port gap - the map is filled by `loadNameToNEUItemMap` after a repo load, which cannot happen here
        let _ = display_name;
        Vec::new()
    }

    /// java: public static NEUConstants getConstants()
    pub fn get_constants() -> crate::unresolved::NEUConstants {
        // java: return NEU_REPO.getConstants();
        // rust: port gap - the NEU repo parser is not ported: its `NEUConstants` is the placeholder (GAPS_L.md)
        crate::unresolved::NEUConstants
    }

    /// java: public static @Nullable NEURepoFile file(String path)
    pub fn file(path: &str) -> Option<crate::unresolved::NEURepoFile> {
        // java: return NEU_REPO.file(path); - null when the repo has no such file
        // rust: port gap - the repo is never downloaded here (see `REPO_LOADING`): no file exists
        let _ = path;
        None
    }

    /// java: public static Stream<NEURepoFile> tree(String path) throws NEURepositoryException
    pub fn tree(path: &str) -> Box<dyn Iterator<Item = crate::unresolved::NEURepoFile> + '_> {
        // java: return NEU_REPO.tree(path);
        // rust: port gap - the repo is never downloaded here (see `REPO_LOADING`): the tree is empty
        let _ = path;
        Box::new(std::iter::empty())
    }

    /// java: public static Map<@NEUId String, Set<NEURecipe>> getRecipes()
    pub fn get_recipes() -> std::collections::HashMap<String, std::collections::HashSet<crate::unresolved::NEURecipe>> {
        // java: return RECIPE_CACHE.getRecipes();
        // rust: port gap - the recipe cache is filled on a repo reload, which cannot happen here: it is empty
        std::collections::HashMap::new()
    }

    /// java: public static Map<@NEUId String, Set<NEURecipe>> getUsages()
    pub fn get_usages() -> std::collections::HashMap<String, std::collections::HashSet<crate::unresolved::NEURecipe>> {
        // java: return RECIPE_CACHE.getUsages();
        // rust: port gap - the recipe cache is filled on a repo reload, which cannot happen here: it is empty
        std::collections::HashMap::new()
    }

    /// java: public static Map<@NEUId String, ItemOverlayFile> getStackOverlays(int maxSupportedDataVersion)
    pub fn get_stack_overlays(mut max_supported_data_version: i32) -> std::collections::HashMap<String, crate::unresolved::ItemOverlayFile> {
        // java: return STACK_OVERLAYS.getMostUpToDateCompatibleWith(maxSupportedDataVersion);
        // rust: port gap - the overlays are read on a repo reload, which cannot happen here: there are none
        let _ = max_supported_data_version;
        std::collections::HashMap::new()
    }

}

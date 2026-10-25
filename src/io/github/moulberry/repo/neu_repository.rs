//! Ported from io.github.moulberry.repo.NEURepository (neurepoparser 1.12.0).

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, LazyLock, Mutex};

use crate::io::github::moulberry::repo::data::neu_crafting_recipe::NEUCraftingRecipe;
use crate::io::github::moulberry::repo::data::neu_forge_recipe::NEUForgeRecipe;
use crate::io::github::moulberry::repo::data::neu_kat_upgrade_recipe::NEUKatUpgradeRecipe;
use crate::io::github::moulberry::repo::data::neu_mob_drop_recipe::NEUMobDropRecipe;
use crate::io::github::moulberry::repo::data::neu_npc_shop_recipe::NEUNpcShopRecipe;
use crate::io::github::moulberry::repo::data::neu_recipe::NEURecipe;
use crate::io::github::moulberry::repo::data::neu_trade_recipe::NEUTradeRecipe;
use crate::io::github::moulberry::repo::data::neu_unknown_recipe::NEUUnknownRecipe;
use crate::io::github::moulberry::repo::i_reloadable::IReloadable;
use crate::io::github::moulberry::repo::neu_constants::NEUConstants;
use crate::io::github::moulberry::repo::neu_items::NEUItems;
use crate::io::github::moulberry::repo::neu_repo_file::NEURepoFile;
use crate::io::github::moulberry::repo::neu_repository_exception::NEURepositoryException;
use crate::io::github::moulberry::repo::vendored::runtime_type_adapter_factory::RuntimeTypeAdapterFactory;

/// java: final Gson gson = new GsonBuilder().setPrettyPrinting()
///         .registerTypeAdapter(NEUUnknownRecipe, new NEUUnknownRecipe.Serializer())
///         .registerTypeAdapter(NEUIngredient, new NEUIngredient.Serializer())
///         .registerTypeAdapter(Coordinate, new Coordinate.CoordinateSerializer())
///         .registerTypeAdapter(NEUMobDropRecipe.Drop, new NEUMobDropRecipe.Drop.Serializer())
///         .registerTypeAdapter(NEUCraftingRecipe, new NEUCraftingRecipe.Serializer())
///         .registerTypeAdapterFactory(RuntimeTypeAdapterFactory.of(NEURecipe.class, "type", true)
///             .registerSubtype(NEUForgeRecipe.class, "forge").registerSubtype(NEUTradeRecipe.class, "trade")
///             .registerSubtype(NEUCraftingRecipe.class, "crafting").registerSubtype(NEUMobDropRecipe.class, "drops")
///             .registerSubtype(NEUNpcShopRecipe.class, "npc_shop").registerSubtype(NEUKatUpgradeRecipe.class, "katgrade")
///             .setFallbackType(NEUUnknownRecipe.class).setDefaultTypeTag("crafting"))
///         .create();
///
/// The per-type adapters are each class's `Serializer::deserialize` / `from_json` (see `gson`); this is the one
/// piece of the Gson that is state - the polymorphic `NEURecipe` adapter. It does not depend on the repository
/// instance, so it is shared.
pub static RECIPE_ADAPTER: LazyLock<RuntimeTypeAdapterFactory<dyn NEURecipe>> = LazyLock::new(|| {
    RuntimeTypeAdapterFactory::of("io.github.moulberry.repo.data.NEURecipe", "type", true)
        .register_subtype(|json| Box::new(NEUForgeRecipe::from_json(json)) as Box<dyn NEURecipe>, "forge")
        .register_subtype(|json| Box::new(NEUTradeRecipe::from_json(json)) as Box<dyn NEURecipe>, "trade")
        .register_subtype(
            |json| Box::new(crate::io::github::moulberry::repo::data::neu_crafting_recipe::Serializer::deserialize(json)) as Box<dyn NEURecipe>,
            "crafting",
        )
        .register_subtype(|json| Box::new(NEUMobDropRecipe::from_json(json)) as Box<dyn NEURecipe>, "drops")
        .register_subtype(|json| Box::new(NEUNpcShopRecipe::from_json(json)) as Box<dyn NEURecipe>, "npc_shop")
        .register_subtype(|json| Box::new(NEUKatUpgradeRecipe::from_json(json)) as Box<dyn NEURecipe>, "katgrade")
        .set_fallback_type(|json| {
            Box::new(crate::io::github::moulberry::repo::data::neu_unknown_recipe::Serializer::deserialize(json)) as Box<dyn NEURecipe>
        })
        .set_default_type_tag("crafting")
});

/// The base object for a structured in memory representation of the NEU repository files.
///
/// java: public final class NEURepository
///
/// Shared as an `Arc` (Java hands the same object to every `NEURepoFile` and reloadable); the fields Java writes
/// during `reload` are atomics / locks because the reload runs on the loader thread while the game reads.
pub struct NEURepository {
    /// The base path of this repository. This is the folder containing the `.git` folder.
    ///
    /// java: final Path baseFolder;
    pub(crate) base_folder: PathBuf,
    /// All constants loaded in the `constant` folder of the repository.
    ///
    /// java: final NEUConstants constants;
    pub(crate) constants: Arc<NEUConstants>,
    /// All `IReloadable`s linked to this repository. Those can be either content that needs to be reloaded, caches
    /// that need to be marked as dirty, or other.
    ///
    /// java: final List<IReloadable> reloadables = new ArrayList<>();
    pub(crate) reloadables: Mutex<Vec<Arc<dyn IReloadable>>>,
    /// All items loaded in the `items` folder of the repository.
    ///
    /// java: final NEUItems items;
    pub(crate) items: Arc<NEUItems>,
    /// java: boolean isUnstable = true;
    is_unstable: AtomicBool,
    /// java: boolean isIncomplete = true;
    is_incomplete: AtomicBool,
    /// java: `public synchronized void reload()` - the monitor.
    reload_monitor: Mutex<()>,
}

impl NEURepository {
    /// Create an in memory loaded repository of the given file path. This function doesn't do any loading, instead
    /// `reload()` has to be manually called by the caller of this function. Consecutive calls with the same
    /// arguments will result in distinct objects, a common instance should be manually maintained.
    ///
    /// java: public static NEURepository of(@NonNull Path baseFolder)
    pub fn of(base_folder: PathBuf) -> Arc<NEURepository> {
        Self::new(base_folder)
    }

    /// java: private NEURepository(@NonNull Path baseFolder)
    fn new(base_folder: PathBuf) -> Arc<NEURepository> {
        let this = Arc::new(NEURepository {
            base_folder,
            constants: Arc::new(NEUConstants::new()),
            reloadables: Mutex::new(Vec::new()),
            items: Arc::new(NEUItems::new()),
            is_unstable: AtomicBool::new(true),
            is_incomplete: AtomicBool::new(true),
            reload_monitor: Mutex::new(()),
        });
        // java: registerReloadListener(this.constants); registerReloadListener(this.items);
        this.register_reload_listener(this.constants.clone());
        this.register_reload_listener(this.items.clone());
        this
    }

    /// Registers a `IReloadable` to be called whenever `reload()` is invoked. Reloaders are called in the order that
    /// they are registered (with internal reloadables like `NEUItems` and `NEUConstants` always being reloaded first).
    ///
    /// java: public void registerReloadListener(IReloadable reloadable)
    pub fn register_reload_listener(&self, reloadable: Arc<dyn IReloadable>) {
        self.reloadables.lock().unwrap_or_else(|p| p.into_inner()).push(reloadable);
    }

    /// Reloads the repository from disk. During reloading the state of the repository may be unstable. However, every
    /// individual state (like the collection of all items, the collection fo all fairy souls) are updated atomically,
    /// therefore it is advised to store those objects if consecutive queries should be executed in the same
    /// repository context.
    ///
    /// Errors with the first exception if there is an exception during loading; every other reloadable will still be
    /// reloaded (which may result in an inconsistent loaded state), and all exceptions are collectively returned after
    /// all reloads have finished using `add_suppressed`.
    ///
    /// java: public synchronized void reload() throws NEURepositoryException
    pub fn reload(self: &Arc<Self>) -> Result<(), NEURepositoryException> {
        let _monitor = self.reload_monitor.lock().unwrap_or_else(|p| p.into_inner());
        let mut stored_exceptions: Vec<NEURepositoryException> = Vec::new();
        self.is_unstable.store(true, Ordering::SeqCst);
        // java: for (IReloadable reloadable : reloadables) - a snapshot, so a listener may register another
        let reloadables: Vec<Arc<dyn IReloadable>> = self.reloadables.lock().unwrap_or_else(|p| p.into_inner()).clone();
        for reloadable in reloadables {
            // java: try { reloadable.reload(this); } catch (NEURepositoryException e) { storedExceptions.add(e); }
            //       catch (Exception e) { storedExceptions.add(new NEURepositoryException("<unknown>", "Invalid non NEU exception thrown during repository reload", e)); }
            // A Java unchecked exception thrown by the parsing (Gson's JsonParseException, NumberFormatException,
            // NullPointerException) is a panic here, caught the same way.
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| reloadable.reload(self))) {
                Ok(Ok(())) => {}
                Ok(Err(e)) => stored_exceptions.push(e),
                Err(panic) => stored_exceptions.push(NEURepositoryException::new(
                    "<unknown>",
                    "Invalid non NEU exception thrown during repository reload",
                    Some(gson::panic_message(&panic)),
                )),
            }
        }
        let mut stored_exceptions = stored_exceptions.into_iter();
        if let Some(mut first) = stored_exceptions.next() {
            for other in stored_exceptions {
                first.add_suppressed(other);
            }
            self.is_incomplete.store(true, Ordering::SeqCst);
            return Err(first);
        }
        self.is_unstable.store(false, Ordering::SeqCst);
        self.is_incomplete.store(false, Ordering::SeqCst);
        Ok(())
    }

    /// Obtain a `NEURepoFile` that is guaranteed to exist.
    ///
    /// Errors if the file does not exist. This exception should be propagated. If you instead want to react to a non
    /// existance of the file, use `file` instead.
    ///
    /// java: public NEURepoFile requireFile(@NonNull String path) throws NEURepositoryException
    pub fn require_file(self: &Arc<Self>, path: &str) -> Result<NEURepoFile, NEURepositoryException> {
        match self.file(path) {
            Some(file) => Ok(file),
            None => Err(NEURepositoryException::new(path, "this file is required", None)),
        }
    }

    /// Obtain a `NEURepoFile`: the file as a repo file, or `None` if it doesn't exist.
    ///
    /// java: public @Nullable NEURepoFile file(@NonNull String path)
    pub fn file(self: &Arc<Self>, path: &str) -> Option<NEURepoFile> {
        let fs_path: PathBuf = self.base_folder.join(path);
        // java: if (!Files.isReadable(fsPath)) return null;
        if !is_readable(&fs_path) {
            return None;
        }
        Some(NEURepoFile::new(self.clone(), fs_path))
    }

    /// A stream of `NEURepoFile`s contained within this relative path (the directory itself first, then everything
    /// under it, depth first - `Files.walk`).
    ///
    /// java: public Stream<NEURepoFile> tree(@NonNull String path) throws NEURepositoryException
    pub fn tree(self: &Arc<Self>, path: &str) -> Result<std::vec::IntoIter<NEURepoFile>, NEURepositoryException> {
        let fs_path: PathBuf = self.base_folder.join(path);
        // java: if (!Files.isDirectory(fsPath)) return Stream.empty();
        if !fs_path.is_dir() {
            return Ok(Vec::new().into_iter());
        }
        // java: return Files.walk(fsPath).map(it -> new NEURepoFile(this, it));
        let mut walked: Vec<PathBuf> = Vec::new();
        walk(&fs_path, &mut walked).map_err(|e| NEURepositoryException::new(path, "could not walk directory", Some(e.to_string())))?;
        Ok(walked.into_iter().map(|it| NEURepoFile::new(self.clone(), it)).collect::<Vec<_>>().into_iter())
    }

    /// The base path of this repository. This is the folder containing the `.git` folder.
    ///
    /// java: public Path getBaseFolder()
    pub fn get_base_folder(&self) -> &Path {
        &self.base_folder
    }

    /// All constants loaded in the `constant` folder of the repository.
    ///
    /// java: public NEUConstants getConstants()
    pub fn get_constants(&self) -> &Arc<NEUConstants> {
        &self.constants
    }

    /// java: public List<IReloadable> getReloadables()
    pub fn get_reloadables(&self) -> Vec<Arc<dyn IReloadable>> {
        self.reloadables.lock().unwrap_or_else(|p| p.into_inner()).clone()
    }

    /// All items loaded in the `items` folder of the repository.
    ///
    /// java: public NEUItems getItems()
    pub fn get_items(&self) -> &Arc<NEUItems> {
        &self.items
    }

    /// If the current state of the repository is unstable. Unstable means that either the repository is
    /// `is_incomplete` or the repository is currently being `reload()`ed and therefore may have mismatched versions of
    /// different objects.
    ///
    /// java: public boolean isUnstable()
    pub fn is_unstable(&self) -> bool {
        self.is_unstable.load(Ordering::SeqCst)
    }

    /// If the current state of the repository is incomplete. Incomplete means that either there hasn't been a
    /// `reload()` yet, or the last reload completed exceptionally.
    ///
    /// java: public boolean isIncomplete()
    pub fn is_incomplete(&self) -> bool {
        self.is_incomplete.load(Ordering::SeqCst)
    }
}

/// `Files.isReadable(path)`: the path exists and can be opened for reading (a directory counts, as in Java).
fn is_readable(path: &Path) -> bool {
    match std::fs::metadata(path) {
        Ok(meta) if meta.is_dir() => std::fs::read_dir(path).is_ok(),
        Ok(_) => std::fs::File::open(path).is_ok(),
        Err(_) => false,
    }
}

/// `Files.walk(start)`: the start, then each entry depth first (a directory before its contents).
fn walk(path: &Path, out: &mut Vec<PathBuf>) -> std::io::Result<()> {
    out.push(path.to_path_buf());
    if path.is_dir() {
        let mut entries: Vec<PathBuf> = std::fs::read_dir(path)?.map(|e| e.map(|e| e.path())).collect::<std::io::Result<_>>()?;
        // `Files.walk` hands entries in the directory's own order; a sorted order is one such order and keeps the
        // walk stable between runs.
        entries.sort();
        for entry in entries {
            walk(&entry, out)?;
        }
    }
    Ok(())
}

/// The read half of Gson's built-in type adapters, as the repository's `gson` applies them while binding a class.
///
/// Gson binds a missing member, or a JSON `null`, as the field's default (`null` / `0` / `false`); the helpers answer
/// `None` for both so the caller picks the field's Java default. Gson's primitive adapters are lenient about the
/// JSON type the way these are: a number reads as a `String` (its literal), a numeric string reads as a number.
pub mod gson {
    use serde_json::Value;

    /// The member `name` of a JSON object (`JsonObject.get`), `None` when absent, `null`, or not an object.
    pub fn member<'a>(json: &'a Value, name: &str) -> Option<&'a Value> {
        match json.as_object()?.get(name) {
            Some(Value::Null) | None => None,
            Some(value) => Some(value),
        }
    }

    /// `TypeAdapters.STRING` / `JsonElement.getAsString()`.
    pub fn as_string(json: &Value) -> Option<String> {
        match json {
            Value::String(s) => Some(s.clone()),
            Value::Number(n) => Some(n.to_string()),
            Value::Bool(b) => Some(b.to_string()),
            _ => None,
        }
    }

    /// `TypeAdapters.DOUBLE` / `getAsDouble()`.
    pub fn as_f64(json: &Value) -> Option<f64> {
        match json {
            Value::Number(n) => n.as_f64(),
            Value::String(s) => s.trim().parse::<f64>().ok(),
            _ => None,
        }
    }

    /// `TypeAdapters.LONG` / `getAsLong()` (`JsonReader.nextLong` accepts an integral double).
    pub fn as_i64(json: &Value) -> Option<i64> {
        match json {
            Value::Number(n) => n.as_i64().or_else(|| n.as_f64().map(|d| d as i64)),
            Value::String(s) => s.trim().parse::<i64>().ok().or_else(|| s.trim().parse::<f64>().ok().map(|d| d as i64)),
            _ => None,
        }
    }

    /// `TypeAdapters.INTEGER` / `getAsInt()`.
    pub fn as_i32(json: &Value) -> Option<i32> {
        as_i64(json).map(|v| v as i32)
    }

    /// `TypeAdapters.FLOAT`.
    pub fn as_f32(json: &Value) -> Option<f32> {
        as_f64(json).map(|v| v as f32)
    }

    /// `TypeAdapters.BOOLEAN` (lenient: the string `"true"`).
    pub fn as_bool(json: &Value) -> Option<bool> {
        match json {
            Value::Bool(b) => Some(*b),
            Value::String(s) => Some(s.eq_ignore_ascii_case("true")),
            _ => None,
        }
    }

    /// A `String` field.
    pub fn string(json: &Value, name: &str) -> Option<String> {
        member(json, name).and_then(as_string)
    }

    /// An `int` field (`0` when absent).
    pub fn int(json: &Value, name: &str) -> i32 {
        member(json, name).and_then(as_i32).unwrap_or(0)
    }

    /// An `Integer` field (`null` when absent).
    pub fn integer(json: &Value, name: &str) -> Option<i32> {
        member(json, name).and_then(as_i32)
    }

    /// A `long` field.
    pub fn long(json: &Value, name: &str) -> i64 {
        member(json, name).and_then(as_i64).unwrap_or(0)
    }

    /// A `double` field.
    pub fn double(json: &Value, name: &str) -> f64 {
        member(json, name).and_then(as_f64).unwrap_or(0.0)
    }

    /// A `boolean` field.
    pub fn boolean(json: &Value, name: &str) -> Option<bool> {
        member(json, name).and_then(as_bool)
    }

    /// A `List<E>` (`CollectionTypeAdapterFactory`): `None` for absent / `null`; each element read by `element`
    /// (a `null` element is skipped - Gson would put a `null` in the list).
    pub fn list<E>(json: &Value, element: impl Fn(&Value) -> Option<E>) -> Option<Vec<E>> {
        Some(json.as_array()?.iter().filter_map(|e| if e.is_null() { None } else { element(e) }).collect())
    }

    /// A `Map<K, V>` (`MapTypeAdapterFactory`): keys read from the member names by `key` (a key that does not parse
    /// - an unknown enum constant is Gson's `null` key - is dropped).
    pub fn map<K: std::hash::Hash + Eq, V>(
        json: &Value,
        key: impl Fn(&str) -> Option<K>,
        value: impl Fn(&Value) -> Option<V>,
    ) -> Option<std::collections::HashMap<K, V>> {
        Some(json.as_object()?.iter().filter_map(|(k, v)| Some((key(k)?, if v.is_null() { return None } else { value(v)? }))).collect())
    }

    /// `List<String>` member.
    pub fn string_list(json: &Value, name: &str) -> Option<Vec<String>> {
        member(json, name).and_then(|v| list(v, as_string))
    }

    /// `Integer` map key (`Integer.parseInt` of the member name).
    pub fn int_key(key: &str) -> Option<i32> {
        key.trim().parse::<i32>().ok()
    }

    /// `String` map key.
    pub fn string_key(key: &str) -> Option<String> {
        Some(key.to_string())
    }

    /// The text of a caught panic - the Java exception's message.
    pub fn panic_message(panic: &Box<dyn std::any::Any + Send>) -> String {
        if let Some(s) = panic.downcast_ref::<String>() {
            s.clone()
        } else if let Some(s) = panic.downcast_ref::<&str>() {
            s.to_string()
        } else {
            "<unknown exception>".to_string()
        }
    }
}

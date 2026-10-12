//! Ported from de.hysky.skyblocker.utils.data.JsonData by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::backup::config_backup_manager;
use crate::entry::skyblocker_mod;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::com::mojang::serialization::data_result::DataResult;
use mcrust::java::util::concurrent::completable_future::CompletableFuture;
use mcrust::com::mojang::serialization::json_ops::JsonOps;
use mcrust::com::mojang::serialization::decoder::Decoder as _;
use mcrust::com::mojang::serialization::encoder::Encoder as _;
use mcrust::net::minecraft::client::gui::components::toasts::system_toast::SystemToastId;
use mcrust::net::minecraft::client::minecraft;
use mcrust::net::minecraft::network::chat::component;

// TODO(fabric2seer): the port's `new` ports this Java method with other parameters ( for Java's display_time)
//   java: public static final SystemToast.SystemToastId ERROR_TOAST_ID = new SystemToast.SystemToastId(10_000L);
/// java: public static final SystemToast.SystemToastId ERROR_TOAST_ID = new SystemToast.SystemToastId(10_000L);
pub static ERROR_TOAST_ID: compat::MainThread<SystemToastId> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `ERROR_TOAST_ID` - see the TODO above"));

/// java: public class JsonData<T>
pub struct JsonDataBase<T> {
    /// java: private final Path file;
    pub file: Option<std::path::PathBuf>,
    /// java: private final Codec<T> codec;
    pub codec: Option<CodecRef<T>>,
    /// java: private final boolean compressed;
    pub compressed: bool,
    /// java: private final boolean loadAsync;
    pub load_async: bool,
    /// java: private final boolean saveAsync;
    pub save_async: bool,
    /// java: private T data;
    pub data: Option<T>,
    /// java: private @Nullable CompletableFuture<Void> loaded;
    pub loaded: Option<mcrust::java::util::concurrent::completable_future::CompletableFuture<()>>,
}

impl<T: Clone + 'static> JsonDataBase<T> {
    /// java: public JsonData(Path file, Codec<T> codec, T defaultValue)
    pub fn new(mut file: std::path::PathBuf, mut codec: CodecRef<T>, mut default_value: T) -> Self {
        let mut this = Self::new_4(file.clone(), codec.clone(), default_value.clone(), false);
        this
    }

    /// java: public JsonData(Path file, Codec<T> codec, T defaultValue, boolean loadAsync, boolean saveAsync)
    pub fn new_5(mut file: std::path::PathBuf, mut codec: CodecRef<T>, mut default_value: T, mut load_async: bool, mut save_async: bool) -> Self {
        let mut this = Self::new_6(file.clone(), codec.clone(), default_value.clone(), false, load_async, save_async);
        this
    }

    /// java: public JsonData(Path file, Codec<T> codec, T defaultValue, boolean compressed)
    pub fn new_4(mut file: std::path::PathBuf, mut codec: CodecRef<T>, mut default_value: T, mut compressed: bool) -> Self {
        let mut this = Self::new_6(file.clone(), codec.clone(), default_value.clone(), compressed, true, true);
        this
    }

    /// java: public JsonData(Path file, Codec<T> codec, T defaultValue, boolean compressed, boolean loadAsync, boolean saveAsync)
    pub fn new_6(mut file: std::path::PathBuf, mut codec: CodecRef<T>, mut default_value: T, mut compressed: bool, mut load_async: bool, mut save_async: bool) -> Self {
        let __init_file = Some(file.clone());
        let __init_codec = Some(codec.clone());
        let __init_data = Some(default_value.clone());
        let __init_compressed = compressed;
        let __init_load_async = load_async;
        let __init_save_async = save_async;
        let mut this = Self {
            file: __init_file,
            codec: __init_codec,
            compressed: __init_compressed,
            load_async: __init_load_async,
            save_async: __init_save_async,
            data: __init_data,
            loaded: None,
        };
        this
    }

}

/// java: the virtual methods of JsonData (implemented by JsonDataBase and every subclass)
pub trait JsonData<T> {
    fn json_data_base(&self) -> &JsonDataBase<T>;
    fn json_data_base_mut(&mut self) -> &mut JsonDataBase<T>;
    fn as_any(&self) -> &dyn std::any::Any;

    /// java: public CompletableFuture<Void> init()
    fn init(&mut self) -> mcrust::java::util::concurrent::completable_future::CompletableFuture<()> {
        // TODO(fabric2seer): no SeerClient event yet: hook HEAD of Minecraft#stop (`net.minecraft.client.Minecraft#stop`: exists)
        //   java: ClientLifecycleEvents.CLIENT_STOPPING.register(_ -> save().join());
        return self.load();
    }

    /// java: public CompletableFuture<Void> load()
    fn load(&mut self) -> mcrust::java::util::concurrent::completable_future::CompletableFuture<()> {
        if self.json_data_base().load_async {
            // java: loaded = CompletableFuture.runAsync(this::loadInternal, SkyblockerMod.VIRTUAL_THREAD_EXECUTOR);
            // rust: port gap - the object, its `Codec` (`Arc<dyn Codec<T>>`) and its data are not `Send`, so the load
            // cannot move to another thread; it runs here and `loaded` is complete when it returns (GAPS_batchA.md)
            self.load_internal();
            self.json_data_base_mut().loaded = Some(CompletableFuture::completed_future(()));
        } else {
            self.load_internal();
            self.json_data_base_mut().loaded = Some(CompletableFuture::completed_future(()));
        }
        return self.json_data_base().loaded.clone().unwrap();
    }

    /// java: private void loadInternal()
    fn load_internal(&mut self) {
        let mut create_backup: bool = false;
        let file: std::path::PathBuf = self.json_data_base().file.clone().expect("JsonData.file");
        // java: try (BufferedReader reader = Files.newBufferedReader(file)) {..} catch (NoSuchFileException _) {}
        // catch (Exception e) {..}
        match std::fs::read_to_string(&file) {
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
            Err(e) => {
                create_backup = true;
                logger().error(&format!("[Skyblocker Json Data] Failed to load data from file: `{}` {}", file.display(), e));
            }
            Ok(reader) => {
                let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    // java: DataResult<T> parsed = codec.parse(compressed ? JsonOps.COMPRESSED : JsonOps.INSTANCE, JsonParser.parseReader(reader));
                    let ops = if self.json_data_base().compressed { JsonOps::compressed() } else { JsonOps::instance() };
                    let json: serde_json::Value = serde_json::from_str(&reader).unwrap_or_else(|e| panic!("JsonSyntaxException: {e}"));
                    let codec: CodecRef<T> = self.json_data_base().codec.clone().expect("JsonData.codec");
                    let mut parsed: DataResult<T> = codec.parse(&ops, &compat::json_to_port(&json));
                    let is_error: bool = parsed.is_error();
                    if let Some(t) = parsed.result_or_partial_with(|s| { logger().error(&format!("[Skyblocker Json Data] Failed to parse data from file: `{}`. {}", file.display(), s)); }) {
                        self.json_data_base_mut().data = Some(t);
                    }
                    is_error
                }));
                match __result {
                    Ok(is_error) => create_backup = is_error,
                    Err(e) => {
                        create_backup = true;
                        logger().error(&format!("[Skyblocker Json Data] Failed to load data from file: `{}` {}", file.display(), compat::panic_message(&e)));
                    }
                }
            }
        }
        if create_backup {
            // TODO(fabric2seer): Path.relativize
            //   java: SystemToast.add(
            //   java: 					Minecraft.getInstance().gui.toastManager(), ERROR_TOAST_ID,
            //   java: 					Component.literal("Skyblocker Config Error"),
            //   java: 					Component.literal("Failed to load '" + FabricLoader.getInstance().getConfigDir().relativize(file) + "'")
            //   java: 							.append("\n")
            //   java: 							.append("See logs for details. A backup of the file has been made.")
            //   java: 			);
            // java: try {..} catch (IOException e) - an exception here is a panic, caught by `catch_unwind`
            let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let mut file_name: String = std::path::PathBuf::from(self.json_data_base().file.as_ref().unwrap().file_name().unwrap_or_default()).to_string_lossy().to_string();
                let mut extension_index: i32 = compat::last_index_of(&file_name, '.');
                let mut new_file_name: String;
                if extension_index >= 0 {
                    let mut extension: String = compat::substring(&file_name, extension_index);
                    // TODO(fabric2seer): static call LocalDateTime.now(..)
                    //   java: newFileName = fileName.substring(0, extensionIndex) + '_' + ConfigBackupManager.FORMATTER.format(LocalDateTime.now()) + extension;
                } else {
                    logger().warn(&format!("[Skyblocker Json Data] No extension? {}", compat::opt_str(&self.json_data_base().file.as_ref().map(|p| p.display().to_string()))));
                    // TODO(fabric2seer): static call LocalDateTime.now(..)
                    //   java: newFileName = fileName + '_' + ConfigBackupManager.FORMATTER.format(LocalDateTime.now());
                }
                // TODO(fabric2seer): static field StandardCopyOption.REPLACE_EXISTING
                //   java: Files.copy(file, file.getParent().resolve(newFileName), StandardCopyOption.REPLACE_EXISTING);
            }));
            if let Err(e) = __result {
                logger().error(&format!("[Skyblocker Json Data] Failed to create backup for file: `{}`", compat::opt_str(&self.json_data_base().file.as_ref().map(|p| p.display().to_string()))));
            }
        }
    }

    /// java: public CompletableFuture<Void> save()
    fn save(&self) -> mcrust::java::util::concurrent::completable_future::CompletableFuture<()> {
        if self.json_data_base().save_async && minecraft::is_running() {
            // java: return CompletableFuture.runAsync(this::saveInternal, SkyblockerMod.VIRTUAL_THREAD_EXECUTOR);
            // rust: port gap - as `load`: the object is not `Send`, so the save runs here (GAPS_batchA.md)
            self.save_internal();
            return CompletableFuture::completed_future(());
        } else {
            self.save_internal();
            return CompletableFuture::completed_future(());
        }
    }

    /// java: private void saveInternal()
    fn save_internal(&self) {
        if self.json_data_base().loaded.is_none() {
            logger().error(&format!("[Skyblocker Json Data] Save data called when loading has not started for file `{}`. This will override the contents of the file with the default value.", compat::opt_str(&self.json_data_base().file.as_ref().map(|p| p.display().to_string()))));
        } else if !self.is_loaded() {
            logger().warn(&format!("[Skyblocker Json Data] Save data called when loading has not finished for file `{}`. Blocking until data is loaded.", compat::opt_str(&self.json_data_base().file.as_ref().map(|p| p.display().to_string()))));
            self.json_data_base().loaded.as_ref().unwrap().join();
        }
        // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            std::fs::create_dir_all(&self.json_data_base().file.as_ref().unwrap().parent().map(|x| x.to_path_buf()).unwrap()).expect("IOException");
        }));
        if let Err(e) = __result {
            logger().error(&format!("[Skyblocker Json Data] Failed to create directories for file: `{}`", compat::opt_str(&self.json_data_base().file.as_ref().map(|p| p.display().to_string()))));
        }
        // java: try (BufferedWriter writer = Files.newBufferedWriter(file)) {
        //   SkyblockerMod.GSON.toJson(codec.encodeStart(compressed ? JsonOps.COMPRESSED : JsonOps.INSTANCE, data).getOrThrow(), writer);
        // } catch (Exception e) {..}
        // (`newBufferedWriter` truncates the file before the data is encoded; GSON is `setPrettyPrinting()`)
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            use std::io::Write as _;
            let file: std::path::PathBuf = self.json_data_base().file.clone().expect("JsonData.file");
            let mut writer = std::io::BufWriter::new(std::fs::File::create(&file).unwrap_or_else(|e| panic!("IOException: {e}")));
            let ops = if self.json_data_base().compressed { JsonOps::compressed() } else { JsonOps::instance() };
            let codec: CodecRef<T> = self.json_data_base().codec.clone().expect("JsonData.codec");
            let encoded = codec.encode_start(&ops, self.json_data_base().data.as_ref().expect("JsonData.data")).get_or_throw().unwrap_or_else(|e| panic!("IllegalStateException: {e}"));
            let text: String = serde_json::to_string_pretty(&compat::json_from_port(&encoded)).unwrap_or_else(|e| panic!("JsonIOException: {e}"));
            writer.write_all(text.as_bytes()).unwrap_or_else(|e| panic!("IOException: {e}"));
            writer.flush().unwrap_or_else(|e| panic!("IOException: {e}"));
        }));
        if let Err(e) = __result {
            logger().error(&format!("[Skyblocker Json Data] Failed to save data to file: `{}` {}", compat::opt_str(&self.json_data_base().file.as_ref().map(|p| p.display().to_string())), compat::panic_message(&e)));
        }
    }

    /// java: @Deprecated(forRemoval = true) public boolean isEmpty()
    fn is_empty(&self) -> bool {
        return !self.is_loaded();
    }

    /// java: public boolean isLoaded()
    fn is_loaded(&self) -> bool {
        return (self.json_data_base().loaded.is_some() && self.json_data_base().loaded.as_ref().unwrap().is_done());
    }

    /// java: public T getData()
    fn get_data(&self) -> T
    where
        T: Clone,
    {
        if self.json_data_base().loaded.is_none() {
            logger().error(&format!("[Skyblocker Json Data] Get data called when loading has not started for file `{}`. Returning default value.", compat::opt_str(&self.json_data_base().file.as_ref().map(|p| p.display().to_string()))));
        } else if !self.is_loaded() {
            logger().warn(&format!("[Skyblocker Json Data] Get data called when loading has not finished for file `{}`. Blocking until data is loaded.", compat::opt_str(&self.json_data_base().file.as_ref().map(|p| p.display().to_string()))));
            self.json_data_base().loaded.as_ref().unwrap().join();
        }
        return self.json_data_base().data.clone().unwrap();
    }

    /// java: public T setData(T data)
    fn set_data(&mut self, mut data: T) -> T {
        let mut old_data: T = self.json_data_base_mut().data.replace(data).expect("JsonData.data");
        return old_data;
    }

}

impl<T: Clone + 'static> JsonData<T> for JsonDataBase<T> {
    fn json_data_base(&self) -> &JsonDataBase<T> {
        self
    }
    fn json_data_base_mut(&mut self) -> &mut JsonDataBase<T> {
        self
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}

//! Ported from de.hysky.skyblocker.utils.data.constants.ConstantData by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::data::constants::emblem_constants;
use crate::de::hysky::skyblocker::utils::data::constants::emblem_constants::EmblemConstants;
use crate::de::hysky::skyblocker::utils::data::constants::minion_constants;
use crate::de::hysky::skyblocker::utils::data::constants::minion_constants::MinionConstants;
use crate::entry::skyblocker_mod;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::java::util::concurrent::completable_future::CompletableFuture;
use mcrust::com::mojang::serialization::decoder::Decoder as _;
use mcrust::net::minecraft::server::packs::resources::resource_provider::ResourceProvider as _;

/// java: private static EmblemConstants emblemConstants = EmblemConstants.EMPTY;
pub static EMBLEM_CONSTANTS: compat::MainThread<EmblemConstants> = compat::MainThread::new(|| emblem_constants::EMPTY.get_mut().clone());
/// java: private static MinionConstants minionConstants = MinionConstants.EMPTY;
pub static MINION_CONSTANTS: LazyLock<Mutex<MinionConstants>> = LazyLock::new(|| Mutex::new(minion_constants::EMPTY.clone()));

/// java: public final class ConstantData
#[derive(Clone)]
pub struct ConstantData {}

impl ConstantData {
    /// java: private ConstantData()
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        // java: ClientLifecycleEvents.CLIENT_STARTED.register(minecraft -> {
        //           loadDataAsync(minecraft, "emblems.json", EmblemConstants.CODEC, data -> emblemConstants = data);
        //           loadDataAsync(minecraft, "minions.json", MinionConstants.CODEC, data -> minionConstants = data); });
        mcrust::mcrust::modding::events::client_lifecycle_events::register_client_started(|minecraft| {
            ConstantData::load_data_async(minecraft, "emblems.json", emblem_constants::CODEC.get(), Arc::new(|data: EmblemConstants| { *EMBLEM_CONSTANTS.get_mut() = data; }));
            ConstantData::load_data_async(minecraft, "minions.json", minion_constants::CODEC.get(), Arc::new(|data: MinionConstants| { compat::set_state(&MINION_CONSTANTS, data); }));
        });
    }

    /// java: public static EmblemConstants getEmblemConstants()
    pub fn get_emblem_constants() -> EmblemConstants {
        return EMBLEM_CONSTANTS.get_mut().clone();
    }

    /// java: public static MinionConstants getMinionConstants()
    pub fn get_minion_constants() -> MinionConstants {
        return compat::get_state(&MINION_CONSTANTS);
    }

    /// java: private static <T> void loadDataAsync(Minecraft minecraft, String fileName, Codec<T> codec, Consumer<T> setter)
    pub fn load_data_async<T: Clone + 'static>(minecraft: &mut mcrust::net::minecraft::client::minecraft::Minecraft, file_name: &str, codec: &CodecRef<T>, setter: Arc<dyn Fn(T)>) {
        // java: CompletableFuture.supplyAsync(() -> loadData(minecraft, fileName, codec), VIRTUAL_THREAD_EXECUTOR)
        //           .thenAcceptAsync(data -> setData(data, setter), minecraft);
        // Verified gap: the port's `Codec` objects are not `Send` (`Arc<dyn Codec<T>>`), so `loadData` is split where the
        // thread changes: the worker reads the resource (`openAsReader`), and the parse runs with `setData` on the
        // render thread (`minecraft` as the executor). `codec` and `setter` stay on the render thread.
        let file_name: String = file_name.to_string();
        let worker_file_name: String = file_name.clone();
        let codec: compat::RenderThread<CodecRef<T>> = compat::RenderThread::new(codec.clone());
        let setter: compat::RenderThread<Arc<dyn Fn(T)>> = compat::RenderThread::new(setter);
        (CompletableFuture::supply_async(move || Ok(ConstantData::open_data(compat::minecraft(), &worker_file_name)), compat::executor))
            .then_accept_async(move |reader: &Result<String, String>| {
                let data: Option<T> = ConstantData::parse_data(reader.clone(), &file_name, codec.get());
                ConstantData::set_data(data, setter.get().clone());
            }, std::sync::Arc::new(|__t: Box<dyn FnOnce() + Send>| crate::compat::execute(__t)));
    }

    /// `loadData`'s `minecraft.getResourceManager().openAsReader(SkyblockerMod.id("constants/" + fileName))`.
    fn open_data(minecraft: &mcrust::net::minecraft::client::minecraft::Minecraft, file_name: &str) -> Result<String, String> {
        minecraft.get_resource_manager().open_as_reader(&crate::entry::skyblocker_mod::id(&format!("constants/{}", file_name)))
    }

    /// java: private static <T> @Nullable T loadData(Minecraft minecraft, String fileName, Codec<T> codec)
    pub fn load_data<T: 'static>(minecraft: &mut mcrust::net::minecraft::client::minecraft::Minecraft, file_name: &str, codec: &CodecRef<T>) -> Option<T> {
        return ConstantData::parse_data(ConstantData::open_data(minecraft, file_name), file_name, codec);
    }

    /// `loadData`'s try block after the resource was opened: `codec.parse(JsonOps.INSTANCE, JsonParser.parseReader(reader))
    /// .getOrThrow()`, or `null` (logged) for any exception, the open's included.
    fn parse_data<T: 'static>(reader: Result<String, String>, file_name: &str, codec: &CodecRef<T>) -> Option<T> {
        // java: try (BufferedReader reader = ..) {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut reader: String = reader.clone().unwrap_or_else(|e| panic!("IOException: {e}"));
            // java: return codec.parse(JsonOps.INSTANCE, JsonParser.parseReader(reader)).getOrThrow();
            let json: serde_json::Value = serde_json::from_str(&reader).unwrap_or_else(|e| panic!("JsonSyntaxException: {e}"));
            return codec.parse(&mcrust::com::mojang::serialization::json_ops::JsonOps::instance(), &compat::json_to_port(&json)).get_or_throw().unwrap_or_else(|e| panic!("IllegalStateException: {e}"));
        })) {
            Ok(__v) => return Some(__v),
            Err(e) => {
                logger().error(&format!("[Skyblocker Constant Data] Failed to load data for {}. {}", file_name, compat::panic_message(&e)));
                return None;
            }
        }
    }

    /// java: private static <T> void setData(@Nullable T data, Consumer<T> setter)
    pub fn set_data<T: 'static>(mut data: Option<T>, setter: Arc<dyn Fn(T)>) {
        if let Some(data) = data {
            (setter)(data);
        }
    }

}

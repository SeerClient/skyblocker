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
        // TODO(fabric2seer): no SeerClient event yet: hook RETURN of Minecraft's constructor (`net.minecraft.client.Minecraft#<init>`: exists)
        //   java: ClientLifecycleEvents.CLIENT_STARTED.register(minecraft -> {
        //   java: 			loadDataAsync(minecraft, "emblems.json", EmblemConstants.CODEC, data -> emblemConstants = data);
        //   java: 			loadDataAsync(minecraft, "minions.json", MinionConstants.CODEC, data -> minionConstants = data);
        //   java: 		});
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
        (CompletableFuture::supply_async(move || Ok((|| ConstantData::load_data(minecraft, file_name, codec))()), compat::executor)).then_accept_async(|data| ConstantData::set_data(Some(data.clone()), setter.clone()), std::sync::Arc::new(|__t: Box<dyn FnOnce() + Send>| mcrust::mcrust::modding::game::execute(__t)));
    }

    /// java: private static <T> @Nullable T loadData(Minecraft minecraft, String fileName, Codec<T> codec)
    pub fn load_data<T: 'static>(minecraft: &mut mcrust::net::minecraft::client::minecraft::Minecraft, file_name: &str, codec: &CodecRef<T>) -> Option<T> {
        {
            let mut reader: crate::unresolved::BufferedReader = minecraft.get_resource_manager().open_as_reader(&crate::entry::skyblocker_mod::id(&format!("constants/{}", file_name))).unwrap();
            // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`; the body's `return` is the closure's value
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                // TODO(fabric2seer): static call JsonParser.parseReader(..)
                //   java: return codec.parse(JsonOps.INSTANCE, JsonParser.parseReader(reader)).getOrThrow();
            })) {
                Ok(__v) => return __v,
                Err(e) => {
                    logger().error(&format!("[Skyblocker Constant Data] Failed to load data for {}.", file_name));
                    return None;
                }
            }
        }
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: private static <T> void setData(@Nullable T data, Consumer<T> setter)
    pub fn set_data<T: 'static>(mut data: Option<T>, setter: Arc<dyn Fn(T)>) {
        if let Some(data) = &data {
            (setter)(data.clone());
        }
    }

}

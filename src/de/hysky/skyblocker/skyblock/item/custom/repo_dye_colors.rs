//! Ported from de.hysky.skyblocker.skyblock.item.custom.RepoDyeColors by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::codec_utils::CodecUtils;
use crate::de::hysky::skyblocker::utils::neu_repo_manager::NEURepoManager;
use mcrust::com::mojang::serialization::codec;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::com::mojang::serialization::codecs::record_codec_builder;

/// java: public static final Map<String, Integer> STATIC_DYES = Object2ObjectMaps.synchronize(new Object2ObjectOpenHashMap<>());
pub static STATIC_DYES: LazyLock<Mutex<std::collections::HashMap<String, i32>>> = LazyLock::new(|| Mutex::new(std::collections::HashMap::new()));
/// java: public static final Map<String, List<Integer>> ANIMATED_DYES = Object2ObjectMaps.synchronize(new Object2ObjectOpenHashMap<>());
pub static ANIMATED_DYES: LazyLock<Mutex<std::collections::HashMap<String, Vec<i32>>>> = LazyLock::new(|| Mutex::new(std::collections::HashMap::new()));
/// java: private static final Codec<Dyes> CODEC = RecordCodecBuilder.create(instance -> instance.group( CodecUtils.object2ObjectMapCodec(Codec.STRING, ExtraCodecs.STRING_RGB_COLOR.listOf()).fieldOf("animated")
pub static DYES_CODEC: compat::MainThread<CodecRef<Dyes>> = compat::MainThread::new(|| record_codec_builder::create(record_codec_builder::apply2(record_codec_builder::of(|o: &Dyes| o.animated_dyes(), codec::field_of(CodecUtils::object2_object_map_codec(&mcrust::com::mojang::serialization::codec::string_codec(), &codec::list(mcrust::net::minecraft::util::extra_codecs::string_rgb_color().clone())), "animated")), record_codec_builder::of(|o: &Dyes| o.static_dyes(), codec::field_of(codec::xmap(codec::unbounded_map(mcrust::com::mojang::serialization::codec::string_codec(), mcrust::net::minecraft::util::extra_codecs::string_rgb_color()), |entries: Vec<(String, i32)>| entries.into_iter().collect::<std::collections::HashMap<String, i32>>(), |map: &std::collections::HashMap<String, i32>| map.iter().map(|(k, v)| (k.clone(), *v)).collect::<Vec<_>>()), "static")), move |__a0, __a1| Dyes::new(__a0.clone(), __a1.clone()))));

/// java: public class RepoDyeColors
#[derive(Clone)]
pub struct RepoDyeColors {}

impl RepoDyeColors {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        NEURepoManager::run_async_after_load(Arc::new(move || { RepoDyeColors::load_dyes(); }));
    }

    /// java: public static void loadDyes()
    pub fn load_dyes() {
        compat::lock(&STATIC_DYES).clear();
        compat::lock(&ANIMATED_DYES).clear();
        let Some(mut file) = NEURepoManager::file("constants/dyes.json") else {
            return;
        };
        {
            // TODO(fabric2seer): `.stream(..)` on a NEURepoFile
            //   java: InputStream stream = file.stream(..);
            let mut stream: crate::unresolved::InputStream = todo!("fabric2seer: the initialiser of `stream` (TODO above)");
            // java: try {..} catch (Exception ex) - an exception here is a panic, caught by `catch_unwind`
            let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                // TODO(fabric2seer): `.readAllBytes(..)` on a InputStream
                //   java: Dyes dyes = Dyes.CODEC.parse(JsonOps.INSTANCE, JsonParser.parseString(new String(stream.readAllBytes()))).getOrThrow();
                let mut dyes: Dyes = todo!("fabric2seer: the initialiser of `dyes` (TODO above)");
                compat::lock(&STATIC_DYES).extend(dyes.static_dyes.iter().map(|(k, v)| (k.clone(), v.clone())));
                compat::lock(&ANIMATED_DYES).extend(dyes.animated_dyes.iter().map(|(k, v)| (k.clone(), v.clone())));
                logger().info(&format!("[Skyblocker] Successfully loaded {} static dyes and {} animated dyes from repo.", (compat::lock(&STATIC_DYES).len() as i32), (compat::lock(&ANIMATED_DYES).len() as i32)));
            }));
            if let Err(ex) = __result {
                logger().info(&format!("[Skyblocker] Failed to load dyes from repo"));
            }
        }
    }

}

/// java: private record Dyes(Object2ObjectMap<String, List<Integer>> animatedDyes, Map<String, Integer> staticDyes)
#[derive(Clone, Debug, PartialEq)]
pub struct Dyes {
    /// java: Object2ObjectMap<String, List<Integer>> animatedDyes
    pub animated_dyes: std::collections::HashMap<String, Vec<i32>>,
    /// java: Map<String, Integer> staticDyes
    pub static_dyes: std::collections::HashMap<String, i32>,
}

impl Dyes {
    /// java: the canonical record constructor
    pub fn new(mut animated_dyes: std::collections::HashMap<String, Vec<i32>>, mut static_dyes: std::collections::HashMap<String, i32>) -> Self {
        Self { animated_dyes, static_dyes }
    }

    /// java: the record accessor `animatedDyes()`
    pub fn animated_dyes(&self) -> std::collections::HashMap<String, Vec<i32>> {
        self.animated_dyes.clone()
    }

    /// java: the record accessor `staticDyes()`
    pub fn static_dyes(&self) -> std::collections::HashMap<String, i32> {
        self.static_dyes.clone()
    }

}

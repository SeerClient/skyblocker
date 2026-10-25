//! Ported from de.hysky.skyblocker.skyblock.item.custom.CustomAnimatedHelmetTextures by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::enum_utils::EnumUtils;
use crate::de::hysky::skyblocker::utils::item_utils::ItemUtils;
use crate::de::hysky::skyblocker::utils::neu_repo_manager::NEURepoManager;
use mcrust::com::mojang::authlib::game_profile::GameProfile;
use mcrust::com::mojang::authlib::properties::property_map::PropertyMap;
use mcrust::com::mojang::serialization::codec;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::com::mojang::serialization::codecs::record_codec_builder;
use mcrust::com::mojang::serialization::data_result::DataResult;
use mcrust::java::util::uuid::Uuid;
use mcrust::net::minecraft::world::item::component::resolvable_profile::ResolvableProfile;
use mcrust::com::mojang::serialization::decoder::Decoder as _;

/// java: private static final Object2ObjectOpenHashMap<String, AnimatedHead> ANIMATED_HEADS = new Object2ObjectOpenHashMap<>();
pub static ANIMATED_HEADS: compat::MainThread<std::collections::HashMap<String, AnimatedHead>> = compat::MainThread::new(|| std::collections::HashMap::new());
/// java: private static final Object2ObjectOpenHashMap<AnimatedHead, AnimatedHeadStateTracker> STATE_TRACKERS = new Object2ObjectOpenHashMap<>();
/// Keyed by the record (its `equals`); the record holds profiles, which have no `Hash` here, so the map is a list
/// looked up with `==`.
pub static STATE_TRACKERS: compat::MainThread<Vec<(AnimatedHead, AnimatedHeadStateTracker)>> = compat::MainThread::new(|| Vec::new());
/// java: private static int ticks = 0;
pub static TICKS: LazyLock<Mutex<i32>> = LazyLock::new(|| Mutex::new(0));
/// java: private static final Codec<ResolvableProfile> FRAME_CODEC = Codec.STRING.xmap(AnimatedHead::fromString, AnimatedHead::toString);
pub static ANIMATED_HEAD_FRAME_CODEC: compat::MainThread<CodecRef<ResolvableProfile>> = compat::MainThread::new(|| codec::xmap(mcrust::com::mojang::serialization::codec::string_codec(), |__a0| AnimatedHead::from_string(&compat::jstring(&__a0)), |__a0| AnimatedHead::to_string(&__a0)));
/// java: private static final Codec<AnimatedHead> CODEC = RecordCodecBuilder.create(instance -> instance.group( Codec.INT.xmap(i -> Math.max(i, 1), Function.identity()).fieldOf("ticks").forGetter(AnimatedHead:
pub static ANIMATED_HEAD_CODEC: compat::MainThread<CodecRef<AnimatedHead>> = compat::MainThread::new(|| record_codec_builder::create(record_codec_builder::apply2(
    record_codec_builder::of(|o: &AnimatedHead| o.tick_threshold(), codec::field_of(codec::xmap(mcrust::com::mojang::serialization::codec::int_codec(), |i: i32| i.max(1), |i: &i32| *i), "ticks")),
    record_codec_builder::of(|o: &AnimatedHead| o.frames(), codec::field_of(codec::validate(codec::list(ANIMATED_HEAD_FRAME_CODEC.get().clone()), |frames: Vec<ResolvableProfile>| AnimatedHead::validate_frames(&frames)), "textures")),
    move |__a0, __a1| AnimatedHead::new(__a0.clone(), __a1.clone()),
)));
/// java: private static final Codec<Map<String, AnimatedHead>> MAP_CODEC = Codec.unboundedMap(Codec.STRING, CODEC);
/// (the port's `unboundedMap` codec answers the entries as ordered pairs)
pub static ANIMATED_HEAD_MAP_CODEC: compat::MainThread<CodecRef<Vec<(String, AnimatedHead)>>> = compat::MainThread::new(|| codec::unbounded_map(mcrust::com::mojang::serialization::codec::string_codec(), ANIMATED_HEAD_CODEC.get_mut().clone()));

/// java: public class CustomAnimatedHelmetTextures
#[derive(Clone)]
pub struct CustomAnimatedHelmetTextures {}

impl CustomAnimatedHelmetTextures {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        NEURepoManager::run_async_after_load(Arc::new(move || { CustomAnimatedHelmetTextures::load_animated_heads(); }));
        client_tick_events::END_CLIENT_TICK.register(Arc::new(|minecraft: &mut Minecraft| {
            compat::set_state(&TICKS, compat::get_state(&TICKS) + 1);
        }));
    }

    /// java: private static void loadAnimatedHeads()
    pub fn load_animated_heads() {
        {
            // java: try (InputStream stream = NEURepoManager.file("constants/animatedskulls.json").stream()) {..} catch (Exception e)
            // - the resource is opened inside the try, so a missing file (null) is caught too; an exception is a panic,
            // caught by `catch_unwind`
            let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let stream = NEURepoManager::file("constants/animatedskulls.json").expect("java.lang.NullPointerException: NEURepoManager.file(\"constants/animatedskulls.json\")");
                //   java: String data = new String(stream.readAllBytes());
                let mut data: String = compat::neu_repo_file_read_all(&stream);
                let mut animated_heads: std::collections::HashMap<String, AnimatedHead> = (ANIMATED_HEAD_MAP_CODEC.get_mut().parse(&mcrust::com::mojang::serialization::json_ops::JsonOps::instance(), &compat::json_to_port(&(compat::json::get_as_json_object(&(serde_json::from_str::<serde_json::Value>(&data).expect("JsonSyntaxException")))).get("skins").cloned().unwrap()))).get_or_throw().unwrap().into_iter().collect();
                crate::compat::execute(move || {
                    ANIMATED_HEADS.get_mut().clear();
                    ANIMATED_HEADS.get_mut().extend(animated_heads.iter().map(|(k, v)| (k.clone(), v.clone())));
                    STATE_TRACKERS.get_mut().clear();
                });
            }));
            if let Err(e) = __result {
                logger().error(&format!("[Skyblocker Custom Animated Helmet Textures] Failed to load animated heads file."));
            }
        }
    }

    /// java: public static Set<String> getAnimatedHeadIds()
    pub fn get_animated_head_ids() -> std::collections::HashSet<String> {
        return ANIMATED_HEADS.get_mut().keys().cloned().collect::<std::collections::HashSet<_>>();
    }

    /// java: public static String formatName(String id)
    pub fn format_name(id: &str) -> String {
        compat::capitalize_fully(&id.replace('_', " "))
    }

    /// java: public static @Nullable ResolvableProfile animateHeadTexture(String id)
    pub fn animate_head_texture(id: &str) -> Option<ResolvableProfile> {
        let mut head: Option<AnimatedHead> = ANIMATED_HEADS.get_mut().get(id).cloned();
        if let Some(head) = &head {
            // STATE_TRACKERS.computeIfAbsent(head, AnimatedHeadStateTracker::new)
            let trackers = STATE_TRACKERS.get_mut();
            let index = match trackers.iter().position(|(k, _)| k == head) {
                Some(index) => index,
                None => {
                    trackers.push((head.clone(), AnimatedHeadStateTracker::new(head.clone())));
                    trackers.len() - 1
                }
            };
            let tracker = &mut trackers[index].1;
            if tracker.last_recorded_tick == compat::get_state(&TICKS) {
                return Some(tracker.get_current_frame());
            }
            tracker.advance_to(compat::get_state(&TICKS));
            tracker.last_recorded_tick = compat::get_state(&TICKS);
            return Some(tracker.get_current_frame());
        }
        return None;
    }

}

/// java: private record AnimatedHead(int tickThreshold, List<ResolvableProfile> frames)
#[derive(Clone, Debug, PartialEq)]
pub struct AnimatedHead {
    /// java: int tickThreshold
    pub tick_threshold: i32,
    /// java: List<ResolvableProfile> frames
    pub frames: Vec<ResolvableProfile>,
}

impl AnimatedHead {
    /// java: the canonical record constructor
    pub fn new(mut tick_threshold: i32, mut frames: Vec<ResolvableProfile>) -> Self {
        Self { tick_threshold, frames }
    }

    /// java: the record accessor `tickThreshold()`
    pub fn tick_threshold(&self) -> i32 {
        self.tick_threshold
    }

    /// java: the record accessor `frames()`
    pub fn frames(&self) -> Vec<ResolvableProfile> {
        self.frames.clone()
    }

    /// java: private static ResolvableProfile fromString(String string)
    pub fn from_string(string: &str) -> ResolvableProfile {
        let mut split: Vec<String> = compat::regex::split(&string, ":");
        let mut uuid: Uuid = Uuid::from_string(&split[(0) as usize]).unwrap();
        let mut property_map: PropertyMap = ItemUtils::property_map_with_texture(&split[(1) as usize]);
        return ResolvableProfile::create_resolved(GameProfile::new(uuid.clone(), "custom", property_map.clone())).unwrap();
    }

    /// java: private static String toString(ResolvableProfile profile)
    pub fn to_string(profile: &ResolvableProfile) -> String {
        //   java: return profile.partialProfile().id() + ":" + profile.partialProfile().properties().get("textures").iterator().next().value();
        format!("{}:{}", profile.partial_profile().id(), profile.partial_profile().properties().get("textures").into_iter().next().expect("java.util.NoSuchElementException").value())
    }

    /// java: private static DataResult<List<ResolvableProfile>> validateFrames(List<ResolvableProfile> frames)
    pub fn validate_frames(frames: &Vec<ResolvableProfile>) -> DataResult<Vec<ResolvableProfile>> {
        return (if !frames.is_empty() { DataResult::success(frames.clone()) } else { DataResult::error(Arc::new(|| "Expected at least a single frame for an animated head but there was none!".to_string())) });
    }

}

/// java: private static class AnimatedHeadStateTracker
#[derive(Clone)]
pub struct AnimatedHeadStateTracker {
    /// java: private final AnimatedHead head;
    pub head: AnimatedHead,
    /// java: private int lastRecordedTick = 0;
    pub last_recorded_tick: i32,
    /// java: private ResolvableProfile currentFrame;
    pub current_frame: ResolvableProfile,
}

impl AnimatedHeadStateTracker {
    /// java: private AnimatedHeadStateTracker(AnimatedHead head)
    pub fn new(mut head: AnimatedHead) -> Self {
        let __init_head = head.clone();
        let __init_current_frame = head.frames()[0].clone();
        let mut this = Self {
            head: __init_head,
            last_recorded_tick: 0,
            current_frame: __init_current_frame,
        };
        this
    }

    /// java: private void advanceTo(int ticks)
    pub fn advance_to(&mut self, mut ticks: i32) {
        let mut advanced_index: i32 = ticks.wrapping_div(self.head.tick_threshold());
        self.current_frame = EnumUtils::cycle_2(&self.head.frames(), advanced_index);
    }

    /// java: private ResolvableProfile getCurrentFrame()
    pub fn get_current_frame(&self) -> ResolvableProfile {
        return self.current_frame.clone();
    }

}

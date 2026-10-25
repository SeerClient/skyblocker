//! Ported from de.hysky.skyblocker.skyblock.item.custom.CustomArmorAnimatedDyes by fabric2seer.

#[allow(unused_imports)]
use mcrust::net::minecraft::world::entity::living_entity::LivingEntity as _;
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::events::skyblock_events;
use crate::de::hysky::skyblocker::utils::ok_lab_color::OkLabColor;
use crate::de::hysky::skyblocker::utils::render::level_render_extraction_callback;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::com::mojang::brigadier::command_dispatcher::CommandDispatcher;
use mcrust::net::minecraft::commands::command_build_context::CommandBuildContext;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;

/// java: private static final Object2ObjectOpenHashMap<AnimatedDye, AnimatedDyeStateTracker> STATE_TRACKER_MAP = new Object2ObjectOpenHashMap<>();
/// Keyed by the record (its `equals`); `AnimatedDye` holds floats, which have no `Hash`, so the map is a list
/// looked up with `==`.
pub static STATE_TRACKER_MAP: LazyLock<Mutex<Vec<(AnimatedDye, AnimatedDyeStateTracker)>>> = LazyLock::new(|| Mutex::new(Vec::new()));
/// java: private static final float DEFAULT_DELAY = 0;
pub const DEFAULT_DELAY: f32 = ((0) as f32) as f32;
/// java: private static int frames;
pub static FRAMES: LazyLock<Mutex<i32>> = LazyLock::new(|| Mutex::new(0));

/// java: public class CustomArmorAnimatedDyes
#[derive(Clone)]
pub struct CustomArmorAnimatedDyes {}

impl CustomArmorAnimatedDyes {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        // java: ClientCommandRegistrationCallback.EVENT.register(CustomArmorAnimatedDyes::registerCommands);
        client_command_registration_callback::EVENT.register(Arc::new(
            |dispatcher: &mut client_command_registration_callback::ClientCommandDispatcher| {
                CustomArmorAnimatedDyes::register_commands(dispatcher.dispatcher(), dispatcher.build_context());
            },
        ));
        level_render_extraction_callback::EVENT.register(Arc::new(move |_| { compat::set_state(&FRAMES, compat::get_state(&FRAMES) + 1); }));
        skyblock_events::LOCATION_CHANGE.register(Arc::new(move |_| { CustomArmorAnimatedDyes::clean_trackers(); }));
    }

    /// java: private static void registerCommands(CommandDispatcher<FabricClientCommandSource> dispatcher, CommandBuildContext registryAccess)
    pub fn register_commands(dispatcher: &CommandDispatcher<crate::unresolved::FabricClientCommandSource>, registry_access: Option<&CommandBuildContext>) {
        //   java: dispatcher.register(literal(NAMESPACE).then(literal("custom").then(literal("animatedDye")
        //   java:     .executes(context -> customizeAnimatedDye(context.getSource(), Integer.MIN_VALUE, Integer.MIN_VALUE, 0, false, 0))
        //   java:     .then(argument("hex1", ColorArgumentType.hex()).then(argument("hex2", ColorArgumentType.hex()).then(argument("duration", FloatArgumentType.floatArg(0.1f, 10f))
        //   java:         .then(argument("cycleBack", BoolArgumentType.bool()).executes(.. DEFAULT_DELAY)
        //   java:             .then(argument("delay", FloatArgumentType.floatArg(0)).executes(.. getFloat(context, "delay")))))))));
        use mcrust::com::mojang::brigadier::builder::argument_builder::ArgumentBuilder as _;
        use mcrust::com::mojang::brigadier::arguments::bool_argument_type::BoolArgumentType;
        use mcrust::com::mojang::brigadier::arguments::float_argument_type::FloatArgumentType;
        use crate::de::hysky::skyblocker::utils::command::argumenttypes::color::color_argument_type::ColorArgumentType;
        type Ctx = mcrust::com::mojang::brigadier::context::command_context::CommandContext<crate::unresolved::FabricClientCommandSource>;
        let _ = registry_access;
        let mut delay = client_command_registration_callback::argument("delay", Arc::new(FloatArgumentType::float_arg_min(0.0)));
        delay.executes(Arc::new(|context: &Ctx| Ok(CustomArmorAnimatedDyes::customize_animated_dye(context.get_source(), ColorArgumentType::get_int_from_hex(context, "hex1"), ColorArgumentType::get_int_from_hex(context, "hex2"), FloatArgumentType::get_float(context, "duration"), BoolArgumentType::get_bool(context, "cycleBack"), FloatArgumentType::get_float(context, "delay")))));
        let mut cycle_back = client_command_registration_callback::argument("cycleBack", Arc::new(BoolArgumentType::bool()));
        cycle_back.executes(Arc::new(|context: &Ctx| Ok(CustomArmorAnimatedDyes::customize_animated_dye(context.get_source(), ColorArgumentType::get_int_from_hex(context, "hex1"), ColorArgumentType::get_int_from_hex(context, "hex2"), FloatArgumentType::get_float(context, "duration"), BoolArgumentType::get_bool(context, "cycleBack"), DEFAULT_DELAY))));
        cycle_back.then(&delay);
        let mut duration = client_command_registration_callback::argument("duration", Arc::new(FloatArgumentType::float_arg_range(0.1, 10.0)));
        duration.then(&cycle_back);
        let mut hex2 = client_command_registration_callback::argument("hex2", Arc::new(ColorArgumentType::hex()));
        hex2.then(&duration);
        let mut hex1 = client_command_registration_callback::argument("hex1", Arc::new(ColorArgumentType::hex()));
        hex1.then(&hex2);
        let mut animated_dye = client_command_registration_callback::literal("animatedDye");
        animated_dye.executes(Arc::new(|context: &Ctx| Ok(CustomArmorAnimatedDyes::customize_animated_dye(context.get_source(), i32::MIN, i32::MIN, 0.0, false, 0.0))));
        animated_dye.then(&hex1);
        let mut custom = client_command_registration_callback::literal("custom");
        custom.then(&animated_dye);
        let mut root = client_command_registration_callback::literal(crate::entry::skyblocker_mod::NAMESPACE);
        root.then(&custom);
        dispatcher.register(&root);
    }

    /// java: private static int customizeAnimatedDye(FabricClientCommandSource source, int color1, int color2, float duration, boolean cycleBack, float delay)
    pub fn customize_animated_dye(source: &crate::unresolved::FabricClientCommandSource, mut color1: i32, mut color2: i32, mut duration: f32, mut cycle_back: bool, mut delay: f32) -> i32 {
        // java: ItemStack heldItem = source.getPlayer().getMainHandItem(); (the source's player is the client's)
        let mut held_item: mcrust::net::minecraft::world::item::item_stack::ItemStack = mcrust::mcrust::modding::game::with_player(|player| player.get_main_hand_item().clone()).unwrap_or_else(mcrust::net::minecraft::world::item::item_stack::empty);
        if Utils::is_on_skyblock() && true /* java: held_item != null: never null here */ && !held_item.is_empty() {
            if held_item.is_in_tag(&mcrust::net::minecraft::tags::item_tags::cauldron_can_remove_dye()) {
                let mut item_uuid: String = held_item.get_uuid();
                if !item_uuid.is_empty() {
                    let custom_animated_dyes = &SkyblockerConfigManager::get().general.custom_animated_dyes;
                    if color1 == i32::MIN && color2 == i32::MIN {
                        if custom_animated_dyes.contains_key(&item_uuid) {
                            SkyblockerConfigManager::update(Arc::new(move |mut config| { config.general.custom_animated_dyes.remove(&item_uuid); }));
                            // java: source.sendFeedback(Constants.PREFIX.get().append(Component.translatable("skyblocker.customAnimatedDyes.removed")));
                            compat::batch_w_send_feedback(&(crate::de::hysky::skyblocker::utils::constants::PREFIX.get())().append(mcrust::net::minecraft::network::chat::component::translatable("skyblocker.customAnimatedDyes.removed")));
                        } else {
                            // java: source.sendError(Constants.PREFIX.get().append(Component.translatable("skyblocker.customAnimatedDyes.neverHad")));
                            compat::batch_w_send_error(&(crate::de::hysky::skyblocker::utils::constants::PREFIX.get())().append(mcrust::net::minecraft::network::chat::component::translatable("skyblocker.customAnimatedDyes.neverHad")));
                        }
                    } else {
                        let mut animated_dye: AnimatedDye = AnimatedDye::new(vec![Keyframe::new(color1, ((0) as f32) as f32), Keyframe::new(color2, ((1) as f32) as f32)], cycle_back, delay, duration);
                        SkyblockerConfigManager::update(Arc::new(move |mut config| { config.general.custom_animated_dyes.insert(item_uuid.clone(), animated_dye.clone()); }));
                        // java: source.sendFeedback(Constants.PREFIX.get().append(Component.translatable("skyblocker.customAnimatedDyes.added")));
                        compat::batch_w_send_feedback(&(crate::de::hysky::skyblocker::utils::constants::PREFIX.get())().append(mcrust::net::minecraft::network::chat::component::translatable("skyblocker.customAnimatedDyes.added")));
                    }
                } else {
                    // java: source.sendError(Constants.PREFIX.get().append(Component.translatable("skyblocker.customAnimatedDyes.noItemUuid").withColor(TextColor.RED)));
                    compat::batch_w_send_error(&(crate::de::hysky::skyblocker::utils::constants::PREFIX.get())().append(mcrust::net::minecraft::network::chat::component::translatable("skyblocker.customAnimatedDyes.noItemUuid").with_color(mcrust::net::minecraft::network::chat::text_color::TextColor::RED.get_value())));
                }
            } else {
                // java: source.sendError(Constants.PREFIX.get().append(Component.translatable("skyblocker.customAnimatedDyes.notDyeable").withColor(TextColor.RED)));
                compat::batch_w_send_error(&(crate::de::hysky::skyblocker::utils::constants::PREFIX.get())().append(mcrust::net::minecraft::network::chat::component::translatable("skyblocker.customAnimatedDyes.notDyeable").with_color(mcrust::net::minecraft::network::chat::text_color::TextColor::RED.get_value())));
            }
        } else {
            // java: source.sendError(Constants.PREFIX.get().append(Component.translatable("skyblocker.customAnimatedDyes.unableToSetDye").withColor(TextColor.RED)));
            compat::batch_w_send_error(&(crate::de::hysky::skyblocker::utils::constants::PREFIX.get())().append(mcrust::net::minecraft::network::chat::component::translatable("skyblocker.customAnimatedDyes.unableToSetDye").with_color(mcrust::net::minecraft::network::chat::text_color::TextColor::RED.get_value())));
        }
        return 1;
    }

    /// java: public static int animateColorTransition(AnimatedDye animatedDye)
    pub fn animate_color_transition(mut animated_dye: AnimatedDye) -> i32 {
        // AnimatedDyeStateTracker trackedState = STATE_TRACKER_MAP.computeIfAbsent(animatedDye, AnimatedDyeStateTracker::new);
        let delta_ticks = compat::minecraft().get_delta_tracker().get_game_time_delta_ticks();
        let mut map = compat::lock(&STATE_TRACKER_MAP);
        let index = match map.iter().position(|(dye, _)| *dye == animated_dye) {
            Some(index) => index,
            None => {
                map.push((animated_dye.clone(), AnimatedDyeStateTracker::new(animated_dye.clone())));
                map.len() - 1
            }
        };
        let tracked_state = &mut map[index].1;
        if tracked_state.last_recorded_frame == compat::get_state(&FRAMES) {
            return tracked_state.last_color;
        }
        tracked_state.last_recorded_frame = compat::get_state(&FRAMES);
        return tracked_state.interpolate(&animated_dye, delta_ticks);
    }

    /// java: public static void cleanTrackers()
    pub fn clean_trackers() {
        compat::lock(&STATE_TRACKER_MAP).clear();
    }

}

/// java: @VisibleForTesting static class AnimatedDyeStateTracker
#[derive(Clone)]
pub struct AnimatedDyeStateTracker {
    /// java: private float progress = 0;
    pub progress: f32,
    /// java: private boolean onBackCycle = false;
    pub on_back_cycle: bool,
    /// java: private int lastColor = 0;
    pub last_color: i32,
    /// java: private int lastRecordedFrame = 0;
    pub last_recorded_frame: i32,
}

impl AnimatedDyeStateTracker {
    /// java: @VisibleForTesting AnimatedDyeStateTracker(AnimatedDye animatedDye)
    pub fn new(mut animated_dye: AnimatedDye) -> Self {
        let mut this = Self {
            progress: ((0) as f32) as f32,
            on_back_cycle: false,
            last_color: 0,
            last_recorded_frame: 0,
        };
        if animated_dye.delay() > (0 as f32) {
            if animated_dye.cycle_back() {
                this.on_back_cycle = true;
                this.progress = (animated_dye.delay() / animated_dye.duration());
            } else {
                this.progress = ((1 as f32) - (animated_dye.delay() / animated_dye.duration()));
            }
            this.progress = AnimatedDyeStateTracker::clamp(this.progress);
        }
        this
    }

    /// java: private static float clamp(float progress)
    pub fn clamp(mut progress: f32) -> f32 {
        return progress.clamp((0 as f32), (1 as f32));
    }

    /// java: @VisibleForTesting int interpolate(AnimatedDye animatedDye, float deltaTicks)
    pub fn interpolate(&mut self, animated_dye: &AnimatedDye, mut delta_ticks: f32) -> i32 {
        self.update(animated_dye, delta_ticks);
        let mut keyframe: i32 = 0;
        while keyframe < (animated_dye.keyframes.len() as i32).wrapping_sub(2) && (animated_dye.keyframes[(keyframe.wrapping_add(1)) as usize]).time < self.progress {
            keyframe += 1;
        }
        let mut current: Keyframe = (if self.on_back_cycle { animated_dye.keyframes[(keyframe.wrapping_add(1)) as usize].clone() } else { animated_dye.keyframes[(keyframe) as usize].clone() });
        let mut next: Keyframe = (if self.on_back_cycle { animated_dye.keyframes[(keyframe) as usize].clone() } else { animated_dye.keyframes[(keyframe.wrapping_add(1)) as usize].clone() });
        let mut color_progress: f32 = ((self.progress - current.time) / (next.time - current.time));
        color_progress = AnimatedDyeStateTracker::clamp(color_progress);
        return { self.last_color = OkLabColor::interpolate(current.color, next.color, color_progress); self.last_color };
    }

    /// java: private void update(AnimatedDye animatedDye, float deltaTicks)
    pub fn update(&mut self, animated_dye: &AnimatedDye, mut delta_ticks: f32) {
        let mut v: f32 = ((delta_ticks * 0.05f32) / animated_dye.duration);
        if self.on_back_cycle {
            self.progress = (self.progress - v);
            if self.progress <= 0.0f32 {
                self.on_back_cycle = false;
                self.progress = self.progress.abs();
            }
        } else {
            self.progress = (self.progress + v);
            if self.progress >= 1.0f32 {
                if animated_dye.cycle_back {
                    self.on_back_cycle = true;
                    self.progress = (2.0f32 - self.progress);
                } else {
                    self.progress = (self.progress % 1.0f32);
                }
            }
        }
        self.progress = AnimatedDyeStateTracker::clamp(self.progress);
    }

}

/// java: public record Keyframe(int color, float time)
#[derive(Clone, Debug, PartialEq)]
pub struct Keyframe {
    /// java: int color
    pub color: i32,
    /// java: float time
    pub time: f32,
}

impl Keyframe {
    /// java: the canonical record constructor
    pub fn new(mut color: i32, mut time: f32) -> Self {
        Self { color, time }
    }

    /// java: the record accessor `color()`
    pub fn color(&self) -> i32 {
        self.color
    }

    /// java: the record accessor `time()`
    pub fn time(&self) -> f32 {
        self.time
    }

}

/// java: public record AnimatedDye(List<Keyframe> keyframes, boolean cycleBack, float delay, float duration)
#[derive(Clone, Debug, PartialEq)]
pub struct AnimatedDye {
    /// java: List<Keyframe> keyframes
    pub keyframes: Vec<Keyframe>,
    /// java: boolean cycleBack
    pub cycle_back: bool,
    /// java: float delay
    pub delay: f32,
    /// java: float duration
    pub duration: f32,
}

impl AnimatedDye {
    /// java: the canonical record constructor
    pub fn new(mut keyframes: Vec<Keyframe>, mut cycle_back: bool, mut delay: f32, mut duration: f32) -> Self {
        Self { keyframes, cycle_back, delay, duration }
    }

    /// java: the record accessor `keyframes()`
    pub fn keyframes(&self) -> Vec<Keyframe> {
        self.keyframes.clone()
    }

    /// java: the record accessor `cycleBack()`
    pub fn cycle_back(&self) -> bool {
        self.cycle_back
    }

    /// java: the record accessor `delay()`
    pub fn delay(&self) -> f32 {
        self.delay
    }

    /// java: the record accessor `duration()`
    pub fn duration(&self) -> f32 {
        self.duration
    }

}

// Generated Gson forms for src/de/hysky/skyblocker/skyblock/item/custom/custom_armor_animated_dyes.rs (fabric2seer round 24): APPEND to that file.
// Needs `serde_json` in Cargo.toml and the `compat` of this round (dandelion::compat::GsonValue, ClassOf, ...).

/// Keyframe's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl Keyframe {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("color".to_string(), dandelion::compat::GsonValue::int(*(&self.color) as i64)),
            ("time".to_string(), dandelion::compat::GsonValue::float(*(&self.time))),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<Keyframe> {
        let __o = __v.as_object()?;
        // no no-argument constructor: Gson's `Unsafe.allocateInstance` - every field zero, then each key present set
        let mut __this = Keyframe { color: 0, time: 0.0 };
        if let Some(__x) = __o.get("color").and_then(dandelion::compat::gson_i64) { __this.color = __x as i32; }
        if let Some(__x) = __o.get("time").and_then(dandelion::compat::gson_f64) { __this.time = __x as f32; }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `Keyframe.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<Keyframe> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.skyblock.item.custom.CustomArmorAnimatedDyes$Keyframe",
            new: || panic!("InstantiationException: no no-argument constructor"),
            to_gson: |__x| __x.__gson_write(),
            read_gson: Keyframe::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// AnimatedDye's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl AnimatedDye {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("keyframes".to_string(), dandelion::compat::GsonValue::Arr(((&self.keyframes)).iter().map(|__x0| (__x0).__gson_write()).collect())),
            ("cycleBack".to_string(), dandelion::compat::GsonValue::Bool(*(&self.cycle_back))),
            ("delay".to_string(), dandelion::compat::GsonValue::float(*(&self.delay))),
            ("duration".to_string(), dandelion::compat::GsonValue::float(*(&self.duration))),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<AnimatedDye> {
        let __o = __v.as_object()?;
        // no no-argument constructor: Gson's `Unsafe.allocateInstance` - every field zero/null (an absent list is an
        // empty one here), then each key present set
        let mut __this = AnimatedDye { keyframes: Vec::new(), cycle_back: false, delay: 0.0, duration: 0.0 };
        if let Some(__x) = __o.get("keyframes").and_then(|__v| __v.as_array()) { __this.keyframes = __x.iter().filter_map(Keyframe::__gson_read).collect(); }
        if let Some(__x) = __o.get("cycleBack").and_then(dandelion::compat::gson_bool) { __this.cycle_back = __x; }
        if let Some(__x) = __o.get("delay").and_then(dandelion::compat::gson_f64) { __this.delay = __x as f32; }
        if let Some(__x) = __o.get("duration").and_then(dandelion::compat::gson_f64) { __this.duration = __x as f32; }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "cycleBack" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.cycle_back = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `AnimatedDye.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<AnimatedDye> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.skyblock.item.custom.CustomArmorAnimatedDyes$AnimatedDye",
            new: || panic!("InstantiationException: no no-argument constructor"),
            to_gson: |__x| __x.__gson_write(),
            read_gson: AnimatedDye::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

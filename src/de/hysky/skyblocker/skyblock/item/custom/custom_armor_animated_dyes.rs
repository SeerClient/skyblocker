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
        client_command_registration_callback::EVENT.register(Arc::new(
            |dispatcher: &mut client_command_registration_callback::ClientCommandDispatcher| {
                // TODO(fabric2seer): a command tree this tool cannot read (arguments, redirects, requires) - SeerClient commands receive the words after the name in `ctx.args`
                //   java: dispatcher.register(literal(SkyblockerMod.NAMESPACE)
                //   java: 				.then(literal("custom")
                //   java: 						.then(literal("animatedDye")
                //   java: 								.executes(context -> customizeAnimatedDye(context.getSource(), Integer.MIN_VALUE, Integer.MIN_VALUE, 0, false, 0))
                //   java: 								.then(argument("hex1", ColorArgumentType.hex())
                //   java: 										.then(argument("hex2", ColorArgumentType.hex())
                //   java: 												.then(argument("duration", FloatArgumentType.floatArg(0.1f, 10f))
                //   java: 														.then(argument("cycleBack", BoolArgumentType.bool())
                //   java: 																.executes(context -> customizeAnimatedDye(context.getSource(), ColorArgumentType.getIntFromHex(context, "hex1"), ColorArgumentType.getIntFromHex(context, "hex2"), FloatArgumentType.getFloat(context, "duration"), BoolArgumentType.getBool(context, "cycleBack"), DEFAULT_DELAY))
                //   java: 																.then(argument("delay", FloatArgumentType.floatArg(0))
                //   java: 																		.executes(context -> customizeAnimatedDye(context.getSource(), ColorArgumentType.getIntFromHex(context, "hex1"), ColorArgumentType.getIntFromHex(context, "hex2"), FloatArgumentType.getFloat(context, "duration"), BoolArgumentType.getBool(context, "cycleBack"), FloatArgumentType.getFloat(context, "delay")))))))))));
            },
        ));
        level_render_extraction_callback::EVENT.register(Arc::new(move |_| { compat::set_state(&FRAMES, compat::get_state(&FRAMES) + 1); }));
        skyblock_events::LOCATION_CHANGE.register(Arc::new(move |_| { CustomArmorAnimatedDyes::clean_trackers(); }));
    }

    /// java: private static void registerCommands(CommandDispatcher<FabricClientCommandSource> dispatcher, CommandBuildContext registryAccess)
    pub fn register_commands(dispatcher: &CommandDispatcher<crate::unresolved::FabricClientCommandSource>, registry_access: &CommandBuildContext) {
        // TODO(fabric2seer): a command builder outside dispatcher.register(..)
        //   java: dispatcher.register(literal(SkyblockerMod.NAMESPACE)
        //   java: 				.then(literal("custom")
        //   java: 						.then(literal("animatedDye")
        //   java: 								.executes(context -> customizeAnimatedDye(context.getSource(), Integer.MIN_VALUE, Integer.MIN_VALUE, 0, false, 0))
        //   java: 								.then(argument("hex1", ColorArgumentType.hex())
        //   java: 										.then(argument("hex2", ColorArgumentType.hex())
        //   java: 												.then(argument("duration", FloatArgumentType.floatArg(0.1f, 10f))
        //   java: 														.then(argument("cycleBack", BoolArgumentType.bool())
        //   java: 																.executes(context -> customizeAnimatedDye(context.getSource(), ColorArgumentType.getIntFromHex(context, "hex1"), ColorArgumentType.getIntFromHex(context, "hex2"), FloatArgumentType.getFloat(context, "duration"), BoolArgumentType.getBool(context, "cycleBack"), DEFAULT_DELAY))
        //   java: 																.then(argument("delay", FloatArgumentType.floatArg(0))
        //   java: 																		.executes(context -> customizeAnimatedDye(context.getSource(), ColorArgumentType.getIntFromHex(context, "hex1"), ColorArgumentType.getIntFromHex(context, "hex2"), FloatArgumentType.getFloat(context, "duration"), BoolArgumentType.getBool(context, "cycleBack"), FloatArgumentType.getFloat(context, "delay")))))))))));
    }

    /// java: private static int customizeAnimatedDye(FabricClientCommandSource source, int color1, int color2, float duration, boolean cycleBack, float delay)
    pub fn customize_animated_dye(source: &crate::unresolved::FabricClientCommandSource, mut color1: i32, mut color2: i32, mut duration: f32, mut cycle_back: bool, mut delay: f32) -> i32 {
        // TODO(fabric2seer): `.getPlayer(..)` on a FabricClientCommandSource
        //   java: ItemStack heldItem = source.getPlayer().getMainHandItem();
        let mut held_item: mcrust::net::minecraft::world::item::item_stack::ItemStack = mcrust::mcrust::modding::game::with_player(|player| player.get_main_hand_item().clone()).unwrap_or_else(mcrust::net::minecraft::world::item::item_stack::empty);
        if Utils::is_on_skyblock() && true /* java: held_item != null: never null here */ && !held_item.is_empty() {
            if held_item.is_in_tag(&mcrust::net::minecraft::tags::item_tags::cauldron_can_remove_dye()) {
                let mut item_uuid: String = held_item.get_uuid();
                if !item_uuid.is_empty() {
                    let custom_animated_dyes = &SkyblockerConfigManager::get().general.custom_animated_dyes;
                    if color1 == i32::MIN && color2 == i32::MIN {
                        if custom_animated_dyes.contains_key(&item_uuid) {
                            SkyblockerConfigManager::update(Arc::new(move |mut config| { config.general.custom_animated_dyes.remove(&item_uuid); }));
                            // TODO(fabric2seer): `.sendFeedback(..)` on a FabricClientCommandSource
                            //   java: source.sendFeedback(Constants.PREFIX.get().append(Component.translatable("skyblocker.customAnimatedDyes.removed")));
                        } else {
                            // TODO(fabric2seer): `.sendError(..)` on a FabricClientCommandSource
                            //   java: source.sendError(Constants.PREFIX.get().append(Component.translatable("skyblocker.customAnimatedDyes.neverHad")));
                        }
                    } else {
                        let mut animated_dye: AnimatedDye = AnimatedDye::new(vec![Keyframe::new(color1, ((0) as f32) as f32), Keyframe::new(color2, ((1) as f32) as f32)], cycle_back, delay, duration);
                        SkyblockerConfigManager::update(Arc::new(move |mut config| { config.general.custom_animated_dyes.insert(item_uuid.clone(), animated_dye.clone()); }));
                        // TODO(fabric2seer): `.sendFeedback(..)` on a FabricClientCommandSource
                        //   java: source.sendFeedback(Constants.PREFIX.get().append(Component.translatable("skyblocker.customAnimatedDyes.added")));
                    }
                } else {
                    // TODO(fabric2seer): `.sendError(..)` on a FabricClientCommandSource
                    //   java: source.sendError(Constants.PREFIX.get().append(Component.translatable("skyblocker.customAnimatedDyes.noItemUuid").withColor(TextColor.RED)));
                }
            } else {
                // TODO(fabric2seer): `.sendError(..)` on a FabricClientCommandSource
                //   java: source.sendError(Constants.PREFIX.get().append(Component.translatable("skyblocker.customAnimatedDyes.notDyeable").withColor(TextColor.RED)));
            }
        } else {
            // TODO(fabric2seer): `.sendError(..)` on a FabricClientCommandSource
            //   java: source.sendError(Constants.PREFIX.get().append(Component.translatable("skyblocker.customAnimatedDyes.unableToSetDye").withColor(TextColor.RED)));
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

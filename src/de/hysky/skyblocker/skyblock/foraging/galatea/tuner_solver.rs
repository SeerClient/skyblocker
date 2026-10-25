//! Ported from de.hysky.skyblocker.skyblock.foraging.galatea.TunerSolver by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::events::play_sound_events;
use crate::de::hysky::skyblocker::skyblock::item::slottext::slot_text;
use crate::de::hysky::skyblocker::skyblock::item::slottext::slot_text::SlotText;
use crate::de::hysky::skyblocker::utils::container::container_matcher::ContainerMatcher;
use crate::de::hysky::skyblocker::utils::container::container_solver::ContainerSolver;
use crate::de::hysky::skyblocker::utils::container::container_solver_manager::ContainerSolverManager;
use crate::de::hysky::skyblocker::utils::container::regex_container_matcher::RegexContainerMatcher;
use crate::de::hysky::skyblocker::utils::container::regex_container_matcher::RegexContainerMatcherBase;
use crate::de::hysky::skyblocker::utils::container::simple_container_solver::SimpleContainerSolver;
use crate::de::hysky::skyblocker::utils::container::simple_container_solver::SimpleContainerSolverBase;
use crate::de::hysky::skyblocker::utils::container::slot_text_adder::SlotTextAdder;
use crate::de::hysky::skyblocker::utils::render::gui::color_highlight::ColorHighlight;
use crate::de::hysky::skyblocker::utils::resettable::Resettable;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::AbstractContainerScreen;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::protocol::game::clientbound_sound_packet::ClientboundSoundPacket;
use mcrust::net::minecraft::world::inventory::slot::Slot;
use mcrust::net::minecraft::world::item::item::Item;
use mcrust::net::minecraft::world::item::items;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: public static final TunerSolver INSTANCE = new TunerSolver();
pub static INSTANCE: compat::MainThread<compat::Shared<TunerSolver>> = compat::MainThread::new(|| TunerSolver::new());
/// java: private static final Item[] COLOR_CYCLE = { Items.DYE.magenta(), Items.DYE.lightBlue(), Items.DYE.yellow(), Items.DYE.lime(), Items.DYE.pink(), Items.DYE.cyan(), Items.DYE.purple(), Items.LAPIS_LAZULI
pub static COLOR_CYCLE: compat::MainThread<Vec<Arc<Item>>> = compat::MainThread::new(|| vec![mcrust::net::minecraft::world::item::items::dye().magenta().clone(), mcrust::net::minecraft::world::item::items::dye().light_blue().clone(), mcrust::net::minecraft::world::item::items::dye().yellow().clone(), mcrust::net::minecraft::world::item::items::dye().lime().clone(), mcrust::net::minecraft::world::item::items::dye().pink().clone(), mcrust::net::minecraft::world::item::items::dye().cyan().clone(), mcrust::net::minecraft::world::item::items::dye().purple().clone(), items::items().get("lapis_lazuli"), items::items().get("cocoa_beans"), mcrust::net::minecraft::world::item::items::dye().green().clone(), mcrust::net::minecraft::world::item::items::dye().red().clone(), items::items().get("bone_meal"), mcrust::net::minecraft::world::item::items::dye().orange().clone()]);
/// java: private static final Item[] GLASS_CYCLE = { Items.STAINED_GLASS_PANE.magenta(), Items.STAINED_GLASS_PANE.lightBlue(), Items.STAINED_GLASS_PANE.yellow(), Items.STAINED_GLASS_PANE.lime(), Items.STAINED_
pub static GLASS_CYCLE: compat::MainThread<Vec<Arc<Item>>> = compat::MainThread::new(|| vec![mcrust::net::minecraft::world::item::items::stained_glass_pane().magenta().clone(), mcrust::net::minecraft::world::item::items::stained_glass_pane().light_blue().clone(), mcrust::net::minecraft::world::item::items::stained_glass_pane().yellow().clone(), mcrust::net::minecraft::world::item::items::stained_glass_pane().lime().clone(), mcrust::net::minecraft::world::item::items::stained_glass_pane().pink().clone(), mcrust::net::minecraft::world::item::items::stained_glass_pane().cyan().clone(), mcrust::net::minecraft::world::item::items::stained_glass_pane().purple().clone(), mcrust::net::minecraft::world::item::items::stained_glass_pane().blue().clone(), mcrust::net::minecraft::world::item::items::stained_glass_pane().brown().clone(), mcrust::net::minecraft::world::item::items::stained_glass_pane().green().clone(), mcrust::net::minecraft::world::item::items::stained_glass_pane().red().clone(), mcrust::net::minecraft::world::item::items::stained_glass_pane().white().clone(), mcrust::net::minecraft::world::item::items::stained_glass_pane().orange().clone()]);
/// java: private static final String[] PITCH_CYCLE = {"Low", "Normal", "High"};
pub static PITCH_CYCLE: LazyLock<Vec<String>> = LazyLock::new(|| vec!["Low".to_string(), "Normal".to_string(), "High".to_string()]);
/// java: private static final float[] PITCH_VALUES = {0.0952381f, 0.7936508f, 1.4920635f};
pub static PITCH_VALUES: LazyLock<Vec<f32>> = LazyLock::new(|| vec![0.0952381f32, 0.7936508f32, 1.4920635f32]);
/// java: private static final int[] SPEED_CYCLE = {1, 2, 3, 4, 5};
pub static SPEED_CYCLE: LazyLock<Vec<i32>> = LazyLock::new(|| vec![1, 2, 3, 4, 5]);
/// java: private static final int[][] SPEED_RANGES = { {50, 64}, // Speed 1 {40, 49}, // Speed 2 {30, 39}, // Speed 3 {20, 29}, // Speed 4 {10, 19} // Speed 5 };
pub static SPEED_RANGES: LazyLock<Vec<Vec<i32>>> = LazyLock::new(|| vec![vec![50, 64], vec![40, 49], vec![30, 39], vec![20, 29], vec![10, 19]]);
/// java: private static final int MAX_PITCH_SAMPLES = 5;
pub const MAX_PITCH_SAMPLES: i32 = 5;

/// java: public class TunerSolver extends SimpleContainerSolver implements SlotTextAdder
#[derive(Clone)]
pub struct TunerSolver {
    /// java: extends SimpleContainerSolver
    pub base: SimpleContainerSolverBase,
    /// java: private int colorClicks = 0;
    pub color_clicks: i32,
    /// java: private int speedClicks = 0;
    pub speed_clicks: i32,
    /// java: private int pitchClicks = 0;
    pub pitch_clicks: i32,
    /// java: private boolean colorSolved = false;
    pub color_solved: bool,
    /// java: private boolean speedSolved = false;
    pub speed_solved: bool,
    /// java: private boolean pitchSolved = false;
    pub pitch_solved: bool,
    /// java: private boolean hasProcessed = false;
    pub has_processed: bool,
    /// java: private boolean isInMenu = false;
    pub is_in_menu: bool,
    /// java: private @Nullable String currentPitch = null;
    pub current_pitch: Option<String>,
    /// java: private final List<Float> recentPitches = new ArrayList<>();
    pub recent_pitches: Vec<f32>,
    /// java: private int lastTargetSlot = -1;
    pub last_target_slot: i32,
    /// java: private int ticksSinceLastMove = 0;
    pub ticks_since_last_move: i32,
    /// java: private int targetSpeed = -1;
    pub target_speed: i32,
    /// java: private int lastSpeedTicks = 0;
    pub last_speed_ticks: i32,
}

thread_local! {
    /// The solver whose `start` registered the open screen's `afterTick` / `remove` listeners.
    static TUNER_SCREEN_LISTENER: std::cell::RefCell<Option<compat::Shared<TunerSolver>>> = const { std::cell::RefCell::new(None) };
}

impl std::ops::Deref for TunerSolver {
    type Target = SimpleContainerSolverBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for TunerSolver {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl TunerSolver {
    /// java: private TunerSolver()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = SimpleContainerSolverBase::new_in("^Tune Frequency$".to_string(), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                color_clicks: 0,
                speed_clicks: 0,
                pitch_clicks: 0,
                color_solved: false,
                speed_solved: false,
                pitch_solved: false,
                has_processed: false,
                is_in_menu: false,
                current_pitch: None,
                recent_pitches: Vec::new(),
                last_target_slot: -1,
                ticks_since_last_move: 0,
                target_speed: -1,
                last_speed_ticks: 0,
            });
            ((__h.clone() as compat::Shared<dyn SimpleContainerSolver>), __h)
        });
        let __this_1 = compat::RenderThread::new(__this.clone());
        play_sound_events::FROM_SERVER.register(Arc::new(move |__a0| { TunerSolver::on_sound(&(*__this_1.get()), &__a0); }));
        __this
    }

    /// java: private static int updateClicks(int clicks, int cycleLength, int delta)
    pub fn update_clicks(mut clicks: i32, mut cycle_length: i32, mut delta: i32) -> i32 {
        let mut forward: i32 = (if clicks >= 0 { clicks } else { cycle_length.wrapping_add(clicks) });
        forward = ((forward.wrapping_add(delta)).wrapping_add(cycle_length)).wrapping_rem(cycle_length);
        let mut backward: i32 = cycle_length.wrapping_sub(forward);
        return (if forward <= backward { forward } else { (-backward) });
    }

    /// java: private static int computeColorClicks(Int2ObjectMap<ItemStack> slots)
    pub fn compute_color_clicks(slots: &std::collections::BTreeMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> i32 {
        let mut dye_stack: Option<mcrust::net::minecraft::world::item::item_stack::ItemStack> = slots.get(&46).cloned();
        if dye_stack.is_none() || dye_stack.as_ref().unwrap().is_empty() {
            logger().warn("No dye found in slot 46");
            return 0;
        }
        let mut dye_item: Arc<Item> = dye_stack.as_ref().unwrap().get_item();
        let mut dye_index: i32 = TunerSolver::get_color_index(&dye_item, COLOR_CYCLE.get_mut());
        if dye_index == -1 {
            logger().warn(&format!("Invalid dye item in slot 46: {}", compat::jstring(&dye_item)));
            return 0;
        }
        let mut moving_pane: Option<mcrust::net::minecraft::world::item::item_stack::ItemStack> = None;
        let mut moving_slot: i32 = -1;
        for slot in 28..=34 {
            let mut stack: Option<mcrust::net::minecraft::world::item::item_stack::ItemStack> = slots.get(&slot).cloned();
            if stack.is_some() && TunerSolver::is_stained_glass_pane(&stack.as_ref().unwrap().get_item()) {
                moving_pane = stack.clone();
                moving_slot = slot;
                break;
            }
        }
        if let Some(moving_pane) = &moving_pane {
        } else {
            logger().warn("No stained glass pane found in slots 28–34");
            return 0;
        }
        let mut moving_item: Arc<Item> = moving_pane.as_ref().unwrap().get_item();
        let mut moving_index: i32 = TunerSolver::get_color_index(&moving_item, GLASS_CYCLE.get_mut());
        if moving_index == -1 {
            logger().warn(&format!("Invalid glass pane item in slot {}: {}", moving_slot, compat::jstring(&moving_item)));
            return 0;
        }
        let mut target_pane: Option<mcrust::net::minecraft::world::item::item_stack::ItemStack> = None;
        let mut target_slot: i32 = -1;
        for slot in 10..=16 {
            let mut stack: Option<mcrust::net::minecraft::world::item::item_stack::ItemStack> = slots.get(&slot).cloned();
            if stack.is_some() && TunerSolver::is_stained_glass_pane(&stack.as_ref().unwrap().get_item()) {
                target_pane = stack.clone();
                target_slot = slot;
                break;
            }
        }
        if let Some(target_pane) = &target_pane {
        } else {
            logger().warn("No stained glass pane found in slots 10–16");
            return 0;
        }
        let mut target_item: Arc<Item> = target_pane.as_ref().unwrap().get_item();
        let mut target_index: i32 = TunerSolver::get_color_index(&target_item, GLASS_CYCLE.get_mut());
        if target_index == -1 {
            logger().warn(&format!("Invalid glass pane item in slot {}: {}", target_slot, compat::jstring(&target_item)));
            return 0;
        }
        let mut clicks: i32 = TunerSolver::calculate_clicks(dye_index, target_index);
        logger().info(&format!("Color solved: Dye={}, Target={}, Required clicks={}", compat::component_of(dye_stack.as_ref().unwrap().get_hover_name()).get_string(), compat::component_of(target_pane.as_ref().unwrap().get_hover_name()).get_string(), (if clicks >= 0 { format!("+{}", clicks) } else { clicks.to_string() })));
        return clicks;
    }

    /// java: private static int readCurrentSpeed(Int2ObjectMap<ItemStack> slots)
    pub fn read_current_speed(slots: &std::collections::BTreeMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> i32 {
        let mut speed_stack: Option<mcrust::net::minecraft::world::item::item_stack::ItemStack> = slots.get(&48).cloned();
        if speed_stack.is_some() && !(speed_stack.as_ref().unwrap().is_empty()) {
            let mut lore: Vec<String> = speed_stack.as_ref().unwrap().skyblocker_get_lore_strings();
            if (lore.len() as i32) >= 4 {
                // java: try { .. Integer.parseInt(parts[1].trim()) .. } catch (NumberFormatException _) {} - only the
                // parse's failure is caught (an index out of bounds still throws)
                {
                    let mut speed_text: String = lore[(3) as usize].clone();
                    let mut parts: Vec<String> = compat::regex::split(&speed_text, ": ");
                    if let Ok(current_speed) = (compat::trim(&(parts[(1) as usize]))).parse::<i32>() {
                        if current_speed >= 1 && current_speed <= 5 {
                            return current_speed;
                        }
                    }
                }
            }
        }
        return 0;
    }

    /// java: private static @Nullable String readCurrentPitch(Int2ObjectMap<ItemStack> slots)
    pub fn read_current_pitch(slots: &std::collections::BTreeMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> Option<String> {
        let mut pitch_stack: Option<mcrust::net::minecraft::world::item::item_stack::ItemStack> = slots.get(&50).cloned();
        if pitch_stack.is_some() && !(pitch_stack.as_ref().unwrap().is_empty()) {
            let mut lore: Vec<String> = pitch_stack.as_ref().unwrap().skyblocker_get_lore_strings();
            if (lore.len() as i32) >= 3 {
                let mut pitch_text: String = lore[(2) as usize].clone();
                if pitch_text.contains("Low") {
                    return Some("Low".to_string());
                }
                if pitch_text.contains("Normal") {
                    return Some("Normal".to_string());
                }
                if pitch_text.contains("High") {
                    return Some("High".to_string());
                }
            }
        }
        return None;
    }

    /// java: private static int getSpeedIndex(int speed)
    pub fn get_speed_index(mut speed: i32) -> i32 {
        for i in 0..(SPEED_CYCLE.len() as i32) {
            if SPEED_CYCLE[(i) as usize] == speed {
                return i;
            }
        }
        return -1;
    }

    /// java: private static int calculateSpeedClicks(int fromIndex, int toIndex)
    pub fn calculate_speed_clicks(mut from_index: i32, mut to_index: i32) -> i32 {
        let mut forward: i32 = ((to_index.wrapping_sub(from_index)).wrapping_add(SPEED_CYCLE.len() as i32)).wrapping_rem(SPEED_CYCLE.len() as i32);
        let mut backward: i32 = ((from_index.wrapping_sub(to_index)).wrapping_add(SPEED_CYCLE.len() as i32)).wrapping_rem(SPEED_CYCLE.len() as i32);
        return (if forward <= backward { forward } else { (-backward) });
    }

    /// java: private static float getPitchValue(String pitch)
    pub fn get_pitch_value(pitch: &str) -> f32 {
        for i in 0..(PITCH_CYCLE.len() as i32) {
            if PITCH_CYCLE[(i) as usize] == pitch {
                return PITCH_VALUES[(i) as usize];
            }
        }
        return 0.0f32;
    }

    /// java: private static @Nullable String getPitchName(float pitch)
    pub fn get_pitch_name(mut pitch: f32) -> Option<String> {
        for i in 0..(PITCH_VALUES.len() as i32) {
            if (pitch - PITCH_VALUES[(i) as usize]).abs() < 0.0001f32 {
                return Some(PITCH_CYCLE[(i) as usize].clone());
            }
        }
        return None;
    }

    /// java: private static int getPitchIndex(String pitch)
    pub fn get_pitch_index(pitch: &str) -> i32 {
        for i in 0..(PITCH_CYCLE.len() as i32) {
            if PITCH_CYCLE[(i) as usize] == pitch {
                return i;
            }
        }
        return -1;
    }

    /// java: private static int calculatePitchClicks(int fromIndex, int toIndex)
    pub fn calculate_pitch_clicks(mut from_index: i32, mut to_index: i32) -> i32 {
        let mut forward: i32 = ((to_index.wrapping_sub(from_index)).wrapping_add(PITCH_CYCLE.len() as i32)).wrapping_rem(PITCH_CYCLE.len() as i32);
        let mut backward: i32 = ((from_index.wrapping_sub(to_index)).wrapping_add(PITCH_CYCLE.len() as i32)).wrapping_rem(PITCH_CYCLE.len() as i32);
        return (if forward <= backward { forward } else { (-backward) });
    }

    /// java: private static boolean isDye(Item item)
    pub fn is_dye(item: &Arc<Item>) -> bool {
        for dye in COLOR_CYCLE.get_mut().iter() {
            if std::ptr::addr_eq(std::sync::Arc::as_ptr(item), std::sync::Arc::as_ptr(dye)) {
                return true;
            }
        }
        return false;
    }

    /// java: private static boolean isStainedGlassPane(Item item)
    pub fn is_stained_glass_pane(item: &Arc<Item>) -> bool {
        for glass in GLASS_CYCLE.get_mut().iter() {
            if std::ptr::addr_eq(std::sync::Arc::as_ptr(item), std::sync::Arc::as_ptr(glass)) {
                return true;
            }
        }
        return false;
    }

    /// java: private static int getColorIndex(Item item, Item[] cycle)
    pub fn get_color_index(item: &Arc<Item>, cycle: &Vec<Arc<Item>>) -> i32 {
        for i in 0..(cycle.len() as i32) {
            if std::ptr::addr_eq(std::sync::Arc::as_ptr(&cycle[(i) as usize]), std::sync::Arc::as_ptr(item)) {
                return i;
            }
        }
        return -1;
    }

    /// java: private static int calculateClicks(int fromIndex, int toIndex)
    pub fn calculate_clicks(mut from_index: i32, mut to_index: i32) -> i32 {
        let mut forward: i32 = ((to_index.wrapping_sub(from_index)).wrapping_add(COLOR_CYCLE.get_mut().len() as i32)).wrapping_rem(COLOR_CYCLE.get_mut().len() as i32);
        let mut backward: i32 = ((from_index.wrapping_sub(to_index)).wrapping_add(COLOR_CYCLE.get_mut().len() as i32)).wrapping_rem(COLOR_CYCLE.get_mut().len() as i32);
        return (if forward <= backward { forward } else { (-backward) });
    }

    /// java: private void resetState()
    pub fn reset_state(__this: &compat::Shared<Self>) {
        { let __v = false; compat::write(__this, |__t| __t.has_processed = __v); }
        { let __v = false; compat::write(__this, |__t| __t.is_in_menu = __v); }
        { let __v = false; compat::write(__this, |__t| __t.color_solved = __v); }
        { let __v = false; compat::write(__this, |__t| __t.speed_solved = __v); }
        { let __v = false; compat::write(__this, |__t| __t.pitch_solved = __v); }
        { let __v = 0; compat::write(__this, |__t| __t.color_clicks = __v); }
        { let __v = 0; compat::write(__this, |__t| __t.speed_clicks = __v); }
        { let __v = 0; compat::write(__this, |__t| __t.pitch_clicks = __v); }
        { let __v = None; compat::write(__this, |__t| __t.current_pitch = __v); }
        __this.lock().borrow_mut().recent_pitches.clear();
        { let __v = -1; compat::write(__this, |__t| __t.last_target_slot = __v); }
        { let __v = 0; compat::write(__this, |__t| __t.ticks_since_last_move = __v); }
        { let __v = -1; compat::write(__this, |__t| __t.target_speed = __v); }
        { let __v = 0; compat::write(__this, |__t| __t.last_speed_ticks = __v); }
    }

    /// java: private void trackTargetPaneMovement(List<Slot> slots)
    pub fn track_target_pane_movement(__this: &compat::Shared<Self>, slots: &Vec<Slot>) {
        let mut current_target_slot: i32 = -1;
        for slot in 10..=16 {
            // java: ItemStack stack = slots.get(slot).getItem(); (read through the open menu's containers)
            let mut stack: mcrust::net::minecraft::world::item::item_stack::ItemStack = compat::slot_item(&slots[(slot) as usize]);
            if true /* java: stack != null: never null here */ && TunerSolver::is_stained_glass_pane(&stack.get_item()) {
                current_target_slot = slot;
                break;
            }
        }
        if current_target_slot != compat::read(__this, |__t| __t.last_target_slot) && compat::read(__this, |__t| __t.last_target_slot) != -1 {
            let mut ticks: i32 = compat::read(__this, |__t| __t.ticks_since_last_move);
            { let __v = -1; compat::write(__this, |__t| __t.target_speed = __v); }
            for i in 0..(SPEED_RANGES.len() as i32) {
                if ticks >= (SPEED_RANGES[(i) as usize])[(0) as usize] && ticks <= (SPEED_RANGES[(i) as usize])[(1) as usize] {
                    { let __v = SPEED_CYCLE[(i) as usize]; compat::write(__this, |__t| __t.target_speed = __v); }
                    break;
                }
            }
            if compat::read(__this, |__t| __t.target_speed) == -1 {
                logger().warn(&format!("Tick interval {} does not match any speed range", ticks));
            }
            { let __v = ticks; compat::write(__this, |__t| __t.last_speed_ticks = __v); }
            { let __v = 0; compat::write(__this, |__t| __t.ticks_since_last_move = __v); }
            if !(compat::read(__this, |__t| __t.speed_solved)) {
                TunerSolver::maybe_solve_speed(__this, &ContainerSolverManager::slot_map(slots));
            }
        } else if current_target_slot != -1 {
            { let __v = compat::read(__this, |__t| __t.ticks_since_last_move) + 1; compat::write(__this, |__t| __t.ticks_since_last_move = __v); }
        }
        { let __v = current_target_slot; compat::write(__this, |__t| __t.last_target_slot = __v); }
    }

    /// java: private void onSound(ClientboundSoundPacket packet)
    pub fn on_sound(__this: &compat::Shared<Self>, packet: &ClientboundSoundPacket) {
        //   java: !SkyblockerConfigManager.get().foraging.moongladeMarsh.enableTunerSolver
        //   java: 				|| pitchSolved || !Utils.isInForagingIsland() || !isInMenu
        //   java: 				|| !packet.getSound().value().location().equals(SoundEvents.NOTE_BLOCK_BASS.value().location())
        if !SkyblockerConfigManager::get().foraging.moonglade_marsh.enable_tuner_solver || compat::read(__this, |__t| __t.pitch_solved) || !Utils::is_in_foraging_island() || !compat::read(__this, |__t| __t.is_in_menu) || mcrust::net::minecraft::sounds::sound_events::holder_value(packet.get_sound()).map(|__e| __e.location().clone()) != Some(mcrust::net::minecraft::sounds::sound_events::note_block_bass().location().clone()) {
            return;
        }
        let mut packet_pitch: f32 = packet.get_pitch();
        __this.lock().borrow_mut().recent_pitches.push(packet_pitch);
        let mut sample_count: i32 = (__this.lock().borrow().recent_pitches.len() as i32);
        let mut target_pitch: Option<String> = TunerSolver::get_pitch_name(packet_pitch);
        if compat::read(__this, |__t| __t.current_pitch.clone()).is_none() {
            logger().warn("Current pitch not set, cannot compare");
            __this.lock().borrow_mut().recent_pitches.clear();
            return;
        }
        let mut expected_pitch: f32 = TunerSolver::get_pitch_value(compat::read(__this, |__t| __t.current_pitch.clone()).as_deref().unwrap());
        if (packet_pitch - expected_pitch).abs() > 0.0001f32 {
            let Some(mut target_pitch) = target_pitch.clone() else {
                logger().warn(&format!("Invalid pitch value received: {}", compat::fstr(packet_pitch as f64)));
                __this.lock().borrow_mut().recent_pitches.clear();
                return;
            };
            let mut current_index: i32 = TunerSolver::get_pitch_index(compat::read(__this, |__t| __t.current_pitch.clone()).as_deref().unwrap());
            let mut target_index: i32 = TunerSolver::get_pitch_index(&target_pitch);
            if current_index == -1 || target_index == -1 {
                logger().warn(&format!("Invalid pitch indices: current={}, target={}", compat::opt_str(&compat::read(__this, |__t| __t.current_pitch.clone())), target_pitch));
                __this.lock().borrow_mut().recent_pitches.clear();
                return;
            }
            let mut clicks: i32 = TunerSolver::calculate_pitch_clicks(current_index, target_index);
            logger().info(&format!("Pitch solved: Current={}, Target={}, Required clicks={}", compat::opt_str(&compat::read(__this, |__t| __t.current_pitch.clone())), target_pitch, (if clicks >= 0 { format!("+{}", clicks) } else { clicks.to_string() })));
            { let __v = clicks; compat::write(__this, |__t| __t.pitch_clicks = __v); }
            { let __v = true; compat::write(__this, |__t| __t.pitch_solved = __v); }
            __this.lock().borrow_mut().recent_pitches.clear();
            return;
        }
        if sample_count >= MAX_PITCH_SAMPLES {
            { let __v = 0; compat::write(__this, |__t| __t.pitch_clicks = __v); }
            { let __v = true; compat::write(__this, |__t| __t.pitch_solved = __v); }
            logger().info(&format!("Pitch solved: Current={}, Target={}, Required clicks=+0 (all samples match)", compat::opt_str(&compat::read(__this, |__t| __t.current_pitch.clone())), compat::opt_str(&compat::read(__this, |__t| __t.current_pitch.clone()))));
            __this.lock().borrow_mut().recent_pitches.clear();
        }
    }

    /// java: private void maybeSolveSpeed(Int2ObjectMap<ItemStack> slots)
    pub fn maybe_solve_speed(__this: &compat::Shared<Self>, slots: &std::collections::BTreeMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack>) {
        let mut current_speed: i32 = TunerSolver::read_current_speed(slots);
        if current_speed > 0 && compat::read(__this, |__t| __t.target_speed) != -1 {
            let mut current_index: i32 = TunerSolver::get_speed_index(current_speed);
            let mut target_index: i32 = TunerSolver::get_speed_index(compat::read(__this, |__t| __t.target_speed));
            if current_index != -1 && target_index != -1 {
                { let __v = TunerSolver::calculate_speed_clicks(current_index, target_index); compat::write(__this, |__t| __t.speed_clicks = __v); }
                { let __v = true; compat::write(__this, |__t| __t.speed_solved = __v); }
                logger().info(&format!("Speed solved: Current={}, Target={}, Ticks={}, Required clicks={}", current_speed, compat::read(__this, |__t| __t.target_speed), compat::read(__this, |__t| __t.last_speed_ticks), (if compat::read(__this, |__t| __t.speed_clicks) >= 0 { format!("+{}", compat::read(__this, |__t| __t.speed_clicks)) } else { compat::read(__this, |__t| __t.speed_clicks).to_string() })));
            } else {
                logger().warn(&format!("Invalid speed indices: current={}, target={}", current_speed, compat::read(__this, |__t| __t.target_speed)));
            }
        }
    }

}

impl SimpleContainerSolver for TunerSolver {
    fn simple_container_solver_base(&self) -> &SimpleContainerSolverBase {
        &self.base
    }
    fn simple_container_solver_base_mut(&mut self) -> &mut SimpleContainerSolverBase {
        &mut self.base
    }

}

impl RegexContainerMatcher for TunerSolver {
    fn regex_container_matcher_base(&self) -> &RegexContainerMatcherBase {
        &self.base.base
    }
    fn regex_container_matcher_base_mut(&mut self) -> &mut RegexContainerMatcherBase {
        &mut self.base.base
    }

    /// java: @Override public Pattern getTitlePattern() (dispatch: the body for this class)
    fn get_title_pattern(&self) -> fn(&compat::Shared<dyn RegexContainerMatcher>) -> Option<compat::regex::Pattern> {
        <dyn SimpleContainerSolver>::get_title_pattern_from::<Self>
    }

}

impl ContainerMatcher for TunerSolver {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public boolean isEnabled() (dispatch: the body for this class)
    fn is_enabled(&self) -> fn(&compat::Shared<dyn ContainerMatcher>) -> bool {
        TunerSolver::is_enabled_dyn
    }

    /// java: @Override public boolean test(Screen screen) (dispatch: the body for this class)
    fn test(&self) -> fn(&compat::Shared<dyn ContainerMatcher>, &mcrust::net::minecraft::client::gui::screens::screen::Screen) -> bool {
        <dyn RegexContainerMatcher>::test_from::<Self>
    }

}

impl ContainerSolver for TunerSolver {
    /// java: @Override public List<ColorHighlight> getColors(Int2ObjectMap<ItemStack> slots) (dispatch: the body for this class)
    fn get_colors(&self) -> fn(&compat::Shared<dyn ContainerSolver>, &mut std::collections::BTreeMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> Vec<ColorHighlight> {
        TunerSolver::get_colors_dyn
    }

    /// java: @Override public void start(AbstractContainerScreen<?> screen) (dispatch: the body for this class)
    fn start(&self) -> fn(&compat::Shared<dyn ContainerSolver>, &AbstractContainerScreen) {
        TunerSolver::start_dyn
    }

    /// java: @Override public boolean onClickSlot(int slotId, ItemStack stack, int screenId, int button) (dispatch: the body for this class)
    fn on_click_slot(&self) -> fn(&compat::Shared<dyn ContainerSolver>, i32, &mcrust::net::minecraft::world::item::item_stack::ItemStack, i32, i32) -> bool {
        TunerSolver::on_click_slot_dyn
    }

}

impl Resettable for TunerSolver {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void reset() (dispatch: the body for this class)
    fn reset(&self) -> fn(&compat::Shared<dyn Resettable>) {
        TunerSolver::reset_dyn
    }

}

impl SlotTextAdder for TunerSolver {
    /// java: @Override public List<SlotText> getText(@Nullable Slot slot, ItemStack stack, int slotId) (dispatch: the body for this class)
    fn get_text(&self) -> fn(&compat::Shared<dyn SlotTextAdder>, Option<Slot>, &mcrust::net::minecraft::world::item::item_stack::ItemStack, i32) -> Vec<SlotText> {
        TunerSolver::get_text_dyn
    }

}

impl TunerSolver {
    /// java: @Override public boolean isEnabled()
    pub fn is_enabled_impl(__this: &compat::Shared<Self>) -> bool {
        return SkyblockerConfigManager::get().foraging.moonglade_marsh.enable_tuner_solver;
    }

    /// TunerSolver.isEnabled as the dispatch of ContainerMatcher.isEnabled
    pub fn is_enabled_dyn(__this: &compat::Shared<dyn ContainerMatcher>) -> bool {
        Self::is_enabled_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public List<ColorHighlight> getColors(Int2ObjectMap<ItemStack> slots)
    pub fn get_colors_impl(__this: &compat::Shared<Self>, slots: &mut std::collections::BTreeMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> Vec<ColorHighlight> {
        if !(compat::read(__this, |__t| __t.has_processed)) {
            let mut dye_stack: Option<mcrust::net::minecraft::world::item::item_stack::ItemStack> = slots.get(&46).cloned();
            if dye_stack.is_some() && !(dye_stack.as_ref().unwrap().is_empty()) && TunerSolver::is_dye(&dye_stack.as_ref().unwrap().get_item()) {
                if !(compat::read(__this, |__t| __t.color_solved)) {
                    { let __v = TunerSolver::compute_color_clicks(slots); compat::write(__this, |__t| __t.color_clicks = __v); }
                    { let __v = true; compat::write(__this, |__t| __t.color_solved = __v); }
                }
                if !(compat::read(__this, |__t| __t.speed_solved)) {
                    TunerSolver::maybe_solve_speed(__this, slots);
                }
                if !(compat::read(__this, |__t| __t.pitch_solved)) {
                    { let __v = TunerSolver::read_current_pitch(slots); compat::write(__this, |__t| __t.current_pitch = __v); }
                }
                { let __v = true; compat::write(__this, |__t| __t.has_processed = __v); }
            }
        }
        return vec![];
    }

    /// TunerSolver.getColors as the dispatch of ContainerSolver.getColors
    pub fn get_colors_dyn(__this: &compat::Shared<dyn ContainerSolver>, __p0: &mut std::collections::BTreeMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> Vec<ColorHighlight> {
        Self::get_colors_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public void start(AbstractContainerScreen<?> screen)
    pub fn start_impl(__this: &compat::Shared<Self>, screen: &AbstractContainerScreen) {
        TunerSolver::reset_state(__this);
        { let __v = true; compat::write(__this, |__t| __t.is_in_menu = __v); }
        // java: ScreenEvents.afterTick(screen).register(_ -> trackTargetPaneMovement(screen.getMenu().slots));
        // java: ScreenEvents.remove(screen).register(_ -> resetState());
        // (both listeners belong to `screen`: they are this solver's, held until that screen's `removed`)
        let _ = screen;
        TunerSolver::install_screen_listeners();
        TUNER_SCREEN_LISTENER.with(|l| *l.borrow_mut() = Some(__this.clone()));
    }

    /// The per-screen `ScreenEvents.afterTick` / `ScreenEvents.remove` listeners `start` registers: the end of the open
    /// container screen's tick (`containerTick()` RETURN) and the HEAD of its `removed()`, which drops both.
    fn install_screen_listeners() {
        static ONCE: std::sync::Once = std::sync::Once::new();
        ONCE.call_once(|| {
            let result = mcrust::mcrust::modding::hook::inject("net.minecraft.client.gui.screens.inventory.AbstractContainerScreen#containerTick", mcrust::mcrust::modding::hook::At::Return, |_ci: &mut mcrust::mcrust::modding::hook::CallbackInfo<'_>| {
                let Some(solver) = TUNER_SCREEN_LISTENER.with(|l| l.borrow().clone()) else {
                    return;
                };
                // java: screen.getMenu().slots
                let Some(slots) = compat::with_open_menu(|menu, _| menu.slots.to_vec()) else {
                    return;
                };
                TunerSolver::track_target_pane_movement(&solver, &slots);
            });
            if let Err(e) = result {
                mcrust::mcrust::modding::log::warn_once("skyblocker-tuner-after-tick", &format!("{e:?}"));
            }
            let result = mcrust::mcrust::modding::hook::inject("net.minecraft.client.gui.screens.inventory.AbstractContainerScreen#removed", mcrust::mcrust::modding::hook::At::Head, |_ci: &mut mcrust::mcrust::modding::hook::CallbackInfo<'_>| {
                let Some(solver) = TUNER_SCREEN_LISTENER.with(|l| l.borrow_mut().take()) else {
                    return;
                };
                TunerSolver::reset_state(&solver);
            });
            if let Err(e) = result {
                mcrust::mcrust::modding::log::warn_once("skyblocker-tuner-remove", &format!("{e:?}"));
            }
        });
    }

    /// TunerSolver.start as the dispatch of ContainerSolver.start
    pub fn start_dyn(__this: &compat::Shared<dyn ContainerSolver>, __p0: &AbstractContainerScreen) {
        Self::start_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public boolean onClickSlot(int slotId, ItemStack stack, int screenId, int button)
    pub fn on_click_slot_impl(__this: &compat::Shared<Self>, mut slot_id: i32, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, mut screen_id: i32, mut button: i32) -> bool {
        if !(SkyblockerConfigManager::get().foraging.moonglade_marsh.enable_tuner_solver) {
            return false;
        }
        if !(compat::read(__this, |__t| __t.is_in_menu)) {
            return false;
        }
        if button != 0 && button != 1 && button != 2 {
            return false;
        }
        let mut delta: i32 = (if button == 1 { 1 } else { -1 });
        if compat::read(__this, |__t| __t.color_solved) && slot_id == 46 {
            { let __v = TunerSolver::update_clicks(compat::read(__this, |__t| __t.color_clicks), (COLOR_CYCLE.get_mut().len() as i32), delta); compat::write(__this, |__t| __t.color_clicks = __v); }
        } else if compat::read(__this, |__t| __t.speed_solved) && slot_id == 48 {
            { let __v = TunerSolver::update_clicks(compat::read(__this, |__t| __t.speed_clicks), (SPEED_CYCLE.len() as i32), delta); compat::write(__this, |__t| __t.speed_clicks = __v); }
        } else if compat::read(__this, |__t| __t.pitch_solved) && slot_id == 50 {
            { let __v = TunerSolver::update_clicks(compat::read(__this, |__t| __t.pitch_clicks), (PITCH_CYCLE.len() as i32), delta); compat::write(__this, |__t| __t.pitch_clicks = __v); }
        }
        return false;
    }

    /// TunerSolver.onClickSlot as the dispatch of ContainerSolver.onClickSlot
    pub fn on_click_slot_dyn(__this: &compat::Shared<dyn ContainerSolver>, __p0: i32, __p1: &mcrust::net::minecraft::world::item::item_stack::ItemStack, __p2: i32, __p3: i32) -> bool {
        Self::on_click_slot_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2, __p3)
    }

    /// java: @Override public void reset()
    pub fn reset_impl(__this: &compat::Shared<Self>) {
        TunerSolver::reset_state(__this);
    }

    /// TunerSolver.reset as the dispatch of Resettable.reset
    pub fn reset_dyn(__this: &compat::Shared<dyn Resettable>) {
        Self::reset_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public List<SlotText> getText(@Nullable Slot slot, ItemStack stack, int slotId)
    pub fn get_text_impl(__this: &compat::Shared<Self>, mut slot: Option<Slot>, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, mut slot_id: i32) -> Vec<SlotText> {
        if !({ let __f = compat::dispatch(__this, |__t| __t.is_enabled()); __f(&compat::view(__this, |a| a as compat::Shared<dyn ContainerMatcher>)) }) {
            return vec![];
        }
        if slot_id == 46 && compat::read(__this, |__t| __t.color_solved) {
            return SlotText::bottom_right_list(&(component::literal(&compat::read(__this, |__t| __t.color_clicks).to_string())).with_color(slot_text::LIGHT_GREEN));
        }
        if slot_id == 48 && compat::read(__this, |__t| __t.speed_solved) {
            return SlotText::bottom_right_list(&(component::literal(&compat::read(__this, |__t| __t.speed_clicks).to_string())).with_color(slot_text::LIGHT_GREEN));
        }
        if slot_id == 50 && compat::read(__this, |__t| __t.pitch_solved) {
            return SlotText::bottom_right_list(&(component::literal(&compat::read(__this, |__t| __t.pitch_clicks).to_string())).with_color(slot_text::LIGHT_GREEN));
        }
        return vec![];
    }

    /// TunerSolver.getText as the dispatch of SlotTextAdder.getText
    pub fn get_text_dyn(__this: &compat::Shared<dyn SlotTextAdder>, __p0: Option<Slot>, __p1: &mcrust::net::minecraft::world::item::item_stack::ItemStack, __p2: i32) -> Vec<SlotText> {
        Self::get_text_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2)
    }

}

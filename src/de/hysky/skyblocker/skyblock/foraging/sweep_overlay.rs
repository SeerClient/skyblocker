//! Ported from de.hysky.skyblocker.skyblock.foraging.SweepOverlay by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::tabhud::util::player_list_manager::PlayerListManager;
use crate::de::hysky::skyblocker::utils::block_pos_set::BlockPosSet;
use crate::de::hysky::skyblocker::utils::constants;
use crate::de::hysky::skyblocker::utils::render::level_render_extraction_callback;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_collector::PrimitiveCollector;
use crate::de::hysky::skyblocker::utils::sky_block_icons;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::java::awt::color::Color;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::core::block_pos::BlockPos;
use mcrust::net::minecraft::core::direction::Direction;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::tags::tag_key::TagKey;
use mcrust::net::minecraft::world::level::block::block::Block;
use mcrust::net::minecraft::world::level::block::blocks;
use mcrust::net::minecraft::world::level::block::state::block_state::BlockState;
use mcrust::net::minecraft::world::level::level::Level;
use mcrust::net::minecraft::world::phys::block_hit_result::BlockHitResult;
use mcrust::net::minecraft::world::phys::hit_result::HitResult;
use mcrust::net::minecraft::world::phys::vec3::Vec3;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;
use mcrust::net::minecraft::world::entity::living_entity::LivingEntity as _;
use mcrust::net::minecraft::world::level::block_getter::BlockGetter as _;

/// java: private static float[] colorComponents;
pub static COLOR_COMPONENTS: LazyLock<Mutex<Vec<f32>>> = LazyLock::new(|| Mutex::new(Default::default()));
/// java: private static final int MAX_BASIC_WOOD_CUT = 36;
pub const MAX_BASIC_WOOD_CUT: i32 = 36;
/// java: private static final Pattern SWEEP_VALUE_PATTERN = Pattern.compile(String.format("Sweep:\\s*(?:[∮%s])*(\\d+)", SkyBlockIcons.SWEEP));
pub static SWEEP_VALUE_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile(&format!("Sweep:\\s*(?:[∮{}])*(\\d+)", sky_block_icons::SWEEP)));
/// java: private static boolean sweepStatNoticeShown = false;
pub static SWEEP_STAT_NOTICE_SHOWN: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));
/// java: private static final Set<String> VALID_AXES = Set.of( "JUNGLE_AXE", "TREECAPITATOR_AXE", "FIG_AXE", "FIGSTONE_AXE", "HELIX_CHOPPER", "ROOKIE_AXE", "PROMISING_AXE", "SWEET_AXE", "EFFICIENT_AXE" );
pub static VALID_AXES: LazyLock<std::collections::HashSet<String>> = LazyLock::new(|| std::collections::HashSet::from(["JUNGLE_AXE".to_string(), "TREECAPITATOR_AXE".to_string(), "FIG_AXE".to_string(), "FIGSTONE_AXE".to_string(), "HELIX_CHOPPER".to_string(), "ROOKIE_AXE".to_string(), "PROMISING_AXE".to_string(), "SWEET_AXE".to_string(), "EFFICIENT_AXE".to_string()]));
/// java: private static final Set<String> THROWABLE_AXES = Set.of( "JUNGLE_AXE", "TREECAPITATOR_AXE", "FIG_AXE", "FIGSTONE_AXE", "HELIX_CHOPPER" );
pub static THROWABLE_AXES: LazyLock<std::collections::HashSet<String>> = LazyLock::new(|| std::collections::HashSet::from(["JUNGLE_AXE".to_string(), "TREECAPITATOR_AXE".to_string(), "FIG_AXE".to_string(), "FIGSTONE_AXE".to_string(), "HELIX_CHOPPER".to_string()]));
/// java: private static final BlockPos[] NEIGHBOR_OFFSETS = { new BlockPos(-1, -1, -1), new BlockPos(-1, -1, 0), new BlockPos(-1, -1, 1), new BlockPos(-1, 0, -1), new BlockPos(-1, 0, 0), new BlockPos(-1, 0, 1)
pub static NEIGHBOR_OFFSETS: compat::MainThread<Vec<BlockPos>> = compat::MainThread::new(|| vec![BlockPos::new(-1, -1, -1), BlockPos::new(-1, -1, 0), BlockPos::new(-1, -1, 1), BlockPos::new(-1, 0, -1), BlockPos::new(-1, 0, 0), BlockPos::new(-1, 0, 1), BlockPos::new(-1, 1, -1), BlockPos::new(-1, 1, 0), BlockPos::new(-1, 1, 1), BlockPos::new(0, -1, -1), BlockPos::new(0, -1, 0), BlockPos::new(0, -1, 1), BlockPos::new(0, 0, -1), BlockPos::new(0, 0, 1), BlockPos::new(0, 1, -1), BlockPos::new(0, 1, 0), BlockPos::new(0, 1, 1), BlockPos::new(1, -1, -1), BlockPos::new(1, -1, 0), BlockPos::new(1, -1, 1), BlockPos::new(1, 0, -1), BlockPos::new(1, 0, 0), BlockPos::new(1, 0, 1), BlockPos::new(1, 1, -1), BlockPos::new(1, 1, 0), BlockPos::new(1, 1, 1)]);
/// java: private static final Map<Block, Float> TOUGHNESS_MAP = Map.of( Blocks.STRIPPED_SPRUCE_LOG, 10f, Blocks.STRIPPED_SPRUCE_WOOD, 10f, Blocks.MANGROVE_LOG, 50f, Blocks.MANGROVE_WOOD, 50f, Blocks.STRIPPED_B
/// (keyed by the block's identity, as `Map.of` over `Block`s that keep `Object.equals`)
pub static TOUGHNESS_MAP: compat::MainThread<Vec<(Arc<Block>, f32)>> = compat::MainThread::new(|| Vec::from([(blocks::blocks().get("STRIPPED_SPRUCE_LOG"), 10.0f32), (blocks::blocks().get("STRIPPED_SPRUCE_WOOD"), 10.0f32), (blocks::blocks().get("MANGROVE_LOG"), 50.0f32), (blocks::blocks().get("MANGROVE_WOOD"), 50.0f32), (blocks::blocks().get("STRIPPED_BIRCH_LOG"), 150.0f32), (blocks::blocks().get("STRIPPED_BIRCH_WOOD"), 150.0f32), (blocks::blocks().get("STRIPPED_MANGROVE_LOG"), 150.0f32), (blocks::blocks().get("STRIPPED_MANGROVE_WOOD"), 150.0f32)]));
/// java: @VisibleForTesting protected static final Map<String, List<Integer>> REQUIRED_SWEEP_VALUES = Map.ofEntries( Map.entry("FIG", List.of(7, 9, 11, 16, 21, 26, 34, 42, 50, 62, 74, 86, 102, 118, 134, 154, 1
pub static REQUIRED_SWEEP_VALUES: LazyLock<std::collections::HashMap<String, Vec<i32>>> = LazyLock::new(|| std::collections::HashMap::from([("FIG".to_string(), vec![7, 9, 11, 16, 21, 26, 34, 42, 50, 62, 74, 86, 102, 118, 134, 154, 174, 194, 214, 234]), ("MANGROVE".to_string(), vec![50, 55, 60, 75, 90, 105, 130, 155, 180, 230, 280, 330, 405, 480, 555, 655, 755, 855, 955, 1055]), ("HELIX".to_string(), vec![150, 160, 170, 200, 230, 260, 320, 380, 440, 540, 640, 740, 890, 1040, 1190, 1390, 1590, 1790, 1990, 2190])]));

/// java: public class SweepOverlay
#[derive(Clone)]
pub struct SweepOverlay {}

impl SweepOverlay {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        SweepOverlay::config_callback(&SkyblockerConfigManager::get().foraging.sweep_overlay.sweep_overlay_color);
        level_render_extraction_callback::EVENT.register(Arc::new(move |__a0| { SweepOverlay::extract_rendering(&__a0); }));
    }

    /// java: private static boolean isValidLocation()
    pub fn is_valid_location() -> bool {
        return (Utils::is_in_foraging_island() || Utils::is_in_hub() || Utils::is_in_private_island());
    }

    /// java: private static void extractRendering(PrimitiveCollector collector)
    pub fn extract_rendering(collector: &compat::Shared<dyn PrimitiveCollector>) {
        let config = &SkyblockerConfigManager::get().foraging.sweep_overlay;
        if !(SweepOverlay::is_valid_location()) || !config.enable_sweep_overlay || mcrust::mcrust::modding::game::with_player(|_| ()).is_none() || mcrust::mcrust::modding::game::level().is_none() {
            return;
        }
        let mut held_item: mcrust::net::minecraft::world::item::item_stack::ItemStack = mcrust::mcrust::modding::game::with_player(|player| player.get_main_hand_item().clone()).unwrap();
        let mut item_id: String = held_item.get_skyblock_id();
        let mut is_valid_axe: bool = VALID_AXES.contains(&item_id);
        let mut is_throwable_axe: bool = THROWABLE_AXES.contains(&item_id);
        if !is_valid_axe && !is_throwable_axe {
            return;
        }
        let mut block_hit_result: Option<BlockHitResult> = None;
        let mut is_thrown: bool = false;
        // TODO(fabric2seer): field `BLOCK` of a value of unknown type
        //   java: if (isValidAxe && CLIENT.hitResult != null && CLIENT.hitResult.getType() == HitResult.Type.BLOCK
        //   java: 				&& CLIENT.hitResult instanceof BlockHitResult hitResult) {
        //   java: 			blockHitResult = hitResult;
        //   java: 		} else if (isThrowableAxe && config.enableThrownAbilityOverlay && !ItemCooldowns.isOnCooldown(heldItem)) {
        //   java: 			// Cast a ray up to 50 blocks for throwable axes
        //   java: 			Vec3 start = CLIENT.player.getEyePosition(1.0f);
        //   java: 			Vec3 look = CLIENT.player.getViewVector(1.0f);
        //   java: 			double lookPitch = CLIENT.player.getXRot();
        //   java: 			HitResult hitResult = thrownAxe(start, look, lookPitch, 50);
        //   java: 			if (hitResult != null && hitResult.getType() == HitResult.Type.BLOCK && hitResult instanceof BlockHitResult rayHitResult) {
        //   java: 				blockHitResult = rayHitResult;
        //   java: 				isThrown = true;
        //   java: 			}
        //   java: 		}
        if let Some(block_hit_result) = &block_hit_result {
            let mut state: Arc<BlockState> = mcrust::mcrust::modding::game::level().unwrap().get_block_state_typed(&block_hit_result.get_block_pos());
            if SweepOverlay::is_log(&state) {
                SweepOverlay::submit_connected_logs(collector, block_hit_result, &state, is_thrown);
            }
        }
    }

    /// java: private static @Nullable BlockHitResult thrownAxe(Vec3 start, Vec3 look, Double lookPitch, int maxDistance)
    pub fn thrown_axe(start: &mut Vec3, look: &Vec3, mut look_pitch: f64, mut max_distance: i32) -> Option<BlockHitResult> {
        if mcrust::mcrust::modding::game::level().is_none() {
            return None;
        }
        let mut max_distance: i32 = max_distance;
        for i in 0..max_distance {
            let mut pos: Vec3 = start.add_vec(&look.scale((i) as f64));
            let mut angle: f64 = (look_pitch / (90 as f64)).abs();
            let mut gravity: f64 = ((((1 as f64) - angle) * (((2.65 + (2.51827 * angle)) - (2.43475 * angle.powf((2 as f64)))) + (1.22748 * angle.powf((3 as f64))))) / (1000 as f64));
            pos = pos.subtract((0) as f64, (gravity * (i as f64).powf((2 as f64))), (0) as f64);
            let mut block: BlockPos = BlockPos::containing_position(&pos);
            let mut state: Arc<BlockState> = mcrust::mcrust::modding::game::level().unwrap().get_block_state_typed(&block);
            if SweepOverlay::is_log(&state) {
                return Some(BlockHitResult::new(pos.clone(), Direction::Down, block.clone(), false));
            }
        }
        return None;
    }

    /// java: private static boolean isLog(BlockState state)
    pub fn is_log(state: &Arc<BlockState>) -> bool {
        if Utils::is_in_galatea() {
            return (SweepOverlay::is_fig_log(state) || SweepOverlay::is_mangrove_log(state));
        } else if Utils::is_in_torrhus_canyon() {
            return SweepOverlay::is_helix_log(state);
        } else if Utils::is_in_hub() {
            return (state.is_block(&blocks::blocks().get("OAK_LOG")) || state.is_block(&blocks::blocks().get("OAK_WOOD")));
        }
        // java: return state.is(BlockTags.LOGS); - `TypedInstance.is(TagKey)`
        use mcrust::net::minecraft::core::typed_instance::TypedInstance as _;
        return state.is_tag(&mcrust::net::minecraft::tags::block_tags::logs());
    }

    /// java: private static boolean isFigLog(BlockState state)
    pub fn is_fig_log(state: &Arc<BlockState>) -> bool {
        return (state.is_block(&blocks::blocks().get("STRIPPED_SPRUCE_LOG")) || state.is_block(&blocks::blocks().get("STRIPPED_SPRUCE_WOOD")));
    }

    /// java: private static boolean isMangroveLog(BlockState state)
    pub fn is_mangrove_log(state: &Arc<BlockState>) -> bool {
        return (state.is_block(&blocks::blocks().get("MANGROVE_LOG")) || state.is_block(&blocks::blocks().get("MANGROVE_WOOD")));
    }

    /// java: private static boolean isHelixLog(BlockState state)
    pub fn is_helix_log(state: &Arc<BlockState>) -> bool {
        return (state.is_block(&blocks::blocks().get("STRIPPED_BIRCH_LOG")) || state.is_block(&blocks::blocks().get("STRIPPED_BIRCH_WOOD")) || state.is_block(&blocks::blocks().get("STRIPPED_MANGROVE_LOG")) || state.is_block(&blocks::blocks().get("STRIPPED_MANGROVE_WOOD")));
    }

    /// java: private static boolean shouldBeChopped(BlockState source, BlockState destination)
    pub fn should_be_chopped(source: &Arc<BlockState>, destination: &Arc<BlockState>) -> bool {
        if Utils::is_in_torrhus_canyon() {
            let mut birch_logs: TagKey = mcrust::net::minecraft::tags::block_item_tags::birch_logs().block();
            let mut mangrove_logs: TagKey = mcrust::net::minecraft::tags::block_item_tags::mangrove_logs().block();
            // java: return source.is(birchLogs) && destination.is(birchLogs) || source.is(mangroveLogs) && destination.is(mangroveLogs);
            use mcrust::net::minecraft::core::typed_instance::TypedInstance as _;
            return (source.is_tag(&birch_logs) && destination.is_tag(&birch_logs)) || (source.is_tag(&mangrove_logs) && destination.is_tag(&mangrove_logs));
        }
        return true;
    }

    /// java: private static float getSweepStat()
    pub fn get_sweep_stat() -> f32 {
        if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() {
            return 0.0f32;
        }
        let mut player_list: Option<Vec<String>> = Some(PlayerListManager::get_player_string_list());
        if let Some(player_list) = &player_list {
            for entry in player_list.iter() {
                let mut matcher: compat::regex::Matcher = SWEEP_VALUE_PATTERN.matcher(entry);
                if matcher.find() {
                    // java: try {..} catch (NumberFormatException e) - an exception here is a panic, caught by `catch_unwind`; the body's `return` is the closure's value
                    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        return matcher.group(1).unwrap().trim().parse::<f32>().expect("NumberFormatException");
                    })) {
                        Ok(__v) => return __v,
                        Err(e) => {
                            logger().warn(&format!("Failed to parse Sweep stat from tab list: {}. Error: {}", entry, compat::panic_message(&e)));
                        }
                    }
                }
            }
        }
        if !(compat::get_state(&SWEEP_STAT_NOTICE_SHOWN)) && Utils::is_in_foraging_island() && mcrust::mcrust::modding::game::with_player(|_| ()).is_some() {
            mcrust::mcrust::modding::game::with_player(|player| { script_host::send_feedback(&(constants::PREFIX.get_mut())().append(component::translatable("skyblocker.config.foraging.sweepOverlay.sweepStatMissingMessage").with_style_format(ChatFormatting::Red)).to_string()); });
            compat::set_state(&SWEEP_STAT_NOTICE_SHOWN, true);
        }
        return 0.0f32;
    }

    /// java: private static int calculateMaxWood(float sweepStat, float toughness, BlockState state)
    pub fn calculate_max_wood(mut sweep_stat: f32, mut toughness: f32, state: &Arc<BlockState>) -> i32 {
        if Utils::is_in_park() || Utils::is_in_hub() {
            return ((((1 as f32) + sweep_stat).min((MAX_BASIC_WOOD_CUT as f32))) as i32);
        }
        let mut sweep_values: Vec<i32> = vec![];
        if Utils::is_in_galatea() {
            if SweepOverlay::is_fig_log(state) {
                sweep_values = REQUIRED_SWEEP_VALUES.get("FIG").cloned().unwrap();
            } else if SweepOverlay::is_mangrove_log(state) {
                sweep_values = REQUIRED_SWEEP_VALUES.get("MANGROVE").cloned().unwrap();
            }
        } else if Utils::is_in_torrhus_canyon() && SweepOverlay::is_helix_log(state) {
            sweep_values = REQUIRED_SWEEP_VALUES.get("HELIX").cloned().unwrap();
        }
        return SweepOverlay::calculate_max_wood_2(sweep_stat, &sweep_values);
    }

    /// java: @VisibleForTesting protected static int calculateMaxWood(float sweepStat, List<Integer> sweepValues)
    pub fn calculate_max_wood_2(mut sweep_stat: f32, sweep_values: &Vec<i32>) -> i32 {
        let mut max_logs: i32 = (sweep_values.len() as i32);
        let mut logs: i32 = max_logs;
        for i in 0..max_logs {
            let mut required_sweep: i32 = sweep_values[(i) as usize];
            if sweep_stat < (required_sweep as f32) {
                logs = logs.wrapping_sub(max_logs.wrapping_sub(i));
                break;
            }
        }
        return logs;
    }

    /// java: private static float getToughness(BlockState state)
    pub fn get_toughness(state: &Arc<BlockState>) -> f32 {
        let __block = state.get_block();
        return TOUGHNESS_MAP.get().iter().find(|(__k, _)| Arc::ptr_eq(__k, &__block)).map(|(_, __v)| *__v).unwrap_or_else(|| 1.0f32);
    }

    /// java: private static void submitConnectedLogs(PrimitiveCollector collector, BlockHitResult blockHitResult, BlockState state, boolean isThrown)
    pub fn submit_connected_logs(collector: &compat::Shared<dyn PrimitiveCollector>, block_hit_result: &BlockHitResult, state: &Arc<BlockState>, mut is_thrown: bool) {
        let mut start_pos: BlockPos = block_hit_result.get_block_pos();
        let mut world: Arc<mcrust::net::minecraft::client::multiplayer::client_level::ClientLevel> = mcrust::mcrust::modding::game::level().unwrap();
        let mut sweep_stat: f32 = SweepOverlay::get_sweep_stat();
        if sweep_stat <= (0 as f32) {
            return;
        }
        let mut render_color: Vec<f32> = compat::get_state(&COLOR_COMPONENTS);
        if is_thrown {
            // one guard for the whole statement: two `compat::lock(&COLOR_COMPONENTS)` temporaries in one
            // expression both live to its end, and the second waits for the first (a self-deadlock)
            #[allow(unused_mut)] let mut __guard_color_components = compat::lock(&COLOR_COMPONENTS);
            render_color = vec![((*__guard_color_components)[(0) as usize] * 0.7f32), ((*__guard_color_components)[(1) as usize] * 0.7f32), ((*__guard_color_components)[(2) as usize] * 0.7f32), (*__guard_color_components)[(3) as usize]];
            drop(__guard_color_components); // released before anything below locks it again
        }
        let mut visited: compat::Shared<BlockPosSet> = BlockPosSet::new();
        let mut queue: std::collections::VecDeque<BlockPos> = std::collections::VecDeque::new();
        let mut wood_count: i32 = 0;
        let mut toughness: f32 = SweepOverlay::get_toughness(state);
        let mut max_wood: i32 = SweepOverlay::calculate_max_wood(sweep_stat, toughness, state);
        if is_thrown {
            max_wood = max_wood.wrapping_div(2);
        }
        queue.push_back(start_pos.clone());
        BlockPosSet::add(&visited, &start_pos);
        while wood_count < max_wood && !queue.is_empty() {
            let mut pos: BlockPos = queue.pop_front().unwrap();
            let mut current_state: Arc<BlockState> = world.get_block_state_typed(&pos);
            if !(SweepOverlay::is_log(&current_state)) {
                continue;
            }
            wood_count += 1;
            ({ let __f = compat::dispatch(collector, |__t| __t.submit_filled_box()); __f(collector, &pos, &render_color, render_color[(3) as usize], false) });
            for offset in NEIGHBOR_OFFSETS.get_mut().iter() {
                let mut neighbor: BlockPos = pos.offset_vec(offset.as_vec3i());
                if BlockPosSet::contains_block_pos(&visited, &neighbor) || queue.iter().any(|e| *e == neighbor.clone()) {
                    continue;
                }
                let mut neighbor_state: Arc<BlockState> = world.get_block_state_typed(&neighbor);
                if SweepOverlay::is_log(&world.get_block_state_typed(&neighbor)) && SweepOverlay::should_be_chopped(&current_state, &neighbor_state) {
                    queue.push_back(neighbor.clone());
                    BlockPosSet::add(&visited, &neighbor);
                }
            }
        }
    }

    /// java: public static void configCallback(Color color)
    pub fn config_callback(color: &Color) {
        // java: colorComponents = color.getRGBComponents(null);
        compat::set_state(&COLOR_COMPONENTS, color.get_rgb_components(None).to_vec());
    }

}

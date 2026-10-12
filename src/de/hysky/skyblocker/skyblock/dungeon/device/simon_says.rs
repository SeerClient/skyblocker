//! Ported from de.hysky.skyblocker.skyblock.dungeon.device.SimonSays by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::events::world_events;
use crate::de::hysky::skyblocker::skyblock::dungeon::dungeon_boss::DungeonBoss;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::dungeon_manager::DungeonManager;
use crate::de::hysky::skyblocker::utils::block_pos_set::BlockPosSet;
use crate::de::hysky::skyblocker::utils::color_utils::ColorUtils;
use crate::de::hysky::skyblocker::utils::render::level_render_extraction_callback;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_collector::PrimitiveCollector;
use crate::de::hysky::skyblocker::utils::render::render_helper::RenderHelper;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::net::minecraft::client::multiplayer::client_level::ClientLevel;
use mcrust::net::minecraft::core::block_pos::BlockPos;
use mcrust::net::minecraft::world::entity::player::player::Player;
use mcrust::net::minecraft::world::interaction_hand::InteractionHand;
use mcrust::net::minecraft::world::interaction_result::InteractionResult;
use mcrust::net::minecraft::world::item::dye_color::DyeColor;
use mcrust::net::minecraft::world::level::block::block::Block;
use mcrust::net::minecraft::world::level::block::blocks;
use mcrust::net::minecraft::world::level::block::state::block_state::BlockState;
use mcrust::net::minecraft::world::level::level::Level;
use mcrust::net::minecraft::world::phys::aabb::AABB;
use mcrust::net::minecraft::world::phys::block_hit_result::BlockHitResult;
use mcrust::net::minecraft::world::phys::vec3;
use mcrust::net::minecraft::world::phys::vec3::Vec3;
use mcrust::net::minecraft::world::level::block_getter::BlockGetter as _;

/// java: private static final AABB BOARD_AREA = AABB.encapsulatingFullBlocks(new BlockPos(111, 123, 92), new BlockPos(111, 120, 95));
pub static BOARD_AREA: compat::MainThread<AABB> = compat::MainThread::new(|| AABB::encapsulating_full_blocks(&BlockPos::new(111, 123, 92), &BlockPos::new(111, 120, 95)));
/// java: private static final AABB BUTTONS_AREA = AABB.encapsulatingFullBlocks(new BlockPos(110, 123, 92), new BlockPos(110, 120, 95));
pub static BUTTONS_AREA: compat::MainThread<AABB> = compat::MainThread::new(|| AABB::encapsulating_full_blocks(&BlockPos::new(110, 123, 92), &BlockPos::new(110, 120, 95)));
/// java: private static final BlockPos START_BUTTON = new BlockPos(110, 121, 91);
pub static START_BUTTON: compat::MainThread<BlockPos> = compat::MainThread::new(|| BlockPos::new(110, 121, 91));
/// java: private static final float[] GREEN = ColorUtils.getFloatComponents(DyeColor.LIME);
pub static GREEN: LazyLock<Vec<f32>> = LazyLock::new(|| ColorUtils::get_float_components_dye_color(&DyeColor::Lime));
/// java: private static final float[] YELLOW = ColorUtils.getFloatComponents(DyeColor.YELLOW);
pub static YELLOW: LazyLock<Vec<f32>> = LazyLock::new(|| ColorUtils::get_float_components_dye_color(&DyeColor::Yellow));
/// java: private static final BlockPosSet CLICKED_BUTTONS = new BlockPosSet();
pub static CLICKED_BUTTONS: compat::MainThread<compat::Shared<BlockPosSet>> = compat::MainThread::new(|| BlockPosSet::new());
/// java: private static final ObjectList<BlockPos> SIMON_PATTERN = new ObjectArrayList<>();
pub static SIMON_PATTERN: compat::MainThread<Vec<BlockPos>> = compat::MainThread::new(|| Vec::new());

/// java: public class SimonSays
#[derive(Clone)]
pub struct SimonSays {}

impl SimonSays {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        // TODO(fabric2seer): UseBlockCallback.EVENT.register: no SeerClient equivalent yet
        //   java: UseBlockCallback.EVENT.register(SimonSays::onBlockInteract);
        client_play_connection_events::JOIN.register(Arc::new(|listener: &mut ClientPacketListener| {
            SimonSays::reset();
        }));
        level_render_extraction_callback::EVENT.register(Arc::new(move |__a0| { SimonSays::extract_rendering(&__a0); }));
        world_events::BLOCK_STATE_UPDATE.register(Arc::new(move |__a0, __a1, __a2| { SimonSays::on_block_update(&__a0, Some(__a1.clone()), &__a2); }));
    }

    /// java: private static InteractionResult onBlockInteract(Player player, Level world, InteractionHand hand, BlockHitResult hitResult)
    pub fn on_block_interact(player: &dyn Player, world: &Level, hand: &InteractionHand, hit_result: &BlockHitResult) -> InteractionResult {
        if SimonSays::should_process() {
            let mut pos: BlockPos = hit_result.get_block_pos();
            let mut block: Arc<Block> = world.get_block_state(&pos).get_block();
            if std::ptr::addr_eq(std::sync::Arc::as_ptr(&block), std::sync::Arc::as_ptr(&blocks::blocks().get("stone_button"))) {
                if BUTTONS_AREA.get_mut().contains_vec(&vec3::at_lower_corner_of(&pos)) {
                    BlockPosSet::add(CLICKED_BUTTONS.get_mut(), &BlockPos::from_vec3i(pos.as_vec3i()));
                } else if pos == START_BUTTON.get_mut() {
                    SimonSays::reset();
                }
            }
        }
        return InteractionResult::Pass;
    }

    /// java: private static void onBlockUpdate(BlockPos pos, @Nullable BlockState oldState, BlockState newState)
    pub fn on_block_update(pos: &BlockPos, mut old_state: Option<Arc<BlockState>>, new_state: &Arc<BlockState>) {
        if SimonSays::should_process() {
            let mut pos_vec: Vec3 = vec3::at_lower_corner_of(pos);
            let mut new_block: Arc<Block> = new_state.get_block();
            if BOARD_AREA.get_mut().contains_vec(&pos_vec) && std::ptr::addr_eq(std::sync::Arc::as_ptr(&new_block), std::sync::Arc::as_ptr(&blocks::blocks().get("obsidian"))) && old_state.is_some() && std::ptr::addr_eq(std::sync::Arc::as_ptr(&old_state.as_ref().unwrap().get_block()), std::sync::Arc::as_ptr(&blocks::blocks().get("sea_lantern"))) {
                SIMON_PATTERN.get_mut().push(pos.immutable());
            } else if BUTTONS_AREA.get_mut().contains_vec(&pos_vec) && std::ptr::addr_eq(std::sync::Arc::as_ptr(&new_block), std::sync::Arc::as_ptr(&blocks::blocks().get("air"))) {
                SimonSays::reset();
            }
        }
    }

    /// java: private static void extractRendering(PrimitiveCollector collector)
    pub fn extract_rendering(collector: &compat::Shared<dyn PrimitiveCollector>) {
        if SimonSays::should_process() {
            let mut buttons_rendered: i32 = 0;
            for pos in SIMON_PATTERN.get_mut().iter() {
                let mut button_pos: BlockPos = pos.west();
                let mut world: ClientLevel = mcrust::mcrust::modding::game::level().unwrap();
                let mut state: Arc<BlockState> = world.get_block_state_typed(&button_pos);
                if !(BlockPosSet::contains_block_pos(CLICKED_BUTTONS.get_mut(), &button_pos)) && std::ptr::addr_eq(std::sync::Arc::as_ptr(&state.get_block()), std::sync::Arc::as_ptr(&blocks::blocks().get("stone_button"))) {
                    let mut outline: Option<AABB> = RenderHelper::get_block_bounding_box_3(&world, &state, &button_pos);
                    if let Some(outline) = &outline {
                        let mut colour: Vec<f32> = (if buttons_rendered == 0 { GREEN.clone() } else { YELLOW.clone() });
                        ({ let __f = compat::dispatch(collector, |__t| __t.submit_filled_box_aabb_float_float_boolean()); __f(collector, outline, &colour, 0.5f32, true) });
                        ({ let __f = compat::dispatch(collector, |__t| __t.submit_outlined_box_aabb_float_float_boolean()); __f(collector, outline, &colour, 5.0f32, true) });
                        if { buttons_rendered += 1; buttons_rendered } == 2 {
                            return;
                        }
                    }
                }
            }
        }
    }

    /// java: private static boolean shouldProcess()
    pub fn should_process() -> bool {
        return (SkyblockerConfigManager::get().dungeons.devices.solve_simon_says && Utils::is_in_dungeons() && DungeonManager::is_in_boss() && DungeonManager::get_boss() == DungeonBoss::Maxor);
    }

    /// java: private static void reset()
    pub fn reset() {
        BlockPosSet::clear(CLICKED_BUTTONS.get_mut());
        SIMON_PATTERN.get_mut().clear();
    }

}

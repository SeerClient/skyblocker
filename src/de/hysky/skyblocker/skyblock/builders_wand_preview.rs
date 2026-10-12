//! Ported from de.hysky.skyblocker.skyblock.BuildersWandPreview by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::utils::block_pos_set::BlockPosSet;
use crate::de::hysky::skyblocker::utils::render::level_render_extraction_callback;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_collector::PrimitiveCollector;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::net::minecraft::core::block_pos::BlockPos;
use mcrust::net::minecraft::core::block_pos::MutableBlockPos;
use mcrust::net::minecraft::core::direction::Direction;
use mcrust::net::minecraft::world::item::items;
use mcrust::net::minecraft::world::level::block::block::Block;
use mcrust::net::minecraft::world::level::block::blocks;
use mcrust::net::minecraft::world::level::block::state::block_state::BlockState;
use mcrust::net::minecraft::world::level::level::Level;
use mcrust::net::minecraft::world::phys::block_hit_result::BlockHitResult;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;
use mcrust::net::minecraft::world::entity::entity::Entity as _;
use mcrust::net::minecraft::world::entity::living_entity::LivingEntity as _;
use mcrust::net::minecraft::world::level::block_getter::BlockGetter as _;

/// java: private static final int MAX_BLOCKS = 241;
pub const MAX_BLOCKS: i32 = 241;
/// java: public static final int PLOT_SIZE = 96;
pub const PLOT_SIZE: i32 = 96;
/// java: public static final int PLOT_OFFSET = 48;
pub const PLOT_OFFSET: i32 = 48;
/// java: private static final float[] RED = {1.0f, 0.0f, 0.0f};
pub static RED: LazyLock<Vec<f32>> = LazyLock::new(|| vec![1.0f32, 0.0f32, 0.0f32]);

/// java: public class BuildersWandPreview
#[derive(Clone)]
pub struct BuildersWandPreview {}

impl BuildersWandPreview {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        level_render_extraction_callback::EVENT.register(Arc::new(move |collector| {
            if !(SkyblockerConfigManager::get().helpers.builders_wand.enable_builders_wand_preview) || !(Utils::is_on_skyblock()) || mcrust::mcrust::modding::game::with_player(|_| ()).is_none() {
                return;
            }
            if !(Utils::is_in_private_island()) && !(Utils::is_in_garden()) {
                return;
            }
            // TODO(fabric2seer): an `instanceof` pattern under `||` / `!`
            //   java: if (!(client.hitResult instanceof BlockHitResult blockHitResult) || blockHitResult.getType() != HitResult.Type.BLOCK) return;
            let mut stack: mcrust::net::minecraft::world::item::item_stack::ItemStack = mcrust::mcrust::modding::game::with_player(|player| player.get_main_hand_item().clone()).unwrap();
            if !(stack.is_item(&items::items().get("blaze_rod"))) {
                return;
            }
            match stack.get_skyblock_id().as_str() {
                "BUILDERS_WAND" => {
                    // TODO(fabric2seer): unknown name `blockHitResult`
                    //   java: extractBuildersWandPreview(collector, blockHitResult);
                }
                "BUILDERS_RULER" => {
                    // TODO(fabric2seer): unknown name `blockHitResult`
                    //   java: extractBuildersRulerPreview(collector, blockHitResult);
                }
                _ => {}
            }
        }));
    }

    /// java: private static void extractBuildersWandPreview(PrimitiveCollector collector, BlockHitResult hitResult)
    pub fn extract_builders_wand_preview(collector: &compat::Shared<dyn PrimitiveCollector>, hit_result: &BlockHitResult) {
        if mcrust::mcrust::modding::game::level().is_none() {
            return;
        }
        let mut hit_pos: BlockPos = hit_result.get_block_pos();
        let mut side: Direction = hit_result.get_direction();
        if !(mcrust::mcrust::modding::game::level().unwrap().get_block_state_typed(&hit_pos.relative(side)).is_air()) {
            return;
        }
        let mut state: Arc<BlockState> = mcrust::mcrust::modding::game::level().unwrap().get_block_state_typed(&hit_pos);
        for pos in BlockPosSet::destroy_and_iterate_mut(&BuildersWandPreview::find_connected_faces(&mcrust::mcrust::modding::game::level().unwrap(), hit_pos.clone(), &side, &state)) {
            BuildersWandPreview::extract_block_preview(collector, &pos.immutable().relative(side), &state);
        }
    }

    /// java: private static BlockPosSet findConnectedFaces(Level world, BlockPos pos, Direction side, BlockState state)
    pub fn find_connected_faces(world: &Level, mut pos: BlockPos, side: &Direction, state: &Arc<BlockState>) -> compat::Shared<BlockPosSet> {
        let mut q: std::collections::VecDeque<BlockPos> = std::collections::VecDeque::new();
        let mut visited: compat::Shared<BlockPosSet> = BlockPosSet::new();
        q.push_back(pos.clone());
        BlockPosSet::add(&visited, &pos);
        while !q.is_empty() {
            let mut current: BlockPos = q.pop_front().unwrap();
            let mut mutable: MutableBlockPos = current.mutable();
            for dir in mcrust::net::minecraft::core::direction::VALUES {
                if dir == *side || dir == side.get_opposite() {
                    continue;
                }
                mutable.move_direction(dir);
                if !(BlockPosSet::contains_block_pos(&visited, &mutable.immutable())) && std::ptr::addr_eq(std::sync::Arc::as_ptr(&world.get_block_state_typed(&mutable.immutable())), std::sync::Arc::as_ptr(state)) {
                    if world.get_block_state_typed(&mutable.move_direction(*side).immutable()).is_air() {
                        let mut neighbor: BlockPos = mutable.move_direction(side.get_opposite()).immutable();
                        q.push_back(neighbor.clone());
                        BlockPosSet::add(&visited, &neighbor);
                    } else {
                        mutable.move_direction(side.get_opposite());
                    }
                }
                mutable.move_direction(dir.get_opposite());
            }
            if BlockPosSet::size(&visited) > MAX_BLOCKS {
                return BlockPosSet::new();
            }
        }
        return visited;
    }

    /// java: private static void extractBuildersRulerPreview(PrimitiveCollector collector, BlockHitResult hitResult)
    pub fn extract_builders_ruler_preview(collector: &compat::Shared<dyn PrimitiveCollector>, hit_result: &BlockHitResult) {
        if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() || mcrust::mcrust::modding::game::level().is_none() || !(Utils::is_in_garden()) {
            return;
        }
        let mut start_pos: BlockPos = hit_result.get_block_pos();
        let mut is_sneaking: bool = mcrust::mcrust::modding::game::with_player(|player| player.is_shift_key_down()).unwrap();
        let mut start_block: Arc<Block> = blocks::blocks().get("air");
        if !is_sneaking {
            start_pos = start_pos.relative(hit_result.get_direction());
        } else {
            start_block = mcrust::mcrust::modding::game::level().unwrap().get_block_state_typed(&start_pos).get_block();
        }
        let mut pos: MutableBlockPos = start_pos.mutable();
        let mut dir: Direction = hit_result.get_direction().get_opposite();
        if dir == Direction::Up || dir == Direction::Down || !is_sneaking {
            dir = mcrust::mcrust::modding::game::with_player(|player| player.base.base.base.base.get_direction()).unwrap();
        }
        {
            let mut i: i32 = 0;
            while i < MAX_BLOCKS && BuildersWandPreview::check_pos(&start_pos, &pos, &mcrust::mcrust::modding::game::level().unwrap().get_block_state_typed(&pos.immutable()), is_sneaking, &start_block) {
                if is_sneaking {
                    ({ let __f = compat::dispatch(collector, |__t| __t.submit_filled_box()); __f(collector, &pos.immutable(), &RED, SkyblockerConfigManager::get().helpers.builders_wand.preview_opacity, true) });
                } else {
                    BuildersWandPreview::extract_block_preview(collector, &pos.immutable(), &blocks::blocks().get("dirt").default_block_state());
                }
                pos.move_direction(dir);
                i += 1;
            }
        }
    }

    /// java: private static boolean checkPos(BlockPos plotPos, BlockPos.MutableBlockPos pos, BlockState state, boolean isSneaking, Block startBlock)
    pub fn check_pos(plot_pos: &BlockPos, pos: &MutableBlockPos, state: &Arc<BlockState>, mut is_sneaking: bool, start_block: &Arc<Block>) -> bool {
        return (BuildersWandPreview::is_in_plot(plot_pos, &pos.immutable()) && if is_sneaking { state.is_block(start_block) } else { state.is_air() });
    }

    /// java: private static boolean isInPlot(BlockPos plotPos, BlockPos pos)
    pub fn is_in_plot(plot_pos: &BlockPos, pos: &BlockPos) -> bool {
        return (compat::floor_div_i32(plot_pos.get_x().wrapping_add(PLOT_OFFSET), PLOT_SIZE) == compat::floor_div_i32(pos.get_x().wrapping_add(PLOT_OFFSET), PLOT_SIZE) && compat::floor_div_i32(plot_pos.get_z().wrapping_add(PLOT_OFFSET), PLOT_SIZE) == compat::floor_div_i32(pos.get_z().wrapping_add(PLOT_OFFSET), PLOT_SIZE));
    }

    /// java: private static void extractBlockPreview(PrimitiveCollector collector, BlockPos pos, BlockState state)
    pub fn extract_block_preview(collector: &compat::Shared<dyn PrimitiveCollector>, pos: &BlockPos, state: &Arc<BlockState>) {
        ({ let __f = compat::dispatch(collector, |__t| __t.submit_block_hologram()); __f(collector, pos, state, SkyblockerConfigManager::get().helpers.builders_wand.preview_opacity) });
    }

}

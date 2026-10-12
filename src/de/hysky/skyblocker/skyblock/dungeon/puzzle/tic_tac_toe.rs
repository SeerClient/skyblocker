//! Ported from de.hysky.skyblocker.skyblock.dungeon.puzzle.TicTacToe by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::dungeon::puzzle::dungeon_puzzle::DungeonPuzzle;
use crate::de::hysky::skyblocker::skyblock::dungeon::puzzle::dungeon_puzzle::DungeonPuzzleBase;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::dungeon_manager::DungeonManager;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::room::Room;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_collector::PrimitiveCollector;
use crate::de::hysky::skyblocker::utils::render::render_helper::RenderHelper;
use crate::de::hysky::skyblocker::utils::render::renderable::Renderable;
use crate::de::hysky::skyblocker::utils::resettable::Resettable;
use crate::de::hysky::skyblocker::utils::tickable::Tickable;
use crate::de::hysky::skyblocker::utils::tictactoe::board_index::BoardIndex;
use crate::de::hysky::skyblocker::utils::tictactoe::tic_tac_toe_utils::TicTacToeUtils;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::net::minecraft::core::block_pos::BlockPos;
use mcrust::net::minecraft::world::entity::decoration::item_frame::ItemFrame;
use mcrust::net::minecraft::world::phys::aabb::AABB;

/// java: private static final float[] GREEN_COLOR_COMPONENTS = { 0.0f, 1.0f, 0.0f };
pub static GREEN_COLOR_COMPONENTS: LazyLock<Vec<f32>> = LazyLock::new(|| vec![0.0f32, 1.0f32, 0.0f32]);
/// java: @SuppressWarnings("unused") private static final TicTacToe INSTANCE = new TicTacToe();
pub static INSTANCE: compat::MainThread<compat::Shared<TicTacToe>> = compat::MainThread::new(|| TicTacToe::new());
/// java: private static @Nullable AABB nextBestMoveToMake = null;
pub static NEXT_BEST_MOVE_TO_MAKE: compat::MainThread<Option<AABB>> = compat::MainThread::new(|| None);

/// java: public class TicTacToe extends DungeonPuzzle
#[derive(Clone)]
pub struct TicTacToe {
    /// java: extends DungeonPuzzle
    pub base: DungeonPuzzleBase,
}

impl std::ops::Deref for TicTacToe {
    type Target = DungeonPuzzleBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for TicTacToe {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl TicTacToe {
    /// java: private TicTacToe()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = DungeonPuzzleBase::new_in("tic-tac-toe".to_string(), vec!["tic-tac-toe-1".to_string()], move |__base| {
            let __h = compat::shared(Self {
                base: __base,
            });
            ((__h.clone() as compat::Shared<dyn DungeonPuzzle>), __h)
        });
        __this
    }

    /// java: @Init public static void init()
    pub fn init() {
    }

}

impl DungeonPuzzle for TicTacToe {
    fn dungeon_puzzle_base(&self) -> &DungeonPuzzleBase {
        &self.base
    }
    fn dungeon_puzzle_base_mut(&mut self) -> &mut DungeonPuzzleBase {
        &mut self.base
    }

}

impl Tickable for TicTacToe {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void tick(Minecraft client) (dispatch: the body for this class)
    fn tick(&self) -> fn(&compat::Shared<dyn Tickable>, &mut mcrust::net::minecraft::client::minecraft::Minecraft) {
        TicTacToe::tick_dyn
    }

}

impl Renderable for TicTacToe {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void extractRendering(PrimitiveCollector collector) (dispatch: the body for this class)
    fn extract_rendering(&self) -> fn(&compat::Shared<dyn Renderable>, &compat::Shared<dyn PrimitiveCollector>) {
        TicTacToe::extract_rendering_dyn
    }

}

impl Resettable for TicTacToe {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void reset() (dispatch: the body for this class)
    fn reset(&self) -> fn(&compat::Shared<dyn Resettable>) {
        <dyn DungeonPuzzle>::reset_from::<Self>
    }

}

impl TicTacToe {
    /// java: @Override public void tick(Minecraft client)
    pub fn tick_impl(__this: &compat::Shared<Self>, client: &mut mcrust::net::minecraft::client::minecraft::Minecraft) {
        if !({ let __f = compat::dispatch(__this, |__t| __t.should_solve()); __f(&compat::view(__this, |a| a as compat::Shared<dyn DungeonPuzzle>)) }) {
            return;
        }
        *NEXT_BEST_MOVE_TO_MAKE.get_mut() = None;
        if mcrust::mcrust::modding::game::level().is_none() || mcrust::mcrust::modding::game::with_player(|_| ()).is_none() || !(Utils::is_in_dungeons()) {
            return;
        }
        let mut search_box: AABB = AABB::new((mcrust::mcrust::modding::game::with_player(|player| player.base.base.base.base.get_x()).unwrap() - (21 as f64)), (mcrust::mcrust::modding::game::with_player(|player| player.base.base.base.base.get_y()).unwrap() - (21 as f64)), (mcrust::mcrust::modding::game::with_player(|player| player.base.base.base.base.get_z()).unwrap() - (21 as f64)), (mcrust::mcrust::modding::game::with_player(|player| player.base.base.base.base.get_x()).unwrap() + (21 as f64)), (mcrust::mcrust::modding::game::with_player(|player| player.base.base.base.base.get_y()).unwrap() + (21 as f64)), (mcrust::mcrust::modding::game::with_player(|player| player.base.base.base.base.get_z()).unwrap() + (21 as f64)));
        // TODO(fabric2seer): `.hasFramedMap(..)` on a T
        //   java: List<ItemFrame> itemFramesThatHoldMaps = client.level.getEntitiesOfClass(ItemFrame.class, searchBox, ItemFrame::hasFramedMap);
        let mut item_frames_that_hold_maps: Vec<ItemFrame> = todo!("fabric2seer: the initialiser of `itemFramesThatHoldMaps` (TODO above)");
        // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if (item_frames_that_hold_maps.len() as i32) != 9 && ((item_frames_that_hold_maps.len() as i32) & 1) == 1 {
                let mut board: Vec<Vec<char>> = vec![vec!['\0'; (3) as usize]; (3) as usize];
                for item_frame in item_frames_that_hold_maps.iter() {
                    let Some(mut map_state) = mcrust::mcrust::modding::game::level().unwrap().get_map_data(&item_frame.get_framed_map_id(&item_frame.get_item()).unwrap()) else {
                        continue;
                    };
                    let mut relative: BlockPos = ({ let __r = &DungeonManager::get_current_room().unwrap(); let __f = compat::dispatch(__r, |__t| __t.actual_to_relative()); __f(__r, &item_frame.base.block_position()) });
                    let mut y: i32 = relative.get_y();
                    let mut row: i32 = match y {
                        72 => 0,
                        71 => 1,
                        70 => 2,
                        _ => -1,
                    };
                    let mut z: i32 = relative.get_z();
                    let mut column: i32 = match z {
                        17 => 0,
                        16 => 1,
                        15 => 2,
                        _ => -1,
                    };
                    if row == -1 || column == -1 {
                        continue;
                    }
                    let mut middle_color: i32 = ((map_state.colors[(8256) as usize] as i32) & 0xFF);
                    if middle_color == 114 {
                        (board[(row) as usize])[(column) as usize] = 'X';
                    } else if middle_color == 33 {
                        (board[(row) as usize])[(column) as usize] = 'O';
                    }
                }
                let mut best_move: BoardIndex = TicTacToeUtils::get_best_move(&board);
                let mut next_x: f64 = ((8) as f64) as f64;
                let mut next_y: f64 = ((72i32.wrapping_sub(best_move.row())) as f64) as f64;
                let mut next_z: f64 = ((17i32.wrapping_sub(best_move.column())) as f64) as f64;
                let mut next_pos: BlockPos = ({ let __r = &DungeonManager::get_current_room().unwrap(); let __f = compat::dispatch(__r, |__t| __t.relative_to_actual()); __f(__r, &BlockPos::containing(next_x, next_y, next_z)) });
                *NEXT_BEST_MOVE_TO_MAKE.get_mut() = RenderHelper::get_block_bounding_box(&mcrust::mcrust::modding::game::level().unwrap(), &next_pos);
            }
        }));
        if let Err(e) = __result {
            logger().error(&format!("[Skyblocker Tic Tac Toe] Encountered an exception while determining a tic tac toe solution!"));
        }
    }

    /// TicTacToe.tick as the dispatch of Tickable.tick
    pub fn tick_dyn(__this: &compat::Shared<dyn Tickable>, __p0: &mut mcrust::net::minecraft::client::minecraft::Minecraft) {
        Self::tick_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public void extractRendering(PrimitiveCollector collector)
    pub fn extract_rendering_impl(__this: &compat::Shared<Self>, collector: &compat::Shared<dyn PrimitiveCollector>) {
        // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if SkyblockerConfigManager::get().dungeons.puzzle_solvers.solve_tic_tac_toe && NEXT_BEST_MOVE_TO_MAKE.get_mut().is_some() {
                ({ let __f = compat::dispatch(collector, |__t| __t.submit_filled_box_aabb_float_float_boolean()); __f(collector, NEXT_BEST_MOVE_TO_MAKE.get_mut().as_ref().unwrap(), &GREEN_COLOR_COMPONENTS, 0.5f32, false) });
                ({ let __f = compat::dispatch(collector, |__t| __t.submit_outlined_box_aabb_float_float_boolean()); __f(collector, NEXT_BEST_MOVE_TO_MAKE.get_mut().as_ref().unwrap(), &GREEN_COLOR_COMPONENTS, 5.0f32, false) });
            }
        }));
        if let Err(e) = __result {
            logger().error(&format!("[Skyblocker Tic Tac Toe] Encountered an exception while rendering the tic tac toe solution!"));
        }
    }

    /// TicTacToe.extractRendering as the dispatch of Renderable.extractRendering
    pub fn extract_rendering_dyn(__this: &compat::Shared<dyn Renderable>, __p0: &compat::Shared<dyn PrimitiveCollector>) {
        Self::extract_rendering_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

}

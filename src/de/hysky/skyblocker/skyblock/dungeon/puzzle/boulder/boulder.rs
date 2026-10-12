//! Ported from de.hysky.skyblocker.skyblock.dungeon.puzzle.boulder.Boulder by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::dungeon::puzzle::boulder::boulder_board::BoulderBoard;
use crate::de::hysky::skyblocker::skyblock::dungeon::puzzle::boulder::boulder_object::BoulderObject;
use crate::de::hysky::skyblocker::skyblock::dungeon::puzzle::boulder::boulder_solver::BoulderSolver;
use crate::de::hysky::skyblocker::skyblock::dungeon::puzzle::boulder::boulder_solver::GameState;
use crate::de::hysky::skyblocker::skyblock::dungeon::puzzle::dungeon_puzzle::DungeonPuzzle;
use crate::de::hysky::skyblocker::skyblock::dungeon::puzzle::dungeon_puzzle::DungeonPuzzleBase;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::dungeon_manager::DungeonManager;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::room::Room;
use crate::de::hysky::skyblocker::utils::color_utils::ColorUtils;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_collector::PrimitiveCollector;
use crate::de::hysky::skyblocker::utils::render::renderable::Renderable;
use crate::de::hysky::skyblocker::utils::render::title::title::Title;
use crate::de::hysky::skyblocker::utils::render::title::title_container::TitleContainer;
use crate::de::hysky::skyblocker::utils::resettable::Resettable;
use crate::de::hysky::skyblocker::utils::tickable::Tickable;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::multiplayer::client_level::ClientLevel;
use mcrust::net::minecraft::core::block_pos::BlockPos;
use mcrust::net::minecraft::world::item::dye_color::DyeColor;
use mcrust::net::minecraft::world::level::block::block::Block;
use mcrust::net::minecraft::world::level::block::blocks;
use mcrust::net::minecraft::world::phys::aabb::AABB;
use mcrust::net::minecraft::world::phys::vec3::Vec3;

/// java: @SuppressWarnings("unused") private static final Boulder INSTANCE = new Boulder();
pub static INSTANCE: compat::MainThread<compat::Shared<Boulder>> = compat::MainThread::new(|| Boulder::new());
/// java: private static final float[] RED_COLOR_COMPONENTS = ColorUtils.getFloatComponents(DyeColor.RED);
pub static RED_COLOR_COMPONENTS: LazyLock<Vec<f32>> = LazyLock::new(|| ColorUtils::get_float_components_dye_color(&DyeColor::Red));
/// java: private static final float[] ORANGE_COLOR_COMPONENTS = ColorUtils.getFloatComponents(DyeColor.ORANGE);
pub static ORANGE_COLOR_COMPONENTS: LazyLock<Vec<f32>> = LazyLock::new(|| ColorUtils::get_float_components_dye_color(&DyeColor::Orange));
/// java: protected static final int BASE_Y = 65;
pub const BASE_Y: i32 = 65;
/// java: static @Nullable AABB boundingBox;
pub static BOUNDING_BOX: compat::MainThread<Option<AABB>> = compat::MainThread::new(|| None);

/// java: public class Boulder extends DungeonPuzzle
#[derive(Clone)]
pub struct Boulder {
    /// java: extends DungeonPuzzle
    pub base: DungeonPuzzleBase,
}

impl std::ops::Deref for Boulder {
    type Target = DungeonPuzzleBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for Boulder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl Boulder {
    /// java: private Boulder()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = DungeonPuzzleBase::new_in("boulder".to_string(), vec!["boxes-room".to_string()], move |__base| {
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

    /// java: private static List<BoulderSolver.GameState> getInitialStates(BoulderBoard board)
    pub fn get_initial_states(board: &BoulderBoard) -> Vec<GameState> {
        let mut board_array: Vec<Vec<char>> = board.get_board_char_array();
        return vec![GameState::new(board_array.clone(), board.get_height().wrapping_sub(1), 0, 0), GameState::new(board_array.clone(), board.get_height().wrapping_sub(1), 1, 0), GameState::new(board_array.clone(), board.get_height().wrapping_sub(1), 2, 0), GameState::new(board_array.clone(), board.get_height().wrapping_sub(1), 3, 0), GameState::new(board_array.clone(), board.get_height().wrapping_sub(1), 4, 0), GameState::new(board_array.clone(), board.get_height().wrapping_sub(1), 5, 0), GameState::new(board_array.clone(), board.get_height().wrapping_sub(1), 6, 0)];
    }

    /// java: public static String getBlockType(ClientLevel world, Room room, int x, int y, int z)
    pub fn get_block_type(world: &ClientLevel, room: &compat::Shared<dyn Room>, mut x: i32, mut y: i32, mut z: i32) -> String {
        let mut block: Arc<Block> = (world.get_block_state_typed(&({ let __f = compat::dispatch(room, |__t| __t.relative_to_actual()); __f(room, &BlockPos::new(x, y, z)) }))).get_block();
        return (if std::ptr::addr_eq(std::sync::Arc::as_ptr(&block), std::sync::Arc::as_ptr(&blocks::blocks().get("BIRCH_PLANKS"))) || std::ptr::addr_eq(std::sync::Arc::as_ptr(&block), std::sync::Arc::as_ptr(&blocks::blocks().get("JUNGLE_PLANKS"))) { "B" } else { "." }).to_string();
    }

    /// java: private static @Nullable BlockPos checkForButtonBlocksOnLine(ClientLevel world, Vec3 point1, Vec3 point2)
    pub fn check_for_button_blocks_on_line(world: &ClientLevel, point1: &Vec3, point2: &Vec3) -> Option<BlockPos> {
        let mut x1: f64 = point1.x();
        let mut y1: f64 = (point1.y() + (1 as f64));
        let mut z1: f64 = point1.z();
        let mut x2: f64 = point2.x();
        let mut y2: f64 = (point2.y() + (1 as f64));
        let mut z2: f64 = point2.z();
        let mut steps: i32 = ((((x2 - x1).abs()).max(((y2 - y1).abs()).max((z2 - z1).abs()))) as i32);
        let mut x_step: f64 = ((x2 - x1) / (steps as f64));
        let mut y_step: f64 = ((y2 - y1) / (steps as f64));
        let mut z_step: f64 = ((z2 - z1) / (steps as f64));
        for step in 0..=steps {
            let mut current_x: f64 = (x1 + ((step as f64) * x_step));
            let mut current_y: f64 = (y1 + ((step as f64) * y_step));
            let mut current_z: f64 = (z1 + ((step as f64) * z_step));
            let mut block_pos: BlockPos = BlockPos::containing(current_x, current_y, current_z);
            let mut block: Arc<Block> = world.get_block_state_typed(&block_pos).get_block();
            if std::ptr::addr_eq(std::sync::Arc::as_ptr(&block), std::sync::Arc::as_ptr(&blocks::blocks().get("STONE_BUTTON"))) {
                return Some(block_pos.clone());
            }
        }
        return None;
    }

}

impl DungeonPuzzle for Boulder {
    fn dungeon_puzzle_base(&self) -> &DungeonPuzzleBase {
        &self.base
    }
    fn dungeon_puzzle_base_mut(&mut self) -> &mut DungeonPuzzleBase {
        &mut self.base
    }

}

impl Tickable for Boulder {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void tick(Minecraft client) (dispatch: the body for this class)
    fn tick(&self) -> fn(&compat::Shared<dyn Tickable>, &mut mcrust::net::minecraft::client::minecraft::Minecraft) {
        Boulder::tick_dyn
    }

}

impl Renderable for Boulder {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void extractRendering(PrimitiveCollector collector) (dispatch: the body for this class)
    fn extract_rendering(&self) -> fn(&compat::Shared<dyn Renderable>, &compat::Shared<dyn PrimitiveCollector>) {
        Boulder::extract_rendering_dyn
    }

}

impl Resettable for Boulder {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void reset() (dispatch: the body for this class)
    fn reset(&self) -> fn(&compat::Shared<dyn Resettable>) {
        Boulder::reset_dyn
    }

}

impl Boulder {
    /// java: @Override public void tick(Minecraft client)
    pub fn tick_impl(__this: &compat::Shared<Self>, client: &mut mcrust::net::minecraft::client::minecraft::Minecraft) {
        if !({ let __f = compat::dispatch(__this, |__t| __t.should_solve()); __f(&compat::view(__this, |a| a as compat::Shared<dyn DungeonPuzzle>)) }) || !(SkyblockerConfigManager::get().dungeons.puzzle_solvers.solve_boulder) || mcrust::mcrust::modding::game::level().is_none() || !(DungeonManager::is_current_room_matched()) {
            return;
        }
        let Some(mut room) = DungeonManager::get_current_room() else {
            return;
        };
        let mut chest_pos: BlockPos = BlockPos::new(15, BASE_Y, 29);
        let mut start: BlockPos = BlockPos::new(25, BASE_Y, 25);
        let mut end: BlockPos = BlockPos::new(5, BASE_Y, 8);
        let mut board: BoulderBoard = BoulderBoard::new(7, 7);
        let mut column: i32 = 0;
        {
            let mut z: i32 = start.get_z();
            while z > end.get_z() {
                let mut row: i32 = 0;
                {
                    let mut x: i32 = start.get_x();
                    while x > end.get_x() {
                        if ((start.get_x().wrapping_sub(x)).wrapping_abs()).wrapping_rem(3) == 1 && ((start.get_z().wrapping_sub(z)).wrapping_abs()).wrapping_rem(3) == 1 {
                            let mut block_type: String = Boulder::get_block_type(&mcrust::mcrust::modding::game::level().unwrap(), &room, x, BASE_Y, z);
                            board.place_object(column, row, BoulderObject::new(x, BASE_Y, z, block_type.clone()));
                            row += 1;
                        }
                        x -= 1;
                    }
                }
                if row == board.get_width() {
                    column += 1;
                }
                z -= 1;
            }
        }
        let mut initial_states: Vec<GameState> = Boulder::get_initial_states(&board);
        let mut solution: Option<Vec<Vec<i32>>> = BoulderSolver::a_star_solve(&initial_states);
        if let Some(solution) = &solution {
            // TODO(fabric2seer): assignment to unknown `linePoints`
            //   java: linePoints = new Vec3[solution.size()];
            let mut index: i32 = 0;
            for coord in solution.iter() {
                let mut x: i32 = coord[(0) as usize];
                let mut y: i32 = coord[(1) as usize];
                // TODO(fabric2seer): unknown name `linePoints`
                //   java: linePoints[index++] = Vec3.atCenterOf(room.relativeToActual(board.getObject3DPosition(x, y)));
            }
            let mut button: Option<BlockPos> = None;
            // TODO(fabric2seer): the condition: unknown name `linePoints`
            //   java: linePoints != null && linePoints.length > 0
            if todo!("fabric2seer: the condition (TODO above)") {
                // TODO(fabric2seer): unknown name `linePoints`
                //   java: for (int i = 0; i < linePoints.length - 1; i++) {
                //   java: 					Vec3 point1 = linePoints[i];
                //   java: 					Vec3 point2 = linePoints[i + 1];
                //   java: 					button = checkForButtonBlocksOnLine(client.level, point1, point2);
                //   java: 					if (button != null) {
                //   java: 						// If a button is found, calculate its bounding box
                //   java: 						boundingBox = RenderHelper.getBlockBoundingBox(client.level, button);
                //   java: 						break;
                //   java: 					}
                //   java: 				}
                if let Some(button) = &button {
                } else {
                    ({ let __f = compat::dispatch(__this, |__t| __t.reset()); __f(&compat::view(__this, |a| a as compat::Shared<dyn Resettable>)) });
                }
            }
        } else {
            let mut title: compat::Shared<Title> = Title::new("skyblocker.dungeons.puzzle.boulder.noSolution".to_string(), ChatFormatting::Green);
            TitleContainer::add_title_and_play_sound_2(&title, 15);
            ({ let __f = compat::dispatch(__this, |__t| __t.reset()); __f(&compat::view(__this, |a| a as compat::Shared<dyn Resettable>)) });
        }
    }

    /// Boulder.tick as the dispatch of Tickable.tick
    pub fn tick_dyn(__this: &compat::Shared<dyn Tickable>, __p0: &mut mcrust::net::minecraft::client::minecraft::Minecraft) {
        Self::tick_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public void extractRendering(PrimitiveCollector collector)
    pub fn extract_rendering_impl(__this: &compat::Shared<Self>, collector: &compat::Shared<dyn PrimitiveCollector>) {
        if !({ let __f = compat::dispatch(__this, |__t| __t.should_solve()); __f(&compat::view(__this, |a| a as compat::Shared<dyn DungeonPuzzle>)) }) || !(SkyblockerConfigManager::get().dungeons.puzzle_solvers.solve_boulder) || !(DungeonManager::is_current_room_matched()) {
            return;
        }
        let mut alpha: f32 = 1.0f32;
        let mut line_width: f32 = 5.0f32;
        // TODO(fabric2seer): the condition: unknown name `linePoints`
        //   java: linePoints != null && linePoints.length > 0
        if todo!("fabric2seer: the condition (TODO above)") {
            // TODO(fabric2seer): unknown name `linePoints`
            //   java: for (int i = 0; i < linePoints.length - 1; i++) {
            //   java: 				Vec3 startPoint = linePoints[i];
            //   java: 				Vec3 endPoint = linePoints[i + 1];
            //   java: 				collector.submitLinesFromPoints(new Vec3[]{startPoint, endPoint}, ORANGE_COLOR_COMPONENTS, alpha, lineWidth, true);
            //   java: 			}
            if BOUNDING_BOX.get_mut().is_some() {
                ({ let __f = compat::dispatch(collector, |__t| __t.submit_filled_box_aabb_float_float_boolean()); __f(collector, BOUNDING_BOX.get_mut().as_ref().unwrap(), &RED_COLOR_COMPONENTS, 0.5f32, false) });
                ({ let __f = compat::dispatch(collector, |__t| __t.submit_outlined_box_aabb_float_float_boolean()); __f(collector, BOUNDING_BOX.get_mut().as_ref().unwrap(), &RED_COLOR_COMPONENTS, 5.0f32, false) });
            }
        }
    }

    /// Boulder.extractRendering as the dispatch of Renderable.extractRendering
    pub fn extract_rendering_dyn(__this: &compat::Shared<dyn Renderable>, __p0: &compat::Shared<dyn PrimitiveCollector>) {
        Self::extract_rendering_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public void reset()
    pub fn reset_impl(__this: &compat::Shared<Self>) {
        <dyn DungeonPuzzle>::reset_body(&compat::view(__this, |a| a as compat::Shared<dyn DungeonPuzzle>));
        // TODO(fabric2seer): assignment to unknown `linePoints`
        //   java: linePoints = null;
        *BOUNDING_BOX.get_mut() = None;
    }

    /// Boulder.reset as the dispatch of Resettable.reset
    pub fn reset_dyn(__this: &compat::Shared<dyn Resettable>) {
        Self::reset_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}

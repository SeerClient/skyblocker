//! Ported from de.hysky.skyblocker.skyblock.dungeon.puzzle.IceFill by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::dungeon::puzzle::dungeon_puzzle::DungeonPuzzle;
use crate::de::hysky::skyblocker::skyblock::dungeon::puzzle::dungeon_puzzle::DungeonPuzzleBase;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::dungeon_manager::DungeonManager;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::room::Room;
use crate::de::hysky::skyblocker::utils::color_utils::ColorUtils;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_collector::PrimitiveCollector;
use crate::de::hysky::skyblocker::utils::render::renderable::Renderable;
use crate::de::hysky::skyblocker::utils::resettable::Resettable;
use crate::de::hysky::skyblocker::utils::tickable::Tickable;
use mcrust::java::util::concurrent::completable_future::CompletableFuture;
use mcrust::net::minecraft::core::block_pos::BlockPos;
use mcrust::net::minecraft::core::block_pos::MutableBlockPos;
use mcrust::net::minecraft::core::direction::Direction;
use mcrust::net::minecraft::world::item::dye_color::DyeColor;
use mcrust::net::minecraft::world::level::level::Level;
use mcrust::net::minecraft::world::phys::vec3;
use mcrust::net::minecraft::world::phys::vec3::Vec3;
use mcrust::org::joml::vector2i::Vector2i;
use mcrust::net::minecraft::world::level::block_getter::BlockGetter as _;

/// java: public static final IceFill INSTANCE = new IceFill();
pub static INSTANCE: compat::MainThread<compat::Shared<IceFill>> = compat::MainThread::new(|| IceFill::new());
/// java: private static final float[] RED_COLOR_COMPONENTS = ColorUtils.getFloatComponents(DyeColor.RED);
pub static RED_COLOR_COMPONENTS: LazyLock<Vec<f32>> = LazyLock::new(|| ColorUtils::get_float_components_dye_color(&DyeColor::Red));
/// java: private static final BlockPos[] BOARD_ORIGINS = { new BlockPos(16, 70, 9), new BlockPos(17, 71, 16), new BlockPos(18, 72, 25) };
pub static BOARD_ORIGINS: compat::MainThread<Vec<BlockPos>> = compat::MainThread::new(|| vec![BlockPos::new(16, 70, 9), BlockPos::new(17, 71, 16), BlockPos::new(18, 72, 25)]);

/// java: public class IceFill extends DungeonPuzzle
#[derive(Clone)]
pub struct IceFill {
    /// java: extends DungeonPuzzle
    pub base: DungeonPuzzleBase,
    /// java: private @Nullable CompletableFuture<Void> solve;
    pub solve: Option<mcrust::java::util::concurrent::completable_future::CompletableFuture<()>>,
    /// java: private final boolean[][][] iceFillBoards = {new boolean[3][3], new boolean[5][5], new boolean[7][7]};
    pub ice_fill_boards: Vec<Vec<Vec<bool>>>,
    /// java: private final List<List<Vector2ic>> iceFillPaths = new ArrayList<>(List.of(List.of(), List.of(), List.of()));
    pub ice_fill_paths: Vec<Vec<Vector2i>>,
}

impl std::ops::Deref for IceFill {
    type Target = DungeonPuzzleBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for IceFill {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl IceFill {
    /// java: private IceFill()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = DungeonPuzzleBase::new_in("ice-fill".to_string(), vec!["ice-path".to_string()], move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                solve: None,
                ice_fill_boards: vec![vec![vec![false; (3) as usize]; (3) as usize], vec![vec![false; (5) as usize]; (5) as usize], vec![vec![false; (7) as usize]; (7) as usize]],
                ice_fill_paths: vec![vec![], vec![], vec![]].into_iter().collect::<Vec<_>>(),
            });
            ((__h.clone() as compat::Shared<dyn DungeonPuzzle>), __h)
        });
        __this
    }

    /// java: @Init public static void init()
    pub fn init() {
        if crate::de::hysky::skyblocker::debug::debug::Debug::debug_enabled() {
            client_command_registration_callback::EVENT.register(Arc::new(
                |dispatcher: &mut client_command_registration_callback::ClientCommandDispatcher| {
                    // TODO(fabric2seer): a command tree this tool cannot read (arguments, redirects, requires) - SeerClient commands receive the words after the name in `ctx.args`
                    //   java: dispatcher.register(..);
                },
            ));
        }
    }

    /// java: public static String boardToString(boolean[][] iceFillBoard)
    pub fn board_to_string(ice_fill_board: &Vec<Vec<bool>>) -> String {
        let mut sb: String = String::new();
        for row in ice_fill_board.iter() {
            { sb.push_str("\n"); };
            for cell in row.iter().copied() {
                { sb.push((if cell { '#' } else { '.' })); };
            }
        }
        return sb.clone();
    }

    /// java: private boolean updateBoard(Level world, Room room, boolean[][] iceFillBoard, BlockPos.MutableBlockPos pos)
    pub fn update_board(__this: &compat::Shared<Self>, world: &Level, room: &compat::Shared<dyn Room>, ice_fill_board: &mut Vec<Vec<bool>>, pos: &mut MutableBlockPos) -> bool {
        let mut board_changed: bool = false;
        {
            let mut row: i32 = 0;
            while row < (ice_fill_board.len() as i32) {
                {
                    let mut col: i32 = 0;
                    while col < ((ice_fill_board[(row) as usize]).len() as i32) {
                        let mut actual_pos: BlockPos = ({ let __f = compat::dispatch(room, |__t| __t.relative_to_actual()); __f(room, &pos.immutable()) });
                        // Don't solve the board if the block below is air
                        if world.get_block_state_typed(&actual_pos.below()).is_air() {
                            return false;
                        }
                        let mut is_block: bool = !(world.get_block_state_typed(&actual_pos).is_air());
                        if (ice_fill_board[(row) as usize])[(col) as usize] != is_block {
                            (ice_fill_board[(row) as usize])[(col) as usize] = is_block;
                            board_changed = true;
                        }
                        pos.move_direction(Direction::West);
                        col += 1;
                    }
                }
                pos.move_by(((ice_fill_board[(row) as usize]).len() as i32), 0, -1);
                row += 1;
            }
        }
        return board_changed;
    }

    /// java: List<Vector2ic> solve(boolean[][] iceFillBoard)
    pub fn solve(__this: &compat::Shared<Self>, ice_fill_board: &Vec<Vec<bool>>) -> Vec<Vector2i> {
        let mut start: Vector2i = Vector2i::new((ice_fill_board.len() as i32).wrapping_sub(1), ((ice_fill_board[(0) as usize]).len() as i32).wrapping_div(2));
        // int count = iceFillBoard.length * iceFillBoard[0].length - Arrays.stream(iceFillBoard).mapToInt(Booleans::countTrue).sum();
        let mut count: i32 = ((ice_fill_board.len() as i32).wrapping_mul((ice_fill_board[(0) as usize]).len() as i32)).wrapping_sub(ice_fill_board.iter().map(|row| row.iter().filter(|b| **b).count() as i32).sum::<i32>());
        let mut visited: Vec<Vec<bool>> = vec![vec![false; (((ice_fill_board[(0) as usize]).len() as i32)) as usize]; ((ice_fill_board.len() as i32)) as usize];
        (visited[(start.x()) as usize])[(start.y()) as usize] = true;
        let mut new_path: Option<Vec<Vector2i>> = IceFill::solve_dfs(__this, ice_fill_board, count.wrapping_sub(1), &mut vec![start.clone()], &mut visited);
        return (if new_path.is_some() { new_path.clone() } else { Some(vec![]) }).unwrap();
    }

    /// java: private @Nullable List<Vector2ic> solveDfs(boolean[][] iceFillBoard, int count, List<Vector2ic> path, boolean[][] visited)
    pub fn solve_dfs(__this: &compat::Shared<Self>, ice_fill_board: &Vec<Vec<bool>>, mut count: i32, path: &mut Vec<Vector2i>, visited: &mut Vec<Vec<bool>>) -> Option<Vec<Vector2i>> {
        let pos: Vector2i = path[path.len() - 1];
        if count == 0 {
            if pos.x() == 0 && pos.y() == ((ice_fill_board[(0) as usize]).len() as i32).wrapping_div(2) {
                return Some(path.clone());
            } else {
                return None;
            }
        }
        // Vector2ic[] newPosArray = {pos.add(1, 0, new Vector2i()), pos.add(-1, 0, new Vector2i()), pos.add(0, 1, new Vector2i()), pos.add(0, -1, new Vector2i())};
        let mut new_pos_array: Vec<Vector2i> = vec![
            *pos.add_dest(1, 0, &mut Vector2i::zero()),
            *pos.add_dest(-1, 0, &mut Vector2i::zero()),
            *pos.add_dest(0, 1, &mut Vector2i::zero()),
            *pos.add_dest(0, -1, &mut Vector2i::zero()),
        ];
        for new_pos in new_pos_array.iter() {
            if new_pos.x() >= 0 && new_pos.x() < (ice_fill_board.len() as i32) && new_pos.y() >= 0 && new_pos.y() < ((ice_fill_board[(0) as usize]).len() as i32) && !((ice_fill_board[(new_pos.x()) as usize])[(new_pos.y()) as usize]) && !((visited[(new_pos.x()) as usize])[(new_pos.y()) as usize]) {
                path.push(new_pos.clone());
                (visited[(new_pos.x()) as usize])[(new_pos.y()) as usize] = true;
                let mut new_path: Option<Vec<Vector2i>> = IceFill::solve_dfs(__this, ice_fill_board, count.wrapping_sub(1), path, visited);
                if let Some(new_path) = &new_path {
                    return Some(new_path.clone());
                }
                path.pop().unwrap();
                (visited[(new_pos.x()) as usize])[(new_pos.y()) as usize] = false;
            }
        }
        return None;
    }

    /// java: private void extractPath(PrimitiveCollector collector, Room room, List<Vector2ic> iceFillPath, BlockPos originPos)
    pub fn extract_path(__this: &compat::Shared<Self>, collector: &compat::Shared<dyn PrimitiveCollector>, room: &compat::Shared<dyn Room>, ice_fill_path: &Vec<Vector2i>, mut origin_pos: BlockPos) {
        let mut pos: MutableBlockPos = MutableBlockPos::new_origin();
        for i in 0..(ice_fill_path.len() as i32).wrapping_sub(1) {
            let mut start: Vec3 = vec3::at_center_of(&({ let __f = compat::dispatch(room, |__t| __t.relative_to_actual()); __f(room, &(pos.set_vec3i(origin_pos.as_vec3i()).move_by((-(ice_fill_path[(i) as usize]).y()), 0, (-(ice_fill_path[(i) as usize]).x()))).immutable()) }));
            let mut end: Vec3 = vec3::at_center_of(&({ let __f = compat::dispatch(room, |__t| __t.relative_to_actual()); __f(room, &(pos.set_vec3i(origin_pos.as_vec3i()).move_by((-(ice_fill_path[(i.wrapping_add(1)) as usize]).y()), 0, (-(ice_fill_path[(i.wrapping_add(1)) as usize]).x()))).immutable()) }));
            ({ let __f = compat::dispatch(collector, |__t| __t.submit_lines_from_points()); __f(collector, &vec![start.clone(), end.clone()], &RED_COLOR_COMPONENTS, 1.0f32, 5.0f32, true) });
        }
    }

}

impl DungeonPuzzle for IceFill {
    fn dungeon_puzzle_base(&self) -> &DungeonPuzzleBase {
        &self.base
    }
    fn dungeon_puzzle_base_mut(&mut self) -> &mut DungeonPuzzleBase {
        &mut self.base
    }

}

impl Tickable for IceFill {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void tick(Minecraft client) (dispatch: the body for this class)
    fn tick(&self) -> fn(&compat::Shared<dyn Tickable>, &mut mcrust::net::minecraft::client::minecraft::Minecraft) {
        IceFill::tick_dyn
    }

}

impl Renderable for IceFill {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void extractRendering(PrimitiveCollector collector) (dispatch: the body for this class)
    fn extract_rendering(&self) -> fn(&compat::Shared<dyn Renderable>, &compat::Shared<dyn PrimitiveCollector>) {
        IceFill::extract_rendering_dyn
    }

}

impl Resettable for IceFill {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void reset() (dispatch: the body for this class)
    fn reset(&self) -> fn(&compat::Shared<dyn Resettable>) {
        <dyn DungeonPuzzle>::reset_from::<Self>
    }

}

impl IceFill {
    /// java: @Override public void tick(Minecraft client)
    pub fn tick_impl(__this: &compat::Shared<Self>, client: &mut mcrust::net::minecraft::client::minecraft::Minecraft) {
        if !(SkyblockerConfigManager::get().dungeons.puzzle_solvers.solve_ice_fill) || mcrust::mcrust::modding::game::level().is_none() || !(DungeonManager::is_current_room_matched()) || compat::read(__this, |__t| __t.solve.clone()).is_some() && !(__this.lock().borrow().solve.as_ref().unwrap().is_done()) {
            return;
        }
        let Some(mut room) = DungeonManager::get_current_room() else {
            return;
        };
        // solve = CompletableFuture.runAsync(() -> {..}) - the task reads the room and writes this object's boards
        // and paths, which are the mod's render-thread objects in this port, so it runs through the game's executor
        // (`game::execute`: inline on the render thread) rather than on a pool thread.
        let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        let __room = compat::RenderThread::new(room.clone());
        let __v = Some(CompletableFuture::<()>::run_async(move || {
            let Some(__this_1) = __this_w1.get().upgrade() else { return; };
            let room = __room.get();
            let mut pos: MutableBlockPos = MutableBlockPos::new_origin();
            for i in 0..3 {
                let mut board = compat::read(&__this_1, |__t| __t.ice_fill_boards[(i) as usize].clone());
                let changed = IceFill::update_board(&__this_1, &mcrust::mcrust::modding::game::level().unwrap(), room, &mut board, pos.set_vec3i((BOARD_ORIGINS.get()[(i) as usize]).as_vec3i()));
                compat::write(&__this_1, |__t| __t.ice_fill_boards[(i) as usize] = board.clone());
                if changed {
                    let path = IceFill::solve(&__this_1, &board);
                    compat::write(&__this_1, |__t| __t.ice_fill_paths[(i) as usize] = path);
                }
            }
        }, |task: Box<dyn FnOnce() + Send>| crate::compat::execute(task)));
        compat::write(__this, |__t| __t.solve = __v);
    }

    /// IceFill.tick as the dispatch of Tickable.tick
    pub fn tick_dyn(__this: &compat::Shared<dyn Tickable>, __p0: &mut mcrust::net::minecraft::client::minecraft::Minecraft) {
        Self::tick_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public void extractRendering(PrimitiveCollector collector)
    pub fn extract_rendering_impl(__this: &compat::Shared<Self>, collector: &compat::Shared<dyn PrimitiveCollector>) {
        if !(SkyblockerConfigManager::get().dungeons.puzzle_solvers.solve_ice_fill) || !(DungeonManager::is_current_room_matched()) {
            return;
        }
        let Some(mut room) = DungeonManager::get_current_room() else {
            return;
        };
        for i in 0..3 {
            IceFill::extract_path(__this, collector, &room, &__this.lock().borrow().ice_fill_paths[(i) as usize], BOARD_ORIGINS.get_mut()[(i) as usize].clone());
        }
    }

    /// IceFill.extractRendering as the dispatch of Renderable.extractRendering
    pub fn extract_rendering_dyn(__this: &compat::Shared<dyn Renderable>, __p0: &compat::Shared<dyn PrimitiveCollector>) {
        Self::extract_rendering_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

}

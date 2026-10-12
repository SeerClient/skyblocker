//! Ported from de.hysky.skyblocker.skyblock.dungeon.puzzle.waterboard.WaterboardPreviewer by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::dungeon::puzzle::dungeon_puzzle::DungeonPuzzle;
use crate::de::hysky::skyblocker::skyblock::dungeon::puzzle::dungeon_puzzle::DungeonPuzzleBase;
use crate::de::hysky::skyblocker::skyblock::dungeon::puzzle::waterboard::waterboard;
use crate::de::hysky::skyblocker::skyblock::dungeon::puzzle::waterboard::waterboard::LeverType;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::dungeon_manager::DungeonManager;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::room::Room;
use crate::de::hysky::skyblocker::utils::color_utils::ColorUtils;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_collector::PrimitiveCollector;
use crate::de::hysky::skyblocker::utils::render::renderable::Renderable;
use crate::de::hysky::skyblocker::utils::resettable::Resettable;
use crate::de::hysky::skyblocker::utils::tickable::Tickable;
use mcrust::net::minecraft::client::multiplayer::client_level::ClientLevel;
use mcrust::net::minecraft::client::player::local_player::LocalPlayer;
use mcrust::net::minecraft::core::block_pos::BlockPos;
use mcrust::net::minecraft::core::block_pos::MutableBlockPos;
use mcrust::net::minecraft::core::direction::Direction;
use mcrust::net::minecraft::world::level::block::blocks;
use mcrust::net::minecraft::world::level::block::state::block_state::BlockState;
use mcrust::net::minecraft::world::level::block_getter;
use mcrust::net::minecraft::world::phys::vec3;
use mcrust::net::minecraft::world::phys::vec3::Vec3;

/// java: public static final WaterboardPreviewer INSTANCE = new WaterboardPreviewer();
pub static INSTANCE: compat::MainThread<compat::Shared<WaterboardPreviewer>> = compat::MainThread::new(|| WaterboardPreviewer::new());

/// java: public class WaterboardPreviewer extends DungeonPuzzle
pub struct WaterboardPreviewer {
    /// java: extends DungeonPuzzle
    pub base: DungeonPuzzleBase,
    /// java: private LeverType prospective;
    pub prospective: Option<LeverType>,
    /// java: private ClientLevel world;
    pub world: Option<ClientLevel>,
    /// java: private Room room;
    pub room: Option<compat::Shared<dyn Room>>,
    /// java: private LocalPlayer player;
    pub player: Option<LocalPlayer>,
}

impl std::ops::Deref for WaterboardPreviewer {
    type Target = DungeonPuzzleBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for WaterboardPreviewer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl WaterboardPreviewer {
    /// java: private WaterboardPreviewer()
    pub fn new() -> compat::Shared<Self> {
        // NOTE(fabric2seer): `player` is read by `reset`, which DungeonPuzzle's constructor calls before this constructor assigns it: Java sees its default there (0 / false / null), and so does this port
        // NOTE(fabric2seer): `prospective` is read by `reset`, which DungeonPuzzle's constructor calls before this constructor assigns it: Java sees its default there (0 / false / null), and so does this port
        // NOTE(fabric2seer): `room` is read by `reset`, which DungeonPuzzle's constructor calls before this constructor assigns it: Java sees its default there (0 / false / null), and so does this port
        // NOTE(fabric2seer): `world` is read by `reset`, which DungeonPuzzle's constructor calls before this constructor assigns it: Java sees its default there (0 / false / null), and so does this port
        let (_, __this) = DungeonPuzzleBase::new_in("waterboard".to_string(), vec!["water-puzzle".to_string()], move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                prospective: None,
                world: None,
                room: None,
                player: None,
            });
            ((__h.clone() as compat::Shared<dyn DungeonPuzzle>), __h)
        });
        __this
    }

    /// java: @Init public static void init()
    pub fn init() {
    }

    /// java: private void extractWaterPath(PrimitiveCollector collector)
    pub fn extract_water_path(__this: &compat::Shared<Self>, collector: &compat::Shared<dyn PrimitiveCollector>) {
        if !(SkyblockerConfigManager::get().dungeons.puzzle_solvers.preview_water_path) {
            return;
        }
        let mut water_path: Vec<(BlockPos, BlockPos)> = Vec::new();
        water_path.push((waterboard::WATER_ENTRANCE_POSITION.get_mut().above_n(5), waterboard::WATER_ENTRANCE_POSITION.get_mut().above_n(3)));
        WaterboardPreviewer::find_water_path_vertical(__this, &waterboard::WATER_ENTRANCE_POSITION.get_mut().above_n(3), &mut water_path);
        for pair in water_path.iter() {
            let mut head: Vec3 = vec3::at_center_of(&({ let __r = &compat::read(__this, |__t| __t.room.clone()).unwrap(); let __f = compat::dispatch(__r, |__t| __t.relative_to_actual()); __f(__r, &pair.0) }));
            let mut tail: Vec3 = vec3::at_center_of(&({ let __r = &compat::read(__this, |__t| __t.room.clone()).unwrap(); let __f = compat::dispatch(__r, |__t| __t.relative_to_actual()); __f(__r, &pair.1) }));
            let mut lines: Vec<Vec<Vec3>> = Vec::new();
            if compat::read(__this, |__t| __t.prospective.clone()).is_none() {
                lines.push(vec![head.clone(), tail.clone()]);
            } else {
                let mut forward: Vec3 = tail.subtract_vec(&head).normalize();
                let mut distance: f64 = head.distance_to(&tail);
                for i in 0..(distance) as i32 {
                    lines.push(vec![head.clone(), head.add_vec(&forward.scale(0.3))]);
                    lines.push(vec![head.add_vec(&forward.scale(0.7)), head.add_vec(&forward)]);
                    head = head.add_vec(&forward);
                }
            }
            for line in lines.iter() {
                ({ let __f = compat::dispatch(collector, |__t| __t.submit_lines_from_points()); __f(collector, line, &ColorUtils::get_float_components_dye_color(&LeverType::Water.fields().color), 1.0f32, 3.0f32, true) });
            }
        }
    }

    /// java: private void findWaterPathVertical(BlockPos root, List<Pair<BlockPos, BlockPos>> waterPath)
    pub fn find_water_path_vertical(__this: &compat::Shared<Self>, root: &BlockPos, water_path: &mut Vec<(BlockPos, BlockPos)>) {
        if WaterboardPreviewer::is_water_passable(__this, &root.below()) {
            let tail = MutableBlockPos::new_origin().set_vec3i(root.below().as_vec3i());
            while WaterboardPreviewer::is_water_passable(__this, &tail.immutable().below()) {
                tail.move_direction(Direction::Down);
            }
            water_path.push((root.clone(), BlockPos::from_vec3i(tail)));
            WaterboardPreviewer::find_water_path_horizontal(__this, tail.immutable(), water_path);
        }
    }

    /// java: private void findWaterPathHorizontal(BlockPos root, List<Pair<BlockPos, BlockPos>> waterPath)
    pub fn find_water_path_horizontal(__this: &compat::Shared<Self>, mut root: BlockPos, water_path: &mut Vec<(BlockPos, BlockPos)>) {
        if !(WaterboardPreviewer::is_water_passable(__this, &root.below())) {
            let left = MutableBlockPos::new_origin().set_vec3i(root.as_vec3i());
            let mut left_steps: i32 = 0;
            while WaterboardPreviewer::is_water_passable(__this, &left.immutable().east()) && !(WaterboardPreviewer::is_water_passable(__this, &left.immutable().below())) && left_steps < 7 {
                left.move_direction(Direction::East);
                left_steps += 1;
            }
            let right = MutableBlockPos::new_origin().set_vec3i(root.as_vec3i());
            let mut right_steps: i32 = 0;
            while WaterboardPreviewer::is_water_passable(__this, &right.immutable().west()) && !(WaterboardPreviewer::is_water_passable(__this, &right.immutable().below())) && right_steps < 7 {
                right.move_direction(Direction::West);
                right_steps += 1;
            }
            if WaterboardPreviewer::is_water_passable(__this, &left.immutable().below()) && left_steps <= 5 && left_steps < right_steps || !(WaterboardPreviewer::is_water_passable(__this, &right.immutable().below())) {
                water_path.push((root.clone(), BlockPos::from_vec3i(left)));
                WaterboardPreviewer::find_water_path_vertical(__this, &left.immutable(), water_path);
            } else if WaterboardPreviewer::is_water_passable(__this, &right.immutable().below()) && right_steps <= 5 && right_steps < left_steps || !(WaterboardPreviewer::is_water_passable(__this, &left.immutable().below())) {
                water_path.push((root.clone(), BlockPos::from_vec3i(right)));
                WaterboardPreviewer::find_water_path_vertical(__this, &right.immutable(), water_path);
            } else {
                if left_steps > 0 {
                    water_path.push((root.clone(), BlockPos::from_vec3i(left)));
                    WaterboardPreviewer::find_water_path_vertical(__this, &left.immutable(), water_path);
                }
                if right_steps > 0 {
                    water_path.push((root.clone(), BlockPos::from_vec3i(right)));
                    WaterboardPreviewer::find_water_path_vertical(__this, &right.immutable(), water_path);
                }
            }
        }
    }

    /// java: private boolean isWaterPassable(BlockPos pos)
    pub fn is_water_passable(__this: &compat::Shared<Self>, pos: &BlockPos) -> bool {
        if pos.get_x() < waterboard::BOARD_MIN_X || pos.get_x() > waterboard::BOARD_MAX_X || pos.get_y() < waterboard::BOARD_MIN_Y || pos.get_y() > waterboard::BOARD_MAX_Y || pos.get_z() != waterboard::BOARD_Z {
            return false;
        }
        let mut state: Arc<BlockState> = { let (__q0,) = (&({ let __r = &compat::read(__this, |__t| __t.room.clone()).unwrap(); let __f = compat::dispatch(__r, |__t| __t.relative_to_actual()); __f(__r, pos) }),); __this.lock().borrow().world.as_ref().unwrap().get_block_state_typed(__q0) };
        let mut behind_state: Arc<BlockState> = { let (__q0,) = (&({ let __r = &compat::read(__this, |__t| __t.room.clone()).unwrap(); let __f = compat::dispatch(__r, |__t| __t.relative_to_actual()); __f(__r, &pos.relative(Direction::South)) }),); __this.lock().borrow().world.as_ref().unwrap().get_block_state_typed(__q0) };
        let mut open: bool = (state.is_air() || state.is_block(&blocks::blocks().get("water")));
        if compat::read(__this, |__t| __t.prospective.clone()).is_none() {
            return open;
        } else {
            return (open && !(behind_state.is_block(&__this.lock().borrow().prospective.as_ref().unwrap().fields().block)) || state.is_block(&__this.lock().borrow().prospective.as_ref().unwrap().fields().block));
        }
    }

    /// java: private void findProspective()
    pub fn find_prospective(__this: &compat::Shared<Self>) {
        if !(SkyblockerConfigManager::get().dungeons.puzzle_solvers.preview_lever_effects) {
            return;
        }
        let mut camera: Vec3 = ({ let __r = &compat::read(__this, |__t| __t.room.clone()).unwrap(); let __f = compat::dispatch(__r, |__t| __t.actual_to_relative_vec3()); __f(__r, &__this.lock().borrow().player.as_ref().unwrap().base.base.base.base.get_eye_position()) });
        let mut look: Vec3 = ({ let __r = &compat::read(__this, |__t| __t.room.clone()).unwrap(); let __f = compat::dispatch(__r, |__t| __t.actual_to_relative_vec3()); __f(__r, &{ let (__q0,) = (&__this.lock().borrow().player.as_ref().unwrap().base.base.base.base.get_look_angle(),); __this.lock().borrow().player.as_ref().unwrap().base.base.base.base.get_eye_position().add_vec(__q0) }) }).subtract_vec(&camera);
        let mut t1: f64 = ((((waterboard::BOARD_Z as f64) - 1.5) - camera.z()) / look.z());
        let mut t2: f64 = ((((waterboard::BOARD_Z as f64) + 1.5) - camera.z()) / look.z());
        let mut start: Vec3 = camera.add_vec(&look.scale(t1));
        let mut end: Vec3 = camera.add_vec(&look.scale(t2));
        let mut behind: Direction = match ({ let __r = &compat::read(__this, |__t| __t.room.clone()).unwrap(); let __f = compat::dispatch(__r, |__t| __t.get_direction()); __f(__r) }) {
            Some(crate::de::hysky::skyblocker::skyblock::dungeon::secrets::room::Direction::Nw) => Direction::South,
            Some(crate::de::hysky::skyblocker::skyblock::dungeon::secrets::room::Direction::Ne) => Direction::West,
            Some(crate::de::hysky::skyblocker::skyblock::dungeon::secrets::room::Direction::Sw) => Direction::East,
            Some(crate::de::hysky::skyblocker::skyblock::dungeon::secrets::room::Direction::Se) => Direction::North,
            _ => unreachable!("switch expression: no case matched"),
        };
        let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        { let __v = Some(block_getter::traverse_blocks(&({ let __r = &compat::read(__this, |__t| __t.room.clone()).unwrap(); let __f = compat::dispatch(__r, |__t| __t.relative_to_actual_vec3()); __f(__r, &start) }), &({ let __r = &compat::read(__this, |__t| __t.room.clone()).unwrap(); let __f = compat::dispatch(__r, |__t| __t.relative_to_actual_vec3()); __f(__r, &end) }), None /* java: null */, |_, pos| { let Some(__this_1) = __this_w1.get().upgrade() else { return Default::default(); }; {
            if ({ let __r = &compat::read((&__this_1), |__t| __t.room.clone()).unwrap(); let __f = compat::dispatch(__r, |__t| __t.actual_to_relative()); __f(__r, &pos) }).get_z() != waterboard::BOARD_Z {
                return Default::default(); // java: return null
            }
            let mut lever_type: Option<LeverType> = LeverType::from_block(&(&__this_1).lock().borrow().world.as_ref().unwrap().get_block_state_typed(&pos).get_block());
            if lever_type.is_none() {
                let mut alternate_pos: BlockPos = pos.relative(behind);
                lever_type = LeverType::from_block(&(&__this_1).lock().borrow().world.as_ref().unwrap().get_block_state_typed(&alternate_pos).get_block());
            }
            return lever_type;
        } }, |_| None)); compat::write(__this, |__t| __t.prospective = __v); }
    }

    /// java: private void extractProspectiveChanges(PrimitiveCollector collector)
    pub fn extract_prospective_changes(__this: &compat::Shared<Self>, collector: &compat::Shared<dyn PrimitiveCollector>) {
        if !(SkyblockerConfigManager::get().dungeons.puzzle_solvers.preview_lever_effects) || compat::read(__this, |__t| __t.prospective.clone()).is_none() {
            return;
        }
        for x in waterboard::BOARD_MIN_X..=waterboard::BOARD_MAX_X {
            for y in waterboard::BOARD_MIN_Y..=waterboard::BOARD_MAX_Y {
                let mut active_pos: BlockPos = ({ let __r = &compat::read(__this, |__t| __t.room.clone()).unwrap(); let __f = compat::dispatch(__r, |__t| __t.relative_to_actual()); __f(__r, &BlockPos::new(x, y, waterboard::BOARD_Z)) });
                let mut inactive_pos: BlockPos = ({ let __r = &compat::read(__this, |__t| __t.room.clone()).unwrap(); let __f = compat::dispatch(__r, |__t| __t.relative_to_actual()); __f(__r, &BlockPos::new(x, y, (waterboard::BOARD_Z).wrapping_add(1))) });
                if { let (__q0,) = (&__this.lock().borrow().prospective.as_ref().unwrap().fields().block,); __this.lock().borrow().world.as_ref().unwrap().get_block_state_typed(&active_pos).is_block(__q0) } {
                    ({ let __f = compat::dispatch(collector, |__t| __t.submit_outlined_box()); __f(collector, &active_pos, &ColorUtils::get_float_components_dye_color(&__this.lock().borrow().prospective.as_ref().unwrap().fields().color), 2.0f32, true) });
                } else if { let (__q0,) = (&__this.lock().borrow().prospective.as_ref().unwrap().fields().block,); __this.lock().borrow().world.as_ref().unwrap().get_block_state_typed(&inactive_pos).is_block(__q0) } {
                    ({ let __f = compat::dispatch(collector, |__t| __t.submit_filled_box()); __f(collector, &active_pos, &ColorUtils::get_float_components_dye_color(&__this.lock().borrow().prospective.as_ref().unwrap().fields().color), 0.8f32, true) });
                }
            }
        }
    }

}

impl DungeonPuzzle for WaterboardPreviewer {
    fn dungeon_puzzle_base(&self) -> &DungeonPuzzleBase {
        &self.base
    }
    fn dungeon_puzzle_base_mut(&mut self) -> &mut DungeonPuzzleBase {
        &mut self.base
    }

}

impl Tickable for WaterboardPreviewer {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void tick(Minecraft client) (dispatch: the body for this class)
    fn tick(&self) -> fn(&compat::Shared<dyn Tickable>, &mut mcrust::net::minecraft::client::minecraft::Minecraft) {
        WaterboardPreviewer::tick_dyn
    }

}

impl Renderable for WaterboardPreviewer {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void extractRendering(PrimitiveCollector collector) (dispatch: the body for this class)
    fn extract_rendering(&self) -> fn(&compat::Shared<dyn Renderable>, &compat::Shared<dyn PrimitiveCollector>) {
        WaterboardPreviewer::extract_rendering_dyn
    }

}

impl Resettable for WaterboardPreviewer {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void reset() (dispatch: the body for this class)
    fn reset(&self) -> fn(&compat::Shared<dyn Resettable>) {
        WaterboardPreviewer::reset_dyn
    }

}

impl WaterboardPreviewer {
    /// java: @Override public void tick(Minecraft client)
    pub fn tick_impl(__this: &compat::Shared<Self>, client: &mut mcrust::net::minecraft::client::minecraft::Minecraft) {
    }

    /// WaterboardPreviewer.tick as the dispatch of Tickable.tick
    pub fn tick_dyn(__this: &compat::Shared<dyn Tickable>, __p0: &mut mcrust::net::minecraft::client::minecraft::Minecraft) {
        Self::tick_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public void extractRendering(PrimitiveCollector collector)
    pub fn extract_rendering_impl(__this: &compat::Shared<Self>, collector: &compat::Shared<dyn PrimitiveCollector>) {
        if !({ let __f = compat::dispatch(__this, |__t| __t.should_solve()); __f(&compat::view(__this, |a| a as compat::Shared<dyn DungeonPuzzle>)) }) || mcrust::mcrust::modding::game::level().is_none() || mcrust::mcrust::modding::game::with_player(|_| ()).is_none() || !(DungeonManager::is_current_room_matched()) {
            return;
        }
        { let __v = mcrust::mcrust::modding::game::level(); compat::write(__this, |__t| __t.world = __v); }
        { let __v = DungeonManager::get_current_room(); compat::write(__this, |__t| __t.room = __v); }
        // TODO(fabric2seer): `Minecraft.player` held as a value: the port lends the player only inside `game::with_player(|player| ..)` (a mod must never lock it)
        //   java: player = Minecraft.getInstance().player;
        // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            WaterboardPreviewer::find_prospective(__this);
            WaterboardPreviewer::extract_water_path(__this, collector);
            WaterboardPreviewer::extract_prospective_changes(__this, collector);
        }));
        if let Err(e) = __result {
            logger().error(&format!("[Skyblocker Waterboard] Error while rendering previews"));
        }
    }

    /// WaterboardPreviewer.extractRendering as the dispatch of Renderable.extractRendering
    pub fn extract_rendering_dyn(__this: &compat::Shared<dyn Renderable>, __p0: &compat::Shared<dyn PrimitiveCollector>) {
        Self::extract_rendering_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public void reset()
    pub fn reset_impl(__this: &compat::Shared<Self>) {
        <dyn DungeonPuzzle>::reset_body(&compat::view(__this, |a| a as compat::Shared<dyn DungeonPuzzle>));
        { let __v = None; compat::write(__this, |__t| __t.world = __v); }
        { let __v = None; compat::write(__this, |__t| __t.room = __v); }
        { let __v = None; compat::write(__this, |__t| __t.player = __v); }
        { let __v = None; compat::write(__this, |__t| __t.prospective = __v); }
    }

    /// WaterboardPreviewer.reset as the dispatch of Resettable.reset
    pub fn reset_dyn(__this: &compat::Shared<dyn Resettable>) {
        Self::reset_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}

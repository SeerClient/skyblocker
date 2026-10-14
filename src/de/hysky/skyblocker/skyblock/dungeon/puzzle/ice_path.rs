//! Ported from de.hysky.skyblocker.skyblock.dungeon.puzzle.IcePath by fabric2seer.

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
use mcrust::net::minecraft::core::block_pos::BlockPos;
use mcrust::net::minecraft::core::block_pos::MutableBlockPos;
use mcrust::net::minecraft::core::direction::Direction;
use mcrust::net::minecraft::world::entity::monster::silverfish::Silverfish;
use mcrust::net::minecraft::world::item::dye_color::DyeColor;
use mcrust::net::minecraft::world::phys::aabb::AABB;
use mcrust::net::minecraft::world::phys::vec3;
use mcrust::net::minecraft::world::phys::vec3::Vec3;
use mcrust::org::joml::vector2i::Vector2i;
use mcrust::net::minecraft::world::level::entity_getter::EntityGetter as _;

/// java: public static final IcePath INSTANCE = new IcePath();
pub static INSTANCE: compat::MainThread<compat::Shared<IcePath>> = compat::MainThread::new(|| IcePath::new());
/// java: private static final float[] RED_COLOR_COMPONENTS = ColorUtils.getFloatComponents(DyeColor.RED);
pub static RED_COLOR_COMPONENTS: LazyLock<Vec<f32>> = LazyLock::new(|| ColorUtils::get_float_components_dye_color(&DyeColor::Red));

/// java: public class IcePath extends DungeonPuzzle
#[derive(Clone)]
pub struct IcePath {
    /// java: extends DungeonPuzzle
    pub base: DungeonPuzzleBase,
    /// java: final boolean[][] silverfishBoard = new boolean[17][17];
    pub silverfish_board: Vec<Vec<bool>>,
    /// java: @Nullable Vector2ic silverfishPos;
    pub silverfish_pos: Option<Vector2i>,
    /// java: final List<Vector2ic> silverfishPath = new ArrayList<>();
    pub silverfish_path: Vec<Vector2i>,
}

impl std::ops::Deref for IcePath {
    type Target = DungeonPuzzleBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for IcePath {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl IcePath {
    /// java: private IcePath()
    pub fn new() -> compat::Shared<Self> {
        // NOTE(fabric2seer): `silverfishBoard` is read by `reset`, which DungeonPuzzle's constructor calls before this constructor assigns it: Java sees its default there (0 / false / null), and so does this port
        // NOTE(fabric2seer): `silverfishPos` is read by `reset`, which DungeonPuzzle's constructor calls before this constructor assigns it: Java sees its default there (0 / false / null), and so does this port
        let (_, __this) = DungeonPuzzleBase::new_in("silverfish".to_string(), vec!["ice-silverfish-room".to_string()], move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                silverfish_board: vec![vec![false; (17) as usize]; (17) as usize],
                silverfish_pos: None,
                silverfish_path: Vec::new(),
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

    /// java: void solve()
    pub fn solve(__this: &compat::Shared<Self>) {
        if compat::read(__this, |__t| __t.silverfish_pos.clone()).is_none() {
            return;
        }
        let mut visited: std::collections::HashSet<Vector2i> = std::collections::HashSet::new();
        let mut queue: std::collections::VecDeque<Vec<Vector2i>> = std::collections::VecDeque::new();
        queue.push_back(vec![compat::read(__this, |__t| __t.silverfish_pos.clone()).unwrap()]);
        visited.insert(compat::read(__this, |__t| __t.silverfish_pos.clone()).unwrap());
        while !queue.is_empty() {
            let mut path: Vec<Vector2i> = queue.pop_front().unwrap();
            let pos = &path[((path.len() as i32).wrapping_sub(1)) as usize];
            if pos.x() == 0 && pos.y() >= 7 && pos.y() <= 9 {
                __this.lock().borrow_mut().silverfish_path.clear();
                let __h0 = path.iter().cloned();
                __this.lock().borrow_mut().silverfish_path.extend(__h0);
                return;
            }
            let mut pos_mutable: Vector2i = Vector2i::new_from(pos);
            while pos_mutable.x() < 17 && !((__this.lock().borrow().silverfish_board[(pos_mutable.x()) as usize])[(pos_mutable.y()) as usize]) {
                pos_mutable.add(1, 0);
            }
            pos_mutable.add(-1, 0);
            IcePath::add_queue(__this, &mut visited, &mut queue, &path, pos_mutable.clone());
            pos_mutable = Vector2i::new_from(pos);
            while pos_mutable.x() >= 0 && !((__this.lock().borrow().silverfish_board[(pos_mutable.x()) as usize])[(pos_mutable.y()) as usize]) {
                pos_mutable.add(-1, 0);
            }
            pos_mutable.add(1, 0);
            IcePath::add_queue(__this, &mut visited, &mut queue, &path, pos_mutable.clone());
            pos_mutable = Vector2i::new_from(pos);
            while pos_mutable.y() < 17 && !((__this.lock().borrow().silverfish_board[(pos_mutable.x()) as usize])[(pos_mutable.y()) as usize]) {
                pos_mutable.add(0, 1);
            }
            pos_mutable.add(0, -1);
            IcePath::add_queue(__this, &mut visited, &mut queue, &path, pos_mutable.clone());
            pos_mutable = Vector2i::new_from(pos);
            while pos_mutable.y() >= 0 && !((__this.lock().borrow().silverfish_board[(pos_mutable.x()) as usize])[(pos_mutable.y()) as usize]) {
                pos_mutable.add(0, -1);
            }
            pos_mutable.add(0, 1);
            IcePath::add_queue(__this, &mut visited, &mut queue, &path, pos_mutable.clone());
        }
    }

    /// java: private void addQueue(Set<Vector2ic> visited, Queue<List<Vector2ic>> queue, List<Vector2ic> path, Vector2ic newPos)
    pub fn add_queue(__this: &compat::Shared<Self>, visited: &mut std::collections::HashSet<Vector2i>, queue: &mut std::collections::VecDeque<Vec<Vector2i>>, path: &Vec<Vector2i>, mut new_pos: Vector2i) {
        if !(visited.contains(&new_pos)) {
            let mut new_path: Vec<Vector2i> = path.iter().cloned().collect::<Vec<_>>();
            new_path.push(new_pos.clone());
            queue.push_back(new_path.clone());
            visited.insert(new_pos.clone());
        }
    }

}

impl DungeonPuzzle for IcePath {
    fn dungeon_puzzle_base(&self) -> &DungeonPuzzleBase {
        &self.base
    }
    fn dungeon_puzzle_base_mut(&mut self) -> &mut DungeonPuzzleBase {
        &mut self.base
    }

}

impl Tickable for IcePath {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void tick(Minecraft client) (dispatch: the body for this class)
    fn tick(&self) -> fn(&compat::Shared<dyn Tickable>, &mut mcrust::net::minecraft::client::minecraft::Minecraft) {
        IcePath::tick_dyn
    }

}

impl Renderable for IcePath {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void extractRendering(PrimitiveCollector collector) (dispatch: the body for this class)
    fn extract_rendering(&self) -> fn(&compat::Shared<dyn Renderable>, &compat::Shared<dyn PrimitiveCollector>) {
        IcePath::extract_rendering_dyn
    }

}

impl Resettable for IcePath {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void reset() (dispatch: the body for this class)
    fn reset(&self) -> fn(&compat::Shared<dyn Resettable>) {
        IcePath::reset_dyn
    }

}

impl IcePath {
    /// java: @Override public void tick(Minecraft client)
    pub fn tick_impl(__this: &compat::Shared<Self>, client: &mut mcrust::net::minecraft::client::minecraft::Minecraft) {
        if !(SkyblockerConfigManager::get().dungeons.puzzle_solvers.solve_silverfish) || mcrust::mcrust::modding::game::level().is_none() || !(DungeonManager::is_current_room_matched()) {
            return;
        }
        let Some(mut room) = DungeonManager::get_current_room() else {
            return;
        };
        let mut board_changed: bool = false;
        let mut pos: MutableBlockPos = MutableBlockPos::new(23, 67, 24);
        {
            let mut row: i32 = 0;
            while row < (__this.lock().borrow().silverfish_board.len() as i32) {
                {
                    let mut col: i32 = 0;
                    while col < ((__this.lock().borrow().silverfish_board[(row) as usize]).len() as i32) {
                        let mut is_block: bool = !((mcrust::mcrust::modding::game::level().unwrap().get_block_state_typed(&({ let __f = compat::dispatch(&room, |__t| __t.relative_to_actual()); __f(&room, &pos.immutable()) }))).is_air());
                        if (__this.lock().borrow().silverfish_board[(row) as usize])[(col) as usize] != is_block {
                            compat::write(__this, |__t| (__t.silverfish_board[(row) as usize])[(col) as usize] = is_block);
                            board_changed = true;
                        }
                        pos.move_direction(Direction::West);
                        col += 1;
                    }
                }
                pos.move_by(((__this.lock().borrow().silverfish_board[(row) as usize]).len() as i32), 0, -1);
                row += 1;
            }
        }
        // TODO(fabric2seer): the port's `get_entities_of_class` ports this Java method with other parameters (base_class, except, bb, selector for Java's base_class, bb, selector)
        // TODO(fabric2seer): `get_entities_of_class` takes 4 argument(s) in the port (`base_class: crate::net::minecraft::world::level::entity::entity_type_test::EntityClass, except: Option<i32>, bb: &AABB, selector: &dyn Fn(&(dyn Entity + Send + Sync)) -> bool`), Java passes 3
        // TODO(fabric2seer): a lambda for a parameter the port does not type as a closure (`Predicate<? super T>`)
        // (the port's class filter has no Silverfish.class: the base class Entity plus `instanceof Silverfish`)
        use mcrust::net::minecraft::world::level::entity_getter::EntityGetter as _;
        let search_box = AABB::of_size(&vec3::at_center_of(&({ let __f = compat::dispatch(&room, |__t| __t.relative_to_actual()); __f(&room, &BlockPos::new(15, 66, 16)) })), (16) as f64, (16) as f64, (16) as f64);
        let entities: Vec<mcrust::net::minecraft::world::level::entity_getter::SharedEntity> = mcrust::mcrust::modding::game::level().unwrap().get_entities_of_class(
            mcrust::net::minecraft::world::level::entity::entity_type_test::EntityClass::Entity,
            None,
            &search_box,
            &|entity| entity.as_any().is_some_and(|any| any.is::<Silverfish>()),
        );
        if entities.is_empty() {
            return;
        }
        let entity_block_pos: BlockPos = mcrust::mcrust::modding::game::lock_entity(&*entities[0], "skyblocker IcePath.tick").base().block_position();
        let mut new_silverfish_block_pos: BlockPos = ({ let __f = compat::dispatch(&room, |__t| __t.actual_to_relative()); __f(&room, &entity_block_pos) });
        let mut new_silverfish_pos: Vector2i = Vector2i::new(24i32.wrapping_sub(new_silverfish_block_pos.get_z()), 23i32.wrapping_sub(new_silverfish_block_pos.get_x()));
        if new_silverfish_pos.x() < 0 || new_silverfish_pos.x() >= 17 || new_silverfish_pos.y() < 0 || new_silverfish_pos.y() >= 17 {
            return;
        }
        let mut silverfish_changed: bool = !(Some(&new_silverfish_pos) == compat::read(__this, |__t| __t.silverfish_pos.clone()).as_ref());
        if silverfish_changed {
            { let __v = Some(new_silverfish_pos.clone()); compat::write(__this, |__t| __t.silverfish_pos = __v); }
        }
        if silverfish_changed || board_changed {
            IcePath::solve(__this);
        }
    }

    /// IcePath.tick as the dispatch of Tickable.tick
    pub fn tick_dyn(__this: &compat::Shared<dyn Tickable>, __p0: &mut mcrust::net::minecraft::client::minecraft::Minecraft) {
        Self::tick_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public void extractRendering(PrimitiveCollector collector)
    pub fn extract_rendering_impl(__this: &compat::Shared<Self>, collector: &compat::Shared<dyn PrimitiveCollector>) {
        if !(SkyblockerConfigManager::get().dungeons.puzzle_solvers.solve_silverfish) || !(DungeonManager::is_current_room_matched()) || __this.lock().borrow().silverfish_path.is_empty() {
            return;
        }
        let Some(mut room) = DungeonManager::get_current_room() else {
            return;
        };
        let mut pos: MutableBlockPos = MutableBlockPos::new_origin();
        for i in { 0..(__this.lock().borrow().silverfish_path.len() as i32).wrapping_sub(1) } {
            let mut start: Vec3 = vec3::at_center_of(&({ let __f = compat::dispatch(&room, |__t| __t.relative_to_actual()); __f(&room, &(pos.set(23i32.wrapping_sub((__this.lock().borrow().silverfish_path[(i) as usize]).y()), 67, 24i32.wrapping_sub((__this.lock().borrow().silverfish_path[(i) as usize]).x()))).immutable()) }));
            let mut end: Vec3 = vec3::at_center_of(&({ let __f = compat::dispatch(&room, |__t| __t.relative_to_actual()); __f(&room, &(pos.set(23i32.wrapping_sub((__this.lock().borrow().silverfish_path[(i.wrapping_add(1)) as usize]).y()), 67, 24i32.wrapping_sub((__this.lock().borrow().silverfish_path[(i.wrapping_add(1)) as usize]).x()))).immutable()) }));
            ({ let __f = compat::dispatch(collector, |__t| __t.submit_lines_from_points()); __f(collector, &vec![start.clone(), end.clone()], &RED_COLOR_COMPONENTS, 1.0f32, 5.0f32, true) });
        }
    }

    /// IcePath.extractRendering as the dispatch of Renderable.extractRendering
    pub fn extract_rendering_dyn(__this: &compat::Shared<dyn Renderable>, __p0: &compat::Shared<dyn PrimitiveCollector>) {
        Self::extract_rendering_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public void reset()
    pub fn reset_impl(__this: &compat::Shared<Self>) {
        <dyn DungeonPuzzle>::reset_body(&compat::view(__this, |a| a as compat::Shared<dyn DungeonPuzzle>));
        compat::write(__this, |__t| {
            for silverfish_board_row in __t.silverfish_board.iter_mut() {
                silverfish_board_row.fill(false);
            }
        });
        { let __v = None; compat::write(__this, |__t| __t.silverfish_pos = __v); }
    }

    /// IcePath.reset as the dispatch of Resettable.reset
    pub fn reset_dyn(__this: &compat::Shared<dyn Resettable>) {
        Self::reset_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}

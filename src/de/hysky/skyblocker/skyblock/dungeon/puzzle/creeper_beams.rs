//! Ported from de.hysky.skyblocker.skyblock.dungeon.puzzle.CreeperBeams by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::dungeon::puzzle::dungeon_puzzle::DungeonPuzzle;
use crate::de::hysky::skyblocker::skyblock::dungeon::puzzle::dungeon_puzzle::DungeonPuzzleBase;
use crate::de::hysky::skyblocker::utils::color_utils::ColorUtils;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_collector::PrimitiveCollector;
use crate::de::hysky::skyblocker::utils::render::renderable::Renderable;
use crate::de::hysky::skyblocker::utils::resettable::Resettable;
use crate::de::hysky::skyblocker::utils::tickable::Tickable;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::net::minecraft::client::multiplayer::client_level::ClientLevel;
use mcrust::net::minecraft::client::player::local_player::LocalPlayer;
use mcrust::net::minecraft::core::block_pos::BlockPos;
use mcrust::net::minecraft::world::entity::monster::creeper::Creeper;
use mcrust::net::minecraft::world::item::dye_color::DyeColor;
use mcrust::net::minecraft::world::level::block::block::Block;
use mcrust::net::minecraft::world::level::block::blocks;
use mcrust::net::minecraft::world::phys::aabb::AABB;
use mcrust::net::minecraft::world::phys::vec3::Vec3;
use mcrust::net::minecraft::world::level::entity_getter::EntityGetter as _;

/// java: private static final float[][] COLORS = { ColorUtils.getFloatComponents(DyeColor.LIGHT_BLUE), ColorUtils.getFloatComponents(DyeColor.LIME), ColorUtils.getFloatComponents(DyeColor.YELLOW), ColorUtils.g
pub static COLORS: LazyLock<Vec<Vec<f32>>> = LazyLock::new(|| vec![ColorUtils::get_float_components_dye_color(&DyeColor::LightBlue), ColorUtils::get_float_components_dye_color(&DyeColor::Lime), ColorUtils::get_float_components_dye_color(&DyeColor::Yellow), ColorUtils::get_float_components_dye_color(&DyeColor::Magenta), ColorUtils::get_float_components_dye_color(&DyeColor::Pink)]);
/// java: private static final float[] GREEN_COLOR_COMPONENTS = ColorUtils.getFloatComponents(DyeColor.GREEN);
pub static GREEN_COLOR_COMPONENTS: LazyLock<Vec<f32>> = LazyLock::new(|| ColorUtils::get_float_components_dye_color(&DyeColor::Green));
/// java: private static final int FLOOR_Y = 68;
pub const FLOOR_Y: i32 = 68;
/// java: private static final int BASE_Y = 74;
pub const BASE_Y: i32 = 74;
/// java: @SuppressWarnings("unused") private static final CreeperBeams INSTANCE = new CreeperBeams();
pub static INSTANCE: compat::MainThread<compat::Shared<CreeperBeams>> = compat::MainThread::new(|| CreeperBeams::new());
/// java: private static ArrayList<Beam> beams = new ArrayList<>();
pub static BEAMS: compat::MainThread<Vec<Beam>> = compat::MainThread::new(|| Vec::new());
/// java: private static @Nullable BlockPos base = null;
pub static BASE: compat::MainThread<Option<BlockPos>> = compat::MainThread::new(|| None);

/// java: public class CreeperBeams extends DungeonPuzzle
#[derive(Clone)]
pub struct CreeperBeams {
    /// java: extends DungeonPuzzle
    pub base: DungeonPuzzleBase,
}

impl std::ops::Deref for CreeperBeams {
    type Target = DungeonPuzzleBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for CreeperBeams {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl CreeperBeams {
    /// java: private CreeperBeams()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = DungeonPuzzleBase::new_in("creeper".to_string(), vec!["creeper-room".to_string()], move |__base| {
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

    /// java: private static @Nullable BlockPos findCreeperBase(LocalPlayer player, ClientLevel world)
    pub fn find_creeper_base(player: &LocalPlayer, world: &ClientLevel) -> Option<BlockPos> {
        // TODO(fabric2seer): static `EntitySelector.ENTITY_STILL_ALIVE` has no Rust counterpart in the port's EntitySelector file
        // TODO(fabric2seer): the port's `get_entities_of_class` ports this Java method with other parameters (base_class, except, bb, selector for Java's base_class, bb, selector)
        // TODO(fabric2seer): `get_entities_of_class` takes 4 argument(s) in the port (`base_class: crate::net::minecraft::world::level::entity::entity_type_test::EntityClass, except: Option<i32>, bb: &AABB, selector: &dyn Fn(&(dyn Entity + Send + Sync)) -> bool`), Java passes 3
        // rust: let mut creepers: Vec<Creeper> = world.get_entities_of_class(std::any::TypeId::of::<Creeper>(), Some(player.base.base.base.base.get_bounding_box().inflate(50.0)), &crate::unresolved::EntitySelector::ENTITY_STILL_ALIVE);
        let mut creepers: Vec<Creeper> = todo!("fabric2seer: the initialiser of `creepers` (TODO above)");
        if creepers.is_empty() {
            return None;
        }
        for ce in creepers.iter() {
            let mut creeper_pos: Vec3 = ce.base.base.base.base.base.position();
            let mut potential_base: BlockPos = BlockPos::containing(creeper_pos.x, (BASE_Y) as f64, creeper_pos.z);
            if CreeperBeams::is_target(world, &potential_base) {
                return Some(potential_base.clone());
            }
        }
        return None;
    }

    /// java: private static ArrayList<BlockPos> findTargets(ClientLevel world, BlockPos basePos)
    pub fn find_targets(world: &ClientLevel, base_pos: &BlockPos) -> Vec<BlockPos> {
        let mut targets: Vec<BlockPos> = Vec::new();
        let mut start: BlockPos = BlockPos::new(base_pos.get_x().wrapping_sub(15), BASE_Y.wrapping_add(12), base_pos.get_z().wrapping_sub(15));
        let mut end: BlockPos = BlockPos::new(base_pos.get_x().wrapping_add(16), FLOOR_Y, base_pos.get_z().wrapping_add(16));
        for pos in BlockPos::between_closed_pos(&start, &end) {
            if CreeperBeams::is_target(world, &pos) {
                targets.push(BlockPos::from_vec3i(pos.as_vec3i()));
            }
        }
        return targets;
    }

    /// java: private static ArrayList<Beam> findLines(Vec3 creeperPos, ArrayList<BlockPos> targets)
    pub fn find_lines(creeper_pos: &Vec3, targets: &Vec<BlockPos>) -> Vec<Beam> {
        let mut all_lines: Vec<(Beam, f64)> = Vec::new();
        for i in 0..(targets.len() as i32) {
            for j in i.wrapping_add(1)..(targets.len() as i32) {
                let mut beam: Beam = Beam::new(targets[(i) as usize].clone(), targets[(j) as usize].clone());
                // java: double dist = Intersectiond.distancePointLine(creeperPos.x, creeperPos.y, creeperPos.z,
                //   beam.line[0].x, beam.line[0].y, beam.line[0].z, beam.line[1].x, beam.line[1].y, beam.line[1].z);
                let mut dist: f64 = distance_point_line(creeper_pos.x, creeper_pos.y, creeper_pos.z, beam.line[0].x, beam.line[0].y, beam.line[0].z, beam.line[1].x, beam.line[1].y, beam.line[1].z);
                all_lines.push((beam.clone(), dist));
            }
        }
        let mut result: Vec<Beam> = Vec::new();
        all_lines.sort_by(|a, b| { let key = |__a0: &(Beam, f64)| __a0.1; key(a).total_cmp(&key(b)) });
        while (result.len() as i32) < 5 && !all_lines.is_empty() {
            let solution = &all_lines[0].0;
            result.push(solution.clone());
            all_lines.remove(0);
            all_lines.retain(|beam| !(solution.contains_component_of(&beam.0)));
        }
        if (result.len() as i32) < 5 {
            logger().error("Not enough solutions found. This is bad...");
        }
        return result;
    }

    /// java: private static boolean isTarget(ClientLevel world, BlockPos pos)
    pub fn is_target(world: &ClientLevel, pos: &BlockPos) -> bool {
        let mut block: Arc<Block> = world.get_block_state_typed(pos).get_block();
        return (std::ptr::addr_eq(std::sync::Arc::as_ptr(&block), std::sync::Arc::as_ptr(&blocks::blocks().get("sea_lantern"))) || std::ptr::addr_eq(std::sync::Arc::as_ptr(&block), std::sync::Arc::as_ptr(&blocks::blocks().get("prismarine"))));
    }

}

impl DungeonPuzzle for CreeperBeams {
    fn dungeon_puzzle_base(&self) -> &DungeonPuzzleBase {
        &self.base
    }
    fn dungeon_puzzle_base_mut(&mut self) -> &mut DungeonPuzzleBase {
        &mut self.base
    }

}

impl Tickable for CreeperBeams {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void tick(Minecraft client) (dispatch: the body for this class)
    fn tick(&self) -> fn(&compat::Shared<dyn Tickable>, &mut mcrust::net::minecraft::client::minecraft::Minecraft) {
        CreeperBeams::tick_dyn
    }

}

impl Renderable for CreeperBeams {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void extractRendering(PrimitiveCollector collector) (dispatch: the body for this class)
    fn extract_rendering(&self) -> fn(&compat::Shared<dyn Renderable>, &compat::Shared<dyn PrimitiveCollector>) {
        CreeperBeams::extract_rendering_dyn
    }

}

impl Resettable for CreeperBeams {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void reset() (dispatch: the body for this class)
    fn reset(&self) -> fn(&compat::Shared<dyn Resettable>) {
        CreeperBeams::reset_dyn
    }

}

impl CreeperBeams {
    /// java: @Override public void tick(Minecraft client)
    pub fn tick_impl(__this: &compat::Shared<Self>, client: &mut mcrust::net::minecraft::client::minecraft::Minecraft) {
        if !({ let __f = compat::dispatch(__this, |__t| __t.should_solve()); __f(&compat::view(__this, |a| a as compat::Shared<dyn DungeonPuzzle>)) }) {
            return;
        }
        if mcrust::mcrust::modding::game::level().is_none() || mcrust::mcrust::modding::game::with_player(|_| ()).is_none() || !(Utils::is_in_dungeons()) {
            return;
        }
        if BASE.get_mut().is_none() {
            *BASE.get_mut() = mcrust::mcrust::modding::game::with_player(|player| CreeperBeams::find_creeper_base(player, &mcrust::mcrust::modding::game::level().unwrap())).flatten();
            if BASE.get_mut().is_none() {
                return;
            }
            let mut creeper_pos: Vec3 = Vec3::new(((BASE.get_mut().as_ref().unwrap().get_x() as f64) + 0.5), ((BASE_Y as f64) + 1.75), ((BASE.get_mut().as_ref().unwrap().get_z() as f64) + 0.5));
            let mut targets: Vec<BlockPos> = CreeperBeams::find_targets(&mcrust::mcrust::modding::game::level().unwrap(), BASE.get_mut().as_ref().unwrap());
            *BEAMS.get_mut() = CreeperBeams::find_lines(&creeper_pos, &targets);
        }
        for b in BEAMS.get_mut().iter_mut() {
            b.update_state(&mcrust::mcrust::modding::game::level().unwrap());
        }
        if !(CreeperBeams::is_target(&mcrust::mcrust::modding::game::level().unwrap(), BASE.get_mut().as_ref().unwrap())) {
            ({ let __f = compat::dispatch(__this, |__t| __t.reset()); __f(&compat::view(__this, |a| a as compat::Shared<dyn Resettable>)) });
        }
    }

    /// CreeperBeams.tick as the dispatch of Tickable.tick
    pub fn tick_dyn(__this: &compat::Shared<dyn Tickable>, __p0: &mut mcrust::net::minecraft::client::minecraft::Minecraft) {
        Self::tick_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public void extractRendering(PrimitiveCollector collector)
    pub fn extract_rendering_impl(__this: &compat::Shared<Self>, collector: &compat::Shared<dyn PrimitiveCollector>) {
        if !({ let __f = compat::dispatch(__this, |__t| __t.should_solve()); __f(&compat::view(__this, |a| a as compat::Shared<dyn DungeonPuzzle>)) }) || !(SkyblockerConfigManager::get().dungeons.puzzle_solvers.creeper_solver) {
            return;
        }
        for i in 0..(BEAMS.get_mut().len() as i32) {
            (BEAMS.get_mut()[(i) as usize]).extract_rendering(collector, &COLORS[(i) as usize]);
        }
    }

    /// CreeperBeams.extractRendering as the dispatch of Renderable.extractRendering
    pub fn extract_rendering_dyn(__this: &compat::Shared<dyn Renderable>, __p0: &compat::Shared<dyn PrimitiveCollector>) {
        Self::extract_rendering_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public void reset()
    pub fn reset_impl(__this: &compat::Shared<Self>) {
        <dyn DungeonPuzzle>::reset_body(&compat::view(__this, |a| a as compat::Shared<dyn DungeonPuzzle>));
        BEAMS.get_mut().clear();
        *BASE.get_mut() = None;
    }

    /// CreeperBeams.reset as the dispatch of Resettable.reset
    pub fn reset_dyn(__this: &compat::Shared<dyn Resettable>) {
        Self::reset_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}

/// java: private static class Beam
#[derive(Clone)]
pub struct Beam {
    /// java: private final BlockPos blockOne;
    pub block_one: BlockPos,
    /// java: private final BlockPos blockTwo;
    pub block_two: BlockPos,
    /// java: private final Vec3[] line = new Vec3[2];
    pub line: Vec<Vec3>,
    /// java: private final AABB outlineOne;
    pub outline_one: Option<AABB>,
    /// java: private final AABB outlineTwo;
    pub outline_two: Option<AABB>,
    /// java: private boolean toDo = true;
    pub to_do: bool,
}

impl Beam {
    /// java: private Beam(BlockPos a, BlockPos b)
    pub fn new(mut a: BlockPos, mut b: BlockPos) -> Self {
        let __init_block_one = a.clone();
        let __init_block_two = b.clone();
        let mut this = Self {
            block_one: __init_block_one,
            block_two: __init_block_two,
            // new Vec3[2], both set below
            line: vec![Vec3::new(((a.get_x() as f64) + 0.5), ((a.get_y() as f64) + 0.5), ((a.get_z() as f64) + 0.5)), Vec3::new(((b.get_x() as f64) + 0.5), ((b.get_y() as f64) + 0.5), ((b.get_z() as f64) + 0.5))],
            outline_one: None,
            outline_two: None,
            to_do: true,
        };
        this.line[(0) as usize] = Vec3::new(((a.get_x() as f64) + 0.5), ((a.get_y() as f64) + 0.5), ((a.get_z() as f64) + 0.5));
        this.line[(1) as usize] = Vec3::new(((b.get_x() as f64) + 0.5), ((b.get_y() as f64) + 0.5), ((b.get_z() as f64) + 0.5));
        this.outline_one = Some(AABB::of_block_pos(&a));
        this.outline_two = Some(AABB::of_block_pos(&b));
        this
    }

    /// java: private boolean containsComponentOf(Beam other)
    pub fn contains_component_of(&self, other: &Beam) -> bool {
        return (self.block_one == other.block_one || self.block_one == other.block_two || self.block_two == other.block_one || self.block_two == other.block_two);
    }

    /// java: private void updateState(ClientLevel world)
    pub fn update_state(&mut self, world: &ClientLevel) {
        self.to_do = !(std::ptr::addr_eq(std::sync::Arc::as_ptr(&world.get_block_state_typed(&self.block_one).get_block()), std::sync::Arc::as_ptr(&blocks::blocks().get("prismarine"))) && std::ptr::addr_eq(std::sync::Arc::as_ptr(&world.get_block_state_typed(&self.block_two).get_block()), std::sync::Arc::as_ptr(&blocks::blocks().get("prismarine"))));
    }

    /// java: private void extractRendering(PrimitiveCollector collector, float[] color)
    pub fn extract_rendering(&self, collector: &compat::Shared<dyn PrimitiveCollector>, color: &Vec<f32>) {
        if self.to_do {
            ({ let __f = compat::dispatch(collector, |__t| __t.submit_outlined_box_aabb_float_float_boolean()); __f(collector, self.outline_one.as_ref().unwrap(), color, ((3) as f32) as f32, false) });
            ({ let __f = compat::dispatch(collector, |__t| __t.submit_outlined_box_aabb_float_float_boolean()); __f(collector, self.outline_two.as_ref().unwrap(), color, ((3) as f32) as f32, false) });
            ({ let __f = compat::dispatch(collector, |__t| __t.submit_lines_from_points()); __f(collector, &self.line, color, ((1) as f32) as f32, ((2) as f32) as f32, false) });
        } else {
            ({ let __f = compat::dispatch(collector, |__t| __t.submit_outlined_box_aabb_float_float_boolean()); __f(collector, self.outline_one.as_ref().unwrap(), &GREEN_COLOR_COMPONENTS, ((1) as f32) as f32, false) });
            ({ let __f = compat::dispatch(collector, |__t| __t.submit_outlined_box_aabb_float_float_boolean()); __f(collector, self.outline_two.as_ref().unwrap(), &GREEN_COLOR_COMPONENTS, ((1) as f32) as f32, false) });
            ({ let __f = compat::dispatch(collector, |__t| __t.submit_lines_from_points()); __f(collector, &self.line, &GREEN_COLOR_COMPONENTS, 0.75f32, ((1) as f32) as f32, false) });
        }
    }

}

/// `org.joml.Intersectiond.distancePointLine(pX, pY, pZ, x0, y0, z0, x1, y1, z1)` (JOML 1.10.8; a library the
/// game crate does not carry): the distance of the point from the line through the two points.
fn distance_point_line(p_x: f64, p_y: f64, p_z: f64, x0: f64, y0: f64, z0: f64, x1: f64, y1: f64, z1: f64) -> f64 {
    let d21x = x1 - x0;
    let d21y = y1 - y0;
    let d21z = z1 - z0;
    let d10x = x0 - p_x;
    let d10y = y0 - p_y;
    let d10z = z0 - p_z;
    let cross_x = d21y * d10z - d21z * d10y;
    let cross_y = d21z * d10x - d21x * d10z;
    let cross_z = d21x * d10y - d21y * d10x;
    ((cross_x * cross_x + cross_y * cross_y + cross_z * cross_z) / (d21x * d21x + d21y * d21y + d21z * d21z)).sqrt()
}

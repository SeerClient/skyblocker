//! Ported from de.hysky.skyblocker.skyblock.dungeon.puzzle.DungeonBlaze by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::dungeon::puzzle::dungeon_puzzle::DungeonPuzzle;
use crate::de::hysky::skyblocker::skyblock::dungeon::puzzle::dungeon_puzzle::DungeonPuzzleBase;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_collector::PrimitiveCollector;
use crate::de::hysky::skyblocker::utils::render::renderable::Renderable;
use crate::de::hysky::skyblocker::utils::resettable::Resettable;
use crate::de::hysky::skyblocker::utils::tickable::Tickable;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::net::minecraft::client::multiplayer::client_level::ClientLevel;
use mcrust::net::minecraft::client::player::local_player::LocalPlayer;
use mcrust::net::minecraft::world::entity::decoration::armor_stand::ArmorStand;
use mcrust::net::minecraft::world::phys::aabb::AABB;
use mcrust::net::minecraft::world::phys::vec3::Vec3;
use mcrust::net::minecraft::network::chat::component::Component as _;
use mcrust::net::minecraft::world::entity::entity::Entity as _;
use mcrust::net::minecraft::world::level::entity_getter::EntityGetter as _;

/// java: private static final float[] GREEN_COLOR_COMPONENTS = {0.0f, 1.0f, 0.0f};
pub static GREEN_COLOR_COMPONENTS: LazyLock<Vec<f32>> = LazyLock::new(|| vec![0.0f32, 1.0f32, 0.0f32]);
/// java: private static final float[] WHITE_COLOR_COMPONENTS = {1.0f, 1.0f, 1.0f};
pub static WHITE_COLOR_COMPONENTS: LazyLock<Vec<f32>> = LazyLock::new(|| vec![1.0f32, 1.0f32, 1.0f32]);
/// java: @SuppressWarnings("unused") private static final DungeonBlaze INSTANCE = new DungeonBlaze();
pub static INSTANCE: compat::MainThread<compat::Shared<DungeonBlaze>> = compat::MainThread::new(|| DungeonBlaze::new());
/// java: private static @Nullable ArmorStand highestBlaze = null;
pub static HIGHEST_BLAZE: compat::MainThread<Option<ArmorStand>> = compat::MainThread::new(|| None);
/// java: private static @Nullable ArmorStand lowestBlaze = null;
pub static LOWEST_BLAZE: compat::MainThread<Option<ArmorStand>> = compat::MainThread::new(|| None);
/// java: private static @Nullable ArmorStand nextHighestBlaze = null;
pub static NEXT_HIGHEST_BLAZE: compat::MainThread<Option<ArmorStand>> = compat::MainThread::new(|| None);
/// java: private static @Nullable ArmorStand nextLowestBlaze = null;
pub static NEXT_LOWEST_BLAZE: compat::MainThread<Option<ArmorStand>> = compat::MainThread::new(|| None);

/// java: public class DungeonBlaze extends DungeonPuzzle
#[derive(Clone)]
pub struct DungeonBlaze {
    /// java: extends DungeonPuzzle
    pub base: DungeonPuzzleBase,
}

impl std::ops::Deref for DungeonBlaze {
    type Target = DungeonPuzzleBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for DungeonBlaze {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl DungeonBlaze {
    /// java: private DungeonBlaze()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = DungeonPuzzleBase::new_in("blaze".to_string(), vec!["blaze-room-1-high".to_string(), "blaze-room-1-low".to_string()], move |__base| {
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

    /// java: private static List<ObjectIntPair<ArmorStand>> getBlazesInWorld(ClientLevel world, LocalPlayer player)
    pub fn get_blazes_in_world(world: &ClientLevel, player: &LocalPlayer) -> Vec<(ArmorStand, i32)> {
        let mut blazes: Vec<(ArmorStand, i32)> = Vec::new();
        // TODO(fabric2seer): static `EntitySelector.ENTITY_NOT_BEING_RIDDEN` has no Rust counterpart in the port's EntitySelector file
        // TODO(fabric2seer): the port's `get_entities_of_class` ports this Java method with other parameters (base_class, except, bb, selector for Java's base_class, bb, selector)
        // TODO(fabric2seer): `get_entities_of_class` takes 4 argument(s) in the port (`base_class: crate::net::minecraft::world::level::entity::entity_type_test::EntityClass, except: Option<i32>, bb: &AABB, selector: &dyn Fn(&(dyn Entity + Send + Sync)) -> bool`), Java passes 3
        for blaze in world.get_entities_of_class(std::any::TypeId::of::<ArmorStand>(), Some(player.base.base.base.base.get_bounding_box().inflate(500.0)), &crate::unresolved::EntitySelector::ENTITY_NOT_BEING_RIDDEN) {
            let mut blaze_name: String = compat::component_of(blaze.get_name()).get_string();
            if blaze_name.contains("Blaze") && blaze_name.contains("/") {
                // java: try {..} catch (NumberFormatException e) - an exception here is a panic, caught by `catch_unwind`
                let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    let mut health: i32 = (compat::regex::replace_all(&(compat::substring2(&blaze_name, (compat::index_of(&blaze_name, "/")).wrapping_add(1), (compat::str_len(&blaze_name)).wrapping_sub(1))), ",", "")).trim().parse::<i32>().expect("NumberFormatException");
                    // TODO(fabric2seer): the mod keeps the game's `ArmorStand` (Java holds a reference to the live object); the port's `ArmorStand` is not a value to copy and the port lends the mod no handle to it
                    // rust: blazes.push((blaze.clone(), health));
                }));
                if let Err(e) = __result {
                    DungeonBlaze::handle_exception(&e);
                }
            }
        }
        return blazes;
    }

    /// java: private static void sortBlazes(List<ObjectIntPair<ArmorStand>> blazes)
    pub fn sort_blazes(blazes: &mut Vec<(ArmorStand, i32)>) {
        blazes.sort_by(|a, b| { let key = |__a0| __a0.1; key(a).cmp(&key(b)) });
    }

    /// java: private static void updateBlazeEntities(List<ObjectIntPair<ArmorStand>> blazes)
    pub fn update_blaze_entities(blazes: &Vec<(ArmorStand, i32)>) {
        if !blazes.is_empty() {
            // TODO(fabric2seer): the mod keeps the game's `ArmorStand` (Java holds a reference to the live object); the port's `ArmorStand` is not a value to copy and the port lends the mod no handle to it
            // rust: *LOWEST_BLAZE.get_mut() = Some(blazes[0].0.clone());
            let mut highest_index: i32 = (blazes.len() as i32).wrapping_sub(1);
            // TODO(fabric2seer): the mod keeps the game's `ArmorStand` (Java holds a reference to the live object); the port's `ArmorStand` is not a value to copy and the port lends the mod no handle to it
            // rust: *HIGHEST_BLAZE.get_mut() = Some(blazes[(highest_index) as usize].0.clone());
            if (blazes.len() as i32) > 1 {
                // TODO(fabric2seer): the mod keeps the game's `ArmorStand` (Java holds a reference to the live object); the port's `ArmorStand` is not a value to copy and the port lends the mod no handle to it
                // rust: *NEXT_LOWEST_BLAZE.get_mut() = Some(blazes[(1) as usize].0.clone());
                // TODO(fabric2seer): the mod keeps the game's `ArmorStand` (Java holds a reference to the live object); the port's `ArmorStand` is not a value to copy and the port lends the mod no handle to it
                // rust: *NEXT_HIGHEST_BLAZE.get_mut() = Some(blazes[(highest_index.wrapping_sub(1)) as usize].0.clone());
            }
        }
    }

    /// java: private static void extractBlazeOutline(ArmorStand blaze, @Nullable ArmorStand nextBlaze, PrimitiveCollector collector)
    pub fn extract_blaze_outline(blaze: &ArmorStand, mut next_blaze: Option<ArmorStand>, collector: &compat::Shared<dyn PrimitiveCollector>) {
        let mut blaze_box: AABB = (blaze.base.base.get_bounding_box().inflate3(0.3, 0.9, 0.3)).move_by((0) as f64, -1.1, (0) as f64);
        ({ let __f = compat::dispatch(collector, |__t| __t.submit_outlined_box_aabb_float_float_boolean()); __f(collector, &blaze_box, &GREEN_COLOR_COMPONENTS, 5.0f32, false) });
        if next_blaze.is_some() && next_blaze.as_ref().unwrap().base.is_alive() && !std::ptr::eq(next_blaze.as_ref().unwrap(), blaze) {
            let mut next_blaze_box: AABB = (next_blaze.as_ref().unwrap().base.base.get_bounding_box().inflate3(0.3, 0.9, 0.3)).move_by((0) as f64, -1.1, (0) as f64);
            ({ let __f = compat::dispatch(collector, |__t| __t.submit_outlined_box_aabb_float_float_boolean()); __f(collector, &next_blaze_box, &WHITE_COLOR_COMPONENTS, 5.0f32, false) });
            let mut blaze_center: Vec3 = blaze_box.get_center();
            let mut next_blaze_center: Vec3 = next_blaze_box.get_center();
            ({ let __f = compat::dispatch(collector, |__t| __t.submit_lines_from_points()); __f(collector, &vec![blaze_center.clone(), next_blaze_center.clone()], &WHITE_COLOR_COMPONENTS, 1.0f32, 5.0f32, false) });
        }
    }

    /// java: private static void handleException(Exception e)
    pub fn handle_exception(e: &crate::unresolved::Exception) {
        logger().error(&format!("[Skyblocker BlazeRenderer] Encountered an unknown exception"));
    }

}

impl DungeonPuzzle for DungeonBlaze {
    fn dungeon_puzzle_base(&self) -> &DungeonPuzzleBase {
        &self.base
    }
    fn dungeon_puzzle_base_mut(&mut self) -> &mut DungeonPuzzleBase {
        &mut self.base
    }

}

impl Tickable for DungeonBlaze {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void tick(Minecraft client) (dispatch: the body for this class)
    fn tick(&self) -> fn(&compat::Shared<dyn Tickable>, &mut mcrust::net::minecraft::client::minecraft::Minecraft) {
        DungeonBlaze::tick_dyn
    }

}

impl Renderable for DungeonBlaze {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void extractRendering(PrimitiveCollector collector) (dispatch: the body for this class)
    fn extract_rendering(&self) -> fn(&compat::Shared<dyn Renderable>, &compat::Shared<dyn PrimitiveCollector>) {
        DungeonBlaze::extract_rendering_dyn
    }

}

impl Resettable for DungeonBlaze {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void reset() (dispatch: the body for this class)
    fn reset(&self) -> fn(&compat::Shared<dyn Resettable>) {
        <dyn DungeonPuzzle>::reset_from::<Self>
    }

}

impl DungeonBlaze {
    /// java: @Override public void tick(Minecraft client)
    pub fn tick_impl(__this: &compat::Shared<Self>, client: &mut mcrust::net::minecraft::client::minecraft::Minecraft) {
        if !({ let __f = compat::dispatch(__this, |__t| __t.should_solve()); __f(&compat::view(__this, |a| a as compat::Shared<dyn DungeonPuzzle>)) }) {
            return;
        }
        if mcrust::mcrust::modding::game::level().is_none() || mcrust::mcrust::modding::game::with_player(|_| ()).is_none() || !(Utils::is_in_dungeons()) {
            return;
        }
        let mut blazes: Vec<(ArmorStand, i32)> = mcrust::mcrust::modding::game::with_player(|player| DungeonBlaze::get_blazes_in_world(&mcrust::mcrust::modding::game::level().unwrap(), player)).unwrap();
        DungeonBlaze::sort_blazes(&mut blazes);
        DungeonBlaze::update_blaze_entities(&blazes);
    }

    /// DungeonBlaze.tick as the dispatch of Tickable.tick
    pub fn tick_dyn(__this: &compat::Shared<dyn Tickable>, __p0: &mut mcrust::net::minecraft::client::minecraft::Minecraft) {
        Self::tick_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public void extractRendering(PrimitiveCollector collector)
    pub fn extract_rendering_impl(__this: &compat::Shared<Self>, collector: &compat::Shared<dyn PrimitiveCollector>) {
        // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if HIGHEST_BLAZE.get_mut().is_some() && LOWEST_BLAZE.get_mut().is_some() && HIGHEST_BLAZE.get_mut().as_ref().unwrap().base.is_alive() && LOWEST_BLAZE.get_mut().as_ref().unwrap().base.is_alive() && SkyblockerConfigManager::get().dungeons.puzzle_solvers.blaze_solver {
                if HIGHEST_BLAZE.get_mut().as_ref().unwrap().base.base.get_y() < (69 as f64) {
                    DungeonBlaze::extract_blaze_outline(HIGHEST_BLAZE.get_mut().as_ref().unwrap(), NEXT_HIGHEST_BLAZE.get_mut().clone(), collector);
                }
                if LOWEST_BLAZE.get_mut().as_ref().unwrap().base.base.get_y() > (69 as f64) {
                    DungeonBlaze::extract_blaze_outline(LOWEST_BLAZE.get_mut().as_ref().unwrap(), NEXT_LOWEST_BLAZE.get_mut().clone(), collector);
                }
            }
        }));
        if let Err(e) = __result {
            DungeonBlaze::handle_exception(&e);
        }
    }

    /// DungeonBlaze.extractRendering as the dispatch of Renderable.extractRendering
    pub fn extract_rendering_dyn(__this: &compat::Shared<dyn Renderable>, __p0: &compat::Shared<dyn PrimitiveCollector>) {
        Self::extract_rendering_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

}

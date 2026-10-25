//! Ported from de.hysky.skyblocker.skyblock.dungeon.device.ArrowAlign by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::dungeon::dungeon_boss::DungeonBoss;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::dungeon_manager::DungeonManager;
use crate::de::hysky::skyblocker::utils::color_utils::ColorUtils;
use crate::de::hysky::skyblocker::utils::render::level_render_extraction_callback;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_collector::PrimitiveCollector;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::net::minecraft::core::block_pos::BlockPos;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::world::entity::decoration::item_frame::ItemFrame;
use mcrust::net::minecraft::world::item::items;
use mcrust::net::minecraft::world::phys::aabb::AABB;
use mcrust::net::minecraft::world::entity::item_owner::ItemOwner as _;
use mcrust::net::minecraft::world::level::entity_getter::EntityGetter as _;

/// java: private static final BlockPos LEFT_TOP = new BlockPos(-2, 124, 79);
pub static LEFT_TOP: compat::MainThread<BlockPos> = compat::MainThread::new(|| BlockPos::new(-2, 124, 79));
/// java: private static final AABB FRAMES_AREA = AABB.encapsulatingFullBlocks(LEFT_TOP, new BlockPos(-3, 120, 75));
pub static FRAMES_AREA: compat::MainThread<AABB> = compat::MainThread::new(|| AABB::encapsulating_full_blocks(LEFT_TOP.get_mut(), &BlockPos::new(-3, 120, 75)));
/// java: private static int[] currentSolution = null;
pub static CURRENT_SOLUTION: LazyLock<Mutex<Option<Vec<i32>>>> = LazyLock::new(|| Mutex::new(None));
/// java: private static boolean noSolution = false;
pub static NO_SOLUTION: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));
/// java: private static final int X = -1;
pub const ALIGN_X: i32 = -1;
/// java: private static final int S = -2;
pub const ALIGN_S: i32 = -2;
/// java: private static final int E = -3;
pub const ALIGN_E: i32 = -3;
/// java: private static final int U = 7;
pub const ALIGN_U: i32 = 7;
/// java: private static final int D = 3;
pub const ALIGN_D: i32 = 3;
/// java: private static final int L = 5;
pub const ALIGN_L: i32 = 5;
/// java: private static final int R = 1;
pub const ALIGN_R: i32 = 1;
/// java: private static final List<int[]> SOLUTIONS = List.of( new int[]{ R, R, D, X, X, U, X, D, X, X, S, X, D, X, E, X, X, D, X, U, X, X, R, R, U}, new int[]{ R, R, E, L, L, U, X, X, X, U, U, L, X, R, U, X, 
pub static ALIGN_SOLUTIONS: LazyLock<Vec<Vec<i32>>> = LazyLock::new(|| vec![vec![ALIGN_R, ALIGN_R, ALIGN_D, ALIGN_X, ALIGN_X, ALIGN_U, ALIGN_X, ALIGN_D, ALIGN_X, ALIGN_X, ALIGN_S, ALIGN_X, ALIGN_D, ALIGN_X, ALIGN_E, ALIGN_X, ALIGN_X, ALIGN_D, ALIGN_X, ALIGN_U, ALIGN_X, ALIGN_X, ALIGN_R, ALIGN_R, ALIGN_U], vec![ALIGN_R, ALIGN_R, ALIGN_E, ALIGN_L, ALIGN_L, ALIGN_U, ALIGN_X, ALIGN_X, ALIGN_X, ALIGN_U, ALIGN_U, ALIGN_L, ALIGN_X, ALIGN_R, ALIGN_U, ALIGN_X, ALIGN_U, ALIGN_X, ALIGN_U, ALIGN_X, ALIGN_X, ALIGN_S, ALIGN_X, ALIGN_S, ALIGN_X], vec![ALIGN_D, ALIGN_L, ALIGN_L, ALIGN_X, ALIGN_X, ALIGN_D, ALIGN_X, ALIGN_U, ALIGN_X, ALIGN_X, ALIGN_E, ALIGN_X, ALIGN_S, ALIGN_X, ALIGN_E, ALIGN_X, ALIGN_X, ALIGN_D, ALIGN_X, ALIGN_U, ALIGN_X, ALIGN_X, ALIGN_R, ALIGN_R, ALIGN_U], vec![ALIGN_S, ALIGN_X, ALIGN_X, ALIGN_X, ALIGN_S, ALIGN_D, ALIGN_X, ALIGN_X, ALIGN_X, ALIGN_D, ALIGN_D, ALIGN_X, ALIGN_E, ALIGN_X, ALIGN_D, ALIGN_D, ALIGN_X, ALIGN_U, ALIGN_X, ALIGN_D, ALIGN_R, ALIGN_R, ALIGN_U, ALIGN_L, ALIGN_L], vec![ALIGN_R, ALIGN_R, ALIGN_R, ALIGN_R, ALIGN_D, ALIGN_U, ALIGN_X, ALIGN_X, ALIGN_X, ALIGN_D, ALIGN_U, ALIGN_X, ALIGN_E, ALIGN_X, ALIGN_D, ALIGN_U, ALIGN_X, ALIGN_U, ALIGN_X, ALIGN_D, ALIGN_S, ALIGN_X, ALIGN_U, ALIGN_L, ALIGN_L], vec![ALIGN_R, ALIGN_R, ALIGN_D, ALIGN_X, ALIGN_E, ALIGN_U, ALIGN_X, ALIGN_D, ALIGN_X, ALIGN_U, ALIGN_U, ALIGN_X, ALIGN_D, ALIGN_X, ALIGN_U, ALIGN_U, ALIGN_X, ALIGN_D, ALIGN_X, ALIGN_U, ALIGN_S, ALIGN_X, ALIGN_R, ALIGN_R, ALIGN_U], vec![ALIGN_S, ALIGN_R, ALIGN_R, ALIGN_R, ALIGN_E, ALIGN_X, ALIGN_X, ALIGN_X, ALIGN_X, ALIGN_X, ALIGN_S, ALIGN_R, ALIGN_R, ALIGN_R, ALIGN_E, ALIGN_X, ALIGN_X, ALIGN_X, ALIGN_X, ALIGN_X, ALIGN_S, ALIGN_R, ALIGN_R, ALIGN_R, ALIGN_E], vec![ALIGN_X, ALIGN_R, ALIGN_R, ALIGN_D, ALIGN_X, ALIGN_X, ALIGN_U, ALIGN_X, ALIGN_D, ALIGN_X, ALIGN_X, ALIGN_U, ALIGN_X, ALIGN_D, ALIGN_X, ALIGN_X, ALIGN_U, ALIGN_X, ALIGN_D, ALIGN_X, ALIGN_S, ALIGN_U, ALIGN_X, ALIGN_R, ALIGN_E], vec![ALIGN_S, ALIGN_R, ALIGN_D, ALIGN_X, ALIGN_X, ALIGN_X, ALIGN_X, ALIGN_R, ALIGN_R, ALIGN_E, ALIGN_S, ALIGN_R, ALIGN_U, ALIGN_X, ALIGN_X, ALIGN_X, ALIGN_X, ALIGN_R, ALIGN_R, ALIGN_E, ALIGN_S, ALIGN_R, ALIGN_U, ALIGN_X, ALIGN_X]]);

/// java: public class ArrowAlign
#[derive(Clone)]
pub struct ArrowAlign {}

impl ArrowAlign {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        client_play_connection_events::JOIN.register(Arc::new(|listener: &mut ClientPacketListener| {
            ArrowAlign::reset();
        }));
        level_render_extraction_callback::EVENT.register(Arc::new(move |__a0| { ArrowAlign::extract_rendering(&__a0); }));
        client_command_registration_callback::EVENT.register(Arc::new(
            |dispatcher: &mut client_command_registration_callback::ClientCommandDispatcher| {
                use mcrust::com::mojang::brigadier::builder::argument_builder::ArgumentBuilder as _;
                use crate::prelude::client_command_registration_callback as ccr;
                type Ctx = mcrust::com::mojang::brigadier::context::command_context::CommandContext<ccr::FabricClientCommandSource>;
                fn prefixed(text: mcrust::net::minecraft::network::chat::mutable_component::MutableComponent) -> mcrust::net::minecraft::network::chat::mutable_component::MutableComponent {
                    (crate::de::hysky::skyblocker::utils::constants::PREFIX.get())().append(text)
                }
                // java: dispatcher.register(literal(NAMESPACE).then(literal("dungeons").then(literal("device").then(literal("arrow-align")
                // java:     .then(literal("solve").executes(_ -> { findSolution(); return Command.SINGLE_SUCCESS; }))))));
                let _ = prefixed;
                let mut solve = ccr::literal("solve");
                solve.executes(Arc::new(|_: &Ctx| { ArrowAlign::find_solution(); Ok(1) }));
                let mut __n0 = ccr::literal("arrow-align");
                __n0.then(&solve);
                let mut __n1 = ccr::literal("device");
                __n1.then(&__n0);
                let mut __n2 = ccr::literal("dungeons");
                __n2.then(&__n1);
                let mut __n3 = ccr::literal(crate::entry::skyblocker_mod::NAMESPACE);
                __n3.then(&__n2);
                dispatcher.dispatcher().register(&__n3);
            },
        ));
    }

    /// java: private static void extractRendering(PrimitiveCollector collector)
    pub fn extract_rendering(collector: &compat::Shared<dyn PrimitiveCollector>) {
        if ArrowAlign::should_process() && mcrust::mcrust::modding::game::with_player(|player| player.position().distance_to_sqr_vec(&FRAMES_AREA.get_mut().get_center())).unwrap() < (64 as f64) {
            if compat::get_state(&CURRENT_SOLUTION).is_none() && !(compat::get_state(&NO_SOLUTION)) {
                ArrowAlign::find_solution();
            }
            if !(compat::get_state(&NO_SOLUTION)) {
                let mut frame_entities_list: Vec<ItemFrameView> = ArrowAlign::get_frame_entities_list();
                for frame_entity in frame_entities_list.iter() {
                    let mut now: i32 = frame_entity.rotation;
                    let mut expect: i32 = compat::lock(&CURRENT_SOLUTION).as_ref().unwrap()[(ArrowAlign::get_solution_index(&frame_entity.block_pos)) as usize];
                    if expect >= 0 {
                        let mut remaining: i32 = ((expect.wrapping_add(8)).wrapping_sub(now)).wrapping_rem(8);
                        if remaining > 0 {
                            ({ let __f = compat::dispatch(collector, |__t| __t.submit_text()); __f(collector, &component::literal(&remaining.to_string()).with_color(ColorUtils::interpolate(0xFF00FF00u32 as i32, 0xFFFF0000u32 as i32, ((remaining as f64) / 7.0))), &frame_entity.position.add(0.3, (0) as f64, (0) as f64), false) });
                        }
                    }
                }
            }
        }
    }

    /// java: private static List<ItemFrame> getFrameEntitiesList()
    /// (each frame is read under its entity lock once and handed back as the fields the callers read)
    pub fn get_frame_entities_list() -> Vec<ItemFrameView> {
        // java: return CLIENT.level.getEntitiesOfClass(ItemFrame.class, FRAMES_AREA, _ -> true);
        let Some(level) = mcrust::mcrust::modding::game::level() else {
            return Vec::new();
        };
        let area = FRAMES_AREA.get_mut().clone();
        let entities = level.get_entities_of_class(
            mcrust::net::minecraft::world::level::entity::entity_type_test::EntityClass::Entity,
            None,
            &area,
            &|entity| entity.as_any().is_some_and(|any| any.is::<ItemFrame>()),
        );
        let mut frames: Vec<ItemFrameView> = Vec::with_capacity(entities.len());
        for entity in entities.iter() {
            let guard = mcrust::mcrust::modding::game::lock_entity(&**entity, "arrow_align.rs:ArrowAlign.getFrameEntitiesList");
            if let Some(frame) = guard.as_any().and_then(|any| any.downcast_ref::<ItemFrame>()) {
                frames.push(ItemFrameView { rotation: frame.get_rotation(), block_pos: frame.base.block_position(), position: frame.base.position(), item: frame.get_item() });
            }
        }
        frames
    }

    /// java: private static int getSolutionIndex(BlockPos pos)
    pub fn get_solution_index(pos: &BlockPos) -> i32 {
        return ((((LEFT_TOP.get_mut().get_y()).wrapping_sub(pos.get_y())).wrapping_mul(5)).wrapping_add(LEFT_TOP.get_mut().get_z())).wrapping_sub(pos.get_z());
    }

    /// java: private static void findSolution()
    pub fn find_solution() {
        let mut frame_entities_list: Vec<ItemFrameView> = ArrowAlign::get_frame_entities_list();
        let mut solution: Option<Vec<i32>> = ALIGN_SOLUTIONS.iter().cloned().filter(|rotations| {
            for item_frame in frame_entities_list.iter() {
                match rotations[(ArrowAlign::get_solution_index(&item_frame.block_pos)) as usize] {
                    ALIGN_X => {
                        return false;
                    }
                    ALIGN_S => {
                        if !(item_frame.item.is_item(mcrust::net::minecraft::world::item::items::wool().lime())) {
                            return false;
                        }
                    }
                    ALIGN_E => {
                        if !(item_frame.item.is_item(mcrust::net::minecraft::world::item::items::wool().red())) {
                            return false;
                        }
                    }
                    _ => {
                        if !(item_frame.item.is_item(&items::items().get("arrow"))) {
                            return false;
                        }
                    }
                }
            }
            return true;
        }).next();
        compat::set_state(&CURRENT_SOLUTION, solution.clone());
        compat::set_state(&NO_SOLUTION, solution.is_none());
        if compat::get_state(&NO_SOLUTION) {
            logger().error("[Skyblocker Arrow Align] Failed to find a solution for Arrow Align device!");
        }
    }

    /// java: private static boolean shouldProcess()
    pub fn should_process() -> bool {
        return (SkyblockerConfigManager::get().dungeons.devices.solve_arrow_align && Utils::is_in_dungeons() && DungeonManager::is_in_boss() && DungeonManager::get_boss() == DungeonBoss::Maxor);
    }

    /// java: private static void reset()
    pub fn reset() {
        compat::set_state(&CURRENT_SOLUTION, None);
        compat::set_state(&NO_SOLUTION, false);
    }

}

/// java: private static class Align
#[derive(Clone)]
pub struct Align {}

impl Align {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

}

/// The fields of an `ItemFrame` this class reads (`getRotation()`, `blockPosition()`, `position()`, `getItem()`),
/// taken under the entity's lock by `get_frame_entities_list`.
pub struct ItemFrameView {
    pub rotation: i32,
    pub block_pos: BlockPos,
    pub position: mcrust::net::minecraft::world::phys::vec3::Vec3,
    pub item: mcrust::net::minecraft::world::item::item_stack::ItemStack,
}

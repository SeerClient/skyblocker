//! Ported from de.hysky.skyblocker.skyblock.foraging.torrhus.DesertTemplePuzzle by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::utils::area::TorrhusCanyon;
use crate::de::hysky::skyblocker::utils::render::level_render_extraction_callback;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_collector::PrimitiveCollector;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler::Scheduler;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::net::minecraft::client::multiplayer::client_level::ClientLevel;
use mcrust::net::minecraft::core::block_pos::BlockPos;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::util::mth;
use mcrust::net::minecraft::world::item::dye_color::DyeColor;
use mcrust::net::minecraft::world::level::block::stained_glass_block;
use mcrust::net::minecraft::world::level::block::stained_glass_block::StainedGlassBlock;
use mcrust::net::minecraft::world::phys::vec3;
use mcrust::net::minecraft::world::phys::vec3::Vec3;
use mcrust::org::joml::quaternionf::Quaternionf;
use mcrust::org::joml::vector3f::Vector3f;
use mcrust::org::joml::vector3i::Vector3i;

/// java: private static final Map<DyeColor, BlockPos> BUTTON_LOCATIONS = Map.ofEntries( Map.entry(DyeColor.BROWN, new BlockPos(-611, 49, 232)), Map.entry(DyeColor.RED, new BlockPos(-612, 49, 232)), Map.entry(D
pub static BUTTON_LOCATIONS: compat::MainThread<std::collections::HashMap<DyeColor, BlockPos>> = compat::MainThread::new(|| std::collections::HashMap::from([(DyeColor::Brown, BlockPos::new(-611, 49, 232)), (DyeColor::Red, BlockPos::new(-612, 49, 232)), (DyeColor::Orange, BlockPos::new(-613, 49, 232)), (DyeColor::Yellow, BlockPos::new(-614, 48, 232)), (DyeColor::Lime, BlockPos::new(-614, 47, 232)), (DyeColor::Green, BlockPos::new(-614, 46, 232)), (DyeColor::Cyan, BlockPos::new(-613, 45, 232)), (DyeColor::LightBlue, BlockPos::new(-612, 45, 232)), (DyeColor::Blue, BlockPos::new(-611, 45, 232)), (DyeColor::Purple, BlockPos::new(-610, 46, 232)), (DyeColor::Magenta, BlockPos::new(-610, 47, 232)), (DyeColor::Pink, BlockPos::new(-610, 48, 232))]));
/// java: private static final BlockPos GLASS_FLOOR_CENTRE = new BlockPos(-612, 42, 227);
pub static GLASS_FLOOR_CENTRE: compat::MainThread<BlockPos> = compat::MainThread::new(|| BlockPos::new(-612, 42, 227));
/// java: private static final Vector3ic[] NEIGHBOUR_OFFSETS = new Vector3ic[] { // North // East // South // West new Vector3i(0, 0, -1), new Vector3i(1, 0, 0), new Vector3i(0, 0, 1), new Vector3i(-1, 0, 0) };
pub static NEIGHBOUR_OFFSETS: compat::MainThread<Vec<Vector3i>> = compat::MainThread::new(|| vec![Vector3i::new(0, 0, -1), Vector3i::new(1, 0, 0), Vector3i::new(0, 0, 1), Vector3i::new(-1, 0, 0)]);
/// java: private static final BlockPos GLASS_WHEEL_CENTRE = new BlockPos(-612, 47, 233);
pub static GLASS_WHEEL_CENTRE: compat::MainThread<BlockPos> = compat::MainThread::new(|| BlockPos::new(-612, 47, 233));
/// java: private static final Map<BlockPos, BlockPos> STATUE_ARROWS = Map.ofEntries( // Bird Map.entry(new BlockPos(-618, 46, 248), new BlockPos(-616, 46, 248)), // Snake Map.entry(new BlockPos(-607, 46, 249),
pub static STATUE_ARROWS: compat::MainThread<std::collections::HashMap<BlockPos, BlockPos>> = compat::MainThread::new(|| std::collections::HashMap::from([(BlockPos::new(-618, 46, 248), BlockPos::new(-616, 46, 248)), (BlockPos::new(-607, 46, 249), BlockPos::new(-607, 46, 247)), (BlockPos::new(-606, 46, 240), BlockPos::new(-608, 46, 240)), (BlockPos::new(-617, 46, 241), BlockPos::new(-617, 46, 239))]));
/// java: private static final float[] WHITE = { 1f, 1f, 1f };
pub static WHITE: LazyLock<Vec<f32>> = LazyLock::new(|| vec![1.0f32, 1.0f32, 1.0f32]);
/// java: private static final float LINE_WIDTH = 5f;
pub const LINE_WIDTH: f32 = 5.0f32;
/// java: private static List<DyeColor> buttonOrder = List.of();
pub static BUTTON_ORDER: compat::MainThread<Vec<DyeColor>> = compat::MainThread::new(|| vec![]);

/// java: public class DesertTemplePuzzle
#[derive(Clone)]
pub struct DesertTemplePuzzle {}

impl DesertTemplePuzzle {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        ({ let __r = scheduler::INSTANCE.get_mut(); let __f = compat::dispatch(__r, |__t| __t.schedule_cyclic()); __f(__r, Arc::new(move || { DesertTemplePuzzle::calculate_solution(); }), 20i32.wrapping_mul(5)) });
        level_render_extraction_callback::EVENT.register(Arc::new(move |__a0| { DesertTemplePuzzle::extract_rendering(&__a0); }));
        client_play_connection_events::JOIN.register(Arc::new(|listener: &mut ClientPacketListener| {
            DesertTemplePuzzle::reset();
        }));
    }

    /// java: private static boolean shouldProcess()
    pub fn should_process() -> bool {
        let mut in_area: bool = (compat::read(&Utils::get_area(), |__t| compat::DynAny::dyn_any(__t).downcast_ref::<TorrhusCanyon>().is_some_and(|__e| *__e == TorrhusCanyon::TorrhusSprings)) || compat::read(&Utils::get_area(), |__t| compat::DynAny::dyn_any(__t).downcast_ref::<TorrhusCanyon>().is_some_and(|__e| *__e == TorrhusCanyon::DesertTemple)));
        return (SkyblockerConfigManager::get().foraging.torrhus_canyon.solve_desert_temple_puzzles && Utils::is_in_torrhus_canyon() && in_area && mcrust::mcrust::modding::game::level().is_some());
    }

    /// java: private static void calculateSolution()
    pub fn calculate_solution() {
        if DesertTemplePuzzle::should_process() {
            *BUTTON_ORDER.get_mut() = DesertTemplePuzzle::solve(&mcrust::mcrust::modding::game::level().unwrap());
        }
    }

    /// java: private static List<DyeColor> solve(ClientLevel level)
    pub fn solve(level: &ClientLevel) -> Vec<DyeColor> {
        let mut colours: Vec<DyeColor> = Vec::new();
        let mut visited: std::collections::HashSet<BlockPos> = std::collections::HashSet::new();
        let mut queue: std::collections::VecDeque<BlockPos> = std::collections::VecDeque::new();
        queue.push_back(GLASS_FLOOR_CENTRE.get_mut().clone());
        while !queue.is_empty() {
            // java: BlockPos current = queue.remove();
            let mut current: BlockPos = queue.pop_front().expect("NoSuchElementException");
            // java: if (level.getBlockState(current).getBlock() instanceof StainedGlassBlock glassBlock) colours.add(glassBlock.getColor());
            let __block = level.get_block_state_typed(&current).get_block();
            if stained_glass_block::is_instance(&__block) {
                colours.push(stained_glass_block::get_color(&__block));
            }
            for offset in NEIGHBOUR_OFFSETS.get_mut().iter() {
                let mut neighbour: BlockPos = current.offset(offset.x, offset.y, offset.z);
                if visited.contains(&neighbour) || queue.iter().any(|e| *e == neighbour.clone()) {
                    continue;
                } else if stained_glass_block::is_instance(&level.get_block_state_typed(&neighbour).get_block()) {
                    visited.insert(neighbour.clone());
                    queue.push_back(neighbour.clone());
                }
            }
        }
        // java: Map<DyeColor, Long> frequencyMap = colours.stream().collect(Collectors.groupingBy(Function.identity(), Collectors.counting()));
        let mut frequency_map: Vec<(DyeColor, i64)> = Vec::new();
        for colour in colours.iter() {
            match frequency_map.iter_mut().find(|(k, _)| k == colour) {
                Some(entry) => entry.1 += 1,
                None => frequency_map.push((*colour, 1)),
            }
        }
        // java: frequencyMap.entrySet().stream().sorted(Map.Entry.comparingByValue()).map(Map.Entry::getKey).toList()
        // (a stable sort; equal counts keep the colour order, where Java keeps the HashMap's)
        frequency_map.sort_by_key(|(colour, _)| colour.get_id());
        frequency_map.sort_by_key(|(_, count)| *count);
        let mut button_order: Vec<DyeColor> = frequency_map.into_iter().map(|(colour, _)| colour).collect();
        return button_order;
    }

    /// java: private static void extractRendering(PrimitiveCollector collector)
    pub fn extract_rendering(collector: &compat::Shared<dyn PrimitiveCollector>) {
        if DesertTemplePuzzle::should_process() {
            if !(BUTTON_ORDER.get_mut().is_empty()) && !(mcrust::mcrust::modding::game::level().unwrap().get_block_state_typed(GLASS_WHEEL_CENTRE.get_mut()).is_air()) {
                for i in 0..(BUTTON_ORDER.get_mut().len() as i32) {
                    let mut colour: DyeColor = BUTTON_ORDER.get_mut()[(i) as usize];
                    let mut pos: Option<BlockPos> = BUTTON_LOCATIONS.get_mut().get(&colour).cloned();
                    if let Some(pos) = &pos {
                        ({ let __f = compat::dispatch(collector, |__t| __t.submit_text()); __f(collector, &component::literal(&i.wrapping_add(1).to_string()), &vec3::at_bottom_center_of(pos).add(0.0, 0.2, 0.2), false) });
                    }
                }
            }
            for entry in STATUE_ARROWS.get_mut().iter().map(|(k, v)| (k.clone(), v.clone())) {
                let mut start: Vec3 = vec3::at_center_of(&entry.0);
                let mut end: Vec3 = vec3::at_center_of(&entry.1);
                ({ let __f = compat::dispatch(collector, |__t| __t.submit_lines_from_points()); __f(collector, &vec![start.clone(), end.clone()], &WHITE, 1.0f32, LINE_WIDTH, false) });
                // java: Quaternionf rotation = new Quaternionf().rotationTo(new Vector3f(1.0f, 0.0f, 0.0f), end.subtract(start).toVector3f().normalize());
                let __to = end.subtract_vec(&start).to_vector3f().normalize();
                let mut rotation: Quaternionf = Quaternionf::new();
                rotation.rotation_to(1.0f32, 0.0f32, 0.0f32, __to.x, __to.y, __to.z);
                let mut len: f32 = ((mth::clamp_f64((end.distance_to(&start) * (0.1f32 as f64)), (0.1f32) as f64, 1.0)) as f32);
                let mut tips: Vec<Vector3f> = vec![rotation.transform((-len), len, 0.0f32), rotation.transform((-len), 0.0f32, len), rotation.transform((-len), (-len), 0.0f32), rotation.transform((-len), 0.0f32, (-len))];
                for tip in tips.iter() {
                    ({ let __f = compat::dispatch(collector, |__t| __t.submit_lines_from_points()); __f(collector, &vec![end.add((tip.x()) as f64, (tip.y()) as f64, (tip.z()) as f64), end.clone()], &WHITE, 1.0f32, LINE_WIDTH, false) });
                }
            }
        }
    }

    /// java: private static void reset()
    pub fn reset() {
        *BUTTON_ORDER.get_mut() = vec![];
    }

}

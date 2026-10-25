//! Ported from de.hysky.skyblocker.skyblock.hunting.FloorDrops by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::events::particle_events;
use crate::de::hysky::skyblocker::utils::render::level_render_extraction_callback;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_collector::PrimitiveCollector;
use crate::de::hysky::skyblocker::utils::render::renderable::Renderable;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler::Scheduler;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use crate::de::hysky::skyblocker::utils::waypoint::waypoint::Type;
use crate::de::hysky::skyblocker::utils::waypoint::waypoint::Waypoint;
use crate::de::hysky::skyblocker::utils::waypoint::waypoint::WaypointBase;
use mcrust::net::minecraft::core::block_pos::BlockPos;
use mcrust::net::minecraft::core::particles::particle_type::ParticleType;
use mcrust::net::minecraft::network::protocol::game::clientbound_level_particles_packet::ClientboundLevelParticlesPacket;
use mcrust::net::minecraft::world::entity::display::ItemDisplay;
use mcrust::net::minecraft::world::item::items;
use mcrust::net::minecraft::world::phys::aabb::AABB;
use mcrust::net::minecraft::world::phys::vec3;
use mcrust::net::minecraft::core::particles::particle_options::ParticleOptions as _;
use mcrust::net::minecraft::world::level::entity_getter::EntityGetter as _;

/// java: private static final Map<BlockPos, ForestNode> forestNodes = new HashMap<>();
pub static FOREST_NODES: compat::MainThread<std::collections::HashMap<BlockPos, compat::Shared<ForestNode>>> = compat::MainThread::new(|| std::collections::HashMap::new());

/// java: public class FloorDrops
#[derive(Clone)]
pub struct FloorDrops {}

impl FloorDrops {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        ({ let __r = scheduler::INSTANCE.get_mut(); let __f = compat::dispatch(__r, |__t| __t.schedule_cyclic()); __f(__r, Arc::new(move || { FloorDrops::update(); }), 20) });
        level_render_extraction_callback::EVENT.register(Arc::new(move |__a0| { FloorDrops::extract_rendering(&__a0); }));
        // java: AttackBlockCallback.EVENT.register((_, _, _, pos, _) -> { if (!shouldProcess()) return PASS; forestNodes.remove(pos); return PASS; });
        mcrust::mcrust::modding::events::attack_block_callback::register(|_, _, _, pos, _| {
            if FloorDrops::should_process() {
                FOREST_NODES.get_mut().remove(pos);
            }
            mcrust::net::minecraft::world::interaction_result::InteractionResult::Pass
        });
        // java: UseBlockCallback.EVENT.register((_, _, _, hitResult) -> { if (!shouldProcess()) return PASS; forestNodes.remove(hitResult.getBlockPos()); return PASS; });
        mcrust::mcrust::modding::events::use_block_callback::register(|_, _, _, hit_result| {
            if FloorDrops::should_process() {
                FOREST_NODES.get_mut().remove(&hit_result.get_block_pos());
            }
            mcrust::net::minecraft::world::interaction_result::InteractionResult::Pass
        });
        client_play_connection_events::JOIN.register(Arc::new(|listener: &mut ClientPacketListener| {
            FloorDrops::reset();
        }));
        particle_events::FROM_SERVER.register(Arc::new(move |__a0| { FloorDrops::on_particle(&__a0); }));
    }

    /// java: private static void onParticle(ClientboundLevelParticlesPacket packet)
    pub fn on_particle(packet: &ClientboundLevelParticlesPacket) {
        if !(FloorDrops::should_process()) {
            return;
        }
        // java: ParticleType<?> particleType = packet.getParticle().getType(); if (!ParticleTypes.HAPPY_VILLAGER.getType().equals(particleType)) return;
        if !packet.particle().is_some_and(|particle| compat::particle_type_is(particle, mcrust::net::minecraft::core::particles::particle_types::happy_villager())) {
            return;
        }
        let mut x: f64 = packet.x();
        let mut y: f64 = (packet.y() - (1 as f64));
        let mut z: f64 = packet.z();
        let mut pos: BlockPos = BlockPos::containing(x, y, z);
        if mcrust::mcrust::modding::game::level().is_some() {
            let mut string_item_count: i32 = FloorDrops::count_string_item_displays(&pos);
            if string_item_count == 3 {
                FOREST_NODES.get_mut().entry(pos.clone()).or_insert_with_key(|k: &BlockPos| ForestNode::new(k.clone()));
            }
        }
    }

    /// java: private static int countStringItemDisplays(BlockPos pos)
    pub fn count_string_item_displays(pos: &BlockPos) -> i32 {
        let Some(mut world) = mcrust::mcrust::modding::game::level() else {
            return 0;
        };
        // java: world.getEntitiesOfClass(Display.ItemDisplay.class, AABB.ofSize(Vec3.atCenterOf(pos), 1, 1, 1), _ -> true)
        // (the port's class tests stop at LivingEntity; the entity type is ItemDisplay's)
        let entities = world.get_entities(None, &AABB::of_size(&vec3::at_center_of(pos), 1.0, 1.0, 1.0), &|entity| compat::is_entity_type(entity, "item_display"));
        return (((entities.iter().filter(|entity| {
            compat::with_entity_handle(entity, |entity| {
                let Some(entity) = entity.as_any().and_then(|any| any.downcast_ref::<ItemDisplay>()) else {
                    return false;
                };
                let Some(mut state) = entity.item_render_state() else {
                    return false;
                };
                let stack = state.item_stack();
                return (!stack.is_empty() && std::ptr::addr_eq(std::sync::Arc::as_ptr(&stack.get_item()), std::sync::Arc::as_ptr(&items::items().get("string"))));
            }).unwrap_or(false)
        }).count() as i64)) as i32);
    }

    /// java: private static void update()
    pub fn update() {
        if !(FloorDrops::should_process()) {
            return;
        }
        //   java: Iterator<Map.Entry<BlockPos, ForestNode>> iterator = forestNodes.entrySet().iterator(); while (iterator.hasNext()) {..}
        // (`iterator.remove()` of the entries whose node stops rendering: `retain`, over the live map)
        FOREST_NODES.get_mut().retain(|_, forest_node| {
            ForestNode::update_waypoint(forest_node);
            { let __f = compat::dispatch(forest_node, |__t| __t.should_render()); __f(&compat::view(forest_node, |a| a as compat::Shared<dyn Waypoint>)) }
        });
    }

    /// java: private static void extractRendering(PrimitiveCollector collector)
    pub fn extract_rendering(collector: &compat::Shared<dyn PrimitiveCollector>) {
        if !(FloorDrops::should_process()) {
            return;
        }
        for forest_node in FOREST_NODES.get_mut().values().cloned() {
            if { let __f = compat::dispatch(&forest_node, |__t| __t.should_render()); __f(&compat::view(&forest_node, |a| a as compat::Shared<dyn Waypoint>)) } {
                ({ let __f = compat::dispatch(&forest_node, |__t| __t.extract_rendering()); __f(&compat::view(&forest_node, |a| a as compat::Shared<dyn Renderable>), collector) });
            }
        }
    }

    /// java: private static boolean shouldProcess()
    pub fn should_process() -> bool {
        return (SkyblockerConfigManager::get().hunting.floor_drops.highlight_floor_drops && (Utils::is_in_galatea() || Utils::is_in_torrhus_canyon() || Utils::is_in_safari()));
    }

    /// java: private static void reset()
    pub fn reset() {
        FOREST_NODES.get_mut().clear();
    }

}

/// java: public static class ForestNode extends Waypoint
#[derive(Clone)]
pub struct ForestNode {
    /// java: extends Waypoint
    pub base: WaypointBase,
    /// java: private long lastConfirmed;
    pub last_confirmed: i64,
}

impl std::ops::Deref for ForestNode {
    type Target = WaypointBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for ForestNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl ForestNode {
    /// java: private ForestNode(BlockPos pos)
    pub fn new(mut pos: BlockPos) -> compat::Shared<Self> {
        let __init_last_confirmed = compat::millis();
        //   java: super(pos, () -> Type.HIGHLIGHT, ColorUtils.getFloatComponents(SkyblockerConfigManager.get().hunting.floorDrops.floorDropHighlightColor.getRGB()), DEFAULT_HIGHLIGHT_ALPHA, DEFAULT_LINE_WIDTH, false);
        let __color_components: Vec<f32> = crate::de::hysky::skyblocker::utils::color_utils::ColorUtils::get_float_components(SkyblockerConfigManager::get().hunting.floor_drops.floor_drop_highlight_color.get_rgb());
        let (_, __this) = WaypointBase::new_6_in(pos, Arc::new(|| Type::Highlight), __color_components, crate::de::hysky::skyblocker::utils::waypoint::waypoint::DEFAULT_HIGHLIGHT_ALPHA, crate::de::hysky::skyblocker::utils::waypoint::waypoint::DEFAULT_LINE_WIDTH, false, move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                last_confirmed: __init_last_confirmed,
            });
            ((__h.clone() as compat::Shared<dyn Waypoint>), __h)
        });
        __this
    }

    /// java: private void updateWaypoint()
    pub fn update_waypoint(__this: &compat::Shared<Self>) {
        let mut current_time_millis: i64 = compat::millis();
        if (compat::read(__this, |__t| __t.last_confirmed)).wrapping_add(2000 as i64) > current_time_millis || mcrust::mcrust::modding::game::level().is_none() {
            return;
        }
        let mut string_item_count: i32 = FloorDrops::count_string_item_displays(&compat::read(__this, |__t| __t.pos.clone()).unwrap());
        if string_item_count == 3 {
            { let __v = compat::millis(); compat::write(__this, |__t| __t.last_confirmed = __v); }
        }
    }

}

impl Waypoint for ForestNode {
    fn waypoint_base(&self) -> &WaypointBase {
        &self.base
    }
    fn waypoint_base_mut(&mut self) -> &mut WaypointBase {
        &mut self.base
    }

    /// java: @Override public boolean shouldRender() (dispatch: the body for this class)
    fn should_render(&self) -> fn(&compat::Shared<dyn Waypoint>) -> bool {
        ForestNode::should_render_dyn
    }

}

impl Renderable for ForestNode {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void extractRendering(PrimitiveCollector collector) (dispatch: the body for this class)
    fn extract_rendering(&self) -> fn(&compat::Shared<dyn Renderable>, &compat::Shared<dyn PrimitiveCollector>) {
        <dyn Waypoint>::extract_rendering_from::<Self>
    }

}

impl ForestNode {
    /// java: @Override public boolean shouldRender()
    pub fn should_render_impl(__this: &compat::Shared<Self>) -> bool {
        return (<dyn Waypoint>::should_render_body(&compat::view(__this, |a| a as compat::Shared<dyn Waypoint>)) && (compat::read(__this, |__t| __t.last_confirmed)).wrapping_add(5000 as i64) > compat::millis());
    }

    /// ForestNode.shouldRender as the dispatch of Waypoint.shouldRender
    pub fn should_render_dyn(__this: &compat::Shared<dyn Waypoint>) -> bool {
        Self::should_render_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}

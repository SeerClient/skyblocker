//! Ported from de.hysky.skyblocker.skyblock.barn.GlowingMushrooms by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::com::mojang::serialization::JavaObject as _;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::events::particle_events;
use crate::de::hysky::skyblocker::utils::area::TheFarmingIslands;
use crate::de::hysky::skyblocker::utils::color_utils::ColorUtils;
use crate::de::hysky::skyblocker::utils::render::level_render_extraction_callback;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_collector::PrimitiveCollector;
use crate::de::hysky::skyblocker::utils::render::render_helper::RenderHelper;
use crate::de::hysky::skyblocker::utils::render::renderable::Renderable;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler::Scheduler;
use crate::de::hysky::skyblocker::utils::tickable::Tickable;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use crate::de::hysky::skyblocker::utils::waypoint::seen_waypoint::SeenWaypoint;
use crate::de::hysky::skyblocker::utils::waypoint::seen_waypoint::SeenWaypointBase;
use crate::de::hysky::skyblocker::utils::waypoint::waypoint::Type;
use crate::de::hysky::skyblocker::utils::waypoint::waypoint::Waypoint;
use crate::de::hysky::skyblocker::utils::waypoint::waypoint::WaypointBase;
use mcrust::net::minecraft::core::block_pos::BlockPos;
use mcrust::net::minecraft::network::protocol::game::clientbound_level_particles_packet::ClientboundLevelParticlesPacket;
use mcrust::net::minecraft::world::item::dye_color::DyeColor;
use mcrust::net::minecraft::world::level::block::block::Block;
use mcrust::net::minecraft::world::level::block::blocks;
use mcrust::net::minecraft::world::phys::aabb::AABB;
use mcrust::net::minecraft::core::particles::particle_options::ParticleOptions as _;

/// java: private static final Map<BlockPos, GlowingMushrooms.GlowingMushroom> glowingMushrooms = new HashMap<>();
pub static GLOWING_MUSHROOMS: compat::MainThread<std::collections::HashMap<BlockPos, compat::Shared<GlowingMushroom>>> = compat::MainThread::new(|| std::collections::HashMap::new());

/// java: public class GlowingMushrooms
#[derive(Clone)]
pub struct GlowingMushrooms {}

impl GlowingMushrooms {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        particle_events::FROM_SERVER.register(Arc::new(move |__a0| { GlowingMushrooms::on_particle(&__a0); }));
        ({ let __r = scheduler::INSTANCE.get_mut(); let __f = compat::dispatch(__r, |__t| __t.schedule_cyclic()); __f(__r, Arc::new(move || { GlowingMushrooms::update(); }), 1) });
        level_render_extraction_callback::EVENT.register(Arc::new(move |__a0| { GlowingMushrooms::extract_rendering(&__a0); }));
        // TODO(fabric2seer): AttackBlockCallback.EVENT.register: no SeerClient equivalent yet
        //   java: AttackBlockCallback.EVENT.register((_, _, _, pos, _) -> {
        //   java: 			if (shouldProcess()) glowingMushrooms.remove(pos);
        //   java: 			return InteractionResult.PASS;
        //   java: 		});
        client_play_connection_events::JOIN.register(Arc::new(|listener: &mut ClientPacketListener| {
            GlowingMushrooms::reset();
        }));
    }

    /// java: public static void onParticle(ClientboundLevelParticlesPacket packet)
    pub fn on_particle(packet: &ClientboundLevelParticlesPacket) {
        if !(GlowingMushrooms::should_process()) || mcrust::mcrust::modding::game::level().is_none() {
            return;
        }
        // `ParticleTypes.ENTITY_EFFECT.equals(packet.getParticle().getType())`: identity of the registered type
        if !(packet.get_particle().unwrap().get_type().java_equals(&**mcrust::net::minecraft::core::particles::particle_types::entity_effect())) {
            return;
        }
        let mut pos: BlockPos = BlockPos::containing(packet.get_x(), packet.get_y(), packet.get_z());
        let mut block: Arc<Block> = mcrust::mcrust::modding::game::level().unwrap().get_block_state_typed(&pos).get_block();
        if !std::ptr::addr_eq(std::sync::Arc::as_ptr(&block), std::sync::Arc::as_ptr(&blocks::blocks().get("RED_MUSHROOM"))) && !std::ptr::addr_eq(std::sync::Arc::as_ptr(&block), std::sync::Arc::as_ptr(&blocks::blocks().get("BROWN_MUSHROOM"))) {
            return;
        }
        let mut mushroom: compat::Shared<GlowingMushroom> = GLOWING_MUSHROOMS.get_mut().entry(pos.clone()).or_insert_with_key(|k: &BlockPos| GlowingMushroom::new(k.clone())).clone();
        GlowingMushroom::add_particle(&mushroom);
    }

    /// java: private static void update()
    pub fn update() {
        if !(GlowingMushrooms::should_process()) {
            return;
        }
        for glowing_mushroom in GLOWING_MUSHROOMS.get_mut().values().cloned() {
            GlowingMushroom::update_waypoint(&glowing_mushroom);
        }
    }

    /// java: private static void extractRendering(PrimitiveCollector collector)
    pub fn extract_rendering(collector: &compat::Shared<dyn PrimitiveCollector>) {
        if !(GlowingMushrooms::should_process()) || mcrust::mcrust::modding::game::level().is_none() {
            return;
        }
        for glowing_mushroom in GLOWING_MUSHROOMS.get_mut().values().cloned() {
            if !({ let __f = compat::dispatch(&glowing_mushroom, |__t| __t.should_render()); __f(&compat::view(&glowing_mushroom, |a| a as compat::Shared<dyn Waypoint>)) }) {
                continue;
            }
            let mut block: Arc<Block> = (mcrust::mcrust::modding::game::level().unwrap().get_block_state_typed(&compat::read(&glowing_mushroom, |__t| __t.pos.clone()).unwrap())).get_block();
            if !std::ptr::addr_eq(std::sync::Arc::as_ptr(&block), std::sync::Arc::as_ptr(&blocks::blocks().get("RED_MUSHROOM"))) && !std::ptr::addr_eq(std::sync::Arc::as_ptr(&block), std::sync::Arc::as_ptr(&blocks::blocks().get("BROWN_MUSHROOM"))) {
                continue;
            }
            let mut bounding_box: AABB = RenderHelper::get_block_bounding_box(&mcrust::mcrust::modding::game::level().unwrap(), &compat::read(&glowing_mushroom, |__t| __t.pos.clone()).unwrap()).unwrap();
            ({ let __f = compat::dispatch(collector, |__t| __t.submit_outlined_box_aabb_float_float_boolean()); __f(collector, &bounding_box, &ColorUtils::get_float_components_dye_color(&DyeColor::Yellow), ((3) as f32) as f32, ({ let __f = compat::dispatch(&glowing_mushroom, |__t| __t.should_render_through_walls()); __f(&compat::view(&glowing_mushroom, |a| a as compat::Shared<dyn Waypoint>)) })) });
        }
    }

    /// java: private static boolean shouldProcess()
    pub fn should_process() -> bool {
        return (SkyblockerConfigManager::get().other_locations.barn.enable_glowing_mushroom_helper && Utils::is_in_farm() && compat::read(&Utils::get_area(), |__t| compat::DynAny::dyn_any(__t).downcast_ref::<TheFarmingIslands>().is_some_and(|__e| *__e == TheFarmingIslands::GlowingMushroomCave)));
    }

    /// java: private static void reset()
    pub fn reset() {
        GLOWING_MUSHROOMS.get_mut().clear();
    }

}

/// java: public static class GlowingMushroom extends SeenWaypoint
#[derive(Clone)]
pub struct GlowingMushroom {
    /// java: extends SeenWaypoint
    pub base: SeenWaypointBase,
    /// java: private int particleCount = 0;
    pub particle_count: i32,
    /// java: private long lastConfirmed;
    pub last_confirmed: i64,
}

impl std::ops::Deref for GlowingMushroom {
    type Target = SeenWaypointBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for GlowingMushroom {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl GlowingMushroom {
    /// java: private GlowingMushroom(BlockPos pos)
    pub fn new(mut pos: BlockPos) -> compat::Shared<Self> {
        let (_, __this) = SeenWaypointBase::new2_in(pos.clone(), Arc::new(move || SkyblockerConfigManager::get().ui_and_visuals.waypoints.waypoint_type), ColorUtils::get_float_components_dye_color(&DyeColor::Cyan), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                particle_count: 0,
                last_confirmed: 0,
            });
            ((__h.clone() as compat::Shared<dyn SeenWaypoint>), __h)
        });
        __this
    }

    /// java: private void updateWaypoint()
    pub fn update_waypoint(__this: &compat::Shared<Self>) {
        <dyn SeenWaypoint>::tick_body(&compat::view(__this, |a| a as compat::Shared<dyn SeenWaypoint>), compat::minecraft());
        if compat::read(__this, |__t| __t.particle_count) < 1 {
            return;
        }
        let mut now: i64 = compat::millis();
        if (compat::read(__this, |__t| __t.last_confirmed)).wrapping_add(2000 as i64) > now {
            return;
        }
        { let __v = now; compat::write(__this, |__t| __t.last_confirmed = __v); }
        { let __v = 0; compat::write(__this, |__t| __t.particle_count = __v); }
    }

    /// java: public void addParticle()
    pub fn add_particle(__this: &compat::Shared<Self>) {
        { let __v = compat::read(__this, |__t| __t.particle_count) + 1; compat::write(__this, |__t| __t.particle_count = __v); }
    }

}

impl SeenWaypoint for GlowingMushroom {
    fn seen_waypoint_base(&self) -> &SeenWaypointBase {
        &self.base
    }
    fn seen_waypoint_base_mut(&mut self) -> &mut SeenWaypointBase {
        &mut self.base
    }

}

impl Waypoint for GlowingMushroom {
    fn waypoint_base(&self) -> &WaypointBase {
        &self.base.base
    }
    fn waypoint_base_mut(&mut self) -> &mut WaypointBase {
        &mut self.base.base
    }

    /// java: @Override public boolean shouldRender() (dispatch: the body for this class)
    fn should_render(&self) -> fn(&compat::Shared<dyn Waypoint>) -> bool {
        GlowingMushroom::should_render_dyn
    }

    /// java: @Override public Type getRenderType() (dispatch: the body for this class)
    fn get_render_type(&self) -> fn(&compat::Shared<dyn Waypoint>) -> Type {
        <dyn SeenWaypoint>::get_render_type_from::<Self>
    }

}

impl Renderable for GlowingMushroom {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void extractRendering(PrimitiveCollector collector) (dispatch: the body for this class)
    fn extract_rendering(&self) -> fn(&compat::Shared<dyn Renderable>, &compat::Shared<dyn PrimitiveCollector>) {
        <dyn Waypoint>::extract_rendering_from::<Self>
    }

}

impl Tickable for GlowingMushroom {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void tick(Minecraft client) (dispatch: the body for this class)
    fn tick(&self) -> fn(&compat::Shared<dyn Tickable>, &mut mcrust::net::minecraft::client::minecraft::Minecraft) {
        <dyn SeenWaypoint>::tick_from::<Self>
    }

}

impl GlowingMushroom {
    /// java: @Override public boolean shouldRender()
    pub fn should_render_impl(__this: &compat::Shared<Self>) -> bool {
        return (<dyn SeenWaypoint>::should_render_body(&compat::view(__this, |a| a as compat::Shared<dyn SeenWaypoint>)) && (compat::read(__this, |__t| __t.last_confirmed)).wrapping_add(5000 as i64) > compat::millis());
    }

    /// GlowingMushroom.shouldRender as the dispatch of Waypoint.shouldRender
    pub fn should_render_dyn(__this: &compat::Shared<dyn Waypoint>) -> bool {
        Self::should_render_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}

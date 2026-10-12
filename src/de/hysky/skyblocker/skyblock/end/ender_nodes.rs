//! Ported from de.hysky.skyblocker.skyblock.end.EnderNodes by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::events::particle_events;
use crate::de::hysky::skyblocker::utils::color_utils::ColorUtils;
use crate::de::hysky::skyblocker::utils::render::level_render_extraction_callback;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_collector::PrimitiveCollector;
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
use mcrust::net::minecraft::core::direction::Direction;
use mcrust::net::minecraft::core::particles::particle_type::ParticleType;
use mcrust::net::minecraft::network::protocol::game::clientbound_level_particles_packet::ClientboundLevelParticlesPacket;
use mcrust::net::minecraft::util::mth;
use mcrust::net::minecraft::world::item::dye_color::DyeColor;
use mcrust::net::minecraft::core::particles::particle_options::ParticleOptions as _;

/// java: private static final Map<BlockPos, EnderNode> enderNodes = new HashMap<>();
pub static ENDER_NODES: compat::MainThread<std::collections::HashMap<BlockPos, compat::Shared<EnderNode>>> = compat::MainThread::new(|| std::collections::HashMap::new());

/// java: public class EnderNodes
#[derive(Clone)]
pub struct EnderNodes {}

impl EnderNodes {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        ({ let __r = scheduler::INSTANCE.get_mut(); let __f = compat::dispatch(__r, |__t| __t.schedule_cyclic()); __f(__r, Arc::new(move || { EnderNodes::update(); }), 20) });
        level_render_extraction_callback::EVENT.register(Arc::new(move |__a0| { EnderNodes::extract_rendering(&__a0); }));
        // TODO(fabric2seer): AttackBlockCallback.EVENT.register: no SeerClient equivalent yet
        //   java: AttackBlockCallback.EVENT.register((_, _, _, pos, _) -> {
        //   java: 			enderNodes.remove(pos);
        //   java: 			return InteractionResult.PASS;
        //   java: 		});
        client_play_connection_events::JOIN.register(Arc::new(|listener: &mut ClientPacketListener| {
            EnderNodes::reset();
        }));
        particle_events::FROM_SERVER.register(Arc::new(move |__a0| { EnderNodes::on_particle(&__a0); }));
    }

    /// java: private static void onParticle(ClientboundLevelParticlesPacket packet)
    pub fn on_particle(packet: &ClientboundLevelParticlesPacket) {
        if !(EnderNodes::should_process()) {
            return;
        }
        // java: ParticleType<?> particleType = packet.getParticle().getType(); (the port erases getType(): compared through Object.equals)
        let Some(particle) = packet.get_particle() else { return; };
        let is_portal: bool = compat::particle_type_is(particle, mcrust::net::minecraft::core::particles::particle_types::portal());
        let is_witch: bool = compat::particle_type_is(particle, mcrust::net::minecraft::core::particles::particle_types::witch());
        if !is_portal && !is_witch {
            return;
        }
        let mut x: f64 = packet.get_x();
        let mut y: f64 = packet.get_y();
        let mut z: f64 = packet.get_z();
        let mut x_frac: f64 = mth::positive_modulo_f64(x, (1) as f64);
        let mut y_frac: f64 = mth::positive_modulo_f64(y, (1) as f64);
        let mut z_frac: f64 = mth::positive_modulo_f64(z, (1) as f64);
        let mut pos: BlockPos;
        let mut direction: Direction;
        if y_frac == 0.25 {
            pos = BlockPos::containing(x, (y - (1 as f64)), z);
            direction = Direction::Up;
        } else if y_frac == 0.75 {
            pos = BlockPos::containing(x, (y + (1 as f64)), z);
            direction = Direction::Down;
        } else if x_frac == 0.25 {
            pos = BlockPos::containing((x - (1 as f64)), y, z);
            direction = Direction::East;
        } else if x_frac == 0.75 {
            pos = BlockPos::containing((x + (1 as f64)), y, z);
            direction = Direction::West;
        } else if z_frac == 0.25 {
            pos = BlockPos::containing(x, y, (z - (1 as f64)));
            direction = Direction::South;
        } else if z_frac == 0.75 {
            pos = BlockPos::containing(x, y, (z + (1 as f64)));
            direction = Direction::North;
        } else {
            return;
        }
        let mut ender_node: compat::Shared<EnderNode> = ENDER_NODES.get_mut().entry(pos.clone()).or_insert_with_key(|k: &BlockPos| EnderNode::new(k.clone())).clone();
        // java: IntIntPair particles = enderNode.particles.get(direction); particles.left(particles.leftInt() + 1) / right(..)
        if is_portal {
            compat::write(&ender_node, |__t| __t.particles.get_mut(&direction).unwrap().0 += 1);
        } else if is_witch {
            compat::write(&ender_node, |__t| __t.particles.get_mut(&direction).unwrap().1 += 1);
        }
    }

    /// java: private static void update()
    pub fn update() {
        if EnderNodes::should_process() {
            for ender_node in ENDER_NODES.get_mut().values().cloned() {
                EnderNode::update_waypoint(&ender_node);
            }
        }
    }

    /// java: private static void extractRendering(PrimitiveCollector collector)
    pub fn extract_rendering(collector: &compat::Shared<dyn PrimitiveCollector>) {
        if EnderNodes::should_process() {
            for ender_node in ENDER_NODES.get_mut().values().cloned() {
                if { let __f = compat::dispatch(&ender_node, |__t| __t.should_render()); __f(&compat::view(&ender_node, |a| a as compat::Shared<dyn Waypoint>)) } {
                    ({ let __f = compat::dispatch(&ender_node, |__t| __t.extract_rendering()); __f(&compat::view(&ender_node, |a| a as compat::Shared<dyn Renderable>), collector) });
                }
            }
        }
    }

    /// java: private static boolean shouldProcess()
    pub fn should_process() -> bool {
        return (SkyblockerConfigManager::get().other_locations.end.enable_ender_node_helper && Utils::is_in_the_end());
    }

    /// java: private static void reset()
    pub fn reset() {
        ENDER_NODES.get_mut().clear();
    }

}

/// java: public static class EnderNode extends SeenWaypoint
#[derive(Clone)]
pub struct EnderNode {
    /// java: extends SeenWaypoint
    pub base: SeenWaypointBase,
    /// java: private final Map<Direction, IntIntPair> particles = Map.of( Direction.UP, new IntIntMutablePair(0, 0), Direction.DOWN, new IntIntMutablePair(0, 0), Direction.E
    pub particles: std::collections::HashMap<Direction, (i32, i32)>,
    /// java: private long lastConfirmed;
    pub last_confirmed: i64,
}

impl std::ops::Deref for EnderNode {
    type Target = SeenWaypointBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for EnderNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl EnderNode {
    /// java: private EnderNode(BlockPos pos)
    pub fn new(mut pos: BlockPos) -> compat::Shared<Self> {
        let (_, __this) = SeenWaypointBase::new2_in(pos.clone(), Arc::new(move || SkyblockerConfigManager::get().other_locations.end.ender_node_waypoint_type), ColorUtils::get_float_components_dye_color(&DyeColor::Cyan), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                particles: std::collections::HashMap::from([(Direction::Up, (0, 0)), (Direction::Down, (0, 0)), (Direction::East, (0, 0)), (Direction::West, (0, 0)), (Direction::South, (0, 0)), (Direction::North, (0, 0))]),
                last_confirmed: 0,
            });
            ((__h.clone() as compat::Shared<dyn SeenWaypoint>), __h)
        });
        __this
    }

    /// java: private void updateWaypoint()
    pub fn update_waypoint(__this: &compat::Shared<Self>) {
        ({ let __f = compat::dispatch(__this, |__t| __t.tick()); __f(&compat::view(__this, |a| a as compat::Shared<dyn Tickable>), compat::minecraft()) });
        let mut current_time_millis: i64 = compat::millis();
        let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        if (compat::read(__this, |__t| __t.last_confirmed)).wrapping_add(2000 as i64) > current_time_millis || mcrust::mcrust::modding::game::level().is_none() || !(__this.lock().borrow().particles.iter().map(|(k, v)| (k.clone(), v.clone())).all(|entry| { let Some(__this_1) = __this_w1.get().upgrade() else { return Default::default(); }; (entry.1.0 >= 5 && entry.1.1 >= 5 || !(mcrust::mcrust::modding::game::level().unwrap().get_block_state_typed(&(&__this_1).lock().borrow().pos.as_ref().unwrap().relative(entry.0)).is_air())) })) {
            return;
        }
        { let __v = current_time_millis; compat::write(__this, |__t| __t.last_confirmed = __v); }
        // java: entry.getValue().left(0); entry.getValue().right(0);
        compat::write(__this, |__t| {
            for value in __t.particles.values_mut() {
                value.0 = 0;
                value.1 = 0;
            }
        });
    }

}

impl SeenWaypoint for EnderNode {
    fn seen_waypoint_base(&self) -> &SeenWaypointBase {
        &self.base
    }
    fn seen_waypoint_base_mut(&mut self) -> &mut SeenWaypointBase {
        &mut self.base
    }

}

impl Waypoint for EnderNode {
    fn waypoint_base(&self) -> &WaypointBase {
        &self.base.base
    }
    fn waypoint_base_mut(&mut self) -> &mut WaypointBase {
        &mut self.base.base
    }

    /// java: @Override public boolean shouldRender() (dispatch: the body for this class)
    fn should_render(&self) -> fn(&compat::Shared<dyn Waypoint>) -> bool {
        EnderNode::should_render_dyn
    }

    /// java: @Override public Type getRenderType() (dispatch: the body for this class)
    fn get_render_type(&self) -> fn(&compat::Shared<dyn Waypoint>) -> Type {
        <dyn SeenWaypoint>::get_render_type_from::<Self>
    }

}

impl Renderable for EnderNode {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void extractRendering(PrimitiveCollector collector) (dispatch: the body for this class)
    fn extract_rendering(&self) -> fn(&compat::Shared<dyn Renderable>, &compat::Shared<dyn PrimitiveCollector>) {
        <dyn Waypoint>::extract_rendering_from::<Self>
    }

}

impl Tickable for EnderNode {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void tick(Minecraft client) (dispatch: the body for this class)
    fn tick(&self) -> fn(&compat::Shared<dyn Tickable>, &mut mcrust::net::minecraft::client::minecraft::Minecraft) {
        <dyn SeenWaypoint>::tick_from::<Self>
    }

}

impl EnderNode {
    /// java: @Override public boolean shouldRender()
    pub fn should_render_impl(__this: &compat::Shared<Self>) -> bool {
        return (<dyn SeenWaypoint>::should_render_body(&compat::view(__this, |a| a as compat::Shared<dyn SeenWaypoint>)) && (compat::read(__this, |__t| __t.last_confirmed)).wrapping_add(5000 as i64) > compat::millis());
    }

    /// EnderNode.shouldRender as the dispatch of Waypoint.shouldRender
    pub fn should_render_dyn(__this: &compat::Shared<dyn Waypoint>) -> bool {
        Self::should_render_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}

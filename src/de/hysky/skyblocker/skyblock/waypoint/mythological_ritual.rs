//! Ported from de.hysky.skyblocker.skyblock.waypoint.MythologicalRitual by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::events::particle_events;
use crate::de::hysky::skyblocker::utils::color_utils::ColorUtils;
use crate::de::hysky::skyblocker::utils::location::Location;
use crate::de::hysky::skyblocker::utils::render::level_render_extraction_callback;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_collector::PrimitiveCollector;
use crate::de::hysky::skyblocker::utils::render::renderable::Renderable;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use crate::de::hysky::skyblocker::utils::waypoint::waypoint::Type;
use crate::de::hysky::skyblocker::utils::waypoint::waypoint::Waypoint;
use crate::de::hysky::skyblocker::utils::waypoint::waypoint::WaypointBase;
use mcrust::net::minecraft::core::block_pos::BlockPos;
use mcrust::net::minecraft::core::direction::Direction;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::protocol::game::clientbound_level_particles_packet::ClientboundLevelParticlesPacket;
use mcrust::net::minecraft::world::entity::player::player::Player;
use mcrust::net::minecraft::world::interaction_hand::InteractionHand;
use mcrust::net::minecraft::world::interaction_result::InteractionResult;
use mcrust::net::minecraft::world::item::dye_color::DyeColor;
use mcrust::net::minecraft::world::level::block::blocks;
use mcrust::net::minecraft::world::level::level::Level;
use mcrust::net::minecraft::world::level::levelgen::heightmap::Heightmap;
use mcrust::net::minecraft::world::phys::block_hit_result::BlockHitResult;
use mcrust::net::minecraft::world::phys::vec3::Vec3;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;
use mcrust::net::minecraft::core::particles::particle_options::ParticleOptions as _;
use mcrust::net::minecraft::network::chat::component::Component as _;
use mcrust::net::minecraft::world::entity::living_entity::LivingEntity as _;

/// java: private static final Pattern GRIFFIN_BURROW_DUG = Pattern.compile("(?<message>You dug out a Griffin Burrow!|You finished the Griffin burrow chain!) \\((?<index>\\d+)/(?<length>\\d+)\\)");
pub static GRIFFIN_BURROW_DUG: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("(?<message>You dug out a Griffin Burrow!|You finished the Griffin burrow chain!) \\((?<index>\\d+)/(?<length>\\d+)\\)"));
/// java: private static final float[] ORANGE_COLOR_COMPONENTS = ColorUtils.getFloatComponents(DyeColor.ORANGE);
pub static ORANGE_COLOR_COMPONENTS: LazyLock<Vec<f32>> = LazyLock::new(|| ColorUtils::get_float_components_dye_color(&DyeColor::Orange));
/// java: private static final float[] RED_COLOR_COMPONENTS = ColorUtils.getFloatComponents(DyeColor.RED);
pub static RED_COLOR_COMPONENTS: LazyLock<Vec<f32>> = LazyLock::new(|| ColorUtils::get_float_components_dye_color(&DyeColor::Red));
/// java: private static final Set<String> SPADES = Set.of("ANCESTRAL_SPADE", "ARCHAIC_SPADE", "DEIFIC_SPADE");
pub static SPADES: LazyLock<std::collections::HashSet<String>> = LazyLock::new(|| std::collections::HashSet::from(["ANCESTRAL_SPADE".to_string(), "ARCHAIC_SPADE".to_string(), "DEIFIC_SPADE".to_string()]));
/// java: private static long lastEchoTime;
pub static LAST_ECHO_TIME: LazyLock<Mutex<i64>> = LazyLock::new(|| Mutex::new(0));
/// java: private static final Map<BlockPos, GriffinBurrow> griffinBurrows = new HashMap<>();
pub static GRIFFIN_BURROWS: compat::MainThread<std::collections::HashMap<BlockPos, compat::Shared<GriffinBurrow>>> = compat::MainThread::new(|| std::collections::HashMap::new());
/// java: private static @Nullable BlockPos lastDugBurrowPos;
pub static LAST_DUG_BURROW_POS: compat::MainThread<Option<BlockPos>> = compat::MainThread::new(|| None);
/// java: private static GriffinBurrow previousBurrow = new GriffinBurrow(BlockPos.ZERO);
pub static PREVIOUS_BURROW: compat::MainThread<compat::Shared<GriffinBurrow>> = compat::MainThread::new(|| GriffinBurrow::new(mcrust::net::minecraft::core::block_pos::ZERO));

/// java: public class MythologicalRitual
#[derive(Clone)]
pub struct MythologicalRitual {}

impl MythologicalRitual {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        level_render_extraction_callback::EVENT.register(Arc::new(move |__a0| { MythologicalRitual::extract_rendering(&__a0); }));
        // TODO(fabric2seer): AttackBlockCallback.EVENT.register: no SeerClient equivalent yet
        //   java: AttackBlockCallback.EVENT.register(MythologicalRitual::onAttackBlock);
        // TODO(fabric2seer): UseBlockCallback.EVENT.register: no SeerClient equivalent yet
        //   java: UseBlockCallback.EVENT.register(MythologicalRitual::onUseBlock);
        // TODO(fabric2seer): UseItemCallback.EVENT.register: no SeerClient equivalent yet
        //   java: UseItemCallback.EVENT.register(MythologicalRitual::onUseItem);
        // TODO(fabric2seer): ClientReceiveMessageEvents.ALLOW_GAME.register: no SeerClient equivalent yet
        //   java: ClientReceiveMessageEvents.ALLOW_GAME.register(MythologicalRitual::onChatMessage);
        client_play_connection_events::JOIN.register(Arc::new(|listener: &mut ClientPacketListener| {
            MythologicalRitual::reset();
        }));
        client_command_registration_callback::EVENT.register(Arc::new(
            |dispatcher: &mut client_command_registration_callback::ClientCommandDispatcher| {
                // TODO(fabric2seer): a command tree this tool cannot read (arguments, redirects, requires) - SeerClient commands receive the words after the name in `ctx.args`
                //   java: dispatcher.register(..);
            },
        ));
        particle_events::FROM_SERVER.register(Arc::new(move |__a0| { MythologicalRitual::on_particle(&__a0); }));
        // TODO(fabric2seer): static field TriState.DEFAULT
        //   java: previousBurrow.confirmed = TriState.DEFAULT;
        GRIFFIN_BURROWS.get_mut().insert(mcrust::net::minecraft::core::block_pos::ZERO, PREVIOUS_BURROW.get_mut().clone());
    }

    /// java: private static void onParticle(ClientboundLevelParticlesPacket packet)
    pub fn on_particle(packet: &ClientboundLevelParticlesPacket) {
        if MythologicalRitual::is_active() {
            if true && (mcrust::net::minecraft::core::particles::particle_types::crit() == packet.get_particle().unwrap().get_type() || mcrust::net::minecraft::core::particles::particle_types::enchant() == packet.get_particle().unwrap().get_type()) {
                MythologicalRitual::handle_burrow_particle(packet);
            } else if true && mcrust::net::minecraft::core::particles::particle_types::dust() == packet.get_particle().unwrap().get_type() {
                MythologicalRitual::handle_next_burrow_particle(packet);
            } else if true && mcrust::net::minecraft::core::particles::particle_types::dripping_lava() == packet.get_particle().unwrap().get_type() && packet.get_count() == 2 {
                MythologicalRitual::handle_echo_burrow_particle(packet);
            } else {
            }
        }
    }

    /// java: private static void handleBurrowParticle(ClientboundLevelParticlesPacket packet)
    pub fn handle_burrow_particle(packet: &ClientboundLevelParticlesPacket) {
        let mut pos: BlockPos = (BlockPos::containing(packet.get_x(), packet.get_y(), packet.get_z())).below();
        if mcrust::mcrust::modding::game::level().is_none() || !(mcrust::mcrust::modding::game::level().unwrap().get_block_state_typed(&pos).is_block(&blocks::blocks().get("grass_block"))) {
            return;
        }
        let mut burrow: compat::Shared<GriffinBurrow> = GRIFFIN_BURROWS.get_mut().entry(pos.clone()).or_insert_with_key(|k| (|__a0| GriffinBurrow::new(__a0.clone()))(k.clone())).clone();
        if mcrust::net::minecraft::core::particles::particle_types::crit() == packet.get_particle().unwrap().get_type() {
            { let __v = compat::read(&burrow, |__t| __t.crit_particle) + 1; compat::write(&burrow, |__t| __t.crit_particle = __v); }
        }
        if mcrust::net::minecraft::core::particles::particle_types::enchant() == packet.get_particle().unwrap().get_type() {
            { let __v = compat::read(&burrow, |__t| __t.enchant_particle) + 1; compat::write(&burrow, |__t| __t.enchant_particle = __v); }
        }
        // TODO(fabric2seer): the condition: static field TriState.FALSE
        //   java: burrow.critParticle >= 5 && burrow.enchantParticle >= 5 && burrow.confirmed == TriState.FALSE
        if todo!("fabric2seer: the condition (TODO above)") {
            GriffinBurrow::init(&GRIFFIN_BURROWS.get_mut().get(&pos).cloned().unwrap());
        }
    }

    /// java: private static void handleNextBurrowParticle(ClientboundLevelParticlesPacket packet)
    pub fn handle_next_burrow_particle(packet: &ClientboundLevelParticlesPacket) {
        let mut pos: BlockPos = BlockPos::containing(packet.get_x(), packet.get_y(), packet.get_z());
        let mut burrow_pos: BlockPos;
        if mcrust::mcrust::modding::game::level().is_some() {
            // TODO(fabric2seer): field `WORLD_SURFACE` of a value of unknown type
            //   java: burrowPos = Minecraft.getInstance().level.getHeightmapPos(Heightmap.Types.WORLD_SURFACE, pos);
        } else {
            burrow_pos = pos.below_n(2);
        }
        let Some(mut burrow) = GRIFFIN_BURROWS.get_mut().get(&burrow_pos).cloned() else {
            return;
        };
        // TODO(fabric2seer): `.addData(..)` on a SimpleRegression
        //   java: burrow.regression.addData(packet.getX(), packet.getZ());
        // TODO(fabric2seer): `.getSlope(..)` on a SimpleRegression
        //   java: double slope = burrow.regression.getSlope();
        let mut slope: f64 = todo!("fabric2seer: the initialiser of `slope` (TODO above)");
        if slope.is_nan() {
            return;
        }
        let mut next_burrow_direction: Vec3 = (Vec3::new((100) as f64, (0) as f64, (slope * (100 as f64)))).normalize();
        // TODO(fabric2seer): `new Vector2D(..)`
        //   java: Vector2D pos2D = new Vector2D(pos.getX() + 0.5, pos.getZ() + 0.5);
        let mut pos2_d: crate::unresolved::Vector2D = todo!("fabric2seer: the initialiser of `pos2D` (TODO above)");
        // TODO(fabric2seer): `new Line(..)`
        //   java: burrow.nextBurrowLineEstimation = new Line(pos2D, pos2D.add(new Vector2D(nextBurrowDirection.x, nextBurrowDirection.z)), 0.0001);
        MythologicalRitual::estimate_next_burrow(&burrow);
        // TODO(fabric2seer): the condition: `nextBurrowLine` is not a field of GriffinBurrow
        //   java: burrow.nextBurrowLine == null
        if todo!("fabric2seer: the condition (TODO above)") {
            // TODO(fabric2seer): `nextBurrowLine` is not a field of GriffinBurrow
            //   java: burrow.nextBurrowLine = new Vec3[1001];
        }
        // TODO(fabric2seer): `nextBurrowLine` is not a field of GriffinBurrow
        //   java: fillLine(burrow.nextBurrowLine, Vec3.atCenterOf(pos.above()), nextBurrowDirection);
    }

    /// java: private static void handleEchoBurrowParticle(ClientboundLevelParticlesPacket packet)
    pub fn handle_echo_burrow_particle(packet: &ClientboundLevelParticlesPacket) {
        if compat::millis() > (compat::get_state(&LAST_ECHO_TIME)).wrapping_add(10000 as i64) {
            return;
        }
        if mcrust::mcrust::modding::game::level().is_some() && !((mcrust::mcrust::modding::game::level().unwrap().get_block_state_typed(&BlockPos::containing(packet.get_x(), (packet.get_y() - 0.25), packet.get_z()))).is_air()) {
            return;
        }
        // TODO(fabric2seer): the condition: `echoBurrowDirection` is not a field of GriffinBurrow
        //   java: previousBurrow.echoBurrowDirection == null
        if todo!("fabric2seer: the condition (TODO above)") {
            // TODO(fabric2seer): `echoBurrowDirection` is not a field of GriffinBurrow
            //   java: previousBurrow.echoBurrowDirection = new Vec3[2];
        }
        // TODO(fabric2seer): `echoBurrowDirection` is not a field of GriffinBurrow
        //   java: previousBurrow.echoBurrowDirection[0] = previousBurrow.echoBurrowDirection[1];
        // TODO(fabric2seer): `echoBurrowDirection` is not a field of GriffinBurrow
        //   java: previousBurrow.echoBurrowDirection[1] = new Vec3(packet.getX(), packet.getY(), packet.getZ());
        // TODO(fabric2seer): the condition: `echoBurrowDirection` is not a field of GriffinBurrow
        //   java: previousBurrow.echoBurrowDirection[0] == null || previousBurrow.echoBurrowDirection[1] == null
        if todo!("fabric2seer: the condition (TODO above)") {
            return;
        }
        // TODO(fabric2seer): `new Vector2D(..)`
        //   java: Vector2D pos1 = new Vector2D(previousBurrow.echoBurrowDirection[0].x, previousBurrow.echoBurrowDirection[0].z);
        let mut pos1: crate::unresolved::Vector2D = todo!("fabric2seer: the initialiser of `pos1` (TODO above)");
        // TODO(fabric2seer): `new Vector2D(..)`
        //   java: Vector2D pos2 = new Vector2D(previousBurrow.echoBurrowDirection[1].x, previousBurrow.echoBurrowDirection[1].z);
        let mut pos2: crate::unresolved::Vector2D = todo!("fabric2seer: the initialiser of `pos2` (TODO above)");
        // TODO(fabric2seer): `new Line(..)`
        //   java: previousBurrow.echoBurrowLineEstimation = new Line(pos1, pos2, 0.0001);
        MythologicalRitual::estimate_next_burrow(PREVIOUS_BURROW.get_mut());
        // TODO(fabric2seer): `echoBurrowDirection` is not a field of GriffinBurrow
        //   java: Vec3 echoBurrowDirection = previousBurrow.echoBurrowDirection[1].subtract(previousBurrow.echoBurrowDirection[0]).normalize();
        let mut echo_burrow_direction: Vec3 = todo!("fabric2seer: the initialiser of `echoBurrowDirection` (TODO above)");
        // TODO(fabric2seer): the condition: `echoBurrowLine` is not a field of GriffinBurrow
        //   java: previousBurrow.echoBurrowLine == null
        if todo!("fabric2seer: the condition (TODO above)") {
            // TODO(fabric2seer): `echoBurrowLine` is not a field of GriffinBurrow
            //   java: previousBurrow.echoBurrowLine = new Vec3[1001];
        }
        // TODO(fabric2seer): `echoBurrowLine` is not a field of GriffinBurrow
        //   java: fillLine(previousBurrow.echoBurrowLine, previousBurrow.echoBurrowDirection[0], echoBurrowDirection);
    }

    /// java: private static void estimateNextBurrow(GriffinBurrow burrow)
    pub fn estimate_next_burrow(burrow: &compat::Shared<GriffinBurrow>) {
        if compat::read(burrow, |__t| __t.next_burrow_line_estimation.clone()).is_none() || compat::read(burrow, |__t| __t.echo_burrow_line_estimation.clone()).is_none() {
            return;
        }
        // TODO(fabric2seer): `.intersection(..)` on a Line
        //   java: Vector2D intersection = burrow.nextBurrowLineEstimation.intersection(burrow.echoBurrowLineEstimation);
        let mut intersection: crate::unresolved::Vector2D = todo!("fabric2seer: the initialiser of `intersection` (TODO above)");
        if false /* java: intersection == null: never null here */ {
            return;
        }
        // TODO(fabric2seer): `.getX(..)` on a Vector2D
        //   java: burrow.nextBurrowEstimatedPos = BlockPos.containing(intersection.getX(), 5, intersection.getY());
    }

    /// java: static void fillLine(Vec3[] line, Vec3 start, Vec3 direction)
    pub fn fill_line(line: &mut Vec<Vec3>, mut start: Vec3, mut direction: Vec3) {
        debug_assert!((line.len() as i32).wrapping_rem(2) == 1);
        let mut middle: i32 = (line.len() as i32).wrapping_div(2);
        line[(middle) as usize] = start.clone();
        for i in 0..middle {
            line[((middle.wrapping_add(1)).wrapping_add(i)) as usize] = (line[(middle.wrapping_add(i)) as usize]).add_vec(&direction);
            line[((middle.wrapping_sub(1)).wrapping_sub(i)) as usize] = (line[(middle.wrapping_sub(i)) as usize]).subtract_vec(&direction);
        }
    }

    /// java: public static void extractRendering(PrimitiveCollector collector)
    pub fn extract_rendering(collector: &compat::Shared<dyn PrimitiveCollector>) {
        if MythologicalRitual::is_active() {
            for burrow in GRIFFIN_BURROWS.get_mut().values().cloned() {
                if { let __f = compat::dispatch(&burrow, |__t| __t.should_render()); __f(&compat::view(&burrow, |a| a as compat::Shared<dyn Waypoint>)) } {
                    ({ let __f = compat::dispatch(&burrow, |__t| __t.extract_rendering()); __f(&compat::view(&burrow, |a| a as compat::Shared<dyn Renderable>), collector) });
                }
                // TODO(fabric2seer): the condition: static field TriState.FALSE
                //   java: burrow.confirmed != TriState.FALSE
                if todo!("fabric2seer: the condition (TODO above)") {
                    // TODO(fabric2seer): the condition: `nextBurrowLine` is not a field of GriffinBurrow
                    //   java: burrow.nextBurrowLine != null
                    if todo!("fabric2seer: the condition (TODO above)") {
                        // TODO(fabric2seer): `nextBurrowLine` is not a field of GriffinBurrow
                        //   java: collector.submitLinesFromPoints(burrow.nextBurrowLine, ORANGE_COLOR_COMPONENTS, 0.5f, 5f, false);
                    }
                    // TODO(fabric2seer): the condition: `echoBurrowLine` is not a field of GriffinBurrow
                    //   java: burrow.echoBurrowLine != null
                    if todo!("fabric2seer: the condition (TODO above)") {
                        // TODO(fabric2seer): `echoBurrowLine` is not a field of GriffinBurrow
                        //   java: collector.submitLinesFromPoints(burrow.echoBurrowLine, ORANGE_COLOR_COMPONENTS, 0.5f, 5f, false);
                    }
                    // TODO(fabric2seer): the condition: static field TriState.DEFAULT
                    //   java: burrow.nextBurrowEstimatedPos != null && burrow.confirmed == TriState.DEFAULT
                    if todo!("fabric2seer: the condition (TODO above)") {
                        ({ let __f = compat::dispatch(collector, |__t| __t.submit_filled_box_with_beacon_beam()); __f(collector, &compat::read(&burrow, |__t| __t.next_burrow_estimated_pos.clone()).unwrap(), &RED_COLOR_COMPONENTS, 0.5f32, true) });
                    }
                }
            }
        }
    }

    /// java: public static InteractionResult onAttackBlock(Player player, Level world, InteractionHand hand, BlockPos pos, Direction direction)
    pub fn on_attack_block(player: &dyn Player, world: &Level, hand: &InteractionHand, pos: &BlockPos, direction: &Direction) -> InteractionResult {
        return MythologicalRitual::on_interact_block(pos.clone());
    }

    /// java: public static InteractionResult onUseBlock(Player player, Level world, InteractionHand hand, BlockHitResult hitResult)
    pub fn on_use_block(player: &dyn Player, world: &Level, hand: &InteractionHand, hit_result: &BlockHitResult) -> InteractionResult {
        return MythologicalRitual::on_interact_block(hit_result.get_block_pos());
    }

    /// java: private static InteractionResult onInteractBlock(BlockPos pos)
    pub fn on_interact_block(mut pos: BlockPos) -> InteractionResult {
        if MythologicalRitual::is_active() && GRIFFIN_BURROWS.get_mut().contains_key(&pos) {
            *LAST_DUG_BURROW_POS.get_mut() = Some(pos.clone());
        }
        return InteractionResult::Pass;
    }

    /// java: public static InteractionResult onUseItem(Player player, Level world, InteractionHand hand)
    pub fn on_use_item(player: &dyn Player, world: &Level, hand: &InteractionHand) -> InteractionResult {
        let stack = player.get_item_in_hand(*hand);
        if MythologicalRitual::is_active() && SPADES.contains(&stack.get_skyblock_id()) {
            compat::set_state(&LAST_ECHO_TIME, compat::millis());
        }
        return InteractionResult::Pass;
    }

    /// java: @SuppressWarnings("SameReturnValue") public static boolean onChatMessage(Component message, boolean overlay)
    pub fn on_chat_message(message: &MutableComponent, mut overlay: bool) -> bool {
        if MythologicalRitual::is_active() && GRIFFIN_BURROW_DUG.matcher(&message.get_string()).matches() {
            // TODO(fabric2seer): static field TriState.FALSE
            //   java: previousBurrow.confirmed = TriState.FALSE;
            if LAST_DUG_BURROW_POS.get_mut().is_none() {
                return true;
            }
            *PREVIOUS_BURROW.get_mut() = GRIFFIN_BURROWS.get_mut().get(LAST_DUG_BURROW_POS.get_mut()).cloned().unwrap();
            // TODO(fabric2seer): static field TriState.DEFAULT
            //   java: previousBurrow.confirmed = TriState.DEFAULT;
        }
        return true;
    }

    /// java: private static boolean isActive()
    pub fn is_active() -> bool {
        return (SkyblockerConfigManager::get().helpers.mythological_ritual.enable_mythological_ritual_helper && Utils::get_location() == Location::Hub);
    }

    /// java: private static void reset()
    pub fn reset() {
        GRIFFIN_BURROWS.get_mut().clear();
        *LAST_DUG_BURROW_POS.get_mut() = None;
        *PREVIOUS_BURROW.get_mut() = GriffinBurrow::new(mcrust::net::minecraft::core::block_pos::ZERO);
        // TODO(fabric2seer): static field TriState.DEFAULT
        //   java: previousBurrow.confirmed = TriState.DEFAULT;
        GRIFFIN_BURROWS.get_mut().insert(mcrust::net::minecraft::core::block_pos::ZERO, PREVIOUS_BURROW.get_mut().clone());
    }

}

/// java: private static class GriffinBurrow extends Waypoint
pub struct GriffinBurrow {
    /// java: extends Waypoint
    pub base: WaypointBase,
    /// java: private int critParticle;
    pub crit_particle: i32,
    /// java: private int enchantParticle;
    pub enchant_particle: i32,
    /// java: private TriState confirmed = TriState.FALSE;
    pub confirmed: crate::unresolved::TriState,
    /// java: private final SimpleRegression regression = new SimpleRegression();
    pub regression: crate::unresolved::SimpleRegression,
    /// java: private @Nullable BlockPos nextBurrowEstimatedPos;
    pub next_burrow_estimated_pos: Option<BlockPos>,
    /// java: private @Nullable Line nextBurrowLineEstimation;
    pub next_burrow_line_estimation: Option<crate::unresolved::Line>,
    /// java: private @Nullable Line echoBurrowLineEstimation;
    pub echo_burrow_line_estimation: Option<crate::unresolved::Line>,
}

impl std::ops::Deref for GriffinBurrow {
    type Target = WaypointBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for GriffinBurrow {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl GriffinBurrow {
    /// java: private GriffinBurrow(BlockPos pos)
    pub fn new(mut pos: BlockPos) -> compat::Shared<Self> {
        // TODO(fabric2seer): the initialiser of `confirmed`: static field TriState.FALSE
        //   java: private TriState confirmed = TriState.FALSE;
        // TODO(fabric2seer): the initialiser of `regression`: `new SimpleRegression(..)`
        //   java: private final SimpleRegression regression = new SimpleRegression();
        let (_, __this) = WaypointBase::new3_in(pos.clone(), Type::Waypoint, ORANGE_COLOR_COMPONENTS.clone(), 0.25f32, move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                crit_particle: 0,
                enchant_particle: 0,
                confirmed: Default::default(),
                regression: Default::default(),
                next_burrow_estimated_pos: None,
                next_burrow_line_estimation: None,
                echo_burrow_line_estimation: None,
            });
            ((__h.clone() as compat::Shared<dyn Waypoint>), __h)
        });
        __this
    }

    /// java: private void init()
    pub fn init(__this: &compat::Shared<Self>) {
        // TODO(fabric2seer): static field TriState.TRUE
        //   java: confirmed = TriState.TRUE;
        // TODO(fabric2seer): `.clear(..)` on a SimpleRegression
        //   java: regression.clear();
    }

}

impl Waypoint for GriffinBurrow {
    fn waypoint_base(&self) -> &WaypointBase {
        &self.base
    }
    fn waypoint_base_mut(&mut self) -> &mut WaypointBase {
        &mut self.base
    }

    /// java: @Override public boolean shouldRender() (dispatch: the body for this class)
    fn should_render(&self) -> fn(&compat::Shared<dyn Waypoint>) -> bool {
        GriffinBurrow::should_render_dyn
    }

}

impl Renderable for GriffinBurrow {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void extractRendering(PrimitiveCollector collector) (dispatch: the body for this class)
    fn extract_rendering(&self) -> fn(&compat::Shared<dyn Renderable>, &compat::Shared<dyn PrimitiveCollector>) {
        <dyn Waypoint>::extract_rendering_from::<Self>
    }

}

impl GriffinBurrow {
    /// java: @Override public boolean shouldRender()
    pub fn should_render_impl(__this: &compat::Shared<Self>) -> bool {
        // TODO(fabric2seer): static field TriState.TRUE
        //   java: return super.shouldRender() && confirmed == TriState.TRUE;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// GriffinBurrow.shouldRender as the dispatch of Waypoint.shouldRender
    pub fn should_render_dyn(__this: &compat::Shared<dyn Waypoint>) -> bool {
        Self::should_render_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}

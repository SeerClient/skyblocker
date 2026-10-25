//! Ported from de.hysky.skyblocker.skyblock.waypoint.MythologicalRitual by fabric2seer.

#[allow(unused_imports)]
use mcrust::com::mojang::brigadier::builder::argument_builder::ArgumentBuilder as _;
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
use mcrust::com::mojang::brigadier::context::command_context::CommandContext;
use crate::de::hysky::skyblocker::utils::command::argumenttypes::blockpos::client_block_pos_argument_type::ClientBlockPosArgumentType;
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
use mcrust::net::minecraft::world::phys::vec3;
use mcrust::net::minecraft::core::particles::particle_types;
use mcrust::net::minecraft::util::tri_state::TriState;
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
        // java: AttackBlockCallback.EVENT.register(MythologicalRitual::onAttackBlock);
        compat::batch_w_register_attack_block_callback(Arc::new(|player: &dyn Player, world: &Level, hand: InteractionHand, pos: &BlockPos, direction: Direction| MythologicalRitual::on_attack_block(player, world, &hand, pos, &direction)));
        // java: UseBlockCallback.EVENT.register(MythologicalRitual::onUseBlock);
        compat::batch_w_register_use_block_callback(Arc::new(|player: &dyn Player, world: &Level, hand: InteractionHand, hit_result: &BlockHitResult| MythologicalRitual::on_use_block(player, world, &hand, hit_result)));
        // java: UseItemCallback.EVENT.register(MythologicalRitual::onUseItem);
        compat::register_use_item_callback(|player: &dyn Player, world: &Level, hand: InteractionHand| MythologicalRitual::on_use_item(player, world, &hand));
        // java: ClientReceiveMessageEvents.ALLOW_GAME.register(MythologicalRitual::onChatMessage);
        compat::batch_w_allow_game_register(Arc::new(|message: &MutableComponent, overlay: bool| MythologicalRitual::on_chat_message(message, overlay)));
        client_play_connection_events::JOIN.register(Arc::new(|listener: &mut ClientPacketListener| {
            MythologicalRitual::reset();
        }));
        client_command_registration_callback::EVENT.register(Arc::new(
            |dispatcher: &mut client_command_registration_callback::ClientCommandDispatcher| {
                // java: dispatcher.register(literal(SkyblockerMod.NAMESPACE).then(literal("diana")
                //   .then(literal("clearGriffinBurrows").executes(_ -> { reset(); return Command.SINGLE_SUCCESS; }))
                //   .then(literal("clearGriffinBurrow").then(argument("position", ClientBlockPosArgumentType.blockPos()).executes(context -> {
                //     griffinBurrows.remove(context.getArgument("position", ClientPosArgument.class).toAbsoluteBlockPos(context.getSource()));
                //     return Command.SINGLE_SUCCESS; })))));
                type Source = client_command_registration_callback::FabricClientCommandSource;
                let mut clear_griffin_burrows = client_command_registration_callback::literal("clearGriffinBurrows");
                clear_griffin_burrows.executes(Arc::new(|_context: &CommandContext<Source>| {
                    MythologicalRitual::reset();
                    Ok(1)
                }));
                let mut position = client_command_registration_callback::argument("position", Arc::new(ClientBlockPosArgumentType::block_pos()));
                position.executes(Arc::new(|context: &CommandContext<Source>| {
                    // `context.getArgument("position", ClientPosArgument.class).toAbsoluteBlockPos(context.getSource())`
                    let pos: BlockPos = ClientBlockPosArgumentType::get_block_pos(context, "position");
                    GRIFFIN_BURROWS.get_mut().remove(&pos);
                    Ok(1)
                }));
                let mut clear_griffin_burrow = client_command_registration_callback::literal("clearGriffinBurrow");
                clear_griffin_burrow.then(&position);
                let mut diana = client_command_registration_callback::literal("diana");
                diana.then(&clear_griffin_burrows);
                diana.then(&clear_griffin_burrow);
                let mut root = client_command_registration_callback::literal(crate::entry::skyblocker_mod::NAMESPACE);
                root.then(&diana);
                dispatcher.dispatcher().register(&root);
            },
        ));
        particle_events::FROM_SERVER.register(Arc::new(move |__a0| { MythologicalRitual::on_particle(&__a0); }));
        // Put a root burrow so echo detection works without a previous burrow
        compat::write(PREVIOUS_BURROW.get(), |__t| __t.confirmed = TriState::Default);
        GRIFFIN_BURROWS.get_mut().insert(mcrust::net::minecraft::core::block_pos::ZERO, PREVIOUS_BURROW.get_mut().clone());
    }

    /// java: private static void onParticle(ClientboundLevelParticlesPacket packet)
    pub fn on_particle(packet: &ClientboundLevelParticlesPacket) {
        if MythologicalRitual::is_active() {
            // `switch (packet.getParticle().getType())` with `case ParticleType<?> type when ParticleTypes.X.equals(type)`
            let particle = packet.particle().expect("NullPointerException");
            if particle_type_is(particle, &**particle_types::crit()) || particle_type_is(particle, &**particle_types::enchant()) {
                MythologicalRitual::handle_burrow_particle(packet);
            } else if particle_type_is(particle, &**particle_types::dust()) {
                MythologicalRitual::handle_next_burrow_particle(packet);
            } else if particle_type_is(particle, &**particle_types::dripping_lava()) && packet.count() == 2 {
                MythologicalRitual::handle_echo_burrow_particle(packet);
            } else {
            }
        }
    }

    /// java: private static void handleBurrowParticle(ClientboundLevelParticlesPacket packet)
    pub fn handle_burrow_particle(packet: &ClientboundLevelParticlesPacket) {
        let mut pos: BlockPos = (BlockPos::containing(packet.x(), packet.y(), packet.z())).below();
        if mcrust::mcrust::modding::game::level().is_none() || !(mcrust::mcrust::modding::game::level().unwrap().get_block_state_typed(&pos).is_block(&blocks::blocks().get("GRASS_BLOCK"))) {
            return;
        }
        let mut burrow: compat::Shared<GriffinBurrow> = GRIFFIN_BURROWS.get_mut().entry(pos.clone()).or_insert_with_key(|k| GriffinBurrow::new(k.clone())).clone();
        let particle = packet.particle().expect("NullPointerException");
        if particle_type_is(particle, &**particle_types::crit()) {
            compat::write(&burrow, |__t| __t.crit_particle += 1);
        }
        if particle_type_is(particle, &**particle_types::enchant()) {
            compat::write(&burrow, |__t| __t.enchant_particle += 1);
        }
        if compat::read(&burrow, |__t| __t.crit_particle >= 5 && __t.enchant_particle >= 5 && __t.confirmed == TriState::False) {
            GriffinBurrow::init(&GRIFFIN_BURROWS.get_mut().get(&pos).cloned().unwrap());
        }
    }

    /// java: private static void handleNextBurrowParticle(ClientboundLevelParticlesPacket packet)
    pub fn handle_next_burrow_particle(packet: &ClientboundLevelParticlesPacket) {
        let mut pos: BlockPos = BlockPos::containing(packet.x(), packet.y(), packet.z());
        let mut burrow_pos: BlockPos;
        if let Some(level) = mcrust::mcrust::modding::game::level() {
            burrow_pos = mcrust::net::minecraft::world::level::level_reader::get_heightmap_pos(&level, mcrust::net::minecraft::world::level::levelgen::heightmap::Types::WorldSurface, &pos);
        } else {
            burrow_pos = pos.below_n(2);
        }
        let Some(mut burrow) = GRIFFIN_BURROWS.get_mut().get(&burrow_pos).cloned() else {
            return;
        };
        compat::write(&burrow, |__t| __t.regression.add_data(packet.x(), packet.z()));
        let mut slope: f64 = compat::read(&burrow, |__t| __t.regression.get_slope());
        if slope.is_nan() {
            return;
        }
        let mut next_burrow_direction: Vec3 = (Vec3::new((100) as f64, (0) as f64, (slope * (100 as f64)))).normalize();
        // Save the line of the next burrow and try to estimate the next burrow
        let mut pos2_d: Vector2D = Vector2D::new((pos.get_x() as f64) + 0.5, (pos.get_z() as f64) + 0.5);
        { let __v = Some(Line::new(pos2_d, pos2_d.add(&Vector2D::new(next_burrow_direction.x, next_burrow_direction.z)), 0.0001)); compat::write(&burrow, |__t| __t.next_burrow_line_estimation = __v); }
        MythologicalRitual::estimate_next_burrow(&burrow);
        // Fill line in the direction of the next burrow
        compat::write(&burrow, |__t| {
            let line = __t.next_burrow_line.get_or_insert_with(|| vec![vec3::ZERO; 1001]);
            MythologicalRitual::fill_line(line, vec3::at_center_of(&pos.above()), next_burrow_direction);
        });
    }

    /// java: private static void handleEchoBurrowParticle(ClientboundLevelParticlesPacket packet)
    pub fn handle_echo_burrow_particle(packet: &ClientboundLevelParticlesPacket) {
        if compat::millis() > (compat::get_state(&LAST_ECHO_TIME)).wrapping_add(10000 as i64) {
            return;
        }
        if mcrust::mcrust::modding::game::level().is_some() && !((mcrust::mcrust::modding::game::level().unwrap().get_block_state_typed(&BlockPos::containing(packet.x(), (packet.y() - 0.25), packet.z()))).is_air()) {
            return;
        }
        let previous_burrow = PREVIOUS_BURROW.get().clone();
        let echo = compat::write(&previous_burrow, |__t| {
            let direction = __t.echo_burrow_direction.get_or_insert_with(|| vec![None; 2]);
            direction[0] = direction[1];
            direction[1] = Some(Vec3::new(packet.x(), packet.y(), packet.z()));
            match (direction[0], direction[1]) {
                (Some(first), Some(second)) => Some((first, second)),
                _ => None,
            }
        });
        let Some((first, second)) = echo else {
            return;
        };
        // Save the line of the echo burrow and try to estimate the next burrow
        let mut pos1: Vector2D = Vector2D::new(first.x, first.z);
        let mut pos2: Vector2D = Vector2D::new(second.x, second.z);
        { let __v = Some(Line::new(pos1, pos2, 0.0001)); compat::write(&previous_burrow, |__t| __t.echo_burrow_line_estimation = __v); }
        MythologicalRitual::estimate_next_burrow(&previous_burrow);
        // Fill line in the direction of the echo burrow
        let mut echo_burrow_direction: Vec3 = second.subtract_vec(&first).normalize();
        compat::write(&previous_burrow, |__t| {
            let line = __t.echo_burrow_line.get_or_insert_with(|| vec![vec3::ZERO; 1001]);
            MythologicalRitual::fill_line(line, first, echo_burrow_direction);
        });
    }

    /// java: private static void estimateNextBurrow(GriffinBurrow burrow)
    pub fn estimate_next_burrow(burrow: &compat::Shared<GriffinBurrow>) {
        let (Some(next_line), Some(echo_line)) = compat::read(burrow, |__t| (__t.next_burrow_line_estimation, __t.echo_burrow_line_estimation)) else {
            return;
        };
        // `Line.intersection` answers null for (near-)parallel lines.
        let Some(intersection) = next_line.intersection(&echo_line) else {
            return;
        };
        { let __v = Some(BlockPos::containing(intersection.get_x(), 5.0, intersection.get_y())); compat::write(burrow, |__t| __t.next_burrow_estimated_pos = __v); }
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
                if compat::read(&burrow, |__t| __t.confirmed != TriState::False) {
                    if let Some(next_burrow_line) = compat::read(&burrow, |__t| __t.next_burrow_line.clone()) {
                        ({ let __f = compat::dispatch(collector, |__t| __t.submit_lines_from_points()); __f(collector, &next_burrow_line, &ORANGE_COLOR_COMPONENTS, 0.5f32, 5f32, false) });
                    }
                    if let Some(echo_burrow_line) = compat::read(&burrow, |__t| __t.echo_burrow_line.clone()) {
                        ({ let __f = compat::dispatch(collector, |__t| __t.submit_lines_from_points()); __f(collector, &echo_burrow_line, &ORANGE_COLOR_COMPONENTS, 0.5f32, 5f32, false) });
                    }
                    if compat::read(&burrow, |__t| __t.next_burrow_estimated_pos.is_some() && __t.confirmed == TriState::Default) {
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
            compat::write(PREVIOUS_BURROW.get(), |__t| __t.confirmed = TriState::False);
            let Some(last_dug_burrow_pos) = LAST_DUG_BURROW_POS.get().clone() else {
                return true;
            };
            // `previousBurrow = griffinBurrows.get(lastDugBurrowPos)`: the position was recorded only for a known burrow,
            // but a `reset()` in between can leave it unmapped, which Java reads as null and throws on below.
            *PREVIOUS_BURROW.get_mut() = GRIFFIN_BURROWS.get_mut().get(&last_dug_burrow_pos).cloned().expect("NullPointerException");
            compat::write(PREVIOUS_BURROW.get(), |__t| __t.confirmed = TriState::Default);
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
        compat::write(PREVIOUS_BURROW.get(), |__t| __t.confirmed = TriState::Default);
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
    /// Fabric's `net.fabricmc.fabric.api.util.TriState` (FALSE / DEFAULT / TRUE, compared by identity here) has no
    /// port; the game's `net.minecraft.util.TriState` has the same three constants.
    pub confirmed: TriState,
    /// java: private final SimpleRegression regression = new SimpleRegression();
    pub regression: SimpleRegression,
    /// java: private Vec3 @Nullable[] nextBurrowLine;
    pub next_burrow_line: Option<Vec<Vec3>>,
    /// java: private @Nullable Vec3 @Nullable[] echoBurrowDirection; - the positions of the last two echo burrow particles.
    pub echo_burrow_direction: Option<Vec<Option<Vec3>>>,
    /// java: private Vec3 @Nullable[] echoBurrowLine;
    pub echo_burrow_line: Option<Vec<Vec3>>,
    /// java: private @Nullable BlockPos nextBurrowEstimatedPos;
    pub next_burrow_estimated_pos: Option<BlockPos>,
    /// java: private @Nullable Line nextBurrowLineEstimation;
    pub next_burrow_line_estimation: Option<Line>,
    /// java: private @Nullable Line echoBurrowLineEstimation;
    pub echo_burrow_line_estimation: Option<Line>,
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
        let (_, __this) = WaypointBase::new3_in(pos.clone(), Type::Waypoint, ORANGE_COLOR_COMPONENTS.clone(), 0.25f32, move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                crit_particle: 0,
                enchant_particle: 0,
                confirmed: TriState::False,
                regression: SimpleRegression::new(),
                next_burrow_line: None,
                echo_burrow_direction: None,
                echo_burrow_line: None,
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
        compat::write(__this, |__t| {
            __t.confirmed = TriState::True;
            __t.regression.clear();
        });
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
        return <dyn Waypoint>::should_render_body(&compat::view(__this, |a| a as compat::Shared<dyn Waypoint>)) && compat::read(__this, |__t| __t.confirmed == TriState::True);
    }

    /// GriffinBurrow.shouldRender as the dispatch of Waypoint.shouldRender
    pub fn should_render_dyn(__this: &compat::Shared<dyn Waypoint>) -> bool {
        Self::should_render_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}

/// `ParticleTypes.X.equals(type)`: the port erases `getType()` (`Obj`), so the registered type is compared through
/// `Object.equals` (identity for a `ParticleType`, as in Java).
fn particle_type_is(options: &dyn mcrust::net::minecraft::core::particles::particle_options::ParticleOptions, particle_type: &dyn mcrust::com::mojang::serialization::JavaObject) -> bool {
    options.get_type().java_equals(particle_type)
}

/// `MathArrays.linearCombination(a1, b1, a2, b2)` (commons-math3): `a1 * b1 + a2 * b2` without the cancellation of
/// the plain sum - each product split exactly (`mul_add` for Dekker's split) and the parts summed with their errors;
/// a NaN answer falls back to the plain sum, as the library does.
fn linear_combination(a1: f64, b1: f64, a2: f64, b2: f64) -> f64 {
    let prod1 = a1 * b1;
    let prod1_low = a1.mul_add(b1, -prod1);
    let prod2 = a2 * b2;
    let prod2_low = a2.mul_add(b2, -prod2);
    let s12_high = prod1 + prod2;
    let s12_prime = s12_high - prod2;
    let s12_low = (prod2 - (s12_high - s12_prime)) + (prod1 - s12_prime);
    let result = s12_high + (prod1_low + prod2_low + s12_low);
    if result.is_nan() { prod1 + prod2 } else { result }
}

/// `org.apache.commons.math3.geometry.euclidean.twod.Vector2D` (commons-math3, unported - `crate::unresolved`): the
/// part this class uses. rust: belongs in its own file with the library's other classes.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector2D {
    x: f64,
    y: f64,
}

impl Vector2D {
    /// `new Vector2D(double x, double y)`
    pub fn new(x: f64, y: f64) -> Vector2D {
        Vector2D { x, y }
    }

    /// `add(Vector<Euclidean2D> v)`
    pub fn add(&self, v: &Vector2D) -> Vector2D {
        Vector2D::new(self.x + v.x, self.y + v.y)
    }

    /// `getX()`
    pub fn get_x(&self) -> f64 {
        self.x
    }

    /// `getY()`
    pub fn get_y(&self) -> f64 {
        self.y
    }
}

/// `org.apache.commons.math3.geometry.euclidean.twod.Line` (commons-math3, unported): `Line(p1, p2, tolerance)`
/// (`reset(p1, p2)`) and `intersection(Line)`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Line {
    angle: f64,
    cos: f64,
    sin: f64,
    origin_offset: f64,
    tolerance: f64,
}

impl Line {
    /// `new Line(Vector2D p1, Vector2D p2, double tolerance)`
    pub fn new(p1: Vector2D, p2: Vector2D, tolerance: f64) -> Line {
        let dx = p2.get_x() - p1.get_x();
        let dy = p2.get_y() - p1.get_y();
        let d = dx.hypot(dy);
        if d == 0.0 {
            Line { angle: 0.0, cos: 1.0, sin: 0.0, origin_offset: p1.get_y(), tolerance }
        } else {
            let angle = std::f64::consts::PI + (-dy).atan2(-dx);
            Line { angle, cos: angle.cos(), sin: angle.sin(), origin_offset: linear_combination(p2.get_x(), p1.get_y(), -p1.get_x(), p2.get_y()) / d, tolerance }
        }
    }

    /// `intersection(Line other)`: null when the lines are parallel within the tolerance.
    pub fn intersection(&self, other: &Line) -> Option<Vector2D> {
        let d = linear_combination(self.sin, other.cos, -other.sin, self.cos);
        if d.abs() < self.tolerance {
            return None;
        }
        Some(Vector2D::new(
            linear_combination(self.cos, other.origin_offset, -other.cos, self.origin_offset) / d,
            linear_combination(self.sin, other.origin_offset, -other.sin, self.origin_offset) / d,
        ))
    }
}

/// `org.apache.commons.math3.stat.regression.SimpleRegression` (commons-math3, unported), with an intercept (the
/// no-argument constructor): `addData`, `getSlope`, `clear`.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SimpleRegression {
    sum_x: f64,
    sum_xx: f64,
    sum_y: f64,
    sum_yy: f64,
    sum_xy: f64,
    n: i64,
    xbar: f64,
    ybar: f64,
}

impl SimpleRegression {
    /// `new SimpleRegression()`
    pub fn new() -> SimpleRegression {
        SimpleRegression::default()
    }

    /// `addData(double x, double y)`: the running means and centred sums.
    pub fn add_data(&mut self, x: f64, y: f64) {
        if self.n == 0 {
            self.xbar = x;
            self.ybar = y;
        } else {
            let fact1 = 1.0 + self.n as f64;
            let fact2 = self.n as f64 / (1.0 + self.n as f64);
            let dx = x - self.xbar;
            let dy = y - self.ybar;
            self.sum_xx += dx * dx * fact2;
            self.sum_yy += dy * dy * fact2;
            self.sum_xy += dx * dy * fact2;
            self.xbar += dx / fact1;
            self.ybar += dy / fact1;
        }
        self.sum_x += x;
        self.sum_y += y;
        self.n += 1;
    }

    /// `getSlope()`: NaN with fewer than two points or no spread in x.
    pub fn get_slope(&self) -> f64 {
        if self.n < 2 {
            return f64::NAN;
        }
        if self.sum_xx.abs() < 10.0 * f64::from_bits(1) {
            return f64::NAN;
        }
        self.sum_xy / self.sum_xx
    }

    /// `clear()`: the sums and the count (the means restart from the next point).
    pub fn clear(&mut self) {
        self.sum_x = 0.0;
        self.sum_xx = 0.0;
        self.sum_y = 0.0;
        self.sum_yy = 0.0;
        self.sum_xy = 0.0;
        self.n = 0;
    }
}

//! Ported from de.hysky.skyblocker.skyblock.crimson.dojo.DojoManager by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::events::particle_events;
use crate::de::hysky::skyblocker::events::world_events;
use crate::de::hysky::skyblocker::skyblock::crimson::dojo::control_test_helper::ControlTestHelper;
use crate::de::hysky::skyblocker::skyblock::crimson::dojo::discipline_test_helper::DisciplineTestHelper;
use crate::de::hysky::skyblocker::skyblock::crimson::dojo::force_test_helper::ForceTestHelper;
use crate::de::hysky::skyblocker::skyblock::crimson::dojo::mastery_test_helper::MasteryTestHelper;
use crate::de::hysky::skyblocker::skyblock::crimson::dojo::stamina_test_helper::StaminaTestHelper;
use crate::de::hysky::skyblocker::skyblock::crimson::dojo::swiftness_test_helper::SwiftnessTestHelper;
use crate::de::hysky::skyblocker::skyblock::crimson::dojo::tenacity_test_helper::TenacityTestHelper;
use crate::de::hysky::skyblocker::utils::render::level_render_extraction_callback;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_collector::PrimitiveCollector;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler::Scheduler;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::net::minecraft::chat_formatting;
use mcrust::net::minecraft::client::minecraft;
use mcrust::net::minecraft::client::multiplayer::client_level::ClientLevel;
use mcrust::net::minecraft::core::block_pos::BlockPos;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::protocol::game::clientbound_level_particles_packet::ClientboundLevelParticlesPacket;
use mcrust::net::minecraft::network::protocol::ping::serverbound_ping_request_packet::ServerboundPingRequestPacket;
use mcrust::net::minecraft::util::util;
use mcrust::net::minecraft::world::entity::entity::Entity;
use mcrust::net::minecraft::world::entity::player::player::Player;
use mcrust::net::minecraft::world::interaction_hand::InteractionHand;
use mcrust::net::minecraft::world::interaction_result::InteractionResult;
use mcrust::net::minecraft::world::level::block::state::block_state::BlockState;
use mcrust::net::minecraft::world::level::level::Level;
use mcrust::net::minecraft::world::phys::entity_hit_result::EntityHitResult;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final String START_MESSAGE = "[NPC] Master Tao: Ahhh, here we go! Let's get you into the Arena.";
pub const START_MESSAGE: &str = "[NPC] Master Tao: Ahhh, here we go! Let's get you into the Arena.";
/// java: private static final Pattern TEST_OF_PATTERN = Pattern.compile("\\s+Test of (\\w+) OBJECTIVES");
pub static TEST_OF_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("\\s+Test of (\\w+) OBJECTIVES"));
/// java: private static final String CHALLENGE_FINISHED_REGEX = "\\s+CHALLENGE ((COMPLETED)|(FAILED))";
pub const CHALLENGE_FINISHED_REGEX: &str = "\\s+CHALLENGE ((COMPLETED)|(FAILED))";
/// java: protected static DojoChallenges currentChallenge = DojoChallenges.NONE;
pub static CURRENT_CHALLENGE: LazyLock<Mutex<DojoChallenges>> = LazyLock::new(|| Mutex::new(DojoChallenges::None));
/// java: public static boolean inArena = false;
pub static IN_ARENA: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));
/// java: protected static long ping = -1;
pub static PING: LazyLock<Mutex<i64>> = LazyLock::new(|| Mutex::new(((-1) as i64) as i64));

/// java: public class DojoManager
#[derive(Clone)]
pub struct DojoManager {}

impl DojoManager {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        // TODO(fabric2seer): ClientReceiveMessageEvents.ALLOW_GAME.register: no SeerClient equivalent yet
        //   java: ClientReceiveMessageEvents.ALLOW_GAME.register(DojoManager::onMessage);
        level_render_extraction_callback::EVENT.register(Arc::new(move |__a0| { DojoManager::extract_rendering(&__a0); }));
        client_play_connection_events::JOIN.register(Arc::new(|listener: &mut ClientPacketListener| {
            DojoManager::reset();
        }));
        // TODO(fabric2seer): no SeerClient event yet: hook HEAD of ClientLevel#addEntity (`net.minecraft.client.multiplayer.ClientLevel#addEntity`: exists)
        //   java: ClientEntityEvents.ENTITY_LOAD.register(DojoManager::onEntitySpawn);
        // TODO(fabric2seer): no SeerClient event yet: hook HEAD of ClientLevel#removeEntity (`net.minecraft.client.multiplayer.ClientLevel#removeEntity`: exists)
        //   java: ClientEntityEvents.ENTITY_UNLOAD.register(DojoManager::onEntityDespawn);
        // TODO(fabric2seer): AttackEntityCallback.EVENT.register: no SeerClient equivalent yet
        //   java: AttackEntityCallback.EVENT.register(DojoManager::onEntityAttacked);
        ({ let __r = scheduler::INSTANCE.get_mut(); let __f = compat::dispatch(__r, |__t| __t.schedule_cyclic()); __f(__r, Arc::new(move || { DojoManager::update(); }), 3) });
        world_events::BLOCK_STATE_UPDATE.register(Arc::new(move |__a0, __a1, __a2| { DojoManager::on_block_update(&__a0, &__a1, &__a2); }));
        particle_events::FROM_SERVER.register(Arc::new(move |__a0| { DojoManager::on_particle(&__a0); }));
    }

    /// java: private static void reset()
    pub fn reset() {
        compat::set_state(&IN_ARENA, false);
        compat::set_state(&CURRENT_CHALLENGE, DojoChallenges::None);
        ForceTestHelper::reset();
        StaminaTestHelper::reset();
        MasteryTestHelper::reset();
        SwiftnessTestHelper::reset();
        ControlTestHelper::reset();
        TenacityTestHelper::reset();
    }

    /// java: private static boolean onMessage(Component text, Boolean overlay)
    pub fn on_message(text: &MutableComponent, mut overlay: bool) -> bool {
        if !(Utils::is_in_crimson()) || overlay {
            return true;
        }
        if chat_formatting::strip_formatting(&text.get_string()) == START_MESSAGE {
            compat::set_state(&IN_ARENA, true);
            DojoManager::get_ping();
            return true;
        }
        if !(compat::get_state(&IN_ARENA)) {
            return true;
        }
        if compat::regex::Pattern::matches(CHALLENGE_FINISHED_REGEX, &text.get_string()) {
            DojoManager::reset();
            return true;
        }
        if compat::get_state(&CURRENT_CHALLENGE) != DojoChallenges::None {
            return true;
        }
        let mut next_challenge: compat::regex::Matcher = TEST_OF_PATTERN.matcher(&text.get_string());
        if next_challenge.matches() {
            compat::set_state(&CURRENT_CHALLENGE, DojoChallenges::from(next_challenge.group(1).as_deref().unwrap()));
            if !(compat::get_state(&CURRENT_CHALLENGE).fields().enabled)(true) {
                compat::set_state(&CURRENT_CHALLENGE, DojoChallenges::None);
            }
        }
        return true;
    }

    /// java: private static void getPing()
    pub fn get_ping() {
        let mut network_handler: Option<mcrust::net::minecraft::client::multiplayer::client_packet_listener::PlayListenerSinks> = minecraft::get_connection(compat::minecraft());
        if let Some(network_handler) = &network_handler {
            // networkHandler.send(packet) = `this.connection.send(packet)`, on the connection's event loop (as
            // `PingDebugMonitor.tick` sends the same packet)
            let packet = ServerboundPingRequestPacket::new(util::get_millis());
            network_handler.connection.execute(Box::new(move |connection| {
                connection.send(Box::new(packet));
            }));
        }
    }

    /// java: public static void onPingResult(long ping)
    pub fn on_ping_result(mut ping: i64) {
        compat::set_state(&PING, ping);
    }

    /// java: @SuppressWarnings("incomplete-switch") private static void update()
    pub fn update() {
        if !(Utils::is_in_crimson()) || !(compat::get_state(&IN_ARENA)) {
            return;
        }
        match compat::get_state(&CURRENT_CHALLENGE) {
            DojoChallenges::Stamina => {
                StaminaTestHelper::update();
            }
            DojoChallenges::Control => {
                ControlTestHelper::update();
            }
            _ => {}
        }
    }

    /// java: public static boolean shouldGlow(String name)
    pub fn should_glow(name: &str) -> bool {
        if !(Utils::is_in_crimson()) || !(compat::get_state(&IN_ARENA)) {
            return false;
        }
        return match compat::get_state(&CURRENT_CHALLENGE) {
            DojoChallenges::Force => ForceTestHelper::should_glow(name),
            DojoChallenges::Discipline => DisciplineTestHelper::should_glow(name),
            _ => false,
        };
    }

    /// java: public static int getColor()
    pub fn get_color() -> i32 {
        if !(Utils::is_in_crimson()) || !(compat::get_state(&IN_ARENA)) {
            return 0xF57738;
        }
        return match compat::get_state(&CURRENT_CHALLENGE) {
            DojoChallenges::Force => ForceTestHelper::get_color(),
            DojoChallenges::Discipline => DisciplineTestHelper::get_color(),
            _ => 0xF57738,
        };
    }

    /// java: @SuppressWarnings("incomplete-switch") private static void onBlockUpdate(BlockPos pos, BlockState oldStatem, BlockState newState)
    pub fn on_block_update(pos: &BlockPos, old_statem: &Arc<BlockState>, new_state: &Arc<BlockState>) {
        if !(Utils::is_in_crimson()) || !(compat::get_state(&IN_ARENA)) {
            return;
        }
        match compat::get_state(&CURRENT_CHALLENGE) {
            DojoChallenges::Mastery => {
                MasteryTestHelper::on_block_update(pos.immutable(), new_state);
            }
            DojoChallenges::Swiftness => {
                SwiftnessTestHelper::on_block_update(&pos.immutable(), new_state);
            }
            _ => {}
        }
    }

    /// java: @SuppressWarnings("incomplete-switch") private static void onEntitySpawn(Entity entity, ClientLevel clientWorld)
    pub fn on_entity_spawn(entity: &dyn Entity, client_world: &ClientLevel) {
        if !(Utils::is_in_crimson()) || !(compat::get_state(&IN_ARENA)) || false /* java: compat::minecraft() == null: never null here */ || mcrust::mcrust::modding::game::with_player(|_| ()).is_none() {
            return;
        }
        if !(mcrust::mcrust::modding::game::with_player(|player| entity.base().closer_than_xz_y(player.base(), (50) as f64, (5) as f64)).unwrap_or(false)) {
            return;
        }
        match compat::get_state(&CURRENT_CHALLENGE) {
            DojoChallenges::Force => {
                ForceTestHelper::on_entity_spawn(entity);
            }
            DojoChallenges::Control => {
                ControlTestHelper::on_entity_spawn(entity);
            }
            DojoChallenges::Tenacity => {
                TenacityTestHelper::on_entity_spawn(entity);
            }
            _ => {}
        }
    }

    /// java: @SuppressWarnings("incomplete-switch") private static void onEntityDespawn(Entity entity, ClientLevel clientWorld)
    pub fn on_entity_despawn(entity: &dyn Entity, client_world: &ClientLevel) {
        if !(Utils::is_in_crimson()) || !(compat::get_state(&IN_ARENA)) {
            return;
        }
        match compat::get_state(&CURRENT_CHALLENGE) {
            DojoChallenges::Force => {
                ForceTestHelper::on_entity_despawn(entity);
            }
            DojoChallenges::Tenacity => {
                TenacityTestHelper::on_entity_despawn(entity);
            }
            _ => {}
        }
    }

    /// java: private static InteractionResult onEntityAttacked(Player playerEntity, Level world, InteractionHand hand, Entity entity, EntityHitResult entityHitResult)
    pub fn on_entity_attacked(player_entity: &dyn Player, world: &Level, hand: &InteractionHand, entity: &dyn Entity, entity_hit_result: &EntityHitResult) -> InteractionResult {
        if !(Utils::is_in_crimson()) || !(compat::get_state(&IN_ARENA)) {
            return InteractionResult::Pass;
        }
        if compat::get_state(&CURRENT_CHALLENGE) == DojoChallenges::Force {
            ForceTestHelper::on_entity_attacked(entity);
        }
        return InteractionResult::Pass;
    }

    /// java: private static void onParticle(ClientboundLevelParticlesPacket packet)
    pub fn on_particle(packet: &ClientboundLevelParticlesPacket) {
        if !(Utils::is_in_crimson()) || !(compat::get_state(&IN_ARENA)) {
            return;
        }
        if compat::get_state(&CURRENT_CHALLENGE) == DojoChallenges::Tenacity {
            TenacityTestHelper::on_particle(packet);
        }
    }

    /// java: @SuppressWarnings("incomplete-switch") private static void extractRendering(PrimitiveCollector collector)
    pub fn extract_rendering(collector: &compat::Shared<dyn PrimitiveCollector>) {
        if !(Utils::is_in_crimson()) || !(compat::get_state(&IN_ARENA)) {
            return;
        }
        match compat::get_state(&CURRENT_CHALLENGE) {
            DojoChallenges::Force => {
                ForceTestHelper::extract_rendering(collector);
            }
            DojoChallenges::Stamina => {
                StaminaTestHelper::extract_rendering(collector);
            }
            DojoChallenges::Mastery => {
                MasteryTestHelper::extract_rendering(collector);
            }
            DojoChallenges::Swiftness => {
                SwiftnessTestHelper::extract_rendering(collector);
            }
            DojoChallenges::Control => {
                ControlTestHelper::extract_rendering(collector);
            }
            DojoChallenges::Tenacity => {
                TenacityTestHelper::extract_rendering(collector);
            }
            _ => {}
        }
    }

}

/// java: protected enum DojoChallenges
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DojoChallenges {
    None,
    Force,
    Stamina,
    Mastery,
    Discipline,
    Swiftness,
    Control,
    Tenacity,
}

impl DojoChallenges {
    pub const VALUES: [DojoChallenges; 8] = [DojoChallenges::None, DojoChallenges::Force, DojoChallenges::Stamina, DojoChallenges::Mastery, DojoChallenges::Discipline, DojoChallenges::Swiftness, DojoChallenges::Control, DojoChallenges::Tenacity];

    /// java: values()
    pub fn values() -> Vec<DojoChallenges> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            DojoChallenges::None => "NONE",
            DojoChallenges::Force => "FORCE",
            DojoChallenges::Stamina => "STAMINA",
            DojoChallenges::Mastery => "MASTERY",
            DojoChallenges::Discipline => "DISCIPLINE",
            DojoChallenges::Swiftness => "SWIFTNESS",
            DojoChallenges::Control => "CONTROL",
            DojoChallenges::Tenacity => "TENACITY",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> DojoChallenges {
        match name {
            "NONE" => DojoChallenges::None,
            "FORCE" => DojoChallenges::Force,
            "STAMINA" => DojoChallenges::Stamina,
            "MASTERY" => DojoChallenges::Mastery,
            "DISCIPLINE" => DojoChallenges::Discipline,
            "SWIFTNESS" => DojoChallenges::Swiftness,
            "CONTROL" => DojoChallenges::Control,
            "TENACITY" => DojoChallenges::Tenacity,
            _ => panic!("IllegalArgumentException: No enum constant DojoChallenges.{name}"),
        }
    }

    /// the constant's constructor data (Java keeps it in the constant's fields)
    pub fn fields(self) -> &'static DojoChallengesFields {
        static FIELDS: LazyLock<Vec<DojoChallengesFields>> = LazyLock::new(|| vec![
            DojoChallengesFields::new("none".to_string(), Arc::new(move |_| false)),
            DojoChallengesFields::new("Force".to_string(), Arc::new(move |_| SkyblockerConfigManager::get().crimson_isle.dojo.enable_force_helper)),
            DojoChallengesFields::new("Stamina".to_string(), Arc::new(move |_| SkyblockerConfigManager::get().crimson_isle.dojo.enable_stamina_helper)),
            DojoChallengesFields::new("Mastery".to_string(), Arc::new(move |_| SkyblockerConfigManager::get().crimson_isle.dojo.enable_mastery_helper)),
            DojoChallengesFields::new("Discipline".to_string(), Arc::new(move |_| SkyblockerConfigManager::get().crimson_isle.dojo.enable_discipline_helper)),
            DojoChallengesFields::new("Swiftness".to_string(), Arc::new(move |_| SkyblockerConfigManager::get().crimson_isle.dojo.enable_swiftness_helper)),
            DojoChallengesFields::new("Control".to_string(), Arc::new(move |_| SkyblockerConfigManager::get().crimson_isle.dojo.enable_control_helper)),
            DojoChallengesFields::new("Tenacity".to_string(), Arc::new(move |_| SkyblockerConfigManager::get().crimson_isle.dojo.enable_tenacity_helper)),
        ]);
        &FIELDS[self as usize]
    }

    /// java: public static DojoChallenges from(String name)
    pub fn from(name: &str) -> DojoChallenges {
        return DojoChallenges::values().iter().copied().filter(|n| (name == n.fields().name)).next().unwrap_or(DojoChallenges::None);
    }

}

impl std::fmt::Display for DojoChallenges {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// the constructor data of each DojoChallenges constant
pub struct DojoChallengesFields {
    pub name: String,
    /// fastutil `BooleanPredicate`
    pub enabled: Arc<dyn Fn(bool) -> bool + Send + Sync>,
}

impl DojoChallengesFields {
    fn new(name: String, enabled: Arc<dyn Fn(bool) -> bool + Send + Sync>) -> Self {
        Self { name, enabled }
    }

}

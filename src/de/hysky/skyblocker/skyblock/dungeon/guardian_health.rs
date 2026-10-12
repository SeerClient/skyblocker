//! Ported from de.hysky.skyblocker.skyblock.dungeon.GuardianHealth by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::utils::render::level_render_extraction_callback;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_collector::PrimitiveCollector;
use crate::de::hysky::skyblocker::utils::render::render_helper::RenderHelper;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::net::minecraft::chat_formatting;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::world::entity::decoration::armor_stand::ArmorStand;
use mcrust::net::minecraft::world::entity::monster::guardian::Guardian;
use mcrust::net::minecraft::world::phys::aabb::AABB;
use mcrust::net::minecraft::network::chat::component::Component as _;
use mcrust::net::minecraft::world::entity::entity::Entity as _;
use mcrust::net::minecraft::world::level::entity_getter::EntityGetter as _;

/// java: private static final AABB bossRoom = new AABB(34, 65, -32, -32, 100, 36);
pub static BOSS_ROOM: compat::MainThread<AABB> = compat::MainThread::new(|| AABB::new((34) as f64, (65) as f64, (-32) as f64, (-32) as f64, (100) as f64, (36) as f64));
/// java: private static final Pattern guardianRegex = Pattern.compile("^(.*?) Guardian (.*?)([A-Za-z])❤$");
pub static GUARDIAN_REGEX: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("^(.*?) Guardian (.*?)([A-Za-z])❤$"));
/// java: private static final Pattern professorRegex = Pattern.compile("^﴾ (The Professor) (.*?)([A-za-z])❤ ﴿$");
pub static PROFESSOR_REGEX: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("^﴾ (The Professor) (.*?)([A-za-z])❤ ﴿$"));
/// java: private static boolean inBoss;
pub static IN_BOSS: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));

/// java: public class GuardianHealth
#[derive(Clone)]
pub struct GuardianHealth {}

impl GuardianHealth {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        // TODO(fabric2seer): ClientReceiveMessageEvents.ALLOW_GAME.register: no SeerClient equivalent yet
        //   java: ClientReceiveMessageEvents.ALLOW_GAME.register(GuardianHealth::onChatMessage);
        client_play_connection_events::JOIN.register(Arc::new(|listener: &mut ClientPacketListener| {
            GuardianHealth::reset();
        }));
        level_render_extraction_callback::EVENT.register(Arc::new(move |__a0| { GuardianHealth::extract_rendering(&__a0); }));
    }

    /// java: private static void extractRendering(PrimitiveCollector collector)
    pub fn extract_rendering(collector: &compat::Shared<dyn PrimitiveCollector>) {
        if !(SkyblockerConfigManager::get().dungeons.the_professor.floor3_guardian_health_display) {
            return;
        }
        let client = compat::minecraft();
        if Utils::is_in_dungeons() && compat::get_state(&IN_BOSS) && mcrust::mcrust::modding::game::with_player(|_| ()).is_some() && mcrust::mcrust::modding::game::level().is_some() {
            // TODO(fabric2seer): the port's `get_entities_of_class` ports this Java method with other parameters (base_class, except, bb, selector for Java's base_class, bb, selector)
            // TODO(fabric2seer): `get_entities_of_class` takes 4 argument(s) in the port (`base_class: crate::net::minecraft::world::level::entity::entity_type_test::EntityClass, except: Option<i32>, bb: &AABB, selector: &dyn Fn(&(dyn Entity + Send + Sync)) -> bool`), Java passes 3
            // TODO(fabric2seer): a lambda for a parameter the port does not type as a closure (`Predicate<? super T>`)
            // rust: let mut guardians: Vec<Guardian> = mcrust::mcrust::modding::game::level().unwrap().get_entities_of_class(std::any::TypeId::of::<Guardian>(), Some(BOSS_ROOM.get_mut().clone()), |_| true);
            let mut guardians: Vec<Guardian> = todo!("fabric2seer: the initialiser of `guardians` (TODO above)");
            for guardian in guardians.iter() {
                // TODO(fabric2seer): static `EntityTypes.ARMOR_STAND` has no Rust counterpart in the port's EntityTypes file
                // TODO(fabric2seer): `ClientLevel.getEntities` has no Rust method in the port (looked for `get_entities` in ClientLevel and its supertypes)
                // rust: let mut armor_stands: Vec<ArmorStand> = mcrust::mcrust::modding::game::level().unwrap().get_entities(&crate::unresolved::EntityTypes::ARMOR_STAND, &guardian.base.base.base.base.base.get_bounding_box().inflate3((0) as f64, (1) as f64, (0) as f64), &Arc::new(move |__a0| GuardianHealth::is_guardian_name(&__a0)));
                let mut armor_stands: Vec<ArmorStand> = todo!("fabric2seer: the initialiser of `armorStands` (TODO above)");
                for armor_stand in armor_stands.iter() {
                    // TODO(fabric2seer): `get_display_name` takes 1 argument(s) in the port (`team: Option<&crate::net::minecraft::world::scores::player_team::PlayerTeam>`), Java passes 0
                    if false /* java: compat::component_of(armor_stand.get_display_name()) == null: never null here */ {
                        continue;
                    }
                    // TODO(fabric2seer): `get_display_name` takes 1 argument(s) in the port (`team: Option<&crate::net::minecraft::world::scores::player_team::PlayerTeam>`), Java passes 0
                    // rust: let mut display: String = compat::component_of(armor_stand.get_display_name()).get_string();
                    let mut display: String = todo!("fabric2seer: the initialiser of `display` (TODO above)");
                    let mut professor: bool = display.contains("The Professor");
                    let mut matcher: compat::regex::Matcher = (if professor { PROFESSOR_REGEX.matcher(&display) } else { GUARDIAN_REGEX.matcher(&display) });
                    if !matcher.matches() {
                        continue;
                    }
                    let mut health: String = matcher.group(2).unwrap();
                    let mut quantity: String = matcher.group(3).unwrap();
                    // TODO(fabric2seer): `DeltaTracker.getGameTimeDeltaPartialTick` has no Rust method in the port (looked for `get_game_time_delta_partial_tick` in DeltaTracker and its supertypes)
                    // rust: let mut distance: f64 = RenderHelper::get_camera().position().distance_to(&guardian.base.base.base.base.base.get_position_partial(RenderHelper::get_tick_counter().get_game_time_delta_partial_tick(false)));
                    let mut distance: f64 = todo!("fabric2seer: the initialiser of `distance` (TODO above)");
                    // TODO(fabric2seer): `DeltaTracker.getGameTimeDeltaPartialTick` has no Rust method in the port (looked for `get_game_time_delta_partial_tick` in DeltaTracker and its supertypes)
                    // rust: ({ let __f = compat::dispatch(collector, |__t| __t.submit_text_4()); __f(collector, &(component::literal(&format!("{}{}", health, quantity))).with_style_format(ChatFormatting::Green), &guardian.base.base.base.base.base.get_position_partial(RenderHelper::get_tick_counter().get_game_time_delta_partial_tick(false)), ((((1 as f64) + (distance / (10 as f64)))) as f32), true) });
                }
            }
        }
    }

    /// java: private static void reset()
    pub fn reset() {
        compat::set_state(&IN_BOSS, false);
    }

    /// java: private static boolean onChatMessage(Component text, boolean overlay)
    pub fn on_chat_message(text: &MutableComponent, mut overlay: bool) -> bool {
        if Utils::is_in_dungeons() && SkyblockerConfigManager::get().dungeons.the_professor.floor3_guardian_health_display && !(compat::get_state(&IN_BOSS)) {
            let mut unformatted: String = chat_formatting::strip_formatting(&text.get_string());
            compat::set_state(&IN_BOSS, (unformatted == "[BOSS] The Professor: I was burdened with terrible news recently..."));
        }
        return true;
    }

    /// java: private static boolean isGuardianName(ArmorStand entity)
    pub fn is_guardian_name(entity: &ArmorStand) -> bool {
        // TODO(fabric2seer): `get_display_name` takes 1 argument(s) in the port (`team: Option<&crate::net::minecraft::world::scores::player_team::PlayerTeam>`), Java passes 0
        if false /* java: compat::component_of(entity.get_display_name()) == null: never null here */ {
            return false;
        }
        // TODO(fabric2seer): `get_display_name` takes 1 argument(s) in the port (`team: Option<&crate::net::minecraft::world::scores::player_team::PlayerTeam>`), Java passes 0
        // rust: let mut display: String = compat::component_of(entity.get_display_name()).get_string();
        let mut display: String = todo!("fabric2seer: the initialiser of `display` (TODO above)");
        if display.contains("The Professor") {
            return PROFESSOR_REGEX.matcher(&display).matches();
        }
        return (!(display == "Armor Stand") && GUARDIAN_REGEX.matcher(&display).matches());
    }

}

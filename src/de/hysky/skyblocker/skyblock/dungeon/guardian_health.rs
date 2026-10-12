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
            // java: List<Guardian> guardians = client.level.getEntitiesOfClass(Guardian.class, bossRoom, _ -> true);
            // (the port's class filter has no Guardian.class: the base class Entity plus `instanceof Guardian`)
            let level = mcrust::mcrust::modding::game::level().unwrap();
            let guardians: Vec<mcrust::net::minecraft::world::level::entity_getter::SharedEntity> = level.get_entities_of_class(
                mcrust::net::minecraft::world::level::entity::entity_type_test::EntityClass::Entity,
                None,
                BOSS_ROOM.get(),
                &|entity| is_guardian(entity),
            );
            for guardian in guardians.iter() {
                let partial_tick: f32 = RenderHelper::get_tick_counter().get_game_time_delta_partial_tick(false);
                // the guardian's box and `getPosition(tickDelta)`, read under one hold
                let (search_box, guardian_position) = {
                    let guardian = mcrust::mcrust::modding::game::lock_entity(&**guardian, "skyblocker GuardianHealth.extractRendering");
                    (guardian.base().get_bounding_box().inflate3((0) as f64, (1) as f64, (0) as f64), guardian.base().get_position_partial(partial_tick))
                };
                // java: List<ArmorStand> armorStands = client.level.getEntities(EntityTypes.ARMOR_STAND, guardian.getBoundingBox().inflate(0, 1, 0), GuardianHealth::isGuardianName);
                let armor_stands: Vec<mcrust::net::minecraft::world::level::entity_getter::SharedEntity> = level.get_entities(
                    None,
                    &search_box,
                    &|entity| entity.as_any().and_then(|any| any.downcast_ref::<ArmorStand>()).is_some_and(|armor_stand| GuardianHealth::is_guardian_name(armor_stand)),
                );
                for armor_stand in armor_stands.iter() {
                    // java: if (armorStand.getDisplayName() == null) continue; - getDisplayName never answers null
                    let mut display: String = {
                        let armor_stand = mcrust::mcrust::modding::game::lock_entity(&**armor_stand, "skyblocker GuardianHealth.extractRendering");
                        display_name_string(&*armor_stand)
                    };
                    let mut professor: bool = display.contains("The Professor");
                    let mut matcher: compat::regex::Matcher = (if professor { PROFESSOR_REGEX.matcher(&display) } else { GUARDIAN_REGEX.matcher(&display) });
                    if !matcher.matches() {
                        continue;
                    }
                    let mut health: String = matcher.group(2).unwrap();
                    let mut quantity: String = matcher.group(3).unwrap();
                    let mut distance: f64 = RenderHelper::get_camera().position().distance_to(&guardian_position);
                    ({ let __f = compat::dispatch(collector, |__t| __t.submit_text_4()); __f(collector, &(component::literal(&format!("{}{}", health, quantity))).with_style_format(ChatFormatting::Green), &guardian_position, ((((1 as f64) + (distance / (10 as f64)))) as f32), true) });
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
        // java: if (entity.getDisplayName() == null) return false; - getDisplayName never answers null
        let mut display: String = display_name_string(entity);
        if display.contains("The Professor") {
            return PROFESSOR_REGEX.matcher(&display).matches();
        }
        return (!(display == "Armor Stand") && GUARDIAN_REGEX.matcher(&display).matches());
    }

}

/// `entity instanceof Guardian` (an `ElderGuardian` is one: its `base` is a real `Guardian`).
fn is_guardian(entity: &(dyn mcrust::net::minecraft::world::entity::entity::Entity + Send + Sync)) -> bool {
    entity.as_any().is_some_and(|any| any.is::<Guardian>() || any.is::<mcrust::net::minecraft::world::entity::monster::elder_guardian::ElderGuardian>())
}

/// `entity.getDisplayName().getString()`: Java's `getDisplayName()` reads the entity's team off the level's scoreboard
/// (`getTeam()`), which the port's `get_display_name` takes as a parameter.
fn display_name_string<E: mcrust::net::minecraft::world::entity::entity::Entity + ?Sized>(entity: &E) -> String {
    let level = mcrust::mcrust::modding::game::level();
    let board = level.as_ref().and_then(|level| level.get_scoreboard());
    let guard = board.as_ref().map(|board| board.lock().unwrap_or_else(|p| p.into_inner()));
    let team = guard.as_deref().and_then(|board| entity.get_team(board, level.as_deref()));
    entity.get_display_name(team).get_string()
}

//! Ported from de.hysky.skyblocker.skyblock.dungeon.DungeonScore by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::configs::dungeons_config::BatMessage;
use crate::de::hysky::skyblocker::config::configs::dungeons_config::MimicMessage;
use crate::de::hysky::skyblocker::config::configs::dungeons_config::PrinceMessage;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::events::dungeon_events;
use crate::de::hysky::skyblocker::events::skyblock_events;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::dungeon_manager::DungeonManager;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::secret_sync::SecretSync;
use crate::de::hysky::skyblocker::skyblock::tabhud::util::player_list_manager::PlayerListManager;
use crate::de::hysky::skyblocker::utils::constants;
use crate::de::hysky::skyblocker::utils::mayor::mayor_utils::MayorUtils;
use crate::de::hysky::skyblocker::utils::profile_utils::ProfileUtils;
use crate::de::hysky::skyblocker::utils::scheduler::message_scheduler;
use crate::de::hysky::skyblocker::utils::scheduler::message_scheduler::MessageScheduler;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler::Scheduler;
use crate::de::hysky::skyblocker::utils::utils;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use crate::de::hysky::skyblocker::utils::item_utils::ItemUtils;
use mcrust::net::minecraft::client::gui::hud;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::world::entity::entity::Entity;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final Supplier<DungeonsConfig.DungeonScore> SCORE_CONFIG = () -> SkyblockerConfigManager.get().dungeons.dungeonScore;
pub static SCORE_CONFIG: compat::MainThread<Arc<dyn Fn() -> compat::Shared<crate::de::hysky::skyblocker::config::configs::dungeons_config::DungeonScore>>> = compat::MainThread::new(|| Arc::new(move || SkyblockerConfigManager::get().dungeons.dungeon_score.clone()));
/// java: private static final Supplier<DungeonsConfig.MimicMessage> MIMIC_MESSAGE_CONFIG = () -> SkyblockerConfigManager.get().dungeons.mimicMessage;
pub static MIMIC_MESSAGE_CONFIG: compat::MainThread<Arc<dyn Fn() -> compat::Shared<MimicMessage>>> = compat::MainThread::new(|| Arc::new(move || SkyblockerConfigManager::get().dungeons.mimic_message.clone()));
/// java: private static final Supplier<DungeonsConfig.PrinceMessage> PRINCE_MESSAGE_CONFIG = () -> SkyblockerConfigManager.get().dungeons.princeMessage;
pub static PRINCE_MESSAGE_CONFIG: compat::MainThread<Arc<dyn Fn() -> compat::Shared<PrinceMessage>>> = compat::MainThread::new(|| Arc::new(move || SkyblockerConfigManager::get().dungeons.prince_message.clone()));
/// java: private static final Supplier<DungeonsConfig.BatMessage> BAT_MESSAGE_CONFIG = () -> SkyblockerConfigManager.get().dungeons.batMessage;
pub static BAT_MESSAGE_CONFIG: compat::MainThread<Arc<dyn Fn() -> compat::Shared<BatMessage>>> = compat::MainThread::new(|| Arc::new(move || SkyblockerConfigManager::get().dungeons.bat_message.clone()));
/// java: private static final Pattern CLEARED_PATTERN = Pattern.compile("Cleared: (?<cleared>\\d+)%.*");
pub static CLEARED_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("Cleared: (?<cleared>\\d+)%.*"));
/// java: private static final Pattern FLOOR_PATTERN = Pattern.compile(".*?(?=T)The Catacombs \\((?<floor>[EFM]\\D*\\d*)\\)");
pub static FLOOR_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile(".*?(?=T)The Catacombs \\((?<floor>[EFM]\\D*\\d*)\\)"));
/// java: private static final Pattern SECRETS_PATTERN = Pattern.compile("Secrets Found: (?<secper>\\d+\\.?\\d*)%");
pub static SECRETS_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("Secrets Found: (?<secper>\\d+\\.?\\d*)%"));
/// java: private static final Pattern PUZZLES_PATTERN = Pattern.compile(".+?(?=:): \\[(?<state>.)](?: \\(\\w*\\))?");
pub static PUZZLES_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile(".+?(?=:): \\[(?<state>.)](?: \\(\\w*\\))?"));
/// java: private static final Pattern PUZZLE_COUNT_PATTERN = Pattern.compile("Puzzles: \\((?<count>\\d+)\\)");
pub static PUZZLE_COUNT_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("Puzzles: \\((?<count>\\d+)\\)"));
/// java: private static final Pattern CRYPTS_PATTERN = Pattern.compile("Crypts: (?<crypts>\\d+)");
pub static CRYPTS_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("Crypts: (?<crypts>\\d+)"));
/// java: private static final Pattern COMPLETED_ROOMS_PATTERN = Pattern.compile(" *Completed Rooms: (?<rooms>\\d+)");
pub static COMPLETED_ROOMS_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile(" *Completed Rooms: (?<rooms>\\d+)"));
/// java: private static final Pattern DEATHS_PATTERN = Pattern.compile(" \\u2620 (?<whodied>\\S+) .*");
pub static DEATHS_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile(" \\u2620 (?<whodied>\\S+) .*"));
/// java: private static final Pattern MIMIC_PATTERN = Pattern.compile(".*?(?:Mimic dead!?|Mimic Killed!|\\$SKYTILS-DUNGEON-SCORE-MIMIC\\$)$");
pub static MIMIC_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile(".*?(?:Mimic dead!?|Mimic Killed!|\\$SKYTILS-DUNGEON-SCORE-MIMIC\\$)$"));
/// java: private static final String HYPIXEL_PRINCE_KILL_MESSAGE = "A Prince falls. +1 Bonus Score";
pub const HYPIXEL_PRINCE_KILL_MESSAGE: &str = "A Prince falls. +1 Bonus Score";
/// java: private static final Pattern PRINCE_PATTERN = Pattern.compile(".*?(?:Prince dead!?|Prince Killed!)$");
pub static PRINCE_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile(".*?(?:Prince dead!?|Prince Killed!)$"));
/// java: private static final String HYPIXEL_BAT_KILL_MESSAGE = "A Bat has been slain. +1 Bonus Score";
pub const HYPIXEL_BAT_KILL_MESSAGE: &str = "A Bat has been slain. +1 Bonus Score";
/// java: private static final Pattern BAT_PATTERN = Pattern.compile(".*?(?:Bat dead!?|Bat Killed!)$");
pub static BAT_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile(".*?(?:Bat dead!?|Bat Killed!)$"));
/// java: private static final Pattern MIMIC_FLOORS_PATTERN = Pattern.compile("[FM][67]");
pub static MIMIC_FLOORS_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("[FM][67]"));
/// java: private static final String MIMIC_MESSAGE = "Mimic dead!";
pub const MIMIC_MESSAGE: &str = "Mimic dead!";
/// java: private static final String PRINCE_MESSAGE = "Prince dead!";
pub const PRINCE_MESSAGE: &str = "Prince dead!";
/// java: private static final String BAT_MESSAGE = "Bat dead!";
pub const BAT_MESSAGE: &str = "Bat dead!";
/// java: private static boolean isMayorPaul = false;
pub static IS_MAYOR_PAUL: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));
/// java: private static FloorRequirement floorRequirement = FloorRequirement.NONE;
pub static FLOOR_REQUIREMENT: LazyLock<Mutex<FloorRequirement>> = LazyLock::new(|| Mutex::new(FloorRequirement::None));
/// java: private static String currentFloor = "";
pub static CURRENT_FLOOR: LazyLock<Mutex<String>> = LazyLock::new(|| Mutex::new("".to_string()));
/// java: private static boolean isCurrentFloorEntrance;
pub static IS_CURRENT_FLOOR_ENTRANCE: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));
/// java: private static boolean floorHasMimics;
pub static FLOOR_HAS_MIMICS: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));
/// java: private static boolean sentCrypts;
pub static SENT_CRYPTS: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));
/// java: private static boolean sent270;
pub static SENT270: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));
/// java: private static boolean sent300;
pub static SENT300: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));
/// java: private static boolean mimicKilled;
pub static MIMIC_KILLED: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));
/// java: private static boolean princeKilled;
pub static PRINCE_KILLED: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));
/// java: private static boolean batKilled;
pub static BAT_KILLED: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));
/// java: private static boolean dungeonStarted;
pub static DUNGEON_STARTED: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));
/// java: private static boolean firstDeathHasSpiritPet;
pub static FIRST_DEATH_HAS_SPIRIT_PET: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));
/// java: private static boolean bloodRoomCompleted;
pub static BLOOD_ROOM_COMPLETED: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));
/// java: private static long startingTime;
pub static STARTING_TIME: LazyLock<Mutex<i64>> = LazyLock::new(|| Mutex::new(0));
/// java: private static int puzzleCount;
pub static PUZZLE_COUNT: LazyLock<Mutex<i32>> = LazyLock::new(|| Mutex::new(0));
/// java: private static int deathCount;
pub static DEATH_COUNT: LazyLock<Mutex<i32>> = LazyLock::new(|| Mutex::new(0));
/// java: private static int score;
pub static SCORE: LazyLock<Mutex<i32>> = LazyLock::new(|| Mutex::new(0));

/// java: public class DungeonScore
#[derive(Clone)]
pub struct DungeonScore {}

impl DungeonScore {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        ({ let __r = scheduler::INSTANCE.get_mut(); let __f = compat::dispatch(__r, |__t| __t.schedule_cyclic()); __f(__r, Arc::new(move || { DungeonScore::tick(); }), 20) });
        client_play_connection_events::JOIN.register(Arc::new(|listener: &mut ClientPacketListener| {
            DungeonScore::reset();
        }));
        dungeon_events::DUNGEON_STARTED.register(Arc::new(move || { DungeonScore::on_dungeon_start(); }));
        dungeon_events::DUNGEON_ENDED.register(Arc::new(move || { DungeonScore::reset(); }));
        // TODO(fabric2seer): ClientReceiveMessageEvents.ALLOW_GAME.register: no SeerClient equivalent yet
        //   java: ClientReceiveMessageEvents.ALLOW_GAME.register((message, overlay) -> {
        //   java: 			if (overlay || !Utils.isInDungeons()) return true;
        //   java: 			String str = message.getString();
        //   java: 			if (dungeonStarted) {
        //   java: 				checkMessageForDeaths(str);
        //   java: 				checkMessageForWatcher(str);
        //   java: 				if (floorHasMimics) checkMessageForMimic(str); //Only called when the message is not cancelled & isn't on the action bar, complementing MimicFilter
        //   java: 				checkMessageForPrince(str);
        //   java: 				checkMessageForBat(str);
        //   java: 			}
        //   java:
        //   java: 			return true;
        //   java: 		});
        skyblock_events::MAYOR_CHANGE.register(Arc::new(move || { compat::set_state(&IS_MAYOR_PAUL, MayorUtils::get_active_perks().iter().any(|e| *e == "EZPZ")); }));
    }

    /// java: public static void tick()
    pub fn tick() {
        let client = compat::minecraft();
        if !(Utils::is_in_dungeons()) || mcrust::mcrust::modding::game::with_player(|_| ()).is_none() {
            DungeonScore::reset();
            return;
        }
        if !(compat::get_state(&DUNGEON_STARTED)) {
            return;
        }
        compat::set_state(&SCORE, DungeonScore::calculate_score());
        if !(compat::get_state(&SENT270)) && !(compat::get_state(&SENT300)) && compat::get_state(&SCORE) >= 270 && compat::get_state(&SCORE) < 300 {
            if compat::read(&(SCORE_CONFIG.get_mut())(), |__t| __t.enable_dungeon_score270_message) {
                MessageScheduler::send_message_after_cooldown(message_scheduler::INSTANCE.get_mut(), &format!("/pc {}{}", (constants::PREFIX.get_mut())().get_string(), compat::regex::replace_all(&(SCORE_CONFIG.get_mut())().lock().borrow().dungeon_score270_message, "\\[score]", "270")), true);
            }
            if compat::read(&(SCORE_CONFIG.get_mut())(), |__t| __t.enable_dungeon_score270_title) {
                client.gui.hud.reset_title_times();
                mcrust::net::minecraft::client::multiplayer::client_packet_listener::apply_set_title_text(&mut client.gui.hud, &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::null_to_empty(Some(&compat::regex::replace_all(&(SCORE_CONFIG.get_mut())().lock().borrow().dungeon_score270_message, "\\[score]", "270")))));
            }
            if compat::read(&(SCORE_CONFIG.get_mut())(), |__t| __t.enable_dungeon_score270_sound) {
                mcrust::mcrust::modding::game::with_player(|player| { player.play_sound(mcrust::net::minecraft::sounds::sound_events::note_block_pling().clone(), 100.0f32, 0.1f32); });
            }
            compat::set_state(&SENT270, true);
        }
        let mut crypts: i32 = DungeonScore::get_crypts();
        if !(compat::get_state(&SENT_CRYPTS)) && compat::get_state(&SCORE) >= compat::read(&(SCORE_CONFIG.get_mut())(), |__t| __t.dungeon_crypts_message_threshold) && crypts < 5 {
            if compat::read(&(SCORE_CONFIG.get_mut())(), |__t| __t.enable_dungeon_crypts_message) {
                MessageScheduler::send_message_after_cooldown(message_scheduler::INSTANCE.get_mut(), &format!("/pc {}{}", (constants::PREFIX.get_mut())().get_string(), compat::regex::replace_all(&(SCORE_CONFIG.get_mut())().lock().borrow().dungeon_crypts_message, "\\[crypts]", &crypts.to_string())), true);
            }
            compat::set_state(&SENT_CRYPTS, true);
        }
        if !(compat::get_state(&SENT300)) && compat::get_state(&SCORE) >= 300 {
            if compat::read(&(SCORE_CONFIG.get_mut())(), |__t| __t.enable_dungeon_score300_message) {
                MessageScheduler::send_message_after_cooldown(message_scheduler::INSTANCE.get_mut(), &format!("/pc {}{}", (constants::PREFIX.get_mut())().get_string(), compat::regex::replace_all(&(SCORE_CONFIG.get_mut())().lock().borrow().dungeon_score300_message, "\\[score]", "300")), true);
            }
            if compat::read(&(SCORE_CONFIG.get_mut())(), |__t| __t.enable_dungeon_score300_title) {
                client.gui.hud.reset_title_times();
                mcrust::net::minecraft::client::multiplayer::client_packet_listener::apply_set_title_text(&mut client.gui.hud, &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::null_to_empty(Some(&compat::regex::replace_all(&(SCORE_CONFIG.get_mut())().lock().borrow().dungeon_score300_message, "\\[score]", "300")))));
            }
            if compat::read(&(SCORE_CONFIG.get_mut())(), |__t| __t.enable_dungeon_score300_sound) {
                mcrust::mcrust::modding::game::with_player(|player| { player.play_sound(mcrust::net::minecraft::sounds::sound_events::note_block_pling().clone(), 100.0f32, 0.1f32); });
            }
            compat::set_state(&SENT300, true);
        }
    }

    /// java: private static void reset()
    pub fn reset() {
        compat::set_state(&FLOOR_REQUIREMENT, FloorRequirement::None);
        compat::set_state(&CURRENT_FLOOR, "".to_string());
        compat::set_state(&IS_CURRENT_FLOOR_ENTRANCE, false);
        compat::set_state(&FLOOR_HAS_MIMICS, false);
        compat::set_state(&SENT_CRYPTS, false);
        compat::set_state(&SENT270, false);
        compat::set_state(&SENT300, false);
        compat::set_state(&MIMIC_KILLED, false);
        compat::set_state(&PRINCE_KILLED, false);
        compat::set_state(&BAT_KILLED, false);
        compat::set_state(&DUNGEON_STARTED, false);
        compat::set_state(&FIRST_DEATH_HAS_SPIRIT_PET, false);
        compat::set_state(&BLOOD_ROOM_COMPLETED, false);
        compat::set_state(&STARTING_TIME, 0);
        compat::set_state(&PUZZLE_COUNT, 0);
        compat::set_state(&DEATH_COUNT, 0);
        compat::set_state(&SCORE, 0);
    }

    /// java: private static void onDungeonStart()
    pub fn on_dungeon_start() {
        DungeonScore::reset();
        DungeonScore::set_current_floor();
        compat::set_state(&DUNGEON_STARTED, true);
        compat::set_state(&PUZZLE_COUNT, DungeonScore::get_puzzle_count());
        compat::set_state(&STARTING_TIME, compat::millis());
        // java: try {..} catch (IllegalArgumentException _) - an exception here is a panic, caught by `catch_unwind`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            compat::set_state(&FLOOR_REQUIREMENT, FloorRequirement::value_of(&compat::get_state(&CURRENT_FLOOR)));
        }));
        if let Err(_) = __result {
            compat::set_state(&FLOOR_REQUIREMENT, FloorRequirement::None);
        }
        compat::set_state(&FLOOR_HAS_MIMICS, MIMIC_FLOORS_PATTERN.matcher(&compat::get_state(&CURRENT_FLOOR)).matches());
        if compat::get_state(&CURRENT_FLOOR) == "E" {
            compat::set_state(&IS_CURRENT_FLOOR_ENTRANCE, true);
        }
    }

    /// java: private static int calculateScore()
    pub fn calculate_score() -> i32 {
        if compat::get_state(&IS_CURRENT_FLOOR_ENTRANCE) {
            return (((compat::java_round_f32(((DungeonScore::calculate_time_score() as f32) * 0.7f32))).wrapping_add(compat::java_round_f32(((DungeonScore::calculate_explore_score() as f32) * 0.7f32)))).wrapping_add(compat::java_round_f32(((DungeonScore::calculate_skill_score() as f32) * 0.7f32)))).wrapping_add(compat::java_round_f32(((DungeonScore::calculate_bonus_score() as f32) * 0.7f32)));
        }
        return (((DungeonScore::calculate_time_score()).wrapping_add(DungeonScore::calculate_explore_score())).wrapping_add(DungeonScore::calculate_skill_score())).wrapping_add(DungeonScore::calculate_bonus_score());
    }

    /// java: private static int calculateSkillScore()
    pub fn calculate_skill_score() -> i32 {
        let mut total_rooms: i32 = DungeonScore::get_total_rooms();
        let mut completed_room_score: i32 = (if total_rooms != 0 { ((((80.0 * ((DungeonScore::get_completed_rooms()).wrapping_add(DungeonScore::get_extra_completed_rooms()) as f64)) / (total_rooms as f64))) as i32) } else { 0 }).clamp(0, 80);
        return 20i32.wrapping_add(((completed_room_score.wrapping_sub(DungeonScore::get_puzzle_penalty())).wrapping_sub(DungeonScore::get_death_score_penalty())).clamp(0, 80));
    }

    /// java: private static int calculateExploreScore()
    pub fn calculate_explore_score() -> i32 {
        let mut total_rooms: i32 = DungeonScore::get_total_rooms();
        let mut completed_room_score: i32 = (if total_rooms != 0 { ((((60.0 * ((DungeonScore::get_completed_rooms()).wrapping_add(DungeonScore::get_extra_completed_rooms()) as f64)) / (total_rooms as f64))) as i32) } else { 0 }).clamp(0, 60);
        let mut secrets_score: i32 = (((((40 as f64) * (compat::lock(&FLOOR_REQUIREMENT).fields().percentage as f64).min(DungeonScore::get_secrets_percentage())) / (compat::lock(&FLOOR_REQUIREMENT).fields().percentage as f64))) as i32).clamp(0, 40);
        return completed_room_score.wrapping_add(secrets_score);
    }

    /// java: private static int calculateTimeScore()
    pub fn calculate_time_score() -> i32 {
        let mut score: i32 = 100;
        let mut time_spent: i32 = (((compat::millis()).wrapping_sub(compat::get_state(&STARTING_TIME))) as i32).wrapping_div(1000);
        if time_spent < compat::lock(&FLOOR_REQUIREMENT).fields().time_limit {
            return score;
        }
        let mut time_past_requirement: f64 = ((((time_spent.wrapping_sub(compat::lock(&FLOOR_REQUIREMENT).fields().time_limit)) as f64) / (compat::lock(&FLOOR_REQUIREMENT).fields().time_limit as f64)) * (100 as f64));
        if time_past_requirement < (20 as f64) {
            return score.wrapping_sub(((time_past_requirement) as i32).wrapping_div(2));
        }
        if time_past_requirement < (40 as f64) {
            return score.wrapping_sub((((10 as f64) + ((time_past_requirement - (20 as f64)) / (4 as f64)))) as i32);
        }
        if time_past_requirement < (50 as f64) {
            return score.wrapping_sub((((15 as f64) + ((time_past_requirement - (40 as f64)) / (5 as f64)))) as i32);
        }
        if time_past_requirement < (60 as f64) {
            return score.wrapping_sub((((17 as f64) + ((time_past_requirement - (50 as f64)) / (6 as f64)))) as i32);
        }
        return (score.wrapping_sub(((((18 as f64) + (2.0 / 3.0)) + ((time_past_requirement - (60 as f64)) / (7 as f64)))) as i32)).clamp(0, 100);
    }

    /// java: private static int calculateBonusScore()
    pub fn calculate_bonus_score() -> i32 {
        let mut paul_score: i32 = (if compat::get_state(&IS_MAYOR_PAUL) { 10 } else { 0 });
        let mut crypts_score: i32 = (DungeonScore::get_crypts()).clamp(0, 5);
        let mut mimic_score: i32 = (if compat::get_state(&MIMIC_KILLED) { 2 } else { 0 });
        if DungeonScore::get_secrets_percentage() >= (100 as f64) && compat::get_state(&FLOOR_HAS_MIMICS) {
            mimic_score = 2;
        }
        let mut prince_score: i32 = (if compat::get_state(&PRINCE_KILLED) { 1 } else { 0 });
        let mut bat_score: i32 = (if compat::get_state(&BAT_KILLED) { 1 } else { 0 });
        return (((paul_score.wrapping_add(crypts_score)).wrapping_add(mimic_score)).wrapping_add(prince_score)).wrapping_add(bat_score);
    }

    /// java: public static boolean isEntityMimic(Entity entity)
    pub fn is_entity_mimic(entity: &dyn Entity) -> bool {
        //   java: if (!Utils.isInDungeons() || !floorHasMimics || !(entity instanceof Zombie zombie) || !zombie.isBaby()) return false;
        // `instanceof Zombie` is the port's `as_zombie` slot (every zombie-family class answers it)
        let Some(zombie) = entity.as_zombie() else {
            return false;
        };
        if !Utils::is_in_dungeons() || !compat::get_state(&FLOOR_HAS_MIMICS) || !zombie.is_baby() {
            return false;
        }
        // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`; the body's `return` is the closure's value
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            //   java: List<ItemStack> armor = ItemUtils.getArmor(zombie);
            let mut armor: Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack> = ItemUtils::get_armor(zombie.as_living());
            return armor.iter().cloned().all(|__a0| __a0.is_empty());
        })) {
            Ok(__v) => return __v,
            Err(e) => {
                logger().error(&format!("[Skyblocker] Failed to check if entity is a mimic!"));
                return false;
            }
        }
    }

    /// java: public static void handleEntityDeath(Entity entity)
    pub fn handle_entity_death(entity: &dyn Entity) {
        if compat::get_state(&MIMIC_KILLED) {
            return;
        }
        if !(DungeonScore::is_entity_mimic(entity)) {
            return;
        }
        if compat::read(&(MIMIC_MESSAGE_CONFIG.get_mut())(), |__t| __t.send_mimic_message) {
            MessageScheduler::send_message_after_cooldown(message_scheduler::INSTANCE.get_mut(), &format!("/pc {}", MIMIC_MESSAGE), true);
        }
        SecretSync::sync_mimic_killed();
        compat::set_state(&MIMIC_KILLED, true);
    }

    /// java: public static void onMimicKill()
    pub fn on_mimic_kill() {
        compat::set_state(&MIMIC_KILLED, true);
    }

    /// java: public static void onPrinceKill(boolean fromHypixel)
    pub fn on_prince_kill(mut from_hypixel: bool) {
        if compat::get_state(&PRINCE_KILLED) {
            return;
        }
        if from_hypixel {
            if compat::read(&(PRINCE_MESSAGE_CONFIG.get_mut())(), |__t| __t.send_prince_message) {
                MessageScheduler::send_message_after_cooldown(message_scheduler::INSTANCE.get_mut(), &format!("/pc {}", PRINCE_MESSAGE), true);
            }
            SecretSync::sync_prince_killed();
        }
        compat::set_state(&PRINCE_KILLED, true);
    }

    /// java: public static void onBatKill(boolean fromHypixel)
    pub fn on_bat_kill(mut from_hypixel: bool) {
        if compat::get_state(&BAT_KILLED) {
            return;
        }
        if from_hypixel {
            if compat::read(&(BAT_MESSAGE_CONFIG.get_mut())(), |__t| __t.send_bat_message) {
                MessageScheduler::send_message_after_cooldown(message_scheduler::INSTANCE.get_mut(), &format!("/pc {}", BAT_MESSAGE), true);
            }
            SecretSync::sync_bat_killed();
        }
        compat::set_state(&BAT_KILLED, true);
    }

    /// java: public static boolean wasPrinceKilled()
    pub fn was_prince_killed() -> bool {
        return compat::get_state(&PRINCE_KILLED);
    }

    /// java: private static int getTotalRooms()
    pub fn get_total_rooms() -> i32 {
        return ((compat::java_round_f64(((DungeonScore::get_completed_rooms() as f64) / DungeonScore::get_clear_percentage()))) as i32);
    }

    /// java: private static int getCompletedRooms()
    pub fn get_completed_rooms() -> i32 {
        let mut matcher: Option<compat::regex::Matcher> = PlayerListManager::regex_at(43, &COMPLETED_ROOMS_PATTERN);
        return (if matcher.is_some() { matcher.as_mut().unwrap().group_named("rooms").unwrap().parse::<i32>().expect("NumberFormatException") } else { 0 });
    }

    /// java: private static int getExtraCompletedRooms()
    pub fn get_extra_completed_rooms() -> i32 {
        if !(compat::get_state(&BLOOD_ROOM_COMPLETED)) {
            return (if compat::get_state(&IS_CURRENT_FLOOR_ENTRANCE) { 1 } else { 2 });
        }
        if !(DungeonManager::is_in_boss()) && !(compat::get_state(&IS_CURRENT_FLOOR_ENTRANCE)) {
            return 1;
        }
        return 0;
    }

    /// java: private static double getClearPercentage()
    pub fn get_clear_percentage() -> f64 {
        for sidebar_line in compat::get_state(&utils::STRING_SCOREBOARD) {
            let mut clear_matcher: compat::regex::Matcher = CLEARED_PATTERN.matcher(&sidebar_line);
            if !clear_matcher.matches() {
                continue;
            }
            return (clear_matcher.group_named("cleared").unwrap().trim().parse::<f64>().expect("NumberFormatException") / 100.0);
        }
        logger().error("[Skyblocker] Clear pattern doesn't match!");
        return ((0) as f64) as f64;
    }

    /// java: private static int getDeathScorePenalty()
    pub fn get_death_score_penalty() -> i32 {
        return ((compat::get_state(&DEATH_COUNT)).wrapping_mul(2)).wrapping_sub(if compat::get_state(&FIRST_DEATH_HAS_SPIRIT_PET) { 1 } else { 0 });
    }

    /// java: private static int getPuzzleCount()
    pub fn get_puzzle_count() -> i32 {
        let mut matcher: Option<compat::regex::Matcher> = PlayerListManager::regex_at(47, &PUZZLE_COUNT_PATTERN);
        return (if matcher.is_some() { matcher.as_mut().unwrap().group_named("count").unwrap().parse::<i32>().expect("NumberFormatException") } else { 0 });
    }

    /// java: private static int getPuzzlePenalty()
    pub fn get_puzzle_penalty() -> i32 {
        let mut incomplete_puzzles: i32 = 0;
        for index in 0..compat::get_state(&PUZZLE_COUNT) {
            let Some(mut puzzle_matcher) = PlayerListManager::regex_at(48i32.wrapping_add(index), &PUZZLES_PATTERN) else {
                break;
            };
            if compat::regex::Pattern::matches("[✖✦]", &puzzle_matcher.group_named("state").unwrap()) {
                incomplete_puzzles += 1;
            }
        }
        return incomplete_puzzles.wrapping_mul(10);
    }

    /// java: private static double getSecretsPercentage()
    pub fn get_secrets_percentage() -> f64 {
        let mut matcher: Option<compat::regex::Matcher> = PlayerListManager::regex_at(44, &SECRETS_PATTERN);
        return (if matcher.is_some() { matcher.as_mut().unwrap().group_named("secper").unwrap().trim().parse::<f64>().expect("NumberFormatException") } else { ((0) as f64) as f64 });
    }

    /// java: private static int getCrypts()
    pub fn get_crypts() -> i32 {
        let mut matcher: Option<compat::regex::Matcher> = PlayerListManager::regex_at(33, &CRYPTS_PATTERN);
        if matcher.is_none() {
            matcher = PlayerListManager::regex_at(32, &CRYPTS_PATTERN);
        }
        return (if matcher.is_some() { matcher.as_mut().unwrap().group_named("crypts").unwrap().parse::<i32>().expect("NumberFormatException") } else { 0 });
    }

    /// java: private static boolean hasSpiritPet(@Nullable JsonObject player, String name)
    pub fn has_spirit_pet(mut player: Option<serde_json::Map<String, serde_json::Value>>, name: &str) -> bool {
        let Some(mut player) = player.clone() else {
            logger().error(&format!("[Skyblocker] Spirit pet lookup by name failed! (likely due to an earlier error!) Name: {}", name));
            return false;
        };
        // java: try {..} catch (Exception e) - an exception in the body is a panic, caught by `catch_unwind`
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            for pet in (compat::json::get_as_json_array_of(&(compat::json::get_as_json_object_of(&player, "pets_data")), "pets")).iter() {
                if !(compat::json::get_as_string(&compat::json::get_as_json_object(&pet).get("type").cloned().unwrap()) == "SPIRIT") {
                    continue;
                }
                if !(compat::json::get_as_string(&compat::json::get_as_json_object(&pet).get("tier").cloned().unwrap()) == "LEGENDARY") {
                    continue;
                }
                return true;
            }
            false
        })) {
            Ok(true) => return true,
            Ok(false) => {}
            Err(e) => {
                logger().error(&format!("[Skyblocker] Spirit pet lookup by name failed! Name: {} {}", name, compat::panic_message(&e)));
            }
        }
        return false;
    }

    /// java: private static void checkMessageForDeaths(String message)
    pub fn check_message_for_deaths(message: &str) {
        if !(compat::starts_with_at(&message, "☠", 1)) {
            return;
        }
        let mut matcher: compat::regex::Matcher = DEATHS_PATTERN.matcher(message);
        if !matcher.matches() {
            return;
        }
        compat::set_state(&DEATH_COUNT, compat::get_state(&DEATH_COUNT) + 1);
        if compat::get_state(&DEATH_COUNT) > 1 {
            return;
        }
        //   java: final String whoDied = matcher.group("whodied").transform(s -> {
        //   java: 			if (s.equals("You")) return Minecraft.getInstance().getUser().getName(); //This will be wrong if the dead player is called 'You' but that's unlikely
        //   java: 			else return s;
        //   java: 		});
        let who_died: String = {
            let s = matcher.group_named("whodied").unwrap_or_default();
            if s == "You" { compat::minecraft().get_user().get_name().to_string() } else { s }
        };
        //   java: ProfileUtils.fetchProfileMember(whoDied).thenAccept(player -> firstDeathHasSpiritPet = hasSpiritPet(player, whoDied));
        if let Some(future) = ProfileUtils::fetch_profile_member(&who_died) {
            future.then_accept(move |player| compat::set_state(&FIRST_DEATH_HAS_SPIRIT_PET, DungeonScore::has_spirit_pet(player.clone(), &who_died)));
        }
    }

    /// java: private static void checkMessageForWatcher(String message)
    pub fn check_message_for_watcher(message: &str) {
        if message == "[BOSS] The Watcher: You have proven yourself. You may pass." {
            ({ let __r = scheduler::INSTANCE.get_mut(); let __f = compat::dispatch(__r, |__t| __t.schedule()); __f(__r, Arc::new(move || {
                compat::set_state(&BLOOD_ROOM_COMPLETED, true);
                DungeonScore::tick();
            }), (20i32.wrapping_mul(5)).wrapping_add(10)) });
        }
    }

    /// java: private static void checkMessageForMimic(String message)
    pub fn check_message_for_mimic(message: &str) {
        if !(MIMIC_PATTERN.matcher(message).matches()) {
            return;
        }
        DungeonScore::on_mimic_kill();
    }

    /// java: private static void checkMessageForPrince(String message)
    pub fn check_message_for_prince(message: &str) {
        if !(PRINCE_PATTERN.matcher(message).matches()) && !(message == HYPIXEL_PRINCE_KILL_MESSAGE) {
            return;
        }
        DungeonScore::on_prince_kill((message == HYPIXEL_PRINCE_KILL_MESSAGE));
    }

    /// java: private static void checkMessageForBat(String message)
    pub fn check_message_for_bat(message: &str) {
        if !(BAT_PATTERN.matcher(message).matches()) && !(message == HYPIXEL_BAT_KILL_MESSAGE) {
            return;
        }
        DungeonScore::on_bat_kill((message == HYPIXEL_BAT_KILL_MESSAGE));
    }

    /// java: public static void setCurrentFloor()
    pub fn set_current_floor() {
        for sidebar_line in compat::get_state(&utils::STRING_SCOREBOARD) {
            let mut floor_matcher: compat::regex::Matcher = FLOOR_PATTERN.matcher(&sidebar_line);
            if !floor_matcher.matches() {
                continue;
            }
            compat::set_state(&CURRENT_FLOOR, floor_matcher.group_named("floor").unwrap());
            return;
        }
        logger().error("[Skyblocker] Floor pattern doesn't match!");
        compat::set_state(&CURRENT_FLOOR, FloorRequirement::None.name().to_string());
    }

    /// java: public static int getScore()
    pub fn get_score() -> i32 {
        return compat::get_state(&SCORE);
    }

    /// java: public static boolean isDungeonStarted()
    pub fn is_dungeon_started() -> bool {
        return compat::get_state(&DUNGEON_STARTED);
    }

    /// java: public static boolean isMimicOnCurrentFloor()
    pub fn is_mimic_on_current_floor() -> bool {
        return compat::get_state(&FLOOR_HAS_MIMICS);
    }

}

/// java: enum FloorRequirement
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FloorRequirement {
    E,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    M1,
    M2,
    M3,
    M4,
    M5,
    M6,
    M7,
    None,
}

impl FloorRequirement {
    pub const VALUES: [FloorRequirement; 16] = [FloorRequirement::E, FloorRequirement::F1, FloorRequirement::F2, FloorRequirement::F3, FloorRequirement::F4, FloorRequirement::F5, FloorRequirement::F6, FloorRequirement::F7, FloorRequirement::M1, FloorRequirement::M2, FloorRequirement::M3, FloorRequirement::M4, FloorRequirement::M5, FloorRequirement::M6, FloorRequirement::M7, FloorRequirement::None];

    /// java: values()
    pub fn values() -> Vec<FloorRequirement> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            FloorRequirement::E => "E",
            FloorRequirement::F1 => "F1",
            FloorRequirement::F2 => "F2",
            FloorRequirement::F3 => "F3",
            FloorRequirement::F4 => "F4",
            FloorRequirement::F5 => "F5",
            FloorRequirement::F6 => "F6",
            FloorRequirement::F7 => "F7",
            FloorRequirement::M1 => "M1",
            FloorRequirement::M2 => "M2",
            FloorRequirement::M3 => "M3",
            FloorRequirement::M4 => "M4",
            FloorRequirement::M5 => "M5",
            FloorRequirement::M6 => "M6",
            FloorRequirement::M7 => "M7",
            FloorRequirement::None => "NONE",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> FloorRequirement {
        match name {
            "E" => FloorRequirement::E,
            "F1" => FloorRequirement::F1,
            "F2" => FloorRequirement::F2,
            "F3" => FloorRequirement::F3,
            "F4" => FloorRequirement::F4,
            "F5" => FloorRequirement::F5,
            "F6" => FloorRequirement::F6,
            "F7" => FloorRequirement::F7,
            "M1" => FloorRequirement::M1,
            "M2" => FloorRequirement::M2,
            "M3" => FloorRequirement::M3,
            "M4" => FloorRequirement::M4,
            "M5" => FloorRequirement::M5,
            "M6" => FloorRequirement::M6,
            "M7" => FloorRequirement::M7,
            "NONE" => FloorRequirement::None,
            _ => panic!("IllegalArgumentException: No enum constant FloorRequirement.{name}"),
        }
    }

    /// the constant's constructor data (Java keeps it in the constant's fields)
    pub fn fields(self) -> &'static FloorRequirementFields {
        static FIELDS: LazyLock<Vec<FloorRequirementFields>> = LazyLock::new(|| vec![
            FloorRequirementFields::new(30, 1200),
            FloorRequirementFields::new(30, 600),
            FloorRequirementFields::new(40, 600),
            FloorRequirementFields::new(50, 600),
            FloorRequirementFields::new(60, 720),
            FloorRequirementFields::new(70, 600),
            FloorRequirementFields::new(85, 720),
            FloorRequirementFields::new(100, 840),
            FloorRequirementFields::new(100, 480),
            FloorRequirementFields::new(100, 480),
            FloorRequirementFields::new(100, 480),
            FloorRequirementFields::new(100, 480),
            FloorRequirementFields::new(100, 480),
            FloorRequirementFields::new(100, 600),
            FloorRequirementFields::new(100, 840),
            FloorRequirementFields::new(0, 0),
        ]);
        &FIELDS[self as usize]
    }

}

impl std::fmt::Display for FloorRequirement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// the constructor data of each FloorRequirement constant
pub struct FloorRequirementFields {
    pub percentage: i32,
    pub time_limit: i32,
}

impl FloorRequirementFields {
    fn new(mut percentage: i32, mut time_limit: i32) -> Self {
        let mut this = Self {
            percentage: 0,
            time_limit: 0,
        };
        this.percentage = percentage;
        this.time_limit = time_limit;
        this
    }

}

//! Ported from de.hysky.skyblocker.skyblock.dungeon.secrets.SecretsTracker by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::events::dungeon_events;
use crate::de::hysky::skyblocker::skyblock::dungeon::dungeon_class::DungeonClass;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::dungeon_player_manager::DungeonPlayerManager;
use crate::de::hysky::skyblocker::utils::api_utils::ApiUtils;
use crate::de::hysky::skyblocker::utils::constants;
use crate::entry::skyblocker_mod;
use mcrust::java::util::concurrent::completable_future::CompletableFuture;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::hover_event::HoverEvent;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::world::entity::player::player::Player;

/// java: private static volatile @Nullable TrackedRun currentRun = null;
pub static CURRENT_RUN: LazyLock<Mutex<Option<TrackedRun>>> = LazyLock::new(|| Mutex::new(None));
/// java: private static volatile @Nullable TrackedRun lastRun = null;
pub static LAST_RUN: LazyLock<Mutex<Option<TrackedRun>>> = LazyLock::new(|| Mutex::new(None));
/// java: private static volatile long lastRunEnded = 0L;
pub static LAST_RUN_ENDED: LazyLock<Mutex<i64>> = LazyLock::new(|| Mutex::new(0));
/// java: private static final SecretData EMPTY = new SecretData(0, false, 0);
pub static SECRET_DATA_EMPTY: LazyLock<SecretData> = LazyLock::new(|| SecretData::new(0, false, 0));

/// java: public class SecretsTracker
#[derive(Clone)]
pub struct SecretsTracker {}

impl SecretsTracker {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        dungeon_events::DUNGEON_STARTED.register(Arc::new(move || { SecretsTracker::calculate(RunPhase::Start); }));
        dungeon_events::DUNGEON_ENDED.register(Arc::new(move || { SecretsTracker::calculate(RunPhase::End); }));
    }

    /// java: private static void calculate(RunPhase phase)
    pub fn calculate(mut phase: RunPhase) {
        if !(SkyblockerConfigManager::get().dungeons.player_secrets_tracker) {
            return;
        }
        match phase {
            RunPhase::Start => {
                CompletableFuture::<()>::run_async(move || (|| {
                    let mut newly_started_run: TrackedRun = TrackedRun::new();
                    for i in 0..5i32 {
                        let mut player_name: String = SecretsTracker::get_player_name_at(i.wrapping_add(1));
                        if !player_name.is_empty() {
                            let mut previous_run: Option<TrackedRun> = compat::get_state(&LAST_RUN);
                            // previousRun.playersSecretData().getOrDefault(playerName, SecretData.EMPTY) != SecretData.EMPTY - Java
                            // compares with the EMPTY instance; the port's records are values, so a stored entry equal to EMPTY
                            // (only ever stored as EMPTY itself, by getPlayerSecrets' failure path - or a fetch of 0 / not cached / age 0) counts as it
                            if previous_run.is_some() && compat::millis() <= (compat::get_state(&LAST_RUN_ENDED)).wrapping_add(300000 as i64) && previous_run.as_ref().unwrap().players_secret_data.get(&player_name).is_some_and(|data| *data != *SECRET_DATA_EMPTY) {
                                newly_started_run.players_secret_data.insert(player_name.clone(), previous_run.as_ref().unwrap().players_secret_data.get(&player_name).cloned().unwrap());
                            } else {
                                newly_started_run.players_secret_data.insert(player_name.clone(), SecretsTracker::get_player_secrets(&player_name));
                            }
                        }
                    }
                    compat::set_state(&CURRENT_RUN, Some(newly_started_run.clone()));
                })(), compat::executor);
            }
            RunPhase::End => {
                CompletableFuture::<()>::run_async(move || (|| {
                    let mut this_run: Option<TrackedRun> = compat::get_state(&CURRENT_RUN);
                    if let Some(mut this_run) = this_run {
                        let mut secrets_found: std::collections::HashMap<String, SecretData> = std::collections::HashMap::new();
                        // for (Map.Entry<String, SecretData> entry : thisRun.playersSecretData().entrySet()) - `entry.setValue`
                        // writes the run's own map
                        let player_names: Vec<String> = this_run.players_secret_data.keys().cloned().collect();
                        for player_name in player_names {
                            let starting_secrets: SecretData = this_run.players_secret_data.get(&player_name).cloned().unwrap();
                            let mut secrets_now: SecretData = SecretsTracker::get_player_secrets(&player_name);
                            let mut secrets_player_found: i32 = secrets_now.secrets().wrapping_sub(starting_secrets.secrets());
                            secrets_found.insert(player_name.clone(), secrets_now.updated(secrets_player_found, (starting_secrets.cached() || secrets_now.cached())));
                            this_run.players_secret_data.insert(player_name.clone(), secrets_now.clone());
                        }
                        for entry in secrets_found.iter().map(|(k, v)| (k.clone(), v.clone())) {
                            mcrust::mcrust::modding::game::execute(move || { SecretsTracker::send_result_message(&entry.0, &entry.1); });
                        }
                        compat::set_state(&LAST_RUN_ENDED, compat::millis());
                        compat::set_state(&LAST_RUN, Some(this_run.clone()));
                        compat::set_state(&CURRENT_RUN, None);
                    } else {
                        mcrust::mcrust::modding::game::execute(move || { SecretsTracker::send_failure_message(); });
                    }
                })(), compat::executor);
            }
            _ => {}
        }
    }

    /// java: private static void sendResultMessage(String player, SecretData secretData)
    pub fn send_result_message(player: &str, secret_data: &SecretData) {
        if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() {
            return;
        }
        let mut dungeon_class: DungeonClass = DungeonPlayerManager::get_class_from_player_string(player);
        mcrust::mcrust::modding::game::with_player(|mc_player| { script_host::send_feedback(&(constants::PREFIX.get_mut())().append(component::translatable_args("skyblocker.dungeons.secretsTracker.feedback", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from((component::literal(player).append_str(&format!(" ({})", dungeon_class.display_name()))).with_color((if dungeon_class != DungeonClass::Unknown { dungeon_class.color() } else { 0xF57542 }))), mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::literal(&secret_data.secrets().to_string()).with_color_text(mcrust::net::minecraft::network::chat::text_color::TextColor::GRAY)), mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(SecretsTracker::get_cache_text(secret_data.cached(), secret_data.cache_age()))])).to_string()); });
    }

    /// java: private static void sendFailureMessage()
    pub fn send_failure_message() {
        if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() {
            return;
        }
        mcrust::mcrust::modding::game::with_player(|player| { script_host::send_feedback(&(constants::PREFIX.get_mut())().append(component::translatable("skyblocker.dungeons.secretsTracker.failFeedback").with_color_text(mcrust::net::minecraft::network::chat::text_color::TextColor::RED)).to_string()); });
    }

    /// java: private static Component getCacheText(boolean cached, int cacheAge)
    pub fn get_cache_text(mut cached: bool, mut cache_age: i32) -> MutableComponent {
        return component::literal("ℹ").with_style_fn(|style| (style.with_color((if cached { 0xEAC864 } else { 0x218BFF }))).with_hover_event(Some(HoverEvent::new_show_text(mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from((if cached { component::translatable_args("skyblocker.api.cache.HIT", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of(cache_age.to_string())]) } else { component::translatable("skyblocker.api.cache.MISS") }))))));
    }

    /// java: private static String getPlayerNameAt(int index)
    pub fn get_player_name_at(mut index: i32) -> String {
        let mut matcher: Option<compat::regex::Matcher> = DungeonPlayerManager::get_player_from_tab(index);
        return (if matcher.is_some() { matcher.as_mut().unwrap().group_named("name") } else { Some("".to_string()) }).unwrap();
    }

    /// java: private static SecretData getPlayerSecrets(String name)
    pub fn get_player_secrets(name: &str) -> SecretData {
        let mut uuid: String = ApiUtils::name2_uuid(name);
        if !uuid.is_empty() {
            {
                // java: try (ApiResponse response = Http.sendHypixelRequest("player", "?uuid=" + uuid)) {
                //   return new SecretData(getSecretCountFromAchievements(JsonParser.parseString(response.content()).getAsJsonObject()), response.cached(), response.age());
                // } catch (Exception e) {..} - an exception here is a panic, caught by `catch_unwind`; the body's `return` is the closure's value
                match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    let mut response: crate::de::hysky::skyblocker::utils::http::ApiResponse = crate::de::hysky::skyblocker::utils::http::Http::send_hypixel_request("player", &format!("?uuid={}", uuid));
                    let parsed: serde_json::Value = serde_json::from_str(&response.content()).expect("JsonParseException");
                    let __v = SecretData::new(SecretsTracker::get_secret_count_from_achievements(&compat::json::get_as_json_object(&parsed)), response.cached(), response.age());
                    response.close();
                    __v
                })) {
                    Ok(__v) => return __v,
                    Err(e) => {
                        logger().error(&format!("[Skyblocker] Encountered an error while trying to fetch {} secret count!", format!("{}'s", name)));
                    }
                }
            }
        }
        return SECRET_DATA_EMPTY.clone();
    }

    /// java: private static int getSecretCountFromAchievements(JsonObject playerJson)
    pub fn get_secret_count_from_achievements(player_json: &serde_json::Map<String, serde_json::Value>) -> i32 {
        let mut player: serde_json::Map<String, serde_json::Value> = compat::json::get_as_json_object_of(&player_json, "player");
        let mut achievements: Option<serde_json::Map<String, serde_json::Value>> = (if compat::json::has(&player, "achievements") { Some(compat::json::get_as_json_object_of(&player, "achievements")) } else { None });
        return (if achievements.is_some() && compat::json::has(&achievements.as_ref().unwrap(), "skyblock_treasure_hunter") { compat::json::get_as_int(&achievements.as_ref().unwrap().get("skyblock_treasure_hunter").cloned().unwrap()) } else { 0 });
    }

}

/// java: private record TrackedRun(Object2ObjectOpenHashMap<String, SecretData> playersSecretData)
#[derive(Clone, Debug, PartialEq)]
pub struct TrackedRun {
    /// java: Object2ObjectOpenHashMap<String, SecretData> playersSecretData
    pub players_secret_data: std::collections::HashMap<String, SecretData>,
}

impl TrackedRun {
    /// java: private TrackedRun()
    pub fn new() -> Self {
        let mut this = Self::new_1(std::collections::HashMap::new());
        this
    }

    /// java: the canonical record constructor
    /// (the canonical record constructor is `new_N` here: Rust has no overloading and `new` is the explicit one)
    pub fn new_1(mut players_secret_data: std::collections::HashMap<String, SecretData>) -> Self {
        Self { players_secret_data }
    }

    /// java: the record accessor `playersSecretData()`
    pub fn players_secret_data(&self) -> std::collections::HashMap<String, SecretData> {
        self.players_secret_data.clone()
    }

}

/// java: private record SecretData(int secrets, boolean cached, int cacheAge)
#[derive(Clone, Debug, PartialEq)]
pub struct SecretData {
    /// java: int secrets
    pub secrets: i32,
    /// java: boolean cached
    pub cached: bool,
    /// java: int cacheAge
    pub cache_age: i32,
}

impl SecretData {
    /// java: the canonical record constructor
    pub fn new(mut secrets: i32, mut cached: bool, mut cache_age: i32) -> Self {
        Self { secrets, cached, cache_age }
    }

    /// java: the record accessor `secrets()`
    pub fn secrets(&self) -> i32 {
        self.secrets
    }

    /// java: the record accessor `cached()`
    pub fn cached(&self) -> bool {
        self.cached
    }

    /// java: the record accessor `cacheAge()`
    pub fn cache_age(&self) -> i32 {
        self.cache_age
    }

    /// java: private SecretData updated(int secrets, boolean cached)
    pub fn updated(&self, mut secrets: i32, mut cached: bool) -> SecretData {
        return SecretData::new(secrets, cached, self.cache_age);
    }

}

/// java: private enum RunPhase
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RunPhase {
    Start,
    End,
}

impl RunPhase {
    pub const VALUES: [RunPhase; 2] = [RunPhase::Start, RunPhase::End];

    /// java: values()
    pub fn values() -> Vec<RunPhase> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            RunPhase::Start => "START",
            RunPhase::End => "END",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> RunPhase {
        match name {
            "START" => RunPhase::Start,
            "END" => RunPhase::End,
            _ => panic!("IllegalArgumentException: No enum constant RunPhase.{name}"),
        }
    }

}

impl std::fmt::Display for RunPhase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

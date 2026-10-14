//! Ported from de.hysky.skyblocker.utils.Utils by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::events::skyblock_events;
use crate::de::hysky::skyblocker::skyblock::slayers::slayer_manager::SlayerManager;
use crate::de::hysky::skyblocker::utils::area;
use crate::de::hysky::skyblocker::utils::area::Area;
use crate::de::hysky::skyblocker::utils::location::Location;
use crate::de::hysky::skyblocker::utils::purse::purse_change_cause::PurseChangeCause;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler;
use crate::de::hysky::skyblocker::utils::sky_block_icons;
use crate::entry::skyblocker_mod;
use mcrust::com::mojang::blaze3d::blaze3d;
use mcrust::com::mojang::util::undashed_uuid;
use mcrust::java::text::decimal_format_symbols::Locale;
use mcrust::java::util::uuid::Uuid;
use mcrust::net::minecraft::chat_formatting;
use mcrust::net::minecraft::client::minecraft;
use mcrust::net::minecraft::client::player::local_player::LocalPlayer;
use mcrust::net::minecraft::core::block_pos::BlockPos;
use mcrust::net::minecraft::core::holder::Holder;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::server::packs::resources::resource::Resource;
use mcrust::net::minecraft::world::entity::player::player::Player;
use mcrust::net::minecraft::world::level::biome::biome::Biome;
use mcrust::net::minecraft::world::level::level::Level;
use mcrust::net::minecraft::world::scores::display_slot::DisplaySlot;
use mcrust::net::minecraft::world::scores::objective::Objective;
use mcrust::net::minecraft::world::scores::player_team::PlayerTeam;
use mcrust::net::minecraft::world::scores::scoreboard::Scoreboard;
use mcrust::net::minecraft::network::chat::component::Component as _;
use mcrust::net::minecraft::world::entity::entity::Entity as _;
use mcrust::net::minecraft::world::level::level_height_accessor::LevelHeightAccessor as _;
use mcrust::net::minecraft::world::scores::score_holder::ScoreHolder as _;

/// java: private static final String ALTERNATE_HYPIXEL_ADDRESS = System.getProperty("skyblocker.alternateHypixelAddress", "");
pub static ALTERNATE_HYPIXEL_ADDRESS: LazyLock<String> = LazyLock::new(|| std::env::var("skyblocker.alternateHypixelAddress").ok().unwrap());
/// java: public static final String HYPIXEL_NAMESPACE = "hypixel";
pub const HYPIXEL_NAMESPACE: &str = "hypixel";
/// java: public static final String HYPIXEL_SKYBLOCK_NAMESPACE = "hypixel_skyblock";
pub const HYPIXEL_SKYBLOCK_NAMESPACE: &str = "hypixel_skyblock";
/// java: private static final String PROFILE_PREFIX = "Profile: ";
pub const PROFILE_PREFIX: &str = "Profile: ";
/// java: private static final String PROFILE_MESSAGE_PREFIX = "§aYou are playing on profile: §e";
pub const PROFILE_MESSAGE_PREFIX: &str = "§aYou are playing on profile: §e";
/// java: public static final String PROFILE_ID_PREFIX = "Profile ID: ";
pub const PROFILE_ID_PREFIX: &str = "Profile ID: ";
/// java: private static final String PROFILE_ID_SUGGEST_PREFIX = "CLICK THIS TO SUGGEST IT IN CHAT";
pub const PROFILE_ID_SUGGEST_PREFIX: &str = "CLICK THIS TO SUGGEST IT IN CHAT";
/// java: private static final String AREA_ICON_REGEX = String.format("[%s%s]", SkyBlockIcons.AREA, SkyBlockIcons.RIFT_AREA);
pub static AREA_ICON_REGEX: LazyLock<String> = LazyLock::new(|| format!("[{}{}]", sky_block_icons::AREA, sky_block_icons::RIFT_AREA));
/// java: private static final Pattern PURSE = Pattern.compile("(Purse|Piggy): (?<purse>[0-9,.]+)( \\((?<change>[+\\-][0-9,.]+)\\))?");
pub static PURSE: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("(Purse|Piggy): (?<purse>[0-9,.]+)( \\((?<change>[+\\-][0-9,.]+)\\))?"));
/// java: private static boolean isOnHypixel = false;
pub static IS_ON_HYPIXEL: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));
/// java: private static boolean isOnSkyblock = false;
pub static IS_ON_SKYBLOCK: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));
/// java: private static boolean isGuest = false;
pub static IS_GUEST: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));
/// java: private static RankType rank = PackageRank.NONE;
pub static RANK: compat::MainThread<crate::unresolved::RankType> = compat::MainThread::new(|| crate::unresolved::RankType::Package(crate::unresolved::PackageRank::None));
/// java: private static Location location = Location.UNKNOWN;
pub static LOCATION: LazyLock<Mutex<Location>> = LazyLock::new(|| Mutex::new(Location::Unknown));
/// java: private static Area area = Area.UNKNOWN;
pub static AREA: compat::MainThread<compat::Shared<dyn Area>> = compat::MainThread::new(|| area::UNKNOWN.get_mut().clone());
/// java: private static String profile = "";
pub static PROFILE: LazyLock<Mutex<String>> = LazyLock::new(|| Mutex::new("".to_string()));
/// java: private static String profileId = "";
pub static PROFILE_ID: LazyLock<Mutex<String>> = LazyLock::new(|| Mutex::new("".to_string()));
/// java: private static int profileIdRequest = 0;
pub static PROFILE_ID_REQUEST: LazyLock<Mutex<i32>> = LazyLock::new(|| Mutex::new(0));
/// java: private static int profileSuggestionMessages = Integer.MAX_VALUE / 2;
pub static PROFILE_SUGGESTION_MESSAGES: LazyLock<Mutex<i32>> = LazyLock::new(|| Mutex::new((i32::MAX).wrapping_div(2)));
/// java: @SuppressWarnings("JavadocDeclaration") private static Environment environment = Environment.PRODUCTION;
pub static ENVIRONMENT: compat::MainThread<crate::unresolved::Environment> = compat::MainThread::new(|| crate::unresolved::Environment::Production);
/// java: private static String server = "";
pub static SERVER: LazyLock<Mutex<String>> = LazyLock::new(|| Mutex::new("".to_string()));
/// java: private static String gameType = "";
pub static GAME_TYPE: LazyLock<Mutex<String>> = LazyLock::new(|| Mutex::new("".to_string()));
/// java: private static String locationRaw = "";
pub static LOCATION_RAW: LazyLock<Mutex<String>> = LazyLock::new(|| Mutex::new("".to_string()));
/// java: private static String map = "";
pub static MAP: LazyLock<Mutex<String>> = LazyLock::new(|| Mutex::new("".to_string()));
/// java: private static @Nullable Holder<Biome> biome = null;
pub static BIOME: compat::MainThread<Option<Holder>> = compat::MainThread::new(|| None);
/// java: public static double purse = 0;
pub static PURSE_2: LazyLock<Mutex<f64>> = LazyLock::new(|| Mutex::new(((0) as f64) as f64));
/// java: public static final ObjectArrayList<Component> TEXT_SCOREBOARD = new ObjectArrayList<>();
pub static TEXT_SCOREBOARD: compat::MainThread<Vec<MutableComponent>> = compat::MainThread::new(|| Vec::new());
/// java: public static final ObjectArrayList<String> STRING_SCOREBOARD = new ObjectArrayList<>();
pub static STRING_SCOREBOARD: LazyLock<Mutex<Vec<String>>> = LazyLock::new(|| Mutex::new(Vec::new()));

/// java: public class Utils
#[derive(Clone)]
pub struct Utils {}

impl Utils {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static boolean isOnHypixel()
    pub fn is_on_hypixel() -> bool {
        return compat::get_state(&IS_ON_HYPIXEL);
    }

    /// java: public static boolean isOnSkyblock()
    pub fn is_on_skyblock() -> bool {
        return compat::get_state(&IS_ON_SKYBLOCK);
    }

    /// java: public static boolean isInDungeons()
    pub fn is_in_dungeons() -> bool {
        return (compat::get_state(&LOCATION) == Location::Dungeon);
    }

    /// java: public static boolean isInCrystalHollows()
    pub fn is_in_crystal_hollows() -> bool {
        return (compat::get_state(&LOCATION) == Location::CrystalHollows);
    }

    /// java: public static boolean isInDwarvenMines()
    pub fn is_in_dwarven_mines() -> bool {
        return (compat::get_state(&LOCATION) == Location::DwarvenMines || compat::get_state(&LOCATION) == Location::GlaciteMineshafts);
    }

    /// java: public static boolean isInTheRift()
    pub fn is_in_the_rift() -> bool {
        return (compat::get_state(&LOCATION) == Location::TheRift);
    }

    /// java: public static boolean isInGarden()
    pub fn is_in_garden() -> bool {
        return (compat::get_state(&LOCATION) == Location::Garden);
    }

    /// java: public static boolean isInTheEnd()
    pub fn is_in_the_end() -> bool {
        return (compat::get_state(&LOCATION) == Location::TheEnd);
    }

    /// java: public static boolean isInKuudra()
    pub fn is_in_kuudra() -> bool {
        return (compat::get_state(&LOCATION) == Location::KuudrasHollow);
    }

    /// java: public static boolean isInCrimson()
    pub fn is_in_crimson() -> bool {
        return (compat::get_state(&LOCATION) == Location::CrimsonIsle);
    }

    /// java: public static boolean isInSpidersDen()
    pub fn is_in_spiders_den() -> bool {
        return (compat::get_state(&LOCATION) == Location::SpidersDen);
    }

    /// java: public static boolean isInFarm()
    pub fn is_in_farm() -> bool {
        return (compat::get_state(&LOCATION) == Location::TheFarmingIsland);
    }

    /// java: public static boolean isInGalatea()
    pub fn is_in_galatea() -> bool {
        return (compat::get_state(&LOCATION) == Location::Galatea);
    }

    /// java: public static boolean isInTorrhusCanyon()
    pub fn is_in_torrhus_canyon() -> bool {
        return (compat::get_state(&LOCATION) == Location::TorrhusCanyon);
    }

    /// java: public static boolean isInForagingIsland()
    pub fn is_in_foraging_island() -> bool {
        return (Utils::is_in_galatea() || Utils::is_in_torrhus_canyon() || Utils::is_in_park());
    }

    /// java: public static boolean isInSafari()
    pub fn is_in_safari() -> bool {
        return (compat::get_state(&LOCATION) == Location::Safari);
    }

    /// java: public static boolean isInHub()
    pub fn is_in_hub() -> bool {
        return (compat::get_state(&LOCATION) == Location::Hub);
    }

    /// java: public static boolean isInPrivateIsland()
    pub fn is_in_private_island() -> bool {
        return (compat::get_state(&LOCATION) == Location::PrivateIsland);
    }

    /// java: public static boolean isInPark()
    pub fn is_in_park() -> bool {
        return (compat::get_state(&LOCATION) == Location::ThePark);
    }

    /// java: public static boolean isInBiome(Identifier targetBiome)
    pub fn is_in_biome(target_biome: &Identifier) -> bool {
        // java: return biome != null && biome.is(targetBiome);
        return BIOME.get().as_ref().is_some_and(|biome| biome.is_identifier(target_biome));
    }

    /// java: public static boolean isOnBingo()
    pub fn is_on_bingo() -> bool {
        return compat::lock(&PROFILE).ends_with("Ⓑ");
    }

    /// java: public static boolean isGuest()
    pub fn is_guest() -> bool {
        return compat::get_state(&IS_GUEST);
    }

    /// java: public static String getProfile()
    pub fn get_profile() -> String {
        return compat::get_state(&PROFILE);
    }

    /// java: public static String getProfileId()
    pub fn get_profile_id() -> String {
        return compat::get_state(&PROFILE_ID);
    }

    /// java: public static Location getLocation()
    pub fn get_location() -> Location {
        return compat::get_state(&LOCATION);
    }

    /// java: @VisibleForTesting public static void setTestLocation(Location located)
    pub fn set_test_location(mut located: Location) {
        compat::set_state(&LOCATION, located);
    }

    /// java: public static Area getArea()
    pub fn get_area() -> compat::Shared<dyn Area> {
        return AREA.get_mut().clone();
    }

    /// java: public static Environment getEnvironment()
    pub fn get_environment() -> &'static crate::unresolved::Environment {
        // java: return environment; - the object itself
        return ENVIRONMENT.get();
    }

    /// java: public static String getServer()
    pub fn get_server() -> String {
        return compat::get_state(&SERVER);
    }

    /// java: public static String getGameType()
    pub fn get_game_type() -> String {
        return compat::get_state(&GAME_TYPE);
    }

    /// java: public static String getLocationRaw()
    pub fn get_location_raw() -> String {
        return compat::get_state(&LOCATION_RAW);
    }

    /// java: public static String getMap()
    pub fn get_map() -> String {
        return compat::get_state(&MAP);
    }

    /// java: public static RankType getRank()
    pub fn get_rank() -> &'static crate::unresolved::RankType {
        // java: return rank; - the object itself
        return RANK.get();
    }

    /// java: @Init public static void init()
    pub fn init() {
        // TODO(fabric2seer): ClientReceiveMessageEvents.ALLOW_GAME.register: no SeerClient equivalent yet
        //   java: ClientReceiveMessageEvents.ALLOW_GAME.register(Utils::onChatMessage);
        client_play_connection_events::DISCONNECT.register(Arc::new(|listener: &mut ClientPacketListener| {
            Utils::on_disconnect();
        }));
        // java: HypixelNetworking.registerToEvents(Util.make(new Object2IntOpenHashMap<>(), map -> map.put(LocationUpdateS2CPacket.ID, 1)));
        crate::net::azureaaron::hmapi::network::hypixel_networking::HypixelNetworking::register_to_events(&[(crate::net::azureaaron::hmapi::network::packet::v1::s2c::location_update_s2c_packet::LocationUpdateS2CPacket::ID, 1)]);
        // java: HypixelPacketEvents.HELLO.register(Utils::onPacket); (and LOCATION_UPDATE, PLAYER_INFO)
        use crate::net::azureaaron::hmapi::events::hypixel_packet_events;
        hypixel_packet_events::HELLO.get_mut().register(Arc::new(Utils::on_packet));
        hypixel_packet_events::LOCATION_UPDATE.get_mut().register(Arc::new(Utils::on_packet));
        hypixel_packet_events::PLAYER_INFO.get_mut().register(Arc::new(Utils::on_packet));
    }

    /// java: public static void update()
    pub fn update() {
        let client = compat::minecraft();
        Utils::update_scoreboard(client);
        Utils::update_player_presence(client);
        Utils::update_from_player_list(client);
        Utils::update_biome(client);
    }

    /// java: private static void updatePlayerPresence(Minecraft client)
    pub fn update_player_presence(client: &mut mcrust::net::minecraft::client::minecraft::Minecraft) {
        if mcrust::mcrust::modding::game::level().is_none() || client.is_local_server() {
            if false {
                compat::set_state(&IS_ON_SKYBLOCK, true);
            }
        }
        if false || Utils::is_connected_to_hypixel(client) {
            if !(compat::get_state(&IS_ON_HYPIXEL)) {
                compat::set_state(&IS_ON_HYPIXEL, true);
            }
        } else if compat::get_state(&IS_ON_HYPIXEL) {
            compat::set_state(&IS_ON_HYPIXEL, false);
        }
    }

    /// java: private static boolean isConnectedToHypixel(Minecraft client)
    pub fn is_connected_to_hypixel(client: &mut mcrust::net::minecraft::client::minecraft::Minecraft) -> bool {
        // java: client.getCurrentServer() - `getConnection()`'s server data, through `compat::get_connection` (see there)
        let mut server_address: String = compat::get_connection(client).and_then(|connection| connection.server_data.clone()).map(|server| server.ip.to_lowercase()).unwrap_or_else(|| "".to_string());
        // java: client.player != null && client.player.connection != null && client.player.connection.serverBrand() != null
        //   ? client.player.connection.serverBrand() : ""
        // The port publishes the connection's brand beside the play listener (`PlayListenerSinks.server_brand`);
        // `get_connection` answers None without a player, as Java's first two guards do.
        let mut server_brand: String = compat::get_connection(client)
            .and_then(|sinks| sinks.server_brand.lock().unwrap_or_else(|p| p.into_inner()).clone())
            .unwrap_or_else(|| "".to_string());
        return (!server_address.is_empty() && compat::equals_ignore_case(&server_address, &ALTERNATE_HYPIXEL_ADDRESS) || server_address.contains("hypixel.net") || server_address.contains("hypixel.io") || server_brand.contains("Hypixel BungeeCord"));
    }

    /// java: @Deprecated public static String getIslandArea()
    pub fn get_island_area() -> String {
        // java: try {..} catch (IndexOutOfBoundsException e) - the body runs inline: a Rust body does not throw checked exceptions (a panic in it propagates)
        {
            for sidebar_line in compat::get_state(&STRING_SCOREBOARD) {
                if compat::index_of(&sidebar_line, sky_block_icons::AREA) >= 0 || compat::index_of(&sidebar_line, sky_block_icons::RIFT_AREA) >= 0 {
                    return sidebar_line.trim().to_string();
                }
            }
        }
        // TODO(fabric2seer): the `catch (IndexOutOfBoundsException e)` block is not reached in Rust - the try body returns, breaks or continues, so it cannot run inside `catch_unwind`
        //   java: {
        //   java: 			LOGGER.error("[Skyblocker] Failed to get location from sidebar", e);
        //   java: 		}
        return "Unknown".to_string();
    }

    /// java: public static double getPurse()
    pub fn get_purse() -> f64 {
        return compat::get_state(&PURSE_2);
    }

    /// java: public static int getBits()
    pub fn get_bits() -> i32 {
        let mut bits: i32 = 0;
        let mut bits_string: Option<String> = None;
        // java: try {..} catch (IndexOutOfBoundsException e) - an exception here is a panic, caught by `catch_unwind`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            for sidebar_line in compat::get_state(&STRING_SCOREBOARD) {
                if sidebar_line.contains("Bits") {
                    bits_string = Some(sidebar_line.clone());
                }
            }
            if let Some(bits_string) = &bits_string {
                bits = (compat::regex::replace_all(&bits_string, "[^0-9.]", "")).trim().to_string().parse::<i32>().expect("NumberFormatException");
            }
        }));
        if let Err(e) = __result {
            logger().error(&format!("[Skyblocker] Failed to get bits from sidebar"));
        }
        return bits;
    }

    /// java: private static void updateScoreboard(Minecraft client)
    pub fn update_scoreboard(client: &mut mcrust::net::minecraft::client::minecraft::Minecraft) {
        // java: try {..} catch (NullPointerException _) - the body runs inline: a Rust body does not throw checked exceptions (a panic in it propagates)
        {
            TEXT_SCOREBOARD.get_mut().clear();
            compat::lock(&STRING_SCOREBOARD).clear();
            let Some(mut world) = mcrust::mcrust::modding::game::level() else {
                return;
            };
            // java: Scoreboard scoreboard = client.level.getScoreboard(); - the level's board (shared with the network side)
            let scoreboard_cell = world.get_scoreboard().expect("NullPointerException: client.level.getScoreboard()");
            let mut scoreboard = scoreboard_cell.lock().unwrap_or_else(|p| p.into_inner());
            let mut objective: Option<Objective> = scoreboard.get_display_objective(DisplaySlot::Sidebar).cloned();
            let mut text_lines: Vec<MutableComponent> = Vec::new();
            let mut string_lines: Vec<String> = Vec::new();
            for score_holder in scoreboard.get_tracked_players() {
                // java: scoreboard.listPlayerScores(scoreHolder).containsKey(objective) - the port's map is by objective name
                if scoreboard.list_scores_of(score_holder).contains_key(objective.as_ref().expect("NullPointerException: objective").get_name()) {
                    let mut team: Option<PlayerTeam> = scoreboard.get_players_team(score_holder).cloned();
                    if let Some(team) = &team {
                        let mut text_line: MutableComponent = component::empty().append(compat::component_of(team.get_player_prefix().clone()).copy()).append(compat::component_of(team.get_player_suffix().clone()).copy());
                        let mut str_line: String = format!("{}{}", compat::component_of(team.get_player_prefix().clone()).get_string(), compat::component_of(team.get_player_suffix().clone()).get_string());
                        if !(compat::trim(&str_line).is_empty()) {
                            let mut formatted: String = chat_formatting::strip_formatting(&str_line);
                            text_lines.push(text_line.clone());
                            string_lines.push(formatted.clone());
                        }
                    }
                }
            }
            if let Some(objective) = &objective {
                string_lines.push(compat::component_of(objective.get_display_name().clone()).get_string());
                text_lines.push(component::empty().append(compat::component_of(objective.get_display_name().clone()).copy()));
                string_lines.reverse();
                text_lines.reverse();
            }
            // the board's lock is not held past the reads Java makes of it (the updates below may read it again)
            drop(scoreboard);
            TEXT_SCOREBOARD.get_mut().extend(text_lines.iter().cloned());
            compat::lock(&STRING_SCOREBOARD).extend(string_lines.iter().cloned());
            if compat::get_state(&IS_ON_SKYBLOCK) {
                Utils::update_purse();
                SlayerManager::check_slayer_quest();
                Utils::update_area();
                // one guard for the whole statement: two `compat::lock(&STRING_SCOREBOARD)` temporaries in one
                // expression both live to its end, and the second waits for the first (a self-deadlock)
                #[allow(unused_mut)] let mut __guard_string_scoreboard = compat::lock(&STRING_SCOREBOARD);
                compat::set_state(&IS_GUEST, (!((*__guard_string_scoreboard).is_empty()) && compat::trim(&(*__guard_string_scoreboard)[0]).ends_with("GUEST")));
                drop(__guard_string_scoreboard); // released before anything below locks it again
            }
        }
        // TODO(fabric2seer): the `catch (NullPointerException _)` block is not reached in Rust - the try body returns, breaks or continues, so it cannot run inside `catch_unwind`
        //   java: {
        //   java: 			//Do nothing
        //   java: 		}
    }

    /// java: private static void updateArea()
    pub fn update_area() {
        let mut area_name: String = (compat::regex::replace_all(&Utils::get_island_area(), &AREA_ICON_REGEX, "")).trim().to_string();
        let mut old_area: compat::Shared<dyn Area> = AREA.get_mut().clone();
        *AREA.get_mut() = area::from(&area_name);
        // java: if (oldArea != area) - the same object (Area constants are enum singletons)
        if !std::sync::Arc::ptr_eq(&old_area, AREA.get_mut()) {
            (skyblock_events::area_change_invoker())(AREA.get_mut().clone());
        }
    }

    /// java: public static void updatePurse()
    pub fn update_purse() {
        // (the board's line is taken out first: the purse-change listeners may read the board again)
        let purse_string: Option<String> = compat::lock(&STRING_SCOREBOARD).iter().find(|s| s.contains("Piggy:") || s.contains("Purse:")).cloned();
        // java: .ifPresent(purseString -> {..}) - the lambda's `return` ends the lambda, which is the method's last statement
        if let Some(purse_string) = purse_string {
            let mut matcher: compat::regex::Matcher = PURSE.matcher(&purse_string);
            if matcher.find() {
                // java: try {..} catch (NumberFormatException e) { LOGGER.error(..); } - `Double.parseDouble` (which trims)
                match compat::regex::replace_all(&matcher.group_named("purse").unwrap(), ",", "").trim().parse::<f64>() {
                    Ok(new_purse) => {
                        let change_since_last: f64 = new_purse - compat::get_state(&PURSE_2);
                        if change_since_last == 0.0 {
                            return;
                        }
                        (skyblock_events::purse_change_invoker())(change_since_last, PurseChangeCause::get_cause(change_since_last));
                        compat::set_state(&PURSE_2, new_purse);
                    }
                    Err(e) => {
                        logger().error(&format!("[Skyblocker] Failed to parse purse string. Input: '{}' ({})", purse_string, e));
                    }
                }
            }
        }
    }

    /// java: private static void updateFromPlayerList(Minecraft client)
    pub fn update_from_player_list(client: &mut mcrust::net::minecraft::client::minecraft::Minecraft) {
        if compat::get_connection(client).is_none() {
            return;
        }
        // java: client.getConnection().getOnlinePlayers() - the port keeps each PlayerInfo's tab list fields in the
        // connection's `tab_list_players` (refreshed every tick)
        let online_players = compat::get_connection(client).unwrap().tab_list_players.lock().unwrap_or_else(|p| p.into_inner()).clone();
        for player_list_entry in online_players.iter() {
            if player_list_entry.tab_list_display_name.is_none() {
                continue;
            }
            let mut name: String = compat::component_of(player_list_entry.tab_list_display_name.clone().unwrap()).get_string();
            if name.starts_with(PROFILE_PREFIX) {
                compat::set_state(&PROFILE, compat::substring(&name, compat::str_len(PROFILE_PREFIX)));
            }
        }
    }

    /// java: private static void updateBiome(Minecraft minecraft)
    pub fn update_biome(minecraft: &mut mcrust::net::minecraft::client::minecraft::Minecraft) {
        if mcrust::mcrust::modding::game::with_player(|_| ()).is_some() {
            // java: Level level = minecraft.player.level(); - the client's level
            let Some(level) = mcrust::mcrust::modding::game::level() else {
                *BIOME.get_mut() = None;
                return;
            };
            let mut feet_pos: BlockPos = mcrust::mcrust::modding::game::with_player(|player| player.base.base.base.base.block_position()).unwrap();
            if level.is_inside_build_height_pos(&feet_pos) {
                // java: biome = level.getBiome(feetPos); - the port's palette keeps the erased `Holder<Biome>` as a
                // `BiomeHolder` (a registry reference)
                let holder = level.get_biome_holder(&feet_pos);
                *BIOME.get_mut() = mcrust::com::mojang::serialization::cast::<mcrust::net::minecraft::world::level::chunk::paletted_container_factory::BiomeHolder>(&holder).map(|b| Holder::Reference(b.0.clone()));
                return;
            }
        }
        *BIOME.get_mut() = None;
    }

    /// java: private static void onDisconnect()
    pub fn on_disconnect() {
        if compat::get_state(&IS_ON_SKYBLOCK) {
            (skyblock_events::leave_invoker())();
        }
        compat::set_state(&IS_ON_SKYBLOCK, false);
        compat::set_state(&SERVER, "".to_string());
        compat::set_state(&GAME_TYPE, "".to_string());
        compat::set_state(&LOCATION_RAW, "".to_string());
        compat::set_state(&LOCATION, Location::Unknown);
        *AREA.get_mut() = area::UNKNOWN.get_mut().clone();
        compat::set_state(&MAP, "".to_string());
        *BIOME.get_mut() = None;
    }

    /// java: private static void onPacket(HypixelS2CPacket packet)
    pub fn on_packet(packet: &crate::unresolved::HypixelS2CPacket) {
        use crate::net::azureaaron::hmapi::network::packet::s2c::hypixel_s2c_packet::HypixelS2CPacket;
        use crate::net::azureaaron::hmapi::network::packet::v1::s2c::location_update_s2c_packet::LocationUpdateS2CPacket;
        match packet {
            HypixelS2CPacket::Hello(hello) => {
                *ENVIRONMENT.get_mut() = hello.environment;
                //Request the player's rank information
                let _ = crate::net::azureaaron::hmapi::network::hypixel_networking::HypixelNetworking::send_player_info_c2s_packet(1);
            }
            HypixelS2CPacket::LocationUpdate(location_update) => {
                compat::set_state(&SERVER, location_update.server_name.clone());
                let previous_server_type: String = compat::get_state(&GAME_TYPE);
                compat::set_state(&GAME_TYPE, location_update.server_type.clone().unwrap_or_default());
                compat::set_state(&LOCATION_RAW, location_update.mode.clone().unwrap_or_default());
                compat::set_state(&LOCATION, Location::from(&compat::get_state(&LOCATION_RAW)));
                compat::set_state(&MAP, location_update.map.clone().unwrap_or_default());

                (skyblock_events::location_change_invoker())(compat::get_state(&LOCATION));

                if compat::get_state(&GAME_TYPE) == "SKYBLOCK" {
                    compat::set_state(&IS_ON_SKYBLOCK, true);
                    Utils::tick_profile_id();

                    if previous_server_type != "SKYBLOCK" {
                        (skyblock_events::join_invoker())();
                    }
                } else if previous_server_type == "SKYBLOCK" {
                    compat::set_state(&IS_ON_SKYBLOCK, false);
                    (skyblock_events::leave_invoker())();
                }
            }
            HypixelS2CPacket::Error(error) if error.id == LocationUpdateS2CPacket::ID => {
                compat::set_state(&SERVER, "".to_string());
                compat::set_state(&GAME_TYPE, "".to_string());
                compat::set_state(&LOCATION_RAW, "".to_string());
                compat::set_state(&LOCATION, Location::Unknown);
                compat::set_state(&MAP, "".to_string());

                if mcrust::mcrust::modding::game::with_player(|_| ()).is_some() {
                    // java: player.sendSystemMessage(Constants.PREFIX.get().append(Component.translatable("skyblocker.utils.locationUpdateError").withStyle(ChatFormatting.RED)));
                    let message = (crate::de::hysky::skyblocker::utils::constants::PREFIX.get())().append(component::translatable("skyblocker.utils.locationUpdateError").with_style_format(mcrust::net::minecraft::chat_formatting::ChatFormatting::Red));
                    script_host::send_feedback(&message.get_string());
                }

                logger().error(&format!("[Skyblocker] Failed to update your current location! Some features of the mod may not work correctly :( - Error: {:?}", error.reason));
            }
            HypixelS2CPacket::PlayerInfo(player_info) => {
                *RANK.get_mut() = crate::unresolved::RankType::get_effective_rank(player_info.player_rank, player_info.package_rank, player_info.monthly_package_rank);
            }
            _ => {} //Do Nothing
        }
    }

    /// java: private static void tickProfileId()
    pub fn tick_profile_id() {
        compat::set_state(&PROFILE_ID_REQUEST, compat::get_state(&PROFILE_ID_REQUEST) + 1);
        // java: Scheduler.INSTANCE.schedule(new Runnable() { private final int requestId = profileIdRequest; .. }, 20 * 8);
        // The anonymous class's one field is captured by value when it is built.
        let request_id: i32 = compat::get_state(&PROFILE_ID_REQUEST);
        let task: Arc<dyn Fn()> = Arc::new(move || {
            if !Utils::is_on_skyblock() {
                return;
            }
            if request_id == compat::get_state(&PROFILE_ID_REQUEST) {
                crate::de::hysky::skyblocker::utils::scheduler::message_scheduler::MessageScheduler::send_message_after_cooldown(crate::de::hysky::skyblocker::utils::scheduler::message_scheduler::INSTANCE.get(), "/profileid", true);
                compat::set_state(&PROFILE_SUGGESTION_MESSAGES, 0);
            }
        });
        { let __r = scheduler::INSTANCE.get_mut(); let __f = compat::dispatch(__r, |__t| __t.schedule()); __f(__r, task, 20 * 8) };
    }

    /// java: @Deprecated private static void parseLocRaw(String message)
    pub fn parse_loc_raw(message: &str) {
        let mut loc_raw: serde_json::Map<String, serde_json::Value> = compat::json::get_as_json_object(&(serde_json::from_str::<serde_json::Value>(message).expect("JsonSyntaxException")));
        if compat::json::has(&loc_raw, "server") {
            compat::set_state(&SERVER, compat::json::get_as_string(&loc_raw.get("server").cloned().unwrap()));
        }
        if compat::json::has(&loc_raw, "gametype") {
            compat::set_state(&GAME_TYPE, compat::json::get_as_string(&loc_raw.get("gametype").cloned().unwrap()));
            compat::set_state(&IS_ON_SKYBLOCK, (compat::get_state(&GAME_TYPE) == "SKYBLOCK"));
        }
        if compat::json::has(&loc_raw, "mode") {
            compat::set_state(&LOCATION_RAW, compat::json::get_as_string(&loc_raw.get("mode").cloned().unwrap()));
            compat::set_state(&LOCATION, Location::from(&compat::get_state(&LOCATION_RAW)));
        } else {
            compat::set_state(&LOCATION, Location::Unknown);
        }
        if compat::json::has(&loc_raw, "map") {
            compat::set_state(&MAP, compat::json::get_as_string(&loc_raw.get("map").cloned().unwrap()));
        }
    }

    /// java: public static boolean onChatMessage(Component text, boolean overlay)
    pub fn on_chat_message(text: &MutableComponent, mut overlay: bool) -> bool {
        if overlay {
            return true;
        }
        let mut message: String = text.get_string();
        if message.starts_with("{\"server\":") && message.ends_with("}") {
            Utils::parse_loc_raw(&message);
        }
        if compat::get_state(&IS_ON_SKYBLOCK) {
            if message.starts_with(PROFILE_MESSAGE_PREFIX) {
                compat::set_state(&PROFILE, (compat::regex::split(&(compat::substring(&message, compat::str_len(PROFILE_MESSAGE_PREFIX))), "§b"))[(0) as usize].clone());
            } else if message.starts_with(PROFILE_ID_PREFIX) {
                let mut prev_profile_id: String = compat::get_state(&PROFILE_ID);
                compat::set_state(&PROFILE_ID, compat::substring(&message, compat::str_len(PROFILE_ID_PREFIX)));
                // java: Utils.getEnvironment() != Environment.PRODUCTION
                if *Utils::get_environment() != crate::unresolved::Environment::Production {
                    compat::set_state(&PROFILE_ID, format!("{}-alpha", compat::get_state(&PROFILE_ID)));
                }
                compat::set_state(&PROFILE_ID_REQUEST, compat::get_state(&PROFILE_ID_REQUEST) + 1);
                if !(prev_profile_id == compat::get_state(&PROFILE_ID)) {
                    (skyblock_events::profile_change_invoker())(prev_profile_id.clone(), compat::get_state(&PROFILE_ID));
                }
            } else if chat_formatting::strip_formatting(&message).starts_with(PROFILE_ID_SUGGEST_PREFIX) {
                let mut suggestions: i32 = compat::get_state(&PROFILE_SUGGESTION_MESSAGES);
                compat::set_state(&PROFILE_SUGGESTION_MESSAGES, compat::get_state(&PROFILE_SUGGESTION_MESSAGES) + 1);
                return (suggestions >= 2);
            }
        }
        return true;
    }

    /// java: public static void sendMessageToBypassEvents(Component message)
    pub fn send_message_to_bypass_events(message: &MutableComponent) {
        let client = compat::minecraft();
        let message = mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(message.clone());
        // java: client.gui.hud.getChat().addClientSystemMessage(message); - as `Minecraft.showDebugChat` calls it
        let chat_focused = client.gui.screen_kind == mcrust::net::minecraft::client::gui::gui::ScreenKind::Chat;
        let options = mcrust::net::minecraft::client::gui::components::chat_component::chat_options(&client.options.borrow(), chat_focused);
        let ticks = client.gui.hud.tick_count;
        let splitter = client.font.splitter();
        client.gui.hud.chat.add_client_system_message(message.clone(), ticks, &splitter, options, chat_focused);
        // java: ((ChatListenerAccessor) client.gui.chatListener()).invokeLogSystemMessage(message, Instant.now());
        mcrust::net::minecraft::client::multiplayer::chat::chat_listener::log_system_message(&message, mcrust::java::time::instant::Instant::now());
        // java: client.getNarrator().saySystemQueued(message);
        client.narrator.say_system_queued(&client.options.borrow(), &message);
    }

    /// java: public static UUID getUuid()
    pub fn get_uuid() -> Uuid {
        // java: return CLIENT.getUser().getProfileId(); - the port's User keeps the id as its string
        let profile_id: String = compat::minecraft().get_user().get_profile_id();
        return Uuid::from_string(&profile_id).or_else(|| mcrust::com::mojang::util::undashed_uuid::from_string(&profile_id)).expect("User.getProfileId()");
    }

    /// java: public static String getUndashedUuid()
    pub fn get_undashed_uuid() -> String {
        return undashed_uuid::to_string(&Utils::get_uuid());
    }

    /// java: public static OptionalInt parseInt(String input)
    pub fn parse_int(input: &str) -> Option<i32> {
        // java: try {..} catch (NumberFormatException _) - an exception here is a panic, caught by `catch_unwind`; the body's `return` is the closure's value
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            return Some(input.parse::<i32>().expect("NumberFormatException"));
        })) {
            Ok(__v) => return __v,
            Err(_) => {
                return None;
            }
        }
    }

    /// java: public static float getEyeHeight(Player player)
    pub fn get_eye_height(player: &dyn Player) -> f32 {
        if !player.is_shift_key_down() {
            return 1.62f32;
        }
        return 1.27f32;
    }

    /// java: public static void checkForIllegalResourceModification(Identifier id, Resource resource, String error)
    pub fn check_for_illegal_resource_modification(id: &Identifier, resource: &Resource, error: &str) {
        if !(resource.source_pack_id() == skyblocker_mod::NAMESPACE) {
            logger().error(&"!".repeat((50) as usize));
            // TODO(fabric2seer): a log call with a non-literal format
            //   java: LOGGER.error(LogUtils.FATAL_MARKER, error, id, resource.sourcePackId());
            logger().error(&"!".repeat((50) as usize));
            blaze3d::you_just_lost_the_game();
        }
    }

}

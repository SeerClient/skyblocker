//! Ported from de.hysky.skyblocker.skyblock.dungeon.secrets.DungeonPlayerManager by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::events::dungeon_events;
use crate::de::hysky::skyblocker::skyblock::dungeon::dungeon_class::DungeonClass;
use crate::de::hysky::skyblocker::skyblock::tabhud::util::player_list_manager::PlayerListManager;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler::Scheduler;
use crate::entry::skyblocker_mod;
use mcrust::java::util::concurrent::completable_future::CompletableFuture;
use mcrust::java::util::uuid::Uuid;
use mcrust::net::minecraft::client::minecraft;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::world::entity::player::player::Player;
use mcrust::com::mojang::authlib::minecraft::minecraft_session_service::MinecraftSessionService as _;
use mcrust::net::minecraft::network::chat::component::Component as _;
use mcrust::net::minecraft::world::entity::entity::Entity as _;

/// java: public static final Pattern PLAYER_TAB_PATTERN = Pattern.compile("\\[\\d+] (?:\\[[A-Za-z]+] )?(?<name>[A-Za-z0-9_]+) (?:.+ )?\\((?<class>\\S+) ?(?<level>[LXVI0]+)?\\)");
pub static PLAYER_TAB_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("\\[\\d+] (?:\\[[A-Za-z]+] )?(?<name>[A-Za-z0-9_]+) (?:.+ )?\\((?<class>\\S+) ?(?<level>[LXVI0]+)?\\)"));
/// java: public static final Pattern PLAYER_GHOST_PATTERN = Pattern.compile(" ☠ (?<name>[A-Za-z0-9_]+) .+ became a ghost\\.");
pub static PLAYER_GHOST_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile(" ☠ (?<name>[A-Za-z0-9_]+) .+ became a ghost\\."));
/// java: private static boolean dungeonLoaded;
pub static DUNGEON_LOADED: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));
/// java: private static final @Nullable DungeonPlayer[] players = new DungeonPlayer[5];
/// `new DungeonPlayer[5]`: each slot null until a player fills it.
pub static PLAYERS: compat::MainThread<Vec<Option<compat::Shared<DungeonPlayer>>>> = compat::MainThread::new(|| vec![None; 5]);

/// java: public class DungeonPlayerManager
#[derive(Clone)]
pub struct DungeonPlayerManager {}

impl DungeonPlayerManager {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        dungeon_events::DUNGEON_LOADED.register(Arc::new(move || { compat::set_state(&DUNGEON_LOADED, true); }));
        ({ let __r = scheduler::INSTANCE.get_mut(); let __f = compat::dispatch(__r, |__t| __t.schedule_cyclic()); __f(__r, Arc::new(move || { DungeonPlayerManager::update_players(); }), 1) });
        // java: ClientReceiveMessageEvents.ALLOW_GAME.register(DungeonPlayerManager::onPlayerGhost);
        compat::register_allow_game(|text, overlay| DungeonPlayerManager::on_player_ghost(text, overlay));
        client_play_connection_events::JOIN.register(Arc::new(|listener: &mut ClientPacketListener| {
            DungeonPlayerManager::reset();
        }));
    }

    /// java: public static @Nullable DungeonPlayer[] getPlayers()
    /// The array (never null); its elements may be.
    pub fn get_players() -> Vec<Option<compat::Shared<DungeonPlayer>>> {
        return PLAYERS.get().clone();
    }

    /// java: public static Optional<DungeonPlayer> getPlayer(String name)
    pub fn get_player(name: &str) -> Option<compat::Shared<DungeonPlayer>> {
        // Arrays.stream(players).filter(Objects::nonNull).filter(p -> p.name().equals(name)).findAny()
        return PLAYERS.get().iter().flatten().cloned().filter(|p| (compat::read(p, |__t| __t.name.clone()) == name)).next();
    }

    /// java: public static DungeonClass getClassFromPlayer(Player player)
    pub fn get_class_from_player(player: &dyn Player) -> DungeonClass {
        return DungeonPlayerManager::get_class_from_player_string(player.player().get_game_profile().name());
    }

    /// java: public static DungeonClass getClassFromPlayer(String name)
    pub fn get_class_from_player_string(name: &str) -> DungeonClass {
        return DungeonPlayerManager::get_player(name).map(|__a0| DungeonPlayer::dungeon_class(&__a0)).unwrap_or(DungeonClass::Unknown);
    }

    /// java: private static void updatePlayers()
    pub fn update_players() {
        if !(compat::get_state(&DUNGEON_LOADED)) {
            return;
        }
        for i in 0..5i32 {
            let Some(mut matcher) = DungeonPlayerManager::get_player_from_tab(i.wrapping_add(1)) else {
                PLAYERS.get_mut()[(i) as usize] = None;
                continue;
            };
            let mut name: String = matcher.group_named("name").unwrap();
            let mut dungeon_class: DungeonClass = DungeonClass::from(matcher.group_named("class").as_deref().unwrap());
            let mut dungeon_player: Option<compat::Shared<DungeonPlayer>> = PLAYERS.get()[(i) as usize].clone();
            if dungeon_player.is_some() && compat::read(dungeon_player.as_ref().unwrap(), |__t| __t.name.clone()) == name {
                DungeonPlayer::update(dungeon_player.as_ref().unwrap(), dungeon_class);
            } else {
                let __v = DungeonPlayer::new(name.clone(), dungeon_class);
                PLAYERS.get_mut()[(i) as usize] = Some(__v);
            }
        }
    }

    /// java: public static @Nullable Matcher getPlayerFromTab(@Range(from = 1, to = 5) int index)
    pub fn get_player_from_tab(mut index: i32) -> Option<compat::regex::Matcher> {
        return PlayerListManager::regex_at(1i32.wrapping_add((index.wrapping_sub(1)).wrapping_mul(4)), &PLAYER_TAB_PATTERN);
    }

    /// java: @SuppressWarnings("SameReturnValue") private static boolean onPlayerGhost(Component text, boolean overlay)
    pub fn on_player_ghost(text: &MutableComponent, mut overlay: bool) -> bool {
        if !(compat::get_state(&DUNGEON_LOADED)) {
            return true;
        }
        let mut matcher: compat::regex::Matcher = PLAYER_GHOST_PATTERN.matcher(&text.get_string());
        if !matcher.find() {
            return true;
        }
        let mut name: String = matcher.group_named("name").unwrap();
        if name == "You" {
            debug_assert!(mcrust::mcrust::modding::game::with_player(|_| ()).is_some());
            name = mcrust::mcrust::modding::game::with_player(|player| compat::component_of(player.get_name()).get_string()).unwrap();
        }
        if let Some(__a0) = DungeonPlayerManager::get_player(&name) {
            DungeonPlayer::ghost(&__a0);
        } else {
            logger().error(&format!("[Skyblocker Dungeon Player Manager] Received ghost message for player '{}' but player was not found in the player list: {}", compat::opt_str(&matcher.group_named("name")), format!("[{}]", PLAYERS.get().iter().map(|p| p.as_ref().map_or("null".to_string(), |p| compat::read(p, |p| p.to_string()))).collect::<Vec<_>>().join(", "))));
        }
        return true;
    }

    /// java: private static void reset()
    pub fn reset() {
        compat::set_state(&DUNGEON_LOADED, false);
        PLAYERS.get_mut().fill(None);
    }

}

/// java: public static class DungeonPlayer
#[derive(Clone)]
pub struct DungeonPlayer {
    /// java: private @Nullable UUID uuid;
    pub uuid: Option<Uuid>,
    /// java: private final String name;
    pub name: String,
    /// java: private DungeonClass dungeonClass = DungeonClass.UNKNOWN;
    pub dungeon_class: DungeonClass,
    /// java: private boolean alive;
    pub alive: bool,
    /// java: private long lastGhostTime;
    pub last_ghost_time: i64,
}

impl DungeonPlayer {
    /// java: public DungeonPlayer(String name, DungeonClass dungeonClass)
    pub fn new(mut name: String, mut dungeon_class: DungeonClass) -> compat::Shared<Self> {
        let __init_uuid = DungeonPlayer::find_player_uuid(&name);
        let __init_name = name.clone();
        let __this = compat::shared(Self {
            uuid: __init_uuid,
            name: __init_name,
            dungeon_class: DungeonClass::Unknown,
            alive: false,
            last_ghost_time: 0,
        });
        DungeonPlayer::update(&__this, dungeon_class);
        // Pre-fetches game profiles for rendering skins in the leap overlay and fancy dungeon map.
        // CompletableFuture.runAsync(() -> Minecraft.getInstance().services().sessionService().fetchProfile(uuid, false), VIRTUAL_THREAD_EXECUTOR)
        // - the task reads `uuid` (a null one throws inside the task, which the future swallows)
        let __uuid = compat::read(&__this, |__t| __t.uuid.clone());
        CompletableFuture::<()>::run_async(move || {
            if let Some(uuid) = __uuid {
                minecraft::session_service().fetch_profile(uuid, false);
            }
        }, compat::executor);
        __this
    }

    /// java: private static @Nullable UUID findPlayerUuid(String name)
    pub fn find_player_uuid(name: &str) -> Option<Uuid> {
        debug_assert!(mcrust::mcrust::modding::game::level().is_some());
        // java: return StreamSupport.stream(Minecraft.getInstance().level.entitiesForRendering().spliterator(), false)
        //   .filter(Player.class::isInstance).map(Player.class::cast).filter(player -> player.getGameProfile().name().equals(name))
        //   .findAny().map(Player::getUUID).orElse(null);
        // The level's players are its Player entities (`ClientLevel.players`).
        use mcrust::net::minecraft::world::entity::entity::Entity as _;
        let players = mcrust::mcrust::modding::game::level().unwrap().with_players(|players| players.to_vec());
        return players.iter().find_map(|avatar| {
            if mcrust::mcrust::modding::game::is_held_here(&**avatar) {
                return None;
            }
            let player = mcrust::mcrust::modding::game::lock_entity(&**avatar, "dungeon_player_manager.rs:DungeonPlayer.findPlayerUuid");
            (player.get_game_profile_name() == name).then(|| compat::uuid_of(player.base().get_uuid()))
        });
    }

    /// java: private void update(DungeonClass dungeonClass)
    pub fn update(__this: &compat::Shared<Self>, mut dungeon_class: DungeonClass) {
        if compat::read(__this, |__t| __t.dungeon_class) == DungeonClass::Unknown && (compat::read(__this, |__t| __t.last_ghost_time)).wrapping_add(2000 as i64) > compat::millis() {
            return;
        }
        { let __v = dungeon_class; compat::write(__this, |__t| __t.dungeon_class = __v); }
        { let __v = (dungeon_class != DungeonClass::Unknown); compat::write(__this, |__t| __t.alive = __v); }
    }

    /// java: private void ghost()
    pub fn ghost(__this: &compat::Shared<Self>) {
        DungeonPlayer::update(__this, DungeonClass::Unknown);
        { let __v = compat::millis(); compat::write(__this, |__t| __t.last_ghost_time = __v); }
    }

    /// java: public @Nullable UUID uuid()
    pub fn uuid(__this: &compat::Shared<Self>) -> Option<Uuid> {
        if compat::read(__this, |__t| __t.uuid.clone()).is_none() {
            // java: return uuid = findPlayerUuid(name); // Try to find the UUID. This shouldn't really happen, but this can act as failsafe.
            let __v = DungeonPlayer::find_player_uuid(&compat::read(__this, |__t| __t.name.clone()));
            compat::write(__this, |__t| __t.uuid = __v.clone());
            return __v;
        }
        return compat::read(__this, |__t| __t.uuid.clone());
    }

    /// java: public String name()
    pub fn name(__this: &compat::Shared<Self>) -> String {
        return compat::read(__this, |__t| __t.name.clone());
    }

    /// java: public DungeonClass dungeonClass()
    pub fn dungeon_class(__this: &compat::Shared<Self>) -> DungeonClass {
        return compat::read(__this, |__t| __t.dungeon_class);
    }

    /// java: public boolean alive()
    pub fn alive(__this: &compat::Shared<Self>) -> bool {
        return compat::read(__this, |__t| __t.alive);
    }

    /// java: @Override @GenToString public native String toString();
    /// The body the build's `ObjectProcessor` generates: `java.lang.runtime.ObjectMethods`' toString -
    /// `DungeonPlayer[f=String.valueOf(f), ..]` over every non-static, non-transient field in declaration order.
    pub fn to_string(&self) -> String {
        format!("DungeonPlayer[uuid={}, name={}, dungeonClass={}, alive={}, lastGhostTime={}]", compat::opt_str(&self.uuid), self.name, self.dungeon_class, self.alive, self.last_ghost_time)
    }

}

impl std::fmt::Display for DungeonPlayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", DungeonPlayer::to_string(self))
    }
}

//! Ported from de.hysky.skyblocker.skyblock.dungeon.secrets.SecretSync by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::events::dungeon_events;
use crate::de::hysky::skyblocker::skyblock::dungeon::dungeon_score::DungeonScore;
use crate::de::hysky::skyblocker::skyblock::dungeon::preview::room_preview_server;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::dungeon_manager::DungeonManager;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::dungeon_player_manager::DungeonPlayer;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::dungeon_player_manager::DungeonPlayerManager;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::room::Room;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::room::RoomBase;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::secret_waypoint::SecretWaypoint;
use crate::de::hysky::skyblocker::utils::waypoint::waypoint::Waypoint;
use crate::de::hysky::skyblocker::utils::ws::message::dungeon_bat_killed_message::DungeonBatKilledMessage;
use crate::de::hysky::skyblocker::utils::ws::message::dungeon_mimic_killed_message::DungeonMimicKilledMessage;
use crate::de::hysky::skyblocker::utils::ws::message::dungeon_prince_killed_message::DungeonPrinceKilledMessage;
use crate::de::hysky::skyblocker::utils::ws::message::dungeon_room_hide_waypoint_message::DungeonRoomHideWaypointMessage;
use crate::de::hysky::skyblocker::utils::ws::message::dungeon_room_match_message::DungeonRoomMatchMessage;
use crate::de::hysky::skyblocker::utils::ws::message::dungeon_room_secret_count_message::DungeonRoomSecretCountMessage;
use crate::de::hysky::skyblocker::utils::ws::message::message::Message;
use crate::de::hysky::skyblocker::utils::ws::service::Service;
use crate::de::hysky::skyblocker::utils::ws::ws_message_handler::WsMessageHandler;
use mcrust::java::util::uuid::Uuid;
use mcrust::org::joml::vector2i::Vector2i;

/// java: private static final Supplier<DungeonsConfig.SecretSync> CONFIG = () -> SkyblockerConfigManager.get().dungeons.secretSync;
pub static CONFIG: compat::MainThread<Arc<dyn Fn() -> compat::Shared<crate::de::hysky::skyblocker::config::configs::dungeons_config::SecretSync>>> = compat::MainThread::new(|| Arc::new(move || SkyblockerConfigManager::get().dungeons.secret_sync.clone()));

/// java: public class SecretSync
#[derive(Clone)]
pub struct SecretSync {}

impl SecretSync {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        dungeon_events::ROOM_MATCHED.register(Arc::new(move |__a0| { SecretSync::sync_room_match(&__a0); }));
        dungeon_events::SECRET_COUNT_UPDATED.register(Arc::new(move |__a0, __a1| { SecretSync::sync_secret_count(&__a0, __a1); }));
        dungeon_events::SECRET_FOUND.register(Arc::new(move |__a0, __a1| { SecretSync::sync_secret_found(&__a0, &__a1); }));
    }

    /// java: public static boolean checkSender(UUID uuid)
    pub fn check_sender(uuid: &Uuid) -> bool {
        if DungeonPlayerManager::get_players().unwrap().iter().cloned().filter(|__a0| true).any(|player| (uuid == DungeonPlayer::uuid(&player).unwrap())) {
            return true;
        }
        logger().error(&format!("[Skyblocker Dungeon Secret Sync] Received a message from a player not in the Dungeons run: {}", compat::jstring(&uuid)));
        return false;
    }

    /// java: public static @Nullable Room getRoomByName(String roomName)
    pub fn get_room_by_name(room_name: &str) -> Option<compat::Shared<dyn Room>> {
        return DungeonManager::get_rooms_stream().filter(|__a0| ({ let __f = compat::dispatch(__a0, |__t| __t.is_matched()); __f(__a0) })).filter(|rm| (({ let __f = compat::dispatch(rm, |__t| __t.get_name()); __f(rm) }).as_deref() == Some(&*(room_name)))).next();
    }

    /// java: public static void syncRoomMatch(Room room)
    pub fn sync_room_match(room: &compat::Shared<dyn Room>) {
        if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() || compat::read(room, |__t| __t.room_base().from_websocket) {
            return;
        }
        let mut segments: Vec<Vector2i> = ({ let __f = compat::dispatch(room, |__t| __t.get_segments()); __f(room) }).into_iter().collect::<Vec<_>>();
        WsMessageHandler::send_server_message(Service::DungeonSecrets, &(compat::shared(DungeonRoomMatchMessage::new(mcrust::mcrust::modding::game::with_player(|player| compat::uuid_of(player.base.base.base.base.get_uuid())).unwrap(), ({ let __f = compat::dispatch(room, |__t| __t.get_type()); __f(room) }), ({ let __f = compat::dispatch(room, |__t| __t.get_shape()); __f(room) }), ({ let __f = compat::dispatch(room, |__t| __t.get_direction()); __f(room) }).unwrap(), ({ let __f = compat::dispatch(room, |__t| __t.get_name()); __f(room) }).unwrap(), segments.clone())) as compat::Shared<dyn Message<compat::Object>>));
    }

    /// java: public static void handleRoomMatch(DungeonRoomMatchMessage msg)
    pub fn handle_room_match(msg: &DungeonRoomMatchMessage) {
        if !(compat::read(&(CONFIG.get_mut())(), |__t| __t.receive_room_match)) || !(SecretSync::check_sender(&msg.sender())) {
            return;
        }
        if (DungeonManager::get_rooms_stream().count() as i64) > (36 as i64) || (msg.pos().len() as i32) > 4 {
            return;
        }
        if DungeonManager::get_room_metadata(&msg.room()).is_none() {
            logger().error(&format!("[Skyblocker Dungeons Secret Sync] Received an invalid room over the websocket, msg: {}", compat::jstring(&msg)));
            return;
        }
        if DungeonManager::check_if_segments_exist(&msg.pos()) {
            return;
        }
        let mut segments: std::collections::HashSet<Vector2i> = msg.pos().to_vec();
        // TODO(fabric2seer): static call IntSortedSets.unmodifiable(..)
        //   java: IntSortedSet segmentsX = IntSortedSets.unmodifiable(new IntRBTreeSet(segments.stream().mapToInt(Vector2ic::x).toArray()));
        let mut segments_x: crate::unresolved::IntSortedSet = todo!("fabric2seer: the initialiser of `segmentsX` (TODO above)");
        // TODO(fabric2seer): static call IntSortedSets.unmodifiable(..)
        //   java: IntSortedSet segmentsY = IntSortedSets.unmodifiable(new IntRBTreeSet(segments.stream().mapToInt(Vector2ic::y).toArray()));
        let mut segments_y: crate::unresolved::IntSortedSet = todo!("fabric2seer: the initialiser of `segmentsY` (TODO above)");
        if RoomBase::determine_shape_4(msg.room_type(), &segments, &segments_x, &segments_y) != msg.shape() {
            logger().error(&format!("[Skyblocker Dungeons Secret Sync] Received a room with an invalid shape!, msg: {}", compat::jstring(&msg)));
            return;
        }
        let mut new_room: compat::Shared<dyn Room> = (RoomBase::new_7(msg.room_type(), msg.shape(), msg.direction(), msg.room(), segments.clone(), segments_x.clone(), segments_y.clone()) as compat::Shared<dyn Room>);
        DungeonManager::add_room_from_ws(new_room.clone());
        logger().info(&format!("[Skyblocker Dungeon Secret Sync] Added room {}", msg.room()));
    }

    /// java: public static void syncSecretCount(Room room, boolean fromWS)
    pub fn sync_secret_count(room: &compat::Shared<dyn Room>, mut from_ws: bool) {
        if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() || from_ws {
            return;
        }
        WsMessageHandler::send_server_message(Service::DungeonSecrets, &(compat::shared(DungeonRoomSecretCountMessage::new(mcrust::mcrust::modding::game::with_player(|player| compat::uuid_of(player.base.base.base.base.get_uuid())).unwrap(), ({ let __f = compat::dispatch(room, |__t| __t.get_name()); __f(room) }).unwrap(), ({ let __f = compat::dispatch(room, |__t| __t.get_found_secret_count()); __f(room) }))) as compat::Shared<dyn Message<compat::Object>>));
    }

    /// java: public static void handleSecretCountUpdate(DungeonRoomSecretCountMessage msg)
    pub fn handle_secret_count_update(msg: &DungeonRoomSecretCountMessage) {
        if !(compat::read(&(CONFIG.get_mut())(), |__t| __t.receive_room_secret_count)) || !(SecretSync::check_sender(&msg.sender())) {
            return;
        }
        let mut room: Option<compat::Shared<dyn Room>> = SecretSync::get_room_by_name(&msg.room_name());
        if room.is_none() || compat::read(room.as_ref().unwrap(), |__t| __t.room_base().secrets_found) >= msg.secret_count() || msg.secret_count() > ({ let __r = room.as_ref().unwrap(); let __f = compat::dispatch(__r, |__t| __t.get_secret_count()); __f(__r) }) {
            return;
        }
        { let __v = msg.secret_count(); compat::write(room.as_ref().unwrap(), |__t| __t.room_base_mut().secrets_found = __v); }
        { let __v = false; compat::write(room.as_ref().unwrap(), |__t| __t.room_base_mut().secret_count_outdated = __v); }
        (dungeon_events::secret_count_updated_invoker())(room.clone().unwrap(), true);
    }

    /// java: public static void syncSecretFound(Room room, SecretWaypoint waypoint)
    pub fn sync_secret_found(room: &compat::Shared<dyn Room>, waypoint: &compat::Shared<SecretWaypoint>) {
        if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() || compat::get_state(&room_preview_server::IS_ACTIVE) {
            return;
        }
        WsMessageHandler::send_server_message(Service::DungeonSecrets, &(compat::shared(DungeonRoomHideWaypointMessage::new(mcrust::mcrust::modding::game::with_player(|player| compat::uuid_of(player.base.base.base.base.get_uuid())).unwrap(), ({ let __f = compat::dispatch(room, |__t| __t.get_name()); __f(room) }).unwrap(), ({ let __f = compat::dispatch(waypoint, |__t| __t.hash_code()); __f(&compat::view(waypoint, |a| a as compat::Shared<dyn Waypoint>)) }))) as compat::Shared<dyn Message<compat::Object>>));
    }

    /// java: public static void handleHideWaypoint(DungeonRoomHideWaypointMessage msg)
    pub fn handle_hide_waypoint(msg: &DungeonRoomHideWaypointMessage) {
        if !(SecretSync::check_sender(&msg.sender())) || !(compat::read(&(CONFIG.get_mut())(), |__t| __t.hide_received_waypoints)) {
            return;
        }
        let Some(mut room) = SecretSync::get_room_by_name(&msg.room_name()) else {
            return;
        };
        let mut secret_index: i32 = ({ let __f = compat::dispatch(&room, |__t| __t.get_index_by_waypoint_hash()); __f(&room, msg.waypoint_hash()) });
        if secret_index == -1 {
            return;
        }
        ({ let __f = compat::dispatch(&room, |__t| __t.mark_secrets()); __f(&room, secret_index, true) });
        logger().info(&format!("[Skyblocker Dungeon Secret Sync] Hiding waypoints for secret #{} in room {}", secret_index, msg.room_name()));
    }

    /// java: public static void syncBatKilled()
    pub fn sync_bat_killed() {
        if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() {
            return;
        }
        WsMessageHandler::send_server_message(Service::DungeonSecrets, &(compat::shared(DungeonBatKilledMessage::new(mcrust::mcrust::modding::game::with_player(|player| compat::uuid_of(player.base.base.base.base.get_uuid())).unwrap())) as compat::Shared<dyn Message<compat::Object>>));
    }

    /// java: public static void handleBatKilled(DungeonBatKilledMessage msg)
    pub fn handle_bat_killed(msg: &DungeonBatKilledMessage) {
        if !(SecretSync::check_sender(&msg.sender())) {
            return;
        }
        DungeonScore::on_bat_kill(false);
        logger().info("[Skyblocker Dungeon Secret Sync] Bat killed!");
    }

    /// java: public static void syncMimicKilled()
    pub fn sync_mimic_killed() {
        if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() {
            return;
        }
        WsMessageHandler::send_server_message(Service::DungeonSecrets, &(compat::shared(DungeonMimicKilledMessage::new(mcrust::mcrust::modding::game::with_player(|player| compat::uuid_of(player.base.base.base.base.get_uuid())).unwrap())) as compat::Shared<dyn Message<compat::Object>>));
    }

    /// java: public static void handleMimicKilled(DungeonMimicKilledMessage msg)
    pub fn handle_mimic_killed(msg: &DungeonMimicKilledMessage) {
        if !(SecretSync::check_sender(&msg.sender())) {
            return;
        }
        DungeonScore::on_mimic_kill();
        logger().info("[Skyblocker Dungeon Secret Sync] Mimic killed!");
    }

    /// java: public static void syncPrinceKilled()
    pub fn sync_prince_killed() {
        if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() {
            return;
        }
        WsMessageHandler::send_server_message(Service::DungeonSecrets, &(compat::shared(DungeonPrinceKilledMessage::new(mcrust::mcrust::modding::game::with_player(|player| compat::uuid_of(player.base.base.base.base.get_uuid())).unwrap())) as compat::Shared<dyn Message<compat::Object>>));
    }

    /// java: public static void handlePrinceKilled(DungeonPrinceKilledMessage msg)
    pub fn handle_prince_killed(msg: &DungeonPrinceKilledMessage) {
        if !(SecretSync::check_sender(&msg.sender())) {
            return;
        }
        DungeonScore::on_prince_kill(false);
        logger().info("[Skyblocker Dungeon Secret Sync] Prince killed!");
    }

}

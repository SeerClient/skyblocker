//! Ported from de.hysky.skyblocker.skyblock.dungeon.secrets.DungeonWebSocket by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::events::dungeon_events;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::secret_sync::SecretSync;
use crate::de::hysky::skyblocker::utils::render::render_helper::RenderHelper;
use crate::de::hysky::skyblocker::utils::ws::message::dungeon_bat_killed_message;
use crate::de::hysky::skyblocker::utils::ws::message::dungeon_mimic_killed_message;
use crate::de::hysky::skyblocker::utils::ws::message::dungeon_prince_killed_message;
use crate::de::hysky::skyblocker::utils::ws::message::dungeon_room_hide_waypoint_message;
use crate::de::hysky::skyblocker::utils::ws::message::dungeon_room_match_message;
use crate::de::hysky::skyblocker::utils::ws::message::dungeon_room_secret_count_message;
use crate::de::hysky::skyblocker::utils::ws::r#type::Type;
use crate::de::hysky::skyblocker::utils::ws::service::Service;
use crate::de::hysky::skyblocker::utils::ws::ws_state_manager::WsStateManager;
use mcrust::com::mojang::serialization::dynamic::Dynamic;
use mcrust::com::mojang::serialization::decoder::Decoder as _;
use mcrust::com::mojang::serialization::dynamic_like::DynamicLike as _;

/// java: public class DungeonWebSocket
#[derive(Clone)]
pub struct DungeonWebSocket {}

impl DungeonWebSocket {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        dungeon_events::DUNGEON_STARTED.register(Arc::new(move || { DungeonWebSocket::connect(); }));
    }

    /// java: private static void connect()
    pub fn connect() {
        WsStateManager::subscribe_server(Service::DungeonSecrets, None);
    }

    /// java: public static void handleMessage(Type type, Optional<Dynamic<?>> rawMsg)
    pub fn handle_message(mut r#type: Type, raw_msg: &Option<Dynamic>) {
        if raw_msg.is_none() {
            return;
        }
        let mut message: Dynamic = raw_msg.clone().unwrap();
        if r#type == Type::Response {
            if message.get("type").as_string().is_error() {
                return;
            }
            let mut message_type: String = message.get("type").as_string().get_or_throw().unwrap();
            match message_type.as_str() {
                dungeon_room_match_message::TYPE => {
                    RenderHelper::run_on_render_thread(Arc::new(move || { SecretSync::handle_room_match(&dungeon_room_match_message::CODEC.get_mut().parse_dynamic(&message).get_or_throw().unwrap()); }));
                }
                dungeon_room_secret_count_message::TYPE => {
                    RenderHelper::run_on_render_thread(Arc::new(move || { SecretSync::handle_secret_count_update(&dungeon_room_secret_count_message::CODEC.get_mut().parse_dynamic(&message).get_or_throw().unwrap()); }));
                }
                dungeon_room_hide_waypoint_message::TYPE => {
                    RenderHelper::run_on_render_thread(Arc::new(move || { SecretSync::handle_hide_waypoint(&dungeon_room_hide_waypoint_message::CODEC.get_mut().parse_dynamic(&message).get_or_throw().unwrap()); }));
                }
                dungeon_bat_killed_message::TYPE => {
                    RenderHelper::run_on_render_thread(Arc::new(move || { SecretSync::handle_bat_killed(&dungeon_bat_killed_message::CODEC.get_mut().parse_dynamic(&message).get_or_throw().unwrap()); }));
                }
                dungeon_mimic_killed_message::TYPE => {
                    RenderHelper::run_on_render_thread(Arc::new(move || { SecretSync::handle_mimic_killed(&dungeon_mimic_killed_message::CODEC.get_mut().parse_dynamic(&message).get_or_throw().unwrap()); }));
                }
                dungeon_prince_killed_message::TYPE => {
                    RenderHelper::run_on_render_thread(Arc::new(move || { SecretSync::handle_prince_killed(&dungeon_prince_killed_message::CODEC.get_mut().parse_dynamic(&message).get_or_throw().unwrap()); }));
                }
                _ => {}
            }
        }
    }

}

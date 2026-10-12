//! Ported from de.hysky.skyblocker.utils.ws.WsMessageHandler by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::dungeon_web_socket::DungeonWebSocket;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use crate::de::hysky::skyblocker::utils::ws::message::crystals_waypoint_message::CrystalsWaypointMessage;
use crate::de::hysky::skyblocker::utils::ws::message::egg_waypoint_message::EggWaypointMessage;
use crate::de::hysky::skyblocker::utils::ws::message::message::Message;
use crate::de::hysky::skyblocker::utils::ws::payload;
use crate::de::hysky::skyblocker::utils::ws::payload::Payload;
use crate::de::hysky::skyblocker::utils::ws::r#type::Type;
use crate::de::hysky::skyblocker::utils::ws::service::Service;
use crate::entry::skyblocker_mod;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::com::mojang::serialization::dynamic::Dynamic;
use mcrust::com::mojang::serialization::decoder::Decoder as _;
use mcrust::com::mojang::serialization::encoder::Encoder as _;

/// java: public class WsMessageHandler
#[derive(Clone)]
pub struct WsMessageHandler {}

impl WsMessageHandler {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static void sendLocationMessage(Service service, Message<? extends Message<?>> message)
    pub fn send_location_message(mut service: Service, message: &compat::Shared<dyn Message<compat::Shared<dyn Message<compat::Object>>>>) {
        WsMessageHandler::send(Type::Publish, service, &Utils::get_location().to_string(), &Some(WsMessageHandler::encode_message(message)));
    }

    /// java: public static void sendServerMessage(Service service, Message<? extends Message<?>> message)
    pub fn send_server_message(mut service: Service, message: &compat::Shared<dyn Message<compat::Shared<dyn Message<compat::Object>>>>) {
        WsMessageHandler::send(Type::Publish, service, &Utils::get_server(), &Some(WsMessageHandler::encode_message(message)));
    }

    /// java: static void sendSimple(Type type, Service service, String serverId, Optional<Message<? extends Message<?>>> message)
    pub fn send_simple(mut r#type: Type, mut service: Service, server_id: &str, message: &Option<compat::Shared<dyn Message<compat::Shared<dyn Message<compat::Object>>>>>) {
        WsMessageHandler::send(r#type, service, server_id, &message.clone().map(|__a0| WsMessageHandler::encode_message(&__a0)));
    }

    /// java: private static void send(Type type, Service service, String serverId, Optional<Dynamic<?>> message)
    pub fn send(mut r#type: Type, mut service: Service, server_id: &str, message: &Option<Dynamic>) {
        // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut payload: Payload = Payload::new(r#type, service, server_id.to_string(), message.clone());
            // TODO(fabric2seer): `.getAsJsonObject(..)` on a T
            //   java: JsonObject encoded = Payload.CODEC.encodeStart(JsonOps.INSTANCE, payload).getOrThrow().getAsJsonObject();
            let mut encoded: serde_json::Map<String, serde_json::Value> = todo!("fabric2seer: the initialiser of `encoded` (TODO above)");
            // TODO(fabric2seer): `.toJson(..)` on a Gson
            //   java: SkyblockerWebSocket.send(SkyblockerMod.GSON_COMPACT.toJson(encoded));
        }));
        if let Err(e) = __result {
            logger().info(&format!("[Skyblocker WebSocket Message Handler] Failed to send message! Type: {}, Service: {}, Message: {}", compat::jstring(&r#type), compat::jstring(&service), compat::jstring(&message)));
        }
    }

    /// java: private static Dynamic<?> encodeMessage(Message<? extends Message<?>> message)
    pub fn encode_message(message: &compat::Shared<dyn Message<compat::Shared<dyn Message<compat::Object>>>>) -> Dynamic {
        // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`; the body's `return` is the closure's value
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut codec: CodecRef<compat::Shared<dyn Message<compat::Object>>> = message.lock().borrow().get_codec();
            // TODO(fabric2seer): `new` takes 1 argument(s) in the port (`ops: Ops`), Java passes 2
            // rust: let mut dynamic: Dynamic = Dynamic::new(mcrust::com::mojang::serialization::json_ops::JsonOps::instance(), (codec.encode_start(&mcrust::com::mojang::serialization::json_ops::JsonOps::instance(), message)).get_or_throw().unwrap());
            let mut dynamic: Dynamic = todo!("fabric2seer: the initialiser of `dynamic` (TODO above)");
            return dynamic;
        })) {
            Ok(__v) => return __v,
            Err(e) => {
                logger().info(&format!("[Skyblocker WebSocket Message Handler] Failed to encode message! Message: {}", compat::jstring(&message)));
            }
        }
        return Dynamic::new(mcrust::com::mojang::serialization::json_ops::JsonOps::instance());
    }

    /// java: static void handleMessage(String message)
    pub fn handle_message(message: &str) {
        // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut payload_encoded: serde_json::Map<String, serde_json::Value> = compat::json::get_as_json_object(&(serde_json::from_str::<serde_json::Value>(message).expect("JsonSyntaxException")));
            if compat::json::has(&payload_encoded, "type") {
                let mut payload: Payload = (payload::CODEC.get_mut().parse(&mcrust::com::mojang::serialization::json_ops::JsonOps::instance(), &compat::json_to_port(&payload_encoded))).get_or_throw().unwrap();
                match payload.service() {
                    Service::CrystalWaypoints => {
                        CrystalsWaypointMessage::handle(payload.r#type(), &payload.message());
                    }
                    Service::DungeonSecrets => {
                        DungeonWebSocket::handle_message(payload.r#type(), &payload.message());
                    }
                    Service::EggWaypoints => {
                        EggWaypointMessage::handle(payload.r#type(), &payload.message());
                    }
                    _ => {}
                }
            }
        }));
        if let Err(e) = __result {
            logger().error(&format!("[Skyblocker WebSocket Message Handler] Failed to handle incoming message!"));
        }
    }

}

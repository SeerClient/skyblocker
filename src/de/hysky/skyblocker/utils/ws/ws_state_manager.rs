//! Ported from de.hysky.skyblocker.utils.ws.WsStateManager by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use crate::de::hysky::skyblocker::utils::ws::message::message::AnyMessage;
use crate::de::hysky::skyblocker::utils::ws::message::message::Message;
use crate::de::hysky::skyblocker::utils::ws::r#type::Type;
use crate::de::hysky::skyblocker::utils::ws::service::Service;
use crate::de::hysky::skyblocker::utils::ws::ws_message_handler::WsMessageHandler;

/// java: private static final EnumSet<Service> SUBSCRIBED_SERVER_SERVICES = EnumSet.noneOf(Service.class);
pub static SUBSCRIBED_SERVER_SERVICES: LazyLock<Mutex<std::collections::HashSet<Service>>> = LazyLock::new(|| Mutex::new(std::collections::HashSet::<_>::new()));
/// java: private static String lastServerId = "";
pub static LAST_SERVER_ID: LazyLock<Mutex<String>> = LazyLock::new(|| Mutex::new("".to_string()));
/// java: private static final EnumSet<Service> SUBSCRIBED_ISLAND_SERVICES = EnumSet.noneOf(Service.class);
pub static SUBSCRIBED_ISLAND_SERVICES: LazyLock<Mutex<std::collections::HashSet<Service>>> = LazyLock::new(|| Mutex::new(std::collections::HashSet::<_>::new()));
/// java: private static String lastLocation = "";
pub static LAST_LOCATION: LazyLock<Mutex<String>> = LazyLock::new(|| Mutex::new("".to_string()));

/// java: public class WsStateManager
#[derive(Clone)]
pub struct WsStateManager {}

impl WsStateManager {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        client_play_connection_events::JOIN.register(Arc::new(|listener: &mut ClientPacketListener| {
            WsStateManager::reset();
        }));
        client_play_connection_events::DISCONNECT.register(Arc::new(|listener: &mut ClientPacketListener| {
            WsStateManager::reset();
        }));
    }

    /// java: private static void reset()
    pub fn reset() {
        if !(compat::lock(&LAST_SERVER_ID).is_empty()) {
            for service in compat::get_state(&SUBSCRIBED_SERVER_SERVICES) {
                WsMessageHandler::send_simple(Type::Unsubscribe, service, &compat::get_state(&LAST_SERVER_ID), None);
            }
            compat::set_state(&LAST_SERVER_ID, "".to_string());
        }
        if !(compat::lock(&LAST_LOCATION).is_empty()) {
            for service in compat::get_state(&SUBSCRIBED_ISLAND_SERVICES) {
                WsMessageHandler::send_simple(Type::Unsubscribe, service, &compat::get_state(&LAST_LOCATION), None);
            }
            compat::set_state(&LAST_LOCATION, "".to_string());
        }
    }

    /// java: public static void subscribeServer(Service service, Optional<Message<? extends Message<?>>> message)
    pub fn subscribe_server(mut service: Service, message: Option<&dyn AnyMessage>) {
        //   java: if (Utils.getEnvironment() != Environment.PRODUCTION) return;
        if *Utils::get_environment() != crate::unresolved::Environment::Production {
            return;
        }
        compat::lock(&SUBSCRIBED_SERVER_SERVICES).insert(service);
        WsMessageHandler::send_simple(Type::Subscribe, service, &Utils::get_server(), message);
        compat::set_state(&LAST_SERVER_ID, Utils::get_server());
    }

    /// java: public static void subscribeIsland(Service service, Optional<Message<? extends Message<?>>> message)
    pub fn subscribe_island(mut service: Service, message: Option<&dyn AnyMessage>) {
        //   java: if (Utils.getEnvironment() != Environment.PRODUCTION) return;
        if *Utils::get_environment() != crate::unresolved::Environment::Production {
            return;
        }
        compat::lock(&SUBSCRIBED_ISLAND_SERVICES).insert(service);
        WsMessageHandler::send_simple(Type::Subscribe, service, &Utils::get_location().to_string(), message);
        compat::set_state(&LAST_LOCATION, Utils::get_location().to_string());
    }

}

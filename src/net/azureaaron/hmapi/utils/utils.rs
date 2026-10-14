//! Ported from net.azureaaron.hmapi.utils.Utils.

use crate::compat;

/// java: private static final String HYPIXEL_ADDRESS_OVERRIDE = System.getProperty("hmapi.alternateHypixelAddress", "");
/// (system properties are environment variables here)
fn hypixel_address_override() -> String {
    std::env::var("hmapi.alternateHypixelAddress").unwrap_or_default()
}

/// java: public class Utils
pub struct Utils;

impl Utils {
    /// java: public static boolean isOnHypixel()
    pub fn is_on_hypixel() -> bool {
        // Java reaches this from mod init (`HypixelNetworking.registerToEvents`), where `getInstance()` exists but no
        // server is set, so it answers false; the port builds its Minecraft after the entrypoints - same answer.
        if !compat::minecraft_ready() {
            return false;
        }
        let client = compat::minecraft();
        // java: client.getCurrentServer() = Optionull.map(getConnection(), ClientPacketListener::getServerData); through
        // `compat::get_connection`, which answers while the frame loop has lent the player out (the port's
        // `minecraft::get_current_server` answered None there, so every C2S send from a packet listener threw)
        let server_address: String = compat::get_connection(client).and_then(|connection| connection.server_data.clone()).map(|server| server.ip.to_lowercase()).unwrap_or_default();
        // client.player != null && client.player.connection != null && serverBrand() != null ? serverBrand() : ""
        let server_brand: String = compat::get_connection(client)
            .and_then(|sinks| sinks.server_brand.lock().unwrap_or_else(|p| p.into_inner()).clone())
            .unwrap_or_default();
        (!server_address.is_empty() && compat::equals_ignore_case(&hypixel_address_override(), &server_address)) || server_address.contains("hypixel.net") || server_address.contains("hypixel.io") || server_brand.contains("Hypixel BungeeCord")
    }

    /// java: public static int requireInRange(int version, int min, int max)
    pub fn require_in_range(version: i32, min: i32, max: i32) -> i32 {
        if !(version >= min && version <= max) {
            panic!("UnsupportedOperationException: Requested an unsupported packet version");
        }
        version
    }

    /// java: public static void requireIsOnHypixel()
    pub fn require_is_on_hypixel() {
        if !Utils::is_on_hypixel() {
            panic!("UnsupportedOperationException: Requested to send a C2S packet to Hypixel whilst not being on the server!");
        }
    }
}

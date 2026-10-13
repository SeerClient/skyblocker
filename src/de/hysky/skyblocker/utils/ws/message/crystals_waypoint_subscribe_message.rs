//! Ported from de.hysky.skyblocker.utils.ws.message.CrystalsWaypointSubscribeMessage by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::ws::message::message::Message;
use mcrust::com::mojang::serialization::codec;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::com::mojang::serialization::codecs::record_codec_builder;
use mcrust::net::minecraft::client::multiplayer::client_level::ClientLevel;
use mcrust::net::minecraft::core::holder::Reference;
use mcrust::net::minecraft::world::clock::clock_manager::ClockManager;
use mcrust::net::minecraft::world::timeline::timeline::Timeline;

/// java: private static final Codec<CrystalsWaypointSubscribeMessage> CODEC = RecordCodecBuilder.create(instance -> instance.group( Codec.LONG.fieldOf("timestamp").forGetter(CrystalsWaypointSubscribeMessage::t
pub static CODEC: compat::MainThread<CodecRef<CrystalsWaypointSubscribeMessage>> = compat::MainThread::new(|| record_codec_builder::create(record_codec_builder::apply1(record_codec_builder::of(|o: &CrystalsWaypointSubscribeMessage| o.timestamp(), codec::field_of(mcrust::com::mojang::serialization::codec::long_codec(), "timestamp")), move |__a0| CrystalsWaypointSubscribeMessage::new(__a0.clone()))));
/// java: private static final long MAX_LOBBY_LIFETIME = 26;
pub const MAX_LOBBY_LIFETIME: i64 = ((26) as i64) as i64;
// TODO(fabric2seer): `.toMillis(..)` on a Duration
//   java: private static final long MILLIS_PER_MINECRAFT_DAY = Duration.ofMinutes(20).toMillis();
/// java: private static final long MILLIS_PER_MINECRAFT_DAY = Duration.ofMinutes(20).toMillis();
pub static MILLIS_PER_MINECRAFT_DAY: LazyLock<i64> = LazyLock::new(|| 20i64 * 60 * 1000);
/// java: private static final long MILLIS_TO_SECONDS = 1000;
pub const MILLIS_TO_SECONDS: i64 = ((1000) as i64) as i64;

/// java: public record CrystalsWaypointSubscribeMessage(long timestamp) implements Message<CrystalsWaypointSubscribeMessage>
#[derive(Clone, Debug, PartialEq)]
pub struct CrystalsWaypointSubscribeMessage {
    /// java: long timestamp
    pub timestamp: i64,
}

impl CrystalsWaypointSubscribeMessage {
    /// java: the canonical record constructor
    pub fn new(mut timestamp: i64) -> Self {
        Self { timestamp }
    }

    /// java: the record accessor `timestamp()`
    pub fn timestamp(&self) -> i64 {
        self.timestamp
    }

    /// java: public static CrystalsWaypointSubscribeMessage create(ClientLevel world)
    pub fn create(world: &ClientLevel) -> CrystalsWaypointSubscribeMessage {
        let clock_manager = world.clock_manager();
        // TODO(fabric2seer): `RegistryAccess.get` has no Rust method in the port (looked for `get` in RegistryAccess and its supertypes)
        // rust: let mut timeline: Option<Reference> = world.registry_access().get(&mcrust::net::minecraft::world::timeline::timelines::overworld_day());
        let mut timeline: Option<Reference> = todo!("fabric2seer: the initialiser of `timeline` (TODO above)");
        if timeline.is_some() {
            // TODO(fabric2seer): `.getPeriodCount(..)` on a value of unknown type
            //   java: int dayCount = timeline.get().value().getPeriodCount(clockManager);
            let mut day_count: i32 = todo!("fabric2seer: the initialiser of `dayCount` (TODO above)");
            if crate::de::hysky::skyblocker::debug::debug::Debug::debug_enabled() && crate::de::hysky::skyblocker::debug::debug::Debug::web_socket_debug() {
                logger().info(&format!("[Skyblocker WebSocket] CH Day Count: {}", day_count));
            }
            if day_count >= 0 && (day_count as i64) < MAX_LOBBY_LIFETIME {
                let mut close_time: i64 = (compat::millis()).wrapping_add((MAX_LOBBY_LIFETIME.wrapping_sub(day_count as i64)).wrapping_mul(*MILLIS_PER_MINECRAFT_DAY));
                return CrystalsWaypointSubscribeMessage::new(close_time.wrapping_div(MILLIS_TO_SECONDS));
            }
        }
        let mut default_close_time: i64 = (compat::millis()).wrapping_add(MAX_LOBBY_LIFETIME.wrapping_mul(*MILLIS_PER_MINECRAFT_DAY));
        logger().warn(&format!("[Skyblocker WebSocket] Falling back to default CH close time of {} due to unavailable timeline.", default_close_time));
        return CrystalsWaypointSubscribeMessage::new(default_close_time.wrapping_div(MILLIS_TO_SECONDS));
    }

}

impl Message<crate::de::hysky::skyblocker::utils::ws::message::crystals_waypoint_subscribe_message::CrystalsWaypointSubscribeMessage> for CrystalsWaypointSubscribeMessage {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public Codec<CrystalsWaypointSubscribeMessage> getCodec()
    fn get_codec(&self) -> CodecRef<CrystalsWaypointSubscribeMessage> {
        return CODEC.get_mut().clone();
    }

}

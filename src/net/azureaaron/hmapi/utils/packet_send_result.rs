//! Ported from net.azureaaron.hmapi.utils.PacketSendResult.

/// java: public record PacketSendResult(boolean sent, OptionalLong retryAfter)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PacketSendResult {
    pub sent: bool,
    pub retry_after: Option<i64>,
}

impl PacketSendResult {
    /// java: public static PacketSendResult success()
    pub fn success() -> PacketSendResult {
        PacketSendResult { sent: true, retry_after: None }
    }

    /// java: public static PacketSendResult onCooldown(long retryAfter)
    pub fn on_cooldown(retry_after: i64) -> PacketSendResult {
        PacketSendResult { sent: false, retry_after: Some(retry_after) }
    }
}

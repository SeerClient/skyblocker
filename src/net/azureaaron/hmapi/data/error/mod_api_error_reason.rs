//! Ported from net.azureaaron.hmapi.data.error.ModApiErrorReason.

use super::error_reason::ErrorReason;
use super::internal_error_reason::InternalErrorReason;

/// java: public enum ModApiErrorReason implements ErrorReason
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ModApiErrorReason {
    Disabled,
    InternalServerError,
    RateLimited,
    InvalidPacketVersion,
    NoLongerSupported,
}

impl ModApiErrorReason {
    pub const VALUES: [ModApiErrorReason; 5] = [
        ModApiErrorReason::Disabled,
        ModApiErrorReason::InternalServerError,
        ModApiErrorReason::RateLimited,
        ModApiErrorReason::InvalidPacketVersion,
        ModApiErrorReason::NoLongerSupported,
    ];

    /// java: public int id()
    pub fn id(&self) -> i32 {
        match self {
            ModApiErrorReason::Disabled => 1,
            ModApiErrorReason::InternalServerError => 2,
            ModApiErrorReason::RateLimited => 3,
            ModApiErrorReason::InvalidPacketVersion => 4,
            ModApiErrorReason::NoLongerSupported => 5,
        }
    }

    /// java: private static final IntFunction<ModApiErrorReason> BY_ID = ByIdMap.sparse(ModApiErrorReason::id, values(), null);
    fn by_id(id: i32) -> Option<ModApiErrorReason> {
        Self::VALUES.into_iter().find(|reason| reason.id() == id)
    }

    /// java: public static ErrorReason tryResolveReason(int id)
    pub fn try_resolve_reason(id: i32) -> ErrorReason {
        match ModApiErrorReason::by_id(id) {
            Some(reason) => ErrorReason::ModApi(reason),
            None => ErrorReason::Internal(InternalErrorReason::Unknown),
        }
    }
}

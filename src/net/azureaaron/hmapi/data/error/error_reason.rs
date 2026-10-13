//! Ported from net.azureaaron.hmapi.data.error.ErrorReason.

use super::internal_error_reason::InternalErrorReason;
use super::mod_api_error_reason::ModApiErrorReason;

/// java: public sealed interface ErrorReason permits ModApiErrorReason, InternalErrorReason
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ErrorReason {
    ModApi(ModApiErrorReason),
    Internal(InternalErrorReason),
}

impl ErrorReason {
    /// java: int id();
    pub fn id(&self) -> i32 {
        match self {
            ErrorReason::ModApi(reason) => reason.id(),
            ErrorReason::Internal(reason) => reason.id(),
        }
    }
}

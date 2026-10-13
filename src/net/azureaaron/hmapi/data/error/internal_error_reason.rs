//! Ported from net.azureaaron.hmapi.data.error.InternalErrorReason.

/// java: public enum InternalErrorReason implements ErrorReason
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum InternalErrorReason {
    Unknown,
}

impl InternalErrorReason {
    /// java: public int id()
    pub fn id(&self) -> i32 {
        match self {
            InternalErrorReason::Unknown => 0,
        }
    }
}

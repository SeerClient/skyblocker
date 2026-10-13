//! net.azureaaron.hmapi.data.server.Environment - ported into `crate::unresolved`, re-exported here at its own path.
pub use crate::unresolved::Environment;

impl Environment {
    pub const VALUES: [Environment; 3] = [Environment::Production, Environment::Beta, Environment::Test];

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }
}

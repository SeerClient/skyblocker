//! Ported from de.hysky.skyblocker.utils.render.gui.ServerTransferHelper by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: private static boolean interrupted = false;
pub static INTERRUPTED: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));

/// java: public class ServerTransferHelper
#[derive(Clone)]
pub struct ServerTransferHelper {}

impl ServerTransferHelper {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static boolean isInterrupted()
    pub fn is_interrupted() -> bool {
        return compat::get_state(&INTERRUPTED);
    }

    /// java: public static void setInterrupted(boolean interrupted)
    pub fn set_interrupted(mut interrupted: bool) {
        compat::set_state(&INTERRUPTED, interrupted);
    }

}

//! Ported from de.hysky.skyblocker.skyblock.garden.CurrentJacobCrop by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: public static @Nullable String CURRENT_CROP_CONTEST;
pub static CURRENT_CROP_CONTEST: LazyLock<Mutex<Option<String>>> = LazyLock::new(|| Mutex::new(None));

/// java: public final class CurrentJacobCrop
#[derive(Clone)]
pub struct CurrentJacobCrop {}

impl CurrentJacobCrop {
    /// java: private CurrentJacobCrop()
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

}

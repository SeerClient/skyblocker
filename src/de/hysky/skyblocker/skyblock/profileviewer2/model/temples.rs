//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.model.Temples by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: public class Temples
#[derive(Clone)]
pub struct Temples {
    /// java: @SerializedName("unlocked_temples") public List<String> unlockedTemples = List.of();
    pub unlocked_temples: Vec<String>,
}

impl Temples {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            unlocked_temples: vec![],
        };
        this
    }

}

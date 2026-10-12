//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.model.WinterPlayerData by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: public class WinterPlayerData
#[derive(Clone)]
pub struct WinterPlayerData {
    /// java: @SerializedName("refined_jyrre_uses") public int refinedJyrreUses;
    pub refined_jyrre_uses: i32,
}

impl WinterPlayerData {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            refined_jyrre_uses: 0,
        };
        this
    }

}

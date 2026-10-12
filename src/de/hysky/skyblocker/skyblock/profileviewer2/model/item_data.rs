//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.model.ItemData by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: public class ItemData
#[derive(Clone)]
pub struct ItemData {
    /// java: public int soulflow;
    pub soulflow: i32,
    /// java: @SerializedName("favorite_arrow") public String favouriteArrow = "";
    pub favourite_arrow: String,
}

impl ItemData {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            soulflow: 0,
            favourite_arrow: "".to_string(),
        };
        this
    }

}

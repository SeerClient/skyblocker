//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.model.FairySouls by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: public class FairySouls
#[derive(Clone)]
pub struct FairySouls {
    /// java: @SerializedName("fairy_exchanges") public int fairyExchanges;
    pub fairy_exchanges: i32,
    /// java: @SerializedName("total_collected") public int totalCollected;
    pub total_collected: i32,
    /// java: @SerializedName("unspent_souls") public int unspentSouls;
    pub unspent_souls: i32,
}

impl FairySouls {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            fairy_exchanges: 0,
            total_collected: 0,
            unspent_souls: 0,
        };
        this
    }

}

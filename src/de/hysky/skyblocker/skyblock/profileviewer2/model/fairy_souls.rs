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

impl FairySouls {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "fairy_exchanges") {
            if let Some(__x) = gson::as_i32(__v) {
                self.fairy_exchanges = __x;
            }
        }
        if let Some(__v) = gson::member(json, "total_collected") {
            if let Some(__x) = gson::as_i32(__v) {
                self.total_collected = __x;
            }
        }
        if let Some(__v) = gson::member(json, "unspent_souls") {
            if let Some(__x) = gson::as_i32(__v) {
                self.unspent_souls = __x;
            }
        }
    }

    /// `GSON.fromJson(json, FairySouls.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> Self {
        let mut __this = Self::new();
        __this.read_json_fields(json);
        __this
    }
}

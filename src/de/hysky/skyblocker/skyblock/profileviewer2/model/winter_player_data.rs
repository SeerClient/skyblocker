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

impl WinterPlayerData {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "refined_jyrre_uses") {
            if let Some(__x) = gson::as_i32(__v) {
                self.refined_jyrre_uses = __x;
            }
        }
    }

    /// `GSON.fromJson(json, WinterPlayerData.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> Self {
        let mut __this = Self::new();
        __this.read_json_fields(json);
        __this
    }
}

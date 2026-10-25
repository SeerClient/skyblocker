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

impl ItemData {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "soulflow") {
            if let Some(__x) = gson::as_i32(__v) {
                self.soulflow = __x;
            }
        }
        if let Some(__v) = gson::member(json, "favorite_arrow") {
            if let Some(__x) = gson::as_string(__v) {
                self.favourite_arrow = __x;
            }
        }
    }

    /// `GSON.fromJson(json, ItemData.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> Self {
        let mut __this = Self::new();
        __this.read_json_fields(json);
        __this
    }
}

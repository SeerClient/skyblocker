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

impl Temples {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "unlocked_temples") {
            if let Some(__x) = gson::list(__v, |__e0| gson::as_string(__e0)) {
                self.unlocked_temples = __x;
            }
        }
    }

    /// `GSON.fromJson(json, Temples.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> Self {
        let mut __this = Self::new();
        __this.read_json_fields(json);
        __this
    }
}

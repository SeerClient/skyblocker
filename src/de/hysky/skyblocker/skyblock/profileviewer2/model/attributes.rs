//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.model.Attributes by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: public class Attributes
#[derive(Clone)]
pub struct Attributes {
    /// java: public Map<String, Integer> stacks = Map.of();
    pub stacks: std::collections::HashMap<String, i32>,
}

impl Attributes {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            stacks: std::collections::HashMap::from([]),
        };
        this
    }

}

impl Attributes {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "stacks") {
            if let Some(__x) = gson::map(__v, gson::string_key, |__e0| gson::as_i32(__e0)) {
                self.stacks = __x;
            }
        }
    }

    /// `GSON.fromJson(json, Attributes.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> Self {
        let mut __this = Self::new();
        __this.read_json_fields(json);
        __this
    }
}

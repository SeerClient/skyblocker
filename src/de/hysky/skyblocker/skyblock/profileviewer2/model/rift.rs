//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.model.Rift by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: public class Rift
#[derive(Clone)]
pub struct Rift {
    /// java: public Access access = new Access();
    pub access: Access,
}

impl Rift {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            access: Access::new(),
        };
        this
    }

}

/// java: public static class Access
#[derive(Clone)]
pub struct Access {
    /// java: @SerializedName("consumed_prism") public boolean consumedPrism;
    pub consumed_prism: bool,
}

impl Access {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            consumed_prism: false,
        };
        this
    }

}

impl Rift {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "access") {
            if let Some(__x) = Some(Access::from_json(__v)) {
                self.access = __x;
            }
        }
    }

    /// `GSON.fromJson(json, Rift.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> Self {
        let mut __this = Self::new();
        __this.read_json_fields(json);
        __this
    }
}

impl Access {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "consumed_prism") {
            if let Some(__x) = gson::as_bool(__v) {
                self.consumed_prism = __x;
            }
        }
    }

    /// `GSON.fromJson(json, Access.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> Self {
        let mut __this = Self::new();
        __this.read_json_fields(json);
        __this
    }
}

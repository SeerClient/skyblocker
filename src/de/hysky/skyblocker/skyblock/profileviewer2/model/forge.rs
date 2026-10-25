//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.model.Forge by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: public class Forge
#[derive(Clone)]
pub struct Forge {
    /// java: @SerializedName("forge_processes") public ForgeProcesses forgeProcesses = new ForgeProcesses();
    pub forge_processes: ForgeProcesses,
}

impl Forge {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            forge_processes: ForgeProcesses::new(),
        };
        this
    }

}

/// java: public static class ForgeProcesses
#[derive(Clone)]
pub struct ForgeProcesses {
    /// java: public Map<String, Node> forge_1 = Map.of();
    pub forge_1: std::collections::HashMap<String, Node>,
}

impl ForgeProcesses {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            forge_1: std::collections::HashMap::from([]),
        };
        this
    }

}

/// java: public static class Node
#[derive(Clone)]
pub struct Node {
    /// java: public String type = "";
    pub r#type: String,
    /// java: public String id = "";
    pub id: String,
    /// java: public long startTime;
    pub start_time: i64,
    /// java: public int slot;
    pub slot: i32,
    /// java: public boolean notified = false;
    pub notified: bool,
}

impl Node {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            r#type: "".to_string(),
            id: "".to_string(),
            start_time: 0,
            slot: 0,
            notified: false,
        };
        this
    }

}

impl Forge {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "forge_processes") {
            if let Some(__x) = Some(ForgeProcesses::from_json(__v)) {
                self.forge_processes = __x;
            }
        }
    }

    /// `GSON.fromJson(json, Forge.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> Self {
        let mut __this = Self::new();
        __this.read_json_fields(json);
        __this
    }
}

impl ForgeProcesses {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "forge_1") {
            if let Some(__x) = gson::map(__v, gson::string_key, |__e0| Some(Node::from_json(__e0))) {
                self.forge_1 = __x;
            }
        }
    }

    /// `GSON.fromJson(json, ForgeProcesses.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> Self {
        let mut __this = Self::new();
        __this.read_json_fields(json);
        __this
    }
}

impl Node {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "id") {
            if let Some(__x) = gson::as_string(__v) {
                self.id = __x;
            }
        }
        if let Some(__v) = gson::member(json, "startTime") {
            if let Some(__x) = gson::as_i64(__v) {
                self.start_time = __x;
            }
        }
        if let Some(__v) = gson::member(json, "slot") {
            if let Some(__x) = gson::as_i32(__v) {
                self.slot = __x;
            }
        }
        if let Some(__v) = gson::member(json, "notified") {
            if let Some(__x) = gson::as_bool(__v) {
                self.notified = __x;
            }
        }
    }

    /// `GSON.fromJson(json, Node.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> Self {
        let mut __this = Self::new();
        __this.read_json_fields(json);
        __this
    }
}

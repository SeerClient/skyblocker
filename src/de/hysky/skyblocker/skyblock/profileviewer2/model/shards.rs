//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.model.Shards by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::location::Location;
use mcrust::java::util::uuid::Uuid;
use mcrust::net::minecraft::core::block_pos::BlockPos;

/// java: public class Shards
#[derive(Clone)]
pub struct Shards {
    /// java: public Traps traps = new Traps();
    pub traps: Traps,
    /// java: public List<OwnedShard> owned = List.of();
    pub owned: Vec<OwnedShard>,
    /// java: public int fused;
    pub fused: i32,
}

impl Shards {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            traps: Traps::new(),
            owned: vec![],
            fused: 0,
        };
        this
    }

}

/// java: public static class Traps
#[derive(Clone)]
pub struct Traps {
    /// java: @SerializedName("active_traps") public List<ActiveTrap> activeTraps = List.of();
    pub active_traps: Vec<ActiveTrap>,
}

impl Traps {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            active_traps: vec![],
        };
        this
    }

}

/// java: public static class ActiveTrap
#[derive(Clone)]
pub struct ActiveTrap {
    /// java: @SerializedName("trap_item") public String trapItem = "";
    pub trap_item: String,
    /// java: @SerializedName("capture_time") public long captureTime;
    pub capture_time: i64,
    /// java: public String mode = "";
    pub mode: String,
    /// java: public String location = "";
    pub location: String,
    /// java: @SerializedName("placed_at") public long placedAt;
    pub placed_at: i64,
    /// java: public String shard = "";
    pub shard: String,
    /// java: public boolean captured;
    pub captured: bool,
    /// java: public boolean museum;
    pub museum: bool,
    /// java: public UUID uuid = UUID.randomUUID();
    pub uuid: Uuid,
}

impl ActiveTrap {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            trap_item: "".to_string(),
            capture_time: 0,
            mode: "".to_string(),
            location: "".to_string(),
            placed_at: 0,
            shard: "".to_string(),
            captured: false,
            museum: false,
            uuid: Uuid::random_uuid(),
        };
        this
    }

    /// java: public Location getIsland()
    pub fn get_island(&self) -> Location {
        return Location::from(&self.mode);
    }

    /// java: public BlockPos getPos()
    pub fn get_pos(&self) -> BlockPos {
        let mut split: Vec<String> = compat::regex::split(&self.location, ",");
        return BlockPos::containing((split[(0) as usize]).trim().parse::<f64>().expect("NumberFormatException"), (split[(1) as usize]).trim().parse::<f64>().expect("NumberFormatException"), (split[(2) as usize]).trim().parse::<f64>().expect("NumberFormatException"));
    }

}

/// java: public static class OwnedShard
#[derive(Clone)]
pub struct OwnedShard {
    /// java: public String type = "";
    pub r#type: String,
    /// java: @SerializedName("amount_owned") public int amountOwned;
    pub amount_owned: i32,
    /// java: public long captured;
    pub captured: i64,
}

impl OwnedShard {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            r#type: "".to_string(),
            amount_owned: 0,
            captured: 0,
        };
        this
    }

}

impl Shards {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "traps") {
            if let Some(__x) = Some(Traps::from_json(__v)) {
                self.traps = __x;
            }
        }
        if let Some(__v) = gson::member(json, "owned") {
            if let Some(__x) = gson::list(__v, |__e0| Some(OwnedShard::from_json(__e0))) {
                self.owned = __x;
            }
        }
        if let Some(__v) = gson::member(json, "fused") {
            if let Some(__x) = gson::as_i32(__v) {
                self.fused = __x;
            }
        }
    }

    /// `GSON.fromJson(json, Shards.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> Self {
        let mut __this = Self::new();
        __this.read_json_fields(json);
        __this
    }
}

impl Traps {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "active_traps") {
            if let Some(__x) = gson::list(__v, |__e0| Some(ActiveTrap::from_json(__e0))) {
                self.active_traps = __x;
            }
        }
    }

    /// `GSON.fromJson(json, Traps.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> Self {
        let mut __this = Self::new();
        __this.read_json_fields(json);
        __this
    }
}

impl ActiveTrap {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "trap_item") {
            if let Some(__x) = gson::as_string(__v) {
                self.trap_item = __x;
            }
        }
        if let Some(__v) = gson::member(json, "capture_time") {
            if let Some(__x) = gson::as_i64(__v) {
                self.capture_time = __x;
            }
        }
        if let Some(__v) = gson::member(json, "mode") {
            if let Some(__x) = gson::as_string(__v) {
                self.mode = __x;
            }
        }
        if let Some(__v) = gson::member(json, "location") {
            if let Some(__x) = gson::as_string(__v) {
                self.location = __x;
            }
        }
        if let Some(__v) = gson::member(json, "placed_at") {
            if let Some(__x) = gson::as_i64(__v) {
                self.placed_at = __x;
            }
        }
        if let Some(__v) = gson::member(json, "shard") {
            if let Some(__x) = gson::as_string(__v) {
                self.shard = __x;
            }
        }
        if let Some(__v) = gson::member(json, "captured") {
            if let Some(__x) = gson::as_bool(__v) {
                self.captured = __x;
            }
        }
        if let Some(__v) = gson::member(json, "museum") {
            if let Some(__x) = gson::as_bool(__v) {
                self.museum = __x;
            }
        }
        if let Some(__v) = gson::member(json, "uuid") {
            if let Some(__x) = crate::de::hysky::skyblocker::skyblock::profileviewer2::profile_viewer::json_uuid(__v) {
                self.uuid = __x;
            }
        }
    }

    /// `GSON.fromJson(json, ActiveTrap.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> Self {
        let mut __this = Self::new();
        __this.read_json_fields(json);
        __this
    }
}

impl OwnedShard {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "amount_owned") {
            if let Some(__x) = gson::as_i32(__v) {
                self.amount_owned = __x;
            }
        }
        if let Some(__v) = gson::member(json, "captured") {
            if let Some(__x) = gson::as_i64(__v) {
                self.captured = __x;
            }
        }
    }

    /// `GSON.fromJson(json, OwnedShard.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> Self {
        let mut __this = Self::new();
        __this.read_json_fields(json);
        __this
    }
}

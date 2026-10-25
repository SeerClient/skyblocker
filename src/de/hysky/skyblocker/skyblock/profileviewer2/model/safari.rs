//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.model.Safari by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: public class Safari
#[derive(Clone)]
pub struct Safari {
    /// java: public Tickets tickets = new Tickets();
    pub tickets: Tickets,
    /// java: @SerializedName("milestone_claimed_tiers") public AbstractBiomeData milestoneTiersClaimed = new AbstractBiomeData();
    pub milestone_tiers_claimed: compat::Shared<AbstractBiomeData>,
    /// java: @SerializedName("biome_captures") public AbstractBiomeData biomeCaptures = new AbstractBiomeData();
    pub biome_captures: compat::Shared<AbstractBiomeData>,
    /// java: @SerializedName("discovered_critters") public List<String> discoveredCritters = List.of();
    pub discovered_critters: Vec<String>,
    /// java: @SerializedName("discovered_sparkling_critters") public List<String> discoveredSparklingCritters = List.of();
    pub discovered_sparkling_critters: Vec<String>,
    /// java: @SerializedName("total_captured_sparkling_critters") public int totalSparklingCrittersCaptured;
    pub total_sparkling_critters_captured: i32,
}

impl Safari {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            tickets: Tickets::new(),
            milestone_tiers_claimed: AbstractBiomeData::new(),
            biome_captures: AbstractBiomeData::new(),
            discovered_critters: vec![],
            discovered_sparkling_critters: vec![],
            total_sparkling_critters_captured: 0,
        };
        this
    }

}

/// java: public static class Tickets
#[derive(Clone)]
pub struct Tickets {
    /// java: public int basic;
    pub basic: i32,
    /// java: public int economy;
    pub economy: i32,
    /// java: public int premium;
    pub premium: i32,
    /// java: @SerializedName("first_class") public int firstClass;
    pub first_class: i32,
}

impl Tickets {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            basic: 0,
            economy: 0,
            premium: 0,
            first_class: 0,
        };
        this
    }

}

/// java: public static class AbstractBiomeData
#[derive(Clone)]
pub struct AbstractBiomeData {
    /// java: public int cavern;
    pub cavern: i32,
    /// java: public int forest;
    pub forest: i32,
    /// java: public int haunted;
    pub haunted: i32,
    /// java: public int icy;
    pub icy: i32,
}

impl AbstractBiomeData {
    /// java: the implicit no-argument constructor
    pub fn new() -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            cavern: 0,
            forest: 0,
            haunted: 0,
            icy: 0,
        });
        __this
    }

}

impl Safari {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "tickets") {
            if let Some(__x) = Some(Tickets::from_json(__v)) {
                self.tickets = __x;
            }
        }
        if let Some(__v) = gson::member(json, "milestone_claimed_tiers") {
            if let Some(__x) = Some(AbstractBiomeData::from_json(__v)) {
                self.milestone_tiers_claimed = __x;
            }
        }
        if let Some(__v) = gson::member(json, "biome_captures") {
            if let Some(__x) = Some(AbstractBiomeData::from_json(__v)) {
                self.biome_captures = __x;
            }
        }
        if let Some(__v) = gson::member(json, "discovered_critters") {
            if let Some(__x) = gson::list(__v, |__e0| gson::as_string(__e0)) {
                self.discovered_critters = __x;
            }
        }
        if let Some(__v) = gson::member(json, "discovered_sparkling_critters") {
            if let Some(__x) = gson::list(__v, |__e0| gson::as_string(__e0)) {
                self.discovered_sparkling_critters = __x;
            }
        }
        if let Some(__v) = gson::member(json, "total_captured_sparkling_critters") {
            if let Some(__x) = gson::as_i32(__v) {
                self.total_sparkling_critters_captured = __x;
            }
        }
    }

    /// `GSON.fromJson(json, Safari.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> Self {
        let mut __this = Self::new();
        __this.read_json_fields(json);
        __this
    }
}

impl Tickets {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "basic") {
            if let Some(__x) = gson::as_i32(__v) {
                self.basic = __x;
            }
        }
        if let Some(__v) = gson::member(json, "economy") {
            if let Some(__x) = gson::as_i32(__v) {
                self.economy = __x;
            }
        }
        if let Some(__v) = gson::member(json, "premium") {
            if let Some(__x) = gson::as_i32(__v) {
                self.premium = __x;
            }
        }
        if let Some(__v) = gson::member(json, "first_class") {
            if let Some(__x) = gson::as_i32(__v) {
                self.first_class = __x;
            }
        }
    }

    /// `GSON.fromJson(json, Tickets.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> Self {
        let mut __this = Self::new();
        __this.read_json_fields(json);
        __this
    }
}

impl AbstractBiomeData {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "cavern") {
            if let Some(__x) = gson::as_i32(__v) {
                self.cavern = __x;
            }
        }
        if let Some(__v) = gson::member(json, "forest") {
            if let Some(__x) = gson::as_i32(__v) {
                self.forest = __x;
            }
        }
        if let Some(__v) = gson::member(json, "haunted") {
            if let Some(__x) = gson::as_i32(__v) {
                self.haunted = __x;
            }
        }
        if let Some(__v) = gson::member(json, "icy") {
            if let Some(__x) = gson::as_i32(__v) {
                self.icy = __x;
            }
        }
    }

    /// `GSON.fromJson(json, AbstractBiomeData.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> compat::Shared<Self> {
        let __this = Self::new();
        compat::write(&__this, |__t| __t.read_json_fields(json));
        __this
    }
}

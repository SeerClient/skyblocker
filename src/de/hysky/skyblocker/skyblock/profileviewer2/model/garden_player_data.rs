//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.model.GardenPlayerData by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: public class GardenPlayerData
#[derive(Clone)]
pub struct GardenPlayerData {
    /// java: public int copper;
    pub copper: i32,
    /// java: @SerializedName("larva_consumed") public int larvaConsumed;
    pub larva_consumed: i32,
    /// java: @SerializedName("analyzed_greenhouse_crops") public List<String> analyzedGreenhouseMutations = List.of();
    pub analyzed_greenhouse_mutations: Vec<String>,
    /// java: @SerializedName("discovered_greenhouse_crops") public List<String> discoveredGreenhouseMutations = List.of();
    pub discovered_greenhouse_mutations: Vec<String>,
}

impl GardenPlayerData {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            copper: 0,
            larva_consumed: 0,
            analyzed_greenhouse_mutations: vec![],
            discovered_greenhouse_mutations: vec![],
        };
        this
    }

}

impl GardenPlayerData {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "copper") {
            if let Some(__x) = gson::as_i32(__v) {
                self.copper = __x;
            }
        }
        if let Some(__v) = gson::member(json, "larva_consumed") {
            if let Some(__x) = gson::as_i32(__v) {
                self.larva_consumed = __x;
            }
        }
        if let Some(__v) = gson::member(json, "analyzed_greenhouse_crops") {
            if let Some(__x) = gson::list(__v, |__e0| gson::as_string(__e0)) {
                self.analyzed_greenhouse_mutations = __x;
            }
        }
        if let Some(__v) = gson::member(json, "discovered_greenhouse_crops") {
            if let Some(__x) = gson::list(__v, |__e0| gson::as_string(__e0)) {
                self.discovered_greenhouse_mutations = __x;
            }
        }
    }

    /// `GSON.fromJson(json, GardenPlayerData.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> Self {
        let mut __this = Self::new();
        __this.read_json_fields(json);
        __this
    }
}

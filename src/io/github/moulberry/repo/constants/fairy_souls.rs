//! Ported from io.github.moulberry.repo.constants.FairySouls (neurepoparser 1.12.0).

use std::collections::HashMap;

use serde_json::Value;

use crate::io::github::moulberry::repo::data::coordinate::{Coordinate, CoordinateSerializer};
use crate::io::github::moulberry::repo::neu_repository::gson;

/// java: @Getter public class FairySouls
#[derive(Clone, Debug, Default)]
pub struct FairySouls {
    /// java: int maxSouls;
    pub(crate) max_souls: i32,
    /// java: Map<String, List<Coordinate>> soulLocations = new HashMap<>();
    pub(crate) soul_locations: HashMap<String, Vec<Coordinate>>,
}

impl FairySouls {
    /// java: public FairySouls(Gson gson, Map<String, JsonElement> json)
    ///
    /// The repository's Gson reads each `List<Coordinate>` through `Coordinate.CoordinateSerializer`; its
    /// `JsonParseException`, and the `NullPointerException` of a missing `"Max Souls"`, are panics.
    pub fn new(json: &serde_json::Map<String, Value>) -> Self {
        let mut soul_locations: HashMap<String, Vec<Coordinate>> = HashMap::new();
        for (key, value) in json {
            // java: if (things.getValue().isJsonArray()) soulLocations.put(things.getKey(), gson.fromJson(things.getValue(), List<Coordinate>));
            if let Some(array) = value.as_array() {
                soul_locations.insert(
                    key.clone(),
                    array.iter().filter(|e| !e.is_null()).map(CoordinateSerializer::deserialize).collect(),
                );
            }
        }
        // java: maxSouls = json.get("Max Souls").getAsInt();
        let max_souls: i32 = json
            .get("Max Souls")
            .and_then(gson::as_i32)
            .unwrap_or_else(|| panic!("java.lang.NullPointerException: constants/fairy_souls.json has no \"Max Souls\""));
        FairySouls { max_souls, soul_locations }
    }

    /// java: public int getMaxSouls()
    pub fn get_max_souls(&self) -> i32 {
        self.max_souls
    }

    /// java: public Map<String, List<Coordinate>> getSoulLocations()
    pub fn get_soul_locations(&self) -> &HashMap<String, Vec<Coordinate>> {
        &self.soul_locations
    }
}

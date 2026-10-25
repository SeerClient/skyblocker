//! Ported from io.github.moulberry.repo.constants.AbiphoneContact (neurepoparser 1.12.0).

use serde_json::Value;

use crate::io::github::moulberry::repo::neu_repository::gson;

/// java: @Getter public class AbiphoneContact
#[derive(Clone, Debug, Default)]
pub struct AbiphoneContact {
    /// java: @Nullable List<String> callNames;
    pub(crate) call_names: Option<Vec<String>>,
    /// java: @SerializedName("requirement") List<String> requirements;
    pub(crate) requirements: Option<Vec<String>>,
    /// java: @Nullable String island;
    pub(crate) island: Option<String>,
    /// java: @Nullable Integer x;
    pub(crate) x: Option<i32>,
    /// java: @Nullable Integer y;
    pub(crate) y: Option<i32>,
    /// java: @Nullable Integer z;
    pub(crate) z: Option<i32>,
}

impl AbiphoneContact {
    /// Gson's reflective binding of the class.
    pub fn from_json(json: &Value) -> Self {
        AbiphoneContact {
            call_names: gson::string_list(json, "callNames"),
            requirements: gson::string_list(json, "requirement"),
            island: gson::string(json, "island"),
            x: gson::integer(json, "x"),
            y: gson::integer(json, "y"),
            z: gson::integer(json, "z"),
        }
    }

    /// java: public @Nullable List<String> getCallNames()
    pub fn get_call_names(&self) -> Option<&[String]> {
        self.call_names.as_deref()
    }

    /// java: public List<String> getRequirements()
    pub fn get_requirements(&self) -> Option<&[String]> {
        self.requirements.as_deref()
    }

    /// java: public @Nullable String getIsland()
    pub fn get_island(&self) -> Option<&str> {
        self.island.as_deref()
    }

    /// java: public @Nullable Integer getX()
    pub fn get_x(&self) -> Option<i32> {
        self.x
    }

    /// java: public @Nullable Integer getY()
    pub fn get_y(&self) -> Option<i32> {
        self.y
    }

    /// java: public @Nullable Integer getZ()
    pub fn get_z(&self) -> Option<i32> {
        self.z
    }
}

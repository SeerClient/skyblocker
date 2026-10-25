//! Ported from io.github.moulberry.repo.constants.RankData (neurepoparser 1.12.0).

use serde_json::Value;

use crate::io::github::moulberry::repo::neu_repository::gson;

/// java: @Value public final class RankData
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct RankData {
    /// java: String color;
    color: Option<String>,
    /// java: String tag;
    tag: Option<String>,
    /// java: @Nullable String plus;
    plus: Option<String>,
}

impl RankData {
    /// java: public RankData(final String color, final String tag, @Nullable final String plus)
    pub fn new(color: Option<String>, tag: Option<String>, plus: Option<String>) -> Self {
        RankData { color, tag, plus }
    }

    /// Gson's reflective binding of the class.
    pub fn from_json(json: &Value) -> Self {
        RankData { color: gson::string(json, "color"), tag: gson::string(json, "tag"), plus: gson::string(json, "plus") }
    }

    /// java: public String getColor()
    pub fn get_color(&self) -> Option<&str> {
        self.color.as_deref()
    }

    /// java: public String getTag()
    pub fn get_tag(&self) -> Option<&str> {
        self.tag.as_deref()
    }

    /// java: public @Nullable String getPlus()
    pub fn get_plus(&self) -> Option<&str> {
        self.plus.as_deref()
    }
}

impl std::fmt::Display for RankData {
    /// java: public String toString()
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RankData(color={}, tag={}, plus={})",
            self.color.as_deref().unwrap_or("null"),
            self.tag.as_deref().unwrap_or("null"),
            self.plus.as_deref().unwrap_or("null")
        )
    }
}

//! java: public record PetInfo(String type, double xp, String tier, int candies, Optional<String> heldItem, Optional<String> skin)
//! (net.azureaaron.networth.item)
//!
//! `CODEC` is not ported: nothing in Skyblocker reads it (its only user is the library's own profile parsing).

/// java: public record PetInfo(String type, double xp, String tier, int candies, Optional<String> heldItem, Optional<String> skin)
#[derive(Clone, Debug, PartialEq)]
pub struct PetInfo {
    pub r#type: String,
    pub xp: f64,
    pub tier: String,
    pub candies: i32,
    pub held_item: Option<String>,
    pub skin: Option<String>,
}

impl PetInfo {
    /// java: the canonical record constructor
    pub fn new(
        r#type: String,
        xp: f64,
        tier: String,
        candies: i32,
        held_item: Option<String>,
        skin: Option<String>,
    ) -> Self {
        Self {
            r#type,
            xp,
            tier,
            candies,
            held_item,
            skin,
        }
    }

    /// java: public static final PetInfo EMPTY = new PetInfo("", 0, "", 0, Optional.empty(), Optional.empty());
    pub fn empty() -> Self {
        Self::new(String::new(), 0.0, String::new(), 0, None, None)
    }

    /// java: the record accessor `type()`
    pub fn r#type(&self) -> &str {
        &self.r#type
    }

    /// java: the record accessor `xp()`
    pub fn xp(&self) -> f64 {
        self.xp
    }

    /// java: the record accessor `tier()`
    pub fn tier(&self) -> &str {
        &self.tier
    }

    /// java: the record accessor `candies()`
    pub fn candies(&self) -> i32 {
        self.candies
    }

    /// java: the record accessor `heldItem()`
    pub fn held_item(&self) -> Option<&String> {
        self.held_item.as_ref()
    }

    /// java: the record accessor `skin()`
    pub fn skin(&self) -> Option<&String> {
        self.skin.as_ref()
    }
}

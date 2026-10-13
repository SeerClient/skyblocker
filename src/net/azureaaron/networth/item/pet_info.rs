//! java: public record PetInfo(String type, double xp, String tier, int candies, Optional<String> heldItem, Optional<String> skin)
//! (net.azureaaron.networth.item)

use std::sync::LazyLock;

use mcrust::com::mojang::serialization::codec::{self, CodecRef};
use mcrust::com::mojang::serialization::codecs::record_codec_builder;

/// java: public static final Codec<PetInfo> CODEC
pub static CODEC: LazyLock<CodecRef<PetInfo>> = LazyLock::new(|| {
    record_codec_builder::create(record_codec_builder::apply6(
        record_codec_builder::of(
            |o: &PetInfo| o.r#type.clone(),
            codec::field_of(codec::string_codec(), "type"),
        ),
        record_codec_builder::of(
            |o: &PetInfo| o.xp,
            codec::field_of(codec::double_codec(), "exp"),
        ),
        record_codec_builder::of(
            |o: &PetInfo| o.tier.clone(),
            codec::field_of(codec::string_codec(), "tier"),
        ),
        record_codec_builder::of(
            |o: &PetInfo| o.candies,
            codec::optional_field_of(codec::int_codec(), "candyUsed", 0),
        ),
        record_codec_builder::of(
            |o: &PetInfo| o.held_item.clone(),
            codec::optional_field_of_opt(codec::string_codec(), "heldItem"),
        ),
        record_codec_builder::of(
            |o: &PetInfo| o.skin.clone(),
            codec::optional_field_of_opt(codec::string_codec(), "skin"),
        ),
        PetInfo::new,
    ))
});

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

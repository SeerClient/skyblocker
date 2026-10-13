//! java: @ApiStatus.Internal public record AccessoryUpgrades(Optional<String> enrichment, OptionalInt thunderCharges)
//! (net.azureaaron.networth.item)

use std::sync::LazyLock;

use mcrust::com::mojang::serialization::codec::{self, CodecRef};
use mcrust::com::mojang::serialization::codecs::record_codec_builder;

use crate::net::azureaaron::networth::utils::codec_utils::CodecUtils;

/// java: static final Codec<AccessoryUpgrades> CODEC
pub(crate) static CODEC: LazyLock<CodecRef<AccessoryUpgrades>> = LazyLock::new(|| {
    record_codec_builder::create(record_codec_builder::apply2(
        record_codec_builder::of(
            |o: &AccessoryUpgrades| o.enrichment.clone(),
            codec::optional_field_of_opt(codec::string_codec(), "talisman_enrichment"),
        ),
        record_codec_builder::of(
            |o: &AccessoryUpgrades| o.thunder_charges,
            CodecUtils::optional_int(codec::optional_field_of_opt(
                codec::int_codec(),
                "thunder_charge",
            )),
        ),
        AccessoryUpgrades::new,
    ))
});

/// java: @ApiStatus.Internal public record AccessoryUpgrades(Optional<String> enrichment, OptionalInt thunderCharges)
#[derive(Clone, Debug, PartialEq)]
pub struct AccessoryUpgrades {
    pub enrichment: Option<String>,
    pub thunder_charges: Option<i32>,
}

impl AccessoryUpgrades {
    /// java: the canonical record constructor
    pub fn new(enrichment: Option<String>, thunder_charges: Option<i32>) -> Self {
        Self {
            enrichment,
            thunder_charges,
        }
    }

    /// java: the record accessor `enrichment()`
    pub fn enrichment(&self) -> Option<&String> {
        self.enrichment.as_ref()
    }

    /// java: the record accessor `thunderCharges()`
    pub fn thunder_charges(&self) -> Option<i32> {
        self.thunder_charges
    }
}

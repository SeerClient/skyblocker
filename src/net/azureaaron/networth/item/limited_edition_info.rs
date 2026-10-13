//! java: @ApiStatus.Internal public record LimitedEditionInfo(OptionalInt newYearCakeYear, Optional<String> partyHatColour, Optional<String> partyHatEmoji, Optional<String> abicaseModel)
//! (net.azureaaron.networth.item)

use std::sync::LazyLock;

use mcrust::com::mojang::serialization::codec::{self, CodecRef};
use mcrust::com::mojang::serialization::codecs::record_codec_builder;

use crate::net::azureaaron::networth::utils::codec_utils::CodecUtils;

/// java: static final Codec<LimitedEditionInfo> CODEC
pub(crate) static CODEC: LazyLock<CodecRef<LimitedEditionInfo>> = LazyLock::new(|| {
    record_codec_builder::create(record_codec_builder::apply4(
        record_codec_builder::of(
            |o: &LimitedEditionInfo| o.new_year_cake_year,
            CodecUtils::optional_int(codec::optional_field_of_opt(
                codec::int_codec(),
                "new_years_cake",
            )),
        ),
        record_codec_builder::of(
            |o: &LimitedEditionInfo| o.party_hat_colour.clone(),
            codec::optional_field_of_opt(codec::string_codec(), "party_hat_color"),
        ),
        record_codec_builder::of(
            |o: &LimitedEditionInfo| o.party_hat_emoji.clone(),
            codec::optional_field_of_opt(codec::string_codec(), "party_hat_emoji"),
        ),
        record_codec_builder::of(
            |o: &LimitedEditionInfo| o.abicase_model.clone(),
            codec::optional_field_of_opt(codec::string_codec(), "model"),
        ),
        LimitedEditionInfo::new,
    ))
});

/// java: @ApiStatus.Internal public record LimitedEditionInfo(OptionalInt newYearCakeYear, Optional<String> partyHatColour, Optional<String> partyHatEmoji, Optional<String> abicaseModel)
#[derive(Clone, Debug, PartialEq)]
pub struct LimitedEditionInfo {
    pub new_year_cake_year: Option<i32>,
    pub party_hat_colour: Option<String>,
    pub party_hat_emoji: Option<String>,
    pub abicase_model: Option<String>,
}

impl LimitedEditionInfo {
    /// java: the canonical record constructor
    pub fn new(
        new_year_cake_year: Option<i32>,
        party_hat_colour: Option<String>,
        party_hat_emoji: Option<String>,
        abicase_model: Option<String>,
    ) -> Self {
        Self {
            new_year_cake_year,
            party_hat_colour,
            party_hat_emoji,
            abicase_model,
        }
    }

    /// java: the record accessor `newYearCakeYear()`
    pub fn new_year_cake_year(&self) -> Option<i32> {
        self.new_year_cake_year
    }

    /// java: the record accessor `partyHatColour()`
    pub fn party_hat_colour(&self) -> Option<&String> {
        self.party_hat_colour.as_ref()
    }

    /// java: the record accessor `partyHatEmoji()`
    pub fn party_hat_emoji(&self) -> Option<&String> {
        self.party_hat_emoji.as_ref()
    }

    /// java: the record accessor `abicaseModel()`
    pub fn abicase_model(&self) -> Option<&String> {
        self.abicase_model.as_ref()
    }
}

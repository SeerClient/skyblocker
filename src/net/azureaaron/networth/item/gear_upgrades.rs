//! java: @ApiStatus.Internal public record GearUpgrades(OptionalInt hotPotatoes, OptionalInt artOfWars, OptionalInt woodSingularities, OptionalInt artOfPeaces, Optional<String> powerScroll)
//! (net.azureaaron.networth.item)

use std::sync::LazyLock;

use mcrust::com::mojang::serialization::codec::{self, CodecRef};
use mcrust::com::mojang::serialization::codecs::record_codec_builder;

use crate::net::azureaaron::networth::utils::codec_utils::CodecUtils;

/// java: static final Codec<GearUpgrades> CODEC
pub(crate) static CODEC: LazyLock<CodecRef<GearUpgrades>> = LazyLock::new(|| {
    record_codec_builder::create(record_codec_builder::apply5(
        record_codec_builder::of(
            |o: &GearUpgrades| o.hot_potatoes,
            CodecUtils::optional_int(codec::optional_field_of_opt(
                codec::int_codec(),
                "hot_potato_count",
            )),
        ),
        record_codec_builder::of(
            |o: &GearUpgrades| o.art_of_wars,
            CodecUtils::optional_int(codec::optional_field_of_opt(
                codec::int_codec(),
                "art_of_war_count",
            )),
        ),
        record_codec_builder::of(
            |o: &GearUpgrades| o.wood_singularities,
            CodecUtils::optional_int(codec::optional_field_of_opt(
                codec::int_codec(),
                "wood_singularity_count",
            )),
        ),
        record_codec_builder::of(
            |o: &GearUpgrades| o.art_of_peaces,
            CodecUtils::optional_int(codec::optional_field_of_opt(
                codec::int_codec(),
                "artOfPeaceApplied",
            )),
        ),
        record_codec_builder::of(
            |o: &GearUpgrades| o.power_scroll.clone(),
            codec::optional_field_of_opt(codec::string_codec(), "power_ability_scroll"),
        ),
        GearUpgrades::new,
    ))
});

/// java: @ApiStatus.Internal public record GearUpgrades(OptionalInt hotPotatoes, OptionalInt artOfWars, OptionalInt woodSingularities, OptionalInt artOfPeaces, Optional<String> powerScroll)
#[derive(Clone, Debug, PartialEq)]
pub struct GearUpgrades {
    pub hot_potatoes: Option<i32>,
    pub art_of_wars: Option<i32>,
    pub wood_singularities: Option<i32>,
    pub art_of_peaces: Option<i32>,
    pub power_scroll: Option<String>,
}

impl GearUpgrades {
    /// java: the canonical record constructor
    pub fn new(
        hot_potatoes: Option<i32>,
        art_of_wars: Option<i32>,
        wood_singularities: Option<i32>,
        art_of_peaces: Option<i32>,
        power_scroll: Option<String>,
    ) -> Self {
        Self {
            hot_potatoes,
            art_of_wars,
            wood_singularities,
            art_of_peaces,
            power_scroll,
        }
    }

    /// java: the record accessor `hotPotatoes()`
    pub fn hot_potatoes(&self) -> Option<i32> {
        self.hot_potatoes
    }

    /// java: the record accessor `artOfWars()`
    pub fn art_of_wars(&self) -> Option<i32> {
        self.art_of_wars
    }

    /// java: the record accessor `woodSingularities()`
    pub fn wood_singularities(&self) -> Option<i32> {
        self.wood_singularities
    }

    /// java: the record accessor `artOfPeaces()`
    pub fn art_of_peaces(&self) -> Option<i32> {
        self.art_of_peaces
    }

    /// java: the record accessor `powerScroll()`
    pub fn power_scroll(&self) -> Option<&String> {
        self.power_scroll.as_ref()
    }
}

//! java: @ApiStatus.Internal public record Cosmetics(Optional<Object2IntMap<String>> runes, Optional<String> skin, Optional<String> dye, boolean shiny)
//! (net.azureaaron.networth.item)

use std::sync::LazyLock;

use mcrust::com::mojang::serialization::codec::{self, CodecRef};
use mcrust::com::mojang::serialization::codecs::record_codec_builder;

use crate::net::azureaaron::networth::utils::codec_utils::CodecUtils;

/// java: static final Codec<Cosmetics> CODEC
pub(crate) static CODEC: LazyLock<CodecRef<Cosmetics>> = LazyLock::new(|| {
    record_codec_builder::create(record_codec_builder::apply4(
        record_codec_builder::of(
            |o: &Cosmetics| o.runes.clone(),
            codec::optional_field_of_opt(
                CodecUtils::create_object2_int_map_codec(codec::string_codec()),
                "runes",
            ),
        ),
        record_codec_builder::of(
            |o: &Cosmetics| o.skin.clone(),
            codec::optional_field_of_opt(codec::string_codec(), "skin"),
        ),
        record_codec_builder::of(
            |o: &Cosmetics| o.dye.clone(),
            codec::optional_field_of_opt(codec::string_codec(), "dye_item"),
        ),
        record_codec_builder::of(
            |o: &Cosmetics| o.shiny,
            codec::optional_field_of(codec::bool_codec(), "is_shiny", false),
        ),
        Cosmetics::new,
    ))
});

/// java: @ApiStatus.Internal public record Cosmetics(Optional<Object2IntMap<String>> runes, Optional<String> skin, Optional<String> dye, boolean shiny)
#[derive(Clone, Debug, PartialEq)]
pub struct Cosmetics {
    pub runes: Option<Vec<(String, i32)>>,
    pub skin: Option<String>,
    pub dye: Option<String>,
    pub shiny: bool,
}

impl Cosmetics {
    /// java: the canonical record constructor
    pub fn new(
        runes: Option<Vec<(String, i32)>>,
        skin: Option<String>,
        dye: Option<String>,
        shiny: bool,
    ) -> Self {
        Self {
            runes,
            skin,
            dye,
            shiny,
        }
    }

    /// java: the record accessor `runes()`
    pub fn runes(&self) -> Option<&Vec<(String, i32)>> {
        self.runes.as_ref()
    }

    /// java: the record accessor `skin()`
    pub fn skin(&self) -> Option<&String> {
        self.skin.as_ref()
    }

    /// java: the record accessor `dye()`
    pub fn dye(&self) -> Option<&String> {
        self.dye.as_ref()
    }

    /// java: the record accessor `shiny()`
    pub fn shiny(&self) -> bool {
        self.shiny
    }
}

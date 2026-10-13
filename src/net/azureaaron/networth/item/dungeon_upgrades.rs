//! java: @ApiStatus.Internal public record DungeonUpgrades(/* Mob Drops */ OptionalInt itemTier, /* Essence Upgrades */ OptionalInt itemLevel, Optional<List<String>> abilityScrolls)
//! (net.azureaaron.networth.item)

use std::sync::LazyLock;

use mcrust::com::mojang::serialization::codec::{self, CodecRef};
use mcrust::com::mojang::serialization::codecs::record_codec_builder;

use crate::net::azureaaron::networth::utils::codec_utils::CodecUtils;

/// java: static final Codec<DungeonUpgrades> CODEC
pub(crate) static CODEC: LazyLock<CodecRef<DungeonUpgrades>> = LazyLock::new(|| {
    record_codec_builder::create(record_codec_builder::apply3(
        record_codec_builder::of(
            |o: &DungeonUpgrades| o.item_tier,
            CodecUtils::optional_int(codec::optional_field_of_opt(
                codec::int_codec(),
                "item_tier",
            )),
        ),
        record_codec_builder::of(
            |o: &DungeonUpgrades| o.item_level,
            CodecUtils::optional_int(codec::optional_field_of_opt(
                codec::int_codec(),
                "dungeon_item_level",
            )),
        ),
        record_codec_builder::of(
            |o: &DungeonUpgrades| o.ability_scrolls.clone(),
            codec::optional_field_of_opt(codec::list(codec::string_codec()), "ability_scroll"),
        ),
        DungeonUpgrades::new,
    ))
});

/// java: @ApiStatus.Internal public record DungeonUpgrades(/* Mob Drops */ OptionalInt itemTier, /* Essence Upgrades */ OptionalInt itemLevel, Optional<List<String>> abilityScrolls)
#[derive(Clone, Debug, PartialEq)]
pub struct DungeonUpgrades {
    pub item_tier: Option<i32>,
    pub item_level: Option<i32>,
    pub ability_scrolls: Option<Vec<String>>,
}

impl DungeonUpgrades {
    /// java: the canonical record constructor
    pub fn new(
        item_tier: Option<i32>,
        item_level: Option<i32>,
        ability_scrolls: Option<Vec<String>>,
    ) -> Self {
        Self {
            item_tier,
            item_level,
            ability_scrolls,
        }
    }

    /// java: the record accessor `itemTier()`
    pub fn item_tier(&self) -> Option<i32> {
        self.item_tier
    }

    /// java: the record accessor `itemLevel()`
    pub fn item_level(&self) -> Option<i32> {
        self.item_level
    }

    /// java: the record accessor `abilityScrolls()`
    pub fn ability_scrolls(&self) -> Option<&Vec<String>> {
        self.ability_scrolls.as_ref()
    }
}

//! java: @ApiStatus.Internal public record MiscModifiers(OptionalInt pocketSackInASacks, OptionalInt jalapenoBooks, OptionalInt manaDisintegrators, OptionalInt transmissionTuners, OptionalInt ethermerge, OptionalInt farming4Dummies)
//! (net.azureaaron.networth.item)

use std::sync::LazyLock;

use mcrust::com::mojang::serialization::codec::{self, CodecRef};
use mcrust::com::mojang::serialization::codecs::record_codec_builder;

use crate::net::azureaaron::networth::utils::codec_utils::CodecUtils;

/// java: static final Codec<MiscModifiers> CODEC
pub(crate) static CODEC: LazyLock<CodecRef<MiscModifiers>> = LazyLock::new(|| {
    record_codec_builder::create(record_codec_builder::apply6(
        record_codec_builder::of(
            |o: &MiscModifiers| o.pocket_sack_in_a_sacks,
            CodecUtils::optional_int(codec::optional_field_of_opt(codec::int_codec(), "sack_pss")),
        ),
        record_codec_builder::of(
            |o: &MiscModifiers| o.jalapeno_books,
            CodecUtils::optional_int(codec::optional_field_of_opt(
                codec::int_codec(),
                "jalapeno_count",
            )),
        ),
        record_codec_builder::of(
            |o: &MiscModifiers| o.mana_disintegrators,
            CodecUtils::optional_int(codec::optional_field_of_opt(
                codec::int_codec(),
                "mana_disintegrator_count",
            )),
        ),
        record_codec_builder::of(
            |o: &MiscModifiers| o.transmission_tuners,
            CodecUtils::optional_int(codec::optional_field_of_opt(
                codec::int_codec(),
                "tuned_transmission",
            )),
        ),
        record_codec_builder::of(
            |o: &MiscModifiers| o.ethermerge,
            CodecUtils::optional_int(codec::optional_field_of_opt(
                codec::int_codec(),
                "ethermerge",
            )),
        ),
        record_codec_builder::of(
            |o: &MiscModifiers| o.farming4_dummies,
            CodecUtils::optional_int(codec::optional_field_of_opt(
                codec::int_codec(),
                "farming_for_dummies_count",
            )),
        ),
        MiscModifiers::new,
    ))
});

/// java: @ApiStatus.Internal public record MiscModifiers(OptionalInt pocketSackInASacks, OptionalInt jalapenoBooks, OptionalInt manaDisintegrators, OptionalInt transmissionTuners, OptionalInt ethermerge, OptionalInt farming4Dummies)
#[derive(Clone, Debug, PartialEq)]
pub struct MiscModifiers {
    pub pocket_sack_in_a_sacks: Option<i32>,
    pub jalapeno_books: Option<i32>,
    pub mana_disintegrators: Option<i32>,
    pub transmission_tuners: Option<i32>,
    pub ethermerge: Option<i32>,
    pub farming4_dummies: Option<i32>,
}

impl MiscModifiers {
    /// java: the canonical record constructor
    pub fn new(
        pocket_sack_in_a_sacks: Option<i32>,
        jalapeno_books: Option<i32>,
        mana_disintegrators: Option<i32>,
        transmission_tuners: Option<i32>,
        ethermerge: Option<i32>,
        farming4_dummies: Option<i32>,
    ) -> Self {
        Self {
            pocket_sack_in_a_sacks,
            jalapeno_books,
            mana_disintegrators,
            transmission_tuners,
            ethermerge,
            farming4_dummies,
        }
    }

    /// java: the record accessor `pocketSackInASacks()`
    pub fn pocket_sack_in_a_sacks(&self) -> Option<i32> {
        self.pocket_sack_in_a_sacks
    }

    /// java: the record accessor `jalapenoBooks()`
    pub fn jalapeno_books(&self) -> Option<i32> {
        self.jalapeno_books
    }

    /// java: the record accessor `manaDisintegrators()`
    pub fn mana_disintegrators(&self) -> Option<i32> {
        self.mana_disintegrators
    }

    /// java: the record accessor `transmissionTuners()`
    pub fn transmission_tuners(&self) -> Option<i32> {
        self.transmission_tuners
    }

    /// java: the record accessor `ethermerge()`
    pub fn ethermerge(&self) -> Option<i32> {
        self.ethermerge
    }

    /// java: the record accessor `farming4Dummies()`
    pub fn farming4_dummies(&self) -> Option<i32> {
        self.farming4_dummies
    }
}

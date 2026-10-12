//! Ported from de.hysky.skyblocker.utils.mayor.Minister by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::mayor::perk;
use crate::de::hysky::skyblocker::utils::mayor::perk::Perk;
use mcrust::com::mojang::serialization::codec;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::com::mojang::serialization::codecs::record_codec_builder;

/// java: public static final Minister EMPTY = new Minister("", "", Perk.EMPTY);
pub static EMPTY: LazyLock<Minister> = LazyLock::new(|| Minister::new("".to_string(), "".to_string(), perk::EMPTY.clone()));
/// java: public static final Codec<Minister> CODEC = RecordCodecBuilder.create(instance -> instance.group( Codec.STRING.fieldOf("key").forGetter(Minister::key), Codec.STRING.fieldOf("name").forGetter(Minister:
pub static CODEC: compat::MainThread<CodecRef<Minister>> = compat::MainThread::new(|| record_codec_builder::create(record_codec_builder::apply3(record_codec_builder::of(|o: &Minister| o.key(), codec::field_of(mcrust::com::mojang::serialization::codec::string_codec(), "key")), record_codec_builder::of(|o: &Minister| o.name(), codec::field_of(mcrust::com::mojang::serialization::codec::string_codec(), "name")), record_codec_builder::of(|o: &Minister| o.perk(), codec::optional_field_of(perk::CODEC.get_mut().clone(), "perk", perk::EMPTY.clone())), move |__a0, __a1, __a2| Minister::new(__a0.clone(), __a1.clone(), __a2.clone()))));

/// java: public record Minister(String key, String name, Perk perk)
#[derive(Clone, Debug, PartialEq)]
pub struct Minister {
    /// java: String key
    pub key: String,
    /// java: String name
    pub name: String,
    /// java: Perk perk
    pub perk: Perk,
}

impl Minister {
    /// java: the canonical record constructor
    pub fn new(mut key: String, mut name: String, mut perk: Perk) -> Self {
        Self { key, name, perk }
    }

    /// java: the record accessor `key()`
    pub fn key(&self) -> String {
        self.key.clone()
    }

    /// java: the record accessor `name()`
    pub fn name(&self) -> String {
        self.name.clone()
    }

    /// java: the record accessor `perk()`
    pub fn perk(&self) -> Perk {
        self.perk.clone()
    }

    /// java: @Override public String toString()
    pub fn to_string(&self) -> String {
        if self.is_empty() {
            return "Minister.EMPTY".to_string();
        }
        return self.name.clone();
    }

    /// java: public boolean isEmpty()
    pub fn is_empty(&self) -> bool {
        // java: this == EMPTY - the port's records are values (copied where Java shares the reference), so the EMPTY
        // instance is recognised by its (empty) value
        return self == &*EMPTY;
    }

}

impl std::fmt::Display for Minister {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Minister::to_string(self))
    }
}

//! Ported from de.hysky.skyblocker.utils.mayor.Mayor by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::mayor::perk;
use crate::de::hysky::skyblocker::utils::mayor::perk::Perk;
use mcrust::com::mojang::serialization::codec;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::com::mojang::serialization::codecs::record_codec_builder;

/// java: public static final Mayor EMPTY = new Mayor("", "", List.of());
pub static EMPTY: LazyLock<Mayor> = LazyLock::new(|| Mayor::new("".to_string(), "".to_string(), vec![]));
/// java: public static final Codec<Mayor> CODEC = RecordCodecBuilder.create(instance -> instance.group( Codec.STRING.fieldOf("key").forGetter(Mayor::key), Codec.STRING.fieldOf("name").forGetter(Mayor::name), P
pub static CODEC: compat::MainThread<CodecRef<Mayor>> = compat::MainThread::new(|| record_codec_builder::create(record_codec_builder::apply3(record_codec_builder::of(|o: &Mayor| o.key(), codec::field_of(mcrust::com::mojang::serialization::codec::string_codec(), "key")), record_codec_builder::of(|o: &Mayor| o.name(), codec::field_of(mcrust::com::mojang::serialization::codec::string_codec(), "name")), record_codec_builder::of(|o: &Mayor| o.perks(), codec::field_of(codec::list(perk::CODEC.get_mut().clone()), "perks")), move |__a0, __a1, __a2| Mayor::new(__a0.clone(), __a1.clone(), __a2.clone()))));

/// java: public record Mayor(String key, String name, List<Perk> perks)
#[derive(Clone, Debug, PartialEq)]
pub struct Mayor {
    /// java: String key
    pub key: String,
    /// java: String name
    pub name: String,
    /// java: List<Perk> perks
    pub perks: Vec<Perk>,
}

impl Mayor {
    /// java: the canonical record constructor
    pub fn new(mut key: String, mut name: String, mut perks: Vec<Perk>) -> Self {
        Self { key, name, perks }
    }

    /// java: the record accessor `key()`
    pub fn key(&self) -> String {
        self.key.clone()
    }

    /// java: the record accessor `name()`
    pub fn name(&self) -> String {
        self.name.clone()
    }

    /// java: the record accessor `perks()`
    pub fn perks(&self) -> Vec<Perk> {
        self.perks.clone()
    }

    /// java: @Override public String toString()
    pub fn to_string(&self) -> String {
        if self.is_empty() {
            return "Mayor.EMPTY".to_string();
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

impl std::fmt::Display for Mayor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Mayor::to_string(self))
    }
}

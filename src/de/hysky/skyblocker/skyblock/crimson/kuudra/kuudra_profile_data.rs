//! Ported from de.hysky.skyblocker.skyblock.crimson.kuudra.KuudraProfileData by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::crimson::crimson_faction;
use crate::de::hysky::skyblocker::skyblock::crimson::crimson_faction::CrimsonFaction;
use crate::de::hysky::skyblocker::skyblock::item::pet_info;
use crate::de::hysky::skyblocker::skyblock::item::pet_info::PetInfo;
use mcrust::com::mojang::serialization::codec;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::com::mojang::serialization::codecs::record_codec_builder;

/// java: public static final Codec<KuudraProfileData> CODEC = RecordCodecBuilder.create(instance -> instance.group( CrimsonFaction.CODEC.fieldOf("faction").forGetter(KuudraProfileData::faction), PetInfo.CODEC.
pub static CODEC: compat::MainThread<CodecRef<KuudraProfileData>> = compat::MainThread::new(|| record_codec_builder::create(record_codec_builder::apply3(record_codec_builder::of(|o: &KuudraProfileData| o.faction(), codec::field_of(crimson_faction::CODEC.get_mut().clone(), "faction")), record_codec_builder::of(|o: &KuudraProfileData| o.kuudra_pet(), codec::field_of(pet_info::CODEC.get_mut().clone(), "kuudraPet")), record_codec_builder::of(|o: &KuudraProfileData| o.kuudra_key_prices(), codec::field_of(codec::xmap(codec::unbounded_map(mcrust::com::mojang::serialization::codec::string_codec(), mcrust::com::mojang::serialization::codec::int_codec()), |entries: Vec<(String, i32)>| entries.into_iter().collect::<std::collections::HashMap<String, i32>>(), |map: &std::collections::HashMap<String, i32>| map.iter().map(|(k, v)| (k.clone(), *v)).collect::<Vec<(String, i32)>>()), "kuudraKeyPrices")), move |__a0, __a1, __a2| KuudraProfileData::new(__a0.clone(), __a1.clone(), __a2.clone()))));
/// java: public static final KuudraProfileData EMPTY = new KuudraProfileData(CrimsonFaction.MAGE, PetInfo.EMPTY, Map.of( "KUUDRA_TIER_KEY", 200_000, "KUUDRA_HOT_TIER_KEY", 400_000, "KUUDRA_BURNING_TIER_KEY", 7
pub static EMPTY: LazyLock<KuudraProfileData> = LazyLock::new(|| KuudraProfileData::new(CrimsonFaction::Mage, pet_info::EMPTY.clone(), std::collections::HashMap::from([("KUUDRA_TIER_KEY".to_string(), 200000), ("KUUDRA_HOT_TIER_KEY".to_string(), 400000), ("KUUDRA_BURNING_TIER_KEY".to_string(), 750000), ("KUUDRA_FIERY_TIER_KEY".to_string(), 1500000), ("KUUDRA_INFERNAL_TIER_KEY".to_string(), 3000000)])));

/// java: public record KuudraProfileData(CrimsonFaction faction, PetInfo kuudraPet, Map<String, Integer> kuudraKeyPrices)
#[derive(Clone, Debug, PartialEq)]
pub struct KuudraProfileData {
    /// java: CrimsonFaction faction
    pub faction: CrimsonFaction,
    /// java: PetInfo kuudraPet
    pub kuudra_pet: PetInfo,
    /// java: Map<String, Integer> kuudraKeyPrices
    pub kuudra_key_prices: std::collections::HashMap<String, i32>,
}

impl KuudraProfileData {
    /// java: the canonical record constructor
    pub fn new(mut faction: CrimsonFaction, mut kuudra_pet: PetInfo, mut kuudra_key_prices: std::collections::HashMap<String, i32>) -> Self {
        Self { faction, kuudra_pet, kuudra_key_prices }
    }

    /// java: the record accessor `faction()`
    pub fn faction(&self) -> CrimsonFaction {
        self.faction
    }

    /// java: the record accessor `kuudraPet()`
    pub fn kuudra_pet(&self) -> PetInfo {
        self.kuudra_pet.clone()
    }

    /// java: the record accessor `kuudraKeyPrices()`
    pub fn kuudra_key_prices(&self) -> std::collections::HashMap<String, i32> {
        self.kuudra_key_prices.clone()
    }

}

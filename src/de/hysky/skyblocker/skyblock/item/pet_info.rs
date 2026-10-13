//! Ported from de.hysky.skyblocker.skyblock.item.PetInfo by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::item::skyblock_item_rarity;
use crate::de::hysky::skyblocker::skyblock::item::skyblock_item_rarity::SkyblockItemRarity;
use mcrust::com::mojang::serialization::codec;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::com::mojang::serialization::codecs::record_codec_builder;

/// java: public static final Codec<PetInfo> CODEC = RecordCodecBuilder.create(instance -> instance.group( Codec.STRING.optionalFieldOf("name").forGetter(PetInfo::name), Codec.STRING.fieldOf("type").forGetter(P
pub static CODEC: compat::MainThread<CodecRef<PetInfo>> = compat::MainThread::new(|| record_codec_builder::create(record_codec_builder::apply7(record_codec_builder::of(|o: &PetInfo| o.name(), codec::optional_field_of_opt(mcrust::com::mojang::serialization::codec::string_codec(), "name")), record_codec_builder::of(|o: &PetInfo| o.r#type(), codec::field_of(mcrust::com::mojang::serialization::codec::string_codec(), "type")), record_codec_builder::of(|o: &PetInfo| o.exp(), codec::optional_field_of(mcrust::com::mojang::serialization::codec::double_codec(), "exp", 0.0)), record_codec_builder::of(|o: &PetInfo| o.tier(), codec::field_of(skyblock_item_rarity::CODEC.get_mut().clone(), "tier")), record_codec_builder::of(|o: &PetInfo| o.uuid(), codec::optional_field_of_opt(mcrust::com::mojang::serialization::codec::string_codec(), "uuid")), record_codec_builder::of(|o: &PetInfo| o.item(), codec::optional_field_of_opt(mcrust::com::mojang::serialization::codec::string_codec(), "heldItem")), record_codec_builder::of(|o: &PetInfo| o.skin(), codec::optional_field_of_opt(mcrust::com::mojang::serialization::codec::string_codec(), "skin")), move |__a0, __a1, __a2, __a3, __a4, __a5, __a6| PetInfo::new(__a0.clone(), __a1.clone(), __a2.clone(), __a3.clone(), __a4.clone(), __a5.clone(), __a6.clone()))));
/// java: public static final PetInfo EMPTY = new PetInfo(Optional.empty(), "", 0, SkyblockItemRarity.UNKNOWN, Optional.empty(), Optional.empty(), Optional.empty());
pub static EMPTY: LazyLock<PetInfo> = LazyLock::new(|| PetInfo::new(None, "".to_string(), ((0) as f64) as f64, SkyblockItemRarity::Unknown, None, None, None));

/// java: public record PetInfo(Optional<String> name, String type, double exp, SkyblockItemRarity tier, Optional<String> uuid, Optional<String> item, Optional<String> skin)
#[derive(Clone, Debug, PartialEq)]
pub struct PetInfo {
    /// java: Optional<String> name
    pub name: Option<String>,
    /// java: String type
    pub r#type: String,
    /// java: double exp
    pub exp: f64,
    /// java: SkyblockItemRarity tier
    pub tier: SkyblockItemRarity,
    /// java: Optional<String> uuid
    pub uuid: Option<String>,
    /// java: Optional<String> item
    pub item: Option<String>,
    /// java: Optional<String> skin
    pub skin: Option<String>,
}

impl PetInfo {
    /// java: the canonical record constructor
    pub fn new(mut name: Option<String>, mut r#type: String, mut exp: f64, mut tier: SkyblockItemRarity, mut uuid: Option<String>, mut item: Option<String>, mut skin: Option<String>) -> Self {
        Self { name, r#type, exp, tier, uuid, item, skin }
    }

    /// java: the record accessor `name()`
    pub fn name(&self) -> Option<String> {
        self.name.clone()
    }

    /// java: the record accessor `type()`
    pub fn r#type(&self) -> String {
        self.r#type.clone()
    }

    /// java: the record accessor `exp()`
    pub fn exp(&self) -> f64 {
        self.exp
    }

    /// java: the record accessor `tier()`
    pub fn tier(&self) -> SkyblockItemRarity {
        self.tier
    }

    /// java: the record accessor `uuid()`
    pub fn uuid(&self) -> Option<String> {
        self.uuid.clone()
    }

    /// java: the record accessor `item()`
    pub fn item(&self) -> Option<String> {
        self.item.clone()
    }

    /// java: the record accessor `skin()`
    pub fn skin(&self) -> Option<String> {
        self.skin.clone()
    }

    /// java: public SkyblockItemRarity rarity()
    pub fn rarity(&self) -> SkyblockItemRarity {
        return self.tier;
    }

    /// java: public int tierIndex()
    pub fn tier_index(&self) -> i32 {
        return self.rarity().ordinal();
    }

    /// java: public int level()
    pub fn level(&self) -> i32 {
        //   java: var convertedPetInfo = new net.azureaaron.networth.item.PetInfo(this.type(), this.exp(), this.tier().name(), 0, Optional.empty(), Optional.empty());
        let converted_pet_info = crate::net::azureaaron::networth::item::pet_info::PetInfo::new(self.r#type(), self.exp(), self.tier().name().to_string(), 0, None, None);
        //   java: return PetCalculator.calculatePetLevel(convertedPetInfo).leftInt();
        return crate::net::azureaaron::networth::pet_calculator::PetCalculator::calculate_pet_level(&converted_pet_info).0;
    }

    /// java: public boolean isEmpty()
    pub fn is_empty(&self) -> bool {
        // java: return this.equals(EMPTY); - the record's field-by-field equality
        return *self == *EMPTY;
    }

}

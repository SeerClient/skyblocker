//! Ported from de.hysky.skyblocker.utils.BazaarProduct by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::codec_utils::CodecUtils;
use mcrust::com::mojang::serialization::codec;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::com::mojang::serialization::codecs::record_codec_builder;

/// java: private static final Codec<BazaarProduct> CODEC = RecordCodecBuilder.create(instance -> instance.group( Codec.STRING.fieldOf("id").forGetter(BazaarProduct::id), Codec.STRING.fieldOf("name").forGetter(
pub static CODEC: compat::MainThread<CodecRef<BazaarProduct>> = compat::MainThread::new(|| record_codec_builder::create(record_codec_builder::apply6(record_codec_builder::of(|o: &BazaarProduct| o.id(), codec::field_of(mcrust::com::mojang::serialization::codec::string_codec(), "id")), record_codec_builder::of(|o: &BazaarProduct| o.name(), codec::field_of(mcrust::com::mojang::serialization::codec::string_codec(), "name")), record_codec_builder::of(|o: &BazaarProduct| o.buy_price(), CodecUtils::optional_double(&codec::lenient_optional_field_of_opt(mcrust::com::mojang::serialization::codec::double_codec(), "buyPrice"))), record_codec_builder::of(|o: &BazaarProduct| o.sell_price(), CodecUtils::optional_double(&codec::lenient_optional_field_of_opt(mcrust::com::mojang::serialization::codec::double_codec(), "sellPrice"))), record_codec_builder::of(|o: &BazaarProduct| o.buy_volume(), codec::field_of(mcrust::com::mojang::serialization::codec::int_codec(), "buyVolume")), record_codec_builder::of(|o: &BazaarProduct| o.sell_volume(), codec::field_of(mcrust::com::mojang::serialization::codec::int_codec(), "sellVolume")), move |__a0, __a1, __a2, __a3, __a4, __a5| BazaarProduct::new(__a0.clone(), __a1.clone(), __a2.clone(), __a3.clone(), __a4.clone(), __a5.clone()))));
/// java: public static final Codec<Object2ObjectMap<String, BazaarProduct>> MAP_CODEC = CodecUtils.object2ObjectMapCodec(Codec.STRING, CODEC);
pub static MAP_CODEC: compat::MainThread<CodecRef<std::collections::HashMap<String, BazaarProduct>>> = compat::MainThread::new(|| CodecUtils::object2_object_map_codec(&mcrust::com::mojang::serialization::codec::string_codec(), CODEC.get_mut()));

/// java: public record BazaarProduct(String id, String name, OptionalDouble buyPrice, OptionalDouble sellPrice, int buyVolume, int sellVolume)
#[derive(Clone, Debug, PartialEq)]
pub struct BazaarProduct {
    /// java: String id
    pub id: String,
    /// java: String name
    pub name: String,
    /// java: OptionalDouble buyPrice
    pub buy_price: Option<f64>,
    /// java: OptionalDouble sellPrice
    pub sell_price: Option<f64>,
    /// java: int buyVolume
    pub buy_volume: i32,
    /// java: int sellVolume
    pub sell_volume: i32,
}

impl BazaarProduct {
    /// java: the canonical record constructor
    pub fn new(mut id: String, mut name: String, mut buy_price: Option<f64>, mut sell_price: Option<f64>, mut buy_volume: i32, mut sell_volume: i32) -> Self {
        Self { id, name, buy_price, sell_price, buy_volume, sell_volume }
    }

    /// java: the record accessor `id()`
    pub fn id(&self) -> String {
        self.id.clone()
    }

    /// java: the record accessor `name()`
    pub fn name(&self) -> String {
        self.name.clone()
    }

    /// java: the record accessor `buyPrice()`
    pub fn buy_price(&self) -> Option<f64> {
        self.buy_price.clone()
    }

    /// java: the record accessor `sellPrice()`
    pub fn sell_price(&self) -> Option<f64> {
        self.sell_price.clone()
    }

    /// java: the record accessor `buyVolume()`
    pub fn buy_volume(&self) -> i32 {
        self.buy_volume
    }

    /// java: the record accessor `sellVolume()`
    pub fn sell_volume(&self) -> i32 {
        self.sell_volume
    }

}

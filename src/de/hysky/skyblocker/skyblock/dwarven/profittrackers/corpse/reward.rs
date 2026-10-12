//! Ported from de.hysky.skyblocker.skyblock.dwarven.profittrackers.corpse.Reward by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::com::mojang::serialization::codec;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::com::mojang::serialization::codecs::record_codec_builder;

/// java: public static final Codec<Reward> CODEC = RecordCodecBuilder.create(instance -> instance.group( Codec.INT.fieldOf("amount").forGetter(Reward::amount), Codec.STRING.fieldOf("itemId").forGetter(Reward::
pub static CODEC: compat::MainThread<CodecRef<compat::Shared<Reward>>> = compat::MainThread::new(|| record_codec_builder::create(record_codec_builder::apply3(record_codec_builder::of(|o: &compat::Shared<Reward>| Reward::amount(o), codec::field_of(mcrust::com::mojang::serialization::codec::int_codec(), "amount")), record_codec_builder::of(|o: &compat::Shared<Reward>| Reward::item_id(o), codec::field_of(mcrust::com::mojang::serialization::codec::string_codec(), "itemId")), record_codec_builder::of(|o: &compat::Shared<Reward>| Reward::price_per_unit(o), codec::field_of(mcrust::com::mojang::serialization::codec::double_codec(), "pricePerUnit")), move |__a0, __a1, __a2| Reward::new(__a0.clone(), __a1.clone(), __a2.clone()))));

/// java: public final class Reward
#[derive(Clone)]
pub struct Reward {
    /// java: private final String itemId;
    pub item_id: String,
    /// java: private int amount;
    pub amount: i32,
    /// java: private double pricePerUnit;
    pub price_per_unit: f64,
}

impl Reward {
    /// java: public Reward(int amount, String itemId, double pricePerUnit)
    pub fn new(mut amount: i32, mut item_id: String, mut price_per_unit: f64) -> compat::Shared<Self> {
        let __init_amount = amount;
        let __init_item_id = item_id.clone();
        let __init_price_per_unit = price_per_unit;
        let __this = compat::shared(Self {
            item_id: __init_item_id,
            amount: __init_amount,
            price_per_unit: __init_price_per_unit,
        });
        __this
    }

    /// java: public Reward(int amount, String itemId)
    pub fn new_2(mut amount: i32, mut item_id: String) -> compat::Shared<Self> {
        let __this = Self::new(amount, item_id.clone(), ((0) as f64) as f64);
        __this
    }

    /// java: public int amount()
    pub fn amount(__this: &compat::Shared<Self>) -> i32 {
        return compat::read(__this, |__t| __t.amount);
    }

    /// java: public void amount(int amount)
    pub fn amount_1(__this: &compat::Shared<Self>, mut amount: i32) {
        { let __v = amount; compat::write(__this, |__t| __t.amount = __v); }
    }

    /// java: public String itemId()
    pub fn item_id(__this: &compat::Shared<Self>) -> String {
        return compat::read(__this, |__t| __t.item_id.clone());
    }

    /// java: public double pricePerUnit()
    pub fn price_per_unit(__this: &compat::Shared<Self>) -> f64 {
        return compat::read(__this, |__t| __t.price_per_unit);
    }

    /// java: public void pricePerUnit(double pricePerUnit)
    pub fn price_per_unit_1(__this: &compat::Shared<Self>, mut price_per_unit: f64) {
        { let __v = price_per_unit; compat::write(__this, |__t| __t.price_per_unit = __v); }
    }

}

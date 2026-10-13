//! Ported from de.hysky.skyblocker.skyblock.dwarven.profittrackers.corpse.CorpseLoot by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::dwarven::corpse_type;
use crate::de::hysky::skyblocker::skyblock::dwarven::corpse_type::CorpseType;
use crate::de::hysky::skyblocker::skyblock::dwarven::profittrackers::corpse::corpse_profit_tracker;
use crate::de::hysky::skyblocker::skyblock::dwarven::profittrackers::corpse::corpse_profit_tracker::CorpseProfitTracker;
use crate::de::hysky::skyblocker::skyblock::dwarven::profittrackers::corpse::reward;
use crate::de::hysky::skyblocker::skyblock::dwarven::profittrackers::corpse::reward::Reward;
use crate::de::hysky::skyblocker::utils::item_utils::ItemUtils;
use mcrust::com::mojang::serialization::codec;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::com::mojang::serialization::codecs::record_codec_builder;

// TODO(fabric2seer): `.ofEpochMilli(..)` on a A
//   java: public static final Codec<CorpseLoot> CODEC = RecordCodecBuilder.create(instance -> instance.group(
//   java: 			CorpseType.CODEC.fieldOf("corpseType").forGetter(CorpseLoot::corpseType),
//   java: 			Reward.CODEC.listOf().fieldOf("rewards").forGetter(CorpseLoot::rewards),
//   java: 			Codec.LONG.xmap(Instant::ofEpochMilli, Instant::toEpochMilli).fieldOf("timestamp").forGetter(CorpseLoot::timestamp),
//   java: 			Codec.DOUBLE.fieldOf("profit").forGetter(CorpseLoot::profit)
//   java: 	).apply(instance, CorpseLoot::new));
/// java: public static final Codec<CorpseLoot> CODEC = RecordCodecBuilder.create(instance -> instance.group( CorpseType.CODEC.fieldOf("corpseType").forGetter(CorpseLoot::corpseType), Reward.CODEC.listOf().fiel
pub static CODEC: compat::MainThread<CodecRef<compat::Shared<CorpseLoot>>> = compat::MainThread::new(|| record_codec_builder::create(record_codec_builder::apply4(record_codec_builder::of(|o: &compat::Shared<CorpseLoot>| CorpseLoot::corpse_type(o), codec::field_of(corpse_type::CODEC.get_mut().clone(), "corpseType")), record_codec_builder::of(|o: &compat::Shared<CorpseLoot>| CorpseLoot::rewards(o), codec::field_of(codec::list(reward::CODEC.get_mut().clone()), "rewards")), record_codec_builder::of(|o: &compat::Shared<CorpseLoot>| CorpseLoot::timestamp(o), codec::field_of(codec::xmap(codec::long_codec(), |millis: i64| if millis >= 0 { std::time::UNIX_EPOCH + std::time::Duration::from_millis(millis as u64) } else { std::time::UNIX_EPOCH - std::time::Duration::from_millis(millis.unsigned_abs()) } /* Instant::ofEpochMilli */, |t: &std::time::SystemTime| match t.duration_since(std::time::UNIX_EPOCH) { Ok(d) => d.as_millis() as i64, Err(e) => -(e.duration().as_millis() as i64) } /* Instant::toEpochMilli */), "timestamp")), record_codec_builder::of(|o: &compat::Shared<CorpseLoot>| CorpseLoot::profit(o), codec::field_of(codec::double_codec(), "profit")), move |__a0, __a1, __a2, __a3| CorpseLoot::new(__a0.clone(), __a1.clone(), __a2.clone(), __a3.clone()))));

/// java: public final class CorpseLoot
#[derive(Clone)]
pub struct CorpseLoot {
    /// java: private final CorpseType corpseType;
    pub corpse_type: Option<CorpseType>,
    /// java: private final List<Reward> rewards;
    pub rewards: Vec<compat::Shared<Reward>>,
    /// java: private final Instant timestamp;
    pub timestamp: Option<std::time::SystemTime>,
    /// java: private double profit;
    pub profit: f64,
    /// java: private boolean isPriceDataComplete = true;
    pub is_price_data_complete: bool,
}

impl CorpseLoot {
    /// java: CorpseLoot(CorpseType corpseType, List<Reward> rewards, Instant timestamp, double profit)
    pub fn new(mut corpse_type: CorpseType, mut rewards: Vec<compat::Shared<Reward>>, mut timestamp: std::time::SystemTime, mut profit: f64) -> compat::Shared<Self> {
        let __init_corpse_type = Some(corpse_type);
        let __init_rewards = rewards.clone();
        let __init_timestamp = Some(timestamp.clone());
        let __init_profit = profit;
        let __this = compat::shared(Self {
            corpse_type: __init_corpse_type,
            rewards: __init_rewards,
            timestamp: __init_timestamp,
            profit: __init_profit,
            is_price_data_complete: true,
        });
        __this
    }

    /// java: CorpseLoot(CorpseType corpseType, List<Reward> rewards, Instant timestamp)
    pub fn new_3(mut corpse_type: CorpseType, mut rewards: Vec<compat::Shared<Reward>>, mut timestamp: std::time::SystemTime) -> compat::Shared<Self> {
        let __this = Self::new(corpse_type, rewards.clone(), timestamp.clone(), ((0) as f64) as f64);
        __this
    }

    /// java: private static String getItemId(String itemName)
    pub fn get_item_id(item_name: &str) -> String {
        return CorpseProfitTracker::get_name2_id_map().get(item_name).cloned().unwrap_or_else(|| "".to_string());
    }

    /// java: public CorpseType corpseType()
    pub fn corpse_type(__this: &compat::Shared<Self>) -> CorpseType {
        return compat::read(__this, |__t| __t.corpse_type.clone()).unwrap();
    }

    /// java: public List<Reward> rewards()
    pub fn rewards(__this: &compat::Shared<Self>) -> Vec<compat::Shared<Reward>> {
        return compat::read(__this, |__t| __t.rewards.clone());
    }

    /// java: public Instant timestamp()
    pub fn timestamp(__this: &compat::Shared<Self>) -> std::time::SystemTime {
        return compat::read(__this, |__t| __t.timestamp.clone()).unwrap();
    }

    /// java: public double profit()
    pub fn profit(__this: &compat::Shared<Self>) -> f64 {
        return compat::read(__this, |__t| __t.profit);
    }

    /// java: public void profit(double profit)
    pub fn profit_1(__this: &compat::Shared<Self>, mut profit: f64) {
        { let __v = profit; compat::write(__this, |__t| __t.profit = __v); }
    }

    /// java: public void addLoot(String itemName, int amount)
    pub fn add_loot(__this: &compat::Shared<Self>, item_name: &str, mut amount: i32) {
        let mut item_id: String = CorpseLoot::get_item_id(item_name);
        if item_id.is_empty() {
            logger().error(&format!("No matching item id for name `{}`. Report this!", item_name));
            return;
        }
        let mut reward: compat::Shared<Reward> = Reward::new_2(amount, item_id.clone());
        let __h0 = reward.clone();
        __this.lock().borrow_mut().rewards.push(__h0);
        if corpse_profit_tracker::PRICELESS_ITEMS.iter().any(|e| *e == item_id.clone()) {
            return;
        }
        let mut price: Option<f64> = ItemUtils::get_item_price_string(Some(item_id.clone()));
        if price.is_none() {
            logger().warn(&format!("No price found for item `{}`.", item_id));
            if compat::read(__this, |__t| __t.is_price_data_complete) {
                logger().warn("Profit calculation will not be accurate due to missing item price, therefore it will not be sent to chat. It will still be added to the corpse history.");
            }
            CorpseLoot::mark_price_data_incomplete(__this);
            return;
        }
        { let __v = (compat::read(__this, |__t| __t.profit) + (price.clone().unwrap() * (amount as f64))); compat::write(__this, |__t| __t.profit = __v); }
        Reward::price_per_unit_1(&reward, price.clone().unwrap());
    }

    /// java: public boolean isPriceDataComplete()
    pub fn is_price_data_complete(__this: &compat::Shared<Self>) -> bool {
        return compat::read(__this, |__t| __t.is_price_data_complete);
    }

    /// java: public void markPriceDataIncomplete()
    pub fn mark_price_data_incomplete(__this: &compat::Shared<Self>) {
        { let __v = false; compat::write(__this, |__t| __t.is_price_data_complete = __v); }
    }

    /// java: public void markPriceDataComplete()
    pub fn mark_price_data_complete(__this: &compat::Shared<Self>) {
        { let __v = true; compat::write(__this, |__t| __t.is_price_data_complete = __v); }
    }

}

//! Ported from de.hysky.skyblocker.skyblock.item.tooltip.info.TooltipInfoType by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::configs::general_config::ItemTooltip;
use crate::de::hysky::skyblocker::config::skyblocker_config::SkyblockerConfig;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::accessories::accessories_helper;
use crate::de::hysky::skyblocker::skyblock::accessories::accessories_helper::Accessory;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::info::data_tooltip_info::DataTooltipInfo;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::info::data_tooltip_info_type::DataTooltipInfoType;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::info::simple_tooltip_info::SimpleTooltipInfo;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::info::simple_tooltip_info::SimpleTooltipInfoBase;
use crate::de::hysky::skyblocker::utils::bazaar_product;
use crate::de::hysky::skyblocker::utils::bazaar_product::BazaarProduct;
use crate::de::hysky::skyblocker::utils::codec_utils::CodecUtils;
use mcrust::com::mojang::serialization::codec;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;

// TODO(fabric2seer): `.containsKey(..)` on a T
//   java: DataTooltipInfoType<Object2DoubleMap<String>> NPC = ofData("https://hysky.de/api/npcprice", CodecUtils.object2DoubleMapCodec(Codec.STRING), true, Object2DoubleMap::containsKey, itemTooltip -> itemTooltip.enableNPCPrice);
/// java: DataTooltipInfoType<Object2DoubleMap<String>> NPC = ofData("https://hysky.de/api/npcprice", CodecUtils.object2DoubleMapCodec(Codec.STRING), true, Object2DoubleMap::containsKey, itemTooltip -> itemTool
pub static NPC: compat::MainThread<compat::Shared<dyn DataTooltipInfoType<std::collections::HashMap<String, f64>>>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `NPC` - see the TODO above"));
// TODO(fabric2seer): `.containsKey(..)` on a T
//   java: DataTooltipInfoType<Object2ObjectMap<String, BazaarProduct>> BAZAAR = ofData("https://hysky.de/api/bazaar", BazaarProduct.MAP_CODEC, false, Object2ObjectMap::containsKey, itemTooltip -> itemTooltip.enableBazaarPrice, itemTooltip -> itemTooltip.enableBazaarPrice || itemTooltip.enableCraftingCost != Craft.OFF || itemTooltip.enableEstimatedItemValue || getConfig().dungeons.dungeonChestProfit.enableProfitCalculator || getConfig().dungeons.dungeonChestProfit.croesusProfit || getConfig().uiAndVisuals.chestValue.enableChestValue || SkyblockerConfigManager.get().helpers.enableBitsTooltip || itemTooltip.showEssenceCost, EssenceShopPrice::refreshEssencePrices);
/// java: DataTooltipInfoType<Object2ObjectMap<String, BazaarProduct>> BAZAAR = ofData("https://hysky.de/api/bazaar", BazaarProduct.MAP_CODEC, false, Object2ObjectMap::containsKey, itemTooltip -> itemTooltip.en
pub static BAZAAR: compat::MainThread<compat::Shared<dyn DataTooltipInfoType<std::collections::HashMap<String, BazaarProduct>>>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `BAZAAR` - see the TODO above"));
// TODO(fabric2seer): `.containsKey(..)` on a T
//   java: DataTooltipInfoType<Object2DoubleMap<String>> LOWEST_BINS = ofData("https://hysky.de/api/auctions/lowestbins", CodecUtils.object2DoubleMapCodec(Codec.STRING), false, Object2DoubleMap::containsKey, itemTooltip -> itemTooltip.enableLowestBIN, itemTooltip -> itemTooltip.enableLowestBIN || itemTooltip.enableCraftingCost != Craft.OFF || getConfig().dungeons.dungeonChestProfit.enableProfitCalculator || getConfig().dungeons.dungeonChestProfit.croesusProfit || getConfig().uiAndVisuals.chestValue.enableChestValue || SkyblockerConfigManager.get().helpers.enableBitsTooltip);
/// java: DataTooltipInfoType<Object2DoubleMap<String>> LOWEST_BINS = ofData("https://hysky.de/api/auctions/lowestbins", CodecUtils.object2DoubleMapCodec(Codec.STRING), false, Object2DoubleMap::containsKey, ite
pub static LOWEST_BINS: compat::MainThread<compat::Shared<dyn DataTooltipInfoType<std::collections::HashMap<String, f64>>>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `LOWEST_BINS` - see the TODO above"));
// TODO(fabric2seer): `.containsKey(..)` on a T
//   java: DataTooltipInfoType<Object2DoubleMap<String>> ONE_DAY_AVERAGE = ofData("https://hysky.de/api/auctions/lowestbins/average?days=1", CodecUtils.object2DoubleMapCodec(Codec.STRING), false, Object2DoubleMap::containsKey, itemTooltip -> itemTooltip.enableAvgBIN, itemTooltip -> itemTooltip.enableAvgBIN && itemTooltip.avg != GeneralConfig.Average.THREE_DAY);
/// java: DataTooltipInfoType<Object2DoubleMap<String>> ONE_DAY_AVERAGE = ofData("https://hysky.de/api/auctions/lowestbins/average?days=1", CodecUtils.object2DoubleMapCodec(Codec.STRING), false, Object2DoubleMa
pub static ONE_DAY_AVERAGE: compat::MainThread<compat::Shared<dyn DataTooltipInfoType<std::collections::HashMap<String, f64>>>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `ONE_DAY_AVERAGE` - see the TODO above"));
// TODO(fabric2seer): `.containsKey(..)` on a T
//   java: DataTooltipInfoType<Object2DoubleMap<String>> THREE_DAY_AVERAGE = ofData("https://hysky.de/api/auctions/lowestbins/average?days=3", CodecUtils.object2DoubleMapCodec(Codec.STRING), false, Object2DoubleMap::containsKey, itemTooltip -> itemTooltip.enableAvgBIN, itemTooltip -> (itemTooltip.enableAvgBIN && itemTooltip.avg != GeneralConfig.Average.ONE_DAY) || getConfig().uiAndVisuals.searchOverlay.enableAuctionHouse || itemTooltip.enableEstimatedItemValue);
/// java: DataTooltipInfoType<Object2DoubleMap<String>> THREE_DAY_AVERAGE = ofData("https://hysky.de/api/auctions/lowestbins/average?days=3", CodecUtils.object2DoubleMapCodec(Codec.STRING), false, Object2Double
pub static THREE_DAY_AVERAGE: compat::MainThread<compat::Shared<dyn DataTooltipInfoType<std::collections::HashMap<String, f64>>>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `THREE_DAY_AVERAGE` - see the TODO above"));
// TODO(fabric2seer): `.containsKey(..)` on a T
//   java: DataTooltipInfoType<Object2IntMap<String>> MOTES = ofData("https://hysky.de/api/motesprice", CodecUtils.object2IntMapCodec(Codec.STRING), true, Object2IntMap::containsKey, itemTooltip -> itemTooltip.enableMotesPrice, itemTooltip -> itemTooltip.enableMotesPrice && Utils.isInTheRift());
/// java: DataTooltipInfoType<Object2IntMap<String>> MOTES = ofData("https://hysky.de/api/motesprice", CodecUtils.object2IntMapCodec(Codec.STRING), true, Object2IntMap::containsKey, itemTooltip -> itemTooltip.e
pub static MOTES: compat::MainThread<compat::Shared<dyn DataTooltipInfoType<std::collections::HashMap<String, i32>>>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `MOTES` - see the TODO above"));
/// java: TooltipInfoType OBTAINED = ofSimple(itemTooltip -> itemTooltip.enableObtainedDate);
pub static OBTAINED: compat::MainThread<compat::Shared<dyn TooltipInfoType>> = compat::MainThread::new(|| of_simple(Arc::new(move |item_tooltip| compat::read(&item_tooltip, |__t| __t.enable_obtained_date))));
// TODO(fabric2seer): static call Map.containsKey(..)
//   java: DataTooltipInfoType<Map<String, String>> MUSEUM = ofData("https://hysky.de/api/museum", Codec.unboundedMap(Codec.STRING, Codec.STRING), true, Map::containsKey, itemTooltip -> itemTooltip.enableMuseumInfo);
/// java: DataTooltipInfoType<Map<String, String>> MUSEUM = ofData("https://hysky.de/api/museum", Codec.unboundedMap(Codec.STRING, Codec.STRING), true, Map::containsKey, itemTooltip -> itemTooltip.enableMuseumI
pub static MUSEUM: compat::MainThread<compat::Shared<dyn DataTooltipInfoType<std::collections::HashMap<String, String>>>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `MUSEUM` - see the TODO above"));
// TODO(fabric2seer): static call Map.containsKey(..)
//   java: DataTooltipInfoType<Map<String, String>> COLOR = ofData("https://hysky.de/api/color", Codec.unboundedMap(Codec.STRING, Codec.STRING), true, Map::containsKey, itemTooltip -> itemTooltip.enableExoticTooltip);
/// java: DataTooltipInfoType<Map<String, String>> COLOR = ofData("https://hysky.de/api/color", Codec.unboundedMap(Codec.STRING, Codec.STRING), true, Map::containsKey, itemTooltip -> itemTooltip.enableExoticToo
pub static COLOR: compat::MainThread<compat::Shared<dyn DataTooltipInfoType<std::collections::HashMap<String, String>>>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `COLOR` - see the TODO above"));
// TODO(fabric2seer): static call Map.containsKey(..)
//   java: DataTooltipInfoType<Map<String, Accessory>> ACCESSORIES = ofData("https://hysky.de/api/accessories", Accessory.MAP_CODEC, true, Map::containsKey, itemTooltip -> itemTooltip.enableAccessoriesHelper || getConfig().helpers.enableAccessoriesHelperWidget || ProfileViewer.ENABLED, AccessoriesHelper::refreshData);
/// java: DataTooltipInfoType<Map<String, Accessory>> ACCESSORIES = ofData("https://hysky.de/api/accessories", Accessory.MAP_CODEC, true, Map::containsKey, itemTooltip -> itemTooltip.enableAccessoriesHelper || 
pub static ACCESSORIES: compat::MainThread<compat::Shared<dyn DataTooltipInfoType<std::collections::HashMap<String, Accessory>>>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `ACCESSORIES` - see the TODO above"));
// TODO(fabric2seer): `.containsKey(..)` on a T
//   java: DataTooltipInfoType<Object2DoubleMap<String>> GEORGE = ofData("https://hysky.de/api/georgeprices", CodecUtils.object2DoubleMapCodec(Codec.STRING), true, Object2DoubleMap::containsKey, itemTooltip -> itemTooltip.enableGeorgePrice);
/// java: DataTooltipInfoType<Object2DoubleMap<String>> GEORGE = ofData("https://hysky.de/api/georgeprices", CodecUtils.object2DoubleMapCodec(Codec.STRING), true, Object2DoubleMap::containsKey, itemTooltip -> i
pub static GEORGE: compat::MainThread<compat::Shared<dyn DataTooltipInfoType<std::collections::HashMap<String, f64>>>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `GEORGE` - see the TODO above"));
/// java: TooltipInfoType ESTIMATED_ITEM_VALUE = ofSimple(itemTooltip -> itemTooltip.enableEstimatedItemValue);
pub static ESTIMATED_ITEM_VALUE: compat::MainThread<compat::Shared<dyn TooltipInfoType>> = compat::MainThread::new(|| of_simple(Arc::new(move |item_tooltip| compat::read(&item_tooltip, |__t| __t.enable_estimated_item_value))));

/// java: public interface TooltipInfoType
pub trait TooltipInfoType: compat::DynAny {
    fn as_any(&self) -> &dyn std::any::Any;
    /// java: boolean isTooltipEnabled(); (dispatch: the body for this class)
    fn is_tooltip_enabled(&self) -> fn(&compat::Shared<dyn TooltipInfoType>) -> bool;

}

/// java: private static SkyblockerConfig getConfig()
pub fn get_config() -> SkyblockerConfig {
    return SkyblockerConfigManager::get();
}

/// java: private static TooltipInfoType ofSimple(Predicate<GeneralConfig.ItemTooltip> tooltipEnabled)
pub fn of_simple(tooltip_enabled: Arc<dyn Fn(compat::Shared<ItemTooltip>) -> bool>) -> compat::Shared<dyn TooltipInfoType> {
    return ((SimpleTooltipInfoBase::new(tooltip_enabled.clone()) as compat::Shared<dyn SimpleTooltipInfo>) as compat::Shared<dyn TooltipInfoType>);
}

/// java: @SafeVarargs private static <T> DataTooltipInfoType<T> ofData(String address, Codec<T> codec, boolean cacheable, BiPredicate<T, String> contains, Predicate<GeneralConfig.ItemTooltip> tooltipEnabled, C
pub fn of_data<T: 'static>(address: &str, codec: &CodecRef<T>, mut cacheable: bool, contains: Arc<dyn Fn(T, String) -> bool>, tooltip_enabled: Arc<dyn Fn(compat::Shared<ItemTooltip>) -> bool>, callbacks: &Vec<Arc<dyn Fn(T)>>) -> compat::Shared<dyn DataTooltipInfoType<T>> {
    return of_data_7(address, codec, cacheable, contains.clone(), tooltip_enabled.clone(), tooltip_enabled.clone(), callbacks);
}

/// java: @SafeVarargs private static <T> DataTooltipInfoType<T> ofData(String address, Codec<T> codec, boolean cacheable, BiPredicate<T, String> contains, Predicate<GeneralConfig.ItemTooltip> tooltipEnabled, P
pub fn of_data_7<T: 'static>(address: &str, codec: &CodecRef<T>, mut cacheable: bool, contains: Arc<dyn Fn(T, String) -> bool>, tooltip_enabled: Arc<dyn Fn(compat::Shared<ItemTooltip>) -> bool>, data_enabled: Arc<dyn Fn(compat::Shared<ItemTooltip>) -> bool>, callbacks: &Vec<Arc<dyn Fn(T)>>) -> compat::Shared<dyn DataTooltipInfoType<T>> {
    return (DataTooltipInfo::new(address.to_string(), codec.clone(), cacheable, contains.clone(), tooltip_enabled.clone(), data_enabled.clone(), callbacks.clone()) as compat::Shared<dyn DataTooltipInfoType<compat::Object>>);
}

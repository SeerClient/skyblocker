//! Ported from de.hysky.skyblocker.skyblock.item.tooltip.info.TooltipInfoType by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::configs::general_config::ItemTooltip;
use crate::de::hysky::skyblocker::config::configs::general_config::Craft;
use crate::de::hysky::skyblocker::config::configs::general_config::Average;
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

/// java: DataTooltipInfoType<Object2DoubleMap<String>> NPC = ofData("https://hysky.de/api/npcprice", CodecUtils.object2DoubleMapCodec(Codec.STRING), true, Object2DoubleMap::containsKey, itemTooltip -> itemTool
pub static NPC: compat::MainThread<compat::Shared<dyn DataTooltipInfoType<std::collections::HashMap<String, f64>>>> = compat::MainThread::new(|| of_data("https://hysky.de/api/npcprice", &CodecUtils::object2_double_map_codec(&mcrust::com::mojang::serialization::codec::string_codec()), true, Arc::new(|map: &std::collections::HashMap<String, f64>, key: &str| map.contains_key(key)), Arc::new(|item_tooltip: compat::Shared<ItemTooltip>| compat::read(&item_tooltip, |item_tooltip| item_tooltip.enable_npc_price)), &vec![]));
/// java: DataTooltipInfoType<Object2ObjectMap<String, BazaarProduct>> BAZAAR = ofData("https://hysky.de/api/bazaar", BazaarProduct.MAP_CODEC, false, Object2ObjectMap::containsKey, itemTooltip -> itemTooltip.en
pub static BAZAAR: compat::MainThread<compat::Shared<dyn DataTooltipInfoType<std::collections::HashMap<String, BazaarProduct>>>> = compat::MainThread::new(|| of_data_7("https://hysky.de/api/bazaar", &bazaar_product::MAP_CODEC.get().clone(), false, Arc::new(|map: &std::collections::HashMap<String, BazaarProduct>, key: &str| map.contains_key(key)), Arc::new(|item_tooltip: compat::Shared<ItemTooltip>| compat::read(&item_tooltip, |item_tooltip| item_tooltip.enable_bazaar_price)), Arc::new(|item_tooltip: compat::Shared<ItemTooltip>| compat::read(&item_tooltip, |item_tooltip| item_tooltip.enable_bazaar_price || item_tooltip.enable_crafting_cost != Craft::Off || item_tooltip.enable_estimated_item_value || get_config().dungeons.dungeon_chest_profit.enable_profit_calculator || get_config().dungeons.dungeon_chest_profit.croesus_profit || get_config().ui_and_visuals.chest_value.enable_chest_value || SkyblockerConfigManager::get().helpers.enable_bits_tooltip || item_tooltip.show_essence_cost)), &vec![Arc::new(|data: &std::collections::HashMap<String, BazaarProduct>| crate::de::hysky::skyblocker::skyblock::item::tooltip::adders::essence_shop_price::EssenceShopPrice::refresh_essence_prices(data)) as Arc<dyn Fn(&std::collections::HashMap<String, BazaarProduct>)>]));
/// java: DataTooltipInfoType<Object2DoubleMap<String>> LOWEST_BINS = ofData("https://hysky.de/api/auctions/lowestbins", CodecUtils.object2DoubleMapCodec(Codec.STRING), false, Object2DoubleMap::containsKey, ite
pub static LOWEST_BINS: compat::MainThread<compat::Shared<dyn DataTooltipInfoType<std::collections::HashMap<String, f64>>>> = compat::MainThread::new(|| of_data_7("https://hysky.de/api/auctions/lowestbins", &CodecUtils::object2_double_map_codec(&mcrust::com::mojang::serialization::codec::string_codec()), false, Arc::new(|map: &std::collections::HashMap<String, f64>, key: &str| map.contains_key(key)), Arc::new(|item_tooltip: compat::Shared<ItemTooltip>| compat::read(&item_tooltip, |item_tooltip| item_tooltip.enable_lowest_bin)), Arc::new(|item_tooltip: compat::Shared<ItemTooltip>| compat::read(&item_tooltip, |item_tooltip| item_tooltip.enable_lowest_bin || item_tooltip.enable_crafting_cost != Craft::Off || get_config().dungeons.dungeon_chest_profit.enable_profit_calculator || get_config().dungeons.dungeon_chest_profit.croesus_profit || get_config().ui_and_visuals.chest_value.enable_chest_value || SkyblockerConfigManager::get().helpers.enable_bits_tooltip)), &vec![]));
/// java: DataTooltipInfoType<Object2DoubleMap<String>> ONE_DAY_AVERAGE = ofData("https://hysky.de/api/auctions/lowestbins/average?days=1", CodecUtils.object2DoubleMapCodec(Codec.STRING), false, Object2DoubleMa
pub static ONE_DAY_AVERAGE: compat::MainThread<compat::Shared<dyn DataTooltipInfoType<std::collections::HashMap<String, f64>>>> = compat::MainThread::new(|| of_data_7("https://hysky.de/api/auctions/lowestbins/average?days=1", &CodecUtils::object2_double_map_codec(&mcrust::com::mojang::serialization::codec::string_codec()), false, Arc::new(|map: &std::collections::HashMap<String, f64>, key: &str| map.contains_key(key)), Arc::new(|item_tooltip: compat::Shared<ItemTooltip>| compat::read(&item_tooltip, |item_tooltip| item_tooltip.enable_avg_bin)), Arc::new(|item_tooltip: compat::Shared<ItemTooltip>| compat::read(&item_tooltip, |item_tooltip| item_tooltip.enable_avg_bin && item_tooltip.avg != Average::ThreeDay)), &vec![]));
/// java: DataTooltipInfoType<Object2DoubleMap<String>> THREE_DAY_AVERAGE = ofData("https://hysky.de/api/auctions/lowestbins/average?days=3", CodecUtils.object2DoubleMapCodec(Codec.STRING), false, Object2Double
pub static THREE_DAY_AVERAGE: compat::MainThread<compat::Shared<dyn DataTooltipInfoType<std::collections::HashMap<String, f64>>>> = compat::MainThread::new(|| of_data_7("https://hysky.de/api/auctions/lowestbins/average?days=3", &CodecUtils::object2_double_map_codec(&mcrust::com::mojang::serialization::codec::string_codec()), false, Arc::new(|map: &std::collections::HashMap<String, f64>, key: &str| map.contains_key(key)), Arc::new(|item_tooltip: compat::Shared<ItemTooltip>| compat::read(&item_tooltip, |item_tooltip| item_tooltip.enable_avg_bin)), Arc::new(|item_tooltip: compat::Shared<ItemTooltip>| compat::read(&item_tooltip, |item_tooltip| (item_tooltip.enable_avg_bin && item_tooltip.avg != Average::OneDay) || get_config().ui_and_visuals.search_overlay.enable_auction_house || item_tooltip.enable_estimated_item_value)), &vec![]));
/// java: DataTooltipInfoType<Object2IntMap<String>> MOTES = ofData("https://hysky.de/api/motesprice", CodecUtils.object2IntMapCodec(Codec.STRING), true, Object2IntMap::containsKey, itemTooltip -> itemTooltip.e
pub static MOTES: compat::MainThread<compat::Shared<dyn DataTooltipInfoType<std::collections::HashMap<String, i32>>>> = compat::MainThread::new(|| of_data_7("https://hysky.de/api/motesprice", &CodecUtils::object2_int_map_codec(&mcrust::com::mojang::serialization::codec::string_codec()), true, Arc::new(|map: &std::collections::HashMap<String, i32>, key: &str| map.contains_key(key)), Arc::new(|item_tooltip: compat::Shared<ItemTooltip>| compat::read(&item_tooltip, |item_tooltip| item_tooltip.enable_motes_price)), Arc::new(|item_tooltip: compat::Shared<ItemTooltip>| compat::read(&item_tooltip, |item_tooltip| item_tooltip.enable_motes_price && crate::de::hysky::skyblocker::utils::utils::Utils::is_in_the_rift())), &vec![]));
/// java: TooltipInfoType OBTAINED = ofSimple(itemTooltip -> itemTooltip.enableObtainedDate);
pub static OBTAINED: compat::MainThread<compat::Shared<dyn TooltipInfoType>> = compat::MainThread::new(|| of_simple(Arc::new(move |item_tooltip| compat::read(&item_tooltip, |__t| __t.enable_obtained_date))));
/// java: DataTooltipInfoType<Map<String, String>> MUSEUM = ofData("https://hysky.de/api/museum", Codec.unboundedMap(Codec.STRING, Codec.STRING), true, Map::containsKey, itemTooltip -> itemTooltip.enableMuseumI
pub static MUSEUM: compat::MainThread<compat::Shared<dyn DataTooltipInfoType<std::collections::HashMap<String, String>>>> = compat::MainThread::new(|| of_data("https://hysky.de/api/museum", &string_map_codec(), true, Arc::new(|map: &std::collections::HashMap<String, String>, key: &str| map.contains_key(key)), Arc::new(|item_tooltip: compat::Shared<ItemTooltip>| compat::read(&item_tooltip, |item_tooltip| item_tooltip.enable_museum_info)), &vec![]));
/// java: DataTooltipInfoType<Map<String, String>> COLOR = ofData("https://hysky.de/api/color", Codec.unboundedMap(Codec.STRING, Codec.STRING), true, Map::containsKey, itemTooltip -> itemTooltip.enableExoticToo
pub static COLOR: compat::MainThread<compat::Shared<dyn DataTooltipInfoType<std::collections::HashMap<String, String>>>> = compat::MainThread::new(|| of_data("https://hysky.de/api/color", &string_map_codec(), true, Arc::new(|map: &std::collections::HashMap<String, String>, key: &str| map.contains_key(key)), Arc::new(|item_tooltip: compat::Shared<ItemTooltip>| compat::read(&item_tooltip, |item_tooltip| item_tooltip.enable_exotic_tooltip)), &vec![]));
/// java: DataTooltipInfoType<Map<String, Accessory>> ACCESSORIES = ofData("https://hysky.de/api/accessories", Accessory.MAP_CODEC, true, Map::containsKey, itemTooltip -> itemTooltip.enableAccessoriesHelper || 
pub static ACCESSORIES: compat::MainThread<compat::Shared<dyn DataTooltipInfoType<std::collections::HashMap<String, Accessory>>>> = compat::MainThread::new(|| of_data("https://hysky.de/api/accessories", &accessories_helper::ACCESSORY_MAP_CODEC.get().clone(), true, Arc::new(|map: &std::collections::HashMap<String, Accessory>, key: &str| map.contains_key(key)), Arc::new(|item_tooltip: compat::Shared<ItemTooltip>| compat::read(&item_tooltip, |item_tooltip| item_tooltip.enable_accessories_helper || get_config().helpers.enable_accessories_helper_widget || *crate::de::hysky::skyblocker::skyblock::profileviewer2::profile_viewer::ENABLED)), &vec![Arc::new(|data: &std::collections::HashMap<String, Accessory>| crate::de::hysky::skyblocker::skyblock::accessories::accessories_helper::AccessoriesHelper::refresh_data(data.clone())) as Arc<dyn Fn(&std::collections::HashMap<String, Accessory>)>]));
/// java: DataTooltipInfoType<Object2DoubleMap<String>> GEORGE = ofData("https://hysky.de/api/georgeprices", CodecUtils.object2DoubleMapCodec(Codec.STRING), true, Object2DoubleMap::containsKey, itemTooltip -> i
pub static GEORGE: compat::MainThread<compat::Shared<dyn DataTooltipInfoType<std::collections::HashMap<String, f64>>>> = compat::MainThread::new(|| of_data("https://hysky.de/api/georgeprices", &CodecUtils::object2_double_map_codec(&mcrust::com::mojang::serialization::codec::string_codec()), true, Arc::new(|map: &std::collections::HashMap<String, f64>, key: &str| map.contains_key(key)), Arc::new(|item_tooltip: compat::Shared<ItemTooltip>| compat::read(&item_tooltip, |item_tooltip| item_tooltip.enable_george_price)), &vec![]));
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
pub fn of_data<T: Clone + Send + 'static>(address: &str, codec: &CodecRef<T>, mut cacheable: bool, contains: Arc<dyn Fn(&T, &str) -> bool>, tooltip_enabled: Arc<dyn Fn(compat::Shared<ItemTooltip>) -> bool>, callbacks: &Vec<Arc<dyn Fn(&T)>>) -> compat::Shared<dyn DataTooltipInfoType<T>> {
    return of_data_7(address, codec, cacheable, contains.clone(), tooltip_enabled.clone(), tooltip_enabled.clone(), callbacks);
}

/// java: @SafeVarargs private static <T> DataTooltipInfoType<T> ofData(String address, Codec<T> codec, boolean cacheable, BiPredicate<T, String> contains, Predicate<GeneralConfig.ItemTooltip> tooltipEnabled, P
pub fn of_data_7<T: Clone + Send + 'static>(address: &str, codec: &CodecRef<T>, mut cacheable: bool, contains: Arc<dyn Fn(&T, &str) -> bool>, tooltip_enabled: Arc<dyn Fn(compat::Shared<ItemTooltip>) -> bool>, data_enabled: Arc<dyn Fn(compat::Shared<ItemTooltip>) -> bool>, callbacks: &Vec<Arc<dyn Fn(&T)>>) -> compat::Shared<dyn DataTooltipInfoType<T>> {
    return (DataTooltipInfo::new(address.to_string(), codec.clone(), cacheable, contains.clone(), tooltip_enabled.clone(), data_enabled.clone(), callbacks.clone()) as compat::Shared<dyn DataTooltipInfoType<T>>);
}

/// `Codec.unboundedMap(Codec.STRING, Codec.STRING)` decoding to the `Map<String, String>` the type carries (the
/// port's `unbounded_map` answers the entries as ordered pairs).
fn string_map_codec() -> CodecRef<std::collections::HashMap<String, String>> {
    codec::xmap(
        codec::unbounded_map(mcrust::com::mojang::serialization::codec::string_codec(), mcrust::com::mojang::serialization::codec::string_codec()),
        |entries: Vec<(String, String)>| entries.into_iter().collect::<std::collections::HashMap<String, String>>(),
        |map: &std::collections::HashMap<String, String>| map.iter().map(|(k, v)| (k.clone(), v.clone())).collect::<Vec<_>>(),
    )
}

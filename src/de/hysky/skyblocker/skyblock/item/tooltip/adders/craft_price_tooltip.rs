//! Ported from de.hysky.skyblocker.skyblock.item.tooltip.adders.CraftPriceTooltip by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::configs::general_config::Craft;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::info::tooltip_info_type;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::item_tooltip::ItemTooltip;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::simple_tooltip_adder::SimpleTooltipAdder;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::simple_tooltip_adder::SimpleTooltipAdderBase;
use crate::de::hysky::skyblocker::utils::container::container_matcher::ContainerMatcher;
use crate::de::hysky::skyblocker::utils::container::regex_container_matcher::RegexContainerMatcher;
use crate::de::hysky::skyblocker::utils::container::regex_container_matcher::RegexContainerMatcherBase;
use crate::de::hysky::skyblocker::utils::container::tooltip_adder::TooltipAdder;
use crate::de::hysky::skyblocker::utils::item_utils::ItemUtils;
use crate::de::hysky::skyblocker::utils::neu_repo_manager::NEURepoManager;
use crate::io::github::moulberry::repo::data::neu_ingredient;
use crate::io::github::moulberry::repo::data::neu_kat_upgrade_recipe::NEUKatUpgradeRecipe;
use crate::io::github::moulberry::repo::data::neu_mob_drop_recipe::NEUMobDropRecipe;
use crate::io::github::moulberry::repo::data::neu_recipe::NEURecipe;
use crate::io::github::moulberry::repo::data::neu_trade_recipe::NEUTradeRecipe;
use crate::de::hysky::skyblocker::utils::render::text::grid_component::GridComponent;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::world::inventory::slot::Slot;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::info::data_tooltip_info_type::DataTooltipInfoType as _;

/// java: private static final Map<String, Double> cachedCraftCosts = Object2DoubleMaps.synchronize(new Object2DoubleOpenHashMap<>());
/// (the synchronized map is the `Mutex`; its monitor is re-entrant and this lock is not, so no caller holds it
/// across a call that reads the map again)
pub static CACHED_CRAFT_COSTS: LazyLock<Mutex<std::collections::HashMap<String, f64>>> = LazyLock::new(|| Mutex::new(std::collections::HashMap::new()));
/// java: private static final int MAX_RECURSION_DEPTH = 15;
pub const MAX_RECURSION_DEPTH: i32 = 15;

/// java: public class CraftPriceTooltip extends SimpleTooltipAdder
#[derive(Clone)]
pub struct CraftPriceTooltip {
    /// java: extends SimpleTooltipAdder
    pub base: SimpleTooltipAdderBase,
}

impl std::ops::Deref for CraftPriceTooltip {
    type Target = SimpleTooltipAdderBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for CraftPriceTooltip {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl CraftPriceTooltip {
    /// java: public CraftPriceTooltip(int priority)
    pub fn new(mut priority: i32) -> compat::Shared<Self> {
        let (_, __this) = SimpleTooltipAdderBase::new_1_in(priority, move |__base| {
            let __h = compat::shared(Self {
                base: __base,
            });
            ((__h.clone() as compat::Shared<dyn SimpleTooltipAdder>), __h)
        });
        __this
    }

    /// java: public static double getItemCost(String neuId)
    pub fn get_item_cost(neu_id: &str) -> f64 {
        return CraftPriceTooltip::get_item_cost_2(neu_id, 0);
    }

    /// java: public static double getItemCost(String neuId, int depth)
    pub fn get_item_cost_2(neu_id: &str, mut depth: i32) -> f64 {
        let __recipes = NEURepoManager::get_recipes();
        let mut neu_recipes: Option<&Vec<Arc<dyn NEURecipe>>> = __recipes.get(neu_id);
        if neu_recipes.is_none() || neu_recipes.as_ref().unwrap().is_empty() {
            return ((-1) as f64) as f64;
        }
        let mut recipe: Option<&Arc<dyn NEURecipe>> = neu_recipes.as_ref().unwrap().iter().filter(|__a0| CraftPriceTooltip::is_valid_recipe(&***__a0)).next();
        let Some(recipe) = recipe else {
            return ((-1) as f64) as f64;
        };
        //   java: return recipe.get().getAllOutputs().stream().filter(x -> x.getItemId().equals(neuId)).map(NEUIngredient::getAmount).findAny()
        //   java: 				.map(amount -> getRecipeCost(recipe.get(), depth + 1) / amount).orElse(-1.0);
        return recipe
            .get_all_outputs()
            .iter()
            .filter(|x| x.get_item_id() == neu_id)
            .map(|x| x.get_amount())
            .next()
            .map(|amount| CraftPriceTooltip::get_recipe_cost(&**recipe, depth + 1) / amount)
            .unwrap_or(-1.0);
    }

    /// java: public static double getRecipeCost(NEURecipe recipe, int depth)
    pub fn get_recipe_cost(recipe: &dyn NEURecipe, mut depth: i32) -> f64 {
        if depth >= MAX_RECURSION_DEPTH || !(CraftPriceTooltip::is_valid_recipe(recipe)) {
            return ((-1) as f64) as f64;
        }
        let mut total_craft_cost: f64 = ((0) as f64) as f64;
        //   java: Object2ObjectMap<String, BazaarProduct> bazaarData = TooltipInfoType.BAZAAR.getData();
        //   java: Object2DoubleMap<String> lowestBinsData = TooltipInfoType.LOWEST_BINS.getData();
        // (read once for the loop - the port's `getData` answers a copy of the map, not the map itself)
        let bazaar_data: Option<std::collections::HashMap<String, crate::de::hysky::skyblocker::utils::bazaar_product::BazaarProduct>> =
            tooltip_info_type::BAZAAR.get_mut().lock().borrow().get_data();
        let lowest_bins_data: Option<std::collections::HashMap<String, f64>> = tooltip_info_type::LOWEST_BINS.get_mut().lock().borrow().get_data();
        //   java: for (NEUIngredient input : recipe.getAllInputs()) {
        for input in recipe.get_all_inputs() {
            let input_item_name: String = input.get_item_id().to_string();
            let input_item_count: f64 = input.get_amount();
            //   java: if (cachedCraftCosts.containsKey(inputItemName)) { totalCraftCost += cachedCraftCosts.get(inputItemName) * inputItemCount; continue; }
            let __cached: Option<f64> = compat::lock(&CACHED_CRAFT_COSTS).get(&input_item_name).copied();
            if let Some(cached) = __cached {
                total_craft_cost += cached * input_item_count;
                continue;
            }
            let mut item_cost: f64 = 0.0;
            if let Some(product) = bazaar_data.as_ref().and_then(|data| data.get(&input_item_name)) {
                //   java: itemCost = SkyblockerConfigManager.get().general.itemTooltip.enableCraftingCost == Craft.BUY_ORDER ? product.buyPrice().orElse(0d) : product.sellPrice().orElse(0d);
                item_cost = if compat::read(&SkyblockerConfigManager::get().general.item_tooltip, |__t| __t.enable_crafting_cost) == Craft::BuyOrder {
                    product.buy_price().unwrap_or(0.0)
                } else {
                    product.sell_price().unwrap_or(0.0)
                };
            } else if let Some(lowest_bin) = lowest_bins_data.as_ref().and_then(|data| data.get(&input_item_name)) {
                //   java: itemCost = lowestBinsData.getDouble(inputItemName);
                item_cost = *lowest_bin;
            }
            if item_cost > 0.0 {
                compat::lock(&CACHED_CRAFT_COSTS).insert(input_item_name.clone(), item_cost);
            }
            //   java: double craftCost = getItemCost(inputItemName, depth + 1);
            let craft_cost: f64 = CraftPriceTooltip::get_item_cost_2(&input_item_name, depth + 1);
            if craft_cost > 0.0 {
                if item_cost > 0.0 {
                    item_cost = item_cost.min(craft_cost);
                } else {
                    item_cost = craft_cost;
                }
                compat::lock(&CACHED_CRAFT_COSTS).insert(input_item_name.clone(), item_cost);
            }
            total_craft_cost += item_cost * input_item_count;
        }
        return total_craft_cost;
    }

    /// java: public static boolean isValidRecipe(NEURecipe recipe)
    pub fn is_valid_recipe(recipe: &dyn NEURecipe) -> bool {
        // `instanceof` the library's recipe subclasses
        return !(recipe.is::<NEUKatUpgradeRecipe>() || recipe.is::<NEUTradeRecipe>() || recipe.is::<NEUMobDropRecipe>());
    }

    /// java: public static void clearCache()
    pub fn clear_cache() {
        compat::lock(&CACHED_CRAFT_COSTS).clear();
        //   java: cachedCraftCosts.put(NEUIngredient.NEU_SENTINEL_COINS, 1d);
        compat::lock(&CACHED_CRAFT_COSTS).insert(neu_ingredient::NEU_SENTINEL_COINS.to_string(), 1.0);
        //   java: cachedCraftCosts.put(NEUIngredient.NEU_SENTINEL_EMPTY, 0d);
        compat::lock(&CACHED_CRAFT_COSTS).insert(neu_ingredient::NEU_SENTINEL_EMPTY.to_string(), 0.0);
    }

}

impl SimpleTooltipAdder for CraftPriceTooltip {
    fn simple_tooltip_adder_base(&self) -> &SimpleTooltipAdderBase {
        &self.base
    }
    fn simple_tooltip_adder_base_mut(&mut self) -> &mut SimpleTooltipAdderBase {
        &mut self.base
    }

}

impl RegexContainerMatcher for CraftPriceTooltip {
    fn regex_container_matcher_base(&self) -> &RegexContainerMatcherBase {
        &self.base.base
    }
    fn regex_container_matcher_base_mut(&mut self) -> &mut RegexContainerMatcherBase {
        &mut self.base.base
    }

}

impl ContainerMatcher for CraftPriceTooltip {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public boolean isEnabled() (dispatch: the body for this class)
    fn is_enabled(&self) -> fn(&compat::Shared<dyn ContainerMatcher>) -> bool {
        CraftPriceTooltip::is_enabled_dyn
    }

    /// java: @Override public boolean test(Screen screen) (dispatch: the body for this class)
    fn test(&self) -> fn(&compat::Shared<dyn ContainerMatcher>, &mcrust::net::minecraft::client::gui::screens::screen::Screen) -> bool {
        <dyn RegexContainerMatcher>::test_from::<Self>
    }

}

impl TooltipAdder for CraftPriceTooltip {
    /// java: @Override public void addToTooltip(@Nullable Slot focusedSlot, ItemStack stack, List<Component> lines) (dispatch: the body for this class)
    fn add_to_tooltip(&self) -> fn(&compat::Shared<dyn TooltipAdder>, Option<Slot>, &mcrust::net::minecraft::world::item::item_stack::ItemStack, &mut Vec<MutableComponent>) {
        CraftPriceTooltip::add_to_tooltip_dyn
    }

    /// java: @Override public int getPriority() (dispatch: the body for this class)
    fn get_priority(&self) -> fn(&compat::Shared<dyn TooltipAdder>) -> i32 {
        <dyn SimpleTooltipAdder>::get_priority_from::<Self>
    }

}

impl CraftPriceTooltip {
    /// java: @Override public boolean isEnabled()
    pub fn is_enabled_impl(__this: &compat::Shared<Self>) -> bool {
        return (compat::read(&SkyblockerConfigManager::get().general.item_tooltip, |__t| __t.enable_crafting_cost) != Craft::Off);
    }

    /// CraftPriceTooltip.isEnabled as the dispatch of ContainerMatcher.isEnabled
    pub fn is_enabled_dyn(__this: &compat::Shared<dyn ContainerMatcher>) -> bool {
        Self::is_enabled_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public void addToTooltip(@Nullable Slot focusedSlot, ItemStack stack, List<Component> lines)
    pub fn add_to_tooltip_impl(__this: &compat::Shared<Self>, mut focused_slot: Option<Slot>, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, lines: &mut Vec<MutableComponent>) {
        if tooltip_info_type::LOWEST_BINS.get_mut().lock().borrow().get_data().is_none() || tooltip_info_type::BAZAAR.get_mut().lock().borrow().get_data().is_none() {
            ItemTooltip::null_warning();
            return;
        }
        let mut item_id: String = stack.get_neu_name();
        // java: try {..} catch (Exception e) { LOGGER.error("[Skyblocker Craft Price] Error calculating craft price for: {}", stack.getNeuName(), e); }
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            // cachedCraftCosts.computeIfAbsent(itemId, CraftPriceTooltip::getItemCost) - getItemCost reads the map
            // again, so the cost is computed outside the (non-re-entrant) lock
            let __cached: Option<f64> = compat::lock(&CACHED_CRAFT_COSTS).get(&item_id).copied();
            let mut craft_price: f64 = match __cached {
                Some(cost) => cost,
                None => {
                    let cost = CraftPriceTooltip::get_item_cost(&item_id);
                    *compat::lock(&CACHED_CRAFT_COSTS).entry(item_id.clone()).or_insert(cost)
                }
            };
            if craft_price <= (0 as f64) {
                return;
            }
            let mut count: i32 = (ItemUtils::get_item_count_in_sack(stack, &stack.skyblocker_get_lore_strings()).unwrap_or(ItemUtils::get_item_count_in_stash_component(&lines[0]).unwrap_or(stack.get_count()))).max(1);
            lines.push(GridComponent::of(&vec![component::literal("Crafting Price:").with_style_format(ChatFormatting::Gold), ItemTooltip::get_coins_message(craft_price, count)]));
        }));
        if let Err(e) = __result {
            logger().error(&format!("[Skyblocker Craft Price] Error calculating craft price for: {} {}", stack.get_neu_name(), compat::panic_message(&e)));
        }
    }

    /// CraftPriceTooltip.addToTooltip as the dispatch of TooltipAdder.addToTooltip
    pub fn add_to_tooltip_dyn(__this: &compat::Shared<dyn TooltipAdder>, __p0: Option<Slot>, __p1: &mcrust::net::minecraft::world::item::item_stack::ItemStack, __p2: &mut Vec<MutableComponent>) {
        Self::add_to_tooltip_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2)
    }

}

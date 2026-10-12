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
        let mut neu_recipes: Option<&std::collections::HashSet<crate::unresolved::NEURecipe>> = __recipes.get(neu_id);
        if neu_recipes.is_none() || neu_recipes.as_ref().unwrap().is_empty() {
            return ((-1) as f64) as f64;
        }
        let mut recipe: Option<&crate::unresolved::NEURecipe> = neu_recipes.as_ref().unwrap().iter().filter(|__a0| CraftPriceTooltip::is_valid_recipe(__a0)).next();
        if recipe.is_none() {
            return ((-1) as f64) as f64;
        }
        // TODO(fabric2seer): `.getAllOutputs(..)` on a NEURecipe
        //   java: return recipe.get().getAllOutputs().stream().filter(x -> x.getItemId().equals(neuId)).map(NEUIngredient::getAmount).findAny()
        //   java: 				.map(amount -> getRecipeCost(recipe.get(), depth + 1) / amount).orElse(-1.0);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public static double getRecipeCost(NEURecipe recipe, int depth)
    pub fn get_recipe_cost(recipe: &crate::unresolved::NEURecipe, mut depth: i32) -> f64 {
        if depth >= MAX_RECURSION_DEPTH || !(CraftPriceTooltip::is_valid_recipe(recipe)) {
            return ((-1) as f64) as f64;
        }
        let mut total_craft_cost: f64 = ((0) as f64) as f64;
        // TODO(fabric2seer): `.getAllInputs(..)` on a NEURecipe
        //   java: for (NEUIngredient input : recipe.getAllInputs()) {
        //   java: 			String inputItemName = input.getItemId();
        //   java: 			double inputItemCount = input.getAmount();
        //   java: 			if (cachedCraftCosts.containsKey(inputItemName)) {
        //   java: 				totalCraftCost += cachedCraftCosts.get(inputItemName) * inputItemCount;
        //   java: 				continue;
        //   java: 			}
        //   java:
        //   java: 			double itemCost = 0;
        //   java:
        //   java: 			Object2ObjectMap<String, BazaarProduct> bazaarData = TooltipInfoType.BAZAAR.getData();
        //   java: 			Object2DoubleMap<String> lowestBinsData = TooltipInfoType.LOWEST_BINS.getData();
        //   java: 			if (bazaarData != null && bazaarData.containsKey(inputItemName)) {
        //   java: 				BazaarProduct product = bazaarData.get(inputItemName);
        //   java: 				itemCost = SkyblockerConfigManager.get().general.itemTooltip.enableCraftingCost == Craft.BUY_ORDER ? product.buyPrice().orElse(0d) : product.sellPrice().orElse(0d);
        //   java: 			} else if (lowestBinsData != null && lowestBinsData.containsKey(inputItemName)) {
        //   java: 				itemCost = lowestBinsData.getDouble(inputItemName);
        //   java: 			}
        //   java:
        //   java: 			if (itemCost > 0) {
        //   java: 				cachedCraftCosts.put(inputItemName, itemCost);
        //   java: 			}
        //   java:
        //   java: 			double craftCost = getItemCost(inputItemName, depth + 1);
        //   java: 			if (craftCost > 0) {
        //   java: 				if (itemCost > 0) itemCost = Math.min(itemCost, craftCost);
        //   java: 				else itemCost = craftCost;
        //   java: 				cachedCraftCosts.put(inputItemName, itemCost);
        //   java: 			}
        //   java:
        //   java: 			totalCraftCost += itemCost * inputItemCount;
        //   java: 		}
        return total_craft_cost;
    }

    /// java: public static boolean isValidRecipe(NEURecipe recipe)
    pub fn is_valid_recipe(recipe: &crate::unresolved::NEURecipe) -> bool {
        // `instanceof` the library's recipe subclasses (io.github.moulberry.repo is not ported - GAPS_batchB.md)
        let recipe: &dyn std::any::Any = recipe;
        return !(recipe.is::<crate::unresolved::NEUKatUpgradeRecipe>() || recipe.is::<crate::unresolved::NEUTradeRecipe>() || recipe.is::<crate::unresolved::NEUMobDropRecipe>());
    }

    /// java: public static void clearCache()
    pub fn clear_cache() {
        compat::lock(&CACHED_CRAFT_COSTS).clear();
        // TODO(fabric2seer): static field NEUIngredient.NEU_SENTINEL_COINS
        //   java: cachedCraftCosts.put(NEUIngredient.NEU_SENTINEL_COINS, 1d);
        // TODO(fabric2seer): static field NEUIngredient.NEU_SENTINEL_EMPTY
        //   java: cachedCraftCosts.put(NEUIngredient.NEU_SENTINEL_EMPTY, 0d);
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
        // java: try {..} catch (Exception e) - the body runs inline: a Rust body does not throw checked exceptions (a panic in it propagates)
        {
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
        }
        // TODO(fabric2seer): the `catch (Exception e)` block is not reached in Rust - the try body returns, breaks or continues, so it cannot run inside `catch_unwind`
        //   java: {
        //   java: 			LOGGER.error("[Skyblocker Craft Price] Error calculating craft price for: {}", stack.getNeuName(), e);
        //   java: 		}
    }

    /// CraftPriceTooltip.addToTooltip as the dispatch of TooltipAdder.addToTooltip
    pub fn add_to_tooltip_dyn(__this: &compat::Shared<dyn TooltipAdder>, __p0: Option<Slot>, __p1: &mcrust::net::minecraft::world::item::item_stack::ItemStack, __p2: &mut Vec<MutableComponent>) {
        Self::add_to_tooltip_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2)
    }

}

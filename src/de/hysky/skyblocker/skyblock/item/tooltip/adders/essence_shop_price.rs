//! Ported from de.hysky.skyblocker.skyblock.item.tooltip.adders.EssenceShopPrice by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::simple_tooltip_adder::SimpleTooltipAdder;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::simple_tooltip_adder::SimpleTooltipAdderBase;
use crate::de::hysky::skyblocker::utils::bazaar_product::BazaarProduct;
use crate::de::hysky::skyblocker::utils::container::container_matcher::ContainerMatcher;
use crate::de::hysky::skyblocker::utils::container::regex_container_matcher::RegexContainerMatcher;
use crate::de::hysky::skyblocker::utils::container::regex_container_matcher::RegexContainerMatcherBase;
use crate::de::hysky::skyblocker::utils::container::tooltip_adder::TooltipAdder;
use crate::de::hysky::skyblocker::utils::formatters;
use crate::de::hysky::skyblocker::utils::item_utils::ItemUtils;
use crate::de::hysky::skyblocker::utils::regex_utils::RegexUtils;
use crate::de::hysky::skyblocker::utils::render::text::grid_component::GridComponent;
use mcrust::java::text::decimal_format_symbols::Locale;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::world::inventory::slot::Slot;

/// java: private static final Pattern ESSENCE_PATTERN = Pattern.compile("Cost (?<amount>[\\d,]+) (?<type>[A-Za-z]+) Essence");
pub static ESSENCE_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("Cost (?<amount>[\\d,]+) (?<type>[A-Za-z]+) Essence"));
/// java: private static final String[] ESSENCE_TYPES = {"WITHER", "SPIDER", "UNDEAD", "DRAGON", "GOLD", "DIAMOND", "ICE", "CRIMSON", "FOREST", "FOSSIL", "SAFARI"};
pub static ESSENCE_TYPES: LazyLock<Vec<String>> = LazyLock::new(|| vec!["WITHER".to_string(), "SPIDER".to_string(), "UNDEAD".to_string(), "DRAGON".to_string(), "GOLD".to_string(), "DIAMOND".to_string(), "ICE".to_string(), "CRIMSON".to_string(), "FOREST".to_string(), "FOSSIL".to_string(), "SAFARI".to_string()]);
/// java: private static final Object2LongArrayMap<String> ESSENCE_PRICES = new Object2LongArrayMap<>(ESSENCE_TYPES, new long[ESSENCE_TYPES.length]);
pub static ESSENCE_PRICES: LazyLock<Mutex<std::collections::HashMap<String, i64>>> = LazyLock::new(|| Mutex::new(ESSENCE_TYPES.iter().map(|essence_type| (essence_type.clone(), 0i64)).collect()));

/// java: public class EssenceShopPrice extends SimpleTooltipAdder
#[derive(Clone)]
pub struct EssenceShopPrice {
    /// java: extends SimpleTooltipAdder
    pub base: SimpleTooltipAdderBase,
}

impl std::ops::Deref for EssenceShopPrice {
    type Target = SimpleTooltipAdderBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for EssenceShopPrice {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl EssenceShopPrice {
    /// java: public EssenceShopPrice(int priority)
    pub fn new(mut priority: i32) -> compat::Shared<Self> {
        let (_, __this) = SimpleTooltipAdderBase::new_in("\\S+ Essence Shop".to_string(), priority, move |__base| {
            let __h = compat::shared(Self {
                base: __base,
            });
            ((__h.clone() as compat::Shared<dyn SimpleTooltipAdder>), __h)
        });
        __this
    }

    /// java: public static void refreshEssencePrices(Object2ObjectMap<String, BazaarProduct> data)
    pub fn refresh_essence_prices(data: &std::collections::HashMap<String, BazaarProduct>) {
        for essence_type in ESSENCE_TYPES.iter() {
            let mut product: Option<BazaarProduct> = data.get(&format!("ESSENCE_{}", essence_type)).cloned();
            if let Some(product) = &product {
                let mut sell_price: Option<f64> = product.sell_price();
                if sell_price.is_some() {
                    compat::lock(&ESSENCE_PRICES).insert(essence_type.to_string(), ((sell_price.clone().unwrap()) as i64));
                }
            }
        }
    }

}

impl SimpleTooltipAdder for EssenceShopPrice {
    fn simple_tooltip_adder_base(&self) -> &SimpleTooltipAdderBase {
        &self.base
    }
    fn simple_tooltip_adder_base_mut(&mut self) -> &mut SimpleTooltipAdderBase {
        &mut self.base
    }

}

impl RegexContainerMatcher for EssenceShopPrice {
    fn regex_container_matcher_base(&self) -> &RegexContainerMatcherBase {
        &self.base.base
    }
    fn regex_container_matcher_base_mut(&mut self) -> &mut RegexContainerMatcherBase {
        &mut self.base.base
    }

}

impl ContainerMatcher for EssenceShopPrice {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public boolean isEnabled() (dispatch: the body for this class)
    fn is_enabled(&self) -> fn(&compat::Shared<dyn ContainerMatcher>) -> bool {
        EssenceShopPrice::is_enabled_dyn
    }

    /// java: @Override public boolean test(Screen screen) (dispatch: the body for this class)
    fn test(&self) -> fn(&compat::Shared<dyn ContainerMatcher>, &mcrust::net::minecraft::client::gui::screens::screen::Screen) -> bool {
        <dyn RegexContainerMatcher>::test_from::<Self>
    }

}

impl TooltipAdder for EssenceShopPrice {
    /// java: @Override public void addToTooltip(@Nullable Slot focusedSlot, ItemStack stack, List<Component> lines) (dispatch: the body for this class)
    fn add_to_tooltip(&self) -> fn(&compat::Shared<dyn TooltipAdder>, Option<Slot>, &mcrust::net::minecraft::world::item::item_stack::ItemStack, &mut Vec<MutableComponent>) {
        EssenceShopPrice::add_to_tooltip_dyn
    }

    /// java: @Override public int getPriority() (dispatch: the body for this class)
    fn get_priority(&self) -> fn(&compat::Shared<dyn TooltipAdder>) -> i32 {
        <dyn SimpleTooltipAdder>::get_priority_from::<Self>
    }

}

impl EssenceShopPrice {
    /// java: @Override public boolean isEnabled()
    pub fn is_enabled_impl(__this: &compat::Shared<Self>) -> bool {
        return compat::read(&SkyblockerConfigManager::get().general.item_tooltip, |__t| __t.show_essence_cost);
    }

    /// EssenceShopPrice.isEnabled as the dispatch of ContainerMatcher.isEnabled
    pub fn is_enabled_dyn(__this: &compat::Shared<dyn ContainerMatcher>) -> bool {
        Self::is_enabled_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public void addToTooltip(@Nullable Slot focusedSlot, ItemStack stack, List<Component> lines)
    pub fn add_to_tooltip_impl(__this: &compat::Shared<Self>, mut focused_slot: Option<Slot>, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, lines: &mut Vec<MutableComponent>) {
        let mut lore: String = ItemUtils::concatenate_lore(lines);
        let mut essence_matcher: compat::regex::Matcher = ESSENCE_PATTERN.matcher(&lore);
        let mut cost: Option<i64> = RegexUtils::find_long_from_matcher(&essence_matcher);
        if cost.is_none() {
            return;
        }
        let mut r#type: String = essence_matcher.group_named("type").unwrap();
        let mut price_data: i64 = compat::lock(&ESSENCE_PRICES).get(&(r#type).to_uppercase()).copied().unwrap_or_default();
        if price_data == (0 as i64) {
            return;
        }
        lines.push(GridComponent::of_2(&compat::jstring(&component::literal("Essence Cost:").with_style_format(ChatFormatting::Aqua)), &vec![(((component::empty().append((component::literal(&format!("{} coins", formatters::INTEGER_NUMBERS.get_mut().format((price_data.wrapping_mul(cost.clone().unwrap())) as f64)))).with_style_format(ChatFormatting::DarkAqua))).append(component::literal(" (").with_style_format(ChatFormatting::Gray))).append((component::literal(&format!("{} each", formatters::INTEGER_NUMBERS.get_mut().format((price_data) as f64)))).with_style_format(ChatFormatting::Gray))).append(component::literal(")").with_style_format(ChatFormatting::Gray))]));
    }

    /// EssenceShopPrice.addToTooltip as the dispatch of TooltipAdder.addToTooltip
    pub fn add_to_tooltip_dyn(__this: &compat::Shared<dyn TooltipAdder>, __p0: Option<Slot>, __p1: &mcrust::net::minecraft::world::item::item_stack::ItemStack, __p2: &mut Vec<MutableComponent>) {
        Self::add_to_tooltip_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2)
    }

}

//! Ported from de.hysky.skyblocker.skyblock.item.tooltip.adders.EstimatedItemValueTooltip by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::info::tooltip_info_type;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::info::tooltip_info_type::TooltipInfoType;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::simple_tooltip_adder::SimpleTooltipAdder;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::simple_tooltip_adder::SimpleTooltipAdderBase;
use crate::de::hysky::skyblocker::utils::container::container_matcher::ContainerMatcher;
use crate::de::hysky::skyblocker::utils::container::regex_container_matcher::RegexContainerMatcher;
use crate::de::hysky::skyblocker::utils::container::regex_container_matcher::RegexContainerMatcherBase;
use crate::de::hysky::skyblocker::utils::container::tooltip_adder::TooltipAdder;
use crate::de::hysky::skyblocker::utils::item_utils::ItemUtils;
use crate::de::hysky::skyblocker::utils::networth::networth_calculator::NetworthCalculator;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::world::inventory::slot::Slot;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::info::data_tooltip_info_type::DataTooltipInfoType as _;

/// java: public class EstimatedItemValueTooltip extends SimpleTooltipAdder
#[derive(Clone)]
pub struct EstimatedItemValueTooltip {
    /// java: extends SimpleTooltipAdder
    pub base: SimpleTooltipAdderBase,
}

impl std::ops::Deref for EstimatedItemValueTooltip {
    type Target = SimpleTooltipAdderBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for EstimatedItemValueTooltip {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl EstimatedItemValueTooltip {
    /// java: public EstimatedItemValueTooltip(int priority)
    pub fn new(mut priority: i32) -> compat::Shared<Self> {
        let (_, __this) = SimpleTooltipAdderBase::new_1_in(priority, move |__base| {
            let __h = compat::shared(Self {
                base: __base,
            });
            ((__h.clone() as compat::Shared<dyn SimpleTooltipAdder>), __h)
        });
        __this
    }

}

impl SimpleTooltipAdder for EstimatedItemValueTooltip {
    fn simple_tooltip_adder_base(&self) -> &SimpleTooltipAdderBase {
        &self.base
    }
    fn simple_tooltip_adder_base_mut(&mut self) -> &mut SimpleTooltipAdderBase {
        &mut self.base
    }

}

impl RegexContainerMatcher for EstimatedItemValueTooltip {
    fn regex_container_matcher_base(&self) -> &RegexContainerMatcherBase {
        &self.base.base
    }
    fn regex_container_matcher_base_mut(&mut self) -> &mut RegexContainerMatcherBase {
        &mut self.base.base
    }

}

impl ContainerMatcher for EstimatedItemValueTooltip {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public boolean isEnabled() (dispatch: the body for this class)
    fn is_enabled(&self) -> fn(&compat::Shared<dyn ContainerMatcher>) -> bool {
        EstimatedItemValueTooltip::is_enabled_dyn
    }

    /// java: @Override public boolean test(Screen screen) (dispatch: the body for this class)
    fn test(&self) -> fn(&compat::Shared<dyn ContainerMatcher>, &mcrust::net::minecraft::client::gui::screens::screen::Screen) -> bool {
        <dyn RegexContainerMatcher>::test_from::<Self>
    }

}

impl TooltipAdder for EstimatedItemValueTooltip {
    /// java: @Override public void addToTooltip(@Nullable Slot focusedSlot, ItemStack stack, List<Component> lines) (dispatch: the body for this class)
    fn add_to_tooltip(&self) -> fn(&compat::Shared<dyn TooltipAdder>, Option<Slot>, &mcrust::net::minecraft::world::item::item_stack::ItemStack, &mut Vec<MutableComponent>) {
        EstimatedItemValueTooltip::add_to_tooltip_dyn
    }

    /// java: @Override public int getPriority() (dispatch: the body for this class)
    fn get_priority(&self) -> fn(&compat::Shared<dyn TooltipAdder>) -> i32 {
        <dyn SimpleTooltipAdder>::get_priority_from::<Self>
    }

}

impl EstimatedItemValueTooltip {
    /// java: @Override public boolean isEnabled()
    pub fn is_enabled_impl(__this: &compat::Shared<Self>) -> bool {
        return ({ let __r = tooltip_info_type::ESTIMATED_ITEM_VALUE.get_mut(); let __f = compat::dispatch(__r, |__t| __t.is_tooltip_enabled()); __f(__r) });
    }

    /// EstimatedItemValueTooltip.isEnabled as the dispatch of ContainerMatcher.isEnabled
    pub fn is_enabled_dyn(__this: &compat::Shared<dyn ContainerMatcher>) -> bool {
        Self::is_enabled_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public void addToTooltip(@Nullable Slot focusedSlot, ItemStack stack, List<Component> lines)
    pub fn add_to_tooltip_impl(__this: &compat::Shared<Self>, mut focused_slot: Option<Slot>, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, lines: &mut Vec<MutableComponent>) {
        //   java: TooltipInfoType.BAZAAR.getData() != null && TooltipInfoType.BAZAAR.getData().containsKey(stack.getSkyblockApiId())
        if tooltip_info_type::BAZAAR.get_mut().lock().borrow().get_data().is_some_and(|__d| __d.contains_key(&stack.get_skyblock_api_id())) {
            return;
        }
        let mut count: i32 = (ItemUtils::get_item_count_in_sack(stack, &stack.skyblocker_get_lore_strings()).unwrap_or(ItemUtils::get_item_count_in_stash_component(&lines[0]).unwrap_or(stack.get_count()))).max(1);
        let mut result: crate::net::azureaaron::networth::networth_result::NetworthResult = NetworthCalculator::get_item_networth_2(stack, count);
        //   java: result.price() > 0
        if result.price() > 0.0 {
            //   java: lines.add(GridComponent.of(
            //   java: 					Component.literal("Est. Item Value:").withStyle(ChatFormatting.GOLD),
            //   java: 					ItemTooltip.getCoinsMessage(result.price(), count, true)));
            lines.push(crate::de::hysky::skyblocker::utils::render::text::grid_component::GridComponent::of(&vec![component::literal("Est. Item Value:").with_style_format(ChatFormatting::Gold), crate::de::hysky::skyblocker::skyblock::item::tooltip::item_tooltip::ItemTooltip::get_coins_message_3(result.price(), count, true)]));
        }
    }

    /// EstimatedItemValueTooltip.addToTooltip as the dispatch of TooltipAdder.addToTooltip
    pub fn add_to_tooltip_dyn(__this: &compat::Shared<dyn TooltipAdder>, __p0: Option<Slot>, __p1: &mcrust::net::minecraft::world::item::item_stack::ItemStack, __p2: &mut Vec<MutableComponent>) {
        Self::add_to_tooltip_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2)
    }

}

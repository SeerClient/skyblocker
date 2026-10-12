//! Ported from de.hysky.skyblocker.skyblock.item.tooltip.adders.MotesTooltip by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::info::tooltip_info_type;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::info::tooltip_info_type::TooltipInfoType;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::simple_tooltip_adder::SimpleTooltipAdder;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::simple_tooltip_adder::SimpleTooltipAdderBase;
use crate::de::hysky::skyblocker::utils::container::container_matcher::ContainerMatcher;
use crate::de::hysky::skyblocker::utils::container::regex_container_matcher::RegexContainerMatcher;
use crate::de::hysky::skyblocker::utils::container::regex_container_matcher::RegexContainerMatcherBase;
use crate::de::hysky::skyblocker::utils::container::tooltip_adder::TooltipAdder;
use mcrust::java::text::decimal_format_symbols::Locale;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::world::inventory::slot::Slot;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::info::data_tooltip_info_type::DataTooltipInfoType as _;

/// java: public class MotesTooltip extends SimpleTooltipAdder
#[derive(Clone)]
pub struct MotesTooltip {
    /// java: extends SimpleTooltipAdder
    pub base: SimpleTooltipAdderBase,
}

impl std::ops::Deref for MotesTooltip {
    type Target = SimpleTooltipAdderBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for MotesTooltip {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl MotesTooltip {
    /// java: public MotesTooltip(int priority)
    pub fn new(mut priority: i32) -> compat::Shared<Self> {
        let (_, __this) = SimpleTooltipAdderBase::new_1_in(priority, move |__base| {
            let __h = compat::shared(Self {
                base: __base,
            });
            ((__h.clone() as compat::Shared<dyn SimpleTooltipAdder>), __h)
        });
        __this
    }

    /// java: private static Component getMotesMessage(int price, int count)
    pub fn get_motes_message(mut price: i32, mut count: i32) -> MutableComponent {
        let mut motes_multiplier: f32 = (((SkyblockerConfigManager::get().other_locations.rift.mc_grubber_stacks as f32) * 0.05f32) + (1 as f32));
        let mut total_price: i32 = price.wrapping_mul(count);
        let mut total_price_string: String = format!("%1$,.1f");
        if count == 1 {
            return (component::literal(&format!("{} Motes", total_price_string.replace(".0", "")))).with_style_format(ChatFormatting::DarkAqua);
        }
        let mut each_price_string: String = format!("%1$,.1f");
        let mut message: MutableComponent = (component::literal(&format!("{} Motes ", total_price_string.replace(".0", "")))).with_style_format(ChatFormatting::DarkAqua);
        message.append((component::literal(&format!("({} each)", each_price_string.replace(".0", "")))).with_style_format(ChatFormatting::Gray));
        return message;
    }

}

impl SimpleTooltipAdder for MotesTooltip {
    fn simple_tooltip_adder_base(&self) -> &SimpleTooltipAdderBase {
        &self.base
    }
    fn simple_tooltip_adder_base_mut(&mut self) -> &mut SimpleTooltipAdderBase {
        &mut self.base
    }

}

impl RegexContainerMatcher for MotesTooltip {
    fn regex_container_matcher_base(&self) -> &RegexContainerMatcherBase {
        &self.base.base
    }
    fn regex_container_matcher_base_mut(&mut self) -> &mut RegexContainerMatcherBase {
        &mut self.base.base
    }

}

impl ContainerMatcher for MotesTooltip {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public boolean isEnabled() (dispatch: the body for this class)
    fn is_enabled(&self) -> fn(&compat::Shared<dyn ContainerMatcher>) -> bool {
        MotesTooltip::is_enabled_dyn
    }

    /// java: @Override public boolean test(Screen screen) (dispatch: the body for this class)
    fn test(&self) -> fn(&compat::Shared<dyn ContainerMatcher>, &mcrust::net::minecraft::client::gui::screens::screen::Screen) -> bool {
        <dyn RegexContainerMatcher>::test_from::<Self>
    }

}

impl TooltipAdder for MotesTooltip {
    /// java: @Override public void addToTooltip(@Nullable Slot focusedSlot, ItemStack stack, List<Component> lines) (dispatch: the body for this class)
    fn add_to_tooltip(&self) -> fn(&compat::Shared<dyn TooltipAdder>, Option<Slot>, &mcrust::net::minecraft::world::item::item_stack::ItemStack, &mut Vec<MutableComponent>) {
        MotesTooltip::add_to_tooltip_dyn
    }

    /// java: @Override public int getPriority() (dispatch: the body for this class)
    fn get_priority(&self) -> fn(&compat::Shared<dyn TooltipAdder>) -> i32 {
        <dyn SimpleTooltipAdder>::get_priority_from::<Self>
    }

}

impl MotesTooltip {
    /// java: @Override public boolean isEnabled()
    pub fn is_enabled_impl(__this: &compat::Shared<Self>) -> bool {
        return ({ let __r = tooltip_info_type::MOTES.get_mut(); let __f = compat::dispatch(__r, |__t| __t.is_tooltip_enabled()); __f(&compat::view(__r, |a| a as compat::Shared<dyn TooltipInfoType>)) });
    }

    /// MotesTooltip.isEnabled as the dispatch of ContainerMatcher.isEnabled
    pub fn is_enabled_dyn(__this: &compat::Shared<dyn ContainerMatcher>) -> bool {
        Self::is_enabled_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public void addToTooltip(@Nullable Slot focusedSlot, ItemStack stack, List<Component> lines)
    pub fn add_to_tooltip_impl(__this: &compat::Shared<Self>, mut focused_slot: Option<Slot>, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, lines: &mut Vec<MutableComponent>) {
        let mut internal_id: String = stack.get_skyblock_id();
        if tooltip_info_type::MOTES.get_mut().lock().borrow().has_or_null_warning(&internal_id) {
            // TODO(fabric2seer): `.getInt(..)` on a T
            //   java: lines.add(GridComponent.of(
            //   java: 					Component.literal("Motes Price:").withStyle(ChatFormatting.LIGHT_PURPLE),
            //   java: 					getMotesMessage(TooltipInfoType.MOTES.getData().getInt(internalID), stack.getCount())));
        }
    }

    /// MotesTooltip.addToTooltip as the dispatch of TooltipAdder.addToTooltip
    pub fn add_to_tooltip_dyn(__this: &compat::Shared<dyn TooltipAdder>, __p0: Option<Slot>, __p1: &mcrust::net::minecraft::world::item::item_stack::ItemStack, __p2: &mut Vec<MutableComponent>) {
        Self::add_to_tooltip_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2)
    }

}

//! Ported from de.hysky.skyblocker.skyblock.item.tooltip.adders.AccessoryTooltip by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::accessories::accessories_helper::AccessoriesHelper;
use crate::de::hysky::skyblocker::skyblock::accessories::accessories_helper::AccessoryReport;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::info::tooltip_info_type;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::info::tooltip_info_type::TooltipInfoType;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::simple_tooltip_adder::SimpleTooltipAdder;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::simple_tooltip_adder::SimpleTooltipAdderBase;
use crate::de::hysky::skyblocker::utils::container::container_matcher::ContainerMatcher;
use crate::de::hysky::skyblocker::utils::container::regex_container_matcher::RegexContainerMatcher;
use crate::de::hysky::skyblocker::utils::container::regex_container_matcher::RegexContainerMatcherBase;
use crate::de::hysky::skyblocker::utils::container::tooltip_adder::TooltipAdder;
use crate::de::hysky::skyblocker::utils::render::text::grid_component::GridComponent;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::world::inventory::slot::Slot;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::info::data_tooltip_info_type::DataTooltipInfoType as _;

/// java: public static final int COLLECTED_COLOUR = TextColor.GREEN.getValue();
pub static COLLECTED_COLOUR: LazyLock<i32> = LazyLock::new(|| mcrust::net::minecraft::network::chat::text_color::TextColor::GREEN.get_value());
/// java: public static final int UPGRADE_COLOUR = 0x218BFF;
pub const UPGRADE_COLOUR: i32 = 0x218BFF;
/// java: public static final int UPGRADABLE_COLOUR = 0xF8D048;
pub const UPGRADABLE_COLOUR: i32 = 0xF8D048;
/// java: public static final int DOWNGRADE_COLOUR = TextColor.GRAY.getValue();
pub static DOWNGRADE_COLOUR: LazyLock<i32> = LazyLock::new(|| mcrust::net::minecraft::network::chat::text_color::TextColor::GRAY.get_value());
/// java: public static final int MISSING_COLOUR = TextColor.RED.getValue();
pub static MISSING_COLOUR: LazyLock<i32> = LazyLock::new(|| mcrust::net::minecraft::network::chat::text_color::TextColor::RED.get_value());

/// java: public class AccessoryTooltip extends SimpleTooltipAdder
#[derive(Clone)]
pub struct AccessoryTooltip {
    /// java: extends SimpleTooltipAdder
    pub base: SimpleTooltipAdderBase,
}

impl std::ops::Deref for AccessoryTooltip {
    type Target = SimpleTooltipAdderBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for AccessoryTooltip {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl AccessoryTooltip {
    /// java: public AccessoryTooltip(int priority)
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

impl SimpleTooltipAdder for AccessoryTooltip {
    fn simple_tooltip_adder_base(&self) -> &SimpleTooltipAdderBase {
        &self.base
    }
    fn simple_tooltip_adder_base_mut(&mut self) -> &mut SimpleTooltipAdderBase {
        &mut self.base
    }

}

impl RegexContainerMatcher for AccessoryTooltip {
    fn regex_container_matcher_base(&self) -> &RegexContainerMatcherBase {
        &self.base.base
    }
    fn regex_container_matcher_base_mut(&mut self) -> &mut RegexContainerMatcherBase {
        &mut self.base.base
    }

}

impl ContainerMatcher for AccessoryTooltip {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public boolean isEnabled() (dispatch: the body for this class)
    fn is_enabled(&self) -> fn(&compat::Shared<dyn ContainerMatcher>) -> bool {
        AccessoryTooltip::is_enabled_dyn
    }

    /// java: @Override public boolean test(Screen screen) (dispatch: the body for this class)
    fn test(&self) -> fn(&compat::Shared<dyn ContainerMatcher>, &mcrust::net::minecraft::client::gui::screens::screen::Screen) -> bool {
        <dyn RegexContainerMatcher>::test_from::<Self>
    }

}

impl TooltipAdder for AccessoryTooltip {
    /// java: @Override public void addToTooltip(@Nullable Slot focusedSlot, ItemStack stack, List<Component> lines) (dispatch: the body for this class)
    fn add_to_tooltip(&self) -> fn(&compat::Shared<dyn TooltipAdder>, Option<Slot>, &mcrust::net::minecraft::world::item::item_stack::ItemStack, &mut Vec<MutableComponent>) {
        AccessoryTooltip::add_to_tooltip_dyn
    }

    /// java: @Override public int getPriority() (dispatch: the body for this class)
    fn get_priority(&self) -> fn(&compat::Shared<dyn TooltipAdder>) -> i32 {
        <dyn SimpleTooltipAdder>::get_priority_from::<Self>
    }

}

impl AccessoryTooltip {
    /// java: @Override public boolean isEnabled()
    pub fn is_enabled_impl(__this: &compat::Shared<Self>) -> bool {
        return ({ let __r = tooltip_info_type::ACCESSORIES.get_mut(); let __f = compat::dispatch(__r, |__t| __t.is_tooltip_enabled()); __f(&compat::view(__r, |a| a as compat::Shared<dyn TooltipInfoType>)) });
    }

    /// AccessoryTooltip.isEnabled as the dispatch of ContainerMatcher.isEnabled
    pub fn is_enabled_dyn(__this: &compat::Shared<dyn ContainerMatcher>) -> bool {
        Self::is_enabled_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public void addToTooltip(@Nullable Slot focusedSlot, ItemStack stack, List<Component> lines)
    pub fn add_to_tooltip_impl(__this: &compat::Shared<Self>, mut focused_slot: Option<Slot>, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, lines: &mut Vec<MutableComponent>) {
        let mut internal_id: String = stack.get_skyblock_id();
        if tooltip_info_type::ACCESSORIES.get_mut().lock().borrow().has_or_null_warning(&internal_id) {
            let mut report: (AccessoryReport, String) = AccessoriesHelper::calculate_report4_accessory(&internal_id);
            if report.0 != AccessoryReport::Ineligible {
                let mut title: MutableComponent = component::literal("Accessory:").with_color(0xF57542);
                let mut state_text: MutableComponent = match report.0 {
                    AccessoryReport::HasHighestTier => (component::literal("✔ Collected")).with_color(*COLLECTED_COLOUR),
                    AccessoryReport::IsGreaterTier => (component::literal("✦ Upgrade ")).with_color(UPGRADE_COLOUR).append(component::literal(&report.1).with_color(0xF8F8FF)),
                    AccessoryReport::HasGreaterTier => (component::literal("↑ Upgradable ")).with_color(UPGRADABLE_COLOUR).append(component::literal(&report.1).with_color(0xF8F8FF)),
                    AccessoryReport::OwnsBetterTier => (component::literal("↓ Downgrade ")).with_color(*DOWNGRADE_COLOUR).append(component::literal(&report.1).with_color(0xF8F8FF)),
                    AccessoryReport::Missing => (component::literal("✖ Missing ")).with_color(*MISSING_COLOUR).append(component::literal(&report.1).with_color(0xF8F8FF)),
                    _ => (component::literal("? Unknown")).with_style_format(ChatFormatting::Gray),
                };
                lines.push(GridComponent::of(&vec![title, state_text.clone()]));
            }
        }
    }

    /// AccessoryTooltip.addToTooltip as the dispatch of TooltipAdder.addToTooltip
    pub fn add_to_tooltip_dyn(__this: &compat::Shared<dyn TooltipAdder>, __p0: Option<Slot>, __p1: &mcrust::net::minecraft::world::item::item_stack::ItemStack, __p2: &mut Vec<MutableComponent>) {
        Self::add_to_tooltip_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2)
    }

}

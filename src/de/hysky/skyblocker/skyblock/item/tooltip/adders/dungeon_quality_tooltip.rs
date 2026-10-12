//! Ported from de.hysky.skyblocker.skyblock.item.tooltip.adders.DungeonQualityTooltip by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::simple_tooltip_adder::SimpleTooltipAdder;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::simple_tooltip_adder::SimpleTooltipAdderBase;
use crate::de::hysky::skyblocker::utils::container::container_matcher::ContainerMatcher;
use crate::de::hysky::skyblocker::utils::container::regex_container_matcher::RegexContainerMatcher;
use crate::de::hysky::skyblocker::utils::container::regex_container_matcher::RegexContainerMatcherBase;
use crate::de::hysky::skyblocker::utils::container::tooltip_adder::TooltipAdder;
use crate::de::hysky::skyblocker::utils::item_utils::ItemUtils;
use crate::de::hysky::skyblocker::utils::render::text::grid_component::GridComponent;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::nbt::tag::Tag;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::world::inventory::slot::Slot;

/// java: private static final ChatFormatting[] MAXED = {ChatFormatting.RED, ChatFormatting.BOLD};
pub static MAXED: compat::MainThread<Vec<ChatFormatting>> = compat::MainThread::new(|| vec![ChatFormatting::Red, ChatFormatting::Bold]);
/// java: private static final ChatFormatting[] NOT_MAXED = {ChatFormatting.BLUE};
pub static NOT_MAXED: compat::MainThread<Vec<ChatFormatting>> = compat::MainThread::new(|| vec![ChatFormatting::Blue]);

/// java: public class DungeonQualityTooltip extends SimpleTooltipAdder
#[derive(Clone)]
pub struct DungeonQualityTooltip {
    /// java: extends SimpleTooltipAdder
    pub base: SimpleTooltipAdderBase,
}

impl std::ops::Deref for DungeonQualityTooltip {
    type Target = SimpleTooltipAdderBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for DungeonQualityTooltip {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl DungeonQualityTooltip {
    /// java: public DungeonQualityTooltip(int priority)
    pub fn new(mut priority: i32) -> compat::Shared<Self> {
        let (_, __this) = SimpleTooltipAdderBase::new_1_in(priority, move |__base| {
            let __h = compat::shared(Self {
                base: __base,
            });
            ((__h.clone() as compat::Shared<dyn SimpleTooltipAdder>), __h)
        });
        __this
    }

    /// java: private String getItemTierFloor(int tier)
    pub fn get_item_tier_floor(__this: &compat::Shared<Self>, mut tier: i32) -> String {
        return match tier {
            0 => "E".to_string(),
            1 => "F1".to_string(),
            2 => "F2".to_string(),
            3 => "F3".to_string(),
            4 => "F4/M1".to_string(),
            5 => "F5/M2".to_string(),
            6 => "F6/M3".to_string(),
            7 => "F7/M4".to_string(),
            8 => "M5".to_string(),
            9 => "M6".to_string(),
            10 => "M7".to_string(),
            _ => "Unknown".to_string(),
        };
    }

}

impl SimpleTooltipAdder for DungeonQualityTooltip {
    fn simple_tooltip_adder_base(&self) -> &SimpleTooltipAdderBase {
        &self.base
    }
    fn simple_tooltip_adder_base_mut(&mut self) -> &mut SimpleTooltipAdderBase {
        &mut self.base
    }

}

impl RegexContainerMatcher for DungeonQualityTooltip {
    fn regex_container_matcher_base(&self) -> &RegexContainerMatcherBase {
        &self.base.base
    }
    fn regex_container_matcher_base_mut(&mut self) -> &mut RegexContainerMatcherBase {
        &mut self.base.base
    }

}

impl ContainerMatcher for DungeonQualityTooltip {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public boolean isEnabled() (dispatch: the body for this class)
    fn is_enabled(&self) -> fn(&compat::Shared<dyn ContainerMatcher>) -> bool {
        DungeonQualityTooltip::is_enabled_dyn
    }

    /// java: @Override public boolean test(Screen screen) (dispatch: the body for this class)
    fn test(&self) -> fn(&compat::Shared<dyn ContainerMatcher>, &mcrust::net::minecraft::client::gui::screens::screen::Screen) -> bool {
        <dyn RegexContainerMatcher>::test_from::<Self>
    }

}

impl TooltipAdder for DungeonQualityTooltip {
    /// java: @Override public void addToTooltip(@Nullable Slot focusedSlot, ItemStack stack, List<Component> lines) (dispatch: the body for this class)
    fn add_to_tooltip(&self) -> fn(&compat::Shared<dyn TooltipAdder>, Option<Slot>, &mcrust::net::minecraft::world::item::item_stack::ItemStack, &mut Vec<MutableComponent>) {
        DungeonQualityTooltip::add_to_tooltip_dyn
    }

    /// java: @Override public int getPriority() (dispatch: the body for this class)
    fn get_priority(&self) -> fn(&compat::Shared<dyn TooltipAdder>) -> i32 {
        <dyn SimpleTooltipAdder>::get_priority_from::<Self>
    }

}

impl DungeonQualityTooltip {
    /// java: @Override public boolean isEnabled()
    pub fn is_enabled_impl(__this: &compat::Shared<Self>) -> bool {
        return compat::read(&SkyblockerConfigManager::get().general.item_tooltip, |__t| __t.dungeon_quality);
    }

    /// DungeonQualityTooltip.isEnabled as the dispatch of ContainerMatcher.isEnabled
    pub fn is_enabled_dyn(__this: &compat::Shared<dyn ContainerMatcher>) -> bool {
        Self::is_enabled_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public void addToTooltip(@Nullable Slot focusedSlot, ItemStack stack, List<Component> lines)
    pub fn add_to_tooltip_impl(__this: &compat::Shared<Self>, mut focused_slot: Option<Slot>, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, lines: &mut Vec<MutableComponent>) {
        let mut custom_data: Option<Tag> = Some(ItemUtils::get_custom_data(stack));
        if custom_data.is_none() || !(custom_data.as_ref().unwrap().contains("baseStatBoostPercentage")) {
            return;
        }
        let mut base_stat_boost_percentage: i32 = custom_data.as_ref().unwrap().get_int_or("baseStatBoostPercentage", 0);
        let mut max_quality: bool = (base_stat_boost_percentage == 50);
        let mut style: Vec<ChatFormatting> = (if max_quality { MAXED.get_mut().clone() } else { NOT_MAXED.get_mut().clone() });
        lines.push(GridComponent::of(&vec![component::literal("Item Quality:").with_style_formats(&style), (component::literal(&format!("{}/50", base_stat_boost_percentage))).with_style_formats(&style)]));
        if custom_data.as_ref().unwrap().contains("item_tier") {
            let mut item_tier: i32 = custom_data.as_ref().unwrap().get_int_or("item_tier", 0);
            lines.push(GridComponent::of(&vec![component::literal("Floor Tier:").with_style_formats(&style), (component::literal(&format!("{} ({})", item_tier, DungeonQualityTooltip::get_item_tier_floor(__this, item_tier)))).with_style_formats(&style)]));
        }
    }

    /// DungeonQualityTooltip.addToTooltip as the dispatch of TooltipAdder.addToTooltip
    pub fn add_to_tooltip_dyn(__this: &compat::Shared<dyn TooltipAdder>, __p0: Option<Slot>, __p1: &mcrust::net::minecraft::world::item::item_stack::ItemStack, __p2: &mut Vec<MutableComponent>) {
        Self::add_to_tooltip_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2)
    }

}

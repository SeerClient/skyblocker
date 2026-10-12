//! Ported from de.hysky.skyblocker.skyblock.item.tooltip.adders.EvolvingItemProgressTooltip by fabric2seer.

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
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;

/// java: public class EvolvingItemProgressTooltip extends SimpleTooltipAdder
#[derive(Clone)]
pub struct EvolvingItemProgressTooltip {
    /// java: extends SimpleTooltipAdder
    pub base: SimpleTooltipAdderBase,
    /// java: final List<String> evolutionItems = List.of("DARK_CACAO_TRUFFLE", "MOBY_DUCK", "ROSEWATER_FLASK", "NOT_VERY_MOLDY_BREAD", "DISCRITE", "NEW_BOTTLE_OF_JYRRE", "TR
    pub evolution_items: Vec<String>,
}

impl std::ops::Deref for EvolvingItemProgressTooltip {
    type Target = SimpleTooltipAdderBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for EvolvingItemProgressTooltip {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl EvolvingItemProgressTooltip {
    /// java: public EvolvingItemProgressTooltip(int priority)
    pub fn new(mut priority: i32) -> compat::Shared<Self> {
        let (_, __this) = SimpleTooltipAdderBase::new_1_in(priority, move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                evolution_items: vec!["DARK_CACAO_TRUFFLE".to_string(), "MOBY_DUCK".to_string(), "ROSEWATER_FLASK".to_string(), "NOT_VERY_MOLDY_BREAD".to_string(), "DISCRITE".to_string(), "NEW_BOTTLE_OF_JYRRE".to_string(), "TRAINING_WEIGHTS".to_string()],
            });
            ((__h.clone() as compat::Shared<dyn SimpleTooltipAdder>), __h)
        });
        __this
    }

}

impl SimpleTooltipAdder for EvolvingItemProgressTooltip {
    fn simple_tooltip_adder_base(&self) -> &SimpleTooltipAdderBase {
        &self.base
    }
    fn simple_tooltip_adder_base_mut(&mut self) -> &mut SimpleTooltipAdderBase {
        &mut self.base
    }

}

impl RegexContainerMatcher for EvolvingItemProgressTooltip {
    fn regex_container_matcher_base(&self) -> &RegexContainerMatcherBase {
        &self.base.base
    }
    fn regex_container_matcher_base_mut(&mut self) -> &mut RegexContainerMatcherBase {
        &mut self.base.base
    }

}

impl ContainerMatcher for EvolvingItemProgressTooltip {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public boolean isEnabled() (dispatch: the body for this class)
    fn is_enabled(&self) -> fn(&compat::Shared<dyn ContainerMatcher>) -> bool {
        EvolvingItemProgressTooltip::is_enabled_dyn
    }

    /// java: @Override public boolean test(Screen screen) (dispatch: the body for this class)
    fn test(&self) -> fn(&compat::Shared<dyn ContainerMatcher>, &mcrust::net::minecraft::client::gui::screens::screen::Screen) -> bool {
        <dyn RegexContainerMatcher>::test_from::<Self>
    }

}

impl TooltipAdder for EvolvingItemProgressTooltip {
    /// java: @Override public void addToTooltip(@Nullable Slot focusedSlot, ItemStack stack, List<Component> lines) (dispatch: the body for this class)
    fn add_to_tooltip(&self) -> fn(&compat::Shared<dyn TooltipAdder>, Option<Slot>, &mcrust::net::minecraft::world::item::item_stack::ItemStack, &mut Vec<MutableComponent>) {
        EvolvingItemProgressTooltip::add_to_tooltip_dyn
    }

    /// java: @Override public int getPriority() (dispatch: the body for this class)
    fn get_priority(&self) -> fn(&compat::Shared<dyn TooltipAdder>) -> i32 {
        <dyn SimpleTooltipAdder>::get_priority_from::<Self>
    }

}

impl EvolvingItemProgressTooltip {
    /// java: @Override public boolean isEnabled()
    pub fn is_enabled_impl(__this: &compat::Shared<Self>) -> bool {
        return compat::read(&SkyblockerConfigManager::get().general.item_tooltip, |__t| __t.enable_evolving_item_progress);
    }

    /// EvolvingItemProgressTooltip.isEnabled as the dispatch of ContainerMatcher.isEnabled
    pub fn is_enabled_dyn(__this: &compat::Shared<dyn ContainerMatcher>) -> bool {
        Self::is_enabled_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public void addToTooltip(@Nullable Slot focusedSlot, ItemStack stack, List<Component> lines)
    pub fn add_to_tooltip_impl(__this: &compat::Shared<Self>, mut focused_slot: Option<Slot>, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, lines: &mut Vec<MutableComponent>) {
        if !(__this.lock().borrow().evolution_items.iter().any(|e| *e == stack.get_skyblock_id())) {
            return;
        }
        let mut custom_data: Tag = ItemUtils::get_custom_data(stack);
        let mut seconds_held: i32 = match stack.get_skyblock_id().as_str() {
            "DISCRITE" => custom_data.get_int_or("rift_discrite_seconds", 0),
            "NEW_BOTTLE_OF_JYRRE" => custom_data.get_int_or("bottle_of_jyrre_seconds", 0),
            "TRAINING_WEIGHTS" => custom_data.get_int_or("trainingWeightsHeldTime", 0),
            _ => custom_data.get_int_or("seconds_held", 0),
        };
        let mut max_seconds: i32 = match stack.get_skyblock_id().as_str() {
            "TRAINING_WEIGHTS" => 5500i32.wrapping_mul(60),
            _ => 3600i32.wrapping_mul(300),
        };
        lines.push(GridComponent::of_2(&compat::jstring(&component::literal("Age:").with_style_format(ChatFormatting::Gray)), &vec![(((((component::empty().append((component::literal(&format!("{:3.1}", (((seconds_held) as f32) / (3600 as f32))))).with_style_format(ChatFormatting::Red))).append((component::literal("/")).with_style_format(ChatFormatting::Gray))).append((component::literal(&format!("{:3.1}h ", (((max_seconds) as f32) / (3600 as f32))))).with_style_format(ChatFormatting::Red))).append(component::literal("(").with_style_format(ChatFormatting::Gray))).append((component::literal(&format!("{:2.1}%", ((100.0 * (seconds_held as f64)) / (max_seconds as f64))))).with_style_format(ChatFormatting::Red))).append(component::literal(")").with_style_format(ChatFormatting::Gray))]));
    }

    /// EvolvingItemProgressTooltip.addToTooltip as the dispatch of TooltipAdder.addToTooltip
    pub fn add_to_tooltip_dyn(__this: &compat::Shared<dyn TooltipAdder>, __p0: Option<Slot>, __p1: &mcrust::net::minecraft::world::item::item_stack::ItemStack, __p2: &mut Vec<MutableComponent>) {
        Self::add_to_tooltip_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2)
    }

}

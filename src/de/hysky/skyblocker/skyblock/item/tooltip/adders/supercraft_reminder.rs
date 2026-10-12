//! Ported from de.hysky.skyblocker.skyblock.item.tooltip.adders.SupercraftReminder by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::simple_tooltip_adder::SimpleTooltipAdder;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::simple_tooltip_adder::SimpleTooltipAdderBase;
use crate::de::hysky::skyblocker::utils::container::container_matcher::ContainerMatcher;
use crate::de::hysky::skyblocker::utils::container::regex_container_matcher::RegexContainerMatcher;
use crate::de::hysky::skyblocker::utils::container::regex_container_matcher::RegexContainerMatcherBase;
use crate::de::hysky::skyblocker::utils::container::tooltip_adder::TooltipAdder;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::world::inventory::slot::Slot;
use mcrust::net::minecraft::world::item::items;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;
use mcrust::net::minecraft::network::chat::component::Component as _;
use mcrust::net::minecraft::world::container::Container as _;

/// java: private static final byte SUPERCRAFT_SLOT = 32;
pub const SUPERCRAFT_SLOT: i8 = (32) as i8;
/// java: private static final byte RECIPE_RESULT_SLOT = 25;
pub const RECIPE_RESULT_SLOT: i8 = (25) as i8;

/// java: public class SupercraftReminder extends SimpleTooltipAdder
#[derive(Clone)]
pub struct SupercraftReminder {
    /// java: extends SimpleTooltipAdder
    pub base: SimpleTooltipAdderBase,
}

impl std::ops::Deref for SupercraftReminder {
    type Target = SimpleTooltipAdderBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for SupercraftReminder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl SupercraftReminder {
    /// java: public SupercraftReminder()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = SimpleTooltipAdderBase::new2_in(compat::regex::Pattern::compile("^.+ Recipe$"), i32::MIN, move |__base| {
            let __h = compat::shared(Self {
                base: __base,
            });
            ((__h.clone() as compat::Shared<dyn SimpleTooltipAdder>), __h)
        });
        __this
    }

}

impl SimpleTooltipAdder for SupercraftReminder {
    fn simple_tooltip_adder_base(&self) -> &SimpleTooltipAdderBase {
        &self.base
    }
    fn simple_tooltip_adder_base_mut(&mut self) -> &mut SimpleTooltipAdderBase {
        &mut self.base
    }

}

impl RegexContainerMatcher for SupercraftReminder {
    fn regex_container_matcher_base(&self) -> &RegexContainerMatcherBase {
        &self.base.base
    }
    fn regex_container_matcher_base_mut(&mut self) -> &mut RegexContainerMatcherBase {
        &mut self.base.base
    }

}

impl ContainerMatcher for SupercraftReminder {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public boolean isEnabled() (dispatch: the body for this class)
    fn is_enabled(&self) -> fn(&compat::Shared<dyn ContainerMatcher>) -> bool {
        SupercraftReminder::is_enabled_dyn
    }

    /// java: @Override public boolean test(Screen screen) (dispatch: the body for this class)
    fn test(&self) -> fn(&compat::Shared<dyn ContainerMatcher>, &mcrust::net::minecraft::client::gui::screens::screen::Screen) -> bool {
        <dyn RegexContainerMatcher>::test_from::<Self>
    }

}

impl TooltipAdder for SupercraftReminder {
    /// java: @Override public void addToTooltip(@Nullable Slot focusedSlot, ItemStack stack, List<Component> lines) (dispatch: the body for this class)
    fn add_to_tooltip(&self) -> fn(&compat::Shared<dyn TooltipAdder>, Option<Slot>, &mcrust::net::minecraft::world::item::item_stack::ItemStack, &mut Vec<MutableComponent>) {
        SupercraftReminder::add_to_tooltip_dyn
    }

    /// java: @Override public int getPriority() (dispatch: the body for this class)
    fn get_priority(&self) -> fn(&compat::Shared<dyn TooltipAdder>) -> i32 {
        <dyn SimpleTooltipAdder>::get_priority_from::<Self>
    }

}

impl SupercraftReminder {
    /// java: @Override public boolean isEnabled()
    pub fn is_enabled_impl(__this: &compat::Shared<Self>) -> bool {
        return true;
    }

    /// SupercraftReminder.isEnabled as the dispatch of ContainerMatcher.isEnabled
    pub fn is_enabled_dyn(__this: &compat::Shared<dyn ContainerMatcher>) -> bool {
        Self::is_enabled_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public void addToTooltip(@Nullable Slot focusedSlot, ItemStack stack, List<Component> lines)
    pub fn add_to_tooltip_impl(__this: &compat::Shared<Self>, mut focused_slot: Option<Slot>, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, lines: &mut Vec<MutableComponent>) {
        if focused_slot.is_none() || focused_slot.as_ref().unwrap().index != (SUPERCRAFT_SLOT as i32) || !(stack.is_item(&items::items().get("golden_pickaxe"))) {
            return;
        }
        let mut uuid: String = (focused_slot.as_ref().unwrap().container.get_item((RECIPE_RESULT_SLOT) as i32)).get_uuid();
        if !uuid.is_empty() {
            return;
        }
        let mut index: i32 = (lines.len() as i32).wrapping_sub(1);
        if (lines[((lines.len() as i32).wrapping_sub(2)) as usize]).get_string() == "Recipe not unlocked!" {
            index -= 1;
        }
        lines.insert((index) as usize, (component::literal("Shift-Click to maximize the amount!")).with_style_format(ChatFormatting::Gold));
    }

    /// SupercraftReminder.addToTooltip as the dispatch of TooltipAdder.addToTooltip
    pub fn add_to_tooltip_dyn(__this: &compat::Shared<dyn TooltipAdder>, __p0: Option<Slot>, __p1: &mcrust::net::minecraft::world::item::item_stack::ItemStack, __p2: &mut Vec<MutableComponent>) {
        Self::add_to_tooltip_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2)
    }

}

//! Ported from de.hysky.skyblocker.skyblock.item.tooltip.adders.LineSmoothener by fabric2seer.

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
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final Component BUMPY_LINE = Component.literal("-----------------").withStyle(ChatFormatting.DARK_GRAY, ChatFormatting.STRIKETHROUGH);
pub static BUMPY_LINE: compat::MainThread<MutableComponent> = compat::MainThread::new(|| (component::literal("-----------------")).with_style_formats(&[ChatFormatting::DarkGray, ChatFormatting::Strikethrough]));

/// java: public class LineSmoothener extends SimpleTooltipAdder
#[derive(Clone)]
pub struct LineSmoothener {
    /// java: extends SimpleTooltipAdder
    pub base: SimpleTooltipAdderBase,
}

impl std::ops::Deref for LineSmoothener {
    type Target = SimpleTooltipAdderBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for LineSmoothener {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl LineSmoothener {
    /// java: public LineSmoothener()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = SimpleTooltipAdderBase::new_1_in(i32::MIN, move |__base| {
            let __h = compat::shared(Self {
                base: __base,
            });
            ((__h.clone() as compat::Shared<dyn SimpleTooltipAdder>), __h)
        });
        __this
    }

    /// java: public static Component createSmoothLine()
    pub fn create_smooth_line() -> MutableComponent {
        return component::literal("                    ").with_style_formats(&[ChatFormatting::DarkGray, ChatFormatting::Strikethrough, ChatFormatting::Bold]);
    }

}

impl SimpleTooltipAdder for LineSmoothener {
    fn simple_tooltip_adder_base(&self) -> &SimpleTooltipAdderBase {
        &self.base
    }
    fn simple_tooltip_adder_base_mut(&mut self) -> &mut SimpleTooltipAdderBase {
        &mut self.base
    }

}

impl RegexContainerMatcher for LineSmoothener {
    fn regex_container_matcher_base(&self) -> &RegexContainerMatcherBase {
        &self.base.base
    }
    fn regex_container_matcher_base_mut(&mut self) -> &mut RegexContainerMatcherBase {
        &mut self.base.base
    }

}

impl ContainerMatcher for LineSmoothener {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public boolean isEnabled() (dispatch: the body for this class)
    fn is_enabled(&self) -> fn(&compat::Shared<dyn ContainerMatcher>) -> bool {
        LineSmoothener::is_enabled_dyn
    }

    /// java: @Override public boolean test(Screen screen) (dispatch: the body for this class)
    fn test(&self) -> fn(&compat::Shared<dyn ContainerMatcher>, &mcrust::net::minecraft::client::gui::screens::screen::Screen) -> bool {
        <dyn RegexContainerMatcher>::test_from::<Self>
    }

}

impl TooltipAdder for LineSmoothener {
    /// java: @Override public void addToTooltip(@Nullable Slot focusedSlot, ItemStack stack, List<Component> lines) (dispatch: the body for this class)
    fn add_to_tooltip(&self) -> fn(&compat::Shared<dyn TooltipAdder>, Option<Slot>, &mcrust::net::minecraft::world::item::item_stack::ItemStack, &mut Vec<MutableComponent>) {
        LineSmoothener::add_to_tooltip_dyn
    }

    /// java: @Override public int getPriority() (dispatch: the body for this class)
    fn get_priority(&self) -> fn(&compat::Shared<dyn TooltipAdder>) -> i32 {
        <dyn SimpleTooltipAdder>::get_priority_from::<Self>
    }

}

impl LineSmoothener {
    /// java: @Override public boolean isEnabled()
    pub fn is_enabled_impl(__this: &compat::Shared<Self>) -> bool {
        return true;
    }

    /// LineSmoothener.isEnabled as the dispatch of ContainerMatcher.isEnabled
    pub fn is_enabled_dyn(__this: &compat::Shared<dyn ContainerMatcher>) -> bool {
        Self::is_enabled_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public void addToTooltip(@Nullable Slot focusedSlot, ItemStack stack, List<Component> lines)
    pub fn add_to_tooltip_impl(__this: &compat::Shared<Self>, mut focused_slot: Option<Slot>, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, lines: &mut Vec<MutableComponent>) {
        for i in 0..(lines.len() as i32) {
            let line_siblings = (lines[(i) as usize]).get_siblings();
            if (line_siblings.len() as i32) == 1 && line_siblings[0] == BUMPY_LINE.get_mut() {
                lines[(i) as usize] = LineSmoothener::create_smooth_line();
            }
        }
    }

    /// LineSmoothener.addToTooltip as the dispatch of TooltipAdder.addToTooltip
    pub fn add_to_tooltip_dyn(__this: &compat::Shared<dyn TooltipAdder>, __p0: Option<Slot>, __p1: &mcrust::net::minecraft::world::item::item_stack::ItemStack, __p2: &mut Vec<MutableComponent>) {
        Self::add_to_tooltip_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2)
    }

}

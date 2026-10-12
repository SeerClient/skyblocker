//! Ported from de.hysky.skyblocker.skyblock.item.tooltip.adders.TrueHexDyeScreenDisplay by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
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
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final Pattern DYE_SCREEN_NAMES = Pattern.compile("Dye Compendium|Dyes");
pub static DYE_SCREEN_NAMES: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("Dye Compendium|Dyes"));
/// java: private static final Pattern HEX_PATTERN = Pattern.compile("Hex (?<hex>#[A-Fa-f0-9]{6})");
pub static HEX_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("Hex (?<hex>#[A-Fa-f0-9]{6})"));

/// java: public class TrueHexDyeScreenDisplay extends SimpleTooltipAdder
#[derive(Clone)]
pub struct TrueHexDyeScreenDisplay {
    /// java: extends SimpleTooltipAdder
    pub base: SimpleTooltipAdderBase,
}

impl std::ops::Deref for TrueHexDyeScreenDisplay {
    type Target = SimpleTooltipAdderBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for TrueHexDyeScreenDisplay {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl TrueHexDyeScreenDisplay {
    /// java: public TrueHexDyeScreenDisplay()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = SimpleTooltipAdderBase::new2_in(DYE_SCREEN_NAMES.clone(), i32::MIN, move |__base| {
            let __h = compat::shared(Self {
                base: __base,
            });
            ((__h.clone() as compat::Shared<dyn SimpleTooltipAdder>), __h)
        });
        __this
    }

}

impl SimpleTooltipAdder for TrueHexDyeScreenDisplay {
    fn simple_tooltip_adder_base(&self) -> &SimpleTooltipAdderBase {
        &self.base
    }
    fn simple_tooltip_adder_base_mut(&mut self) -> &mut SimpleTooltipAdderBase {
        &mut self.base
    }

}

impl RegexContainerMatcher for TrueHexDyeScreenDisplay {
    fn regex_container_matcher_base(&self) -> &RegexContainerMatcherBase {
        &self.base.base
    }
    fn regex_container_matcher_base_mut(&mut self) -> &mut RegexContainerMatcherBase {
        &mut self.base.base
    }

}

impl ContainerMatcher for TrueHexDyeScreenDisplay {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public boolean isEnabled() (dispatch: the body for this class)
    fn is_enabled(&self) -> fn(&compat::Shared<dyn ContainerMatcher>) -> bool {
        TrueHexDyeScreenDisplay::is_enabled_dyn
    }

    /// java: @Override public boolean test(Screen screen) (dispatch: the body for this class)
    fn test(&self) -> fn(&compat::Shared<dyn ContainerMatcher>, &mcrust::net::minecraft::client::gui::screens::screen::Screen) -> bool {
        <dyn RegexContainerMatcher>::test_from::<Self>
    }

}

impl TooltipAdder for TrueHexDyeScreenDisplay {
    /// java: @Override public void addToTooltip(@Nullable Slot focusedSlot, ItemStack stack, List<Component> lines) (dispatch: the body for this class)
    fn add_to_tooltip(&self) -> fn(&compat::Shared<dyn TooltipAdder>, Option<Slot>, &mcrust::net::minecraft::world::item::item_stack::ItemStack, &mut Vec<MutableComponent>) {
        TrueHexDyeScreenDisplay::add_to_tooltip_dyn
    }

    /// java: @Override public int getPriority() (dispatch: the body for this class)
    fn get_priority(&self) -> fn(&compat::Shared<dyn TooltipAdder>) -> i32 {
        <dyn SimpleTooltipAdder>::get_priority_from::<Self>
    }

}

impl TrueHexDyeScreenDisplay {
    /// java: @Override public boolean isEnabled()
    pub fn is_enabled_impl(__this: &compat::Shared<Self>) -> bool {
        return !(SkyblockerConfigManager::get().debug.enable_repo_dev);
    }

    /// TrueHexDyeScreenDisplay.isEnabled as the dispatch of ContainerMatcher.isEnabled
    pub fn is_enabled_dyn(__this: &compat::Shared<dyn ContainerMatcher>) -> bool {
        Self::is_enabled_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public void addToTooltip(@Nullable Slot focusedSlot, ItemStack stack, List<Component> lines)
    pub fn add_to_tooltip_impl(__this: &compat::Shared<Self>, mut focused_slot: Option<Slot>, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, lines: &mut Vec<MutableComponent>) {
        let mut name: String = compat::component_of(stack.get_hover_name()).get_string();
        if stack.is_item(&items::items().get("player_head")) && name.ends_with("Dye") {
            for i in 0..(lines.len() as i32) {
                let mut line: MutableComponent = lines[(i) as usize].clone();
                let mut matcher: compat::regex::Matcher = HEX_PATTERN.matcher(&line.get_string());
                if matcher.matches() {
                    let mut hex: String = matcher.group_named("hex").unwrap();
                    line = line.copy();
                    let siblings = line.get_siblings();
                    siblings.clear();
                    siblings.push(component::literal("Hex ").with_style_format(ChatFormatting::DarkGray));
                    siblings.push(component::literal(&hex).with_color(hex.trim().parse::<i32>().expect("NumberFormatException")));
                    lines[(i) as usize] = line.clone();
                    return;
                }
            }
        }
    }

    /// TrueHexDyeScreenDisplay.addToTooltip as the dispatch of TooltipAdder.addToTooltip
    pub fn add_to_tooltip_dyn(__this: &compat::Shared<dyn TooltipAdder>, __p0: Option<Slot>, __p1: &mcrust::net::minecraft::world::item::item_stack::ItemStack, __p2: &mut Vec<MutableComponent>) {
        Self::add_to_tooltip_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2)
    }

}

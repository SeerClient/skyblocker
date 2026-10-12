//! Ported from de.hysky.skyblocker.skyblock.auction.CopyUnderbidPrice by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::utils::constants;
use crate::de::hysky::skyblocker::utils::container::container_matcher::ContainerMatcher;
use crate::de::hysky::skyblocker::utils::container::container_solver::ContainerSolver;
use crate::de::hysky::skyblocker::utils::container::regex_container_matcher::RegexContainerMatcher;
use crate::de::hysky::skyblocker::utils::container::regex_container_matcher::RegexContainerMatcherBase;
use crate::de::hysky::skyblocker::utils::container::simple_container_solver::SimpleContainerSolver;
use crate::de::hysky::skyblocker::utils::container::simple_container_solver::SimpleContainerSolverBase;
use crate::de::hysky::skyblocker::utils::formatters;
use crate::de::hysky::skyblocker::utils::item_utils::ItemUtils;
use crate::de::hysky::skyblocker::utils::render::gui::color_highlight::ColorHighlight;
use crate::de::hysky::skyblocker::utils::resettable::Resettable;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::AbstractContainerScreen;
use mcrust::net::minecraft::client::gui::screens::inventory::container_screen::ContainerScreen;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::world::item::item_stack;
use mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::ContainerScreenHost as _;

/// java: public class CopyUnderbidPrice extends SimpleContainerSolver
#[derive(Clone)]
pub struct CopyUnderbidPrice {
    /// java: extends SimpleContainerSolver
    pub base: SimpleContainerSolverBase,
    /// java: private boolean copied;
    pub copied: bool,
    /// java: private ItemStack previousItem = ItemStack.EMPTY;
    pub previous_item: mcrust::net::minecraft::world::item::item_stack::ItemStack,
}

impl std::ops::Deref for CopyUnderbidPrice {
    type Target = SimpleContainerSolverBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for CopyUnderbidPrice {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl CopyUnderbidPrice {
    /// java: public CopyUnderbidPrice()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = SimpleContainerSolverBase::new_in("^Create BIN Auction$".to_string(), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                copied: false,
                previous_item: mcrust::net::minecraft::world::item::item_stack::empty(),
            });
            ((__h.clone() as compat::Shared<dyn SimpleContainerSolver>), __h)
        });
        __this
    }

}

impl SimpleContainerSolver for CopyUnderbidPrice {
    fn simple_container_solver_base(&self) -> &SimpleContainerSolverBase {
        &self.base
    }
    fn simple_container_solver_base_mut(&mut self) -> &mut SimpleContainerSolverBase {
        &mut self.base
    }

}

impl RegexContainerMatcher for CopyUnderbidPrice {
    fn regex_container_matcher_base(&self) -> &RegexContainerMatcherBase {
        &self.base.base
    }
    fn regex_container_matcher_base_mut(&mut self) -> &mut RegexContainerMatcherBase {
        &mut self.base.base
    }

    /// java: @Override public Pattern getTitlePattern() (dispatch: the body for this class)
    fn get_title_pattern(&self) -> fn(&compat::Shared<dyn RegexContainerMatcher>) -> Option<compat::regex::Pattern> {
        <dyn SimpleContainerSolver>::get_title_pattern_from::<Self>
    }

}

impl ContainerMatcher for CopyUnderbidPrice {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public boolean isEnabled() (dispatch: the body for this class)
    fn is_enabled(&self) -> fn(&compat::Shared<dyn ContainerMatcher>) -> bool {
        CopyUnderbidPrice::is_enabled_dyn
    }

    /// java: @Override public boolean test(Screen screen) (dispatch: the body for this class)
    fn test(&self) -> fn(&compat::Shared<dyn ContainerMatcher>, &mcrust::net::minecraft::client::gui::screens::screen::Screen) -> bool {
        <dyn RegexContainerMatcher>::test_from::<Self>
    }

}

impl ContainerSolver for CopyUnderbidPrice {
    /// java: @Override public void start(AbstractContainerScreen<?> screen) (dispatch: the body for this class)
    fn start(&self) -> fn(&compat::Shared<dyn ContainerSolver>, &AbstractContainerScreen) {
        CopyUnderbidPrice::start_dyn
    }

    /// java: @Override public void markDirty() (dispatch: the body for this class)
    fn mark_dirty(&self) -> fn(&compat::Shared<dyn ContainerSolver>) {
        CopyUnderbidPrice::mark_dirty_dyn
    }

    /// java: @Override public List<ColorHighlight> getColors(Int2ObjectMap<ItemStack> slots) (dispatch: the body for this class)
    fn get_colors(&self) -> fn(&compat::Shared<dyn ContainerSolver>, &mut std::collections::HashMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> Vec<ColorHighlight> {
        CopyUnderbidPrice::get_colors_dyn
    }

}

impl Resettable for CopyUnderbidPrice {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void reset() (dispatch: the body for this class)
    fn reset(&self) -> fn(&compat::Shared<dyn Resettable>) {
        CopyUnderbidPrice::reset_dyn
    }

}

impl CopyUnderbidPrice {
    /// java: @Override public boolean isEnabled()
    pub fn is_enabled_impl(__this: &compat::Shared<Self>) -> bool {
        return SkyblockerConfigManager::get().helpers.enable_copy_underbid_price;
    }

    /// CopyUnderbidPrice.isEnabled as the dispatch of ContainerMatcher.isEnabled
    pub fn is_enabled_dyn(__this: &compat::Shared<dyn ContainerMatcher>) -> bool {
        Self::is_enabled_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public void start(AbstractContainerScreen<?> screen)
    pub fn start_impl(__this: &compat::Shared<Self>, screen: &AbstractContainerScreen) {
        { let __v = false; compat::write(__this, |__t| __t.copied = __v); }
        { let __v = mcrust::net::minecraft::world::item::item_stack::empty(); compat::write(__this, |__t| __t.previous_item = __v); }
    }

    /// CopyUnderbidPrice.start as the dispatch of ContainerSolver.start
    pub fn start_dyn(__this: &compat::Shared<dyn ContainerSolver>, __p0: &AbstractContainerScreen) {
        Self::start_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public void markDirty()
    pub fn mark_dirty_impl(__this: &compat::Shared<Self>) {
        let client = compat::minecraft();
        // java: if (!(client.screen instanceof ContainerScreen screen)) return; - the port names the open screen by its kind
        if !matches!(client.gui.screen_kind, mcrust::net::minecraft::client::gui::gui::ScreenKind::Container) {
            return;
        }
        // java: ItemStack stack = screen.getMenu().getSlot(13).getItem(); - the screen's menu is the player's open menu
        let mut stack: mcrust::net::minecraft::world::item::item_stack::ItemStack = compat::with_open_menu(|menu, containers| menu.slots[13].get_item(containers).duplicate()).unwrap_or_else(mcrust::net::minecraft::world::item::item_stack::empty);
        if stack.is_empty() {
            { let __v = false; compat::write(__this, |__t| __t.copied = __v); }
            { let __v = mcrust::net::minecraft::world::item::item_stack::empty(); compat::write(__this, |__t| __t.previous_item = __v); }
            return;
        }
        if !(item_stack::matches(&stack, &compat::read(__this, |__t| __t.previous_item.clone()))) {
            { let __v = false; compat::write(__this, |__t| __t.copied = __v); }
            { let __v = stack.copy(); compat::write(__this, |__t| __t.previous_item = __v); }
        }
        if compat::read(__this, |__t| __t.copied) {
            return;
        }
        let mut price: f64 = ItemUtils::get_item_price(&stack).unwrap_or(((0) as f64) as f64);
        if price <= (1 as f64) {
            return;
        }
        let mut underbid: i64 = ((price) as i64).wrapping_sub(1 as i64);
        // java: client.keyboardHandler.setClipboard(String.valueOf(underbid));
        {
            let mut __memory = mcrust::net::minecraft::client::gui::components::edit_box::MemoryClipboard::default();
            match mcrust::net::minecraft::client::minecraft::GlfwClipboard::published() {
                Some(mut __glfw) => client.keyboard_handler.set_clipboard(&mut __glfw, &underbid.to_string()),
                None => client.keyboard_handler.set_clipboard(&mut __memory, &underbid.to_string()),
            }
        }
        if mcrust::mcrust::modding::game::with_player(|_| ()).is_some() {
            let mut price_text: MutableComponent = (component::literal(&formatters::INTEGER_NUMBERS.get_mut().format((underbid) as f64))).with_style_format(ChatFormatting::Gold);
            mcrust::mcrust::modding::game::with_player(|player| { script_host::send_feedback(&(constants::PREFIX.get_mut())().append((component::translatable_args("skyblocker.copyUnderbidPrice.copied", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(price_text.clone())])).with_style_format(ChatFormatting::Gray)).to_string()); });
        }
        { let __v = true; compat::write(__this, |__t| __t.copied = __v); }
    }

    /// CopyUnderbidPrice.markDirty as the dispatch of ContainerSolver.markDirty
    pub fn mark_dirty_dyn(__this: &compat::Shared<dyn ContainerSolver>) {
        Self::mark_dirty_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public List<ColorHighlight> getColors(Int2ObjectMap<ItemStack> slots)
    pub fn get_colors_impl(__this: &compat::Shared<Self>, slots: &mut std::collections::HashMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> Vec<ColorHighlight> {
        return vec![];
    }

    /// CopyUnderbidPrice.getColors as the dispatch of ContainerSolver.getColors
    pub fn get_colors_dyn(__this: &compat::Shared<dyn ContainerSolver>, __p0: &mut std::collections::HashMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> Vec<ColorHighlight> {
        Self::get_colors_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public void reset()
    pub fn reset_impl(__this: &compat::Shared<Self>) {
        { let __v = false; compat::write(__this, |__t| __t.copied = __v); }
        { let __v = mcrust::net::minecraft::world::item::item_stack::empty(); compat::write(__this, |__t| __t.previous_item = __v); }
    }

    /// CopyUnderbidPrice.reset as the dispatch of Resettable.reset
    pub fn reset_dyn(__this: &compat::Shared<dyn Resettable>) {
        Self::reset_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}

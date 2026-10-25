//! Ported from de.hysky.skyblocker.skyblock.dungeon.terminal.LightsOnTerminal by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::dungeon::terminal::terminal_solver::TerminalSolver;
use crate::de::hysky::skyblocker::utils::container::container_matcher::ContainerMatcher;
use crate::de::hysky::skyblocker::utils::container::container_solver::ContainerSolver;
use crate::de::hysky::skyblocker::utils::container::regex_container_matcher::RegexContainerMatcher;
use crate::de::hysky::skyblocker::utils::container::regex_container_matcher::RegexContainerMatcherBase;
use crate::de::hysky::skyblocker::utils::container::simple_container_solver::SimpleContainerSolver;
use crate::de::hysky::skyblocker::utils::container::simple_container_solver::SimpleContainerSolverBase;
use crate::de::hysky::skyblocker::utils::render::gui::color_highlight::ColorHighlight;
use crate::de::hysky::skyblocker::utils::resettable::Resettable;

/// java: private static final List<ColorHighlight> EMPTY = List.of();
pub static EMPTY: LazyLock<Vec<ColorHighlight>> = LazyLock::new(|| vec![]);

/// java: public final class LightsOnTerminal extends SimpleContainerSolver implements TerminalSolver
#[derive(Clone)]
pub struct LightsOnTerminal {
    /// java: extends SimpleContainerSolver
    pub base: SimpleContainerSolverBase,
}

impl std::ops::Deref for LightsOnTerminal {
    type Target = SimpleContainerSolverBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for LightsOnTerminal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl LightsOnTerminal {
    /// java: public LightsOnTerminal()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = SimpleContainerSolverBase::new_in("^Correct all the panes!$".to_string(), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
            });
            ((__h.clone() as compat::Shared<dyn SimpleContainerSolver>), __h)
        });
        __this
    }

}

impl SimpleContainerSolver for LightsOnTerminal {
    fn simple_container_solver_base(&self) -> &SimpleContainerSolverBase {
        &self.base
    }
    fn simple_container_solver_base_mut(&mut self) -> &mut SimpleContainerSolverBase {
        &mut self.base
    }

}

impl RegexContainerMatcher for LightsOnTerminal {
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

impl ContainerMatcher for LightsOnTerminal {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public boolean isEnabled() (dispatch: the body for this class)
    fn is_enabled(&self) -> fn(&compat::Shared<dyn ContainerMatcher>) -> bool {
        LightsOnTerminal::is_enabled_dyn
    }

    /// java: @Override public boolean test(Screen screen) (dispatch: the body for this class)
    fn test(&self) -> fn(&compat::Shared<dyn ContainerMatcher>, &mcrust::net::minecraft::client::gui::screens::screen::Screen) -> bool {
        <dyn RegexContainerMatcher>::test_from::<Self>
    }

}

impl ContainerSolver for LightsOnTerminal {
    /// java: @Override public List<ColorHighlight> getColors(Int2ObjectMap<ItemStack> slots) (dispatch: the body for this class)
    fn get_colors(&self) -> fn(&compat::Shared<dyn ContainerSolver>, &mut std::collections::BTreeMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> Vec<ColorHighlight> {
        LightsOnTerminal::get_colors_dyn
    }

    /// java: @Override public boolean onClickSlot(int slot, ItemStack stack, int screenId, int button) (dispatch: the body for this class)
    fn on_click_slot(&self) -> fn(&compat::Shared<dyn ContainerSolver>, i32, &mcrust::net::minecraft::world::item::item_stack::ItemStack, i32, i32) -> bool {
        LightsOnTerminal::on_click_slot_dyn
    }

}

impl Resettable for LightsOnTerminal {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override default void reset() (dispatch: the body for this class)
    fn reset(&self) -> fn(&compat::Shared<dyn Resettable>) {
        <dyn ContainerSolver>::reset_from::<Self>
    }

}

impl TerminalSolver for LightsOnTerminal {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}

impl LightsOnTerminal {
    /// java: @Override public boolean isEnabled()
    pub fn is_enabled_impl(__this: &compat::Shared<Self>) -> bool {
        return ({ let __f = compat::dispatch(__this, |__t| __t.should_block_incorrect_clicks()); __f(&compat::view(__this, |a| a as compat::Shared<dyn TerminalSolver>)) });
    }

    /// LightsOnTerminal.isEnabled as the dispatch of ContainerMatcher.isEnabled
    pub fn is_enabled_dyn(__this: &compat::Shared<dyn ContainerMatcher>) -> bool {
        Self::is_enabled_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public List<ColorHighlight> getColors(Int2ObjectMap<ItemStack> slots)
    pub fn get_colors_impl(__this: &compat::Shared<Self>, slots: &mut std::collections::BTreeMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> Vec<ColorHighlight> {
        return EMPTY.clone();
    }

    /// LightsOnTerminal.getColors as the dispatch of ContainerSolver.getColors
    pub fn get_colors_dyn(__this: &compat::Shared<dyn ContainerSolver>, __p0: &mut std::collections::BTreeMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> Vec<ColorHighlight> {
        Self::get_colors_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public boolean onClickSlot(int slot, ItemStack stack, int screenId, int button)
    pub fn on_click_slot_impl(__this: &compat::Shared<Self>, mut slot: i32, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, mut screen_id: i32, mut button: i32) -> bool {
        return stack.is_item(mcrust::net::minecraft::world::item::items::stained_glass_pane().lime());
    }

    /// LightsOnTerminal.onClickSlot as the dispatch of ContainerSolver.onClickSlot
    pub fn on_click_slot_dyn(__this: &compat::Shared<dyn ContainerSolver>, __p0: i32, __p1: &mcrust::net::minecraft::world::item::item_stack::ItemStack, __p2: i32, __p3: i32) -> bool {
        Self::on_click_slot_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2, __p3)
    }

}

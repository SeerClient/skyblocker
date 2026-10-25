//! Ported from de.hysky.skyblocker.skyblock.dungeon.terminal.StartsWithTerminal by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::dungeon::terminal::terminal_solver::TerminalSolver;
use crate::de::hysky::skyblocker::utils::container::container_matcher::ContainerMatcher;
use crate::de::hysky::skyblocker::utils::container::container_solver;
use crate::de::hysky::skyblocker::utils::container::container_solver::ContainerSolver;
use crate::de::hysky::skyblocker::utils::container::regex_container_matcher::RegexContainerMatcher;
use crate::de::hysky::skyblocker::utils::container::regex_container_matcher::RegexContainerMatcherBase;
use crate::de::hysky::skyblocker::utils::container::simple_container_solver::SimpleContainerSolver;
use crate::de::hysky::skyblocker::utils::container::simple_container_solver::SimpleContainerSolverBase;
use crate::de::hysky::skyblocker::utils::container::stack_display_modifier::StackDisplayModifier;
use crate::de::hysky::skyblocker::utils::render::gui::color_highlight::ColorHighlight;
use crate::de::hysky::skyblocker::utils::resettable::Resettable;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: public final class StartsWithTerminal extends SimpleContainerSolver implements TerminalSolver, StackDisplayModifier
#[derive(Clone)]
pub struct StartsWithTerminal {
    /// java: extends SimpleContainerSolver
    pub base: SimpleContainerSolverBase,
    /// java: private List<Integer> clickedSlotIds = List.of();
    pub clicked_slot_ids: Vec<i32>,
}

impl std::ops::Deref for StartsWithTerminal {
    type Target = SimpleContainerSolverBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for StartsWithTerminal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl StartsWithTerminal {
    /// java: public StartsWithTerminal()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = SimpleContainerSolverBase::new_in("^What starts with: '([A-Z])'\\?$".to_string(), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                clicked_slot_ids: vec![],
            });
            ((__h.clone() as compat::Shared<dyn SimpleContainerSolver>), __h)
        });
        __this
    }

}

impl SimpleContainerSolver for StartsWithTerminal {
    fn simple_container_solver_base(&self) -> &SimpleContainerSolverBase {
        &self.base
    }
    fn simple_container_solver_base_mut(&mut self) -> &mut SimpleContainerSolverBase {
        &mut self.base
    }

}

impl RegexContainerMatcher for StartsWithTerminal {
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

impl ContainerMatcher for StartsWithTerminal {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public boolean isEnabled() (dispatch: the body for this class)
    fn is_enabled(&self) -> fn(&compat::Shared<dyn ContainerMatcher>) -> bool {
        StartsWithTerminal::is_enabled_dyn
    }

    /// java: @Override public boolean test(Screen screen) (dispatch: the body for this class)
    fn test(&self) -> fn(&compat::Shared<dyn ContainerMatcher>, &mcrust::net::minecraft::client::gui::screens::screen::Screen) -> bool {
        <dyn RegexContainerMatcher>::test_from::<Self>
    }

}

impl ContainerSolver for StartsWithTerminal {
    /// java: @Override public List<ColorHighlight> getColors(Int2ObjectMap<ItemStack> slots) (dispatch: the body for this class)
    fn get_colors(&self) -> fn(&compat::Shared<dyn ContainerSolver>, &mut std::collections::BTreeMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> Vec<ColorHighlight> {
        StartsWithTerminal::get_colors_dyn
    }

    /// java: @Override public boolean onClickSlot(int slot, ItemStack stack, int screenId, int button) (dispatch: the body for this class)
    fn on_click_slot(&self) -> fn(&compat::Shared<dyn ContainerSolver>, i32, &mcrust::net::minecraft::world::item::item_stack::ItemStack, i32, i32) -> bool {
        StartsWithTerminal::on_click_slot_dyn
    }

}

impl Resettable for StartsWithTerminal {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override default void reset() (dispatch: the body for this class)
    fn reset(&self) -> fn(&compat::Shared<dyn Resettable>) {
        <dyn ContainerSolver>::reset_from::<Self>
    }

}

impl TerminalSolver for StartsWithTerminal {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}

impl StackDisplayModifier for StartsWithTerminal {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public ItemStack modifyDisplayStack(int slotIndex, ItemStack stack) (dispatch: the body for this class)
    fn modify_display_stack(&self) -> fn(&compat::Shared<dyn StackDisplayModifier>, i32, &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> mcrust::net::minecraft::world::item::item_stack::ItemStack {
        StartsWithTerminal::modify_display_stack_dyn
    }

}

impl StartsWithTerminal {
    /// java: @Override public boolean isEnabled()
    pub fn is_enabled_impl(__this: &compat::Shared<Self>) -> bool {
        { let __v = Vec::new(); compat::write(__this, |__t| __t.clicked_slot_ids = __v); }
        return SkyblockerConfigManager::get().dungeons.terminals.solve_starts_with;
    }

    /// StartsWithTerminal.isEnabled as the dispatch of ContainerMatcher.isEnabled
    pub fn is_enabled_dyn(__this: &compat::Shared<dyn ContainerMatcher>) -> bool {
        Self::is_enabled_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public List<ColorHighlight> getColors(Int2ObjectMap<ItemStack> slots)
    pub fn get_colors_impl(__this: &compat::Shared<Self>, slots: &mut std::collections::BTreeMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> Vec<ColorHighlight> {
        container_solver::trim_edges(slots, 6);
        let mut prefix: String = __this.lock().borrow().groups.as_ref().unwrap()[(0) as usize].clone();
        let mut highlights: Vec<ColorHighlight> = Vec::new();
        for slot in slots.iter().map(|(k, v)| (k.clone(), v.clone())) {
            let stack = &slot.1;
            let mut clicked: bool = __this.lock().borrow().clicked_slot_ids.iter().any(|e| *e == slot.0);
            if !clicked && compat::component_of(stack.get_hover_name()).get_string().starts_with(&prefix) {
                highlights.push(ColorHighlight::green(slot.0));
            }
        }
        return highlights;
    }

    /// StartsWithTerminal.getColors as the dispatch of ContainerSolver.getColors
    pub fn get_colors_dyn(__this: &compat::Shared<dyn ContainerSolver>, __p0: &mut std::collections::BTreeMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> Vec<ColorHighlight> {
        Self::get_colors_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public boolean onClickSlot(int slot, ItemStack stack, int screenId, int button)
    pub fn on_click_slot_impl(__this: &compat::Shared<Self>, mut slot: i32, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, mut screen_id: i32, mut button: i32) -> bool {
        let mut clicked: bool = __this.lock().borrow().clicked_slot_ids.iter().any(|e| *e == slot);
        if clicked || false /* java: stack == null: never null here */ || stack.is_empty() {
            return ({ let __f = compat::dispatch(__this, |__t| __t.should_block_incorrect_clicks()); __f(&compat::view(__this, |a| a as compat::Shared<dyn TerminalSolver>)) });
        }
        let mut prefix: String = __this.lock().borrow().groups.as_ref().unwrap()[(0) as usize].clone();
        if !(compat::component_of(stack.get_hover_name()).get_string().starts_with(&prefix)) {
            return ({ let __f = compat::dispatch(__this, |__t| __t.should_block_incorrect_clicks()); __f(&compat::view(__this, |a| a as compat::Shared<dyn TerminalSolver>)) });
        } else {
            __this.lock().borrow_mut().clicked_slot_ids.push(slot);
        }
        return false;
    }

    /// StartsWithTerminal.onClickSlot as the dispatch of ContainerSolver.onClickSlot
    pub fn on_click_slot_dyn(__this: &compat::Shared<dyn ContainerSolver>, __p0: i32, __p1: &mcrust::net::minecraft::world::item::item_stack::ItemStack, __p2: i32, __p3: i32) -> bool {
        Self::on_click_slot_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2, __p3)
    }

    /// java: @Override public ItemStack modifyDisplayStack(int slotIndex, ItemStack stack)
    pub fn modify_display_stack_impl(__this: &compat::Shared<Self>, mut slot_index: i32, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> mcrust::net::minecraft::world::item::item_stack::ItemStack {
        return (if slot_index >= 45 || compat::component_of(stack.get_hover_name()).get_string().starts_with(&__this.lock().borrow().groups.as_ref().unwrap()[(0) as usize]) { stack.clone() } else { mcrust::net::minecraft::world::item::item_stack::empty() });
    }

    /// StartsWithTerminal.modifyDisplayStack as the dispatch of StackDisplayModifier.modifyDisplayStack
    pub fn modify_display_stack_dyn(__this: &compat::Shared<dyn StackDisplayModifier>, __p0: i32, __p1: &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> mcrust::net::minecraft::world::item::item_stack::ItemStack {
        Self::modify_display_stack_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1)
    }

}

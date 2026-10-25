//! Ported from de.hysky.skyblocker.skyblock.experiment.SuperpairsSolver by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::configs::helper_config::Experiments;
use crate::de::hysky::skyblocker::skyblock::experiment::experiment_solver::ExperimentSolver;
use crate::de::hysky::skyblocker::skyblock::experiment::experiment_solver::ExperimentSolverBase;
use crate::de::hysky::skyblocker::skyblock::experiment::experiment_solver::State;
use crate::de::hysky::skyblocker::utils::container::container_matcher::ContainerMatcher;
use crate::de::hysky::skyblocker::utils::container::container_solver::ContainerSolver;
use crate::de::hysky::skyblocker::utils::container::regex_container_matcher::RegexContainerMatcher;
use crate::de::hysky::skyblocker::utils::container::regex_container_matcher::RegexContainerMatcherBase;
use crate::de::hysky::skyblocker::utils::container::simple_container_solver::SimpleContainerSolver;
use crate::de::hysky::skyblocker::utils::container::simple_container_solver::SimpleContainerSolverBase;
use crate::de::hysky::skyblocker::utils::container::stack_display_modifier::StackDisplayModifier;
use crate::de::hysky::skyblocker::utils::render::gui::color_highlight::ColorHighlight;
use crate::de::hysky::skyblocker::utils::resettable::Resettable;
use mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::AbstractContainerScreen;
use mcrust::net::minecraft::world::item::item_stack;
use mcrust::net::minecraft::world::item::items;
use mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::ContainerScreenHost as _;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: public final class SuperpairsSolver extends ExperimentSolver
#[derive(Clone)]
pub struct SuperpairsSolver {
    /// java: extends ExperimentSolver
    pub base: ExperimentSolverBase,
    /// java: private int superpairsPrevClickedSlot = 0;
    pub superpairs_prev_clicked_slot: i32,
    /// java: private ItemStack superpairsCurrentSlot = ItemStack.EMPTY;
    pub superpairs_current_slot: mcrust::net::minecraft::world::item::item_stack::ItemStack,
    /// java: private final IntSet superpairsDuplicatedSlots = new IntArraySet(28);
    pub superpairs_duplicated_slots: std::collections::HashSet<i32>,
}

impl std::ops::Deref for SuperpairsSolver {
    type Target = ExperimentSolverBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for SuperpairsSolver {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl SuperpairsSolver {
    /// java: public SuperpairsSolver()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = ExperimentSolverBase::new_in("^Superpairs \\(\\w+\\)$".to_string(), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                superpairs_prev_clicked_slot: 0,
                superpairs_current_slot: mcrust::net::minecraft::world::item::item_stack::empty(),
                superpairs_duplicated_slots: Default::default(),
            });
            ((__h.clone() as compat::Shared<dyn ExperimentSolver>), __h)
        });
        __this
    }

}

impl ExperimentSolver for SuperpairsSolver {
    fn experiment_solver_base(&self) -> &ExperimentSolverBase {
        &self.base
    }
    fn experiment_solver_base_mut(&mut self) -> &mut ExperimentSolverBase {
        &mut self.base
    }

    /// java: @Override protected boolean isEnabled(HelperConfig.Experiments experimentsConfig) (dispatch: the body for this class)
    fn is_enabled_1(&self) -> fn(&compat::Shared<dyn ExperimentSolver>, &Experiments) -> bool {
        SuperpairsSolver::is_enabled_1_dyn
    }

    /// java: @Override protected void tick(AbstractContainerScreen<?> screen) (dispatch: the body for this class)
    fn tick(&self) -> fn(&compat::Shared<dyn ExperimentSolver>, &AbstractContainerScreen) {
        SuperpairsSolver::tick_dyn
    }

}

impl SimpleContainerSolver for SuperpairsSolver {
    fn simple_container_solver_base(&self) -> &SimpleContainerSolverBase {
        &self.base.base
    }
    fn simple_container_solver_base_mut(&mut self) -> &mut SimpleContainerSolverBase {
        &mut self.base.base
    }

}

impl RegexContainerMatcher for SuperpairsSolver {
    fn regex_container_matcher_base(&self) -> &RegexContainerMatcherBase {
        &self.base.base.base
    }
    fn regex_container_matcher_base_mut(&mut self) -> &mut RegexContainerMatcherBase {
        &mut self.base.base.base
    }

    /// java: @Override public Pattern getTitlePattern() (dispatch: the body for this class)
    fn get_title_pattern(&self) -> fn(&compat::Shared<dyn RegexContainerMatcher>) -> Option<compat::regex::Pattern> {
        <dyn SimpleContainerSolver>::get_title_pattern_from::<Self>
    }

}

impl ContainerMatcher for SuperpairsSolver {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public boolean test(Screen screen) (dispatch: the body for this class)
    fn test(&self) -> fn(&compat::Shared<dyn ContainerMatcher>, &mcrust::net::minecraft::client::gui::screens::screen::Screen) -> bool {
        <dyn RegexContainerMatcher>::test_from::<Self>
    }

    /// java: @Override public final boolean isEnabled() (dispatch: the body for this class)
    fn is_enabled(&self) -> fn(&compat::Shared<dyn ContainerMatcher>) -> bool {
        <dyn ExperimentSolver>::is_enabled_from::<Self>
    }

}

impl ContainerSolver for SuperpairsSolver {
    /// java: @Override public void start(AbstractContainerScreen<?> screen) (dispatch: the body for this class)
    fn start(&self) -> fn(&compat::Shared<dyn ContainerSolver>, &AbstractContainerScreen) {
        SuperpairsSolver::start_dyn
    }

    /// java: @Override public List<ColorHighlight> getColors(Int2ObjectMap<ItemStack> displaySlots) (dispatch: the body for this class)
    fn get_colors(&self) -> fn(&compat::Shared<dyn ContainerSolver>, &mut std::collections::BTreeMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> Vec<ColorHighlight> {
        SuperpairsSolver::get_colors_dyn
    }

    /// java: @Override public boolean onClickSlot(int slot, ItemStack stack, int screenId, int button) (dispatch: the body for this class)
    fn on_click_slot(&self) -> fn(&compat::Shared<dyn ContainerSolver>, i32, &mcrust::net::minecraft::world::item::item_stack::ItemStack, i32, i32) -> bool {
        SuperpairsSolver::on_click_slot_dyn
    }

}

impl Resettable for SuperpairsSolver {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void reset() (dispatch: the body for this class)
    fn reset(&self) -> fn(&compat::Shared<dyn Resettable>) {
        SuperpairsSolver::reset_dyn
    }

}

impl StackDisplayModifier for SuperpairsSolver {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public ItemStack modifyDisplayStack(int slotIndex, ItemStack stack) (dispatch: the body for this class)
    fn modify_display_stack(&self) -> fn(&compat::Shared<dyn StackDisplayModifier>, i32, &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> mcrust::net::minecraft::world::item::item_stack::ItemStack {
        <dyn ExperimentSolver>::modify_display_stack_from::<Self>
    }

}

impl SuperpairsSolver {
    /// java: @Override protected boolean isEnabled(HelperConfig.Experiments experimentsConfig)
    pub fn is_enabled_1_impl(__this: &compat::Shared<Self>, experiments_config: &Experiments) -> bool {
        return experiments_config.enable_superpairs_solver;
    }

    /// SuperpairsSolver.isEnabled as the dispatch of ExperimentSolver.isEnabled
    pub fn is_enabled_1_dyn(__this: &compat::Shared<dyn ExperimentSolver>, __p0: &Experiments) -> bool {
        Self::is_enabled_1_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override protected void tick(AbstractContainerScreen<?> screen)
    pub fn tick_impl(__this: &compat::Shared<Self>, screen: &AbstractContainerScreen) {
        if ({ let __f = compat::dispatch(__this, |__t| __t.get_state()); __f(&compat::view(__this, |a| a as compat::Shared<dyn ExperimentSolver>)) }) == State::Show && (({ let __f = compat::dispatch(__this, |__t| __t.get_slots()); __f(&compat::view(__this, |a| a as compat::Shared<dyn ExperimentSolver>)) })).get(&compat::read(__this, |__t| __t.superpairs_prev_clicked_slot)).cloned().is_none() {
            //   java: ItemStack itemStack = screen.getMenu().getSlot(superpairsPrevClickedSlot).getItem();
            // (`screen.getMenu()` is the player's open menu; its slots read their stacks through the menu's containers)
            let __index: i32 = compat::read(__this, |__t| __t.superpairs_prev_clicked_slot);
            let _ = screen;
            let mut item_stack: mcrust::net::minecraft::world::item::item_stack::ItemStack = compat::with_open_menu(|__menu, __containers| __menu.get_slot(__index).get_item(__containers).duplicate()).unwrap_or_else(mcrust::net::minecraft::world::item::item_stack::empty);
            if !(item_stack.is_item(mcrust::net::minecraft::world::item::items::stained_glass().cyan()) || item_stack.is_item(mcrust::net::minecraft::world::item::items::stained_glass_pane().black()) || item_stack.is_item(&items::items().get("air"))) {
                if let Some(entry) = (({ let __f = compat::dispatch(__this, |__t| __t.get_slots()); __f(&compat::view(__this, |a| a as compat::Shared<dyn ExperimentSolver>)) })).iter().map(|(k, v)| (k.clone(), v.clone())).filter(|entry| item_stack::matches(&entry.1, &item_stack)).next() {
                    __this.lock().borrow_mut().superpairs_duplicated_slots.insert(entry.0);
                    let __h0 = compat::read(__this, |__t| __t.superpairs_prev_clicked_slot);
                    __this.lock().borrow_mut().superpairs_duplicated_slots.insert(__h0);
                }
                // java: getSlots().put(..) - `getSlots()` answers the solver's own map (the port's getter answers a copy)
                { let __k = compat::read(__this, |__t| __t.superpairs_prev_clicked_slot); let __v = item_stack.clone(); compat::write(__this, |__t| __t.experiment_solver_base_mut().slots.insert(__k, __v)); }
                { let __v = item_stack.clone(); compat::write(__this, |__t| __t.superpairs_current_slot = __v); }
            }
        }
    }

    /// SuperpairsSolver.tick as the dispatch of ExperimentSolver.tick
    pub fn tick_dyn(__this: &compat::Shared<dyn ExperimentSolver>, __p0: &AbstractContainerScreen) {
        Self::tick_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public void start(AbstractContainerScreen<?> screen)
    pub fn start_impl(__this: &compat::Shared<Self>, screen: &AbstractContainerScreen) {
        <dyn ExperimentSolver>::start_body(&compat::view(__this, |a| a as compat::Shared<dyn ExperimentSolver>), screen);
        ({ let __f = compat::dispatch(__this, |__t| __t.set_state()); __f(&compat::view(__this, |a| a as compat::Shared<dyn ExperimentSolver>), State::Show) });
    }

    /// SuperpairsSolver.start as the dispatch of ContainerSolver.start
    pub fn start_dyn(__this: &compat::Shared<dyn ContainerSolver>, __p0: &AbstractContainerScreen) {
        Self::start_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public List<ColorHighlight> getColors(Int2ObjectMap<ItemStack> displaySlots)
    pub fn get_colors_impl(__this: &compat::Shared<Self>, display_slots: &mut std::collections::BTreeMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> Vec<ColorHighlight> {
        let mut highlights: Vec<ColorHighlight> = Vec::new();
        if ({ let __f = compat::dispatch(__this, |__t| __t.get_state()); __f(&compat::view(__this, |a| a as compat::Shared<dyn ExperimentSolver>)) }) == State::Show {
            for index_stack in display_slots.iter().map(|(k, v)| (k.clone(), v.clone())) {
                let mut index: i32 = index_stack.0;
                let display_stack = &index_stack.1;
                let mut stack: Option<mcrust::net::minecraft::world::item::item_stack::ItemStack> = (({ let __f = compat::dispatch(__this, |__t| __t.get_slots()); __f(&compat::view(__this, |a| a as compat::Shared<dyn ExperimentSolver>)) })).get(&index).cloned();
                if stack.is_some() && !(item_stack::matches(stack.as_ref().unwrap(), display_stack)) {
                    if item_stack::matches(&compat::read(__this, |__t| __t.superpairs_current_slot.clone()), stack.as_ref().unwrap()) && compat::component_of(display_stack.get_hover_name()).get_string() == "Click a second button!" {
                        highlights.push(ColorHighlight::green(index));
                    } else if __this.lock().borrow().superpairs_duplicated_slots.contains(&index) {
                        highlights.push(ColorHighlight::yellow(index));
                    } else {
                        highlights.push(ColorHighlight::red(index));
                    }
                }
            }
        }
        return highlights;
    }

    /// SuperpairsSolver.getColors as the dispatch of ContainerSolver.getColors
    pub fn get_colors_dyn(__this: &compat::Shared<dyn ContainerSolver>, __p0: &mut std::collections::BTreeMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> Vec<ColorHighlight> {
        Self::get_colors_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public boolean onClickSlot(int slot, ItemStack stack, int screenId, int button)
    pub fn on_click_slot_impl(__this: &compat::Shared<Self>, mut slot: i32, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, mut screen_id: i32, mut button: i32) -> bool {
        if ({ let __f = compat::dispatch(__this, |__t| __t.get_state()); __f(&compat::view(__this, |a| a as compat::Shared<dyn ExperimentSolver>)) }) == State::Show {
            { let __v = slot; compat::write(__this, |__t| __t.superpairs_prev_clicked_slot = __v); }
            { let __v = mcrust::net::minecraft::world::item::item_stack::empty(); compat::write(__this, |__t| __t.superpairs_current_slot = __v); }
        }
        return <dyn ContainerSolver>::on_click_slot_body(&compat::view(__this, |a| a as compat::Shared<dyn ContainerSolver>), slot, stack, screen_id, button);
    }

    /// SuperpairsSolver.onClickSlot as the dispatch of ContainerSolver.onClickSlot
    pub fn on_click_slot_dyn(__this: &compat::Shared<dyn ContainerSolver>, __p0: i32, __p1: &mcrust::net::minecraft::world::item::item_stack::ItemStack, __p2: i32, __p3: i32) -> bool {
        Self::on_click_slot_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2, __p3)
    }

    /// java: @Override public void reset()
    pub fn reset_impl(__this: &compat::Shared<Self>) {
        { let __v = 0; compat::write(__this, |__t| __t.superpairs_prev_clicked_slot = __v); }
        { let __v = mcrust::net::minecraft::world::item::item_stack::empty(); compat::write(__this, |__t| __t.superpairs_current_slot = __v); }
        __this.lock().borrow_mut().superpairs_duplicated_slots.clear();
        <dyn ExperimentSolver>::reset_body(&compat::view(__this, |a| a as compat::Shared<dyn ExperimentSolver>));
    }

    /// SuperpairsSolver.reset as the dispatch of Resettable.reset
    pub fn reset_dyn(__this: &compat::Shared<dyn Resettable>) {
        Self::reset_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}

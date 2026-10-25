//! Ported from de.hysky.skyblocker.skyblock.experiment.ExperimentSolver by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::configs::helper_config::Experiments;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::experiment::superpairs_solver::SuperpairsSolver;
use crate::de::hysky::skyblocker::skyblock::experiment::ultrasequencer_solver::UltrasequencerSolver;
use crate::de::hysky::skyblocker::utils::container::container_matcher::ContainerMatcher;
use crate::de::hysky::skyblocker::utils::container::container_solver::ContainerSolver;
use crate::de::hysky::skyblocker::utils::container::simple_container_solver::SimpleContainerSolver;
use crate::de::hysky::skyblocker::utils::container::simple_container_solver::SimpleContainerSolverBase;
use crate::de::hysky::skyblocker::utils::container::stack_display_modifier::StackDisplayModifier;
use crate::de::hysky::skyblocker::utils::resettable::Resettable;
use mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::AbstractContainerScreen;

/// java: public abstract sealed class ExperimentSolver extends SimpleContainerSolver implements StackDisplayModifier permits ChronomatronSolver, SuperpairsSolver, UltrasequencerSolver
#[derive(Clone)]
pub struct ExperimentSolverBase {
    /// java: extends SimpleContainerSolver
    pub base: SimpleContainerSolverBase,
    /// java: private State state = State.REMEMBER;
    pub state: State,
    /// java: private final Int2ObjectMap<ItemStack> slots = new Int2ObjectOpenHashMap<>();
    pub slots: std::collections::BTreeMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack>,
}

impl std::ops::Deref for ExperimentSolverBase {
    type Target = SimpleContainerSolverBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for ExperimentSolverBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl ExperimentSolverBase {
    /// java: protected ExperimentSolver(@Language("RegExp") String containerName) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_in<__R>(mut container_name: String, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn ExperimentSolver>, __R)) -> (compat::Shared<dyn ExperimentSolver>, __R) {
        let (_, (__this, __r)) = SimpleContainerSolverBase::new_in(container_name.clone(), move |__base| {
            let this = Self {
                base: __base,
                state: State::Remember,
                slots: std::collections::BTreeMap::new(),
            };
            let (__h, __r) = __wrap(this);
            ((__h.clone() as compat::Shared<dyn SimpleContainerSolver>), (__h, __r))
        });
        (__this, __r)
    }

}

/// java: the virtual methods of ExperimentSolver (implemented by ExperimentSolverBase and every subclass)
pub trait ExperimentSolver: SimpleContainerSolver + StackDisplayModifier {
    fn experiment_solver_base(&self) -> &ExperimentSolverBase;
    fn experiment_solver_base_mut(&mut self) -> &mut ExperimentSolverBase;

    /// java: public State getState() (dispatch: the body for this class)
    fn get_state(&self) -> fn(&compat::Shared<dyn ExperimentSolver>) -> State {
        <dyn ExperimentSolver>::get_state_body
    }

    /// java: public void setState(State state) (dispatch: the body for this class)
    fn set_state(&self) -> fn(&compat::Shared<dyn ExperimentSolver>, State) {
        <dyn ExperimentSolver>::set_state_body
    }

    /// java: public Int2ObjectMap<ItemStack> getSlots() (dispatch: the body for this class)
    fn get_slots(&self) -> fn(&compat::Shared<dyn ExperimentSolver>) -> std::collections::BTreeMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack> {
        <dyn ExperimentSolver>::get_slots_body
    }

    /// java: protected abstract boolean isEnabled(HelperConfig.Experiments experimentsConfig); (dispatch: the body for this class)
    fn is_enabled_1(&self) -> fn(&compat::Shared<dyn ExperimentSolver>, &Experiments) -> bool;

    /// java: protected boolean shouldBlockIncorrectClicks() (dispatch: the body for this class)
    fn should_block_incorrect_clicks(&self) -> fn(&compat::Shared<dyn ExperimentSolver>) -> bool {
        <dyn ExperimentSolver>::should_block_incorrect_clicks_body
    }

    /// java: protected abstract void tick(AbstractContainerScreen<?> screen); (dispatch: the body for this class)
    fn tick(&self) -> fn(&compat::Shared<dyn ExperimentSolver>, &AbstractContainerScreen);

}

impl dyn ExperimentSolver {
    /// java: public State getState()
    pub fn get_state_body(__this: &compat::Shared<dyn ExperimentSolver>) -> State {
        return compat::read(__this, |__t| __t.experiment_solver_base().state);
    }

    /// java: public void setState(State state)
    pub fn set_state_body(__this: &compat::Shared<dyn ExperimentSolver>, mut state: State) {
        { let __v = state; compat::write(__this, |__t| __t.experiment_solver_base_mut().state = __v); }
    }

    /// java: public Int2ObjectMap<ItemStack> getSlots()
    pub fn get_slots_body(__this: &compat::Shared<dyn ExperimentSolver>) -> std::collections::BTreeMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack> {
        return compat::read(__this, |__t| __t.experiment_solver_base().slots.clone());
    }

    /// java: @Override public final boolean isEnabled()
    pub fn is_enabled_body(__this: &compat::Shared<dyn ExperimentSolver>) -> bool {
        return ({ let __f = compat::dispatch(__this, |__t| __t.is_enabled_1()); __f(__this, &SkyblockerConfigManager::get().helpers.experiments) });
    }

    /// ExperimentSolver.isEnabled as the dispatch of ContainerMatcher.isEnabled for the concrete class `__C`
    pub fn is_enabled_from<__C: ExperimentSolver + 'static>(__this: &compat::Shared<dyn ContainerMatcher>) -> bool {
        <dyn ExperimentSolver>::is_enabled_body(&compat::view_as::<__C, _, _>(__this, |a| a as compat::Shared<dyn ExperimentSolver>))
    }

    /// java: protected boolean shouldBlockIncorrectClicks()
    pub fn should_block_incorrect_clicks_body(__this: &compat::Shared<dyn ExperimentSolver>) -> bool {
        return SkyblockerConfigManager::get().helpers.experiments.block_incorrect_clicks;
    }

    /// java: @Override public void start(AbstractContainerScreen<?> screen)
    pub fn start_body(__this: &compat::Shared<dyn ExperimentSolver>, screen: &AbstractContainerScreen) {
        { let __v = State::Remember; compat::write(__this, |__t| __t.experiment_solver_base_mut().state = __v); }
        //   java: ScreenEvents.afterTick(screen).register(_ -> tick(screen));
        // (Fabric's per-screen `afterTick` is the end of `Screen.tick`, whose last statement for a container screen is
        // `containerTick()`: the listener is this solver, run by `install_after_tick`'s RETURN injector there, for the
        // screen it was started on - the solver is reset when that screen is removed, which ends the listener as the
        // screen's own listener list ends with it. As `AccessoriesHelper` does.)
        let _ = screen;
        <dyn ExperimentSolver>::install_after_tick();
        AFTER_TICK_SOLVER.with(|s| *s.borrow_mut() = Some(__this.clone()));
    }

    /// The `containerTick` RETURN injector that runs the started solver's `tick(screen)` (see `start_body`).
    fn install_after_tick() {
        static HOOKED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
        if HOOKED.swap(true, std::sync::atomic::Ordering::AcqRel) {
            return;
        }
        let result = mcrust::mcrust::modding::hook::inject(
            "net.minecraft.client.gui.screens.inventory.AbstractContainerScreen#containerTick",
            mcrust::mcrust::modding::hook::At::Return,
            |ci: &mut mcrust::mcrust::modding::hook::CallbackInfo<'_>| {
                let Some(solver) = AFTER_TICK_SOLVER.with(|s| s.borrow().clone()) else {
                    return;
                };
                let Some(screen) = ci.this::<AbstractContainerScreen>() else {
                    return;
                };
                // java: _ -> tick(screen)
                let __f = compat::dispatch(&solver, |__t| __t.tick());
                __f(&solver, screen);
            },
        );
        if let Err(e) = result {
            mcrust::mcrust::modding::log::warn_once("skyblocker-experiment-after-tick", &format!("{e:?}"));
        }
    }

    /// ExperimentSolver.start as the dispatch of ContainerSolver.start for the concrete class `__C`
    pub fn start_from<__C: ExperimentSolver + 'static>(__this: &compat::Shared<dyn ContainerSolver>, __p0: &AbstractContainerScreen) {
        <dyn ExperimentSolver>::start_body(&compat::view_as::<__C, _, _>(__this, |a| a as compat::Shared<dyn ExperimentSolver>), __p0)
    }

    /// java: @Override public void reset()
    pub fn reset_body(__this: &compat::Shared<dyn ExperimentSolver>) {
        // the screen this solver's `afterTick` listener belonged to is gone (see `start_body`)
        AFTER_TICK_SOLVER.with(|s| {
            let mut s = s.borrow_mut();
            if s.as_ref().is_some_and(|started| std::sync::Arc::ptr_eq(started, __this)) {
                *s = None;
            }
        });
        { let __v = State::Remember; compat::write(__this, |__t| __t.experiment_solver_base_mut().state = __v); }
        __this.lock().borrow_mut().experiment_solver_base_mut().slots.clear();
    }

    /// ExperimentSolver.reset as the dispatch of Resettable.reset for the concrete class `__C`
    pub fn reset_from<__C: ExperimentSolver + 'static>(__this: &compat::Shared<dyn Resettable>) {
        <dyn ExperimentSolver>::reset_body(&compat::view_as::<__C, _, _>(__this, |a| a as compat::Shared<dyn ExperimentSolver>))
    }

    /// java: @Override public ItemStack modifyDisplayStack(int slotIndex, ItemStack stack)
    pub fn modify_display_stack_body(__this: &compat::Shared<dyn ExperimentSolver>, mut slot_index: i32, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> mcrust::net::minecraft::world::item::item_stack::ItemStack {
        if (crate::de::hysky::skyblocker::utils::container::container_matcher::ContainerMatcher::as_any(&*__this.lock().borrow()).is::<SuperpairsSolver>() || crate::de::hysky::skyblocker::utils::container::container_matcher::ContainerMatcher::as_any(&*__this.lock().borrow()).is::<UltrasequencerSolver>()) && ({ let __f = compat::dispatch(__this, |__t| __t.get_state()); __f(__this) }) == State::Show {
            let mut display_stack: Option<mcrust::net::minecraft::world::item::item_stack::ItemStack> = ({ let __f = compat::dispatch(__this, |__t| __t.get_slots()); __f(__this) }).get(&slot_index).cloned();
            return (if display_stack.is_some() { display_stack.clone() } else { Some(stack.clone()) }).unwrap();
        }
        return stack.clone();
    }

    /// ExperimentSolver.modifyDisplayStack as the dispatch of StackDisplayModifier.modifyDisplayStack for the concrete class `__C`
    pub fn modify_display_stack_from<__C: ExperimentSolver + 'static>(__this: &compat::Shared<dyn StackDisplayModifier>, __p0: i32, __p1: &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> mcrust::net::minecraft::world::item::item_stack::ItemStack {
        <dyn ExperimentSolver>::modify_display_stack_body(&compat::view_as::<__C, _, _>(__this, |a| a as compat::Shared<dyn ExperimentSolver>), __p0, __p1)
    }

}

// ExperimentSolver.isEnabled implements ContainerMatcher.isEnabled: it is emitted in each concrete subclass's `impl ContainerMatcher` (Rust has no inherited impls)
// ExperimentSolver.start implements ContainerSolver.start: it is emitted in each concrete subclass's `impl ContainerSolver` (Rust has no inherited impls)
// ExperimentSolver.reset implements Resettable.reset: it is emitted in each concrete subclass's `impl Resettable` (Rust has no inherited impls)
// ExperimentSolver.modifyDisplayStack implements StackDisplayModifier.modifyDisplayStack: it is emitted in each concrete subclass's `impl StackDisplayModifier` (Rust has no inherited impls)
/// java: public enum State
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum State {
    Remember,
    Wait,
    Show,
    End,
}

impl State {
    pub const VALUES: [State; 4] = [State::Remember, State::Wait, State::Show, State::End];

    /// java: values()
    pub fn values() -> Vec<State> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            State::Remember => "REMEMBER",
            State::Wait => "WAIT",
            State::Show => "SHOW",
            State::End => "END",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> State {
        match name {
            "REMEMBER" => State::Remember,
            "WAIT" => State::Wait,
            "SHOW" => State::Show,
            "END" => State::End,
            _ => panic!("IllegalArgumentException: No enum constant State.{name}"),
        }
    }

}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

thread_local! {
    /// The solver whose `ScreenEvents.afterTick(screen)` listener is registered (`start`), until its `reset`.
    static AFTER_TICK_SOLVER: std::cell::RefCell<Option<compat::Shared<dyn ExperimentSolver>>> = const { std::cell::RefCell::new(None) };
}

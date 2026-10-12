//! Ported from de.hysky.skyblocker.skyblock.experiment.ChronomatronSolver by fabric2seer.

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
use mcrust::net::minecraft::world::inventory::abstract_container_menu::AbstractContainerMenu;
use mcrust::net::minecraft::world::item::item::Item;
use mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::ContainerScreenHost as _;
use mcrust::net::minecraft::network::chat::component::Component as _;

// TODO(fabric2seer): static call Object2ObjectMap.ofEntries(..)
//   java: public static final Object2ObjectMap<Item, Item> TERRACOTTA_TO_GLASS = Object2ObjectMap.ofEntries(
//   java: 			Object2ObjectMap.entry(Items.DYED_TERRACOTTA.red(), Items.STAINED_GLASS.red()),
//   java: 			Object2ObjectMap.entry(Items.DYED_TERRACOTTA.orange(), Items.STAINED_GLASS.orange()),
//   java: 			Object2ObjectMap.entry(Items.DYED_TERRACOTTA.yellow(), Items.STAINED_GLASS.yellow()),
//   java: 			Object2ObjectMap.entry(Items.DYED_TERRACOTTA.lime(), Items.STAINED_GLASS.lime()),
//   java: 			Object2ObjectMap.entry(Items.DYED_TERRACOTTA.green(), Items.STAINED_GLASS.green()),
//   java: 			Object2ObjectMap.entry(Items.DYED_TERRACOTTA.cyan(), Items.STAINED_GLASS.cyan()),
//   java: 			Object2ObjectMap.entry(Items.DYED_TERRACOTTA.lightBlue(), Items.STAINED_GLASS.lightBlue()),
//   java: 			Object2ObjectMap.entry(Items.DYED_TERRACOTTA.blue(), Items.STAINED_GLASS.blue()),
//   java: 			Object2ObjectMap.entry(Items.DYED_TERRACOTTA.purple(), Items.STAINED_GLASS.purple()),
//   java: 			Object2ObjectMap.entry(Items.DYED_TERRACOTTA.pink(), Items.STAINED_GLASS.pink())
//   java: 	);
/// java: public static final Object2ObjectMap<Item, Item> TERRACOTTA_TO_GLASS = Object2ObjectMap.ofEntries( Object2ObjectMap.entry(Items.DYED_TERRACOTTA.red(), Items.STAINED_GLASS.red()), Object2ObjectMap.entr
/// (`Item` keeps `Object.equals`: the map is its entries, looked up by identity)
pub static TERRACOTTA_TO_GLASS: compat::MainThread<Vec<(Arc<Item>, Arc<Item>)>> = compat::MainThread::new(|| {
    use mcrust::net::minecraft::world::item::items;
    vec![(items::dyed_terracotta().red().clone(), items::stained_glass().red().clone()), (items::dyed_terracotta().orange().clone(), items::stained_glass().orange().clone()), (items::dyed_terracotta().yellow().clone(), items::stained_glass().yellow().clone()), (items::dyed_terracotta().lime().clone(), items::stained_glass().lime().clone()), (items::dyed_terracotta().green().clone(), items::stained_glass().green().clone()), (items::dyed_terracotta().cyan().clone(), items::stained_glass().cyan().clone()), (items::dyed_terracotta().light_blue().clone(), items::stained_glass().light_blue().clone()), (items::dyed_terracotta().blue().clone(), items::stained_glass().blue().clone()), (items::dyed_terracotta().purple().clone(), items::stained_glass().purple().clone()), (items::dyed_terracotta().pink().clone(), items::stained_glass().pink().clone())]
});

/// java: `TERRACOTTA_TO_GLASS.get(item)` - `null` (`None`) for an item that is not a key
fn terracotta_to_glass(item: &Arc<Item>) -> Option<Arc<Item>> {
    TERRACOTTA_TO_GLASS.get().iter().find(|(k, _)| Arc::ptr_eq(k, item)).map(|(_, v)| v.clone())
}

/// java: public final class ChronomatronSolver extends ExperimentSolver implements ContainerListener
pub struct ChronomatronSolver {
    /// java: extends ExperimentSolver
    pub base: ExperimentSolverBase,
    /// java: private @Nullable AbstractContainerScreen<?> screen;
    pub screen: Option<AbstractContainerScreen>,
    /// java: private final List<Item> chronomatronSlots = new ArrayList<>();
    pub chronomatron_slots: Vec<Arc<Item>>,
    /// java: private int chronomatronChainLengthCount;
    pub chronomatron_chain_length_count: i32,
    /// java: private int chronomatronCurrentSlot;
    pub chronomatron_current_slot: i32,
    /// java: private int chronomatronCurrentOrdinal;
    pub chronomatron_current_ordinal: i32,
    /// java: private boolean isSingleRow;
    pub is_single_row: bool,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl std::ops::Deref for ChronomatronSolver {
    type Target = ExperimentSolverBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for ChronomatronSolver {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl ChronomatronSolver {
    /// java: public ChronomatronSolver()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = ExperimentSolverBase::new_in("^Chronomatron \\(\\w+\\)$".to_string(), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                self_ref: std::sync::Weak::new(),
                screen: None,
                chronomatron_slots: Vec::new(),
                chronomatron_chain_length_count: 0,
                chronomatron_current_slot: 0,
                chronomatron_current_ordinal: 0,
                is_single_row: false,
            });
            compat::write(&__h, |__t| __t.self_ref = std::sync::Arc::downgrade(&__h));
            ((__h.clone() as compat::Shared<dyn ExperimentSolver>), __h)
        });
        __this
    }

    /// java: @SuppressWarnings("incomplete-switch") @Override public void slotChanged(AbstractContainerMenu handler, int slotId, ItemStack stack)
    pub fn slot_changed(__this: &compat::Shared<Self>, handler: &AbstractContainerMenu, mut slot_id: i32, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack) {
        if slot_id < 17 || slot_id > (if compat::read(__this, |__t| __t.is_single_row) { 25 } else { 34 }) && slot_id != 49 {
            return;
        }
        'switch_1: {
            match ({ let __f = compat::dispatch(__this, |__t| __t.get_state()); __f(&compat::view(__this, |a| a as compat::Shared<dyn ExperimentSolver>)) }) {
                State::Remember => {
                    if slot_id == 49 {
                        break 'switch_1;
                    }
                    if compat::read(__this, |__t| __t.chronomatron_current_slot) == 0 {
                        if stack.has_foil() {
                            if (__this.lock().borrow().chronomatron_slots.len() as i32) <= compat::read(__this, |__t| __t.chronomatron_chain_length_count) {
                                let __h0 = terracotta_to_glass(&stack.get_item()).expect("NullPointerException");
                                __this.lock().borrow_mut().chronomatron_slots.push(__h0);
                                ({ let __f = compat::dispatch(__this, |__t| __t.set_state()); __f(&compat::view(__this, |a| a as compat::Shared<dyn ExperimentSolver>), State::Wait) });
                            } else {
                                { let __v = compat::read(__this, |__t| __t.chronomatron_chain_length_count) + 1; compat::write(__this, |__t| __t.chronomatron_chain_length_count = __v); }
                            }
                            { let __v = slot_id; compat::write(__this, |__t| __t.chronomatron_current_slot = __v); }
                        }
                    } else if compat::read(__this, |__t| __t.chronomatron_current_slot) == slot_id && !stack.has_foil() {
                        { let __v = 0; compat::write(__this, |__t| __t.chronomatron_current_slot = __v); }
                    }
                }
                State::Wait => {
                    if slot_id == 49 && compat::component_of(stack.get_hover_name()).get_string().starts_with("Timer: ") {
                        ({ let __f = compat::dispatch(__this, |__t| __t.set_state()); __f(&compat::view(__this, |a| a as compat::Shared<dyn ExperimentSolver>), State::Show) });
                    }
                }
                State::End => {
                    let mut name: String = compat::component_of(stack.get_hover_name()).get_string();
                    if slot_id == 49 && !(name.starts_with("Timer: ")) {
                        if name == "Remember the pattern!" {
                            { let __v = 0; compat::write(__this, |__t| __t.chronomatron_chain_length_count = __v); }
                            { let __v = 0; compat::write(__this, |__t| __t.chronomatron_current_ordinal = __v); }
                            ({ let __f = compat::dispatch(__this, |__t| __t.set_state()); __f(&compat::view(__this, |a| a as compat::Shared<dyn ExperimentSolver>), State::Remember) });
                        } else {
                            ({ let __f = compat::dispatch(__this, |__t| __t.reset()); __f(&compat::view(__this, |a| a as compat::Shared<dyn Resettable>)) });
                        }
                    }
                }
                _ => {}
            }
        }
    }

    /// java: @Override public void dataChanged(AbstractContainerMenu handler, int property, int value)
    pub fn data_changed(__this: &compat::Shared<Self>, handler: &AbstractContainerMenu, mut property: i32, mut value: i32) {
    }

}

impl ExperimentSolver for ChronomatronSolver {
    fn experiment_solver_base(&self) -> &ExperimentSolverBase {
        &self.base
    }
    fn experiment_solver_base_mut(&mut self) -> &mut ExperimentSolverBase {
        &mut self.base
    }

    /// java: @Override protected boolean isEnabled(HelperConfig.Experiments experimentsConfig) (dispatch: the body for this class)
    fn is_enabled_1(&self) -> fn(&compat::Shared<dyn ExperimentSolver>, &Experiments) -> bool {
        ChronomatronSolver::is_enabled_1_dyn
    }

    /// java: @Override protected void tick(AbstractContainerScreen<?> screen) (dispatch: the body for this class)
    fn tick(&self) -> fn(&compat::Shared<dyn ExperimentSolver>, &AbstractContainerScreen) {
        ChronomatronSolver::tick_dyn
    }

}

impl SimpleContainerSolver for ChronomatronSolver {
    fn simple_container_solver_base(&self) -> &SimpleContainerSolverBase {
        &self.base.base
    }
    fn simple_container_solver_base_mut(&mut self) -> &mut SimpleContainerSolverBase {
        &mut self.base.base
    }

}

impl RegexContainerMatcher for ChronomatronSolver {
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

impl ContainerMatcher for ChronomatronSolver {
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

impl ContainerSolver for ChronomatronSolver {
    /// java: @Override public List<ColorHighlight> getColors(Int2ObjectMap<ItemStack> slots) (dispatch: the body for this class)
    fn get_colors(&self) -> fn(&compat::Shared<dyn ContainerSolver>, &mut std::collections::HashMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> Vec<ColorHighlight> {
        ChronomatronSolver::get_colors_dyn
    }

    /// java: @Override public boolean onClickSlot(int slot, ItemStack stack, int screenId, int button) (dispatch: the body for this class)
    fn on_click_slot(&self) -> fn(&compat::Shared<dyn ContainerSolver>, i32, &mcrust::net::minecraft::world::item::item_stack::ItemStack, i32, i32) -> bool {
        ChronomatronSolver::on_click_slot_dyn
    }

    /// java: @Override public void start(AbstractContainerScreen<?> screen) (dispatch: the body for this class)
    fn start(&self) -> fn(&compat::Shared<dyn ContainerSolver>, &AbstractContainerScreen) {
        ChronomatronSolver::start_dyn
    }

}

impl Resettable for ChronomatronSolver {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void reset() (dispatch: the body for this class)
    fn reset(&self) -> fn(&compat::Shared<dyn Resettable>) {
        ChronomatronSolver::reset_dyn
    }

}

impl StackDisplayModifier for ChronomatronSolver {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public ItemStack modifyDisplayStack(int slotIndex, ItemStack stack) (dispatch: the body for this class)
    fn modify_display_stack(&self) -> fn(&compat::Shared<dyn StackDisplayModifier>, i32, &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> mcrust::net::minecraft::world::item::item_stack::ItemStack {
        <dyn ExperimentSolver>::modify_display_stack_from::<Self>
    }

}

impl ChronomatronSolver {
    /// java: @Override protected boolean isEnabled(HelperConfig.Experiments experimentsConfig)
    pub fn is_enabled_1_impl(__this: &compat::Shared<Self>, experiments_config: &Experiments) -> bool {
        return experiments_config.enable_chronomatron_solver;
    }

    /// ChronomatronSolver.isEnabled as the dispatch of ExperimentSolver.isEnabled
    pub fn is_enabled_1_dyn(__this: &compat::Shared<dyn ExperimentSolver>, __p0: &Experiments) -> bool {
        Self::is_enabled_1_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override protected void tick(AbstractContainerScreen<?> screen)
    pub fn tick_impl(__this: &compat::Shared<Self>, screen: &AbstractContainerScreen) {
    }

    /// ChronomatronSolver.tick as the dispatch of ExperimentSolver.tick
    pub fn tick_dyn(__this: &compat::Shared<dyn ExperimentSolver>, __p0: &AbstractContainerScreen) {
        Self::tick_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public List<ColorHighlight> getColors(Int2ObjectMap<ItemStack> slots)
    pub fn get_colors_impl(__this: &compat::Shared<Self>, slots: &mut std::collections::HashMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> Vec<ColorHighlight> {
        let mut highlights: Vec<ColorHighlight> = Vec::new();
        if ({ let __f = compat::dispatch(__this, |__t| __t.get_state()); __f(&compat::view(__this, |a| a as compat::Shared<dyn ExperimentSolver>)) }) == State::Show && (__this.lock().borrow().chronomatron_slots.len() as i32) > compat::read(__this, |__t| __t.chronomatron_current_ordinal) {
            for index_stack in slots.iter().map(|(k, v)| (k.clone(), v.clone())) {
                let mut index: i32 = index_stack.0;
                let stack = &index_stack.1;
                let mut item: Arc<Item> = __this.lock().borrow().chronomatron_slots[(compat::read(__this, |__t| __t.chronomatron_current_ordinal)) as usize].clone();
                if stack.is_item(&item) || /* java: item == TERRACOTTA_TO_GLASS.get(stack.getItem()) */ terracotta_to_glass(&stack.get_item()).is_some_and(|__g| Arc::ptr_eq(&__g, &item)) {
                    highlights.push(ColorHighlight::green(index));
                }
            }
        }
        return highlights;
    }

    /// ChronomatronSolver.getColors as the dispatch of ContainerSolver.getColors
    pub fn get_colors_dyn(__this: &compat::Shared<dyn ContainerSolver>, __p0: &mut std::collections::HashMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> Vec<ColorHighlight> {
        Self::get_colors_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public boolean onClickSlot(int slot, ItemStack stack, int screenId, int button)
    pub fn on_click_slot_impl(__this: &compat::Shared<Self>, mut slot: i32, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, mut screen_id: i32, mut button: i32) -> bool {
        if ({ let __f = compat::dispatch(__this, |__t| __t.get_state()); __f(&compat::view(__this, |a| a as compat::Shared<dyn ExperimentSolver>)) }) == State::Show {
            let mut item: Arc<Item> = __this.lock().borrow().chronomatron_slots[(compat::read(__this, |__t| __t.chronomatron_current_ordinal)) as usize].clone();
            if stack.is_item(&item) || /* java: item == TERRACOTTA_TO_GLASS.get(stack.getItem()) */ terracotta_to_glass(&stack.get_item()).is_some_and(|__g| Arc::ptr_eq(&__g, &item)) {
                // TODO(fabric2seer): the condition: ++/-- inside an expression
                //   java: ++chronomatronCurrentOrdinal >= chronomatronSlots.size()
                if todo!("fabric2seer: the condition (TODO above)") {
                    ({ let __f = compat::dispatch(__this, |__t| __t.set_state()); __f(&compat::view(__this, |a| a as compat::Shared<dyn ExperimentSolver>), State::End) });
                }
            } else {
                return ({ let __f = compat::dispatch(__this, |__t| __t.should_block_incorrect_clicks()); __f(&compat::view(__this, |a| a as compat::Shared<dyn ExperimentSolver>)) });
            }
        }
        return <dyn ContainerSolver>::on_click_slot_body(&compat::view(__this, |a| a as compat::Shared<dyn ContainerSolver>), slot, stack, screen_id, button);
    }

    /// ChronomatronSolver.onClickSlot as the dispatch of ContainerSolver.onClickSlot
    pub fn on_click_slot_dyn(__this: &compat::Shared<dyn ContainerSolver>, __p0: i32, __p1: &mcrust::net::minecraft::world::item::item_stack::ItemStack, __p2: i32, __p3: i32) -> bool {
        Self::on_click_slot_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2, __p3)
    }

    /// java: @Override public void start(AbstractContainerScreen<?> screen)
    pub fn start_impl(__this: &compat::Shared<Self>, screen: &AbstractContainerScreen) {
        <dyn ExperimentSolver>::start_body(&compat::view(__this, |a| a as compat::Shared<dyn ExperimentSolver>), screen);
        // TODO(fabric2seer): the mod keeps the game's `AbstractContainerScreen` (Java holds a reference to the live object); the port's `AbstractContainerScreen` is not a value to copy and the port lends the mod no handle to it
        // rust: { let __v = Some(screen.clone()); compat::write(__this, |__t| __t.screen = __v); }
        // TODO(fabric2seer): `addSlotListener` on a Object
        //   java: screen.getMenu().addSlotListener(this);
        let mut title: String = screen.screen.get_title().to_string();
        { let __v = (title.ends_with("(High)") || title.ends_with("(Grand)") || title.ends_with("(Supreme)")); compat::write(__this, |__t| __t.is_single_row = __v); }
    }

    /// ChronomatronSolver.start as the dispatch of ContainerSolver.start
    pub fn start_dyn(__this: &compat::Shared<dyn ContainerSolver>, __p0: &AbstractContainerScreen) {
        Self::start_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public void reset()
    pub fn reset_impl(__this: &compat::Shared<Self>) {
        __this.lock().borrow_mut().chronomatron_slots.clear();
        { let __v = 0; compat::write(__this, |__t| __t.chronomatron_chain_length_count = __v); }
        { let __v = 0; compat::write(__this, |__t| __t.chronomatron_current_slot = __v); }
        { let __v = 0; compat::write(__this, |__t| __t.chronomatron_current_ordinal = __v); }
        if compat::read(__this, |__t| __t.screen.is_some()) {
            // TODO(fabric2seer): `removeSlotListener` on a Object
            //   java: screen.getMenu().removeSlotListener(this);
        }
        <dyn ExperimentSolver>::reset_body(&compat::view(__this, |a| a as compat::Shared<dyn ExperimentSolver>));
    }

    /// ChronomatronSolver.reset as the dispatch of Resettable.reset
    pub fn reset_dyn(__this: &compat::Shared<dyn Resettable>) {
        Self::reset_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}

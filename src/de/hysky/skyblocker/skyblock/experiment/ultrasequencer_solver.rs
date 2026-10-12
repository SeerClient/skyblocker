//! Ported from de.hysky.skyblocker.skyblock.experiment.UltrasequencerSolver by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::configs::helper_config::Experiments;
use crate::de::hysky::skyblocker::skyblock::experiment::experiment_solver::ExperimentSolver;
use crate::de::hysky::skyblocker::skyblock::experiment::experiment_solver::ExperimentSolverBase;
use crate::de::hysky::skyblocker::skyblock::experiment::experiment_solver::State;
use crate::de::hysky::skyblocker::utils::container::container_matcher::ContainerMatcher;
use crate::de::hysky::skyblocker::utils::container::container_solver::ContainerSolver;
use crate::de::hysky::skyblocker::utils::container::container_solver_manager::ContainerSolverManager;
use crate::de::hysky::skyblocker::utils::container::regex_container_matcher::RegexContainerMatcher;
use crate::de::hysky::skyblocker::utils::container::regex_container_matcher::RegexContainerMatcherBase;
use crate::de::hysky::skyblocker::utils::container::simple_container_solver::SimpleContainerSolver;
use crate::de::hysky::skyblocker::utils::container::simple_container_solver::SimpleContainerSolverBase;
use crate::de::hysky::skyblocker::utils::container::stack_display_modifier::StackDisplayModifier;
use crate::de::hysky::skyblocker::utils::render::gui::color_highlight::ColorHighlight;
use crate::de::hysky::skyblocker::utils::resettable::Resettable;
use mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::AbstractContainerScreen;
use mcrust::net::minecraft::client::gui::screens::inventory::container_screen::ContainerScreen;
use mcrust::net::minecraft::world::inventory::abstract_container_menu::AbstractContainerMenu;
use mcrust::net::minecraft::world::inventory::slot::Slot;
use mcrust::net::minecraft::world::item::block_item;
use mcrust::net::minecraft::world::item::dye_color::DyeColor;
use mcrust::net::minecraft::world::item::item::Item;
use mcrust::net::minecraft::world::level::block::stained_glass_pane_block;
use mcrust::net::minecraft::world::level::block::stained_glass_pane_block::StainedGlassPaneBlock;
use mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::ContainerScreenHost as _;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: public static final UltrasequencerSolver INSTANCE = new UltrasequencerSolver();
pub static INSTANCE: compat::MainThread<compat::Shared<UltrasequencerSolver>> = compat::MainThread::new(|| UltrasequencerSolver::new());
/// java: private static final int[] PANE_SLOTS = IntStream.rangeClosed(9, 44).toArray();
pub static PANE_SLOTS: LazyLock<Vec<i32>> = LazyLock::new(|| (9..=44).collect::<Vec<_>>());

/// java: public final class UltrasequencerSolver extends ExperimentSolver
#[derive(Clone)]
pub struct UltrasequencerSolver {
    /// java: extends ExperimentSolver
    pub base: ExperimentSolverBase,
    /// java: private int ultrasequencerNextSlot;
    pub ultrasequencer_next_slot: i32,
    /// java: private @Nullable DyeColor lastColor;
    pub last_color: Option<DyeColor>,
}

impl std::ops::Deref for UltrasequencerSolver {
    type Target = ExperimentSolverBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for UltrasequencerSolver {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl UltrasequencerSolver {
    /// java: private UltrasequencerSolver()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = ExperimentSolverBase::new_in("^Ultrasequencer \\(\\w+\\)$".to_string(), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                ultrasequencer_next_slot: 0,
                last_color: None,
            });
            ((__h.clone() as compat::Shared<dyn ExperimentSolver>), __h)
        });
        __this
    }

}

impl ExperimentSolver for UltrasequencerSolver {
    fn experiment_solver_base(&self) -> &ExperimentSolverBase {
        &self.base
    }
    fn experiment_solver_base_mut(&mut self) -> &mut ExperimentSolverBase {
        &mut self.base
    }

    /// java: @Override protected boolean isEnabled(HelperConfig.Experiments experimentsConfig) (dispatch: the body for this class)
    fn is_enabled_1(&self) -> fn(&compat::Shared<dyn ExperimentSolver>, &Experiments) -> bool {
        UltrasequencerSolver::is_enabled_1_dyn
    }

    /// java: @SuppressWarnings( (dispatch: the body for this class)
    fn tick(&self) -> fn(&compat::Shared<dyn ExperimentSolver>, &AbstractContainerScreen) {
        UltrasequencerSolver::tick_dyn
    }

}

impl SimpleContainerSolver for UltrasequencerSolver {
    fn simple_container_solver_base(&self) -> &SimpleContainerSolverBase {
        &self.base.base
    }
    fn simple_container_solver_base_mut(&mut self) -> &mut SimpleContainerSolverBase {
        &mut self.base.base
    }

}

impl RegexContainerMatcher for UltrasequencerSolver {
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

impl ContainerMatcher for UltrasequencerSolver {
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

impl ContainerSolver for UltrasequencerSolver {
    /// java: @Override public List<ColorHighlight> getColors(Int2ObjectMap<ItemStack> slots) (dispatch: the body for this class)
    fn get_colors(&self) -> fn(&compat::Shared<dyn ContainerSolver>, &mut std::collections::HashMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> Vec<ColorHighlight> {
        UltrasequencerSolver::get_colors_dyn
    }

    /// java: @SuppressWarnings("JavadocReference") @Override public boolean onClickSlot(int slot, ItemStack stack, int screenId, int button) (dispatch: the body for this class)
    fn on_click_slot(&self) -> fn(&compat::Shared<dyn ContainerSolver>, i32, &mcrust::net::minecraft::world::item::item_stack::ItemStack, i32, i32) -> bool {
        UltrasequencerSolver::on_click_slot_dyn
    }

    /// java: @Override public void markDirty() (dispatch: the body for this class)
    fn mark_dirty(&self) -> fn(&compat::Shared<dyn ContainerSolver>) {
        UltrasequencerSolver::mark_dirty_dyn
    }

    /// java: @Override public void start(AbstractContainerScreen<?> screen) (dispatch: the body for this class)
    fn start(&self) -> fn(&compat::Shared<dyn ContainerSolver>, &AbstractContainerScreen) {
        <dyn ExperimentSolver>::start_from::<Self>
    }

}

impl Resettable for UltrasequencerSolver {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void reset() (dispatch: the body for this class)
    fn reset(&self) -> fn(&compat::Shared<dyn Resettable>) {
        UltrasequencerSolver::reset_dyn
    }

}

impl StackDisplayModifier for UltrasequencerSolver {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public ItemStack modifyDisplayStack(int slotIndex, ItemStack stack) (dispatch: the body for this class)
    fn modify_display_stack(&self) -> fn(&compat::Shared<dyn StackDisplayModifier>, i32, &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> mcrust::net::minecraft::world::item::item_stack::ItemStack {
        <dyn ExperimentSolver>::modify_display_stack_from::<Self>
    }

}

impl UltrasequencerSolver {
    /// java: @Override protected boolean isEnabled(HelperConfig.Experiments experimentsConfig)
    pub fn is_enabled_1_impl(__this: &compat::Shared<Self>, experiments_config: &Experiments) -> bool {
        return experiments_config.enable_ultrasequencer_solver;
    }

    /// UltrasequencerSolver.isEnabled as the dispatch of ExperimentSolver.isEnabled
    pub fn is_enabled_1_dyn(__this: &compat::Shared<dyn ExperimentSolver>, __p0: &Experiments) -> bool {
        Self::is_enabled_1_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @SuppressWarnings(
    pub fn tick_impl(__this: &compat::Shared<Self>, screen: &AbstractContainerScreen) {
        match ({ let __f = compat::dispatch(__this, |__t| __t.get_state()); __f(&compat::view(__this, |a| a as compat::Shared<dyn ExperimentSolver>)) }) {
            State::Remember => {
                let mut inventory: AbstractContainerMenu = screen.menu();
                // TODO(fabric2seer): `get_item` takes 1 argument(s) in the port (`containers: &'a Containers<'_>`), Java passes 0
                if compat::component_of(inventory.get_slot(49).get_item().get_hover_name()).get_string() == "Remember the pattern!" {
                    for index in 9..45 {
                        // TODO(fabric2seer): `get_item` takes 1 argument(s) in the port (`containers: &'a Containers<'_>`), Java passes 0
                        // rust: let item_stack = inventory.get_slot(index).get_item();
                        let mut item_stack: mcrust::net::minecraft::world::item::item_stack::ItemStack = todo!("fabric2seer: the initialiser of `itemStack` (TODO above)");
                        let mut name: String = compat::component_of(item_stack.get_hover_name()).get_string();
                        if compat::regex::Pattern::matches("\\d+", &name) {
                            if name == "1" {
                                { let __v = index; compat::write(__this, |__t| __t.ultrasequencer_next_slot = __v); }
                            }
                            (({ let __f = compat::dispatch(__this, |__t| __t.get_slots()); __f(&compat::view(__this, |a| a as compat::Shared<dyn ExperimentSolver>)) })).insert(index, item_stack.clone());
                        }
                    }
                    ({ let __f = compat::dispatch(__this, |__t| __t.set_state()); __f(&compat::view(__this, |a| a as compat::Shared<dyn ExperimentSolver>), State::Wait) });
                }
            }
            State::Wait => {
                // TODO(fabric2seer): the condition: `getSlot` on a Object
                //   java: screen.getMenu(..).getSlot(..).getItem(..).getHoverName(..).getString(..).startsWith(..)
                if todo!("fabric2seer: the condition (TODO above)") {
                    ({ let __f = compat::dispatch(__this, |__t| __t.set_state()); __f(&compat::view(__this, |a| a as compat::Shared<dyn ExperimentSolver>), State::Show) });
                    ContainerSolverManager::mark_highlights_dirty();
                }
            }
            State::End => {
                // TODO(fabric2seer): `getSlot` on a Object
                //   java: String name = screen.getMenu().getSlot(49).getItem().getHoverName().getString();
                let mut name: String = todo!("fabric2seer: the initialiser of `name` (TODO above)");
                if !(name.starts_with("Timer: ")) {
                    if name == "Remember the pattern!" {
                        (({ let __f = compat::dispatch(__this, |__t| __t.get_slots()); __f(&compat::view(__this, |a| a as compat::Shared<dyn ExperimentSolver>)) })).clear();
                        ({ let __f = compat::dispatch(__this, |__t| __t.set_state()); __f(&compat::view(__this, |a| a as compat::Shared<dyn ExperimentSolver>), State::Remember) });
                    } else {
                        ({ let __f = compat::dispatch(__this, |__t| __t.reset()); __f(&compat::view(__this, |a| a as compat::Shared<dyn Resettable>)) });
                    }
                }
            }
            _ => {}
        }
    }

    /// UltrasequencerSolver.tick as the dispatch of ExperimentSolver.tick
    pub fn tick_dyn(__this: &compat::Shared<dyn ExperimentSolver>, __p0: &AbstractContainerScreen) {
        Self::tick_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public List<ColorHighlight> getColors(Int2ObjectMap<ItemStack> slots)
    pub fn get_colors_impl(__this: &compat::Shared<Self>, slots: &mut std::collections::HashMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> Vec<ColorHighlight> {
        return (if ({ let __f = compat::dispatch(__this, |__t| __t.get_state()); __f(&compat::view(__this, |a| a as compat::Shared<dyn ExperimentSolver>)) }) == State::Show && compat::read(__this, |__t| __t.ultrasequencer_next_slot) != 0 { vec![ColorHighlight::green(compat::read(__this, |__t| __t.ultrasequencer_next_slot))] } else { vec![] });
    }

    /// UltrasequencerSolver.getColors as the dispatch of ContainerSolver.getColors
    pub fn get_colors_dyn(__this: &compat::Shared<dyn ContainerSolver>, __p0: &mut std::collections::HashMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> Vec<ColorHighlight> {
        Self::get_colors_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @SuppressWarnings("JavadocReference") @Override public boolean onClickSlot(int slot, ItemStack stack, int screenId, int button)
    pub fn on_click_slot_impl(__this: &compat::Shared<Self>, mut slot: i32, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, mut screen_id: i32, mut button: i32) -> bool {
        if ({ let __f = compat::dispatch(__this, |__t| __t.get_state()); __f(&compat::view(__this, |a| a as compat::Shared<dyn ExperimentSolver>)) }) != State::Show {
            return ({ let __f = compat::dispatch(__this, |__t| __t.should_block_incorrect_clicks()); __f(&compat::view(__this, |a| a as compat::Shared<dyn ExperimentSolver>)) });
        }
        if slot != compat::read(__this, |__t| __t.ultrasequencer_next_slot) {
            return ({ let __f = compat::dispatch(__this, |__t| __t.should_block_incorrect_clicks()); __f(&compat::view(__this, |a| a as compat::Shared<dyn ExperimentSolver>)) });
        }
        let mut count: i32 = (((({ let __f = compat::dispatch(__this, |__t| __t.get_slots()); __f(&compat::view(__this, |a| a as compat::Shared<dyn ExperimentSolver>)) })).get(&compat::read(__this, |__t| __t.ultrasequencer_next_slot)).cloned().unwrap()).get_count()).wrapping_add(1);
        // TODO(fabric2seer): static field Int2ObjectMap.Entry
        //   java: getSlots().int2ObjectEntrySet().stream()
        //   java: 			.filter(entry -> entry.getValue().getCount() == count)
        //   java: 			.findAny()
        //   java: 			.map(Int2ObjectMap.Entry::getIntKey)
        //   java: 			.ifPresent(nextSlot -> this.ultrasequencerNextSlot = nextSlot);
        return <dyn ContainerSolver>::on_click_slot_body(&compat::view(__this, |a| a as compat::Shared<dyn ContainerSolver>), slot, stack, screen_id, button);
    }

    /// UltrasequencerSolver.onClickSlot as the dispatch of ContainerSolver.onClickSlot
    pub fn on_click_slot_dyn(__this: &compat::Shared<dyn ContainerSolver>, __p0: i32, __p1: &mcrust::net::minecraft::world::item::item_stack::ItemStack, __p2: i32, __p3: i32) -> bool {
        Self::on_click_slot_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2, __p3)
    }

    /// java: @Override public void markDirty()
    pub fn mark_dirty_impl(__this: &compat::Shared<Self>) {
        if let Some(generic_container_screen) = compat::minecraft().gui.screen().unwrap().as_any().downcast_ref::<ContainerScreen>() {
            // TODO(fabric2seer): field `slots` of a value of unknown type
            //   java: List<Slot> slots = genericContainerScreen.getMenu().slots.subList(0, genericContainerScreen.getMenu().getRowCount() * 9);
            let mut slots: Vec<Slot> = todo!("fabric2seer: the initialiser of `slots` (TODO above)");
            let mut slot_map: std::collections::HashMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack> = ContainerSolverManager::slot_map(&slots);
            for pane_slot in PANE_SLOTS.iter().copied() {
                let mut slot_item: Option<mcrust::net::minecraft::world::item::item_stack::ItemStack> = slot_map.get(&pane_slot).cloned();
                // TODO(fabric2seer): `get_block` takes 1 argument(s) in the port (`item: &Arc<Item>`), Java passes 0
                if slot_item.is_some() && !(slot_item.as_ref().unwrap().is_empty()) && let Some(block_item) = slot_item.as_ref().unwrap().get_item().as_any().downcast_ref::<Item>() && let Some(stained_glass_pane_block) = block_item::get_block().unwrap().as_any().downcast_ref::<StainedGlassPaneBlock>() {
                    // TODO(fabric2seer): `get_color` takes 1 argument(s) in the port (`block: &Block`), Java passes 0
                    // rust: let mut pane_color: DyeColor = stained_glass_pane_block::get_color();
                    let mut pane_color: DyeColor = todo!("fabric2seer: the initialiser of `paneColor` (TODO above)");
                    if pane_color == DyeColor::Black {
                        continue;
                    }
                    if compat::read(__this, |__t| __t.last_color.clone()).unwrap() != pane_color {
                        if compat::read(__this, |__t| __t.last_color.clone()).is_some() {
                            ({ let __f = compat::dispatch(__this, |__t| __t.set_state()); __f(&compat::view(__this, |a| a as compat::Shared<dyn ExperimentSolver>), State::End) });
                        }
                        { let __v = Some(pane_color); compat::write(__this, |__t| __t.last_color = __v); }
                        return;
                    }
                }
            }
        }
    }

    /// UltrasequencerSolver.markDirty as the dispatch of ContainerSolver.markDirty
    pub fn mark_dirty_dyn(__this: &compat::Shared<dyn ContainerSolver>) {
        Self::mark_dirty_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public void reset()
    pub fn reset_impl(__this: &compat::Shared<Self>) {
        { let __v = 0; compat::write(__this, |__t| __t.ultrasequencer_next_slot = __v); }
        { let __v = None; compat::write(__this, |__t| __t.last_color = __v); }
        <dyn ExperimentSolver>::reset_body(&compat::view(__this, |a| a as compat::Shared<dyn ExperimentSolver>));
    }

    /// UltrasequencerSolver.reset as the dispatch of Resettable.reset
    pub fn reset_dyn(__this: &compat::Shared<dyn Resettable>) {
        Self::reset_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}

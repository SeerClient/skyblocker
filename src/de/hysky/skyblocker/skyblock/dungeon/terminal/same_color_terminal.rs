//! Ported from de.hysky.skyblocker.skyblock.dungeon.terminal.SameColorTerminal by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::dungeon::terminal::terminal_solver::TerminalSolver;
use crate::de::hysky::skyblocker::skyblock::item::slottext::slot_text::SlotText;
use crate::de::hysky::skyblocker::utils::container::container_matcher::ContainerMatcher;
use crate::de::hysky::skyblocker::utils::container::container_solver::ContainerSolver;
use crate::de::hysky::skyblocker::utils::container::regex_container_matcher::RegexContainerMatcher;
use crate::de::hysky::skyblocker::utils::container::regex_container_matcher::RegexContainerMatcherBase;
use crate::de::hysky::skyblocker::utils::container::simple_container_solver::SimpleContainerSolver;
use crate::de::hysky::skyblocker::utils::container::simple_container_solver::SimpleContainerSolverBase;
use crate::de::hysky::skyblocker::utils::container::slot_text_adder::SlotTextAdder;
use crate::de::hysky::skyblocker::utils::render::gui::color_highlight::ColorHighlight;
use crate::de::hysky::skyblocker::utils::resettable::Resettable;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::world::inventory::slot::Slot;
use mcrust::net::minecraft::world::item::item::Item;

/// java: public static final SameColorTerminal INSTANCE = new SameColorTerminal();
pub static INSTANCE: compat::MainThread<compat::Shared<SameColorTerminal>> = compat::MainThread::new(|| SameColorTerminal::new());
/// java: private static final Item[] ORDER = new Item[]{ Items.STAINED_GLASS_PANE.red(), Items.STAINED_GLASS_PANE.orange(), Items.STAINED_GLASS_PANE.yellow(), Items.STAINED_GLASS_PANE.green(), Items.STAINED_GL
pub static ORDER: compat::MainThread<Vec<Arc<Item>>> = compat::MainThread::new(|| vec![mcrust::net::minecraft::world::item::items::stained_glass_pane().red().clone(), mcrust::net::minecraft::world::item::items::stained_glass_pane().orange().clone(), mcrust::net::minecraft::world::item::items::stained_glass_pane().yellow().clone(), mcrust::net::minecraft::world::item::items::stained_glass_pane().green().clone(), mcrust::net::minecraft::world::item::items::stained_glass_pane().blue().clone()]);
/// java: private static final ImmutableMap<Item, Integer> INDEX;
/// Keyed by the item OBJECT (Java's `Item` has identity equality; the port's `Item` has no `Hash`/`Eq`): pairs
/// looked up with `Arc::ptr_eq` (`index_of`).
pub static INDEX: compat::MainThread<Vec<(Arc<Item>, i32)>> = compat::MainThread::new(|| Vec::new());

/// java: public final class SameColorTerminal extends SimpleContainerSolver implements TerminalSolver, SlotTextAdder
#[derive(Clone)]
pub struct SameColorTerminal {
    /// java: extends SimpleContainerSolver
    pub base: SimpleContainerSolverBase,
    /// java: private final Int2IntMap clickMap = new Int2IntOpenHashMap();
    pub click_map: std::collections::HashMap<i32, i32>,
}

impl std::ops::Deref for SameColorTerminal {
    type Target = SimpleContainerSolverBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for SameColorTerminal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl SameColorTerminal {
    /// java: static { .. } - Java runs it when the class is first used; every static method and
    /// constructor calls this first.
    fn clinit() {
        static ONCE: std::sync::Once = std::sync::Once::new();
        ONCE.call_once(|| {
            // ImmutableMap.Builder<Item, Integer> builder = ImmutableMap.builderWithExpectedSize(ORDER.length);
            let mut builder: Vec<(Arc<Item>, i32)> = Vec::with_capacity(ORDER.get().len());
            for i in 0..(ORDER.get_mut().len() as i32) {
                // builder.put(ORDER[i], i);
                builder.push((ORDER.get()[(i) as usize].clone(), i));
            }
            // INDEX = builder.build();
            *INDEX.get_mut() = builder;
        });
    }

    /// java: private SameColorTerminal()
    pub fn new() -> compat::Shared<Self> {
        Self::clinit();
        let (_, __this) = SimpleContainerSolverBase::new_in("^Change all to same color!$".to_string(), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                click_map: std::collections::HashMap::new(),
            });
            ((__h.clone() as compat::Shared<dyn SimpleContainerSolver>), __h)
        });
        __this
    }

    /// java: private void computeClicks(Int2ObjectMap<ItemStack> slots)
    pub fn compute_clicks(__this: &compat::Shared<Self>, slots: &std::collections::HashMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack>) {
        __this.lock().borrow_mut().click_map.clear();
        let mut counts: Vec<i32> = vec![0; ((ORDER.get_mut().len() as i32)) as usize];
        let mut slot_colors: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();
        for entry in slots.iter().map(|(k, v)| (k.clone(), v.clone())) {
            let mut item: Arc<Item> = entry.1.get_item();
            // Integer idx = INDEX.get(item); if (idx != null)
            if let Some(idx) = INDEX.get().iter().find(|(i, _)| Arc::ptr_eq(i, &item)).map(|(_, idx)| *idx) {
                slot_colors.insert(entry.0, idx);
                counts[(idx) as usize] += 1;
            }
        }
        if slot_colors.is_empty() {
            return;
        }
        let mut target: i32 = 0;
        for i in 1..(counts.len() as i32) {
            if counts[(i) as usize] > counts[(target) as usize] {
                target = i;
            }
        }
        // TODO(fabric2seer): Map.int2IntEntrySet(..)
        //   java: for (Int2IntMap.Entry entry : slotColors.int2IntEntrySet()) {
        //   java: 			int slot = entry.getIntKey();
        //   java: 			int color = entry.getIntValue();
        //   java: 			int diffForward = Math.floorMod(target - color, ORDER.length);
        //   java: 			int diffBackward = Math.floorMod(color - target, ORDER.length);
        //   java: 			int clicks = diffForward <= diffBackward ? diffForward : -diffBackward;
        //   java: 			clickMap.put(slot, clicks);
        //   java: 		}
    }

}

impl SimpleContainerSolver for SameColorTerminal {
    fn simple_container_solver_base(&self) -> &SimpleContainerSolverBase {
        &self.base
    }
    fn simple_container_solver_base_mut(&mut self) -> &mut SimpleContainerSolverBase {
        &mut self.base
    }

}

impl RegexContainerMatcher for SameColorTerminal {
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

impl ContainerMatcher for SameColorTerminal {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public boolean isEnabled() (dispatch: the body for this class)
    fn is_enabled(&self) -> fn(&compat::Shared<dyn ContainerMatcher>) -> bool {
        SameColorTerminal::is_enabled_dyn
    }

    /// java: @Override public boolean test(Screen screen) (dispatch: the body for this class)
    fn test(&self) -> fn(&compat::Shared<dyn ContainerMatcher>, &mcrust::net::minecraft::client::gui::screens::screen::Screen) -> bool {
        <dyn RegexContainerMatcher>::test_from::<Self>
    }

}

impl ContainerSolver for SameColorTerminal {
    /// java: @Override public List<ColorHighlight> getColors(Int2ObjectMap<ItemStack> slots) (dispatch: the body for this class)
    fn get_colors(&self) -> fn(&compat::Shared<dyn ContainerSolver>, &mut std::collections::HashMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> Vec<ColorHighlight> {
        SameColorTerminal::get_colors_dyn
    }

    /// java: @Override public boolean onClickSlot(int slot, ItemStack stack, int screenId, int button) (dispatch: the body for this class)
    fn on_click_slot(&self) -> fn(&compat::Shared<dyn ContainerSolver>, i32, &mcrust::net::minecraft::world::item::item_stack::ItemStack, i32, i32) -> bool {
        SameColorTerminal::on_click_slot_dyn
    }

}

impl Resettable for SameColorTerminal {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override default void reset() (dispatch: the body for this class)
    fn reset(&self) -> fn(&compat::Shared<dyn Resettable>) {
        <dyn ContainerSolver>::reset_from::<Self>
    }

}

impl TerminalSolver for SameColorTerminal {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}

impl SlotTextAdder for SameColorTerminal {
    /// java: @Override public List<SlotText> getText(@Nullable Slot slot, ItemStack stack, int slotId) (dispatch: the body for this class)
    fn get_text(&self) -> fn(&compat::Shared<dyn SlotTextAdder>, Option<Slot>, &mcrust::net::minecraft::world::item::item_stack::ItemStack, i32) -> Vec<SlotText> {
        SameColorTerminal::get_text_dyn
    }

}

impl SameColorTerminal {
    /// java: @Override public boolean isEnabled()
    pub fn is_enabled_impl(__this: &compat::Shared<Self>) -> bool {
        __this.lock().borrow_mut().click_map.clear();
        return SkyblockerConfigManager::get().dungeons.terminals.solve_same_color;
    }

    /// SameColorTerminal.isEnabled as the dispatch of ContainerMatcher.isEnabled
    pub fn is_enabled_dyn(__this: &compat::Shared<dyn ContainerMatcher>) -> bool {
        Self::is_enabled_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public List<ColorHighlight> getColors(Int2ObjectMap<ItemStack> slots)
    pub fn get_colors_impl(__this: &compat::Shared<Self>, slots: &mut std::collections::HashMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> Vec<ColorHighlight> {
        SameColorTerminal::compute_clicks(__this, slots);
        return vec![];
    }

    /// SameColorTerminal.getColors as the dispatch of ContainerSolver.getColors
    pub fn get_colors_dyn(__this: &compat::Shared<dyn ContainerSolver>, __p0: &mut std::collections::HashMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> Vec<ColorHighlight> {
        Self::get_colors_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public boolean onClickSlot(int slot, ItemStack stack, int screenId, int button)
    pub fn on_click_slot_impl(__this: &compat::Shared<Self>, mut slot: i32, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, mut screen_id: i32, mut button: i32) -> bool {
        if __this.lock().borrow().click_map.contains_key(&slot) {
            let mut clicks: i32 = __this.lock().borrow().click_map.get(&slot).copied().unwrap_or_default();
            if clicks == 0 {
                return ({ let __f = compat::dispatch(__this, |__t| __t.should_block_incorrect_clicks()); __f(&compat::view(__this, |a| a as compat::Shared<dyn TerminalSolver>)) });
            } else {
                let mut positive: bool = (clicks.signum() == 1);
                let mut using_correct_button: bool = (positive && button == 0 || !positive && button == 1);
                if !using_correct_button {
                    return ({ let __f = compat::dispatch(__this, |__t| __t.should_block_incorrect_clicks()); __f(&compat::view(__this, |a| a as compat::Shared<dyn TerminalSolver>)) });
                }
            }
        }
        return false;
    }

    /// SameColorTerminal.onClickSlot as the dispatch of ContainerSolver.onClickSlot
    pub fn on_click_slot_dyn(__this: &compat::Shared<dyn ContainerSolver>, __p0: i32, __p1: &mcrust::net::minecraft::world::item::item_stack::ItemStack, __p2: i32, __p3: i32) -> bool {
        Self::on_click_slot_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2, __p3)
    }

    /// java: @Override public List<SlotText> getText(@Nullable Slot slot, ItemStack stack, int slotId)
    pub fn get_text_impl(__this: &compat::Shared<Self>, mut slot: Option<Slot>, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, mut slot_id: i32) -> Vec<SlotText> {
        let mut clicks: i32 = __this.lock().borrow().click_map.get(&slot_id).cloned().unwrap_or_else(|| 0);
        if clicks == 0 {
            return vec![];
        }
        return SlotText::top_left_list(&component::literal(&clicks.to_string()).with_style_format(ChatFormatting::Gold));
    }

    /// SameColorTerminal.getText as the dispatch of SlotTextAdder.getText
    pub fn get_text_dyn(__this: &compat::Shared<dyn SlotTextAdder>, __p0: Option<Slot>, __p1: &mcrust::net::minecraft::world::item::item_stack::ItemStack, __p2: i32) -> Vec<SlotText> {
        Self::get_text_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2)
    }

}

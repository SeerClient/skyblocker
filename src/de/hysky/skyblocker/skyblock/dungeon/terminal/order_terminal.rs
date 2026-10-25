//! Ported from de.hysky.skyblocker.skyblock.dungeon.terminal.OrderTerminal by fabric2seer.

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
use crate::de::hysky::skyblocker::utils::render::gui::color_highlight::ColorHighlight;
use crate::de::hysky::skyblocker::utils::resettable::Resettable;
use mcrust::net::minecraft::util::argb;

/// java: private static final int PANES_NUM = 14;
pub const PANES_NUM: i32 = 14;

/// java: public final class OrderTerminal extends SimpleContainerSolver implements TerminalSolver
#[derive(Clone)]
pub struct OrderTerminal {
    /// java: extends SimpleContainerSolver
    pub base: SimpleContainerSolverBase,
    /// java: private int @Nullable [] orderedSlots;
    pub ordered_slots: Option<Vec<i32>>,
    /// java: private int currentNum = Integer.MAX_VALUE;
    pub current_num: i32,
}

impl std::ops::Deref for OrderTerminal {
    type Target = SimpleContainerSolverBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for OrderTerminal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl OrderTerminal {
    /// java: public OrderTerminal()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = SimpleContainerSolverBase::new_in("^Click in order!$".to_string(), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                ordered_slots: None,
                current_num: i32::MAX,
            });
            ((__h.clone() as compat::Shared<dyn SimpleContainerSolver>), __h)
        });
        __this
    }

    /// java: public boolean orderSlots(Int2ObjectMap<ItemStack> slots)
    pub fn order_slots(__this: &compat::Shared<Self>, slots: &mut std::collections::BTreeMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> bool {
        container_solver::trim_edges(slots, 4);
        // java: orderedSlots = new int[PANES_NUM]; Arrays.fill(orderedSlots, -1);
        let mut ordered_slots: Vec<i32> = vec![-1; PANES_NUM as usize];
        for slot in slots.iter().map(|(k, v)| (k.clone(), v.clone())) {
            let stack = &slot.1;
            if stack.is_item(mcrust::net::minecraft::world::item::items::stained_glass_pane().black()) {
                continue;
            }
            if stack.is_empty() {
                // java: orderedSlots = null;
                compat::write(__this, |__t| __t.ordered_slots = None);
                return false;
            }
            // java: orderedSlots[stack.getCount() - 1] = slot.getIntKey(); (out of range throws, as Java's array store)
            ordered_slots[(stack.get_count().wrapping_sub(1)) as usize] = slot.0;
        }
        compat::write(__this, |__t| __t.ordered_slots = Some(ordered_slots));
        { let __v = 0; compat::write(__this, |__t| __t.current_num = __v); }
        return true;
    }

}

impl SimpleContainerSolver for OrderTerminal {
    fn simple_container_solver_base(&self) -> &SimpleContainerSolverBase {
        &self.base
    }
    fn simple_container_solver_base_mut(&mut self) -> &mut SimpleContainerSolverBase {
        &mut self.base
    }

}

impl RegexContainerMatcher for OrderTerminal {
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

impl ContainerMatcher for OrderTerminal {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public boolean isEnabled() (dispatch: the body for this class)
    fn is_enabled(&self) -> fn(&compat::Shared<dyn ContainerMatcher>) -> bool {
        OrderTerminal::is_enabled_dyn
    }

    /// java: @Override public boolean test(Screen screen) (dispatch: the body for this class)
    fn test(&self) -> fn(&compat::Shared<dyn ContainerMatcher>, &mcrust::net::minecraft::client::gui::screens::screen::Screen) -> bool {
        <dyn RegexContainerMatcher>::test_from::<Self>
    }

}

impl ContainerSolver for OrderTerminal {
    /// java: @Override public List<ColorHighlight> getColors(Int2ObjectMap<ItemStack> slots) (dispatch: the body for this class)
    fn get_colors(&self) -> fn(&compat::Shared<dyn ContainerSolver>, &mut std::collections::BTreeMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> Vec<ColorHighlight> {
        OrderTerminal::get_colors_dyn
    }

    /// java: @Override public boolean onClickSlot(int slot, ItemStack stack, int screenId, int button) (dispatch: the body for this class)
    fn on_click_slot(&self) -> fn(&compat::Shared<dyn ContainerSolver>, i32, &mcrust::net::minecraft::world::item::item_stack::ItemStack, i32, i32) -> bool {
        OrderTerminal::on_click_slot_dyn
    }

}

impl Resettable for OrderTerminal {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override default void reset() (dispatch: the body for this class)
    fn reset(&self) -> fn(&compat::Shared<dyn Resettable>) {
        <dyn ContainerSolver>::reset_from::<Self>
    }

}

impl TerminalSolver for OrderTerminal {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}

impl OrderTerminal {
    /// java: @Override public boolean isEnabled()
    pub fn is_enabled_impl(__this: &compat::Shared<Self>) -> bool {
        // java: orderedSlots = null;
        compat::write(__this, |__t| __t.ordered_slots = None);
        { let __v = 0; compat::write(__this, |__t| __t.current_num = __v); }
        return SkyblockerConfigManager::get().dungeons.terminals.solve_order;
    }

    /// OrderTerminal.isEnabled as the dispatch of ContainerMatcher.isEnabled
    pub fn is_enabled_dyn(__this: &compat::Shared<dyn ContainerMatcher>) -> bool {
        Self::is_enabled_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public List<ColorHighlight> getColors(Int2ObjectMap<ItemStack> slots)
    pub fn get_colors_impl(__this: &compat::Shared<Self>, slots: &mut std::collections::BTreeMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> Vec<ColorHighlight> {
        // java: orderedSlots == null && !orderSlots(slots)
        if compat::read(__this, |__t| __t.ordered_slots.is_none()) && !OrderTerminal::order_slots(__this, slots) {
            return Default::default();
        }
        // java: currentNum < PANES_NUM && slots.containsKey(orderedSlots[currentNum]) && slots.get(orderedSlots[currentNum]).is(Items.STAINED_GLASS_PANE.lime())
        let ordered_slots: Vec<i32> = compat::read(__this, |__t| __t.ordered_slots.clone()).expect("NullPointerException: orderedSlots");
        while compat::read(__this, |__t| __t.current_num) < PANES_NUM && {
            let __slot = ordered_slots[compat::read(__this, |__t| __t.current_num) as usize];
            slots.get(&__slot).is_some_and(|__stack| __stack.is_item(mcrust::net::minecraft::world::item::items::stained_glass_pane().lime()))
        } {
            { let __v = compat::read(__this, |__t| __t.current_num) + 1; compat::write(__this, |__t| __t.current_num = __v); }
        }
        let mut highlights: Vec<ColorHighlight> = Vec::with_capacity((3) as usize);
        let mut last: i32 = 3i32.min(PANES_NUM.wrapping_sub(compat::read(__this, |__t| __t.current_num)));
        for i in 0..last {
            // java: int slotNum = orderedSlots[currentNum + i];
            let mut slot_num: i32 = ordered_slots[(compat::read(__this, |__t| __t.current_num).wrapping_add(i)) as usize];
            if slot_num == -1 {
                continue;
            }
            highlights.push(ColorHighlight::new(slot_num, argb::color_alpha_rgb(224i32.wrapping_sub(64i32.wrapping_mul(i)), 0x4060FF)));
        }
        return highlights;
    }

    /// OrderTerminal.getColors as the dispatch of ContainerSolver.getColors
    pub fn get_colors_dyn(__this: &compat::Shared<dyn ContainerSolver>, __p0: &mut std::collections::BTreeMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> Vec<ColorHighlight> {
        Self::get_colors_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public boolean onClickSlot(int slot, ItemStack stack, int screenId, int button)
    pub fn on_click_slot_impl(__this: &compat::Shared<Self>, mut slot: i32, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, mut screen_id: i32, mut button: i32) -> bool {
        if stack.is_empty() {
            return false;
        }
        if !(stack.is_item(mcrust::net::minecraft::world::item::items::stained_glass_pane().red())) || stack.get_count() != (compat::read(__this, |__t| __t.current_num)).wrapping_add(1) {
            return ({ let __f = compat::dispatch(__this, |__t| __t.should_block_incorrect_clicks()); __f(&compat::view(__this, |a| a as compat::Shared<dyn TerminalSolver>)) });
        }
        return false;
    }

    /// OrderTerminal.onClickSlot as the dispatch of ContainerSolver.onClickSlot
    pub fn on_click_slot_dyn(__this: &compat::Shared<dyn ContainerSolver>, __p0: i32, __p1: &mcrust::net::minecraft::world::item::item_stack::ItemStack, __p2: i32, __p3: i32) -> bool {
        Self::on_click_slot_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2, __p3)
    }

}

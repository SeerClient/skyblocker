//! Ported from de.hysky.skyblocker.skyblock.dungeon.terminal.ColorTerminal by fabric2seer.

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
use mcrust::java::text::decimal_format_symbols::Locale;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::world::item::dye_color::DyeColor;
use mcrust::net::minecraft::world::item::item::Item;
use mcrust::net::minecraft::world::item::items;
use mcrust::net::minecraft::core::registry::Registry as _;

/// java: private static final Map<String, DyeColor> colorFromName;
pub static COLOR_FROM_NAME: compat::MainThread<std::collections::HashMap<String, DyeColor>> = compat::MainThread::new(|| {
    let mut value;
    value = std::collections::HashMap::new();
    value
});
/// java: private static final Map<Item, DyeColor> itemColor;
/// Keyed by the item OBJECT (Java's `Item` has identity equality; the port's `Item` has no `Hash`/`Eq`): pairs
/// looked up with `Arc::ptr_eq` (`item_color`). A later `put` of the same item replaces its value, as `Map.put`.
pub static ITEM_COLOR: compat::MainThread<Vec<(Arc<Item>, DyeColor)>> = compat::MainThread::new(|| Vec::new());

/// java: public final class ColorTerminal extends SimpleContainerSolver implements TerminalSolver, StackDisplayModifier
#[derive(Clone)]
pub struct ColorTerminal {
    /// java: extends SimpleContainerSolver
    pub base: SimpleContainerSolverBase,
    /// java: private @Nullable DyeColor targetColor;
    pub target_color: Option<DyeColor>,
}

impl std::ops::Deref for ColorTerminal {
    type Target = SimpleContainerSolverBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for ColorTerminal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl ColorTerminal {
    /// java: static { .. } - Java runs it when the class is first used; every static method and
    /// constructor calls this first.
    fn clinit() {
        static ONCE: std::sync::Once = std::sync::Once::new();
        ONCE.call_once(|| {
            for color in mcrust::net::minecraft::world::item::dye_color::VALUES {
                COLOR_FROM_NAME.get_mut().insert(color.get_name().to_uppercase(), color);
            }
            COLOR_FROM_NAME.get_mut().insert("SILVER".to_string(), DyeColor::LightGray);
            COLOR_FROM_NAME.get_mut().insert("LIGHT BLUE".to_string(), DyeColor::LightBlue);
            *ITEM_COLOR.get_mut() = Vec::new();
            for color in mcrust::net::minecraft::world::item::dye_color::VALUES {
                for item in vec!["dye".to_string(), "wool".to_string(), "stained_glass".to_string(), "terracotta".to_string()] {
                    // java: BuiltInRegistries.ITEM.getValue(Identifier.withDefaultNamespace(color.getName() + "_" + item)) - the
                    // registry's typed table, by the same id
                    ColorTerminal::put_item_color(items::items().get(&format!("{}_{}", color.get_name(), item)), color);
                }
            }
            ColorTerminal::put_item_color(items::items().get("bone_meal"), DyeColor::White);
            ColorTerminal::put_item_color(items::items().get("lapis_lazuli"), DyeColor::Blue);
            ColorTerminal::put_item_color(items::items().get("cocoa_beans"), DyeColor::Brown);
            ColorTerminal::put_item_color(items::items().get("ink_sac"), DyeColor::Black);
        });
    }

    /// `itemColor.put(item, color)`.
    fn put_item_color(item: Arc<Item>, color: DyeColor) {
        let map = ITEM_COLOR.get_mut();
        match map.iter().position(|(i, _)| Arc::ptr_eq(i, &item)) {
            Some(index) => map[index].1 = color,
            None => map.push((item, color)),
        }
    }

    /// `itemColor.get(item)` - `null` for an item that is not a colour.
    fn item_color(item: &Arc<Item>) -> Option<DyeColor> {
        ITEM_COLOR.get().iter().find(|(i, _)| Arc::ptr_eq(i, item)).map(|(_, color)| *color)
    }

    /// java: public ColorTerminal()
    pub fn new() -> compat::Shared<Self> {
        Self::clinit();
        let (_, __this) = SimpleContainerSolverBase::new_in("^Select all the ([A-Z ]+) items!$".to_string(), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                target_color: None,
            });
            ((__h.clone() as compat::Shared<dyn SimpleContainerSolver>), __h)
        });
        __this
    }

}

impl SimpleContainerSolver for ColorTerminal {
    fn simple_container_solver_base(&self) -> &SimpleContainerSolverBase {
        &self.base
    }
    fn simple_container_solver_base_mut(&mut self) -> &mut SimpleContainerSolverBase {
        &mut self.base
    }

}

impl RegexContainerMatcher for ColorTerminal {
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

impl ContainerMatcher for ColorTerminal {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public boolean isEnabled() (dispatch: the body for this class)
    fn is_enabled(&self) -> fn(&compat::Shared<dyn ContainerMatcher>) -> bool {
        ColorTerminal::is_enabled_dyn
    }

    /// java: @Override public boolean test(Screen screen) (dispatch: the body for this class)
    fn test(&self) -> fn(&compat::Shared<dyn ContainerMatcher>, &mcrust::net::minecraft::client::gui::screens::screen::Screen) -> bool {
        <dyn RegexContainerMatcher>::test_from::<Self>
    }

}

impl ContainerSolver for ColorTerminal {
    /// java: @Override public List<ColorHighlight> getColors(Int2ObjectMap<ItemStack> slots) (dispatch: the body for this class)
    fn get_colors(&self) -> fn(&compat::Shared<dyn ContainerSolver>, &mut std::collections::BTreeMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> Vec<ColorHighlight> {
        ColorTerminal::get_colors_dyn
    }

    /// java: @Override public boolean onClickSlot(int slot, ItemStack stack, int screenId, int button) (dispatch: the body for this class)
    fn on_click_slot(&self) -> fn(&compat::Shared<dyn ContainerSolver>, i32, &mcrust::net::minecraft::world::item::item_stack::ItemStack, i32, i32) -> bool {
        ColorTerminal::on_click_slot_dyn
    }

}

impl Resettable for ColorTerminal {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override default void reset() (dispatch: the body for this class)
    fn reset(&self) -> fn(&compat::Shared<dyn Resettable>) {
        <dyn ContainerSolver>::reset_from::<Self>
    }

}

impl TerminalSolver for ColorTerminal {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}

impl StackDisplayModifier for ColorTerminal {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public ItemStack modifyDisplayStack(int slotIndex, ItemStack stack) (dispatch: the body for this class)
    fn modify_display_stack(&self) -> fn(&compat::Shared<dyn StackDisplayModifier>, i32, &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> mcrust::net::minecraft::world::item::item_stack::ItemStack {
        ColorTerminal::modify_display_stack_dyn
    }

}

impl ColorTerminal {
    /// java: @Override public boolean isEnabled()
    pub fn is_enabled_impl(__this: &compat::Shared<Self>) -> bool {
        { let __v = None; compat::write(__this, |__t| __t.target_color = __v); }
        return SkyblockerConfigManager::get().dungeons.terminals.solve_color;
    }

    /// ColorTerminal.isEnabled as the dispatch of ContainerMatcher.isEnabled
    pub fn is_enabled_dyn(__this: &compat::Shared<dyn ContainerMatcher>) -> bool {
        Self::is_enabled_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public List<ColorHighlight> getColors(Int2ObjectMap<ItemStack> slots)
    pub fn get_colors_impl(__this: &compat::Shared<Self>, slots: &mut std::collections::BTreeMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> Vec<ColorHighlight> {
        container_solver::trim_edges(slots, 6);
        let mut color_string: String = __this.lock().borrow().groups.as_ref().unwrap()[(0) as usize].clone();
        if compat::read(__this, |__t| __t.target_color.clone()).is_none() {
            { let __v = COLOR_FROM_NAME.get_mut().get(&color_string).cloned(); compat::write(__this, |__t| __t.target_color = __v); }
            if compat::read(__this, |__t| __t.target_color.clone()).is_none() {
                logger().error(&format!("[Skyblocker] Couldn't find dye color corresponding to \"{}\"", color_string));
                return Default::default();
            }
        }
        let mut highlights: Vec<ColorHighlight> = Vec::new();
        for slot in slots.iter().map(|(k, v)| (k.clone(), v.clone())) {
            let item_stack = &slot.1;
            if !item_stack.has_foil() && compat::read(__this, |__t| __t.target_color.clone()) == ColorTerminal::item_color(&item_stack.get_item()) {
                highlights.push(ColorHighlight::green(slot.0));
            }
        }
        return highlights;
    }

    /// ColorTerminal.getColors as the dispatch of ContainerSolver.getColors
    pub fn get_colors_dyn(__this: &compat::Shared<dyn ContainerSolver>, __p0: &mut std::collections::BTreeMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> Vec<ColorHighlight> {
        Self::get_colors_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public boolean onClickSlot(int slot, ItemStack stack, int screenId, int button)
    pub fn on_click_slot_impl(__this: &compat::Shared<Self>, mut slot: i32, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, mut screen_id: i32, mut button: i32) -> bool {
        if stack.has_foil() || !(compat::read(__this, |__t| __t.target_color.clone()) == ColorTerminal::item_color(&stack.get_item())) {
            return ({ let __f = compat::dispatch(__this, |__t| __t.should_block_incorrect_clicks()); __f(&compat::view(__this, |a| a as compat::Shared<dyn TerminalSolver>)) });
        }
        return false;
    }

    /// ColorTerminal.onClickSlot as the dispatch of ContainerSolver.onClickSlot
    pub fn on_click_slot_dyn(__this: &compat::Shared<dyn ContainerSolver>, __p0: i32, __p1: &mcrust::net::minecraft::world::item::item_stack::ItemStack, __p2: i32, __p3: i32) -> bool {
        Self::on_click_slot_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2, __p3)
    }

    /// java: @Override public ItemStack modifyDisplayStack(int slotIndex, ItemStack stack)
    pub fn modify_display_stack_impl(__this: &compat::Shared<Self>, mut slot_index: i32, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> mcrust::net::minecraft::world::item::item_stack::ItemStack {
        return (if slot_index >= 54 || compat::read(__this, |__t| __t.target_color.clone()).is_some() && compat::read(__this, |__t| __t.target_color.clone()) == ColorTerminal::item_color(&stack.get_item()) { stack.clone() } else { mcrust::net::minecraft::world::item::item_stack::empty() });
    }

    /// ColorTerminal.modifyDisplayStack as the dispatch of StackDisplayModifier.modifyDisplayStack
    pub fn modify_display_stack_dyn(__this: &compat::Shared<dyn StackDisplayModifier>, __p0: i32, __p1: &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> mcrust::net::minecraft::world::item::item_stack::ItemStack {
        Self::modify_display_stack_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1)
    }

}

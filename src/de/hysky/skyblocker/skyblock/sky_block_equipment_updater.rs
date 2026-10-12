//! Ported from de.hysky.skyblocker.skyblock.SkyBlockEquipmentUpdater by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::item::skyblock_inventory_screen;
use crate::de::hysky::skyblocker::skyblock::item::skyblock_inventory_screen::SkyblockInventoryScreen;
use crate::de::hysky::skyblocker::utils::container::container_matcher::ContainerMatcher;
use crate::de::hysky::skyblocker::utils::container::container_solver::ContainerSolver;
use crate::de::hysky::skyblocker::utils::container::regex_container_matcher::RegexContainerMatcher;
use crate::de::hysky::skyblocker::utils::container::regex_container_matcher::RegexContainerMatcherBase;
use crate::de::hysky::skyblocker::utils::container::simple_container_solver::SimpleContainerSolver;
use crate::de::hysky::skyblocker::utils::container::simple_container_solver::SimpleContainerSolverBase;
use crate::de::hysky::skyblocker::utils::render::gui::color_highlight::ColorHighlight;
use crate::de::hysky::skyblocker::utils::resettable::Resettable;
use mcrust::java::text::decimal_format_symbols::Locale;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: public class SkyBlockEquipmentUpdater extends SimpleContainerSolver
#[derive(Clone)]
pub struct SkyBlockEquipmentUpdaterBase {
    /// java: extends SimpleContainerSolver
    pub base: SimpleContainerSolverBase,
}

impl std::ops::Deref for SkyBlockEquipmentUpdaterBase {
    type Target = SimpleContainerSolverBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for SkyBlockEquipmentUpdaterBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl SkyBlockEquipmentUpdaterBase {
    /// java: public SkyBlockEquipmentUpdater() (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_in<__R>(__wrap: impl FnOnce(Self) -> (compat::Shared<dyn SkyBlockEquipmentUpdater>, __R)) -> (compat::Shared<dyn SkyBlockEquipmentUpdater>, __R) {
        let (_, (__this, __r)) = SimpleContainerSolverBase::new_in("^Stats & Equipment|(?:\\(\\d/\\d\\) )?Loadouts$".to_string(), move |__base| {
            let this = Self {
                base: __base,
            };
            let (__h, __r) = __wrap(this);
            ((__h.clone() as compat::Shared<dyn SimpleContainerSolver>), (__h, __r))
        });
        (__this, __r)
    }

    /// java: public SkyBlockEquipmentUpdater()
    pub fn new() -> compat::Shared<Self> {
        Self::new_in(|this| {
            let __h = compat::shared(this);
            ((__h.clone() as compat::Shared<dyn SkyBlockEquipmentUpdater>), __h)
        }).1
    }

    /// java: public SkyBlockEquipmentUpdater(@Language("RegExp") String titlePattern) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_1_in<__R>(mut title_pattern: String, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn SkyBlockEquipmentUpdater>, __R)) -> (compat::Shared<dyn SkyBlockEquipmentUpdater>, __R) {
        let (_, (__this, __r)) = SimpleContainerSolverBase::new_in(title_pattern.clone(), move |__base| {
            let this = Self {
                base: __base,
            };
            let (__h, __r) = __wrap(this);
            ((__h.clone() as compat::Shared<dyn SimpleContainerSolver>), (__h, __r))
        });
        (__this, __r)
    }

    /// java: public SkyBlockEquipmentUpdater(@Language("RegExp") String titlePattern)
    pub fn new_1(mut title_pattern: String) -> compat::Shared<Self> {
        Self::new_1_in(title_pattern, |this| {
            let __h = compat::shared(this);
            ((__h.clone() as compat::Shared<dyn SkyBlockEquipmentUpdater>), __h)
        }).1
    }

}

/// java: the virtual methods of SkyBlockEquipmentUpdater (implemented by SkyBlockEquipmentUpdaterBase and every subclass)
pub trait SkyBlockEquipmentUpdater: SimpleContainerSolver {
    fn sky_block_equipment_updater_base(&self) -> &SkyBlockEquipmentUpdaterBase;
    fn sky_block_equipment_updater_base_mut(&mut self) -> &mut SkyBlockEquipmentUpdaterBase;

    /// java: public ItemStack[] getEquipmentInColumn(Int2ObjectMap<ItemStack> slots, int column) (dispatch: the body for this class)
    fn get_equipment_in_column(&self) -> fn(&compat::Shared<dyn SkyBlockEquipmentUpdater>, &std::collections::HashMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack>, i32) -> Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack> {
        <dyn SkyBlockEquipmentUpdater>::get_equipment_in_column_body
    }

}

impl dyn SkyBlockEquipmentUpdater {
    /// java: public ItemStack[] getEquipmentInColumn(Int2ObjectMap<ItemStack> slots, int column)
    pub fn get_equipment_in_column_body(__this: &compat::Shared<dyn SkyBlockEquipmentUpdater>, slots: &std::collections::HashMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack>, mut column: i32) -> Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack> {
        return slots.iter().map(|(k, v)| (k.clone(), v.clone())).filter(|entry| (entry.0.wrapping_rem(9) == column)).filter(|entry| !(entry.1.is_item(mcrust::net::minecraft::world::item::items::stained_glass_pane().black()))).map(|entry| {
            let mut name: String = compat::trim(&compat::component_of(entry.1.get_hover_name()).get_string()).to_lowercase();
            let mut is_empty: bool = (name.starts_with("empty") || name.starts_with("slot "));
            if !is_empty {
                return entry.1.clone();
            }
            return mcrust::net::minecraft::world::item::item_stack::empty();
        }).collect::<Vec<_>>();
    }

    /// java: @Override public List<ColorHighlight> getColors(Int2ObjectMap<ItemStack> slots)
    pub fn get_colors_body(__this: &compat::Shared<dyn SkyBlockEquipmentUpdater>, slots: &mut std::collections::HashMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> Vec<ColorHighlight> {
        let mut equipment: Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack> = ({ let __f = compat::dispatch(__this, |__t| __t.get_equipment_in_column()); __f(__this, slots, 1) });
        if (equipment.len() as i32) < 4 {
            return vec![];
        }
        let mut destination: Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack> = SkyblockInventoryScreen::get_current_equipment_set();
        destination[(0) as usize..((0) + (4)) as usize].clone_from_slice(&equipment[(0) as usize..((0) + (4)) as usize]);
        return vec![];
    }

    /// SkyBlockEquipmentUpdater.getColors as the dispatch of ContainerSolver.getColors for the concrete class `__C`
    pub fn get_colors_from<__C: SkyBlockEquipmentUpdater + 'static>(__this: &compat::Shared<dyn ContainerSolver>, __p0: &mut std::collections::HashMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> Vec<ColorHighlight> {
        <dyn SkyBlockEquipmentUpdater>::get_colors_body(&compat::view_as::<__C, _, _>(__this, |a| a as compat::Shared<dyn SkyBlockEquipmentUpdater>), __p0)
    }

    /// java: @Override public boolean isEnabled()
    pub fn is_enabled_body(__this: &compat::Shared<dyn SkyBlockEquipmentUpdater>) -> bool {
        return SkyblockerConfigManager::get().ui_and_visuals.show_equipment_in_inventory;
    }

    /// SkyBlockEquipmentUpdater.isEnabled as the dispatch of ContainerMatcher.isEnabled for the concrete class `__C`
    pub fn is_enabled_from<__C: SkyBlockEquipmentUpdater + 'static>(__this: &compat::Shared<dyn ContainerMatcher>) -> bool {
        <dyn SkyBlockEquipmentUpdater>::is_enabled_body(&compat::view_as::<__C, _, _>(__this, |a| a as compat::Shared<dyn SkyBlockEquipmentUpdater>))
    }

}

impl SkyBlockEquipmentUpdater for SkyBlockEquipmentUpdaterBase {
    fn sky_block_equipment_updater_base(&self) -> &SkyBlockEquipmentUpdaterBase {
        self
    }
    fn sky_block_equipment_updater_base_mut(&mut self) -> &mut SkyBlockEquipmentUpdaterBase {
        self
    }

}

impl SimpleContainerSolver for SkyBlockEquipmentUpdaterBase {
    fn simple_container_solver_base(&self) -> &SimpleContainerSolverBase {
        &self.base
    }
    fn simple_container_solver_base_mut(&mut self) -> &mut SimpleContainerSolverBase {
        &mut self.base
    }

}

impl RegexContainerMatcher for SkyBlockEquipmentUpdaterBase {
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

impl ContainerMatcher for SkyBlockEquipmentUpdaterBase {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public boolean isEnabled() (dispatch: the body for this class)
    fn is_enabled(&self) -> fn(&compat::Shared<dyn ContainerMatcher>) -> bool {
        <dyn SkyBlockEquipmentUpdater>::is_enabled_from::<Self>
    }

    /// java: @Override public boolean test(Screen screen) (dispatch: the body for this class)
    fn test(&self) -> fn(&compat::Shared<dyn ContainerMatcher>, &mcrust::net::minecraft::client::gui::screens::screen::Screen) -> bool {
        <dyn RegexContainerMatcher>::test_from::<Self>
    }

}

impl ContainerSolver for SkyBlockEquipmentUpdaterBase {
    /// java: @Override public List<ColorHighlight> getColors(Int2ObjectMap<ItemStack> slots) (dispatch: the body for this class)
    fn get_colors(&self) -> fn(&compat::Shared<dyn ContainerSolver>, &mut std::collections::HashMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> Vec<ColorHighlight> {
        <dyn SkyBlockEquipmentUpdater>::get_colors_from::<Self>
    }

}

impl Resettable for SkyBlockEquipmentUpdaterBase {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override default void reset() (dispatch: the body for this class)
    fn reset(&self) -> fn(&compat::Shared<dyn Resettable>) {
        <dyn ContainerSolver>::reset_from::<Self>
    }

}

/// java: public static final class EquipmentWardrobe extends SkyBlockEquipmentUpdater
#[derive(Clone)]
pub struct EquipmentWardrobe {
    /// java: extends SkyBlockEquipmentUpdater
    pub base: SkyBlockEquipmentUpdaterBase,
}

impl std::ops::Deref for EquipmentWardrobe {
    type Target = SkyBlockEquipmentUpdaterBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for EquipmentWardrobe {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl EquipmentWardrobe {
    /// java: public EquipmentWardrobe()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = SkyBlockEquipmentUpdaterBase::new_1_in("^(?:\\(\\d/\\d\\) )?Equipment Sets$".to_string(), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
            });
            ((__h.clone() as compat::Shared<dyn SkyBlockEquipmentUpdater>), __h)
        });
        __this
    }

}

impl SkyBlockEquipmentUpdater for EquipmentWardrobe {
    fn sky_block_equipment_updater_base(&self) -> &SkyBlockEquipmentUpdaterBase {
        &self.base
    }
    fn sky_block_equipment_updater_base_mut(&mut self) -> &mut SkyBlockEquipmentUpdaterBase {
        &mut self.base
    }

}

impl SimpleContainerSolver for EquipmentWardrobe {
    fn simple_container_solver_base(&self) -> &SimpleContainerSolverBase {
        &self.base.base
    }
    fn simple_container_solver_base_mut(&mut self) -> &mut SimpleContainerSolverBase {
        &mut self.base.base
    }

}

impl RegexContainerMatcher for EquipmentWardrobe {
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

impl ContainerMatcher for EquipmentWardrobe {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public boolean test(Screen screen) (dispatch: the body for this class)
    fn test(&self) -> fn(&compat::Shared<dyn ContainerMatcher>, &mcrust::net::minecraft::client::gui::screens::screen::Screen) -> bool {
        <dyn RegexContainerMatcher>::test_from::<Self>
    }

    /// java: @Override public boolean isEnabled() (dispatch: the body for this class)
    fn is_enabled(&self) -> fn(&compat::Shared<dyn ContainerMatcher>) -> bool {
        <dyn SkyBlockEquipmentUpdater>::is_enabled_from::<Self>
    }

}

impl ContainerSolver for EquipmentWardrobe {
    /// java: @Override public List<ColorHighlight> getColors(Int2ObjectMap<ItemStack> slots) (dispatch: the body for this class)
    fn get_colors(&self) -> fn(&compat::Shared<dyn ContainerSolver>, &mut std::collections::HashMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> Vec<ColorHighlight> {
        EquipmentWardrobe::get_colors_dyn
    }

    /// java: @Override public boolean onClickSlot(int slot, ItemStack stack, int screenId, int button) (dispatch: the body for this class)
    fn on_click_slot(&self) -> fn(&compat::Shared<dyn ContainerSolver>, i32, &mcrust::net::minecraft::world::item::item_stack::ItemStack, i32, i32) -> bool {
        EquipmentWardrobe::on_click_slot_dyn
    }

}

impl Resettable for EquipmentWardrobe {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override default void reset() (dispatch: the body for this class)
    fn reset(&self) -> fn(&compat::Shared<dyn Resettable>) {
        <dyn ContainerSolver>::reset_from::<Self>
    }

}

impl EquipmentWardrobe {
    /// java: @Override public List<ColorHighlight> getColors(Int2ObjectMap<ItemStack> slots)
    pub fn get_colors_impl(__this: &compat::Shared<Self>, slots: &mut std::collections::HashMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> Vec<ColorHighlight> {
        let mut selected_set: Option<i32> = slots.iter().map(|(k, v)| (k.clone(), v.clone())).filter(|entry| (entry.0 > 35 && entry.0 < 45)).filter(|entry| entry.1.is_item(mcrust::net::minecraft::world::item::items::dye().lime())).map(|entry| entry.0.wrapping_rem(9)).next();
        if selected_set.is_none() {
            return vec![];
        }
        let mut equipment: Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack> = ({ let __f = compat::dispatch(__this, |__t| __t.get_equipment_in_column()); __f(&compat::view(__this, |a| a as compat::Shared<dyn SkyBlockEquipmentUpdater>), slots, selected_set.clone().unwrap()) });
        if (equipment.len() as i32) < 4 {
            return vec![];
        }
        skyblock_inventory_screen::EQUIPMENT.get_mut()[(0) as usize..((0) + (4)) as usize].clone_from_slice(&equipment[(0) as usize..((0) + (4)) as usize]);
        return vec![];
    }

    /// EquipmentWardrobe.getColors as the dispatch of ContainerSolver.getColors
    pub fn get_colors_dyn(__this: &compat::Shared<dyn ContainerSolver>, __p0: &mut std::collections::HashMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> Vec<ColorHighlight> {
        Self::get_colors_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public boolean onClickSlot(int slot, ItemStack stack, int screenId, int button)
    pub fn on_click_slot_impl(__this: &compat::Shared<Self>, mut slot: i32, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, mut screen_id: i32, mut button: i32) -> bool {
        if stack.is_item(mcrust::net::minecraft::world::item::items::dye().lime()) && slot > 35 && slot < 45 {
            skyblock_inventory_screen::EQUIPMENT.get_mut().fill(mcrust::net::minecraft::world::item::item_stack::empty());
        }
        return false;
    }

    /// EquipmentWardrobe.onClickSlot as the dispatch of ContainerSolver.onClickSlot
    pub fn on_click_slot_dyn(__this: &compat::Shared<dyn ContainerSolver>, __p0: i32, __p1: &mcrust::net::minecraft::world::item::item_stack::ItemStack, __p2: i32, __p3: i32) -> bool {
        Self::on_click_slot_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2, __p3)
    }

}

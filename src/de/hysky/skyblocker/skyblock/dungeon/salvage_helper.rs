//! Ported from de.hysky.skyblocker.skyblock.dungeon.SalvageHelper by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::museum::museum_item_cache::MuseumItemCache;
use crate::de::hysky::skyblocker::utils::container::container_matcher::ContainerMatcher;
use crate::de::hysky::skyblocker::utils::container::container_solver::ContainerSolver;
use crate::de::hysky::skyblocker::utils::container::regex_container_matcher::RegexContainerMatcher;
use crate::de::hysky::skyblocker::utils::container::regex_container_matcher::RegexContainerMatcherBase;
use crate::de::hysky::skyblocker::utils::container::simple_container_solver::SimpleContainerSolver;
use crate::de::hysky::skyblocker::utils::container::simple_container_solver::SimpleContainerSolverBase;
use crate::de::hysky::skyblocker::utils::item_utils::ItemUtils;
use crate::de::hysky::skyblocker::utils::networth::networth_calculator::NetworthCalculator;
use crate::de::hysky::skyblocker::utils::render::gui::color_highlight::ColorHighlight;
use crate::de::hysky::skyblocker::utils::resettable::Resettable;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;

/// java: private static final Pattern DUNGEON_SALVAGABLE = Pattern.compile("DUNGEON(?! ITEM)");
pub static DUNGEON_SALVAGABLE: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("DUNGEON(?! ITEM)"));

/// java: public class SalvageHelper extends SimpleContainerSolver
#[derive(Clone)]
pub struct SalvageHelper {
    /// java: extends SimpleContainerSolver
    pub base: SimpleContainerSolverBase,
}

impl std::ops::Deref for SalvageHelper {
    type Target = SimpleContainerSolverBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for SalvageHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl SalvageHelper {
    /// java: public SalvageHelper()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = SimpleContainerSolverBase::new_in("^Salvage Items".to_string(), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
            });
            ((__h.clone() as compat::Shared<dyn SimpleContainerSolver>), __h)
        });
        __this
    }

    /// java: private boolean isPriceWithinRange(ItemStack stack)
    pub fn is_price_within_range(__this: &compat::Shared<Self>, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> bool {
        let mut result: crate::net::azureaaron::networth::networth_result::NetworthResult = NetworthCalculator::get_item_networth(stack);
        //   java: return result.price() > 0 && result.price() < 100_000;
        return result.price() > 0.0 && result.price() < 100_000.0;
    }

    /// java: private boolean wasDonatedToMuseum(ItemStack stack)
    pub fn was_donated_to_museum(__this: &compat::Shared<Self>, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> bool {
        if !(SkyblockerConfigManager::get().dungeons.only_highlight_donated_items) {
            return true;
        }
        let mut item_id: String = stack.get_skyblock_id();
        return MuseumItemCache::has_item_in_museum(&item_id);
    }

}

impl SimpleContainerSolver for SalvageHelper {
    fn simple_container_solver_base(&self) -> &SimpleContainerSolverBase {
        &self.base
    }
    fn simple_container_solver_base_mut(&mut self) -> &mut SimpleContainerSolverBase {
        &mut self.base
    }

}

impl RegexContainerMatcher for SalvageHelper {
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

impl ContainerMatcher for SalvageHelper {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public boolean isEnabled() (dispatch: the body for this class)
    fn is_enabled(&self) -> fn(&compat::Shared<dyn ContainerMatcher>) -> bool {
        SalvageHelper::is_enabled_dyn
    }

    /// java: @Override public boolean test(Screen screen) (dispatch: the body for this class)
    fn test(&self) -> fn(&compat::Shared<dyn ContainerMatcher>, &mcrust::net::minecraft::client::gui::screens::screen::Screen) -> bool {
        <dyn RegexContainerMatcher>::test_from::<Self>
    }

}

impl ContainerSolver for SalvageHelper {
    /// java: @Override public List<ColorHighlight> getColors(Int2ObjectMap<ItemStack> slots) (dispatch: the body for this class)
    fn get_colors(&self) -> fn(&compat::Shared<dyn ContainerSolver>, &mut std::collections::HashMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> Vec<ColorHighlight> {
        SalvageHelper::get_colors_dyn
    }

    /// java: @Override public boolean chestInventoryOnly() (dispatch: the body for this class)
    fn chest_inventory_only(&self) -> fn(&compat::Shared<dyn ContainerSolver>) -> bool {
        SalvageHelper::chest_inventory_only_dyn
    }

}

impl Resettable for SalvageHelper {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override default void reset() (dispatch: the body for this class)
    fn reset(&self) -> fn(&compat::Shared<dyn Resettable>) {
        <dyn ContainerSolver>::reset_from::<Self>
    }

}

impl SalvageHelper {
    /// java: @Override public boolean isEnabled()
    pub fn is_enabled_impl(__this: &compat::Shared<Self>) -> bool {
        return SkyblockerConfigManager::get().dungeons.salvage_helper;
    }

    /// SalvageHelper.isEnabled as the dispatch of ContainerMatcher.isEnabled
    pub fn is_enabled_dyn(__this: &compat::Shared<dyn ContainerMatcher>) -> bool {
        Self::is_enabled_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public List<ColorHighlight> getColors(Int2ObjectMap<ItemStack> slots)
    pub fn get_colors_impl(__this: &compat::Shared<Self>, slots: &mut std::collections::HashMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> Vec<ColorHighlight> {
        let __this_1 = compat::RenderThread::new(__this.clone());
        let __this_2 = compat::RenderThread::new(__this.clone());
        return slots.iter().map(|(k, v)| (k.clone(), v.clone())).filter(|entry| ItemUtils::get_lore_line_if_contains_match(&entry.1, &DUNGEON_SALVAGABLE).is_some()).filter(|entry| SalvageHelper::is_price_within_range(__this_1.get(), &entry.1)).filter(|entry| SalvageHelper::was_donated_to_museum(__this_2.get(), &entry.1)).map(|entry| ColorHighlight::yellow(entry.0)).collect::<Vec<_>>();
    }

    /// SalvageHelper.getColors as the dispatch of ContainerSolver.getColors
    pub fn get_colors_dyn(__this: &compat::Shared<dyn ContainerSolver>, __p0: &mut std::collections::HashMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> Vec<ColorHighlight> {
        Self::get_colors_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public boolean chestInventoryOnly()
    pub fn chest_inventory_only_impl(__this: &compat::Shared<Self>) -> bool {
        return false;
    }

    /// SalvageHelper.chestInventoryOnly as the dispatch of ContainerSolver.chestInventoryOnly
    pub fn chest_inventory_only_dyn(__this: &compat::Shared<dyn ContainerSolver>) -> bool {
        Self::chest_inventory_only_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}

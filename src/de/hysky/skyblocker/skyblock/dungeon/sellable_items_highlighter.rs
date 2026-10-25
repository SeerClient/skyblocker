//! Ported from de.hysky.skyblocker.skyblock.dungeon.SellableItemsHighlighter by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::utils::container::container_matcher::ContainerMatcher;
use crate::de::hysky::skyblocker::utils::container::container_solver::ContainerSolver;
use crate::de::hysky::skyblocker::utils::container::regex_container_matcher::RegexContainerMatcher;
use crate::de::hysky::skyblocker::utils::container::regex_container_matcher::RegexContainerMatcherBase;
use crate::de::hysky::skyblocker::utils::container::simple_container_solver::SimpleContainerSolver;
use crate::de::hysky::skyblocker::utils::container::simple_container_solver::SimpleContainerSolverBase;
use crate::de::hysky::skyblocker::utils::render::gui::color_highlight::ColorHighlight;
use crate::de::hysky::skyblocker::utils::resettable::Resettable;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final Set<String> ITEM_IDS = Set.of( "DEFUSE_KIT", "TRAINING_WEIGHTS", "DUNGEON_LORE_PAPER", "REVIVE_STONE" );
pub static ITEM_IDS: LazyLock<std::collections::HashSet<String>> = LazyLock::new(|| std::collections::HashSet::from(["DEFUSE_KIT".to_string(), "TRAINING_WEIGHTS".to_string(), "DUNGEON_LORE_PAPER".to_string(), "REVIVE_STONE".to_string()]));
/// java: private static final Set<String> POTION_ITEM_NAMES = Set.of( "Healing VIII Splash Potion" // todo: check if need to also add without roman numeral );
pub static POTION_ITEM_NAMES: LazyLock<std::collections::HashSet<String>> = LazyLock::new(|| std::collections::HashSet::from(["Healing VIII Splash Potion".to_string()]));

/// java: public class SellableItemsHighlighter extends SimpleContainerSolver
#[derive(Clone)]
pub struct SellableItemsHighlighter {
    /// java: extends SimpleContainerSolver
    pub base: SimpleContainerSolverBase,
}

impl std::ops::Deref for SellableItemsHighlighter {
    type Target = SimpleContainerSolverBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for SellableItemsHighlighter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl SellableItemsHighlighter {
    /// java: public SellableItemsHighlighter()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = SimpleContainerSolverBase::new_in("^(Ophelia|Booster Cookie)$".to_string(), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
            });
            ((__h.clone() as compat::Shared<dyn SimpleContainerSolver>), __h)
        });
        __this
    }

    /// java: private boolean isValidItem(ItemStack stack)
    pub fn is_valid_item(__this: &compat::Shared<Self>, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> bool {
        let mut skyblock_id: String = stack.get_skyblock_id();
        if skyblock_id == "POTION" {
            let mut display_name: String = compat::component_of(stack.get_hover_name()).get_string();
            return POTION_ITEM_NAMES.iter().cloned().any(|__a0| display_name.contains(&__a0));
        }
        return ITEM_IDS.iter().cloned().any(|__a0| (skyblock_id == __a0));
    }

}

impl SimpleContainerSolver for SellableItemsHighlighter {
    fn simple_container_solver_base(&self) -> &SimpleContainerSolverBase {
        &self.base
    }
    fn simple_container_solver_base_mut(&mut self) -> &mut SimpleContainerSolverBase {
        &mut self.base
    }

}

impl RegexContainerMatcher for SellableItemsHighlighter {
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

impl ContainerMatcher for SellableItemsHighlighter {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public boolean isEnabled() (dispatch: the body for this class)
    fn is_enabled(&self) -> fn(&compat::Shared<dyn ContainerMatcher>) -> bool {
        SellableItemsHighlighter::is_enabled_dyn
    }

    /// java: @Override public boolean test(Screen screen) (dispatch: the body for this class)
    fn test(&self) -> fn(&compat::Shared<dyn ContainerMatcher>, &mcrust::net::minecraft::client::gui::screens::screen::Screen) -> bool {
        <dyn RegexContainerMatcher>::test_from::<Self>
    }

}

impl ContainerSolver for SellableItemsHighlighter {
    /// java: @Override public List<ColorHighlight> getColors(Int2ObjectMap<ItemStack> slots) (dispatch: the body for this class)
    fn get_colors(&self) -> fn(&compat::Shared<dyn ContainerSolver>, &mut std::collections::BTreeMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> Vec<ColorHighlight> {
        SellableItemsHighlighter::get_colors_dyn
    }

    /// java: @Override public boolean chestInventoryOnly() (dispatch: the body for this class)
    fn chest_inventory_only(&self) -> fn(&compat::Shared<dyn ContainerSolver>) -> bool {
        SellableItemsHighlighter::chest_inventory_only_dyn
    }

}

impl Resettable for SellableItemsHighlighter {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override default void reset() (dispatch: the body for this class)
    fn reset(&self) -> fn(&compat::Shared<dyn Resettable>) {
        <dyn ContainerSolver>::reset_from::<Self>
    }

}

impl SellableItemsHighlighter {
    /// java: @Override public boolean isEnabled()
    pub fn is_enabled_impl(__this: &compat::Shared<Self>) -> bool {
        return SkyblockerConfigManager::get().dungeons.sellable_items_highlighter;
    }

    /// SellableItemsHighlighter.isEnabled as the dispatch of ContainerMatcher.isEnabled
    pub fn is_enabled_dyn(__this: &compat::Shared<dyn ContainerMatcher>) -> bool {
        Self::is_enabled_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public List<ColorHighlight> getColors(Int2ObjectMap<ItemStack> slots)
    pub fn get_colors_impl(__this: &compat::Shared<Self>, slots: &mut std::collections::BTreeMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> Vec<ColorHighlight> {
        let __this_1 = compat::RenderThread::new(__this.clone());
        return slots.iter().map(|(k, v)| (k.clone(), v.clone())).filter(|entry| (entry.0 > 53 && entry.0 < 81)).filter(|entry| SellableItemsHighlighter::is_valid_item(__this_1.get(), &entry.1)).map(|entry| ColorHighlight::yellow(entry.0)).collect::<Vec<_>>();
    }

    /// SellableItemsHighlighter.getColors as the dispatch of ContainerSolver.getColors
    pub fn get_colors_dyn(__this: &compat::Shared<dyn ContainerSolver>, __p0: &mut std::collections::BTreeMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> Vec<ColorHighlight> {
        Self::get_colors_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public boolean chestInventoryOnly()
    pub fn chest_inventory_only_impl(__this: &compat::Shared<Self>) -> bool {
        return false;
    }

    /// SellableItemsHighlighter.chestInventoryOnly as the dispatch of ContainerSolver.chestInventoryOnly
    pub fn chest_inventory_only_dyn(__this: &compat::Shared<dyn ContainerSolver>) -> bool {
        Self::chest_inventory_only_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}

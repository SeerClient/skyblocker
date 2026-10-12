//! Ported from de.hysky.skyblocker.skyblock.item.AnvilHelper by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::utils::container::container_matcher::ContainerMatcher;
use crate::de::hysky::skyblocker::utils::container::container_solver::ContainerSolver;
use crate::de::hysky::skyblocker::utils::container::regex_container_matcher::RegexContainerMatcher;
use crate::de::hysky::skyblocker::utils::container::regex_container_matcher::RegexContainerMatcherBase;
use crate::de::hysky::skyblocker::utils::container::simple_container_solver::SimpleContainerSolver;
use crate::de::hysky::skyblocker::utils::container::simple_container_solver::SimpleContainerSolverBase;
use crate::de::hysky::skyblocker::utils::item_utils::ItemUtils;
use crate::de::hysky::skyblocker::utils::render::gui::color_highlight::ColorHighlight;
use crate::de::hysky::skyblocker::utils::resettable::Resettable;
use mcrust::net::minecraft::nbt::compound_tag;
use mcrust::net::minecraft::nbt::tag::Tag;
use mcrust::net::minecraft::world::item::items;

/// java: public class AnvilHelper extends SimpleContainerSolver
#[derive(Clone)]
pub struct AnvilHelper {
    /// java: extends SimpleContainerSolver
    pub base: SimpleContainerSolverBase,
}

impl std::ops::Deref for AnvilHelper {
    type Target = SimpleContainerSolverBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for AnvilHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl AnvilHelper {
    /// java: public AnvilHelper()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = SimpleContainerSolverBase::new_in("Anvil".to_string(), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
            });
            ((__h.clone() as compat::Shared<dyn SimpleContainerSolver>), __h)
        });
        __this
    }

    /// java: private @Nullable CompoundTag getEnchantments(ItemStack stack)
    pub fn get_enchantments(__this: &compat::Shared<Self>, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> Option<Tag> {
        let mut nbt: Tag = ItemUtils::get_custom_data(stack);
        // TODO(fabric2seer): `is_empty` takes 1 argument(s) in the port (`tags: &BTreeMap<String, Tag>`), Java passes 0
        if compound_tag::is_empty() || !(nbt.contains("enchantments")) {
            return None;
        }
        return Some(nbt.get_compound_or_empty("enchantments"));
    }

    /// java: private boolean haveUnequalEnchantments(ItemStack stack1, ItemStack stack2)
    pub fn have_unequal_enchantments(__this: &compat::Shared<Self>, stack1: &mcrust::net::minecraft::world::item::item_stack::ItemStack, stack2: &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> bool {
        let mut enchantments1: Option<Tag> = AnvilHelper::get_enchantments(__this, stack1);
        let mut enchantments2: Option<Tag> = AnvilHelper::get_enchantments(__this, stack2);
        if enchantments1.is_none() || enchantments2.is_none() {
            return false;
        }
        for enchantment_id in enchantments2.as_ref().unwrap().key_set() {
            let mut level1: i32 = enchantments1.as_ref().unwrap().get_int_or(&enchantment_id, 0);
            let mut level2: i32 = enchantments2.as_ref().unwrap().get_int_or(&enchantment_id, 0);
            if level1 != level2 {
                return true;
            }
        }
        return false;
    }

}

impl SimpleContainerSolver for AnvilHelper {
    fn simple_container_solver_base(&self) -> &SimpleContainerSolverBase {
        &self.base
    }
    fn simple_container_solver_base_mut(&mut self) -> &mut SimpleContainerSolverBase {
        &mut self.base
    }

}

impl RegexContainerMatcher for AnvilHelper {
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

impl ContainerMatcher for AnvilHelper {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public boolean isEnabled() (dispatch: the body for this class)
    fn is_enabled(&self) -> fn(&compat::Shared<dyn ContainerMatcher>) -> bool {
        AnvilHelper::is_enabled_dyn
    }

    /// java: @Override public boolean test(Screen screen) (dispatch: the body for this class)
    fn test(&self) -> fn(&compat::Shared<dyn ContainerMatcher>, &mcrust::net::minecraft::client::gui::screens::screen::Screen) -> bool {
        <dyn RegexContainerMatcher>::test_from::<Self>
    }

}

impl ContainerSolver for AnvilHelper {
    /// java: @Override public List<ColorHighlight> getColors(Int2ObjectMap<ItemStack> slots) (dispatch: the body for this class)
    fn get_colors(&self) -> fn(&compat::Shared<dyn ContainerSolver>, &mut std::collections::HashMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> Vec<ColorHighlight> {
        AnvilHelper::get_colors_dyn
    }

}

impl Resettable for AnvilHelper {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override default void reset() (dispatch: the body for this class)
    fn reset(&self) -> fn(&compat::Shared<dyn Resettable>) {
        <dyn ContainerSolver>::reset_from::<Self>
    }

}

impl AnvilHelper {
    /// java: @Override public boolean isEnabled()
    pub fn is_enabled_impl(__this: &compat::Shared<Self>) -> bool {
        return SkyblockerConfigManager::get().helpers.enable_anvil_helper;
    }

    /// AnvilHelper.isEnabled as the dispatch of ContainerMatcher.isEnabled
    pub fn is_enabled_dyn(__this: &compat::Shared<dyn ContainerMatcher>) -> bool {
        Self::is_enabled_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public List<ColorHighlight> getColors(Int2ObjectMap<ItemStack> slots)
    pub fn get_colors_impl(__this: &compat::Shared<Self>, slots: &mut std::collections::HashMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> Vec<ColorHighlight> {
        let mut stack1: mcrust::net::minecraft::world::item::item_stack::ItemStack = slots.get(&29).cloned().unwrap();
        let mut stack2: mcrust::net::minecraft::world::item::item_stack::ItemStack = slots.get(&33).cloned().unwrap();
        if !(stack1.is_item(&items::items().get("enchanted_book"))) || !(stack2.is_item(&items::items().get("enchanted_book"))) {
            return vec![];
        }
        if !(AnvilHelper::have_unequal_enchantments(__this, &stack1, &stack2)) {
            return vec![];
        }
        return vec![ColorHighlight::red(13)];
    }

    /// AnvilHelper.getColors as the dispatch of ContainerSolver.getColors
    pub fn get_colors_dyn(__this: &compat::Shared<dyn ContainerSolver>, __p0: &mut std::collections::HashMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> Vec<ColorHighlight> {
        Self::get_colors_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

}

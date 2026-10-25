//! Ported from de.hysky.skyblocker.skyblock.accessories.AccessoriesContainerSolver by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::accessories::accessories_helper;
use crate::de::hysky::skyblocker::utils::container::container_matcher::ContainerMatcher;
use crate::de::hysky::skyblocker::utils::container::container_solver::ContainerSolver;
use crate::de::hysky::skyblocker::utils::container::regex_container_matcher::RegexContainerMatcher;
use crate::de::hysky::skyblocker::utils::container::regex_container_matcher::RegexContainerMatcherBase;
use crate::de::hysky::skyblocker::utils::container::simple_container_solver::SimpleContainerSolver;
use crate::de::hysky::skyblocker::utils::container::simple_container_solver::SimpleContainerSolverBase;
use crate::de::hysky::skyblocker::utils::render::gui::color_highlight::ColorHighlight;
use crate::de::hysky::skyblocker::utils::resettable::Resettable;
use mcrust::net::minecraft::util::argb;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;

/// java: private static final int COLOR = ARGB.color(0.7f, CommonColors.GREEN);
pub static COLOR: LazyLock<i32> = LazyLock::new(|| argb::color_alpha_f32_rgb(0.7f32, mcrust::net::minecraft::util::common_colors::GREEN));
/// java: public static final AccessoriesContainerSolver INSTANCE = new AccessoriesContainerSolver();
pub static INSTANCE: compat::MainThread<compat::Shared<AccessoriesContainerSolver>> = compat::MainThread::new(|| AccessoriesContainerSolver::new());

/// java: public class AccessoriesContainerSolver extends SimpleContainerSolver
#[derive(Clone)]
pub struct AccessoriesContainerSolver {
    /// java: extends SimpleContainerSolver
    pub base: SimpleContainerSolverBase,
    /// java: @Nullable String highlightedAccessory;
    pub highlighted_accessory: Option<String>,
}

impl std::ops::Deref for AccessoriesContainerSolver {
    type Target = SimpleContainerSolverBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for AccessoriesContainerSolver {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl AccessoriesContainerSolver {
    /// java: protected AccessoriesContainerSolver()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = SimpleContainerSolverBase::new2_in(accessories_helper::ACCESSORY_BAG_TITLE.clone(), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                highlighted_accessory: None,
            });
            ((__h.clone() as compat::Shared<dyn SimpleContainerSolver>), __h)
        });
        __this
    }

}

impl SimpleContainerSolver for AccessoriesContainerSolver {
    fn simple_container_solver_base(&self) -> &SimpleContainerSolverBase {
        &self.base
    }
    fn simple_container_solver_base_mut(&mut self) -> &mut SimpleContainerSolverBase {
        &mut self.base
    }

}

impl RegexContainerMatcher for AccessoriesContainerSolver {
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

impl ContainerMatcher for AccessoriesContainerSolver {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public boolean isEnabled() (dispatch: the body for this class)
    fn is_enabled(&self) -> fn(&compat::Shared<dyn ContainerMatcher>) -> bool {
        AccessoriesContainerSolver::is_enabled_dyn
    }

    /// java: @Override public boolean test(Screen screen) (dispatch: the body for this class)
    fn test(&self) -> fn(&compat::Shared<dyn ContainerMatcher>, &mcrust::net::minecraft::client::gui::screens::screen::Screen) -> bool {
        <dyn RegexContainerMatcher>::test_from::<Self>
    }

}

impl ContainerSolver for AccessoriesContainerSolver {
    /// java: @Override public List<ColorHighlight> getColors(Int2ObjectMap<ItemStack> slots) (dispatch: the body for this class)
    fn get_colors(&self) -> fn(&compat::Shared<dyn ContainerSolver>, &mut std::collections::BTreeMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> Vec<ColorHighlight> {
        AccessoriesContainerSolver::get_colors_dyn
    }

}

impl Resettable for AccessoriesContainerSolver {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override default void reset() (dispatch: the body for this class)
    fn reset(&self) -> fn(&compat::Shared<dyn Resettable>) {
        <dyn ContainerSolver>::reset_from::<Self>
    }

}

impl AccessoriesContainerSolver {
    /// java: @Override public boolean isEnabled()
    pub fn is_enabled_impl(__this: &compat::Shared<Self>) -> bool {
        return SkyblockerConfigManager::get().helpers.enable_accessories_helper_widget;
    }

    /// AccessoriesContainerSolver.isEnabled as the dispatch of ContainerMatcher.isEnabled
    pub fn is_enabled_dyn(__this: &compat::Shared<dyn ContainerMatcher>) -> bool {
        Self::is_enabled_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public List<ColorHighlight> getColors(Int2ObjectMap<ItemStack> slots)
    pub fn get_colors_impl(__this: &compat::Shared<Self>, slots: &mut std::collections::BTreeMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> Vec<ColorHighlight> {
        if compat::read(__this, |__t| __t.highlighted_accessory.clone()).is_none() {
            return vec![];
        }
        let __this_1 = compat::RenderThread::new(__this.clone());
        return slots.iter().map(|(k, v)| (k.clone(), v.clone())).filter(|entry| (Some(&*(entry.1.get_skyblock_id())) == compat::read(__this_1.get(), |__t| __t.highlighted_accessory.clone()).as_deref())).map(|entry| ColorHighlight::new(entry.0, *COLOR)).collect::<Vec<_>>();
    }

    /// AccessoriesContainerSolver.getColors as the dispatch of ContainerSolver.getColors
    pub fn get_colors_dyn(__this: &compat::Shared<dyn ContainerSolver>, __p0: &mut std::collections::BTreeMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> Vec<ColorHighlight> {
        Self::get_colors_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

}

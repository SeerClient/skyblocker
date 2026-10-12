//! Ported from de.hysky.skyblocker.skyblock.garden.StereoHarmonyHelper by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::garden::current_jacob_crop;
use crate::de::hysky::skyblocker::skyblock::garden::garden_constants;
use crate::de::hysky::skyblocker::utils::container::container_matcher::ContainerMatcher;
use crate::de::hysky::skyblocker::utils::container::container_solver::ContainerSolver;
use crate::de::hysky::skyblocker::utils::container::regex_container_matcher::RegexContainerMatcher;
use crate::de::hysky::skyblocker::utils::container::regex_container_matcher::RegexContainerMatcherBase;
use crate::de::hysky::skyblocker::utils::container::simple_container_solver::SimpleContainerSolver;
use crate::de::hysky::skyblocker::utils::container::simple_container_solver::SimpleContainerSolverBase;
use crate::de::hysky::skyblocker::utils::item_utils::ItemUtils;
use crate::de::hysky::skyblocker::utils::regex_list_utils::RegexListUtils;
use crate::de::hysky::skyblocker::utils::render::gui::color_highlight::ColorHighlight;
use crate::de::hysky::skyblocker::utils::resettable::Resettable;
use mcrust::net::minecraft::chat_formatting;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;

/// java: private static final Pattern PEST_NAME_PATTERN = Pattern.compile("^When playing, (?<name>.+) Pests.*");
pub static PEST_NAME_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("^When playing, (?<name>.+) Pests.*"));
/// java: public static final StereoHarmonyHelper INSTANCE = new StereoHarmonyHelper();
pub static INSTANCE: compat::MainThread<compat::Shared<StereoHarmonyHelper>> = compat::MainThread::new(|| StereoHarmonyHelper::new());

/// java: public class StereoHarmonyHelper extends SimpleContainerSolver
#[derive(Clone)]
pub struct StereoHarmonyHelper {
    /// java: extends SimpleContainerSolver
    pub base: SimpleContainerSolverBase,
}

impl std::ops::Deref for StereoHarmonyHelper {
    type Target = SimpleContainerSolverBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for StereoHarmonyHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl StereoHarmonyHelper {
    /// java: private StereoHarmonyHelper()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = SimpleContainerSolverBase::new_in("Stereo Harmony".to_string(), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
            });
            ((__h.clone() as compat::Shared<dyn SimpleContainerSolver>), __h)
        });
        __this
    }

}

impl SimpleContainerSolver for StereoHarmonyHelper {
    fn simple_container_solver_base(&self) -> &SimpleContainerSolverBase {
        &self.base
    }
    fn simple_container_solver_base_mut(&mut self) -> &mut SimpleContainerSolverBase {
        &mut self.base
    }

}

impl RegexContainerMatcher for StereoHarmonyHelper {
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

impl ContainerMatcher for StereoHarmonyHelper {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public boolean isEnabled() (dispatch: the body for this class)
    fn is_enabled(&self) -> fn(&compat::Shared<dyn ContainerMatcher>) -> bool {
        StereoHarmonyHelper::is_enabled_dyn
    }

    /// java: @Override public boolean test(Screen screen) (dispatch: the body for this class)
    fn test(&self) -> fn(&compat::Shared<dyn ContainerMatcher>, &mcrust::net::minecraft::client::gui::screens::screen::Screen) -> bool {
        <dyn RegexContainerMatcher>::test_from::<Self>
    }

}

impl ContainerSolver for StereoHarmonyHelper {
    /// java: @Override public List<ColorHighlight> getColors(Int2ObjectMap<ItemStack> slots) (dispatch: the body for this class)
    fn get_colors(&self) -> fn(&compat::Shared<dyn ContainerSolver>, &mut std::collections::HashMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> Vec<ColorHighlight> {
        StereoHarmonyHelper::get_colors_dyn
    }

}

impl Resettable for StereoHarmonyHelper {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override default void reset() (dispatch: the body for this class)
    fn reset(&self) -> fn(&compat::Shared<dyn Resettable>) {
        <dyn ContainerSolver>::reset_from::<Self>
    }

}

impl StereoHarmonyHelper {
    /// java: @Override public boolean isEnabled()
    pub fn is_enabled_impl(__this: &compat::Shared<Self>) -> bool {
        return SkyblockerConfigManager::get().farming.pest_highlighter.enable_stereo_harmony_helper_for_contest;
    }

    /// StereoHarmonyHelper.isEnabled as the dispatch of ContainerMatcher.isEnabled
    pub fn is_enabled_dyn(__this: &compat::Shared<dyn ContainerMatcher>) -> bool {
        Self::is_enabled_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public List<ColorHighlight> getColors(Int2ObjectMap<ItemStack> slots)
    pub fn get_colors_impl(__this: &compat::Shared<Self>, slots: &mut std::collections::HashMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> Vec<ColorHighlight> {
        let mut highlights: Vec<ColorHighlight> = Vec::new();
        for entry in slots.iter().map(|(k, v)| (k.clone(), v.clone())) {
            // java: Matcher matcher = RegexListUtils.matchInList(entry.getValue().skyblocker$getLoreStrings(), ChatFormatting::stripFormatting, PEST_NAME_PATTERN);
            let mut matcher: Option<compat::regex::Matcher> = RegexListUtils::match_in_list_list_function_pattern(&entry.1.skyblocker_get_lore_strings(), Arc::new(move |__a0: String| chat_formatting::strip_formatting(&__a0)), &PEST_NAME_PATTERN);
            if let Some(mut matcher) = matcher {
                let mut pest_name: String = compat::trim(&matcher.group_named("name").unwrap());
                let Some(mut crop) = garden_constants::CROP_BY_PEST.get(&pest_name).cloned() else {
                    continue;
                };
                let mut is_playing: bool = ItemUtils::get_lore_line_if(&entry.1, Arc::new(move |text| text.contains("PLAYING"))).is_some();
                if Some(&*(crop)) == compat::get_state(&current_jacob_crop::CURRENT_CROP_CONTEST).as_deref() {
                    highlights.push((if is_playing { ColorHighlight::yellow(entry.0) } else { ColorHighlight::green(entry.0) }));
                }
            }
        }
        return highlights;
    }

    /// StereoHarmonyHelper.getColors as the dispatch of ContainerSolver.getColors
    pub fn get_colors_dyn(__this: &compat::Shared<dyn ContainerSolver>, __p0: &mut std::collections::HashMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> Vec<ColorHighlight> {
        Self::get_colors_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

}

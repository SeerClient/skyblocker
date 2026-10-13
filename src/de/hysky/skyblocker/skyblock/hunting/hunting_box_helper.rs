//! Ported from de.hysky.skyblocker.skyblock.hunting.HuntingBoxHelper by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::utils::container::container_matcher::ContainerMatcher;
use crate::de::hysky::skyblocker::utils::container::container_solver;
use crate::de::hysky::skyblocker::utils::container::container_solver::ContainerSolver;
use crate::de::hysky::skyblocker::utils::container::regex_container_matcher::RegexContainerMatcher;
use crate::de::hysky::skyblocker::utils::container::regex_container_matcher::RegexContainerMatcherBase;
use crate::de::hysky::skyblocker::utils::container::simple_container_solver::SimpleContainerSolver;
use crate::de::hysky::skyblocker::utils::container::simple_container_solver::SimpleContainerSolverBase;
use crate::de::hysky::skyblocker::utils::render::gui::color_highlight::ColorHighlight;
use crate::de::hysky::skyblocker::utils::resettable::Resettable;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;

/// java: public static final Pattern HUNTING_BOX_TITLE_PATTERN = Pattern.compile("^(?:\\(\\d+\\/\\d+\\) )?Hunting Box$");
pub static HUNTING_BOX_TITLE_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("^(?:\\(\\d+\\/\\d+\\) )?Hunting Box$"));
/// java: private static final Pattern OWNED_PATTERN = Pattern.compile("Owned: ([\\d,]+) Shards?");
pub static OWNED_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("Owned: ([\\d,]+) Shards?"));
/// java: private static final Pattern SYPHON_PATTERN = Pattern.compile("Syphon (\\d+) more to level up!");
pub static SYPHON_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("Syphon (\\d+) more to level up!"));

/// java: public class HuntingBoxHelper extends SimpleContainerSolver
#[derive(Clone)]
pub struct HuntingBoxHelper {
    /// java: extends SimpleContainerSolver
    pub base: SimpleContainerSolverBase,
}

impl std::ops::Deref for HuntingBoxHelper {
    type Target = SimpleContainerSolverBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for HuntingBoxHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl HuntingBoxHelper {
    /// java: public HuntingBoxHelper()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = SimpleContainerSolverBase::new2_in(HUNTING_BOX_TITLE_PATTERN.clone(), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
            });
            ((__h.clone() as compat::Shared<dyn SimpleContainerSolver>), __h)
        });
        __this
    }

}

impl SimpleContainerSolver for HuntingBoxHelper {
    fn simple_container_solver_base(&self) -> &SimpleContainerSolverBase {
        &self.base
    }
    fn simple_container_solver_base_mut(&mut self) -> &mut SimpleContainerSolverBase {
        &mut self.base
    }

}

impl RegexContainerMatcher for HuntingBoxHelper {
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

impl ContainerMatcher for HuntingBoxHelper {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public boolean isEnabled() (dispatch: the body for this class)
    fn is_enabled(&self) -> fn(&compat::Shared<dyn ContainerMatcher>) -> bool {
        HuntingBoxHelper::is_enabled_dyn
    }

    /// java: @Override public boolean test(Screen screen) (dispatch: the body for this class)
    fn test(&self) -> fn(&compat::Shared<dyn ContainerMatcher>, &mcrust::net::minecraft::client::gui::screens::screen::Screen) -> bool {
        <dyn RegexContainerMatcher>::test_from::<Self>
    }

}

impl ContainerSolver for HuntingBoxHelper {
    /// java: @Override public List<ColorHighlight> getColors(Int2ObjectMap<ItemStack> slots) (dispatch: the body for this class)
    fn get_colors(&self) -> fn(&compat::Shared<dyn ContainerSolver>, &mut std::collections::HashMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> Vec<ColorHighlight> {
        HuntingBoxHelper::get_colors_dyn
    }

}

impl Resettable for HuntingBoxHelper {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override default void reset() (dispatch: the body for this class)
    fn reset(&self) -> fn(&compat::Shared<dyn Resettable>) {
        <dyn ContainerSolver>::reset_from::<Self>
    }

}

impl HuntingBoxHelper {
    /// java: @Override public boolean isEnabled()
    pub fn is_enabled_impl(__this: &compat::Shared<Self>) -> bool {
        return SkyblockerConfigManager::get().hunting.hunting_box.enabled;
    }

    /// HuntingBoxHelper.isEnabled as the dispatch of ContainerMatcher.isEnabled
    pub fn is_enabled_dyn(__this: &compat::Shared<dyn ContainerMatcher>) -> bool {
        Self::is_enabled_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public List<ColorHighlight> getColors(Int2ObjectMap<ItemStack> slots)
    pub fn get_colors_impl(__this: &compat::Shared<Self>, slots: &mut std::collections::HashMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> Vec<ColorHighlight> {
        let mut highlights: Vec<ColorHighlight> = Vec::new();
        container_solver::trim_edges(slots, 6);
        for entry in slots.iter().map(|(k, v)| (k.clone(), v.clone())) {
            let stack = &entry.1;
            if stack.is_empty() {
                continue;
            }
            let mut lore: Vec<String> = stack.skyblocker_get_lore_strings();
            if lore.is_empty() {
                continue;
            }
            let mut owned: Option<String> = None;
            let mut syphon: Option<String> = None;
            for text in lore.iter() {
                if owned.is_none() {
                    let mut matcher: compat::regex::Matcher = OWNED_PATTERN.matcher(text);
                    if matcher.matches() {
                        owned = Some(matcher.group(1).unwrap().replace(",", ""));
                    }
                } else {
                    let mut matcher: compat::regex::Matcher = SYPHON_PATTERN.matcher(text);
                    if matcher.matches() {
                        syphon = matcher.group(1);
                    }
                    break;
                }
            }
            if owned.is_none() || syphon.is_none() {
                continue;
            }
            // `NumberUtils.toInt(str, -1)`: `Integer.parseInt(str)` (no trimming), the default for null or a bad number
            //   java: int ownedCount = NumberUtils.toInt(owned, -1);
            let mut owned_count: i32 = owned.as_deref().and_then(|__s| __s.parse::<i32>().ok()).unwrap_or(-1);
            //   java: int syphonCount = NumberUtils.toInt(syphon, -1);
            let mut syphon_count: i32 = syphon.as_deref().and_then(|__s| __s.parse::<i32>().ok()).unwrap_or(-1);
            if owned_count < 0 || syphon_count < 0 {
                logger().warn(&format!("Invalid owned or syphon count in Hunting Box: owned={}, syphon={}.", compat::opt_str(&owned), compat::opt_str(&syphon)));
                continue;
            }
            if owned_count >= syphon_count {
                let mut enough_but_not_unlocked: bool = (lore[lore.len() - 1]).starts_with("Requires");
                highlights.push((if enough_but_not_unlocked { ColorHighlight::yellow(entry.0) } else { ColorHighlight::green(entry.0) }));
            }
        }
        return highlights;
    }

    /// HuntingBoxHelper.getColors as the dispatch of ContainerSolver.getColors
    pub fn get_colors_dyn(__this: &compat::Shared<dyn ContainerSolver>, __p0: &mut std::collections::HashMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> Vec<ColorHighlight> {
        Self::get_colors_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

}

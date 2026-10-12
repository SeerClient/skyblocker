//! Ported from de.hysky.skyblocker.skyblock.end.EndStatsBestiaryUpdater by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::end::end_hud_widget::EndHudWidget;
use crate::de::hysky::skyblocker::skyblock::end::the_end;
use crate::de::hysky::skyblocker::skyblock::end::the_end::EndStats;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element_based_widget::ElementBasedWidget;
use crate::de::hysky::skyblocker::utils::container::container_matcher::ContainerMatcher;
use crate::de::hysky::skyblocker::utils::container::container_solver::ContainerSolver;
use crate::de::hysky::skyblocker::utils::container::regex_container_matcher::RegexContainerMatcher;
use crate::de::hysky::skyblocker::utils::container::regex_container_matcher::RegexContainerMatcherBase;
use crate::de::hysky::skyblocker::utils::container::simple_container_solver::SimpleContainerSolver;
use crate::de::hysky::skyblocker::utils::container::simple_container_solver::SimpleContainerSolverBase;
use crate::de::hysky::skyblocker::utils::data::profiled_data::ProfiledData;
use crate::de::hysky::skyblocker::utils::item_utils::ItemUtils;
use crate::de::hysky::skyblocker::utils::render::gui::color_highlight::ColorHighlight;
use crate::de::hysky::skyblocker::utils::resettable::Resettable;
use mcrust::net::minecraft::world::item::items;

/// java: private static final Pattern KILLS_PATTERN = Pattern.compile("Kills: ([0-9,]+)");
pub static KILLS_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("Kills: ([0-9,]+)"));
/// java: private static final int ALL_ZEALOTS_INDEX = 4;
pub const ALL_ZEALOTS_INDEX: i32 = 4;
/// java: private static final int SPECIAL_ZEALOT_INDEX = 22;
pub const SPECIAL_ZEALOT_INDEX: i32 = 22;

/// java: public class EndStatsBestiaryUpdater extends SimpleContainerSolver
#[derive(Clone)]
pub struct EndStatsBestiaryUpdater {
    /// java: extends SimpleContainerSolver
    pub base: SimpleContainerSolverBase,
}

impl std::ops::Deref for EndStatsBestiaryUpdater {
    type Target = SimpleContainerSolverBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for EndStatsBestiaryUpdater {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl EndStatsBestiaryUpdater {
    /// java: public EndStatsBestiaryUpdater()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = SimpleContainerSolverBase::new_in("The End ➜ Zealot".to_string(), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
            });
            ((__h.clone() as compat::Shared<dyn SimpleContainerSolver>), __h)
        });
        __this
    }

}

impl SimpleContainerSolver for EndStatsBestiaryUpdater {
    fn simple_container_solver_base(&self) -> &SimpleContainerSolverBase {
        &self.base
    }
    fn simple_container_solver_base_mut(&mut self) -> &mut SimpleContainerSolverBase {
        &mut self.base
    }

}

impl RegexContainerMatcher for EndStatsBestiaryUpdater {
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

impl ContainerMatcher for EndStatsBestiaryUpdater {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public boolean isEnabled() (dispatch: the body for this class)
    fn is_enabled(&self) -> fn(&compat::Shared<dyn ContainerMatcher>) -> bool {
        EndStatsBestiaryUpdater::is_enabled_dyn
    }

    /// java: @Override public boolean test(Screen screen) (dispatch: the body for this class)
    fn test(&self) -> fn(&compat::Shared<dyn ContainerMatcher>, &mcrust::net::minecraft::client::gui::screens::screen::Screen) -> bool {
        <dyn RegexContainerMatcher>::test_from::<Self>
    }

}

impl ContainerSolver for EndStatsBestiaryUpdater {
    /// java: @Override public List<ColorHighlight> getColors(Int2ObjectMap<ItemStack> slots) (dispatch: the body for this class)
    fn get_colors(&self) -> fn(&compat::Shared<dyn ContainerSolver>, &mut std::collections::HashMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> Vec<ColorHighlight> {
        EndStatsBestiaryUpdater::get_colors_dyn
    }

}

impl Resettable for EndStatsBestiaryUpdater {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override default void reset() (dispatch: the body for this class)
    fn reset(&self) -> fn(&compat::Shared<dyn Resettable>) {
        <dyn ContainerSolver>::reset_from::<Self>
    }

}

impl EndStatsBestiaryUpdater {
    /// java: @Override public boolean isEnabled()
    pub fn is_enabled_impl(__this: &compat::Shared<Self>) -> bool {
        return compat::read(&EndHudWidget::get_instance(), |__t| __t.show_zealot_kills);
    }

    /// EndStatsBestiaryUpdater.isEnabled as the dispatch of ContainerMatcher.isEnabled
    pub fn is_enabled_dyn(__this: &compat::Shared<dyn ContainerMatcher>) -> bool {
        Self::is_enabled_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public List<ColorHighlight> getColors(Int2ObjectMap<ItemStack> slots)
    pub fn get_colors_impl(__this: &compat::Shared<Self>, slots: &mut std::collections::HashMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> Vec<ColorHighlight> {
        for entry in slots.iter().map(|(k, v)| (k.clone(), v.clone())).filter(|entry| (entry.0 == ALL_ZEALOTS_INDEX || entry.0 == SPECIAL_ZEALOT_INDEX)).filter(|entry| entry.1.is_item(&items::items().get("ender_chest"))) {
            let Some(mut matcher) = ItemUtils::get_lore_line_if_match(&entry.1, &KILLS_PATTERN) else {
                continue;
            };
            let mut kills_str: String = matcher.group(1).unwrap().replace(",", "");
            // TODO(fabric2seer): the condition: static call NumberUtils.isCreatable(..)
            //   java: !NumberUtils.isCreatable(killsStr)
            if todo!("fabric2seer: the condition (TODO above)") {
                continue;
            }
            // TODO(fabric2seer): static call NumberUtils.toInt(..)
            //   java: int kills = NumberUtils.toInt(killsStr);
            let mut kills: i32 = todo!("fabric2seer: the initialiser of `kills` (TODO above)");
            let mut stats = ProfiledData::compute_if_absent(the_end::PROFILES_STATS.get_mut(), the_end::END_STATS_EMPTY.get_mut().clone());
            if entry.0 == ALL_ZEALOTS_INDEX {
                // TODO(fabric2seer): `.zealotsSinceLastEye(..)` on a T
                //   java: TheEnd.PROFILES_STATS.put(new TheEnd.EndStats(kills, stats.zealotsSinceLastEye(), stats.eyes()));
            } else if entry.0 == SPECIAL_ZEALOT_INDEX {
                // TODO(fabric2seer): `.totalZealotKills(..)` on a T
                //   java: TheEnd.PROFILES_STATS.put(new TheEnd.EndStats(stats.totalZealotKills(), stats.zealotsSinceLastEye(), kills));
            }
            ({ let __r = &EndHudWidget::get_instance(); let __f = compat::dispatch(__r, |__t| __t.update()); __f(&compat::view(__r, |a| a as compat::Shared<dyn ElementBasedWidget>)) });
        }
        return vec![];
    }

    /// EndStatsBestiaryUpdater.getColors as the dispatch of ContainerSolver.getColors
    pub fn get_colors_dyn(__this: &compat::Shared<dyn ContainerSolver>, __p0: &mut std::collections::HashMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> Vec<ColorHighlight> {
        Self::get_colors_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

}

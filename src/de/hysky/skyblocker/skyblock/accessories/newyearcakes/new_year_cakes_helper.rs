//! Ported from de.hysky.skyblocker.skyblock.accessories.newyearcakes.NewYearCakesHelper by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::utils::container::container_matcher::ContainerMatcher;
use crate::de::hysky::skyblocker::utils::container::container_solver::ContainerSolver;
use crate::de::hysky::skyblocker::utils::container::regex_container_matcher::RegexContainerMatcher;
use crate::de::hysky::skyblocker::utils::container::regex_container_matcher::RegexContainerMatcherBase;
use crate::de::hysky::skyblocker::utils::container::simple_container_solver::SimpleContainerSolver;
use crate::de::hysky::skyblocker::utils::container::simple_container_solver::SimpleContainerSolverBase;
use crate::de::hysky::skyblocker::utils::formatters;
use crate::de::hysky::skyblocker::utils::render::gui::color_highlight::ColorHighlight;
use crate::de::hysky::skyblocker::utils::resettable::Resettable;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final Pattern NEW_YEAR_CAKE = Pattern.compile("New Year Cake \\(Year (?<year>\\d+)\\)");
pub static NEW_YEAR_CAKE: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("New Year Cake \\(Year (?<year>\\d+)\\)"));
/// java: private static final Pattern NEW_YEAR_CAKE_PURCHASE = Pattern.compile("You purchased New Year Cake \\(Year (?<year>\\d+)\\) for .+ coins!");
pub static NEW_YEAR_CAKE_PURCHASE: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("You purchased New Year Cake \\(Year (?<year>\\d+)\\) for .+ coins!"));
/// java: public static final NewYearCakesHelper INSTANCE = new NewYearCakesHelper();
pub static INSTANCE: compat::MainThread<compat::Shared<NewYearCakesHelper>> = compat::MainThread::new(|| NewYearCakesHelper::new());

/// java: public class NewYearCakesHelper extends SimpleContainerSolver
#[derive(Clone)]
pub struct NewYearCakesHelper {
    /// java: extends SimpleContainerSolver
    pub base: SimpleContainerSolverBase,
    /// java: private final Map<String, IntSet> cakes = new HashMap<>();
    pub cakes: std::collections::HashMap<String, std::collections::HashSet<i32>>,
}

impl std::ops::Deref for NewYearCakesHelper {
    type Target = SimpleContainerSolverBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for NewYearCakesHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl NewYearCakesHelper {
    /// java: private NewYearCakesHelper()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = SimpleContainerSolverBase::new_in("Auctions: \".*\"".to_string(), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                cakes: std::collections::HashMap::new(),
            });
            ((__h.clone() as compat::Shared<dyn SimpleContainerSolver>), __h)
        });
        // TODO(fabric2seer): ClientReceiveMessageEvents.ALLOW_GAME.register: no SeerClient equivalent yet
        //   java: ClientReceiveMessageEvents.ALLOW_GAME.register(this::onChatMessage);
        __this
    }

    /// java: public static int getCakeYear(ItemStack stack)
    pub fn get_cake_year(stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> i32 {
        return NewYearCakesHelper::get_cake_year_2(&NEW_YEAR_CAKE, &compat::component_of(stack.get_hover_name()).get_string());
    }

    /// java: public static int getCakeYear(Pattern pattern, String name)
    pub fn get_cake_year_2(pattern: &compat::regex::Pattern, name: &str) -> i32 {
        let mut matcher: compat::regex::Matcher = pattern.matcher(name);
        if matcher.matches() {
            // java: try {..} catch (ParseException e) - an exception here is a panic, caught by `catch_unwind`; the body's `return` is the closure's value
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                // TODO(fabric2seer): `.intValue(..)` on a value of unknown type
                //   java: return Formatters.INTEGER_NUMBERS.parse(matcher.group("year")).intValue();
            })) {
                Ok(__v) => return __v,
                Err(e) => {
                    // TODO(fabric2seer): a log call with a non-literal format
                    //   java: LOGGER.info("Failed to parse year from New Year Cake: " + name, e);
                }
            }
        }
        return -1;
    }

    /// java: public boolean addCake(int year)
    pub fn add_cake(__this: &compat::Shared<Self>, mut year: i32) -> bool {
        if year < 0 {
            return false;
        }
        return (__this.lock().borrow_mut().cakes.entry(Utils::get_profile()).or_insert_with_key(|k| (|_| std::collections::HashSet::new())(k.clone()))).insert(year);
    }

    /// java: private boolean onChatMessage(Component message, boolean overlay)
    pub fn on_chat_message(__this: &compat::Shared<Self>, message: &MutableComponent, mut overlay: bool) -> bool {
        if { let __f = compat::dispatch(__this, |__t| __t.is_enabled()); __f(&compat::view(__this, |a| a as compat::Shared<dyn ContainerMatcher>)) } {
            NewYearCakesHelper::add_cake(__this, NewYearCakesHelper::get_cake_year_2(&NEW_YEAR_CAKE_PURCHASE, &message.get_string()));
        }
        return true;
    }

}

impl SimpleContainerSolver for NewYearCakesHelper {
    fn simple_container_solver_base(&self) -> &SimpleContainerSolverBase {
        &self.base
    }
    fn simple_container_solver_base_mut(&mut self) -> &mut SimpleContainerSolverBase {
        &mut self.base
    }

}

impl RegexContainerMatcher for NewYearCakesHelper {
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

impl ContainerMatcher for NewYearCakesHelper {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public boolean isEnabled() (dispatch: the body for this class)
    fn is_enabled(&self) -> fn(&compat::Shared<dyn ContainerMatcher>) -> bool {
        NewYearCakesHelper::is_enabled_dyn
    }

    /// java: @Override public boolean test(Screen screen) (dispatch: the body for this class)
    fn test(&self) -> fn(&compat::Shared<dyn ContainerMatcher>, &mcrust::net::minecraft::client::gui::screens::screen::Screen) -> bool {
        <dyn RegexContainerMatcher>::test_from::<Self>
    }

}

impl ContainerSolver for NewYearCakesHelper {
    /// java: @Override public List<ColorHighlight> getColors(Int2ObjectMap<ItemStack> slots) (dispatch: the body for this class)
    fn get_colors(&self) -> fn(&compat::Shared<dyn ContainerSolver>, &mut std::collections::HashMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> Vec<ColorHighlight> {
        NewYearCakesHelper::get_colors_dyn
    }

}

impl Resettable for NewYearCakesHelper {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override default void reset() (dispatch: the body for this class)
    fn reset(&self) -> fn(&compat::Shared<dyn Resettable>) {
        <dyn ContainerSolver>::reset_from::<Self>
    }

}

impl NewYearCakesHelper {
    /// java: @Override public boolean isEnabled()
    pub fn is_enabled_impl(__this: &compat::Shared<Self>) -> bool {
        return SkyblockerConfigManager::get().helpers.enable_new_year_cakes_helper;
    }

    /// NewYearCakesHelper.isEnabled as the dispatch of ContainerMatcher.isEnabled
    pub fn is_enabled_dyn(__this: &compat::Shared<dyn ContainerMatcher>) -> bool {
        Self::is_enabled_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public List<ColorHighlight> getColors(Int2ObjectMap<ItemStack> slots)
    pub fn get_colors_impl(__this: &compat::Shared<Self>, slots: &mut std::collections::HashMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> Vec<ColorHighlight> {
        let mut profile: String = Utils::get_profile();
        if __this.lock().borrow().cakes.is_empty() || !(__this.lock().borrow().cakes.contains_key(&profile)) || __this.lock().borrow().cakes.contains_key(&profile) && __this.lock().borrow().cakes.get(&profile).cloned().unwrap().is_empty() {
            return vec![];
        }
        let mut highlights: Vec<ColorHighlight> = Vec::new();
        for entry in slots.iter().map(|(k, v)| (k.clone(), v.clone())) {
            let mut year: i32 = NewYearCakesHelper::get_cake_year(&entry.1);
            if year >= 0 && __this.lock().borrow().cakes.contains_key(&profile) {
                highlights.push((if __this.lock().borrow().cakes.get(&profile).cloned().unwrap().contains(&year) { ColorHighlight::red(entry.0) } else { ColorHighlight::green(entry.0) }));
            }
        }
        return highlights;
    }

    /// NewYearCakesHelper.getColors as the dispatch of ContainerSolver.getColors
    pub fn get_colors_dyn(__this: &compat::Shared<dyn ContainerSolver>, __p0: &mut std::collections::HashMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> Vec<ColorHighlight> {
        Self::get_colors_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

}

//! Ported from de.hysky.skyblocker.skyblock.item.tooltip.adders.DateCalculatorTooltip by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::simple_tooltip_adder::SimpleTooltipAdder;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::simple_tooltip_adder::SimpleTooltipAdderBase;
use crate::de::hysky::skyblocker::utils::container::container_matcher::ContainerMatcher;
use crate::de::hysky::skyblocker::utils::container::regex_container_matcher::RegexContainerMatcher;
use crate::de::hysky::skyblocker::utils::container::regex_container_matcher::RegexContainerMatcherBase;
use crate::de::hysky::skyblocker::utils::container::tooltip_adder::TooltipAdder;
use crate::de::hysky::skyblocker::utils::formatters;
use crate::de::hysky::skyblocker::utils::time::skyblock_time;
use crate::de::hysky::skyblocker::utils::time::skyblock_time::Month;
use mcrust::java::time::format::date_time_formatter;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::world::entity::player::inventory::Inventory;
use mcrust::net::minecraft::world::inventory::slot::Slot;
use mcrust::net::minecraft::network::chat::component::Component as _;
use mcrust::net::minecraft::world::item::item_instance::ItemInstance as _;

/// java: private static final Pattern TIMER_PATTERN = Pattern.compile("((?<days>\\d+)d)? ?((?<hours>\\d+)h)? ?((?<minutes>\\d+)m)? ?((?<seconds>\\d+)s)?");
pub static TIMER_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("((?<days>\\d+)d)? ?((?<hours>\\d+)h)? ?((?<minutes>\\d+)m)? ?((?<seconds>\\d+)s)?"));
/// java: private static final TimeProvider[] PROVIDERS = new TimeProvider[] {new Events(), new Calendar()};
pub static PROVIDERS: compat::MainThread<Vec<compat::Shared<dyn TimeProvider>>> = compat::MainThread::new(|| vec![(Events::new() as compat::Shared<dyn TimeProvider>), (Calendar::new() as compat::Shared<dyn TimeProvider>)]);
/// java: private static final Pattern PATTERN = Pattern.compile("(?<month>.+), Year (?<year>\\d+)");
pub static CALENDAR_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("(?<month>.+), Year (?<year>\\d+)"));

/// java: public class DateCalculatorTooltip extends SimpleTooltipAdder
#[derive(Clone)]
pub struct DateCalculatorTooltip {
    /// java: extends SimpleTooltipAdder
    pub base: SimpleTooltipAdderBase,
    /// java: private @Nullable TimeProvider currentTimer;
    pub current_timer: Option<compat::Shared<dyn TimeProvider>>,
}

impl std::ops::Deref for DateCalculatorTooltip {
    type Target = SimpleTooltipAdderBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for DateCalculatorTooltip {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl DateCalculatorTooltip {
    /// java: public DateCalculatorTooltip(int priority)
    pub fn new(mut priority: i32) -> compat::Shared<Self> {
        let (_, __this) = SimpleTooltipAdderBase::new_1_in(priority, move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                current_timer: None,
            });
            ((__h.clone() as compat::Shared<dyn SimpleTooltipAdder>), __h)
        });
        __this
    }

    /// java: private static boolean hasAnyGroup(MatchResult result)
    pub fn has_any_group(result: &crate::unresolved::MatchResult) -> bool {
        // TODO(fabric2seer): `.namedGroups(..)` on a MatchResult
        //   java: for (String group : result.namedGroups().keySet()) {
        //   java: 			if (result.group(group) != null) return true;
        //   java: 		}
        return false;
    }

}

impl SimpleTooltipAdder for DateCalculatorTooltip {
    fn simple_tooltip_adder_base(&self) -> &SimpleTooltipAdderBase {
        &self.base
    }
    fn simple_tooltip_adder_base_mut(&mut self) -> &mut SimpleTooltipAdderBase {
        &mut self.base
    }

}

impl RegexContainerMatcher for DateCalculatorTooltip {
    fn regex_container_matcher_base(&self) -> &RegexContainerMatcherBase {
        &self.base.base
    }
    fn regex_container_matcher_base_mut(&mut self) -> &mut RegexContainerMatcherBase {
        &mut self.base.base
    }

}

impl ContainerMatcher for DateCalculatorTooltip {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public boolean test(Screen screen) (dispatch: the body for this class)
    fn test(&self) -> fn(&compat::Shared<dyn ContainerMatcher>, &mcrust::net::minecraft::client::gui::screens::screen::Screen) -> bool {
        DateCalculatorTooltip::test_dyn
    }

    /// java: @Override public boolean isEnabled() (dispatch: the body for this class)
    fn is_enabled(&self) -> fn(&compat::Shared<dyn ContainerMatcher>) -> bool {
        DateCalculatorTooltip::is_enabled_dyn
    }

}

impl TooltipAdder for DateCalculatorTooltip {
    /// java: @Override public void addToTooltip(@Nullable Slot focusedSlot, ItemStack stack, List<Component> lines) (dispatch: the body for this class)
    fn add_to_tooltip(&self) -> fn(&compat::Shared<dyn TooltipAdder>, Option<Slot>, &mcrust::net::minecraft::world::item::item_stack::ItemStack, &mut Vec<MutableComponent>) {
        DateCalculatorTooltip::add_to_tooltip_dyn
    }

    /// java: @Override public int getPriority() (dispatch: the body for this class)
    fn get_priority(&self) -> fn(&compat::Shared<dyn TooltipAdder>) -> i32 {
        <dyn SimpleTooltipAdder>::get_priority_from::<Self>
    }

}

impl DateCalculatorTooltip {
    /// java: @Override public boolean test(Screen screen)
    pub fn test_impl(__this: &compat::Shared<Self>, screen: &mcrust::net::minecraft::client::gui::screens::screen::Screen) -> bool {
        let mut screen_title: String = screen.get_title().to_string();
        for timer in PROVIDERS.get_mut().iter() {
            if { let __f = compat::dispatch(timer, |__t| __t.test()); __f(timer, &screen_title) } {
                { let __v = Some(timer.clone()); compat::write(__this, |__t| __t.current_timer = __v); }
                return true;
            }
        }
        { let __v = None; compat::write(__this, |__t| __t.current_timer = __v); }
        return false;
    }

    /// DateCalculatorTooltip.test as the dispatch of ContainerMatcher.test
    pub fn test_dyn(__this: &compat::Shared<dyn ContainerMatcher>, __p0: &mcrust::net::minecraft::client::gui::screens::screen::Screen) -> bool {
        Self::test_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public boolean isEnabled()
    pub fn is_enabled_impl(__this: &compat::Shared<Self>) -> bool {
        return SkyblockerConfigManager::get().helpers.enable_date_calculator;
    }

    /// DateCalculatorTooltip.isEnabled as the dispatch of ContainerMatcher.isEnabled
    pub fn is_enabled_dyn(__this: &compat::Shared<dyn ContainerMatcher>) -> bool {
        Self::is_enabled_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public void addToTooltip(@Nullable Slot focusedSlot, ItemStack stack, List<Component> lines)
    pub fn add_to_tooltip_impl(__this: &compat::Shared<Self>, mut focused_slot: Option<Slot>, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, lines: &mut Vec<MutableComponent>) {
        if compat::read(__this, |__t| __t.current_timer.clone()).is_none() || focused_slot.is_none() || focused_slot.as_ref().unwrap().container.as_any().is::<Inventory>() {
            return;
        }
        {
            let mut i: i32 = 1;
            while i < (lines.len() as i32) {
                'continue_1: {
                    let mut text: String = (lines[(i) as usize]).get_string();
                    if !((({ let __r = &compat::read(__this, |__t| __t.current_timer.clone()).unwrap(); let __f = compat::dispatch(__r, |__t| __t.qualifier()); __f(__r) }))(text.clone())) {
                        break 'continue_1;
                    }
                    let mut instant: Option<std::time::SystemTime> = ({ let __r = &compat::read(__this, |__t| __t.current_timer.clone()).unwrap(); let __f = compat::dispatch(__r, |__t| __t.get_start_time()); __f(__r, stack, &text) });
                    if let Some(instant) = &instant {
                        lines.insert(({ i += 1; i }) as usize, component::literal(&date_time_formatter::format_iso_instant(instant.clone())).with_style_formats(&[ChatFormatting::Italic, ChatFormatting::DarkGray]));
                    }
                }
                i += 1;
            }
        }
    }

    /// DateCalculatorTooltip.addToTooltip as the dispatch of TooltipAdder.addToTooltip
    pub fn add_to_tooltip_dyn(__this: &compat::Shared<dyn TooltipAdder>, __p0: Option<Slot>, __p1: &mcrust::net::minecraft::world::item::item_stack::ItemStack, __p2: &mut Vec<MutableComponent>) {
        Self::add_to_tooltip_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2)
    }

}

/// java: private interface TimeProvider
pub trait TimeProvider: compat::DynAny {
    fn as_any(&self) -> &dyn std::any::Any;
    /// java: boolean test(String screenTitle); (dispatch: the body for this class)
    fn test(&self) -> fn(&compat::Shared<dyn TimeProvider>, &str) -> bool;

    /// java: default Predicate<String> qualifier() (dispatch: the body for this class)
    fn qualifier(&self) -> fn(&compat::Shared<dyn TimeProvider>) -> Arc<dyn Fn(String) -> bool> {
        <dyn TimeProvider>::qualifier_body
    }

    /// java: @Nullable Instant getStartTime(ItemStack stack, String qualifiedLine); (dispatch: the body for this class)
    fn get_start_time(&self) -> fn(&compat::Shared<dyn TimeProvider>, &mcrust::net::minecraft::world::item::item_stack::ItemStack, &str) -> Option<std::time::SystemTime>;

}

impl dyn TimeProvider {
    /// java: default Predicate<String> qualifier()
    pub fn qualifier_body(__this: &compat::Shared<dyn TimeProvider>) -> Arc<dyn Fn(String) -> bool> {
        return Arc::new(move |l| (l.contains("Starts in:") || l.contains(" (")));
    }

}

/// java: private static class Events implements TimeProvider
#[derive(Clone)]
pub struct Events {}

impl Events {
    /// java: the implicit no-argument constructor
    pub fn new() -> compat::Shared<Self> {
        let __this = compat::shared(Self {
        });
        __this
    }

}

impl TimeProvider for Events {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public boolean test(String screenTitle) (dispatch: the body for this class)
    fn test(&self) -> fn(&compat::Shared<dyn TimeProvider>, &str) -> bool {
        Events::test_dyn
    }

    /// java: @Override public @Nullable Instant getStartTime(ItemStack stack, String qualifiedLine) (dispatch: the body for this class)
    fn get_start_time(&self) -> fn(&compat::Shared<dyn TimeProvider>, &mcrust::net::minecraft::world::item::item_stack::ItemStack, &str) -> Option<std::time::SystemTime> {
        Events::get_start_time_dyn
    }

}

impl Events {
    /// java: @Override public boolean test(String screenTitle)
    pub fn test_impl(__this: &compat::Shared<Self>, screen_title: &str) -> bool {
        return (screen_title == "Calendar and Events");
    }

    /// Events.test as the dispatch of TimeProvider.test
    pub fn test_dyn(__this: &compat::Shared<dyn TimeProvider>, __p0: &str) -> bool {
        Self::test_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public @Nullable Instant getStartTime(ItemStack stack, String qualifiedLine)
    pub fn get_start_time_impl(__this: &compat::Shared<Self>, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, qualified_line: &str) -> Option<std::time::SystemTime> {
        let mut result: Option<crate::unresolved::MatchResult> = TIMER_PATTERN.matcher(qualified_line).results().into_iter().filter(|__a0| DateCalculatorTooltip::has_any_group(__a0)).next();
        if let Some(result) = &result {
            // TODO(fabric2seer): `.plus(..)` on a Instant
            //   java: return Instant.now()
            //   java: 					.plus(RegexUtils.parseOptionalIntFromMatcher(result, "days").orElse(0), ChronoUnit.DAYS)
            //   java: 					.plus(RegexUtils.parseOptionalIntFromMatcher(result, "hours").orElse(0), ChronoUnit.HOURS)
            //   java: 					.plus(RegexUtils.parseOptionalIntFromMatcher(result, "minutes").orElse(0), ChronoUnit.MINUTES)
            //   java: 					.plusSeconds(RegexUtils.parseOptionalIntFromMatcher(result, "seconds").orElse(0))
            //   java: 					.plusSeconds(30) // Add 30 seconds to round to the nearest minute
            //   java: 					.truncatedTo(ChronoUnit.MINUTES);
        }
        return None;
    }

    /// Events.getStartTime as the dispatch of TimeProvider.getStartTime
    pub fn get_start_time_dyn(__this: &compat::Shared<dyn TimeProvider>, __p0: &mcrust::net::minecraft::world::item::item_stack::ItemStack, __p1: &str) -> Option<std::time::SystemTime> {
        Self::get_start_time_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1)
    }

}

/// java: private static class Calendar implements TimeProvider
#[derive(Clone)]
pub struct Calendar {
    /// java: private SkyblockTime.Month month = SkyblockTime.Month.EARLY_SPRING;
    pub month: Month,
    /// java: private int year = 1;
    pub year: i32,
}

impl Calendar {
    /// java: the implicit no-argument constructor
    pub fn new() -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            month: Month::EarlySpring,
            year: 1,
        });
        __this
    }

}

impl TimeProvider for Calendar {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public boolean test(String screenTitle) (dispatch: the body for this class)
    fn test(&self) -> fn(&compat::Shared<dyn TimeProvider>, &str) -> bool {
        Calendar::test_dyn
    }

    /// java: @Override public @Nullable Instant getStartTime(ItemStack stack, String qualifiedLine) (dispatch: the body for this class)
    fn get_start_time(&self) -> fn(&compat::Shared<dyn TimeProvider>, &mcrust::net::minecraft::world::item::item_stack::ItemStack, &str) -> Option<std::time::SystemTime> {
        Calendar::get_start_time_dyn
    }

}

impl Calendar {
    /// java: @Override public boolean test(String screenTitle)
    pub fn test_impl(__this: &compat::Shared<Self>, screen_title: &str) -> bool {
        let mut matcher: compat::regex::Matcher = CALENDAR_PATTERN.matcher(screen_title);
        if matcher.matches() {
            let Some(mut maybe_month) = Month::of(matcher.group_named("month").as_deref().unwrap()) else {
                return false;
            };
            { let __v = maybe_month; compat::write(__this, |__t| __t.month = __v); }
            // TODO(fabric2seer): static call NumberUtils.toInt(..)
            //   java: int maybeYear = NumberUtils.toInt(matcher.group("year"));
            let mut maybe_year: i32 = todo!("fabric2seer: the initialiser of `maybeYear` (TODO above)");
            if maybe_year == 0 {
                return false;
            }
            { let __v = maybe_year; compat::write(__this, |__t| __t.year = __v); }
            return true;
        }
        return false;
    }

    /// Calendar.test as the dispatch of TimeProvider.test
    pub fn test_dyn(__this: &compat::Shared<dyn TimeProvider>, __p0: &str) -> bool {
        Self::test_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public @Nullable Instant getStartTime(ItemStack stack, String qualifiedLine)
    pub fn get_start_time_impl(__this: &compat::Shared<Self>, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, qualified_line: &str) -> Option<std::time::SystemTime> {
        if stack.count() > 31 {
            return None;
        }
        // TODO(fabric2seer): `.with(..)` on a Instant
        //   java: return SkyblockTime.SKYBLOCK_EPOCH
        //   java: 					.with(SkyblockTimeField.YEAR, year)
        //   java: 					.with(SkyblockTimeField.MONTH_OF_YEAR, month.ordinal() + 1)
        //   java: 					.with(SkyblockTimeField.DAY_OF_MONTH, stack.count());
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// Calendar.getStartTime as the dispatch of TimeProvider.getStartTime
    pub fn get_start_time_dyn(__this: &compat::Shared<dyn TimeProvider>, __p0: &mcrust::net::minecraft::world::item::item_stack::ItemStack, __p1: &str) -> Option<std::time::SystemTime> {
        Self::get_start_time_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1)
    }

}

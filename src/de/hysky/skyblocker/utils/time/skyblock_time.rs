//! Ported from de.hysky.skyblocker.utils.time.SkyblockTime by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler::Scheduler;
use mcrust::mcrust::modding::events::Event;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public static final Instant SKYBLOCK_EPOCH = Instant.ofEpochMilli(1560275700000L);
pub static SKYBLOCK_EPOCH: compat::MainThread<mcrust::java::time::instant::Instant> = compat::MainThread::new(|| mcrust::java::time::instant::Instant::of_epoch_milli(1560275700000));
/// java: public static final AtomicInteger skyblockYear = new AtomicInteger(0);
pub static SKYBLOCK_YEAR: LazyLock<Mutex<std::sync::atomic::AtomicI32>> = LazyLock::new(|| Mutex::new(std::sync::atomic::AtomicI32::new(0)));
// TODO(fabric2seer): `new AtomicReference(..)`
//   java: public static final AtomicReference<Season> skyblockSeason = new AtomicReference<>(Season.SPRING);
/// java: public static final AtomicReference<Season> skyblockSeason = new AtomicReference<>(Season.SPRING);
pub static SKYBLOCK_SEASON: compat::MainThread<crate::unresolved::AtomicReference> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `skyblockSeason` - see the TODO above"));
// TODO(fabric2seer): `new AtomicReference(..)`
//   java: public static final AtomicReference<Month> skyblockMonth = new AtomicReference<>(Month.EARLY_SPRING);
/// java: public static final AtomicReference<Month> skyblockMonth = new AtomicReference<>(Month.EARLY_SPRING);
pub static SKYBLOCK_MONTH: compat::MainThread<crate::unresolved::AtomicReference> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `skyblockMonth` - see the TODO above"));
/// java: public static final AtomicInteger skyblockDay = new AtomicInteger(0);
pub static SKYBLOCK_DAY: LazyLock<Mutex<std::sync::atomic::AtomicI32>> = LazyLock::new(|| Mutex::new(std::sync::atomic::AtomicI32::new(0)));
/// java: public static final AtomicInteger skyblockHour = new AtomicInteger(0);
pub static SKYBLOCK_HOUR: LazyLock<Mutex<std::sync::atomic::AtomicI32>> = LazyLock::new(|| Mutex::new(std::sync::atomic::AtomicI32::new(0)));
/// java: public static final double HOUR_LENGTH = 50000;
pub const HOUR_LENGTH: f64 = ((50000) as f64) as f64;
/// java: public static final double DAY_LENGTH = HOUR_LENGTH * 24;
pub static DAY_LENGTH: LazyLock<f64> = LazyLock::new(|| (HOUR_LENGTH * (24 as f64)));
/// java: public static final double MONTH_LENGTH = DAY_LENGTH * 31;
pub static MONTH_LENGTH: LazyLock<f64> = LazyLock::new(|| (*DAY_LENGTH * (31 as f64)));
/// java: public static final double SEASON_LENGTH = MONTH_LENGTH * 3;
pub static SEASON_LENGTH: LazyLock<f64> = LazyLock::new(|| (*MONTH_LENGTH * (3 as f64)));
/// java: public static final double YEAR_LENGTH = SEASON_LENGTH * 4;
pub static YEAR_LENGTH: LazyLock<f64> = LazyLock::new(|| (*SEASON_LENGTH * (4 as f64)));
/// java: public static final Event<OnHourChange> HOUR_CHANGE = EventFactory.createArrayBacked(OnHourChange.class, listeners -> hour -> { for (OnHourChange listener : listeners) { listener.onHourChange(hour); }
pub static HOUR_CHANGE: compat::MainThread<Event<dyn Fn(i32)>> = compat::MainThread::new(|| Event::new());
/// the invoker of `HOUR_CHANGE` (`HOUR_CHANGE.invoker()`): the factory's lambda over the listeners
pub fn hour_change_invoker() -> OnHourChange {
    let callbacks: Vec<OnHourChange> = HOUR_CHANGE.listeners();
    Arc::new(move |hour| {
        for listener in callbacks.iter() {
            (listener)(hour);
        }
    })
}
/// java: public static final Event<OnDayChange> DAY_CHANGE = EventFactory.createArrayBacked(OnDayChange.class, listeners -> day -> { for (OnDayChange listener : listeners) { listener.onDayChange(day); } });
pub static DAY_CHANGE: compat::MainThread<Event<dyn Fn(i32)>> = compat::MainThread::new(|| Event::new());
/// the invoker of `DAY_CHANGE` (`DAY_CHANGE.invoker()`): the factory's lambda over the listeners
pub fn day_change_invoker() -> OnDayChange {
    let callbacks: Vec<OnDayChange> = DAY_CHANGE.listeners();
    Arc::new(move |day| {
        for listener in callbacks.iter() {
            (listener)(day);
        }
    })
}
/// java: public static final Event<OnMonthChange> MONTH_CHANGE = EventFactory.createArrayBacked(OnMonthChange.class, listeners -> month -> { for (OnMonthChange listener : listeners) { listener.onMonthChange(mo
pub static MONTH_CHANGE: compat::MainThread<Event<dyn Fn(Month)>> = compat::MainThread::new(|| Event::new());
/// the invoker of `MONTH_CHANGE` (`MONTH_CHANGE.invoker()`): the factory's lambda over the listeners
pub fn month_change_invoker() -> OnMonthChange {
    let callbacks: Vec<OnMonthChange> = MONTH_CHANGE.listeners();
    Arc::new(move |month| {
        for listener in callbacks.iter() {
            (listener)(month);
        }
    })
}
/// java: public static final Event<OnSeasonChange> SEASON_CHANGE = EventFactory.createArrayBacked(OnSeasonChange.class, listeners -> season -> { for (OnSeasonChange listener : listeners) { listener.onSeasonCha
pub static SEASON_CHANGE: compat::MainThread<Event<dyn Fn(Season)>> = compat::MainThread::new(|| Event::new());
/// the invoker of `SEASON_CHANGE` (`SEASON_CHANGE.invoker()`): the factory's lambda over the listeners
pub fn season_change_invoker() -> OnSeasonChange {
    let callbacks: Vec<OnSeasonChange> = SEASON_CHANGE.listeners();
    Arc::new(move |season| {
        for listener in callbacks.iter() {
            (listener)(season);
        }
    })
}
/// java: public static final Event<OnYearChange> YEAR_CHANGE = EventFactory.createArrayBacked(OnYearChange.class, listeners -> year -> { for (OnYearChange listener : listeners) { listener.onYearChange(year); }
pub static YEAR_CHANGE: compat::MainThread<Event<dyn Fn(i32)>> = compat::MainThread::new(|| Event::new());
/// the invoker of `YEAR_CHANGE` (`YEAR_CHANGE.invoker()`): the factory's lambda over the listeners
pub fn year_change_invoker() -> OnYearChange {
    let callbacks: Vec<OnYearChange> = YEAR_CHANGE.listeners();
    Arc::new(move |year| {
        for listener in callbacks.iter() {
            (listener)(year);
        }
    })
}
/// java: public static final Event<OnTimeUpdate> TIME_UPDATE = EventFactory.createArrayBacked(OnTimeUpdate.class, listeners -> (year, season, month, day, hour) -> { for (OnTimeUpdate listener : listeners) { li
pub static TIME_UPDATE: compat::MainThread<Event<dyn Fn(i32, Season, Month, i32, i32)>> = compat::MainThread::new(|| Event::new());
/// the invoker of `TIME_UPDATE` (`TIME_UPDATE.invoker()`): the factory's lambda over the listeners
pub fn time_update_invoker() -> OnTimeUpdate {
    let callbacks: Vec<OnTimeUpdate> = TIME_UPDATE.listeners();
    Arc::new(move |year, season, month, day, hour| {
        for listener in callbacks.iter() {
            (listener)(year, season, month, day, hour);
        }
    })
}

/// java: public class SkyblockTime
#[derive(Clone)]
pub struct SkyblockTime {}

impl SkyblockTime {
    /// java: private SkyblockTime()
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        SkyblockTime::update_time();
        ({ let __r = scheduler::INSTANCE.get_mut(); let __f = compat::dispatch(__r, |__t| __t.schedule()); __f(__r, Arc::new(move || { ({ let __r = scheduler::INSTANCE.get_mut(); let __f = compat::dispatch(__r, |__t| __t.schedule_cyclic()); __f(__r, Arc::new(move || { SkyblockTime::update_time(); }), (((HOUR_LENGTH / (50 as f64))) as i32)) }); }), (((HOUR_LENGTH - ((SkyblockTime::get_skyblock_millis() as f64) % HOUR_LENGTH))) as i32).wrapping_div(50)) });
    }

    /// java: public static long getSkyblockMillis()
    pub fn get_skyblock_millis() -> i64 {
        // TODO(fabric2seer): `.toEpochMilli(..)` on a Instant
        //   java: return System.currentTimeMillis() - SKYBLOCK_EPOCH.toEpochMilli();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: private static int calculateSkyblockYear()
    pub fn calculate_skyblock_year() -> i32 {
        return (((((SkyblockTime::get_skyblock_millis() as f64) / *YEAR_LENGTH).floor() + (1 as f64))) as i32);
    }

    /// java: private static int calculateSkyblockMonth()
    pub fn calculate_skyblock_month() -> i32 {
        return (((((SkyblockTime::get_skyblock_millis() as f64) / *MONTH_LENGTH).floor() % (12 as f64))) as i32);
    }

    /// java: private static int calculateSkyblockDay()
    pub fn calculate_skyblock_day() -> i32 {
        return ((((((SkyblockTime::get_skyblock_millis() as f64) / *DAY_LENGTH).floor() % (31 as f64)) + (1 as f64))) as i32);
    }

    /// java: private static int calculateSkyblockHour()
    pub fn calculate_skyblock_hour() -> i32 {
        return (((((SkyblockTime::get_skyblock_millis() as f64) / HOUR_LENGTH).floor() % (24 as f64))) as i32);
    }

    /// java: private static void updateTime()
    pub fn update_time() {
        let mut year: i32 = SkyblockTime::calculate_skyblock_year();
        if compat::lock(&SKYBLOCK_YEAR).load(std::sync::atomic::Ordering::SeqCst) != year {
            compat::lock(&SKYBLOCK_YEAR).store(year, std::sync::atomic::Ordering::SeqCst);
            (year_change_invoker())(year);
        }
        let mut season: Season = Season::values()[((SkyblockTime::calculate_skyblock_month()).wrapping_div(3)) as usize];
        // TODO(fabric2seer): the condition: `.get(..)` on a AtomicReference
        //   java: skyblockSeason.get() != season
        if todo!("fabric2seer: the condition (TODO above)") {
            // TODO(fabric2seer): `.set(..)` on a AtomicReference
            //   java: skyblockSeason.set(season);
            (season_change_invoker())(season);
        }
        let mut month: Month = Month::values()[(SkyblockTime::calculate_skyblock_month()) as usize];
        // TODO(fabric2seer): the condition: `.get(..)` on a AtomicReference
        //   java: skyblockMonth.get() != month
        if todo!("fabric2seer: the condition (TODO above)") {
            // TODO(fabric2seer): `.set(..)` on a AtomicReference
            //   java: skyblockMonth.set(month);
            (month_change_invoker())(month);
        }
        let mut day: i32 = SkyblockTime::calculate_skyblock_day();
        if compat::lock(&SKYBLOCK_DAY).load(std::sync::atomic::Ordering::SeqCst) != day {
            compat::lock(&SKYBLOCK_DAY).store(day, std::sync::atomic::Ordering::SeqCst);
            (day_change_invoker())(day);
        }
        let mut hour: i32 = SkyblockTime::calculate_skyblock_hour();
        if compat::lock(&SKYBLOCK_HOUR).load(std::sync::atomic::Ordering::SeqCst) != hour {
            compat::lock(&SKYBLOCK_HOUR).store(hour, std::sync::atomic::Ordering::SeqCst);
            (hour_change_invoker())(hour);
        }
        (time_update_invoker())(year, season, month, day, hour);
    }

    /// java: public static MutableComponent formatTime(double seconds)
    pub fn format_time(mut seconds: f64) -> MutableComponent {
        if seconds <= (0 as f64) {
            return component::literal("Now").with_style_format(ChatFormatting::Green);
        }
        return component::literal(&SkyblockTime::format_time_string(seconds));
    }

    /// java: public static String formatTimeString(double seconds)
    pub fn format_time_string(mut seconds: f64) -> String {
        seconds = (seconds.max((0 as f64))).ceil();
        let mut builder: String = String::new();
        if seconds >= (86400 as f64) {
            { builder.push_str(&(((seconds / (86400 as f64))) as i32).to_string()); builder.push_str("d "); };
            seconds = (seconds % (86400 as f64));
        }
        if seconds >= (3600 as f64) {
            { builder.push_str(&(((seconds / (3600 as f64))) as i32).to_string()); builder.push_str("h "); };
            seconds = (seconds % (3600 as f64));
        }
        if seconds >= (60 as f64) {
            { builder.push_str(&(((seconds / (60 as f64))) as i32).to_string()); builder.push_str("m "); };
            seconds = (seconds % (60 as f64));
        }
        if seconds >= (1 as f64) {
            { builder.push_str(&((seconds) as i32).to_string()); builder.push_str("s"); };
        }
        return compat::trim(&builder.clone());
    }

}

/// java: public enum Season
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Season {
    Spring,
    Summer,
    Fall,
    Winter,
}

impl Season {
    pub const VALUES: [Season; 4] = [Season::Spring, Season::Summer, Season::Fall, Season::Winter];

    /// java: values()
    pub fn values() -> Vec<Season> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            Season::Spring => "SPRING",
            Season::Summer => "SUMMER",
            Season::Fall => "FALL",
            Season::Winter => "WINTER",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> Season {
        match name {
            "SPRING" => Season::Spring,
            "SUMMER" => Season::Summer,
            "FALL" => Season::Fall,
            "WINTER" => Season::Winter,
            _ => panic!("IllegalArgumentException: No enum constant Season.{name}"),
        }
    }

}

impl std::fmt::Display for Season {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// java: public enum Month
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Month {
    EarlySpring,
    Spring,
    LateSpring,
    EarlySummer,
    Summer,
    LateSummer,
    EarlyAutumn,
    Autumn,
    LateAutumn,
    EarlyWinter,
    Winter,
    LateWinter,
}

impl Month {
    pub const VALUES: [Month; 12] = [Month::EarlySpring, Month::Spring, Month::LateSpring, Month::EarlySummer, Month::Summer, Month::LateSummer, Month::EarlyAutumn, Month::Autumn, Month::LateAutumn, Month::EarlyWinter, Month::Winter, Month::LateWinter];

    /// java: values()
    pub fn values() -> Vec<Month> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            Month::EarlySpring => "EARLY_SPRING",
            Month::Spring => "SPRING",
            Month::LateSpring => "LATE_SPRING",
            Month::EarlySummer => "EARLY_SUMMER",
            Month::Summer => "SUMMER",
            Month::LateSummer => "LATE_SUMMER",
            Month::EarlyAutumn => "EARLY_AUTUMN",
            Month::Autumn => "AUTUMN",
            Month::LateAutumn => "LATE_AUTUMN",
            Month::EarlyWinter => "EARLY_WINTER",
            Month::Winter => "WINTER",
            Month::LateWinter => "LATE_WINTER",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> Month {
        match name {
            "EARLY_SPRING" => Month::EarlySpring,
            "SPRING" => Month::Spring,
            "LATE_SPRING" => Month::LateSpring,
            "EARLY_SUMMER" => Month::EarlySummer,
            "SUMMER" => Month::Summer,
            "LATE_SUMMER" => Month::LateSummer,
            "EARLY_AUTUMN" => Month::EarlyAutumn,
            "AUTUMN" => Month::Autumn,
            "LATE_AUTUMN" => Month::LateAutumn,
            "EARLY_WINTER" => Month::EarlyWinter,
            "WINTER" => Month::Winter,
            "LATE_WINTER" => Month::LateWinter,
            _ => panic!("IllegalArgumentException: No enum constant Month.{name}"),
        }
    }

    /// the constant's constructor data (Java keeps it in the constant's fields)
    pub fn fields(self) -> &'static MonthFields {
        static FIELDS: LazyLock<Vec<MonthFields>> = LazyLock::new(|| vec![
            MonthFields::new(),
            MonthFields::new(),
            MonthFields::new(),
            MonthFields::new(),
            MonthFields::new(),
            MonthFields::new(),
            MonthFields::new(),
            MonthFields::new(),
            MonthFields::new(),
            MonthFields::new(),
            MonthFields::new(),
            MonthFields::new(),
        ]);
        &FIELDS[self as usize]
    }

    /// java: @Override public String toString()
    pub fn to_string(&self) -> String {
        return self.fields().name.clone();
    }

    /// java: public static @Nullable Month of(String name)
    pub fn of(name: &str) -> Option<Month> {
        // TODO(fabric2seer): call to `values(..)` (not a method of this class)
        //   java: for (Month month : values()) {
        //   java: 				if (month.name.equalsIgnoreCase(name)) {
        //   java: 					return month;
        //   java: 				}
        //   java: 			}
        return None;
    }

}

impl std::fmt::Display for Month {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Month::to_string(self))
    }
}

/// the constructor data of each Month constant
pub struct MonthFields {
    pub name: String,
}

impl MonthFields {
    fn new() -> Self {
        let mut this = Self {
            name: Default::default(),
        };
        // TODO(fabric2seer): static call WordUtils.capitalizeFully(..)
        //   java: this.name = WordUtils.capitalizeFully(this.name().replace("_", " "));
        this
    }

}

/// java: public interface OnHourChange
/// a functional interface: `onHourChange(int hour)` - a lambda / method reference is this closure type
pub type OnHourChange = Arc<dyn Fn(i32)>;

/// java: public interface OnDayChange
/// a functional interface: `onDayChange(int day)` - a lambda / method reference is this closure type
pub type OnDayChange = Arc<dyn Fn(i32)>;

/// java: public interface OnMonthChange
/// a functional interface: `onMonthChange(Month month)` - a lambda / method reference is this closure type
pub type OnMonthChange = Arc<dyn Fn(Month)>;

/// java: public interface OnSeasonChange
/// a functional interface: `onSeasonChange(Season season)` - a lambda / method reference is this closure type
pub type OnSeasonChange = Arc<dyn Fn(Season)>;

/// java: public interface OnYearChange
/// a functional interface: `onYearChange(int year)` - a lambda / method reference is this closure type
pub type OnYearChange = Arc<dyn Fn(i32)>;

/// java: public interface OnTimeUpdate
/// a functional interface: `onTimeUpdate(int year, Season season, Month month, int day, int hour)` - a lambda / method reference is this closure type
pub type OnTimeUpdate = Arc<dyn Fn(i32, Season, Month, i32, i32)>;

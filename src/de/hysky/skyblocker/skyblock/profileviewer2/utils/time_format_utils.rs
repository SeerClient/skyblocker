//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.utils.TimeFormatUtils by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::java::text::decimal_format_symbols::Locale;
use mcrust::java::time::zone_id::ZoneId;

/// java: public static final ZoneId GMT = ZoneId.of("GMT");
pub static GMT: compat::MainThread<ZoneId> = compat::MainThread::new(|| ZoneId::of("GMT").expect("ZoneId.of(GMT)"));

/// java: public class TimeFormatUtils
#[derive(Clone)]
pub struct TimeFormatUtils {}

impl TimeFormatUtils {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static String getShortestReasonableUnit(Duration duration)
    pub fn get_shortest_reasonable_unit(duration: &std::time::Duration) -> String {
        // java: duration.toDays() / toHours() / toMinutes() / toSeconds() / toMillis() - each the whole units of the duration
        let days: i64 = (duration.as_secs() / 86_400) as i64;
        let hours: i64 = (duration.as_secs() / 3_600) as i64;
        let minutes: i64 = (duration.as_secs() / 60) as i64;
        let seconds: i64 = duration.as_secs() as i64;
        let millis: i64 = duration.as_millis() as i64;
        if days > (0 as i64) {
            return format!("{}d", days);
        } else if hours > (0 as i64) {
            return format!("{}h", hours);
        } else if minutes > (0 as i64) {
            return format!("{}m", minutes);
        } else if seconds > (0 as i64) {
            return format!("{}s", seconds);
        } else {
            return format!("{}ms", millis);
        }
    }

    /// java: public static String getDurationString(long millis)
    pub fn get_duration_string(mut millis: i64) -> String {
        // java: long minutes = TimeUnit.MILLISECONDS.toMinutes(millis);
        let minutes: i64 = millis / 60_000;
        // java: long seconds = TimeUnit.MILLISECONDS.toSeconds(millis) % 60;
        let seconds: i64 = (millis / 1_000) % 60;
        return format!("{}:{:02}", minutes, seconds);
    }

}

//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.utils.TimeFormatUtils by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::java::text::decimal_format_symbols::Locale;
use mcrust::java::time::zone_id::ZoneId;

// TODO(fabric2seer): static `ZoneId.of` has no Rust counterpart in the port
//   java: public static final ZoneId GMT = ZoneId.of("GMT");
/// java: public static final ZoneId GMT = ZoneId.of("GMT");
pub static GMT: compat::MainThread<ZoneId> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `GMT` - see the TODO above"));

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
        // TODO(fabric2seer): `.toDays(..)` on a Duration
        //   java: long days = duration.toDays();
        let mut days: i64 = todo!("fabric2seer: the initialiser of `days` (TODO above)");
        // TODO(fabric2seer): `.toHours(..)` on a Duration
        //   java: long hours = duration.toHours();
        let mut hours: i64 = todo!("fabric2seer: the initialiser of `hours` (TODO above)");
        // TODO(fabric2seer): `.toMinutes(..)` on a Duration
        //   java: long minutes = duration.toMinutes();
        let mut minutes: i64 = todo!("fabric2seer: the initialiser of `minutes` (TODO above)");
        // TODO(fabric2seer): `.toSeconds(..)` on a Duration
        //   java: long seconds = duration.toSeconds();
        let mut seconds: i64 = todo!("fabric2seer: the initialiser of `seconds` (TODO above)");
        // TODO(fabric2seer): `.toMillis(..)` on a Duration
        //   java: long millis = duration.toMillis();
        let mut millis: i64 = todo!("fabric2seer: the initialiser of `millis` (TODO above)");
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
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public static String getDurationString(long millis)
    pub fn get_duration_string(mut millis: i64) -> String {
        // TODO(fabric2seer): `.toMinutes(..)` on a TimeUnit
        //   java: long minutes = TimeUnit.MILLISECONDS.toMinutes(millis);
        let mut minutes: i64 = todo!("fabric2seer: the initialiser of `minutes` (TODO above)");
        // TODO(fabric2seer): `.toSeconds(..)` on a TimeUnit
        //   java: long seconds = TimeUnit.MILLISECONDS.toSeconds(millis) % 60;
        let mut seconds: i64 = todo!("fabric2seer: the initialiser of `seconds` (TODO above)");
        return format!("{}:{:02}", minutes, seconds);
    }

}

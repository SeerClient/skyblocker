//! Ported from de.hysky.skyblocker.compatibility.MacCompatibility by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: public class MacCompatibility
#[derive(Clone)]
pub struct MacCompatibility {}

impl MacCompatibility {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static boolean is12HourClock()
    pub fn is12_hour_clock() -> bool {
        // TODO(fabric2seer): static call Client.getInstance(..)
        //   java: Object locale = Client.getInstance().send("NSLocale", "currentLocale");
        let mut locale: compat::Object = todo!("fabric2seer: the initialiser of `locale` (TODO above)");
        // TODO(fabric2seer): static call Client.getInstance(..)
        //   java: String timeFormat = (String) Client.getInstance().send("NSDateFormatter", "dateFormatFromTemplate:options:locale:", "j", 0, locale);
        let mut time_format: String = todo!("fabric2seer: the initialiser of `timeFormat` (TODO above)");
        return time_format.contains("a");
    }

}

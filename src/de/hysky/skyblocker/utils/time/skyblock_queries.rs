//! Ported from de.hysky.skyblocker.utils.time.SkyblockQueries by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::time::skyblock_time::{Month, Season};
use crate::de::hysky::skyblocker::utils::time::skyblock_time_field::SkyblockTimeField;
use crate::de::hysky::skyblocker::utils::time::skyblock_time_unit::{TemporalAccessor, TemporalQuery};

//   java: public static final TemporalQuery<SkyblockTime.@Nullable Month> MONTH = temporal -> temporal.isSupported(SkyblockTimeField.MONTH_OF_YEAR) ? SkyblockTime.Month.values()[temporal.get(SkyblockTimeField.MONTH_OF_YEAR) - 1] : null;
/// java: public static final TemporalQuery<SkyblockTime.@Nullable Month> MONTH
pub static MONTH: TemporalQuery<Option<Month>> = TemporalQuery(|temporal: &dyn TemporalAccessor| {
    if temporal.is_supported(SkyblockTimeField::MonthOfYear) { Some(Month::VALUES[(temporal.get(SkyblockTimeField::MonthOfYear) - 1) as usize]) } else { None }
});
//   java: public static final TemporalQuery<SkyblockTime.@Nullable Season> SEASON = temporal -> temporal.isSupported(SkyblockTimeField.SEASON_OF_YEAR) ? SkyblockTime.Season.values()[temporal.get(SkyblockTimeField.SEASON_OF_YEAR) - 1] : null;
/// java: public static final TemporalQuery<SkyblockTime.@Nullable Season> SEASON
pub static SEASON: TemporalQuery<Option<Season>> = TemporalQuery(|temporal: &dyn TemporalAccessor| {
    if temporal.is_supported(SkyblockTimeField::SeasonOfYear) { Some(Season::VALUES[(temporal.get(SkyblockTimeField::SeasonOfYear) - 1) as usize]) } else { None }
});

/// java: public final class SkyblockQueries
#[derive(Clone)]
pub struct SkyblockQueries {}

impl SkyblockQueries {
    /// java: private SkyblockQueries()
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

}

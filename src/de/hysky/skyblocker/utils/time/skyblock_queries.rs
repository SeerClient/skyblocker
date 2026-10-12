//! Ported from de.hysky.skyblocker.utils.time.SkyblockQueries by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

// TODO(fabric2seer): `.isSupported(..)` on a value of unknown type
//   java: public static final TemporalQuery<SkyblockTime.@Nullable Month> MONTH = temporal -> temporal.isSupported(SkyblockTimeField.MONTH_OF_YEAR) ? SkyblockTime.Month.values()[temporal.get(SkyblockTimeField.MONTH_OF_YEAR) - 1] : null;
/// java: public static final TemporalQuery<SkyblockTime.@Nullable Month> MONTH = temporal -> temporal.isSupported(SkyblockTimeField.MONTH_OF_YEAR) ? SkyblockTime.Month.values()[temporal.get(SkyblockTimeField.M
pub static MONTH: compat::MainThread<crate::unresolved::TemporalQuery> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `MONTH` - see the TODO above"));
// TODO(fabric2seer): `.isSupported(..)` on a value of unknown type
//   java: public static final TemporalQuery<SkyblockTime.@Nullable Season> SEASON = temporal -> temporal.isSupported(SkyblockTimeField.SEASON_OF_YEAR) ? SkyblockTime.Season.values()[temporal.get(SkyblockTimeField.SEASON_OF_YEAR) - 1] : null;
/// java: public static final TemporalQuery<SkyblockTime.@Nullable Season> SEASON = temporal -> temporal.isSupported(SkyblockTimeField.SEASON_OF_YEAR) ? SkyblockTime.Season.values()[temporal.get(SkyblockTimeFie
pub static SEASON: compat::MainThread<crate::unresolved::TemporalQuery> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `SEASON` - see the TODO above"));

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

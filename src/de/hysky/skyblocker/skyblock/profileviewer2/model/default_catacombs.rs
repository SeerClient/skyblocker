//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.model.DefaultCatacombs by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::model::generic_catacombs::AggregateStat;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::model::generic_catacombs::GenericCatacombs;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::model::generic_catacombs::GenericCatacombsBase;

/// java: public class DefaultCatacombs extends GenericCatacombs
#[derive(Clone)]
pub struct DefaultCatacombs {
    /// java: extends GenericCatacombs
    pub base: GenericCatacombsBase,
    /// java: public double experience;
    pub experience: f64,
    /// java: @SerializedName("times_played") public AggregateStat timesPlayed;
    pub times_played: Option<compat::Shared<AggregateStat>>,
    /// java: @SerializedName("watcher_kills") public AggregateStat watcherKills;
    pub watcher_kills: Option<compat::Shared<AggregateStat>>,
}

impl std::ops::Deref for DefaultCatacombs {
    type Target = GenericCatacombsBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for DefaultCatacombs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl DefaultCatacombs {
    /// java: the implicit no-argument constructor
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = GenericCatacombsBase::new_in(move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                experience: 0.0,
                times_played: None,
                watcher_kills: None,
            });
            ((__h.clone() as compat::Shared<dyn GenericCatacombs>), __h)
        });
        __this
    }

}

impl GenericCatacombs for DefaultCatacombs {
    fn generic_catacombs_base(&self) -> &GenericCatacombsBase {
        &self.base
    }
    fn generic_catacombs_base_mut(&mut self) -> &mut GenericCatacombsBase {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}

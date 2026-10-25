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

impl DefaultCatacombs {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        // java: the fields `GenericCatacombs` declares (`extends GenericCatacombs`)
        self.base.read_json_fields(json);
        if let Some(__v) = gson::member(json, "experience") {
            if let Some(__x) = gson::as_f64(__v) {
                self.experience = __x;
            }
        }
        if let Some(__v) = gson::member(json, "times_played") {
            if let Some(__x) = (Some(AggregateStat::from_json(__v))).map(Some) {
                self.times_played = __x;
            }
        }
        if let Some(__v) = gson::member(json, "watcher_kills") {
            if let Some(__x) = (Some(AggregateStat::from_json(__v))).map(Some) {
                self.watcher_kills = __x;
            }
        }
    }

    /// `GSON.fromJson(json, DefaultCatacombs.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> compat::Shared<Self> {
        let __this = Self::new();
        compat::write(&__this, |__t| __t.read_json_fields(json));
        __this
    }
}

//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.model.ApiProfileResponse by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::model::api_profile::ApiProfile;

/// java: public class ApiProfileResponse
#[derive(Clone)]
pub struct ApiProfileResponse {
    /// java: public List<ApiProfile> profiles = List.of();
    pub profiles: Vec<compat::Shared<ApiProfile>>,
}

impl ApiProfileResponse {
    /// java: the implicit no-argument constructor
    pub fn new() -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            profiles: vec![],
        });
        __this
    }

    /// java: public @Nullable ApiProfile getSelectedProfile()
    pub fn get_selected_profile(__this: &compat::Shared<Self>) -> Option<compat::Shared<ApiProfile>> {
        return compat::read(__this, |__t| __t.profiles.clone()).into_iter().filter(|profile| compat::read(profile, |__t| __t.selected)).next();
    }

}

impl ApiProfileResponse {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "profiles") {
            if let Some(__x) = gson::list(__v, |__e0| Some(ApiProfile::from_json(__e0))) {
                self.profiles = __x;
            }
        }
    }

    /// `GSON.fromJson(json, ApiProfileResponse.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> compat::Shared<Self> {
        let __this = Self::new();
        compat::write(&__this, |__t| __t.read_json_fields(json));
        __this
    }
}

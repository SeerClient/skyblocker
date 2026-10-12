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

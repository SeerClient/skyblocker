//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.model.ApiProfile by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::model::community_upgrades::CommunityUpgrades;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::model::coop_banking::CoopBanking;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::model::profile_member::ProfileMember;
use mcrust::java::util::uuid::Uuid;

/// java: public class ApiProfile
#[derive(Clone)]
pub struct ApiProfile {
    /// java: @SerializedName("profile_id") public UUID profileId = UUID.randomUUID();
    pub profile_id: Uuid,
    /// java: public CommunityUpgrades communityUpgrades = new CommunityUpgrades();
    pub community_upgrades: CommunityUpgrades,
    /// java: public Map<UUID, ProfileMember> members = Map.of();
    pub members: std::collections::HashMap<Uuid, compat::Shared<ProfileMember>>,
    /// java: public CoopBanking banking = new CoopBanking();
    pub banking: CoopBanking,
    /// java: @SerializedName("cute_name") public String cuteName = "";
    pub cute_name: String,
    /// java: public boolean selected = false;
    pub selected: bool,
}

impl ApiProfile {
    /// java: the implicit no-argument constructor
    pub fn new() -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            profile_id: Uuid::random_uuid(),
            community_upgrades: CommunityUpgrades::new(),
            members: std::collections::HashMap::from([]),
            banking: CoopBanking::new(),
            cute_name: "".to_string(),
            selected: false,
        });
        __this
    }

    /// java: public boolean hasBeenCoop()
    pub fn has_been_coop(__this: &compat::Shared<Self>) -> bool {
        return ((__this.lock().borrow().members.len() as i32) > 1);
    }

}

impl ApiProfile {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "profile_id") {
            if let Some(__x) = crate::de::hysky::skyblocker::skyblock::profileviewer2::profile_viewer::json_uuid(__v) {
                self.profile_id = __x;
            }
        }
        if let Some(__v) = gson::member(json, "communityUpgrades") {
            if let Some(__x) = Some(CommunityUpgrades::from_json(__v)) {
                self.community_upgrades = __x;
            }
        }
        if let Some(__v) = gson::member(json, "members") {
            if let Some(__x) = gson::map(__v, crate::de::hysky::skyblocker::skyblock::profileviewer2::profile_viewer::json_uuid_key, |__e0| Some(ProfileMember::from_json(__e0))) {
                self.members = __x;
            }
        }
        if let Some(__v) = gson::member(json, "banking") {
            if let Some(__x) = Some(CoopBanking::from_json(__v)) {
                self.banking = __x;
            }
        }
        if let Some(__v) = gson::member(json, "cute_name") {
            if let Some(__x) = gson::as_string(__v) {
                self.cute_name = __x;
            }
        }
        if let Some(__v) = gson::member(json, "selected") {
            if let Some(__x) = gson::as_bool(__v) {
                self.selected = __x;
            }
        }
    }

    /// `GSON.fromJson(json, ApiProfile.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> compat::Shared<Self> {
        let __this = Self::new();
        compat::write(&__this, |__t| __t.read_json_fields(json));
        __this
    }
}

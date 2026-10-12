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

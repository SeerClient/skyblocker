//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.LoadingInformation by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::model::api_profile::ApiProfile;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::model::profile_member::ProfileMember;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::utils::elite_leaderboards;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::utils::profile_item_storage::ProfileItemStorage;
use mcrust::com::mojang::authlib::game_profile::GameProfile;

/// java: public record LoadingInformation(ApiProfile profile, GameProfile mainMember, ProfileMember member, Map<String, Integer> leaderboards, CompletableFuture<ProfileItemStorage> itemStorage)
#[derive(Clone)]
pub struct LoadingInformation {
    /// java: ApiProfile profile
    pub profile: compat::Shared<ApiProfile>,
    /// java: GameProfile mainMember
    pub main_member: GameProfile,
    /// java: ProfileMember member
    pub member: compat::Shared<ProfileMember>,
    /// java: Map<String, Integer> leaderboards
    pub leaderboards: std::collections::HashMap<String, i32>,
    /// java: CompletableFuture<ProfileItemStorage> itemStorage
    pub item_storage: compat::LocalFuture<ProfileItemStorage>,
}

impl LoadingInformation {
    /// java: the canonical record constructor
    pub fn new(mut profile: compat::Shared<ApiProfile>, mut main_member: GameProfile, mut member: compat::Shared<ProfileMember>, mut leaderboards: std::collections::HashMap<String, i32>, item_storage: compat::LocalFuture<ProfileItemStorage>) -> Self {
        Self { profile, main_member, member, leaderboards, item_storage }
    }

    /// java: the record accessor `profile()`
    pub fn profile(&self) -> compat::Shared<ApiProfile> {
        self.profile.clone()
    }

    /// java: the record accessor `mainMember()`
    pub fn main_member(&self) -> GameProfile {
        self.main_member.clone()
    }

    /// java: the record accessor `member()`
    pub fn member(&self) -> compat::Shared<ProfileMember> {
        self.member.clone()
    }

    /// java: the record accessor `leaderboards()`
    pub fn leaderboards(&self) -> std::collections::HashMap<String, i32> {
        self.leaderboards.clone()
    }

    /// java: the record accessor `itemStorage()`
    pub fn item_storage(&self) -> compat::LocalFuture<ProfileItemStorage> {
        self.item_storage.clone()
    }

    /// java: public int getLeaderboardPosition(String leaderboard)
    pub fn get_leaderboard_position(&self, leaderboard: &str) -> i32 {
        return self.leaderboards().get(leaderboard).cloned().unwrap_or_else(|| elite_leaderboards::NO_POSITION);
    }

}

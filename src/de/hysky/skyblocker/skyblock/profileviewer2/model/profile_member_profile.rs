//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.model.ProfileMemberProfile by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::java::util::uuid::Uuid;

/// java: public class ProfileMemberProfile
#[derive(Clone)]
pub struct ProfileMemberProfile {
    /// java: @SerializedName("first_join") public long firstJoin;
    pub first_join: i64,
    /// java: public int personal_bank_upgrade;
    pub personal_bank_upgrade: i32,
    /// java: public boolean cookie_buff_active;
    pub cookie_buff_active: bool,
    /// java: @SerializedName("coop_invitation") public @Nullable CoopInvitation coopInvitation;
    pub coop_invitation: Option<CoopInvitation>,
    /// java: @SerializedName("deletion_notice") public @Nullable DeletionNotice deletionNotice;
    pub deletion_notice: Option<DeletionNotice>,
    /// java: @SerializedName("bank_account") public @Nullable Double personalBankAccount;
    pub personal_bank_account: Option<f64>,
}

impl ProfileMemberProfile {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            first_join: 0,
            personal_bank_upgrade: 0,
            cookie_buff_active: false,
            coop_invitation: None,
            deletion_notice: None,
            personal_bank_account: None,
        };
        this
    }

}

/// java: public static class CoopInvitation
#[derive(Clone)]
pub struct CoopInvitation {
    /// java: public long timestamp;
    pub timestamp: i64,
    /// java: @SerializedName("invited_by") public UUID invitedBy = UUID.randomUUID();
    pub invited_by: Uuid,
    /// java: public boolean confirmed;
    pub confirmed: bool,
    /// java: @SerializedName("confirmed_timestamp") public long confirmedTimestamp;
    pub confirmed_timestamp: i64,
}

impl CoopInvitation {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            timestamp: 0,
            invited_by: Uuid::random_uuid(),
            confirmed: false,
            confirmed_timestamp: 0,
        };
        this
    }

}

/// java: public static class DeletionNotice
#[derive(Clone)]
pub struct DeletionNotice {
    /// java: public long timestamp;
    pub timestamp: i64,
}

impl DeletionNotice {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            timestamp: 0,
        };
        this
    }

}

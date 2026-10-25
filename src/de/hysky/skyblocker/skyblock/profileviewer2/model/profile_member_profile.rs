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

impl ProfileMemberProfile {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "first_join") {
            if let Some(__x) = gson::as_i64(__v) {
                self.first_join = __x;
            }
        }
        if let Some(__v) = gson::member(json, "personal_bank_upgrade") {
            if let Some(__x) = gson::as_i32(__v) {
                self.personal_bank_upgrade = __x;
            }
        }
        if let Some(__v) = gson::member(json, "cookie_buff_active") {
            if let Some(__x) = gson::as_bool(__v) {
                self.cookie_buff_active = __x;
            }
        }
        if let Some(__v) = gson::member(json, "coop_invitation") {
            if let Some(__x) = (Some(CoopInvitation::from_json(__v))).map(Some) {
                self.coop_invitation = __x;
            }
        }
        if let Some(__v) = gson::member(json, "deletion_notice") {
            if let Some(__x) = (Some(DeletionNotice::from_json(__v))).map(Some) {
                self.deletion_notice = __x;
            }
        }
        if let Some(__v) = gson::member(json, "bank_account") {
            if let Some(__x) = (gson::as_f64(__v)).map(Some) {
                self.personal_bank_account = __x;
            }
        }
    }

    /// `GSON.fromJson(json, ProfileMemberProfile.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> Self {
        let mut __this = Self::new();
        __this.read_json_fields(json);
        __this
    }
}

impl CoopInvitation {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "timestamp") {
            if let Some(__x) = gson::as_i64(__v) {
                self.timestamp = __x;
            }
        }
        if let Some(__v) = gson::member(json, "invited_by") {
            if let Some(__x) = crate::de::hysky::skyblocker::skyblock::profileviewer2::profile_viewer::json_uuid(__v) {
                self.invited_by = __x;
            }
        }
        if let Some(__v) = gson::member(json, "confirmed") {
            if let Some(__x) = gson::as_bool(__v) {
                self.confirmed = __x;
            }
        }
        if let Some(__v) = gson::member(json, "confirmed_timestamp") {
            if let Some(__x) = gson::as_i64(__v) {
                self.confirmed_timestamp = __x;
            }
        }
    }

    /// `GSON.fromJson(json, CoopInvitation.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> Self {
        let mut __this = Self::new();
        __this.read_json_fields(json);
        __this
    }
}

impl DeletionNotice {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "timestamp") {
            if let Some(__x) = gson::as_i64(__v) {
                self.timestamp = __x;
            }
        }
    }

    /// `GSON.fromJson(json, DeletionNotice.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> Self {
        let mut __this = Self::new();
        __this.read_json_fields(json);
        __this
    }
}

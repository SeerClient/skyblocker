//! Ported from net.azureaaron.hmapi.data.party.PartyRole.

/// java: public enum PartyRole
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PartyRole {
    Leader,
    Moderator,
    Member,
}

impl PartyRole {
    pub const VALUES: [PartyRole; 3] = [PartyRole::Leader, PartyRole::Moderator, PartyRole::Member];

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }
}

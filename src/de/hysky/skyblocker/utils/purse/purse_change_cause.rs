//! Ported from de.hysky.skyblocker.utils.purse.PurseChangeCause by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::area::Hub;
use crate::de::hysky::skyblocker::utils::utils::Utils;

/// java: public enum PurseChangeCause
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PurseChangeCause {
    MobKill,
    TalismanOfCoins,
    DiceSix,
    TakeBank,
    UnknownGain,
    SlayerQuest,
    DiceRoll,
    DepoBank,
    UnknownLoss,
}

impl PurseChangeCause {
    pub const VALUES: [PurseChangeCause; 9] = [PurseChangeCause::MobKill, PurseChangeCause::TalismanOfCoins, PurseChangeCause::DiceSix, PurseChangeCause::TakeBank, PurseChangeCause::UnknownGain, PurseChangeCause::SlayerQuest, PurseChangeCause::DiceRoll, PurseChangeCause::DepoBank, PurseChangeCause::UnknownLoss];

    /// java: values()
    pub fn values() -> Vec<PurseChangeCause> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            PurseChangeCause::MobKill => "MOB_KILL",
            PurseChangeCause::TalismanOfCoins => "TALISMAN_OF_COINS",
            PurseChangeCause::DiceSix => "DICE_SIX",
            PurseChangeCause::TakeBank => "TAKE_BANK",
            PurseChangeCause::UnknownGain => "UNKNOWN_GAIN",
            PurseChangeCause::SlayerQuest => "SLAYER_QUEST",
            PurseChangeCause::DiceRoll => "DICE_ROLL",
            PurseChangeCause::DepoBank => "DEPO_BANK",
            PurseChangeCause::UnknownLoss => "UNKNOWN_LOSS",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> PurseChangeCause {
        match name {
            "MOB_KILL" => PurseChangeCause::MobKill,
            "TALISMAN_OF_COINS" => PurseChangeCause::TalismanOfCoins,
            "DICE_SIX" => PurseChangeCause::DiceSix,
            "TAKE_BANK" => PurseChangeCause::TakeBank,
            "UNKNOWN_GAIN" => PurseChangeCause::UnknownGain,
            "SLAYER_QUEST" => PurseChangeCause::SlayerQuest,
            "DICE_ROLL" => PurseChangeCause::DiceRoll,
            "DEPO_BANK" => PurseChangeCause::DepoBank,
            "UNKNOWN_LOSS" => PurseChangeCause::UnknownLoss,
            _ => panic!("IllegalArgumentException: No enum constant PurseChangeCause.{name}"),
        }
    }

    /// java: public static PurseChangeCause getCause(double diff)
    pub fn get_cause(mut diff: f64) -> PurseChangeCause {
        if diff > (0 as f64) {
            if diff == (5 as f64) || diff == (25 as f64) {
                return TALISMAN_OF_COINS;
            }
            if diff == (15000000 as f64) || diff == (100000000 as f64) {
                return DICE_SIX;
            }
            if compat::minecraft().gui.screen().is_none() {
                return MOB_KILL;
            } else if compat::read(&Utils::get_area(), |__t| compat::DynAny::dyn_any(__t).downcast_ref::<Hub>().is_some_and(|__e| *__e == Hub::Bank)) {
                return TAKE_BANK;
            }
            return UNKNOWN_GAIN;
        } else {
            if diff == (-6666666 as f64) || diff == (-666666 as f64) {
                return DICE_ROLL;
            }
            return UNKNOWN_LOSS;
        }
    }

}

impl std::fmt::Display for PurseChangeCause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

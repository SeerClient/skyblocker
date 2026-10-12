//! Ported from de.hysky.skyblocker.skyblock.dwarven.profittrackers.AbstractProfitTracker by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::sky_block_icons;
use crate::entry::skyblocker_mod;

/// java: private static final String REWARD_TRACKERS_DIR = "reward-trackers";
pub const REWARD_TRACKERS_DIR: &str = "reward-trackers";
/// java: protected static final Pattern REWARD_PATTERN = Pattern.compile(" {4}(.*?) ?x?([\\d,]*)");
pub static REWARD_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile(" {4}(.*?) ?x?([\\d,]*)"));
/// java: protected static final Pattern HOTM_XP_PATTERN = Pattern.compile(" {4}\\+[\\d,]+ HOTM Experience");
pub static HOTM_XP_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile(" {4}\\+[\\d,]+ HOTM Experience"));
/// java: private static final Pattern GEMSTONE_SYMBOLS = Pattern.compile(String.format("[%s%s%s%s%s%s%s%s%s%s%s%s] ", SkyBlockIcons.MINING_FORTUNE, SkyBlockIcons.MINING_SPEED, SkyBlockIcons.PRISTINE, SkyBlockI
pub static GEMSTONE_SYMBOLS: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile(&format!("[{}{}{}{}{}{}{}{}{}{}{}{}] ", sky_block_icons::MINING_FORTUNE, sky_block_icons::MINING_SPEED, sky_block_icons::PRISTINE, sky_block_icons::INTELLIGENCE, sky_block_icons::DEFENSE, sky_block_icons::STRENGTH, sky_block_icons::HEALTH, sky_block_icons::TRUE_DEFENSE, sky_block_icons::CRIT_DAMAGE, sky_block_icons::FISHING_SPEED, sky_block_icons::FORAGING_FORTUNE, sky_block_icons::FARMING_FORTUNE)));

/// java: public abstract class AbstractProfitTracker
#[derive(Clone)]
pub struct AbstractProfitTrackerBase {}

impl AbstractProfitTrackerBase {
    /// java: the implicit no-argument constructor (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_in<__R>(__wrap: impl FnOnce(Self) -> (compat::Shared<dyn AbstractProfitTracker>, __R)) -> (compat::Shared<dyn AbstractProfitTracker>, __R) {
        let this = Self {
        };
        let (__h, __r) = __wrap(this);
        let __this = __h;
        (__this, __r)
    }

    /// java: protected static String replaceGemstoneSymbols(String reward)
    pub fn replace_gemstone_symbols(reward: &str) -> String {
        return GEMSTONE_SYMBOLS.matcher(reward).replace_all("");
    }

}

/// java: the virtual methods of AbstractProfitTracker (implemented by AbstractProfitTrackerBase and every subclass)
pub trait AbstractProfitTracker: compat::DynAny {
    fn abstract_profit_tracker_base(&self) -> &AbstractProfitTrackerBase;
    fn abstract_profit_tracker_base_mut(&mut self) -> &mut AbstractProfitTrackerBase;
    fn as_any(&self) -> &dyn std::any::Any;

    /// java: protected Path getRewardFilePath(String fileName) (dispatch: the body for this class)
    fn get_reward_file_path(&self) -> fn(&compat::Shared<dyn AbstractProfitTracker>, &str) -> std::path::PathBuf {
        <dyn AbstractProfitTracker>::get_reward_file_path_body
    }

}

impl dyn AbstractProfitTracker {
    /// java: protected Path getRewardFilePath(String fileName)
    pub fn get_reward_file_path_body(__this: &compat::Shared<dyn AbstractProfitTracker>, file_name: &str) -> std::path::PathBuf {
        return skyblocker_mod::CONFIG_DIR.get_mut().join(REWARD_TRACKERS_DIR).join(file_name);
    }

}

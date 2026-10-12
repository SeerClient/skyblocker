//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.utils.LevelCalculator by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::loading_information::LoadingInformation;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::model::api_profile::ApiProfile;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::model::profile_member::ProfileMember;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::utils::collection_tiers::CollectionTiers;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::utils::level_info::Cap;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::utils::level_info::LevelInfo;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::utils::level_info::Progress;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::utils::skill::Skill;
use crate::de::hysky::skyblocker::skyblock::slayers::slayer_type::SlayerType;
use crate::de::hysky::skyblocker::utils::neu_repo_manager::NEURepoManager;
use mcrust::java::text::decimal_format_symbols::Locale;

/// java: private static final int SKYBLOCK_LEVEL_INTERVAL = 100;
pub const SKYBLOCK_LEVEL_INTERVAL: i32 = 100;

/// java: public class LevelCalculator
#[derive(Clone)]
pub struct LevelCalculator {}

impl LevelCalculator {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static LevelInfo getSkillLevel(long xp, Skill skill, LoadingInformation info)
    pub fn get_skill_level(mut xp: i64, mut skill: Skill, info: &LoadingInformation) -> LevelInfo {
        // TODO(fabric2seer): `.getLeveling(..)` on a NEUConstants
        //   java: Leveling levelling = NEURepoManager.getConstants().getLeveling();
        let mut levelling: crate::unresolved::Leveling = todo!("fabric2seer: the initialiser of `levelling` (TODO above)");
        let mut has_levelling_constants: bool = (!(NEURepoManager::is_loading()) && true /* java: levelling != null: never null here */);
        // TODO(fabric2seer): `.getCatacombsExperienceRequiredPerLevel(..)` on a Leveling
        //   java: List<Integer> xpChart = hasLevellingConstants ? switch (skill) {
        //   java: 			case CATACOMBS -> levelling.getCatacombsExperienceRequiredPerLevel();
        //   java: 			case RUNECRAFTING -> levelling.getRunecraftingExperienceRequiredPerLevel();
        //   java: 			case SOCIAL -> levelling.getSocialExperienceRequiredPerLevel();
        //   java:
        //   java: 			default -> levelling.getSkillExperienceRequiredPerLevel();
        //   java: 		} : List.of();
        let mut xp_chart: Vec<i32> = todo!("fabric2seer: the initialiser of `xpChart` (TODO above)");
        let mut level_cap_increase: i32 = LevelCalculator::get_skill_cap_increase(skill, info);
        let mut level_cap: i32 = skill.get_base_cap().wrapping_add(level_cap_increase);
        let mut level: i32 = 0;
        let mut remaining_xp: i64 = xp;
        for xp_required in xp_chart.iter().copied() {
            if remaining_xp >= (xp_required as i64) {
                level += 1;
                remaining_xp = remaining_xp.wrapping_sub(xp_required as i64);
            } else {
                break;
            }
        }
        let mut is_catacombs: bool = (skill == Skill::Catacombs);
        let mut capped_level: i32 = (if is_catacombs { level } else { level_cap.min(level) });
        let mut progress: Option<Progress> = None;
        if capped_level < level_cap || is_catacombs && (xp_chart.len() as i32) > capped_level {
            let mut xp_for_next_level: i64 = (xp_chart[(capped_level) as usize]) as i64;
            progress = Some(Progress::new(remaining_xp, xp_for_next_level));
        }
        return LevelInfo::new(xp, capped_level, Cap::new(level_cap, skill.get_absolute_cap()), progress.clone());
    }

    /// java: private static int getSkillCapIncrease(Skill skill, LoadingInformation info)
    pub fn get_skill_cap_increase(mut skill: Skill, info: &LoadingInformation) -> i32 {
        let mut profile: compat::Shared<ApiProfile> = info.profile();
        let mut current_member: compat::Shared<ProfileMember> = info.member();
        return match skill {
            Skill::Farming => current_member.lock().borrow().jacobs_contest.perks.farming_level_cap,
            Skill::Foraging => 'yield_1: {
                let mut increase: i32 = 0;
                if CollectionTiers::unlocked_tier(&profile, "FIG_LOG", 9) {
                    increase += 1;
                }
                if CollectionTiers::unlocked_tier(&profile, "MANGROVE_LOG", 9) {
                    increase += 1;
                }
                if CollectionTiers::unlocked_tier(&profile, "HELIX_LOG", 9) {
                    increase += 1;
                }
                increase = increase.wrapping_add(2);
                if current_member.lock().borrow().skill_tree.experience.get_hotf_level().level() >= 4 {
                    increase = increase.wrapping_add(2);
                }
                break 'yield_1 increase;
            }
            Skill::Taming => (current_member.lock().borrow().pets_data.pet_care.pet_types_sacrificed.len() as i32),
            _ => 0,
        };
    }

    /// java: public static LevelInfo getSlayerLevel(long xp, SlayerType slayer)
    pub fn get_slayer_level(mut xp: i64, mut slayer: SlayerType) -> LevelInfo {
        // TODO(fabric2seer): `.getLeveling(..)` on a NEUConstants
        //   java: Leveling levelling = NEURepoManager.getConstants().getLeveling();
        let mut levelling: crate::unresolved::Leveling = todo!("fabric2seer: the initialiser of `levelling` (TODO above)");
        let mut has_levelling_constants: bool = (!(NEURepoManager::is_loading()) && true /* java: levelling != null: never null here */);
        let mut neu_slayer_id: String = slayer.fields().friendly_name.to_lowercase();
        // TODO(fabric2seer): `.getSlayerExperienceRequiredPerLevel(..)` on a Leveling
        //   java: List<Integer> xpChart = hasLevellingConstants ? levelling.getSlayerExperienceRequiredPerLevel().getOrDefault(neuSlayerId, List.of()) : List.of();
        let mut xp_chart: Vec<i32> = todo!("fabric2seer: the initialiser of `xpChart` (TODO above)");
        let mut level: i32 = 0;
        let mut remaining_xp: i64 = xp;
        for xp_required in xp_chart.iter().copied() {
            if remaining_xp >= (xp_required as i64) {
                level += 1;
                remaining_xp = remaining_xp.wrapping_sub(xp_required as i64);
            } else {
                break;
            }
        }
        let mut capped_level: i32 = (slayer.fields().max_level).min(level);
        let mut progress: Option<Progress> = None;
        if capped_level < slayer.fields().max_level {
            let mut xp_for_next_level: i64 = (xp_chart[(capped_level) as usize]) as i64;
            progress = Some(Progress::new(remaining_xp, xp_for_next_level));
        }
        return LevelInfo::new(xp, capped_level, Cap::new(slayer.fields().max_level, slayer.fields().max_level), progress.clone());
    }

    /// java: public static LevelInfo getSkyblockLevel(int xp)
    pub fn get_skyblock_level(mut xp: i32) -> LevelInfo {
        let mut level: i32 = xp.wrapping_div(SKYBLOCK_LEVEL_INTERVAL);
        let mut cap: Cap = Cap::new(i32::MAX, i32::MAX);
        let mut progress: Progress = Progress::new(((xp.wrapping_sub(level.wrapping_mul(SKYBLOCK_LEVEL_INTERVAL))) as i64) as i64, ((SKYBLOCK_LEVEL_INTERVAL) as i64) as i64);
        let mut level_info: LevelInfo = LevelInfo::new((xp) as i64, level, cap.clone(), Some(progress.clone()));
        return level_info;
    }

    /// java: public static LevelInfo getHotmLevel(long xp)
    pub fn get_hotm_level(mut xp: i64) -> LevelInfo {
        return LevelCalculator::get_hotx_level(xp, "HOTM");
    }

    /// java: public static LevelInfo getHotfLevel(long xp)
    pub fn get_hotf_level(mut xp: i64) -> LevelInfo {
        return LevelCalculator::get_hotx_level(xp, "HOTF");
    }

    /// java: private static LevelInfo getHotxLevel(long xp, String id)
    pub fn get_hotx_level(mut xp: i64, id: &str) -> LevelInfo {
        // TODO(fabric2seer): `.getLeveling(..)` on a NEUConstants
        //   java: Leveling levelling = NEURepoManager.getConstants().getLeveling();
        let mut levelling: crate::unresolved::Leveling = todo!("fabric2seer: the initialiser of `levelling` (TODO above)");
        let mut has_levelling_constants: bool = (!(NEURepoManager::is_loading()) && true /* java: levelling != null: never null here */);
        // TODO(fabric2seer): `.getHotmExperienceRequiredPerLevel(..)` on a Leveling
        //   java: List<Integer> xpChart = hasLevellingConstants ? levelling.getHotmExperienceRequiredPerLevel() : List.of();
        let mut xp_chart: Vec<i32> = todo!("fabric2seer: the initialiser of `xpChart` (TODO above)");
        // TODO(fabric2seer): `.getMaximumLevels(..)` on a Leveling
        //   java: int maxLevel = hasLevellingConstants ? levelling.getMaximumLevels().getOrDefault(id, 0) : 0;
        let mut max_level: i32 = todo!("fabric2seer: the initialiser of `maxLevel` (TODO above)");
        let mut level: i32 = 0;
        let mut remaining_xp: i64 = xp;
        for xp_required in xp_chart.iter().copied() {
            if remaining_xp >= (xp_required as i64) {
                level += 1;
                remaining_xp = remaining_xp.wrapping_sub(xp_required as i64);
            } else {
                break;
            }
        }
        let mut progress: Option<Progress> = None;
        if level < max_level {
            let mut xp_for_next_level: i64 = (xp_chart[(level) as usize]) as i64;
            progress = Some(Progress::new(remaining_xp, xp_for_next_level));
        }
        return LevelInfo::new(xp, level, Cap::new(max_level, max_level), progress.clone());
    }

}

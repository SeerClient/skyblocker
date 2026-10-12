//! Ported from de.hysky.skyblocker.skyblock.profileviewer.utils.LevelFinder by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: private static final long CATA_XP_PER_LEVEL = 200_000_000;
pub const CATA_XP_PER_LEVEL: i64 = ((200000000) as i64) as i64;
/// java: private static final List<LevelInfo> GENERIC_SKILL_BOUNDARIES = createGenericSkillBoundaries();
pub static GENERIC_SKILL_BOUNDARIES: LazyLock<Vec<compat::Shared<LevelInfo>>> = LazyLock::new(|| LevelFinder::create_generic_skill_boundaries());
/// java: private static final List<LevelInfo> CATACOMBS_SKILL_BOUNDARIES = createCatacombsSkillBoundaries();
pub static CATACOMBS_SKILL_BOUNDARIES: LazyLock<Vec<compat::Shared<LevelInfo>>> = LazyLock::new(|| LevelFinder::create_catacombs_skill_boundaries());
/// java: private static final List<LevelInfo> RUNECRAFT_SKILL_BOUNDARIES = createRunecraftSkillBoundaries();
pub static RUNECRAFT_SKILL_BOUNDARIES: LazyLock<Vec<compat::Shared<LevelInfo>>> = LazyLock::new(|| LevelFinder::create_runecraft_skill_boundaries());
/// java: private static final List<LevelInfo> SOCIAL_SKILL_BOUNDARIES = createSocialSkillBoundaries();
pub static SOCIAL_SKILL_BOUNDARIES: LazyLock<Vec<compat::Shared<LevelInfo>>> = LazyLock::new(|| LevelFinder::create_social_skill_boundaries());
/// java: private static final List<LevelInfo> COMMON_PET_BOUNDARIES = createCommonPetBoundaries();
pub static COMMON_PET_BOUNDARIES: LazyLock<Vec<compat::Shared<LevelInfo>>> = LazyLock::new(|| LevelFinder::create_common_pet_boundaries());
/// java: private static final List<LevelInfo> UNCOMMON_PET_BOUNDARIES = createUncommonPetBoundaries();
pub static UNCOMMON_PET_BOUNDARIES: LazyLock<Vec<compat::Shared<LevelInfo>>> = LazyLock::new(|| LevelFinder::create_uncommon_pet_boundaries());
/// java: private static final List<LevelInfo> RARE_PET_BOUNDARIES = createRarePetBoundaries();
pub static RARE_PET_BOUNDARIES: LazyLock<Vec<compat::Shared<LevelInfo>>> = LazyLock::new(|| LevelFinder::create_rare_pet_boundaries());
/// java: private static final List<LevelInfo> EPIC_PET_BOUNDARIES = createEpicPetBoundaries();
pub static EPIC_PET_BOUNDARIES: LazyLock<Vec<compat::Shared<LevelInfo>>> = LazyLock::new(|| LevelFinder::create_epic_pet_boundaries());
/// java: private static final List<LevelInfo> LEGENDARY_PET_BOUNDARIES = createLegendaryPetBoundaries();
pub static LEGENDARY_PET_BOUNDARIES: LazyLock<Vec<compat::Shared<LevelInfo>>> = LazyLock::new(|| LevelFinder::create_legendary_pet_boundaries());
/// java: private static final List<LevelInfo> GENERIC_SLAYER_BOUNDARIES = createGenericSlayerBoundaries();
pub static GENERIC_SLAYER_BOUNDARIES: LazyLock<Vec<compat::Shared<LevelInfo>>> = LazyLock::new(|| LevelFinder::create_generic_slayer_boundaries());
/// java: private static final List<LevelInfo> VAMPIRE_SLAYER_BOUNDARIES = createVampireSlayerBoundaries();
pub static VAMPIRE_SLAYER_BOUNDARIES: LazyLock<Vec<compat::Shared<LevelInfo>>> = LazyLock::new(|| LevelFinder::create_vampire_slayer_boundaries());

/// java: public class LevelFinder
#[derive(Clone)]
pub struct LevelFinder {}

impl LevelFinder {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: private static List<LevelInfo> createGenericSkillBoundaries()
    pub fn create_generic_skill_boundaries() -> Vec<compat::Shared<LevelInfo>> {
        let mut boundaries: Vec<compat::Shared<LevelInfo>> = Vec::new();
        let mut cumulative_xp: Vec<i64> = vec![0, 50, 175, 375, 675, 1175, 1925, 2925, 4425, 6425, 9925, 14925, 22425, 32425, 47425, 67425, 97425, 147425, 222425, 322425, 522425, 822425, 1222425, 1722425, 2322425, 3022425, 3822425, 4722425, 5722425, 6822425, 8022425, 9322425, 10722425, 12222425, 13822425, 15522425, 17322425, 19222425, 21222425, 23322425, 25522425, 27822425, 30222425, 32722425, 35322425, 38072425, 40972425, 44072425, 47472425, 51172425, 55172425, 59472425, 64072425, 68972425, 74172425, 79672425, 85472425, 91572425, 97972425, 104672425, 111672425];
        for i in 0..(cumulative_xp.len() as i32) {
            boundaries.push(LevelInfo::new(cumulative_xp[(i) as usize], i));
        }
        return boundaries;
    }

    /// java: private static List<LevelInfo> createCatacombsSkillBoundaries()
    pub fn create_catacombs_skill_boundaries() -> Vec<compat::Shared<LevelInfo>> {
        let mut boundaries: Vec<compat::Shared<LevelInfo>> = Vec::new();
        let mut cumulative_xp: Vec<i64> = vec![0, 50, 125, 235, 395, 625, 955, 1425, 2095, 3045, 4385, 6275, 8940, 12700, 17960, 25340, 35640, 50040, 70040, 97640, 135640, 188140, 259640, 356640, 488640, 668640, 911640, 1239640, 1684640, 2284640, 3084640, 4149640, 5559640, 7459640, 9959640, 13259640, 17559640, 23159640, 30359640, 39359640, 51359640, 66359640, 85359640, 109559640, 139559640, 177559640, 225559640, 295559640, 360559640, 453559640, 569809640];
        for i in 0..(cumulative_xp.len() as i32) {
            boundaries.push(LevelInfo::new(cumulative_xp[(i) as usize], i));
        }
        return boundaries;
    }

    /// java: private static List<LevelInfo> createRunecraftSkillBoundaries()
    pub fn create_runecraft_skill_boundaries() -> Vec<compat::Shared<LevelInfo>> {
        let mut boundaries: Vec<compat::Shared<LevelInfo>> = Vec::new();
        let mut cumulative_xp: Vec<i64> = vec![0, 50, 150, 275, 435, 635, 885, 1200, 1600, 2100, 2725, 3150, 4510, 5760, 7325, 9325, 11825, 14950, 18950, 23950, 30200, 38050, 47850, 60100, 75400, 94500];
        for i in 0..(cumulative_xp.len() as i32) {
            boundaries.push(LevelInfo::new(cumulative_xp[(i) as usize], i));
        }
        return boundaries;
    }

    /// java: private static List<LevelInfo> createSocialSkillBoundaries()
    pub fn create_social_skill_boundaries() -> Vec<compat::Shared<LevelInfo>> {
        let mut boundaries: Vec<compat::Shared<LevelInfo>> = Vec::new();
        let mut cumulative_xp: Vec<i64> = vec![0, 50, 150, 300, 550, 1050, 1800, 2800, 4050, 5550, 7550, 10050, 13050, 16800, 21300, 27300, 35300, 45300, 57800, 72800, 92800, 117800, 147800, 182800, 222800, 272800];
        for i in 0..(cumulative_xp.len() as i32) {
            boundaries.push(LevelInfo::new(cumulative_xp[(i) as usize], i));
        }
        return boundaries;
    }

    /// java: private static List<LevelInfo> createGenericSlayerBoundaries()
    pub fn create_generic_slayer_boundaries() -> Vec<compat::Shared<LevelInfo>> {
        let mut boundaries: Vec<compat::Shared<LevelInfo>> = Vec::new();
        let mut cumulative_xp: Vec<i64> = vec![0, 5, 15, 200, 1000, 5000, 20000, 100000, 400000, 1000000];
        for i in 0..(cumulative_xp.len() as i32) {
            boundaries.push(LevelInfo::new(cumulative_xp[(i) as usize], i));
        }
        return boundaries;
    }

    /// java: private static List<LevelInfo> createVampireSlayerBoundaries()
    pub fn create_vampire_slayer_boundaries() -> Vec<compat::Shared<LevelInfo>> {
        let mut boundaries: Vec<compat::Shared<LevelInfo>> = Vec::new();
        let mut cumulative_xp: Vec<i64> = vec![0, 20, 75, 240, 840, 2400];
        for i in 0..(cumulative_xp.len() as i32) {
            boundaries.push(LevelInfo::new(cumulative_xp[(i) as usize], i));
        }
        return boundaries;
    }

    /// java: private static List<LevelInfo> createCommonPetBoundaries()
    pub fn create_common_pet_boundaries() -> Vec<compat::Shared<LevelInfo>> {
        let mut boundaries: Vec<compat::Shared<LevelInfo>> = Vec::new();
        let mut cumulative_xp: Vec<i64> = vec![0, 0, 100, 210, 330, 460, 605, 765, 940, 1130, 1340, 1570, 1820, 2095, 2395, 2725, 3085, 3485, 3925, 4415, 4955, 5555, 6215, 6945, 7745, 8625, 9585, 10635, 11785, 13045, 14425, 15935, 17585, 19385, 21345, 23475, 25785, 28285, 30985, 33905, 37065, 40485, 44185, 48185, 52535, 57285, 62485, 68185, 74485, 81485, 89285, 97985, 107685, 118485, 130485, 143785, 158485, 174685, 192485, 211985, 233285, 256485, 281685, 309085, 338885, 371285, 406485, 444685, 486085, 530885, 579285, 631485, 687685, 748085, 812885, 882285, 956485, 1035685, 1120385, 1211085, 1308285, 1412485, 1524185, 1643885, 1772085, 1909285, 2055985, 2212685, 2380385, 2560085, 2752785, 2959485, 3181185, 3418885, 3673585, 3946285, 4237985, 4549685, 4883385, 5241085, 5624785];
        for i in 0..(cumulative_xp.len() as i32) {
            boundaries.push(LevelInfo::new(cumulative_xp[(i) as usize], i));
        }
        return boundaries;
    }

    /// java: private static List<LevelInfo> createUncommonPetBoundaries()
    pub fn create_uncommon_pet_boundaries() -> Vec<compat::Shared<LevelInfo>> {
        let mut boundaries: Vec<compat::Shared<LevelInfo>> = Vec::new();
        let mut cumulative_xp: Vec<i64> = vec![0, 0, 175, 365, 575, 805, 1055, 1330, 1630, 1960, 2320, 2720, 3160, 3650, 4190, 4790, 5450, 6180, 6980, 7860, 8820, 9870, 11020, 12280, 13660, 15170, 16820, 18620, 20580, 22710, 25020, 27520, 30220, 33140, 36300, 39720, 43420, 47420, 51770, 56520, 61720, 67420, 73720, 80720, 88520, 97220, 106920, 117720, 129720, 143020, 157720, 173920, 191720, 211220, 232520, 255720, 280920, 308320, 338120, 370520, 405720, 443920, 485320, 530120, 578520, 630720, 686920, 747320, 812120, 881520, 955720, 1034920, 1119620, 1210320, 1307520, 1411720, 1523420, 1643120, 1771320, 1908520, 2055220, 2211920, 2379620, 2559320, 2752020, 2958720, 3180420, 3418120, 3672820, 3945520, 4237220, 4548920, 4882620, 5240320, 5624020, 6035720, 6477420, 6954120, 7470820, 8032520, 8644220];
        for i in 0..(cumulative_xp.len() as i32) {
            boundaries.push(LevelInfo::new(cumulative_xp[(i) as usize], i));
        }
        return boundaries;
    }

    /// java: private static List<LevelInfo> createRarePetBoundaries()
    pub fn create_rare_pet_boundaries() -> Vec<compat::Shared<LevelInfo>> {
        let mut boundaries: Vec<compat::Shared<LevelInfo>> = Vec::new();
        let mut cumulative_xp: Vec<i64> = vec![0, 0, 275, 575, 905, 1265, 1665, 2105, 2595, 3135, 3735, 4395, 5125, 5925, 6805, 7765, 8815, 9965, 11225, 12605, 14115, 15765, 17565, 19525, 21655, 23965, 26465, 29165, 32085, 35245, 38665, 42365, 46365, 50715, 55465, 60665, 66365, 72665, 79665, 87465, 96165, 105865, 116665, 128665, 141965, 156665, 172865, 190665, 210165, 231465, 254665, 279865, 307265, 337065, 369465, 404665, 442865, 484265, 529065, 577465, 629665, 685865, 746265, 811065, 880465, 954665, 1033865, 1118565, 1209265, 1306465, 1410665, 1522365, 1642065, 1770265, 1907465, 2054165, 2210865, 2378565, 2558265, 2750965, 2957665, 3179365, 3417065, 3671765, 3944465, 4236165, 4547865, 4881565, 5239265, 5622965, 6034665, 6476365, 6953065, 7469765, 8031465, 8643165, 9309865, 10036565, 10828265, 11689965, 12626665];
        for i in 0..(cumulative_xp.len() as i32) {
            boundaries.push(LevelInfo::new(cumulative_xp[(i) as usize], i));
        }
        return boundaries;
    }

    /// java: private static List<LevelInfo> createEpicPetBoundaries()
    pub fn create_epic_pet_boundaries() -> Vec<compat::Shared<LevelInfo>> {
        let mut boundaries: Vec<compat::Shared<LevelInfo>> = Vec::new();
        let mut cumulative_xp: Vec<i64> = vec![0, 0, 440, 930, 1470, 2070, 2730, 3460, 4260, 5140, 6100, 7150, 8300, 9560, 10940, 12450, 14100, 15900, 17860, 19990, 22300, 24800, 27500, 30420, 33580, 37000, 40700, 44700, 49050, 53800, 59000, 64700, 71000, 78000, 85800, 94500, 104200, 115000, 127000, 140300, 155000, 171200, 189000, 208500, 229800, 253000, 278200, 305600, 335400, 367800, 403000, 441200, 482600, 527400, 575800, 628000, 684200, 744600, 809400, 878800, 953000, 1032200, 1116900, 1207600, 1304800, 1409000, 1520700, 1640400, 1768600, 1905800, 2052500, 2209200, 2376900, 2556600, 2749300, 2956000, 3177700, 3415400, 3670100, 3942800, 4234500, 4546200, 4879900, 5237600, 5621300, 6033000, 6474700, 6951400, 7468100, 8029800, 8641500, 9308200, 10034900, 10826600, 11688300, 12625000, 13641700, 14743400, 15935100, 17221800, 18608500];
        for i in 0..(cumulative_xp.len() as i32) {
            boundaries.push(LevelInfo::new(cumulative_xp[(i) as usize], i));
        }
        return boundaries;
    }

    /// java: private static List<LevelInfo> createLegendaryPetBoundaries()
    pub fn create_legendary_pet_boundaries() -> Vec<compat::Shared<LevelInfo>> {
        let mut boundaries: Vec<compat::Shared<LevelInfo>> = Vec::new();
        let mut cumulative_xp: Vec<i64> = vec![0, 0, 660, 1390, 2190, 3070, 4030, 5080, 6230, 7490, 8870, 10380, 12030, 13830, 15790, 17920, 20230, 22730, 25430, 28350, 31510, 34930, 38630, 42630, 46980, 51730, 56930, 62630, 68930, 75930, 83730, 92430, 102130, 112930, 124930, 138230, 152930, 169130, 186930, 206430, 227730, 250930, 276130, 303530, 333330, 365730, 400930, 439130, 480530, 525330, 573730, 625930, 682130, 742530, 807330, 876730, 950930, 1030130, 1114830, 1205530, 1302730, 1406930, 1518630, 1638330, 1766530, 1903730, 2050430, 2207130, 2374830, 2554530, 2747230, 2953930, 3175630, 3413330, 3668030, 3940730, 4232430, 4544130, 4877830, 5235530, 5619230, 6030930, 6472630, 6949330, 7466030, 8027730, 8639430, 9306130, 10032830, 10824530, 11686230, 12622930, 13639630, 14741330, 15933030, 17219730, 18606430, 20103130, 21719830, 23466530, 25353230, 25353230, 25358785, 27245485, 29132185, 31018885, 32905585, 34792285, 36678985, 38565685, 40452385, 42339085, 44225785, 46112485, 47999185, 49885885, 51772585, 53659285, 55545985, 57432685, 59319385, 61206085, 63092785, 64979485, 66866185, 68752885, 70639585, 72526285, 74412985, 76299685, 78186385, 80073085, 81959785, 83846485, 85733185, 87619885, 89506585, 91393285, 93279985, 95166685, 97053385, 98940085, 100826785, 102713485, 104600185, 106486885, 108373585, 110260285, 112146985, 114033685, 115920385, 117807085, 119693785, 121580485, 123467185, 125353885, 127240585, 129127285, 131013985, 132900685, 134787385, 136674085, 138560785, 140447485, 142334185, 144220885, 146107585, 147994285, 149880985, 151767685, 153654385, 155541085, 157427785, 159314485, 161201185, 163087885, 164974585, 166861285, 168747985, 170634685, 172521385, 174408085, 176294785, 178181485, 180068185, 181954885, 183841585, 185728285, 187614985, 189501685, 191388385, 193275085, 195161785, 197048485, 198935185, 200821885, 202708585, 204595285, 206481985, 208368685, 210255385];
        for i in 0..(cumulative_xp.len() as i32) {
            boundaries.push(LevelInfo::new(cumulative_xp[(i) as usize], i));
        }
        return boundaries;
    }

    /// java: public static LevelInfo getLevelInfo(String name, long xp)
    pub fn get_level_info(name: &str, mut xp: i64) -> compat::Shared<LevelInfo> {
        let mut boundaries: Vec<compat::Shared<LevelInfo>> = LevelFinder::get_level_boundaries(name, xp);
        {
            let mut i: i32 = (boundaries.len() as i32).wrapping_sub(1);
            while i >= 0 {
                if xp >= compat::read(&boundaries[(i) as usize], |__t| __t.xp) {
                    let mut fill: f64;
                    let mut xp_in_current_level: f64;
                    let mut level_xp_range: f64;
                    if i < compat::read(&boundaries[boundaries.len() - 1], |__t| __t.level) {
                        let mut current_level_xp: f64 = ((compat::read(&boundaries[(i) as usize], |__t| __t.xp)) as f64) as f64;
                        let mut next_level_xp: f64 = ((compat::read(&boundaries[(i.wrapping_add(1)) as usize], |__t| __t.xp)) as f64) as f64;
                        level_xp_range = (next_level_xp - current_level_xp);
                        xp_in_current_level = ((xp as f64) - current_level_xp);
                        fill = (xp_in_current_level / level_xp_range);
                    } else {
                        fill = 1.0;
                        xp_in_current_level = ((xp.wrapping_sub(compat::read(&boundaries[boundaries.len() - 1], |__t| __t.xp))) as f64) as f64;
                        level_xp_range = (((compat::read(&boundaries[boundaries.len() - 1], |__t| __t.xp)).wrapping_sub(compat::read(&boundaries[((boundaries.len() as i32).wrapping_sub(2)) as usize], |__t| __t.xp))) as f64) as f64;
                    }
                    return LevelInfo::new_5(xp, compat::read(&boundaries[(i) as usize], |__t| __t.level), fill, xp_in_current_level, level_xp_range);
                }
                i -= 1;
            }
        }
        return LevelInfo::new(0, 0);
    }

    /// java: private static List<LevelInfo> getLevelBoundaries(String levelName, long xp)
    pub fn get_level_boundaries(level_name: &str, mut xp: i64) -> Vec<compat::Shared<LevelInfo>> {
        return match level_name {
            "Vampire" => VAMPIRE_SLAYER_BOUNDARIES.clone(),
            "Zombie" | "Spider" | "Wolf" | "Enderman" | "Blaze" => GENERIC_SLAYER_BOUNDARIES.clone(),
            "PET_COMMON" => COMMON_PET_BOUNDARIES.clone(),
            "PET_UNCOMMON" => UNCOMMON_PET_BOUNDARIES.clone(),
            "PET_RARE" => RARE_PET_BOUNDARIES.clone(),
            "PET_EPIC" => EPIC_PET_BOUNDARIES.clone(),
            "PET_LEGENDARY" | "PET_MYTHIC" => LEGENDARY_PET_BOUNDARIES[(0) as usize..(101) as usize].to_vec(),
            "PET_GREG" => LEGENDARY_PET_BOUNDARIES.clone(),
            "Social" => SOCIAL_SKILL_BOUNDARIES.clone(),
            "Runecraft" => RUNECRAFT_SKILL_BOUNDARIES.clone(),
            "Catacombs" => LevelFinder::calculate_catacombs_skill_boundaries(xp),
            _ => GENERIC_SKILL_BOUNDARIES.clone(),
        };
    }

    /// java: private static List<LevelInfo> calculateCatacombsSkillBoundaries(long xp)
    pub fn calculate_catacombs_skill_boundaries(mut xp: i64) -> Vec<compat::Shared<LevelInfo>> {
        if xp >= compat::read(&CATACOMBS_SKILL_BOUNDARIES[CATACOMBS_SKILL_BOUNDARIES.len() - 1], |__t| __t.xp) {
            let mut additional_levels: i32 = (((xp.wrapping_sub(compat::read(&CATACOMBS_SKILL_BOUNDARIES[CATACOMBS_SKILL_BOUNDARIES.len() - 1], |__t| __t.xp))).wrapping_div(CATA_XP_PER_LEVEL)) as i32);
            let mut updated_boundaries: Vec<compat::Shared<LevelInfo>> = CATACOMBS_SKILL_BOUNDARIES.iter().cloned().collect::<Vec<_>>();
            for i in 0..=additional_levels {
                let mut level: i32 = ((compat::read(&CATACOMBS_SKILL_BOUNDARIES[CATACOMBS_SKILL_BOUNDARIES.len() - 1], |__t| __t.level)).wrapping_add(i)).wrapping_add(1);
                let mut next_level_xp: i64 = (compat::read(&updated_boundaries[updated_boundaries.len() - 1], |__t| __t.xp)).wrapping_add(CATA_XP_PER_LEVEL);
                updated_boundaries.push(LevelInfo::new(next_level_xp, level));
            }
            return updated_boundaries;
        }
        return CATACOMBS_SKILL_BOUNDARIES.clone();
    }

}

/// java: public static class LevelInfo
#[derive(Clone)]
pub struct LevelInfo {
    /// java: public long xp;
    pub xp: i64,
    /// java: public int level;
    pub level: i32,
    /// java: public double fill;
    pub fill: f64,
    /// java: public long levelXP;
    pub level_xp: i64,
    /// java: public long nextLevelXP;
    pub next_level_xp: i64,
}

impl LevelInfo {
    /// java: public LevelInfo(long xp, int level)
    pub fn new(mut xp: i64, mut level: i32) -> compat::Shared<Self> {
        let __init_xp = xp;
        let __init_level = level;
        let __this = compat::shared(Self {
            xp: __init_xp,
            level: __init_level,
            fill: 0.0,
            level_xp: 0,
            next_level_xp: 0,
        });
        __this
    }

    /// java: public LevelInfo(long xp, int level, double fill, double levelXP, double nextLevelXP)
    pub fn new_5(mut xp: i64, mut level: i32, mut fill: f64, mut level_xp: f64, mut next_level_xp: f64) -> compat::Shared<Self> {
        let __init_xp = xp;
        let __init_level = level;
        let __init_fill = fill;
        let __init_level_xp = ((level_xp) as i64);
        let __init_next_level_xp = ((next_level_xp) as i64);
        let __this = compat::shared(Self {
            xp: __init_xp,
            level: __init_level,
            fill: __init_fill,
            level_xp: __init_level_xp,
            next_level_xp: __init_next_level_xp,
        });
        __this
    }

}

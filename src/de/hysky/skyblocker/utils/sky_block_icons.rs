//! Ported from de.hysky.skyblocker.utils.SkyBlockIcons by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: public static final char HEALTH = '\uE010';
pub const HEALTH: char = '';
/// java: public static final char DEFENSE = '\uE008';
pub const DEFENSE: char = '';
/// java: public static final char TRUE_DEFENSE = '\uE027';
pub const TRUE_DEFENSE: char = '';
/// java: public static final char STRENGTH = '\uE00D';
pub const STRENGTH: char = '';
/// java: public static final char CRIT_CHANCE = '\uE02C';
pub const CRIT_CHANCE: char = '';
/// java: public static final char CRIT_DAMAGE = '\uE007';
pub const CRIT_DAMAGE: char = '';
/// java: public static final char ATTACK_SPEED = '\uE001';
pub const ATTACK_SPEED: char = '';
/// java: public static final char FEROCITY = '\uE00B';
pub const FEROCITY: char = '';
/// java: public static final char INTELLIGENCE = '\uE003';
pub const INTELLIGENCE: char = '';
/// java: public static final char MANA = INTELLIGENCE;
pub static MANA: LazyLock<char> = LazyLock::new(|| INTELLIGENCE);
/// java: public static final char ABILITY_DAMAGE = '\uE002';
pub const ABILITY_DAMAGE: char = '';
/// java: public static final char OVERFLOW_MANA = '\uE017';
pub const OVERFLOW_MANA: char = '';
/// java: public static final char VITALITY = '\uE028';
pub const VITALITY: char = '';
/// java: public static final char MINING_FORTUNE = '\uE053';
pub const MINING_FORTUNE: char = '';
/// java: public static final char MINING_SPEED = '\uE015';
pub const MINING_SPEED: char = '';
/// java: public static final char PRISTINE = '\uE01C';
pub const PRISTINE: char = '';
/// java: public static final char FARMING_FORTUNE = '\uE051';
pub const FARMING_FORTUNE: char = '';
/// java: public static final char OVERBLOOM = '\uE02B';
pub const OVERBLOOM: char = '';
/// java: public static final char SWEEP = '\uE023';
pub const SWEEP: char = '';
/// java: public static final char FORAGING_FORTUNE = '\uE054';
pub const FORAGING_FORTUNE: char = '';
/// java: public static final char FISHING_SPEED = '\uE00C';
pub const FISHING_SPEED: char = '';
/// java: public static final char SPEED = '\uE022';
pub const SPEED: char = '';
/// java: public static final char MAGIC_FIND = '\uE01A';
pub const MAGIC_FIND: char = '';
/// java: public static final char COMBAT_WISDOM = '\u262F';
pub const COMBAT_WISDOM: char = '☯';
/// java: public static final char RIFT_TIME = '\uE020';
pub const RIFT_TIME: char = '';
/// java: public static final char AREA = '\uE067';
pub const AREA: char = '';
/// java: public static final char RIFT_AREA = '\uE020';
pub const RIFT_AREA: char = '';
/// java: public static final char UNDEAD = '\uE084';
pub const UNDEAD: char = '';

/// java: public final class SkyBlockIcons
#[derive(Clone)]
pub struct SkyBlockIcons {}

impl SkyBlockIcons {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

}

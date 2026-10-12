//! Ported from de.hysky.skyblocker.skyblock.dungeon.DungeonBoss by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

// TODO(fabric2seer): call to `values(..)` (not a method of this class)
//   java: private static final Map<String, DungeonBoss> BOSSES = Arrays.stream(values()).collect(Collectors.toUnmodifiableMap(DungeonBoss::message, Function.identity()));
/// java: private static final Map<String, DungeonBoss> BOSSES = Arrays.stream(values()).collect(Collectors.toUnmodifiableMap(DungeonBoss::message, Function.identity()));
pub static BOSSES: LazyLock<std::collections::HashMap<String, DungeonBoss>> = LazyLock::new(|| todo!("fabric2seer: the initialiser of `BOSSES` - see the TODO above"));

/// java: public enum DungeonBoss
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DungeonBoss {
    None,
    Bonzo,
    Scarf,
    Professor,
    Thorn,
    Livid,
    Sadan,
    Maxor,
}

impl DungeonBoss {
    pub const VALUES: [DungeonBoss; 8] = [DungeonBoss::None, DungeonBoss::Bonzo, DungeonBoss::Scarf, DungeonBoss::Professor, DungeonBoss::Thorn, DungeonBoss::Livid, DungeonBoss::Sadan, DungeonBoss::Maxor];

    /// java: values()
    pub fn values() -> Vec<DungeonBoss> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            DungeonBoss::None => "NONE",
            DungeonBoss::Bonzo => "BONZO",
            DungeonBoss::Scarf => "SCARF",
            DungeonBoss::Professor => "PROFESSOR",
            DungeonBoss::Thorn => "THORN",
            DungeonBoss::Livid => "LIVID",
            DungeonBoss::Sadan => "SADAN",
            DungeonBoss::Maxor => "MAXOR",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> DungeonBoss {
        match name {
            "NONE" => DungeonBoss::None,
            "BONZO" => DungeonBoss::Bonzo,
            "SCARF" => DungeonBoss::Scarf,
            "PROFESSOR" => DungeonBoss::Professor,
            "THORN" => DungeonBoss::Thorn,
            "LIVID" => DungeonBoss::Livid,
            "SADAN" => DungeonBoss::Sadan,
            "MAXOR" => DungeonBoss::Maxor,
            _ => panic!("IllegalArgumentException: No enum constant DungeonBoss.{name}"),
        }
    }

    /// the constant's constructor data (Java keeps it in the constant's fields)
    pub fn fields(self) -> &'static DungeonBossFields {
        static FIELDS: LazyLock<Vec<DungeonBossFields>> = LazyLock::new(|| vec![
            DungeonBossFields::new(-1, "".to_string()),
            DungeonBossFields::new(1, "[BOSS] Bonzo: Gratz for making it this far, but I'm basically unbeatable.".to_string()),
            DungeonBossFields::new(2, "[BOSS] Scarf: This is where the journey ends for you, Adventurers.".to_string()),
            DungeonBossFields::new(3, "[BOSS] The Professor: I was burdened with terrible news recently...".to_string()),
            DungeonBossFields::new(4, "[BOSS] Thorn: Welcome Adventurers! I am Thorn, the Spirit! And host of the Vegan Trials!".to_string()),
            DungeonBossFields::new(5, "[BOSS] Livid: Welcome, you've arrived right on time. I am Livid, the Master of Shadows.".to_string()),
            DungeonBossFields::new(6, "[BOSS] Sadan: So you made it all the way here... Now you wish to defy me? Sadan?!".to_string()),
            DungeonBossFields::new(7, "[BOSS] Maxor: WELL! WELL! WELL! LOOK WHO'S HERE!".to_string()),
        ]);
        &FIELDS[self as usize]
    }

    /// java: public int floor()
    pub fn floor(&self) -> i32 {
        return self.fields().floor;
    }

    /// java: public String message()
    pub fn message(&self) -> String {
        return self.fields().message.clone();
    }

    /// java: public boolean isInBoss()
    pub fn is_in_boss(&self) -> bool {
        return (*self != DungeonBoss::None);
    }

    /// java: public boolean isFloor(int floor)
    pub fn is_floor(&self, mut floor: i32) -> bool {
        return (self.fields().floor == floor);
    }

    /// java: public static DungeonBoss fromMessage(String message)
    pub fn from_message(message: &str) -> DungeonBoss {
        return BOSSES.get(message).cloned().unwrap_or_else(|| DungeonBoss::None);
    }

}

impl std::fmt::Display for DungeonBoss {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// the constructor data of each DungeonBoss constant
pub struct DungeonBossFields {
    pub floor: i32,
    pub message: String,
}

impl DungeonBossFields {
    fn new(mut floor: i32, mut message: String) -> Self {
        let mut this = Self {
            floor: 0,
            message: Default::default(),
        };
        this.floor = floor;
        this.message = message.clone();
        this
    }

}

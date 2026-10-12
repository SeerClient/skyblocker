//! Ported from de.hysky.skyblocker.skyblock.dungeon.DungeonClass by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::entity::mob_glow;
use crate::de::hysky::skyblocker::skyblock::tabhud::util::ico;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use mcrust::net::minecraft::util::argb;

// TODO(fabric2seer): call to `values(..)` (not a method of this class)
//   java: private static final Map<String, DungeonClass> CLASSES_BY_DISPLAY_NAME = Arrays.stream(values())
//   java: 			.collect(Collectors.toUnmodifiableMap(DungeonClass::displayName, Function.identity()));
/// java: private static final Map<String, DungeonClass> CLASSES_BY_DISPLAY_NAME = Arrays.stream(values()) .collect(Collectors.toUnmodifiableMap(DungeonClass::displayName, Function.identity()));
pub static CLASSES_BY_DISPLAY_NAME: LazyLock<std::collections::HashMap<String, DungeonClass>> = LazyLock::new(|| todo!("fabric2seer: the initialiser of `CLASSES_BY_DISPLAY_NAME` - see the TODO above"));

/// java: public enum DungeonClass
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DungeonClass {
    Unknown,
    Healer,
    Mage,
    Berserk,
    Archer,
    Tank,
}

impl DungeonClass {
    pub const VALUES: [DungeonClass; 6] = [DungeonClass::Unknown, DungeonClass::Healer, DungeonClass::Mage, DungeonClass::Berserk, DungeonClass::Archer, DungeonClass::Tank];

    /// java: values()
    pub fn values() -> Vec<DungeonClass> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            DungeonClass::Unknown => "UNKNOWN",
            DungeonClass::Healer => "HEALER",
            DungeonClass::Mage => "MAGE",
            DungeonClass::Berserk => "BERSERK",
            DungeonClass::Archer => "ARCHER",
            DungeonClass::Tank => "TANK",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> DungeonClass {
        match name {
            "UNKNOWN" => DungeonClass::Unknown,
            "HEALER" => DungeonClass::Healer,
            "MAGE" => DungeonClass::Mage,
            "BERSERK" => DungeonClass::Berserk,
            "ARCHER" => DungeonClass::Archer,
            "TANK" => DungeonClass::Tank,
            _ => panic!("IllegalArgumentException: No enum constant DungeonClass.{name}"),
        }
    }

    /// the constant's constructor data (Java keeps it in the constant's fields)
    pub fn fields(self) -> &'static DungeonClassFields {
        static FIELDS: LazyLock<Vec<DungeonClassFields>> = LazyLock::new(|| vec![
            DungeonClassFields::new("Unknown".to_string(), *mob_glow::NO_GLOW, ico::BARRIER.get_mut().clone()),
            DungeonClassFields::new("Healer".to_string(), 0x820DD1, ico::POTION.get_mut().clone()),
            DungeonClassFields::new("Mage".to_string(), 0x36C6E3, ico::B_ROD.get_mut().clone()),
            DungeonClassFields::new("Berserk".to_string(), 0xFA5B16, ico::IRON_SWORD.get_mut().clone()),
            DungeonClassFields::new("Archer".to_string(), 0xED240E, ico::BOW.get_mut().clone()),
            DungeonClassFields::new("Tank".to_string(), 0x138717, ico::L_CHESTPLATE.get_mut().clone()),
        ]);
        &FIELDS[self as usize]
    }

    /// java: public String displayName()
    pub fn display_name(&self) -> String {
        return self.fields().name.clone();
    }

    /// java: public String apiName()
    pub fn api_name(&self) -> String {
        // TODO(fabric2seer): call to `name(..)` (not a method of this class)
        //   java: return this.name().toLowerCase(Locale.ENGLISH);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public int color()
    pub fn color(&self) -> i32 {
        return self.fields().color;
    }

    /// java: public int glowColor()
    pub fn glow_color(&self) -> i32 {
        return self.fields().glow_color;
    }

    /// java: public FlexibleItemStack icon()
    pub fn icon(&self) -> compat::Shared<FlexibleItemStack> {
        return self.fields().icon.clone();
    }

    /// java: public static DungeonClass from(String name)
    pub fn from(name: &str) -> DungeonClass {
        return CLASSES_BY_DISPLAY_NAME.get(name).cloned().unwrap_or_else(|| UNKNOWN);
    }

}

impl std::fmt::Display for DungeonClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// the constructor data of each DungeonClass constant
pub struct DungeonClassFields {
    pub name: String,
    pub color: i32,
    pub glow_color: i32,
    pub icon: compat::Shared<FlexibleItemStack>,
}

impl DungeonClassFields {
    fn new(mut name: String, mut color: i32, mut icon: compat::Shared<FlexibleItemStack>) -> Self {
        let mut this = Self {
            name: Default::default(),
            color: 0,
            glow_color: 0,
            icon: Default::default(),
        };
        this.name = name.clone();
        this.color = argb::opaque(color);
        this.glow_color = color;
        this.icon = icon.clone();
        this
    }

}

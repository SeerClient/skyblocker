//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.utils.Skill by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::tabhud::util::ico;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;

/// java: public static final Set<Skill> CONVENTIONAL_SKILLS = Set.copyOf(EnumSet.complementOf(EnumSet.of(CATACOMBS, RUNECRAFTING, SOCIAL)));
pub static CONVENTIONAL_SKILLS: LazyLock<std::collections::HashSet<Skill>> = LazyLock::new(|| Skill::values().into_iter().filter(|s| !matches!(s, Skill::Catacombs | Skill::Runecrafting | Skill::Social)).collect());

/// java: public enum Skill
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Skill {
    Alchemy,
    Carpentry,
    Catacombs,
    Combat,
    Enchanting,
    Farming,
    Fishing,
    Foraging,
    Hunting,
    Mining,
    Runecrafting,
    Social,
    Taming,
}

impl Skill {
    pub const VALUES: [Skill; 13] = [Skill::Alchemy, Skill::Carpentry, Skill::Catacombs, Skill::Combat, Skill::Enchanting, Skill::Farming, Skill::Fishing, Skill::Foraging, Skill::Hunting, Skill::Mining, Skill::Runecrafting, Skill::Social, Skill::Taming];

    /// java: values()
    pub fn values() -> Vec<Skill> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            Skill::Alchemy => "ALCHEMY",
            Skill::Carpentry => "CARPENTRY",
            Skill::Catacombs => "CATACOMBS",
            Skill::Combat => "COMBAT",
            Skill::Enchanting => "ENCHANTING",
            Skill::Farming => "FARMING",
            Skill::Fishing => "FISHING",
            Skill::Foraging => "FORAGING",
            Skill::Hunting => "HUNTING",
            Skill::Mining => "MINING",
            Skill::Runecrafting => "RUNECRAFTING",
            Skill::Social => "SOCIAL",
            Skill::Taming => "TAMING",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> Skill {
        match name {
            "ALCHEMY" => Skill::Alchemy,
            "CARPENTRY" => Skill::Carpentry,
            "CATACOMBS" => Skill::Catacombs,
            "COMBAT" => Skill::Combat,
            "ENCHANTING" => Skill::Enchanting,
            "FARMING" => Skill::Farming,
            "FISHING" => Skill::Fishing,
            "FORAGING" => Skill::Foraging,
            "HUNTING" => Skill::Hunting,
            "MINING" => Skill::Mining,
            "RUNECRAFTING" => Skill::Runecrafting,
            "SOCIAL" => Skill::Social,
            "TAMING" => Skill::Taming,
            _ => panic!("IllegalArgumentException: No enum constant Skill.{name}"),
        }
    }

    /// the constant's constructor data (Java keeps it in the constant's fields)
    pub fn fields(self) -> &'static SkillFields {
        static FIELDS: LazyLock<Vec<SkillFields>> = LazyLock::new(|| vec![
            SkillFields::new("Alchemy".to_string(), 50, ico::BREWING_STAND.get_mut().clone()),
            SkillFields::new("Carpentry".to_string(), 50, ico::CRAFTING_TABLE.get_mut().clone()),
            SkillFields::new("Catacombs".to_string(), 50, ico::CATACOMBS.get_mut().clone()),
            SkillFields::new("Combat".to_string(), 60, ico::STONE_SWORD.get_mut().clone()),
            SkillFields::new("Enchanting".to_string(), 60, ico::ENCHANTING_TABLE.get_mut().clone()),
            SkillFields::new_4("Farming".to_string(), 50, 60, ico::GOLDEN_HOE.get_mut().clone()),
            SkillFields::new("Fishing".to_string(), 50, ico::FISH_ROD.get_mut().clone()),
            SkillFields::new_4("Foraging".to_string(), 50, 57, ico::JUNGLE_SAPLING.get_mut().clone()),
            SkillFields::new("Hunting".to_string(), 50, ico::LEAD.get_mut().clone()),
            SkillFields::new("Mining".to_string(), 60, ico::STONE_PICKAXE.get_mut().clone()),
            SkillFields::new("Runecrafting".to_string(), 25, ico::MAGMA_CREAM.get_mut().clone()),
            SkillFields::new("Social".to_string(), 25, ico::EMERALD.get_mut().clone()),
            SkillFields::new_4("Taming".to_string(), 50, 60, ico::SPAWN_EGG.get_mut().clone()),
        ]);
        &FIELDS[self as usize]
    }

    /// java: public String getFriendlyName()
    pub fn get_friendly_name(&self) -> String {
        return self.fields().name.clone();
    }

    /// java: public int getBaseCap()
    pub fn get_base_cap(&self) -> i32 {
        return self.fields().base_cap;
    }

    /// java: public int getAbsoluteCap()
    pub fn get_absolute_cap(&self) -> i32 {
        return self.fields().absolute_cap;
    }

    /// java: public FlexibleItemStack getIcon()
    pub fn get_icon(&self) -> compat::Shared<FlexibleItemStack> {
        return self.fields().icon.clone();
    }

}

impl std::fmt::Display for Skill {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// the constructor data of each Skill constant
pub struct SkillFields {
    pub name: String,
    pub base_cap: i32,
    pub absolute_cap: i32,
    pub icon: compat::Shared<FlexibleItemStack>,
}

impl SkillFields {
    /// java: Skill(String name, int baseCap, FlexibleItemStack icon)
    fn new(name: String, base_cap: i32, icon: compat::Shared<FlexibleItemStack>) -> Self {
        // java: this(name, baseCap, baseCap, icon);
        Self::new_4(name, base_cap, base_cap, icon)
    }

    /// java: Skill(String name, int baseCap, int absoluteCap, FlexibleItemStack icon)
    fn new_4(name: String, base_cap: i32, absolute_cap: i32, icon: compat::Shared<FlexibleItemStack>) -> Self {
        Self { name, base_cap, absolute_cap, icon }
    }

}

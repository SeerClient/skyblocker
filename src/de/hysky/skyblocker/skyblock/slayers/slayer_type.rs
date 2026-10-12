//! Ported from de.hysky.skyblocker.skyblock.slayers.SlayerType by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::profileviewer::slayers::slayer_widget;
use crate::de::hysky::skyblocker::skyblock::slayers::slayer_tier::SlayerTier;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::world::entity::entity::Entity;
use mcrust::net::minecraft::world::entity::entity_type::EntityType;

/// java: public static final Codec<SlayerType> CODEC = StringRepresentable.fromEnum(SlayerType::values);
pub static CODEC: compat::MainThread<CodecRef<SlayerType>> = compat::MainThread::new(|| mcrust::net::minecraft::util::string_representable::from_enum(SlayerType::values(), |e: &SlayerType| e.get_serialized_name()));

/// java: public enum SlayerType implements StringRepresentable
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SlayerType {
    Revenant,
    Tarantula,
    Sven,
    Voidgloom,
    Demonlord,
    Vampire,
}

impl SlayerType {
    pub const VALUES: [SlayerType; 6] = [SlayerType::Revenant, SlayerType::Tarantula, SlayerType::Sven, SlayerType::Voidgloom, SlayerType::Demonlord, SlayerType::Vampire];

    /// java: values()
    pub fn values() -> Vec<SlayerType> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            SlayerType::Revenant => "REVENANT",
            SlayerType::Tarantula => "TARANTULA",
            SlayerType::Sven => "SVEN",
            SlayerType::Voidgloom => "VOIDGLOOM",
            SlayerType::Demonlord => "DEMONLORD",
            SlayerType::Vampire => "VAMPIRE",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> SlayerType {
        match name {
            "REVENANT" => SlayerType::Revenant,
            "TARANTULA" => SlayerType::Tarantula,
            "SVEN" => SlayerType::Sven,
            "VOIDGLOOM" => SlayerType::Voidgloom,
            "DEMONLORD" => SlayerType::Demonlord,
            "VAMPIRE" => SlayerType::Vampire,
            _ => panic!("IllegalArgumentException: No enum constant SlayerType.{name}"),
        }
    }

    /// the constant's constructor data (Java keeps it in the constant's fields)
    pub fn fields(self) -> &'static SlayerTypeFields {
        static FIELDS: LazyLock<Vec<SlayerTypeFields>> = LazyLock::new(|| vec![
            SlayerTypeFields::new("revenant".to_string(), "Zombie".to_string(), compat::entity_type("zombie"), "Revenant Horror".to_string(), slayer_widget::HEAD_ICON.get_mut().get("Zombie").cloned().unwrap(), slayer_widget::DROP_ICON.get_mut().get("Zombie").cloned().unwrap(), vec![500, 20_000, 400_000, 1_500_000, 10_000_000], vec![5, 25, 100, 500, 1500], vec![5, 15, 200, 1000, 5000, 20000, 100000, 400000, 1000000], vec!["Revenant Sycophant".to_string()], vec!["Revenant Champion".to_string(), "Deformed Revenant".to_string()], vec!["Atoned Champion".to_string(), "Atoned Revenant".to_string()]),
            SlayerTypeFields::new("tarantula".to_string(), "Spider".to_string(), compat::entity_type("spider"), "Tarantula Broodfather".to_string(), slayer_widget::HEAD_ICON.get_mut().get("Spider").cloned().unwrap(), slayer_widget::DROP_ICON.get_mut().get("Spider").cloned().unwrap(), vec![750, 30_000, 900_000, 2_400_000, 10_000_000], vec![5, 25, 100, 500, 1500], vec![5, 25, 200, 1000, 5000, 20000, 100000, 400000, 1000000], vec!["Tarantula Vermin".to_string()], vec!["Tarantula Beast".to_string(), "Mutant Tarantula".to_string()], vec!["Primordial Jockey".to_string(), "Primordial Viscount".to_string()]),
            SlayerTypeFields::new("sven".to_string(), "Wolf".to_string(), compat::entity_type("wolf"), "Sven Packmaster".to_string(), slayer_widget::HEAD_ICON.get_mut().get("Wolf").cloned().unwrap(), slayer_widget::DROP_ICON.get_mut().get("Wolf").cloned().unwrap(), vec![2_000, 40_000, 750_000, 2_000_000], vec![5, 25, 100, 500, 1500], vec![10, 30, 250, 1500, 5000, 20000, 100000, 400000, 1000000], vec!["Pack Enforcer".to_string()], vec!["Sven Follower".to_string(), "Sven Alpha".to_string()], vec![]),
            SlayerTypeFields::new("voidgloom".to_string(), "Enderman".to_string(), compat::entity_type("enderman"), "Voidgloom Seraph".to_string(), slayer_widget::HEAD_ICON.get_mut().get("Enderman").cloned().unwrap(), slayer_widget::DROP_ICON.get_mut().get("Enderman").cloned().unwrap(), vec![300_000, 12_000_000, 50_000_000, 210_000_000], vec![5, 25, 100, 500, 1500], vec![10, 30, 250, 1500, 5000, 20000, 100000, 400000, 1000000], vec!["Voidling Devotee".to_string()], vec!["Voidling Radical".to_string(), "Voidcrazed Maniac".to_string()], vec![]),
            SlayerTypeFields::new("demonlord".to_string(), "Blaze".to_string(), compat::entity_type("blaze"), "Inferno Demonlord".to_string(), slayer_widget::HEAD_ICON.get_mut().get("Blaze").cloned().unwrap(), slayer_widget::DROP_ICON.get_mut().get("Blaze").cloned().unwrap(), vec![2_500_000, 10_000_000, 45_000_000, 150_000_000], vec![5, 25, 100, 500, 1500], vec![10, 30, 250, 1500, 5000, 20000, 100000, 400000, 1000000], vec!["Flare Demon".to_string()], vec!["Kindleheart Demon".to_string(), "Burningsoul Demon".to_string()], vec![]),
            SlayerTypeFields::new("vampire".to_string(), "Vampire".to_string(), mcrust::net::minecraft::world::entity::entity_types::player(), "Riftstalker Bloodfiend".to_string(), slayer_widget::HEAD_ICON.get_mut().get("Vampire").cloned().unwrap(), slayer_widget::DROP_ICON.get_mut().get("Vampire").cloned().unwrap(), vec![625, 1100, 1800, 2400, 3000], vec![10, 25, 60, 120, 150], vec![20, 75, 240, 840, 2400], vec![], vec![], vec![]),
        ]);
        &FIELDS[self as usize]
    }

    /// java: public static @Nullable SlayerType fromBossName(String bossName)
    pub fn from_boss_name(boss_name: &str) -> Option<SlayerType> {
        return match boss_name {
            "Revenant Horror" | "Atoned Horror" => Some(SlayerType::Revenant),
            "Tarantula Broodfather" | "Conjoined Brood" => Some(SlayerType::Tarantula),
            "Sven Packmaster" => Some(SlayerType::Sven),
            "Voidgloom Seraph" => Some(SlayerType::Voidgloom),
            "Inferno Demonlord" => Some(SlayerType::Demonlord),
            "Riftstalker Bloodfiend" | "Bloodfiend" => Some(SlayerType::Vampire),
            _ => None,
        };
    }

    /// java: public boolean isMiniboss(String name, SlayerTier slayerTier)
    pub fn is_miniboss(&self, name: &str, mut slayer_tier: SlayerTier) -> bool {
        let mut miniboss_lists: Vec<Vec<String>> = match slayer_tier {
            SlayerTier::Iii => vec![self.fields().t3_minibosses.clone()],
            SlayerTier::Iv => vec![self.fields().t3_minibosses.clone(), self.fields().t4_minibosses.clone()],
            SlayerTier::V => vec![self.fields().t3_minibosses.clone(), self.fields().t4_minibosses.clone(), self.fields().t5_minibosses.clone()],
            _ => vec![],
        };
        return miniboss_lists.iter().flat_map(|list| list.iter()).any(|miniboss| name.contains(miniboss.as_str()));
    }

    /// java: public EntityType<?> getMinibossType(String minibossName)
    pub fn get_miniboss_type(&self, miniboss_name: &str) -> Arc<EntityType> {
        if *self == SlayerType::Tarantula {
            if miniboss_name.contains("Primordial Jockey") {
                return compat::entity_type("skeleton");
            }
            if miniboss_name.contains("Primordial Viscount") {
                return compat::entity_type("cave_spider");
            }
        }
        return self.fields().mob_type.clone();
    }

    /// java: public int getHealth(String bossName, SlayerTier slayerTier)
    pub fn get_health(&self, boss_name: &str, mut slayer_tier: SlayerTier) -> i32 {
        if boss_name.contains("Conjoined Brood") {
            return 20000000;
        }
        return self.fields().health[(slayer_tier.ordinal()) as usize];
    }

    /// java: @Override public String getSerializedName()
    pub fn get_serialized_name(&self) -> String {
        return self.fields().name.clone();
    }

}

impl std::fmt::Display for SlayerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// the constructor data of each SlayerType constant
pub struct SlayerTypeFields {
    pub name: String,
    pub friendly_name: String,
    pub mob_type: Arc<EntityType>,
    pub boss_name: String,
    pub texture: Identifier,
    pub drop_icon: compat::Shared<FlexibleItemStack>,
    pub health: Vec<i32>,
    pub max_level: i32,
    pub xp_per_tier: Vec<i32>,
    pub level_milestones: Vec<i32>,
    pub t3_minibosses: Vec<String>,
    pub t4_minibosses: Vec<String>,
    pub t5_minibosses: Vec<String>,
}

impl SlayerTypeFields {
    fn new(mut name: String, mut friendly_name: String, mut mob_type: Arc<EntityType>, mut boss_name: String, mut texture: Identifier, mut drop_icon: compat::Shared<FlexibleItemStack>, mut health: Vec<i32>, mut xp_per_tier: Vec<i32>, mut level_milestones: Vec<i32>, mut t3_minibosses: Vec<String>, mut t4_minibosses: Vec<String>, mut t5_minibosses: Vec<String>) -> Self {
        Self {
            max_level: level_milestones.len() as i32,
            name,
            friendly_name,
            mob_type,
            boss_name,
            texture,
            drop_icon,
            health,
            xp_per_tier,
            level_milestones,
            t3_minibosses,
            t4_minibosses,
            t5_minibosses,
        }
    }

}

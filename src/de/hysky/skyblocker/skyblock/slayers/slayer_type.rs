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

// TODO(fabric2seer): `from_enum` takes 2 argument(s) in the port (`values: Vec<E>, serialized_name: impl Fn(&E) -> String + Send + Sync + Copy + 'static`), Java passes 1
//   java: public static final Codec<SlayerType> CODEC = StringRepresentable.fromEnum(SlayerType::values);
/// java: public static final Codec<SlayerType> CODEC = StringRepresentable.fromEnum(SlayerType::values);
pub static CODEC: compat::MainThread<CodecRef<SlayerType>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `CODEC` - see the TODO above"));

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
            todo!("REVENANT"),
            todo!("TARANTULA"),
            todo!("SVEN"),
            todo!("VOIDGLOOM"),
            todo!("DEMONLORD"),
            SlayerTypeFields::new("vampire".to_string(), "Vampire".to_string(), mcrust::net::minecraft::world::entity::entity_types::player(), "Riftstalker Bloodfiend".to_string(), slayer_widget::HEAD_ICON.get_mut().get("Vampire").cloned().unwrap(), slayer_widget::DROP_ICON.get_mut().get("Vampire").cloned().unwrap(), vec![625, 1100, 1800, 2400, 3000], vec![10, 25, 60, 120, 150], vec![20, 75, 240, 840, 2400], vec![], vec![], vec![]),
        ]);
        &FIELDS[self as usize]
    }

    /// java: public static @Nullable SlayerType fromBossName(String bossName)
    pub fn from_boss_name(boss_name: &str) -> Option<SlayerType> {
        return Some(match boss_name.as_str() {
            "Revenant Horror" | "Atoned Horror" => REVENANT,
            "Tarantula Broodfather" | "Conjoined Brood" => TARANTULA,
            "Sven Packmaster" => SVEN,
            "Voidgloom Seraph" => VOIDGLOOM,
            "Inferno Demonlord" => DEMONLORD,
            "Riftstalker Bloodfiend" | "Bloodfiend" => VAMPIRE,
            _ => None,
        });
    }

    /// java: public boolean isMiniboss(String name, SlayerTier slayerTier)
    pub fn is_miniboss(&self, name: &str, mut slayer_tier: SlayerTier) -> bool {
        let mut miniboss_lists: Vec<Vec<String>> = match slayer_tier {
            SlayerTier::Iii => vec![self.fields().t3_minibosses.clone()],
            SlayerTier::Iv => vec![self.fields().t3_minibosses.clone(), self.fields().t4_minibosses.clone()],
            SlayerTier::V => vec![self.fields().t3_minibosses.clone(), self.fields().t4_minibosses.clone(), self.fields().t5_minibosses.clone()],
            _ => vec![],
        };
        // TODO(fabric2seer): static `List.stream` has no Rust counterpart in the port
        // rust: return miniboss_lists.iter().cloned().flat_map(|__a0| List::stream(__a0.clone()).iter().cloned()).any(|__a0| name.contains(&compat::jstring(&__a0)));
        return todo!("fabric2seer: see the TODO above");
    }

    /// java: public EntityType<?> getMinibossType(String minibossName)
    pub fn get_miniboss_type(&self, miniboss_name: &str) -> Arc<EntityType> {
        if *self == TARANTULA {
            if miniboss_name.contains("Primordial Jockey") {
                // TODO(fabric2seer): static `EntityTypes.SKELETON` has no Rust counterpart in the port's EntityTypes file
                // rust: return crate::unresolved::EntityTypes::SKELETON.clone();
                return todo!("fabric2seer: see the TODO above");
            }
            if miniboss_name.contains("Primordial Viscount") {
                // TODO(fabric2seer): static `EntityTypes.CAVE_SPIDER` has no Rust counterpart in the port's EntityTypes file
                // rust: return crate::unresolved::EntityTypes::CAVE_SPIDER.clone();
                return todo!("fabric2seer: see the TODO above");
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
        let mut this = Self {
            name: Default::default(),
            friendly_name: Default::default(),
            mob_type: Default::default(),
            boss_name: Default::default(),
            texture: Default::default(),
            drop_icon: Default::default(),
            health: Default::default(),
            max_level: 0,
            xp_per_tier: Default::default(),
            level_milestones: Default::default(),
            t3_minibosses: Default::default(),
            t4_minibosses: Default::default(),
            t5_minibosses: Default::default(),
        };
        this.name = name.clone();
        this.friendly_name = friendly_name.clone();
        this.mob_type = mob_type.clone();
        this.boss_name = boss_name.clone();
        this.texture = texture.clone();
        this.drop_icon = drop_icon.clone();
        this.health = health.clone();
        this.max_level = (level_milestones.len() as i32);
        this.xp_per_tier = xp_per_tier.clone();
        this.level_milestones = level_milestones.clone();
        this.t3_minibosses = t3_minibosses.clone();
        this.t4_minibosses = t4_minibosses.clone();
        this.t5_minibosses = t5_minibosses.clone();
        this
    }

}

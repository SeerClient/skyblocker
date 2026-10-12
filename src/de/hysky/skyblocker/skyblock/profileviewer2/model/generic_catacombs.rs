//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.model.GenericCatacombs by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::java::util::uuid::Uuid;

/// java: public class GenericCatacombs
#[derive(Clone)]
pub struct GenericCatacombsBase {
    /// java: @SerializedName("best_score") public PersonalBest bestScore = new PersonalBest();
    pub best_score: compat::Shared<PersonalBest>,
    /// java: @SerializedName("fastest_time") public PersonalBest fastestTime = new PersonalBest();
    pub fastest_time: compat::Shared<PersonalBest>,
    /// java: @SerializedName("mobs_killed") public AggregateStat mobsKilled = new AggregateStat();
    pub mobs_killed: compat::Shared<AggregateStat>,
    /// java: @SerializedName("most_mobs_killed") public PersonalBest mobsKilledInOneRun = new PersonalBest();
    pub mobs_killed_in_one_run: compat::Shared<PersonalBest>,
    /// java: @SerializedName("most_healing") public PersonalBest mostHealing = new PersonalBest();
    pub most_healing: compat::Shared<PersonalBest>,
    /// java: @SerializedName("tier_completions") public AggregateStat tierCompletions = new AggregateStat();
    pub tier_completions: compat::Shared<AggregateStat>,
    /// java: @SerializedName("milestone_completions") public AggregateStat milestoneCompletions = new AggregateStat();
    pub milestone_completions: compat::Shared<AggregateStat>,
    /// java: @SerializedName("fastest_time_s") public PersonalBest fastestTimeS = new PersonalBest();
    pub fastest_time_s: compat::Shared<PersonalBest>,
    /// java: @SerializedName("fastest_time_s_plus") public PersonalBest fastestTimeSPlus = new PersonalBest();
    pub fastest_time_s_plus: compat::Shared<PersonalBest>,
    /// java: @SerializedName("most_damage_mage") public PersonalBest mostMageDamage = new PersonalBest();
    pub most_mage_damage: compat::Shared<PersonalBest>,
    /// java: @SerializedName("most_damage_archer") public PersonalBest mostDamageArcher = new PersonalBest();
    pub most_damage_archer: compat::Shared<PersonalBest>,
    /// java: @SerializedName("most_damage_berserk") public PersonalBest mostDamageBeserk = new PersonalBest();
    pub most_damage_beserk: compat::Shared<PersonalBest>,
    /// java: @SerializedName("most_damage_tank") public PersonalBest mostDamageTank = new PersonalBest();
    pub most_damage_tank: compat::Shared<PersonalBest>,
    /// java: @SerializedName("most_damage_healer") public PersonalBest mostDamageHealer = new PersonalBest();
    pub most_damage_healer: compat::Shared<PersonalBest>,
    /// java: @SerializedName("highest_tier_completed") public int highestTierCompleted;
    pub highest_tier_completed: i32,
    /// java: @SerializedName("best_runs") public Map<String, List<BestRun>> bestRuns = Map.of();
    pub best_runs: std::collections::HashMap<String, Vec<BestRun>>,
}

impl GenericCatacombsBase {
    /// java: the implicit no-argument constructor (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_in<__R>(__wrap: impl FnOnce(Self) -> (compat::Shared<dyn GenericCatacombs>, __R)) -> (compat::Shared<dyn GenericCatacombs>, __R) {
        let this = Self {
            best_score: PersonalBest::new(),
            fastest_time: PersonalBest::new(),
            mobs_killed: AggregateStat::new(),
            mobs_killed_in_one_run: PersonalBest::new(),
            most_healing: PersonalBest::new(),
            tier_completions: AggregateStat::new(),
            milestone_completions: AggregateStat::new(),
            fastest_time_s: PersonalBest::new(),
            fastest_time_s_plus: PersonalBest::new(),
            most_mage_damage: PersonalBest::new(),
            most_damage_archer: PersonalBest::new(),
            most_damage_beserk: PersonalBest::new(),
            most_damage_tank: PersonalBest::new(),
            most_damage_healer: PersonalBest::new(),
            highest_tier_completed: 0,
            best_runs: std::collections::HashMap::from([]),
        };
        let (__h, __r) = __wrap(this);
        let __this = __h;
        (__this, __r)
    }

    /// java: the implicit no-argument constructor
    pub fn new() -> compat::Shared<Self> {
        Self::new_in(|this| {
            let __h = compat::shared(this);
            ((__h.clone() as compat::Shared<dyn GenericCatacombs>), __h)
        }).1
    }

}

/// java: the virtual methods of GenericCatacombs (implemented by GenericCatacombsBase and every subclass)
pub trait GenericCatacombs: compat::DynAny {
    fn generic_catacombs_base(&self) -> &GenericCatacombsBase;
    fn generic_catacombs_base_mut(&mut self) -> &mut GenericCatacombsBase;
    fn as_any(&self) -> &dyn std::any::Any;

}

impl GenericCatacombs for GenericCatacombsBase {
    fn generic_catacombs_base(&self) -> &GenericCatacombsBase {
        &self
    }
    fn generic_catacombs_base_mut(&mut self) -> &mut GenericCatacombsBase {
        &mut self
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}

/// java: public static class BestRun
#[derive(Clone)]
pub struct BestRun {
    /// java: public long timestamp;
    pub timestamp: i64,
    /// java: @SerializedName("score_exploration") public int scoreExploration;
    pub score_exploration: i32,
    /// java: @SerializedName("score_speed") public int scoreSpeed;
    pub score_speed: i32,
    /// java: @SerializedName("score_skill") public int scoreSkill;
    pub score_skill: i32,
    /// java: @SerializedName("score_bonus") public int scoreBonus;
    pub score_bonus: i32,
    /// java: @SerializedName("dungeon_class") public String dungeonClass;
    pub dungeon_class: String,
    /// java: public List<UUID> teammates = List.of();
    pub teammates: Vec<Uuid>,
    /// java: @SerializedName("elapsed_time") public int elapsedTime;
    pub elapsed_time: i32,
    /// java: @SerializedName("damage_dealt") public double damageDealt;
    pub damage_dealt: f64,
    /// java: public int deaths;
    pub deaths: i32,
    /// java: @SerializedName("mobs_killed") public int mobsKilled;
    pub mobs_killed: i32,
    /// java: @SerializedName("secrets_found") public int secretsFound;
    pub secrets_found: i32,
    /// java: @SerializedName("damage_mitigated") public double damageMitigated;
    pub damage_mitigated: f64,
    /// java: @SerializedName("ally_healing") public double allyHealing;
    pub ally_healing: f64,
}

impl BestRun {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            timestamp: 0,
            score_exploration: 0,
            score_speed: 0,
            score_skill: 0,
            score_bonus: 0,
            dungeon_class: Default::default(),
            teammates: vec![],
            elapsed_time: 0,
            damage_dealt: 0.0,
            deaths: 0,
            mobs_killed: 0,
            secrets_found: 0,
            damage_mitigated: 0.0,
            ally_healing: 0.0,
        };
        this
    }

}

/// java: public static class PerFloorDisambiguation
#[derive(Clone)]
pub struct PerFloorDisambiguationBase {
    /// java: @SerializedName("0") public @Nullable Double entrance;
    pub entrance: Option<f64>,
    /// java: @SerializedName("1") public @Nullable Double one;
    pub one: Option<f64>,
    /// java: @SerializedName("2") public @Nullable Double two;
    pub two: Option<f64>,
    /// java: @SerializedName("3") public @Nullable Double three;
    pub three: Option<f64>,
    /// java: @SerializedName("4") public @Nullable Double four;
    pub four: Option<f64>,
    /// java: @SerializedName("5") public @Nullable Double five;
    pub five: Option<f64>,
    /// java: @SerializedName("6") public @Nullable Double six;
    pub six: Option<f64>,
    /// java: @SerializedName("7") public @Nullable Double seven;
    pub seven: Option<f64>,
}

impl PerFloorDisambiguationBase {
    /// java: the implicit no-argument constructor (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_in<__R>(__wrap: impl FnOnce(Self) -> (compat::Shared<dyn PerFloorDisambiguation>, __R)) -> (compat::Shared<dyn PerFloorDisambiguation>, __R) {
        let this = Self {
            entrance: None,
            one: None,
            two: None,
            three: None,
            four: None,
            five: None,
            six: None,
            seven: None,
        };
        let (__h, __r) = __wrap(this);
        let __this = __h;
        (__this, __r)
    }

    /// java: the implicit no-argument constructor
    pub fn new() -> compat::Shared<Self> {
        Self::new_in(|this| {
            let __h = compat::shared(this);
            ((__h.clone() as compat::Shared<dyn PerFloorDisambiguation>), __h)
        }).1
    }

}

/// java: the virtual methods of PerFloorDisambiguation (implemented by PerFloorDisambiguationBase and every subclass)
pub trait PerFloorDisambiguation: compat::DynAny {
    fn per_floor_disambiguation_base(&self) -> &PerFloorDisambiguationBase;
    fn per_floor_disambiguation_base_mut(&mut self) -> &mut PerFloorDisambiguationBase;
    fn as_any(&self) -> &dyn std::any::Any;

    /// java: public double getValueOrZero(int oneIndexedFloor) (dispatch: the body for this class)
    fn get_value_or_zero(&self) -> fn(&compat::Shared<dyn PerFloorDisambiguation>, i32) -> f64 {
        <dyn PerFloorDisambiguation>::get_value_or_zero_body
    }

    /// java: public @Nullable Double getValue(int oneIndexedFloor) (dispatch: the body for this class)
    fn get_value(&self) -> fn(&compat::Shared<dyn PerFloorDisambiguation>, i32) -> Option<f64> {
        <dyn PerFloorDisambiguation>::get_value_body
    }

}

impl dyn PerFloorDisambiguation {
    /// java: public double getValueOrZero(int oneIndexedFloor)
    pub fn get_value_or_zero_body(__this: &compat::Shared<dyn PerFloorDisambiguation>, mut one_indexed_floor: i32) -> f64 {
        let Some(mut value) = ({ let __f = compat::dispatch(__this, |__t| __t.get_value()); __f(__this, one_indexed_floor) }) else {
            return ((0) as f64) as f64;
        };
        return value;
    }

    /// java: public @Nullable Double getValue(int oneIndexedFloor)
    pub fn get_value_body(__this: &compat::Shared<dyn PerFloorDisambiguation>, mut one_indexed_floor: i32) -> Option<f64> {
        return Some(match one_indexed_floor {
            0 => compat::read(__this, |__t| __t.per_floor_disambiguation_base().entrance.clone()).unwrap(),
            1 => compat::read(__this, |__t| __t.per_floor_disambiguation_base().one.clone()).unwrap(),
            2 => compat::read(__this, |__t| __t.per_floor_disambiguation_base().two.clone()).unwrap(),
            3 => compat::read(__this, |__t| __t.per_floor_disambiguation_base().three.clone()).unwrap(),
            4 => compat::read(__this, |__t| __t.per_floor_disambiguation_base().four.clone()).unwrap(),
            5 => compat::read(__this, |__t| __t.per_floor_disambiguation_base().five.clone()).unwrap(),
            6 => compat::read(__this, |__t| __t.per_floor_disambiguation_base().six.clone()).unwrap(),
            7 => compat::read(__this, |__t| __t.per_floor_disambiguation_base().seven.clone()).unwrap(),
            _ => {
                panic!("IllegalStateException: {}", format!("Unexpected floor: {}", one_indexed_floor).to_string());
            }
        });
    }

}

impl PerFloorDisambiguation for PerFloorDisambiguationBase {
    fn per_floor_disambiguation_base(&self) -> &PerFloorDisambiguationBase {
        &self
    }
    fn per_floor_disambiguation_base_mut(&mut self) -> &mut PerFloorDisambiguationBase {
        &mut self
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}

/// java: public static class PersonalBest extends PerFloorDisambiguation
#[derive(Clone)]
pub struct PersonalBest {
    /// java: extends PerFloorDisambiguation
    pub base: PerFloorDisambiguationBase,
    /// java: public @Nullable Double best;
    pub best: Option<f64>,
}

impl std::ops::Deref for PersonalBest {
    type Target = PerFloorDisambiguationBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for PersonalBest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl PersonalBest {
    /// java: the implicit no-argument constructor
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = PerFloorDisambiguationBase::new_in(move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                best: None,
            });
            ((__h.clone() as compat::Shared<dyn PerFloorDisambiguation>), __h)
        });
        __this
    }

}

impl PerFloorDisambiguation for PersonalBest {
    fn per_floor_disambiguation_base(&self) -> &PerFloorDisambiguationBase {
        &self.base
    }
    fn per_floor_disambiguation_base_mut(&mut self) -> &mut PerFloorDisambiguationBase {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}

/// java: public static class AggregateStat extends PerFloorDisambiguation
#[derive(Clone)]
pub struct AggregateStat {
    /// java: extends PerFloorDisambiguation
    pub base: PerFloorDisambiguationBase,
    /// java: public double total;
    pub total: f64,
}

impl std::ops::Deref for AggregateStat {
    type Target = PerFloorDisambiguationBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for AggregateStat {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl AggregateStat {
    /// java: the implicit no-argument constructor
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = PerFloorDisambiguationBase::new_in(move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                total: 0.0,
            });
            ((__h.clone() as compat::Shared<dyn PerFloorDisambiguation>), __h)
        });
        __this
    }

    /// java: private static double coerce0(@Nullable Double d)
    pub fn coerce0(mut d: f64) -> f64 {
        return (if true /* java: d != null: never null here */ { d } else { ((0) as f64) as f64 });
    }

    /// java: public double getManuallyCalculatedTotal()
    pub fn get_manually_calculated_total(__this: &compat::Shared<Self>) -> f64 {
        let mut result: f64 = ((0) as f64) as f64;
        result = (result + AggregateStat::coerce0(compat::read(__this, |__t| __t.entrance.clone()).unwrap()));
        result = (result + AggregateStat::coerce0(compat::read(__this, |__t| __t.one.clone()).unwrap()));
        result = (result + AggregateStat::coerce0(compat::read(__this, |__t| __t.two.clone()).unwrap()));
        result = (result + AggregateStat::coerce0(compat::read(__this, |__t| __t.three.clone()).unwrap()));
        result = (result + AggregateStat::coerce0(compat::read(__this, |__t| __t.four.clone()).unwrap()));
        result = (result + AggregateStat::coerce0(compat::read(__this, |__t| __t.five.clone()).unwrap()));
        result = (result + AggregateStat::coerce0(compat::read(__this, |__t| __t.six.clone()).unwrap()));
        result = (result + AggregateStat::coerce0(compat::read(__this, |__t| __t.seven.clone()).unwrap()));
        return result;
    }

}

impl PerFloorDisambiguation for AggregateStat {
    fn per_floor_disambiguation_base(&self) -> &PerFloorDisambiguationBase {
        &self.base
    }
    fn per_floor_disambiguation_base_mut(&mut self) -> &mut PerFloorDisambiguationBase {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}

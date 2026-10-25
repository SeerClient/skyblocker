//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.model.PlayerStats by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: public class PlayerStats
#[derive(Clone)]
pub struct PlayerStats {
    /// java: @SerializedName("sea_creature_kills") public int seaCreatureKills;
    pub sea_creature_kills: i32,
    /// java: @SerializedName("glowing_mushrooms_broken") public float glowingMushroomsBroken;
    pub glowing_mushrooms_broken: f32,
    /// java: @SerializedName("highest_damage") public long highestDamage;
    pub highest_damage: i64,
    /// java: @SerializedName("highest_critical_damage") public long highestCriticalDamage;
    pub highest_critical_damage: i64,
    /// java: public Map<String, Integer> kills = Map.of();
    pub kills: std::collections::HashMap<String, i32>,
    /// java: public Map<String, Integer> deaths = Map.of();
    pub deaths: std::collections::HashMap<String, i32>,
    /// java: public Pets pets = new Pets();
    pub pets: Pets,
    /// java: public Gifts gifts = new Gifts();
    pub gifts: Gifts,
    /// java: public Auctions auctions = new Auctions();
    pub auctions: Auctions,
    /// java: public ItemsFished items_fished = new ItemsFished();
    pub items_fished: ItemsFished,
    /// java: public Rift rift = new Rift();
    pub rift: Rift,
    /// java: @SerializedName("mythos") public MythologicalData mythologicalData = new MythologicalData();
    pub mythological_data: MythologicalData,
    /// java: @SerializedName("unique_shards") public int uniqueShards;
    pub unique_shards: i32,
    /// java: @SerializedName("shard_combat_hunts") public int combatHunts;
    pub combat_hunts: i32,
    /// java: @SerializedName("shard_fishing_hunts") public int fishingHunts;
    pub fishing_hunts: i32,
    /// java: @SerializedName("shard_forest_hunts") public int forestHunts;
    pub forest_hunts: i32,
    /// java: @SerializedName("shard_salt_hunts") public int saltHunts;
    pub salt_hunts: i32,
    /// java: @SerializedName("shard_trap_hunts") public int trapHunts;
    pub trap_hunts: i32,
}

impl PlayerStats {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            sea_creature_kills: 0,
            glowing_mushrooms_broken: 0.0,
            highest_damage: 0,
            highest_critical_damage: 0,
            kills: std::collections::HashMap::from([]),
            deaths: std::collections::HashMap::from([]),
            pets: Pets::new(),
            gifts: Gifts::new(),
            auctions: Auctions::new(),
            items_fished: ItemsFished::new(),
            rift: Rift::new(),
            mythological_data: MythologicalData::new(),
            unique_shards: 0,
            combat_hunts: 0,
            fishing_hunts: 0,
            forest_hunts: 0,
            salt_hunts: 0,
            trap_hunts: 0,
        };
        this
    }

}

/// java: public static class Pets
#[derive(Clone)]
pub struct Pets {
    /// java: @SerializedName("total_exp_gained") public double totalExpGained;
    pub total_exp_gained: f64,
    /// java: public Milestone milestone = new Milestone();
    pub milestone: Milestone,
}

impl Pets {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            total_exp_gained: 0.0,
            milestone: Milestone::new(),
        };
        this
    }

}

/// java: public static class Milestone
#[derive(Clone)]
pub struct Milestone {
    /// java: @SerializedName("sea_creatures_killed") public int seaCreaturesKilled;
    pub sea_creatures_killed: i32,
    /// java: @SerializedName("ores_mined") public int oresMined;
    pub ores_mined: i32,
}

impl Milestone {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            sea_creatures_killed: 0,
            ores_mined: 0,
        };
        this
    }

}

/// java: public static class Gifts
#[derive(Clone)]
pub struct Gifts {
    /// java: @SerializedName("total_given") public int totalGiven;
    pub total_given: i32,
    /// java: @SerializedName("total_received") public int totalReceived;
    pub total_received: i32,
}

impl Gifts {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            total_given: 0,
            total_received: 0,
        };
        this
    }

}

/// java: public static class Auctions
#[derive(Clone)]
pub struct Auctions {
    /// java: @SerializedName("highest_bid") public double highestBid;
    pub highest_bid: f64,
    /// java: public int bids;
    pub bids: i32,
    /// java: public int won;
    pub won: i32,
    /// java: public int created;
    pub created: i32,
    /// java: @SerializedName("gold_spent") public double goldSpent;
    pub gold_spent: f64,
    /// java: @SerializedName("gold_earned") public double goldEarned;
    pub gold_earned: f64,
    /// java: @SerializedName("no_bids") public int noBids;
    pub no_bids: i32,
    /// java: public double fees;
    pub fees: f64,
    /// java: @SerializedName("total_bought") public RarityStats totalBought = new RarityStats();
    pub total_bought: compat::Shared<RarityStats>,
    /// java: @SerializedName("total_sold") public RarityStats totalSold = new RarityStats();
    pub total_sold: compat::Shared<RarityStats>,
}

impl Auctions {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            highest_bid: 0.0,
            bids: 0,
            won: 0,
            created: 0,
            gold_spent: 0.0,
            gold_earned: 0.0,
            no_bids: 0,
            fees: 0.0,
            total_bought: RarityStats::new(),
            total_sold: RarityStats::new(),
        };
        this
    }

}

/// java: public static class RarityStats
#[derive(Clone)]
pub struct RarityStats {
    /// java: @SerializedName("COMMON") public int common;
    pub common: i32,
    /// java: @SerializedName("UNCOMMON") public int uncommon;
    pub uncommon: i32,
    /// java: @SerializedName("RARE") public int rare;
    pub rare: i32,
    /// java: @SerializedName("EPIC") public int epic;
    pub epic: i32,
    /// java: @SerializedName("LEGENDARY") public int legendary;
    pub legendary: i32,
    /// java: @SerializedName("MYTHIC") public int mythic;
    pub mythic: i32,
    /// java: @SerializedName("ULTIMATE") public int ultimate;
    pub ultimate: i32,
    /// java: @SerializedName("SPECIAL") public int special;
    pub special: i32,
    /// java: public int total;
    pub total: i32,
}

impl RarityStats {
    /// java: the implicit no-argument constructor
    pub fn new() -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            common: 0,
            uncommon: 0,
            rare: 0,
            epic: 0,
            legendary: 0,
            mythic: 0,
            ultimate: 0,
            special: 0,
            total: 0,
        });
        __this
    }

}

/// java: public static class ItemsFished
#[derive(Clone)]
pub struct ItemsFished {
    /// java: public int total;
    pub total: i32,
    /// java: public int normal;
    pub normal: i32,
    /// java: public int treasure;
    pub treasure: i32,
    /// java: @SerializedName("large_treasure") public int largeTreasure;
    pub large_treasure: i32,
    /// java: @SerializedName("trophy_fish") public int trophyFish;
    pub trophy_fish: i32,
    /// java: @SerializedName("trophy_frog") public int trophyFrog;
    pub trophy_frog: i32,
    /// java: public int outstanding;
    pub outstanding: i32,
}

impl ItemsFished {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            total: 0,
            normal: 0,
            treasure: 0,
            large_treasure: 0,
            trophy_fish: 0,
            trophy_frog: 0,
            outstanding: 0,
        };
        this
    }

}

/// java: public static class Rift
#[derive(Clone)]
pub struct Rift {
    /// java: @SerializedName("visits") public int visits;
    pub visits: i32,
    /// java: @SerializedName("pass_consumed") public int passConsumed;
    pub pass_consumed: i32,
    /// java: @SerializedName("lifetime_motes_earned") public int lifetimeMotesEarned;
    pub lifetime_motes_earned: i32,
    /// java: @SerializedName("motes_orb_pickup") public int motesOrbPickup;
    pub motes_orb_pickup: i32,
    /// java: @SerializedName("lagoon_leech_supreme_killed") public int leechSupremeKills;
    pub leech_supreme_kills: i32,
    /// java: @SerializedName("colosseum_bacte_defeated") public int bacteKills;
    pub bacte_kills: i32,
    /// java: @SerializedName("living_cave_snake_collected") public int livingCaveSnakesCollected;
    pub living_cave_snakes_collected: i32,
    /// java: @SerializedName("castle_sent_to_prison") public int timesSentToPrison;
    pub times_sent_to_prison: i32,
    /// java: @SerializedName("castle_effigy_broken") public int castleEffigiesBroken;
    pub castle_effigies_broken: i32,
}

impl Rift {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`); an absent member leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "visits") {
            if let Some(__x) = gson::as_i32(__v) {
                self.visits = __x;
            }
        }
        if let Some(__v) = gson::member(json, "pass_consumed") {
            if let Some(__x) = gson::as_i32(__v) {
                self.pass_consumed = __x;
            }
        }
        if let Some(__v) = gson::member(json, "lifetime_motes_earned") {
            if let Some(__x) = gson::as_i32(__v) {
                self.lifetime_motes_earned = __x;
            }
        }
        if let Some(__v) = gson::member(json, "motes_orb_pickup") {
            if let Some(__x) = gson::as_i32(__v) {
                self.motes_orb_pickup = __x;
            }
        }
        if let Some(__v) = gson::member(json, "lagoon_leech_supreme_killed") {
            if let Some(__x) = gson::as_i32(__v) {
                self.leech_supreme_kills = __x;
            }
        }
        if let Some(__v) = gson::member(json, "colosseum_bacte_defeated") {
            if let Some(__x) = gson::as_i32(__v) {
                self.bacte_kills = __x;
            }
        }
        if let Some(__v) = gson::member(json, "living_cave_snake_collected") {
            if let Some(__x) = gson::as_i32(__v) {
                self.living_cave_snakes_collected = __x;
            }
        }
        if let Some(__v) = gson::member(json, "castle_sent_to_prison") {
            if let Some(__x) = gson::as_i32(__v) {
                self.times_sent_to_prison = __x;
            }
        }
        if let Some(__v) = gson::member(json, "castle_effigy_broken") {
            if let Some(__x) = gson::as_i32(__v) {
                self.castle_effigies_broken = __x;
            }
        }
    }

    /// `GSON.fromJson(json, Rift.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> Self {
        let mut __this = Self::new();
        __this.read_json_fields(json);
        __this
    }
}

impl Rift {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            visits: 0,
            pass_consumed: 0,
            lifetime_motes_earned: 0,
            motes_orb_pickup: 0,
            leech_supreme_kills: 0,
            bacte_kills: 0,
            living_cave_snakes_collected: 0,
            times_sent_to_prison: 0,
            castle_effigies_broken: 0,
        };
        this
    }

}

/// java: public static class MythologicalData
#[derive(Clone)]
pub struct MythologicalData {
    /// java: @SerializedName("kills") public int kills;
    pub kills: i32,
    /// java: @SerializedName("burrows_dug_next") public BurrowStats burrowsDugNext = new BurrowStats();
    pub burrows_dug_next: compat::Shared<BurrowStats>,
    /// java: @SerializedName("burrows_dug_combat") public BurrowStats burrowsDugCombat = new BurrowStats();
    pub burrows_dug_combat: compat::Shared<BurrowStats>,
    /// java: @SerializedName("burrows_dug_treasure") public BurrowStats burrowsDugTreasure = new BurrowStats();
    pub burrows_dug_treasure: compat::Shared<BurrowStats>,
    /// java: @SerializedName("burrows_chains_complete") public BurrowStats burrowsChainsComplete = new BurrowStats();
    pub burrows_chains_complete: compat::Shared<BurrowStats>,
}

impl MythologicalData {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            kills: 0,
            burrows_dug_next: BurrowStats::new(),
            burrows_dug_combat: BurrowStats::new(),
            burrows_dug_treasure: BurrowStats::new(),
            burrows_chains_complete: BurrowStats::new(),
        };
        this
    }

}

/// java: public static class BurrowStats
#[derive(Clone)]
pub struct BurrowStats {
    /// java: @SerializedName("total") public int total;
    pub total: i32,
    /// java: @SerializedName("none") public int none;
    pub none: i32,
    /// java: @SerializedName("null") public int nulls;
    pub nulls: i32,
    /// java: @SerializedName("COMMON") public int common;
    pub common: i32,
    /// java: @SerializedName("UNCOMMON") public int uncommon;
    pub uncommon: i32,
    /// java: @SerializedName("RARE") public int rare;
    pub rare: i32,
    /// java: @SerializedName("EPIC") public int epic;
    pub epic: i32,
    /// java: @SerializedName("LEGENDARY") public int legendary;
    pub legendary: i32,
}

impl BurrowStats {
    /// java: the implicit no-argument constructor
    pub fn new() -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            total: 0,
            none: 0,
            nulls: 0,
            common: 0,
            uncommon: 0,
            rare: 0,
            epic: 0,
            legendary: 0,
        });
        __this
    }

}

impl Milestone {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "sea_creatures_killed") {
            if let Some(__x) = gson::as_i32(__v) {
                self.sea_creatures_killed = __x;
            }
        }
        if let Some(__v) = gson::member(json, "ores_mined") {
            if let Some(__x) = gson::as_i32(__v) {
                self.ores_mined = __x;
            }
        }
    }

    /// `GSON.fromJson(json, Milestone.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> Self {
        let mut __this = Self::new();
        __this.read_json_fields(json);
        __this
    }
}

impl PlayerStats {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "sea_creature_kills") {
            if let Some(__x) = gson::as_i32(__v) {
                self.sea_creature_kills = __x;
            }
        }
        if let Some(__v) = gson::member(json, "glowing_mushrooms_broken") {
            if let Some(__x) = gson::as_f32(__v) {
                self.glowing_mushrooms_broken = __x;
            }
        }
        if let Some(__v) = gson::member(json, "highest_damage") {
            if let Some(__x) = gson::as_i64(__v) {
                self.highest_damage = __x;
            }
        }
        if let Some(__v) = gson::member(json, "highest_critical_damage") {
            if let Some(__x) = gson::as_i64(__v) {
                self.highest_critical_damage = __x;
            }
        }
        if let Some(__v) = gson::member(json, "kills") {
            if let Some(__x) = gson::map(__v, gson::string_key, |__e0| gson::as_i32(__e0)) {
                self.kills = __x;
            }
        }
        if let Some(__v) = gson::member(json, "deaths") {
            if let Some(__x) = gson::map(__v, gson::string_key, |__e0| gson::as_i32(__e0)) {
                self.deaths = __x;
            }
        }
        if let Some(__v) = gson::member(json, "pets") {
            if let Some(__x) = Some(Pets::from_json(__v)) {
                self.pets = __x;
            }
        }
        if let Some(__v) = gson::member(json, "gifts") {
            if let Some(__x) = Some(Gifts::from_json(__v)) {
                self.gifts = __x;
            }
        }
        if let Some(__v) = gson::member(json, "auctions") {
            if let Some(__x) = Some(Auctions::from_json(__v)) {
                self.auctions = __x;
            }
        }
        if let Some(__v) = gson::member(json, "items_fished") {
            if let Some(__x) = Some(ItemsFished::from_json(__v)) {
                self.items_fished = __x;
            }
        }
        if let Some(__v) = gson::member(json, "rift") {
            if let Some(__x) = Some(Rift::from_json(__v)) {
                self.rift = __x;
            }
        }
        if let Some(__v) = gson::member(json, "mythos") {
            if let Some(__x) = Some(MythologicalData::from_json(__v)) {
                self.mythological_data = __x;
            }
        }
        if let Some(__v) = gson::member(json, "unique_shards") {
            if let Some(__x) = gson::as_i32(__v) {
                self.unique_shards = __x;
            }
        }
        if let Some(__v) = gson::member(json, "shard_combat_hunts") {
            if let Some(__x) = gson::as_i32(__v) {
                self.combat_hunts = __x;
            }
        }
        if let Some(__v) = gson::member(json, "shard_fishing_hunts") {
            if let Some(__x) = gson::as_i32(__v) {
                self.fishing_hunts = __x;
            }
        }
        if let Some(__v) = gson::member(json, "shard_forest_hunts") {
            if let Some(__x) = gson::as_i32(__v) {
                self.forest_hunts = __x;
            }
        }
        if let Some(__v) = gson::member(json, "shard_salt_hunts") {
            if let Some(__x) = gson::as_i32(__v) {
                self.salt_hunts = __x;
            }
        }
        if let Some(__v) = gson::member(json, "shard_trap_hunts") {
            if let Some(__x) = gson::as_i32(__v) {
                self.trap_hunts = __x;
            }
        }
    }

    /// `GSON.fromJson(json, PlayerStats.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> Self {
        let mut __this = Self::new();
        __this.read_json_fields(json);
        __this
    }
}

impl Pets {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "total_exp_gained") {
            if let Some(__x) = gson::as_f64(__v) {
                self.total_exp_gained = __x;
            }
        }
        if let Some(__v) = gson::member(json, "milestone") {
            if let Some(__x) = Some(Milestone::from_json(__v)) {
                self.milestone = __x;
            }
        }
    }

    /// `GSON.fromJson(json, Pets.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> Self {
        let mut __this = Self::new();
        __this.read_json_fields(json);
        __this
    }
}

impl Gifts {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "total_given") {
            if let Some(__x) = gson::as_i32(__v) {
                self.total_given = __x;
            }
        }
        if let Some(__v) = gson::member(json, "total_received") {
            if let Some(__x) = gson::as_i32(__v) {
                self.total_received = __x;
            }
        }
    }

    /// `GSON.fromJson(json, Gifts.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> Self {
        let mut __this = Self::new();
        __this.read_json_fields(json);
        __this
    }
}

impl Auctions {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "highest_bid") {
            if let Some(__x) = gson::as_f64(__v) {
                self.highest_bid = __x;
            }
        }
        if let Some(__v) = gson::member(json, "bids") {
            if let Some(__x) = gson::as_i32(__v) {
                self.bids = __x;
            }
        }
        if let Some(__v) = gson::member(json, "won") {
            if let Some(__x) = gson::as_i32(__v) {
                self.won = __x;
            }
        }
        if let Some(__v) = gson::member(json, "created") {
            if let Some(__x) = gson::as_i32(__v) {
                self.created = __x;
            }
        }
        if let Some(__v) = gson::member(json, "gold_spent") {
            if let Some(__x) = gson::as_f64(__v) {
                self.gold_spent = __x;
            }
        }
        if let Some(__v) = gson::member(json, "gold_earned") {
            if let Some(__x) = gson::as_f64(__v) {
                self.gold_earned = __x;
            }
        }
        if let Some(__v) = gson::member(json, "no_bids") {
            if let Some(__x) = gson::as_i32(__v) {
                self.no_bids = __x;
            }
        }
        if let Some(__v) = gson::member(json, "fees") {
            if let Some(__x) = gson::as_f64(__v) {
                self.fees = __x;
            }
        }
        if let Some(__v) = gson::member(json, "total_bought") {
            if let Some(__x) = Some(RarityStats::from_json(__v)) {
                self.total_bought = __x;
            }
        }
        if let Some(__v) = gson::member(json, "total_sold") {
            if let Some(__x) = Some(RarityStats::from_json(__v)) {
                self.total_sold = __x;
            }
        }
    }

    /// `GSON.fromJson(json, Auctions.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> Self {
        let mut __this = Self::new();
        __this.read_json_fields(json);
        __this
    }
}

impl RarityStats {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "COMMON") {
            if let Some(__x) = gson::as_i32(__v) {
                self.common = __x;
            }
        }
        if let Some(__v) = gson::member(json, "UNCOMMON") {
            if let Some(__x) = gson::as_i32(__v) {
                self.uncommon = __x;
            }
        }
        if let Some(__v) = gson::member(json, "RARE") {
            if let Some(__x) = gson::as_i32(__v) {
                self.rare = __x;
            }
        }
        if let Some(__v) = gson::member(json, "EPIC") {
            if let Some(__x) = gson::as_i32(__v) {
                self.epic = __x;
            }
        }
        if let Some(__v) = gson::member(json, "LEGENDARY") {
            if let Some(__x) = gson::as_i32(__v) {
                self.legendary = __x;
            }
        }
        if let Some(__v) = gson::member(json, "MYTHIC") {
            if let Some(__x) = gson::as_i32(__v) {
                self.mythic = __x;
            }
        }
        if let Some(__v) = gson::member(json, "ULTIMATE") {
            if let Some(__x) = gson::as_i32(__v) {
                self.ultimate = __x;
            }
        }
        if let Some(__v) = gson::member(json, "SPECIAL") {
            if let Some(__x) = gson::as_i32(__v) {
                self.special = __x;
            }
        }
        if let Some(__v) = gson::member(json, "total") {
            if let Some(__x) = gson::as_i32(__v) {
                self.total = __x;
            }
        }
    }

    /// `GSON.fromJson(json, RarityStats.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> compat::Shared<Self> {
        let __this = Self::new();
        compat::write(&__this, |__t| __t.read_json_fields(json));
        __this
    }
}

impl ItemsFished {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "total") {
            if let Some(__x) = gson::as_i32(__v) {
                self.total = __x;
            }
        }
        if let Some(__v) = gson::member(json, "normal") {
            if let Some(__x) = gson::as_i32(__v) {
                self.normal = __x;
            }
        }
        if let Some(__v) = gson::member(json, "treasure") {
            if let Some(__x) = gson::as_i32(__v) {
                self.treasure = __x;
            }
        }
        if let Some(__v) = gson::member(json, "large_treasure") {
            if let Some(__x) = gson::as_i32(__v) {
                self.large_treasure = __x;
            }
        }
        if let Some(__v) = gson::member(json, "trophy_fish") {
            if let Some(__x) = gson::as_i32(__v) {
                self.trophy_fish = __x;
            }
        }
        if let Some(__v) = gson::member(json, "trophy_frog") {
            if let Some(__x) = gson::as_i32(__v) {
                self.trophy_frog = __x;
            }
        }
        if let Some(__v) = gson::member(json, "outstanding") {
            if let Some(__x) = gson::as_i32(__v) {
                self.outstanding = __x;
            }
        }
    }

    /// `GSON.fromJson(json, ItemsFished.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> Self {
        let mut __this = Self::new();
        __this.read_json_fields(json);
        __this
    }
}

impl MythologicalData {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "kills") {
            if let Some(__x) = gson::as_i32(__v) {
                self.kills = __x;
            }
        }
        if let Some(__v) = gson::member(json, "burrows_dug_next") {
            if let Some(__x) = Some(BurrowStats::from_json(__v)) {
                self.burrows_dug_next = __x;
            }
        }
        if let Some(__v) = gson::member(json, "burrows_dug_combat") {
            if let Some(__x) = Some(BurrowStats::from_json(__v)) {
                self.burrows_dug_combat = __x;
            }
        }
        if let Some(__v) = gson::member(json, "burrows_dug_treasure") {
            if let Some(__x) = Some(BurrowStats::from_json(__v)) {
                self.burrows_dug_treasure = __x;
            }
        }
        if let Some(__v) = gson::member(json, "burrows_chains_complete") {
            if let Some(__x) = Some(BurrowStats::from_json(__v)) {
                self.burrows_chains_complete = __x;
            }
        }
    }

    /// `GSON.fromJson(json, MythologicalData.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> Self {
        let mut __this = Self::new();
        __this.read_json_fields(json);
        __this
    }
}

impl BurrowStats {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "total") {
            if let Some(__x) = gson::as_i32(__v) {
                self.total = __x;
            }
        }
        if let Some(__v) = gson::member(json, "none") {
            if let Some(__x) = gson::as_i32(__v) {
                self.none = __x;
            }
        }
        if let Some(__v) = gson::member(json, "null") {
            if let Some(__x) = gson::as_i32(__v) {
                self.nulls = __x;
            }
        }
        if let Some(__v) = gson::member(json, "COMMON") {
            if let Some(__x) = gson::as_i32(__v) {
                self.common = __x;
            }
        }
        if let Some(__v) = gson::member(json, "UNCOMMON") {
            if let Some(__x) = gson::as_i32(__v) {
                self.uncommon = __x;
            }
        }
        if let Some(__v) = gson::member(json, "RARE") {
            if let Some(__x) = gson::as_i32(__v) {
                self.rare = __x;
            }
        }
        if let Some(__v) = gson::member(json, "EPIC") {
            if let Some(__x) = gson::as_i32(__v) {
                self.epic = __x;
            }
        }
        if let Some(__v) = gson::member(json, "LEGENDARY") {
            if let Some(__x) = gson::as_i32(__v) {
                self.legendary = __x;
            }
        }
    }

    /// `GSON.fromJson(json, BurrowStats.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> compat::Shared<Self> {
        let __this = Self::new();
        compat::write(&__this, |__t| __t.read_json_fields(json));
        __this
    }
}

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

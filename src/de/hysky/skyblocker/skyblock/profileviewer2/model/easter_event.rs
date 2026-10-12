//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.model.EasterEvent by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::entry::skyblocker_mod;

/// java: public class EasterEvent
#[derive(Clone)]
pub struct EasterEvent {
    /// java: @SerializedName("chocolate") public long currentChocolate;
    pub current_chocolate: i64,
    /// java: @SerializedName("chocolate_since_prestige") public long chocolateSincePrestige;
    pub chocolate_since_prestige: i64,
    /// java: @SerializedName("total_chocolate") public long allTimeChocolate;
    pub all_time_chocolate: i64,
    /// java: @SerializedName("employees") public Employees employeeLevels = new Employees();
    pub employee_levels: Employees,
    /// java: @SerializedName("last_viewed_chocolate_factory") public long lastViewedChocolateFactory;
    pub last_viewed_chocolate_factory: i64,
    /// java: @SerializedName("rabbits") private JsonObject rawRabbits = new JsonObject();
    pub raw_rabbits: serde_json::Map<String, serde_json::Value>,
    /// java: public Shop shop = new Shop();
    pub shop: Shop,
    /// java: @SerializedName("rabbit_barn_capacity_level") public int barnCapacity;
    pub barn_capacity: i32,
    /// java: @SerializedName("chocolate_level") public int chocolateFactoryLevel;
    pub chocolate_factory_level: i32,
    /// java: @SerializedName("time_tower") public TimeTower timeTower = new TimeTower();
    pub time_tower: TimeTower,
    /// java: @SerializedName("el_dorado_progress") public int elDoradoProgress;
    pub el_dorado_progress: i32,
    /// java: @SerializedName("chocolate_multiplier_upgrades") public int chocolateMultiplierUpgrades;
    pub chocolate_multiplier_upgrades: i32,
    /// java: @SerializedName("rabbit_rarity_upgrades") public int rabbitRarityUpgrade;
    pub rabbit_rarity_upgrade: i32,
    /// java: @SerializedName("golden_click_amount") public int goldenClickAmount;
    pub golden_click_amount: i32,
    /// java: @SerializedName("golden_click_year") public int goldenClickYear;
    pub golden_click_year: i32,
    /// java: @SerializedName("click_upgrades") public int clickUpgrades;
    pub click_upgrades: i32,
    /// java: @SerializedName("supreme_chocolate_bars") public int supremeChocolateBars;
    pub supreme_chocolate_bars: i32,
    /// java: @SerializedName("refined_dark_cacao_truffles") public int refinedDarkCocoaTrufflesConsumer;
    pub refined_dark_cocoa_truffles_consumer: i32,
    /// java: @SerializedName("rabbit_hitmen") public Hitmen hitmen = new Hitmen();
    pub hitmen: Hitmen,
    /// java: private transient @Nullable CollectedEggs collectedEggs;
    pub collected_eggs: Option<compat::Shared<CollectedEggs>>,
    /// java: private transient @Nullable Map<String, Integer> rabbitCollectionCount;
    pub rabbit_collection_count: Option<std::collections::HashMap<String, i32>>,
}

impl EasterEvent {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            current_chocolate: 0,
            chocolate_since_prestige: 0,
            all_time_chocolate: 0,
            employee_levels: Employees::new(),
            last_viewed_chocolate_factory: 0,
            raw_rabbits: serde_json::Map::new(),
            shop: Shop::new(),
            barn_capacity: 0,
            chocolate_factory_level: 0,
            time_tower: TimeTower::new(),
            el_dorado_progress: 0,
            chocolate_multiplier_upgrades: 0,
            rabbit_rarity_upgrade: 0,
            golden_click_amount: 0,
            golden_click_year: 0,
            click_upgrades: 0,
            supreme_chocolate_bars: 0,
            refined_dark_cocoa_truffles_consumer: 0,
            hitmen: Hitmen::new(),
            collected_eggs: None,
            rabbit_collection_count: None,
        };
        this
    }

    /// java: public Map<String, Integer> getRabbitCount()
    pub fn get_rabbit_count(&mut self) -> std::collections::HashMap<String, i32> {
        if self.rabbit_collection_count.is_none() {
            self.rabbit_collection_count = Some(std::collections::HashMap::with_capacity(((self.raw_rabbits.len() as i32)) as usize));
            for entry in self.raw_rabbits.iter().map(|(k, v)| (k.clone(), v.clone())) {
                if entry.0 == "collected_eggs" {
                    continue;
                }
                self.rabbit_collection_count.as_mut().unwrap().insert(entry.0.clone(), compat::json::get_as_int(&entry.1));
            }
        }
        return self.rabbit_collection_count.clone().unwrap();
    }

    /// java: public CollectedEggs getLastCollectedEggs()
    pub fn get_last_collected_eggs(&mut self) -> compat::Shared<CollectedEggs> {
        if self.collected_eggs.is_none() {
            // TODO(fabric2seer): `.fromJson(..)` on a Gson
            //   java: this.collectedEggs = SkyblockerMod.GSON.fromJson(this.rawRabbits.getAsJsonObject("collected_eggs"), CollectedEggs.class);
        }
        return self.collected_eggs.clone().unwrap();
    }

}

/// java: public static class Hitmen
#[derive(Clone)]
pub struct Hitmen {
    /// java: @SerializedName("rabbit_hitmen_slots") public int unlockedSlots;
    pub unlocked_slots: i32,
    /// java: @SerializedName("missed_uncollected_eggs") public int uncollectedEggCount;
    pub uncollected_egg_count: i32,
    /// java: @SerializedName("egg_slot_cooldown_mark") public long eggSlotCooldownTimestamp;
    pub egg_slot_cooldown_timestamp: i64,
    /// java: @SerializedName("egg_slot_cooldown_sum") public long eggSlotCooldownSum;
    pub egg_slot_cooldown_sum: i64,
}

impl Hitmen {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            unlocked_slots: 0,
            uncollected_egg_count: 0,
            egg_slot_cooldown_timestamp: 0,
            egg_slot_cooldown_sum: 0,
        };
        this
    }

}

/// java: public static class TimeTower
#[derive(Clone)]
pub struct TimeTower {
    /// java: public int charges;
    pub charges: i32,
    /// java: public int level;
    pub level: i32,
    /// java: @SerializedName("activation_time") public long activationTime;
    pub activation_time: i64,
    /// java: @SerializedName("last_charge_time") public long lastChargeTime;
    pub last_charge_time: i64,
}

impl TimeTower {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            charges: 0,
            level: 0,
            activation_time: 0,
            last_charge_time: 0,
        };
        this
    }

}

/// java: public static class Shop
#[derive(Clone)]
pub struct Shop {
    /// java: public int year;
    pub year: i32,
    /// java: public List<String> rabbits = List.of();
    pub rabbits: Vec<String>,
    /// java: @SerializedName("chocolate_spent") public long chocolateSpent;
    pub chocolate_spent: i64,
    /// java: @SerializedName("cocoa_fortune_upgrades") public int cocoaFortuneUpgrade;
    pub cocoa_fortune_upgrade: i32,
}

impl Shop {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            year: 0,
            rabbits: vec![],
            chocolate_spent: 0,
            cocoa_fortune_upgrade: 0,
        };
        this
    }

}

/// java: public static class CollectedEggs
#[derive(Clone)]
pub struct CollectedEggs {
    /// java: public long breakfast;
    pub breakfast: i64,
    /// java: public long dinner;
    pub dinner: i64,
    /// java: public long lunch;
    pub lunch: i64,
    /// java: public long brunch;
    pub brunch: i64,
    /// java: public long dejeuner;
    pub dejeuner: i64,
    /// java: public long supper;
    pub supper: i64,
}

impl CollectedEggs {
    /// java: the implicit no-argument constructor
    pub fn new() -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            breakfast: 0,
            dinner: 0,
            lunch: 0,
            brunch: 0,
            dejeuner: 0,
            supper: 0,
        });
        __this
    }

}

/// java: public static class Employees
#[derive(Clone)]
pub struct Employees {
    /// java: @SerializedName("rabbit_bro") public long rabbitBro;
    pub rabbit_bro: i64,
    /// java: @SerializedName("rabbit_cousin") public long rabbitCousin;
    pub rabbit_cousin: i64,
    /// java: @SerializedName("rabbit_sis") public long rabbitSis;
    pub rabbit_sis: i64,
    /// java: @SerializedName("rabbit_father") public long rabbitFather;
    pub rabbit_father: i64,
    /// java: @SerializedName("rabbit_grandma") public long rabbitGrandma;
    pub rabbit_grandma: i64,
    /// java: @SerializedName("rabbit_uncle") public long rabbitUncle;
    pub rabbit_uncle: i64,
    /// java: @SerializedName("rabbit_dog") public long rabbitDog;
    pub rabbit_dog: i64,
}

impl Employees {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            rabbit_bro: 0,
            rabbit_cousin: 0,
            rabbit_sis: 0,
            rabbit_father: 0,
            rabbit_grandma: 0,
            rabbit_uncle: 0,
            rabbit_dog: 0,
        };
        this
    }

}

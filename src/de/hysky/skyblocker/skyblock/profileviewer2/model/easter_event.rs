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
            // java: this.collectedEggs = SkyblockerMod.GSON.fromJson(this.rawRabbits.getAsJsonObject("collected_eggs"), CollectedEggs.class);
            // (`fromJson(null)` is `null`; a member that is no object is Gson's ClassCastException)
            self.collected_eggs = match self.raw_rabbits.get("collected_eggs") {
                None | Some(serde_json::Value::Null) => None,
                Some(value @ serde_json::Value::Object(_)) => Some(CollectedEggs::from_json(value)),
                Some(_) => panic!("ClassCastException: collected_eggs is not a JsonObject"),
            };
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

impl EasterEvent {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "chocolate") {
            if let Some(__x) = gson::as_i64(__v) {
                self.current_chocolate = __x;
            }
        }
        if let Some(__v) = gson::member(json, "chocolate_since_prestige") {
            if let Some(__x) = gson::as_i64(__v) {
                self.chocolate_since_prestige = __x;
            }
        }
        if let Some(__v) = gson::member(json, "total_chocolate") {
            if let Some(__x) = gson::as_i64(__v) {
                self.all_time_chocolate = __x;
            }
        }
        if let Some(__v) = gson::member(json, "employees") {
            if let Some(__x) = Some(Employees::from_json(__v)) {
                self.employee_levels = __x;
            }
        }
        if let Some(__v) = gson::member(json, "last_viewed_chocolate_factory") {
            if let Some(__x) = gson::as_i64(__v) {
                self.last_viewed_chocolate_factory = __x;
            }
        }
        if let Some(__v) = gson::member(json, "rabbits") {
            if let Some(__x) = __v.as_object().cloned() {
                self.raw_rabbits = __x;
            }
        }
        if let Some(__v) = gson::member(json, "shop") {
            if let Some(__x) = Some(Shop::from_json(__v)) {
                self.shop = __x;
            }
        }
        if let Some(__v) = gson::member(json, "rabbit_barn_capacity_level") {
            if let Some(__x) = gson::as_i32(__v) {
                self.barn_capacity = __x;
            }
        }
        if let Some(__v) = gson::member(json, "chocolate_level") {
            if let Some(__x) = gson::as_i32(__v) {
                self.chocolate_factory_level = __x;
            }
        }
        if let Some(__v) = gson::member(json, "time_tower") {
            if let Some(__x) = Some(TimeTower::from_json(__v)) {
                self.time_tower = __x;
            }
        }
        if let Some(__v) = gson::member(json, "el_dorado_progress") {
            if let Some(__x) = gson::as_i32(__v) {
                self.el_dorado_progress = __x;
            }
        }
        if let Some(__v) = gson::member(json, "chocolate_multiplier_upgrades") {
            if let Some(__x) = gson::as_i32(__v) {
                self.chocolate_multiplier_upgrades = __x;
            }
        }
        if let Some(__v) = gson::member(json, "rabbit_rarity_upgrades") {
            if let Some(__x) = gson::as_i32(__v) {
                self.rabbit_rarity_upgrade = __x;
            }
        }
        if let Some(__v) = gson::member(json, "golden_click_amount") {
            if let Some(__x) = gson::as_i32(__v) {
                self.golden_click_amount = __x;
            }
        }
        if let Some(__v) = gson::member(json, "golden_click_year") {
            if let Some(__x) = gson::as_i32(__v) {
                self.golden_click_year = __x;
            }
        }
        if let Some(__v) = gson::member(json, "click_upgrades") {
            if let Some(__x) = gson::as_i32(__v) {
                self.click_upgrades = __x;
            }
        }
        if let Some(__v) = gson::member(json, "supreme_chocolate_bars") {
            if let Some(__x) = gson::as_i32(__v) {
                self.supreme_chocolate_bars = __x;
            }
        }
        if let Some(__v) = gson::member(json, "refined_dark_cacao_truffles") {
            if let Some(__x) = gson::as_i32(__v) {
                self.refined_dark_cocoa_truffles_consumer = __x;
            }
        }
        if let Some(__v) = gson::member(json, "rabbit_hitmen") {
            if let Some(__x) = Some(Hitmen::from_json(__v)) {
                self.hitmen = __x;
            }
        }
    }

    /// `GSON.fromJson(json, EasterEvent.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> Self {
        let mut __this = Self::new();
        __this.read_json_fields(json);
        __this
    }
}

impl Hitmen {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "rabbit_hitmen_slots") {
            if let Some(__x) = gson::as_i32(__v) {
                self.unlocked_slots = __x;
            }
        }
        if let Some(__v) = gson::member(json, "missed_uncollected_eggs") {
            if let Some(__x) = gson::as_i32(__v) {
                self.uncollected_egg_count = __x;
            }
        }
        if let Some(__v) = gson::member(json, "egg_slot_cooldown_mark") {
            if let Some(__x) = gson::as_i64(__v) {
                self.egg_slot_cooldown_timestamp = __x;
            }
        }
        if let Some(__v) = gson::member(json, "egg_slot_cooldown_sum") {
            if let Some(__x) = gson::as_i64(__v) {
                self.egg_slot_cooldown_sum = __x;
            }
        }
    }

    /// `GSON.fromJson(json, Hitmen.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> Self {
        let mut __this = Self::new();
        __this.read_json_fields(json);
        __this
    }
}

impl TimeTower {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "charges") {
            if let Some(__x) = gson::as_i32(__v) {
                self.charges = __x;
            }
        }
        if let Some(__v) = gson::member(json, "level") {
            if let Some(__x) = gson::as_i32(__v) {
                self.level = __x;
            }
        }
        if let Some(__v) = gson::member(json, "activation_time") {
            if let Some(__x) = gson::as_i64(__v) {
                self.activation_time = __x;
            }
        }
        if let Some(__v) = gson::member(json, "last_charge_time") {
            if let Some(__x) = gson::as_i64(__v) {
                self.last_charge_time = __x;
            }
        }
    }

    /// `GSON.fromJson(json, TimeTower.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> Self {
        let mut __this = Self::new();
        __this.read_json_fields(json);
        __this
    }
}

impl Shop {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "year") {
            if let Some(__x) = gson::as_i32(__v) {
                self.year = __x;
            }
        }
        if let Some(__v) = gson::member(json, "rabbits") {
            if let Some(__x) = gson::list(__v, |__e0| gson::as_string(__e0)) {
                self.rabbits = __x;
            }
        }
        if let Some(__v) = gson::member(json, "chocolate_spent") {
            if let Some(__x) = gson::as_i64(__v) {
                self.chocolate_spent = __x;
            }
        }
        if let Some(__v) = gson::member(json, "cocoa_fortune_upgrades") {
            if let Some(__x) = gson::as_i32(__v) {
                self.cocoa_fortune_upgrade = __x;
            }
        }
    }

    /// `GSON.fromJson(json, Shop.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> Self {
        let mut __this = Self::new();
        __this.read_json_fields(json);
        __this
    }
}

impl CollectedEggs {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "breakfast") {
            if let Some(__x) = gson::as_i64(__v) {
                self.breakfast = __x;
            }
        }
        if let Some(__v) = gson::member(json, "dinner") {
            if let Some(__x) = gson::as_i64(__v) {
                self.dinner = __x;
            }
        }
        if let Some(__v) = gson::member(json, "lunch") {
            if let Some(__x) = gson::as_i64(__v) {
                self.lunch = __x;
            }
        }
        if let Some(__v) = gson::member(json, "brunch") {
            if let Some(__x) = gson::as_i64(__v) {
                self.brunch = __x;
            }
        }
        if let Some(__v) = gson::member(json, "dejeuner") {
            if let Some(__x) = gson::as_i64(__v) {
                self.dejeuner = __x;
            }
        }
        if let Some(__v) = gson::member(json, "supper") {
            if let Some(__x) = gson::as_i64(__v) {
                self.supper = __x;
            }
        }
    }

    /// `GSON.fromJson(json, CollectedEggs.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> compat::Shared<Self> {
        let __this = Self::new();
        compat::write(&__this, |__t| __t.read_json_fields(json));
        __this
    }
}

impl Employees {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "rabbit_bro") {
            if let Some(__x) = gson::as_i64(__v) {
                self.rabbit_bro = __x;
            }
        }
        if let Some(__v) = gson::member(json, "rabbit_cousin") {
            if let Some(__x) = gson::as_i64(__v) {
                self.rabbit_cousin = __x;
            }
        }
        if let Some(__v) = gson::member(json, "rabbit_sis") {
            if let Some(__x) = gson::as_i64(__v) {
                self.rabbit_sis = __x;
            }
        }
        if let Some(__v) = gson::member(json, "rabbit_father") {
            if let Some(__x) = gson::as_i64(__v) {
                self.rabbit_father = __x;
            }
        }
        if let Some(__v) = gson::member(json, "rabbit_grandma") {
            if let Some(__x) = gson::as_i64(__v) {
                self.rabbit_grandma = __x;
            }
        }
        if let Some(__v) = gson::member(json, "rabbit_uncle") {
            if let Some(__x) = gson::as_i64(__v) {
                self.rabbit_uncle = __x;
            }
        }
        if let Some(__v) = gson::member(json, "rabbit_dog") {
            if let Some(__x) = gson::as_i64(__v) {
                self.rabbit_dog = __x;
            }
        }
    }

    /// `GSON.fromJson(json, Employees.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> Self {
        let mut __this = Self::new();
        __this.read_json_fields(json);
        __this
    }
}

//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.utils.AccessoryPowerCalculator by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::accessories::accessories_helper::Accessory;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::info::tooltip_info_type;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::model::profile_member::ProfileMember;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::profile_viewer;
use crate::de::hysky::skyblocker::utils::formatters;
use crate::de::hysky::skyblocker::utils::sky_block_icons;
use crate::entry::skyblocker_mod;
use mcrust::com::mojang::serialization::codec;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::com::mojang::serialization::codecs::record_codec_builder;
use mcrust::java::text::decimal_format_symbols::Locale;
use mcrust::java::util::concurrent::completable_future::CompletableFuture;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::info::data_tooltip_info_type::DataTooltipInfoType as _;
use mcrust::com::mojang::serialization::decoder::Decoder as _;

/// java: private static final boolean DEBUG_AP_CALCULATION = false;
pub const DEBUG_AP_CALCULATION: bool = false;
/// java: private static final List<String> STAT_ORDER = List.of( "health", "defence", "true_defence", "strength", "critical_chance", "critical_damage", "attack_speed", "ferocity", "intelligence", "walk_speed",
pub static STAT_ORDER: LazyLock<Vec<String>> = LazyLock::new(|| vec!["health".to_string(), "defence".to_string(), "true_defence".to_string(), "strength".to_string(), "critical_chance".to_string(), "critical_damage".to_string(), "attack_speed".to_string(), "ferocity".to_string(), "intelligence".to_string(), "walk_speed".to_string(), "combat_wisdom".to_string()]);
/// java: private static Map<String, Power> powers = Map.of();
pub static POWERS: LazyLock<Mutex<std::collections::HashMap<String, Power>>> = LazyLock::new(|| Mutex::new(std::collections::HashMap::from([])));
/// java: private static final Codec<Power> CODEC = RecordCodecBuilder.create(instance -> instance.group( Codec.unboundedMap(Codec.STRING, Codec.DOUBLE).fieldOf("stats").forGetter(Power::stats), Codec.unbounded
pub static POWER_CODEC: compat::MainThread<CodecRef<Power>> = compat::MainThread::new(|| record_codec_builder::create(record_codec_builder::apply2(record_codec_builder::of(|o: &Power| o.stats(), codec::field_of(codec::unbounded_map(mcrust::com::mojang::serialization::codec::string_codec(), mcrust::com::mojang::serialization::codec::double_codec()), "stats")), record_codec_builder::of(|o: &Power| o.bonus(), codec::optional_field_of(codec::unbounded_map(mcrust::com::mojang::serialization::codec::string_codec(), mcrust::com::mojang::serialization::codec::double_codec()), "bonus", std::collections::HashMap::from([]))), move |__a0, __a1| Power::new(__a0.clone(), __a1.clone()))));
/// java: private static final Codec<Map<String, Power>> MAP_CODEC = Codec.unboundedMap(Codec.STRING, CODEC);
pub static POWER_MAP_CODEC: compat::MainThread<CodecRef<std::collections::HashMap<String, Power>>> = compat::MainThread::new(|| codec::unbounded_map(mcrust::com::mojang::serialization::codec::string_codec(), POWER_CODEC.get_mut().clone()));
/// java: private static final Power EMPTY = new Power(Map.of(), Map.of());
pub static POWER_EMPTY: LazyLock<Power> = LazyLock::new(|| Power::new(std::collections::HashMap::from([]), std::collections::HashMap::from([])));

/// java: public class AccessoryPowerCalculator
#[derive(Clone)]
pub struct AccessoryPowerCalculator {}

impl AccessoryPowerCalculator {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        if *profile_viewer::ENABLED {
            // TODO(fabric2seer): an assignment used as a value
            //   java: CompletableFuture.supplyAsync(AccessoryPowerCalculator::loadPowers, SkyblockerMod.VIRTUAL_THREAD_EXECUTOR)
            //   java: 			.thenAcceptAsync(data -> powers = data, Minecraft.getInstance());
        }
    }

    /// java: private static Map<String, Power> loadPowers()
    pub fn load_powers() -> std::collections::HashMap<String, Power> {
        // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`; the body's `return` is the closure's value
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            // TODO(fabric2seer): `Http.sendGetRequest` is not a static method of Http
            //   java: String response = Http.sendGetRequest("https://api.azureaaron.net/skyblock/magicalpowers");
            let mut response: String = todo!("fabric2seer: the initialiser of `response` (TODO above)");
            return (POWER_MAP_CODEC.get_mut().parse(&mcrust::com::mojang::serialization::json_ops::JsonOps::instance(), &compat::json_to_port(&serde_json::from_str::<serde_json::Value>(&response).expect("JsonSyntaxException")))).get_or_throw().unwrap();
        })) {
            Ok(__v) => return __v,
            Err(e) => {
                logger().info(&format!("[Skyblocker Profile Viewer] Failed to load accessory powers."));
            }
        }
        return std::collections::HashMap::from([]);
    }

    /// java: public static int calculateAccessoryPower(ProfileMember member, List<ItemStack> accessories)
    pub fn calculate_accessory_power(member: &compat::Shared<ProfileMember>, accessories: &Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> i32 {
        let mut consumed_rift_prism: bool = member.lock().borrow().rift.access.consumed_prism;
        let mut highest_rarities: std::collections::HashMap<String, mcrust::net::minecraft::world::item::item_stack::ItemStack> = std::collections::HashMap::new();
        for accessory in accessories.iter() {
            let mut id: String = accessory.get_skyblock_id();
            if id.is_empty() {
                continue;
            }
            let mut mapping: Option<mcrust::net::minecraft::world::item::item_stack::ItemStack> = highest_rarities.get(&id).cloned();
            if mapping.is_none() || compat::ord_int(accessory.get_skyblock_rarity().cmp(&mapping.as_ref().unwrap().get_skyblock_rarity())) > 0 {
                highest_rarities.insert(id.clone(), accessory.clone());
            }
        }
        if consumed_rift_prism {
            highest_rarities.remove("RIFT_PRISM");
        }
        let mut accessory_data: std::collections::HashMap<String, Accessory> = tooltip_info_type::ACCESSORIES.get_mut().lock().borrow().get_data().unwrap();
        let mut accessories_to_drop: Vec<String> = Vec::new();
        for id in highest_rarities.keys().cloned() {
            let mut data: Option<Accessory> = accessory_data.get(&id).cloned();
            if data.is_some() && data.as_ref().unwrap().family().is_some() && !(accessories_to_drop.iter().any(|e| *e == id.clone())) {
                let mut family_members: Vec<Accessory> = accessory_data.values().cloned().filter(|__a0| __a0.has_family()).filter(|__a0| data.as_ref().unwrap().has_same_family(__a0)).collect::<Vec<_>>();
                let mut highest_tier: i32 = (family_members.iter().cloned().max_by(|a, b| { let key = |__a0| __a0.tier(); key(a).cmp(&key(b)) }).unwrap()).tier();
                if data.as_ref().unwrap().tier() == highest_tier {
                    // TODO(fabric2seer): not a lambda
                    //   java: List<String> siblings = familyMembers.stream()
                    //   java: 							.filter(Predicate.not(data::equals))
                    //   java: 							.map(Accessory::id)
                    //   java: 							.toList();
                    let mut siblings: Vec<String> = todo!("fabric2seer: the initialiser of `siblings` (TODO above)");
                    accessories_to_drop.extend(siblings.iter().cloned());
                }
            }
        }
        for __a0 in accessories_to_drop.iter() {
            highest_rarities.remove(__a0);
        }
        let mut accessory_power: i32 = highest_rarities.values().cloned().map(|__a0| __a0.get_skyblock_rarity()).map(|__a0| __a0.get_mp()).sum::<i32>();
        if consumed_rift_prism {
            accessory_power = accessory_power.wrapping_add(11);
        }
        if accessories.iter().cloned().any(|stack| (stack.get_skyblock_id() == "ABICASE")) {
            let mut acquired_contacts: f64 = (((member.lock().borrow().nether_island_player_data.abiphone.active_contacts.len() as i32)) as f64) as f64;
            accessory_power = (((accessory_power as f64) + (acquired_contacts / 2.0).floor())) as i32;
        }
        let mut hegemony: Option<mcrust::net::minecraft::world::item::item_stack::ItemStack> = highest_rarities.get("HEGEMONY_ARTIFACT").cloned();
        if let Some(hegemony) = &hegemony {
            accessory_power = accessory_power.wrapping_add(hegemony.get_skyblock_rarity().get_mp());
        }
        if DEBUG_AP_CALCULATION {
            // TODO(fabric2seer): static call IO.println(..)
            //   java: IO.println(highestRarities.entrySet().stream().collect(Collectors.groupingBy(entry -> entry.getValue().getSkyblockRarity(), Collectors.mapping(Map.Entry::getKey, Collectors.toList()))));
            // TODO(fabric2seer): static call IO.println(..)
            //   java: IO.println(highestRarities.values().stream().map(ItemStack::getSkyblockRarity).collect(Collectors.groupingBy(Function.identity(), Collectors.counting())));
        }
        return accessory_power;
    }

    /// java: private static double calculateStatMultiplier(double accessoryPower)
    pub fn calculate_stat_multiplier(mut accessory_power: f64) -> f64 {
        return (29.97 * (((0.0019 * accessory_power) + 1.0).ln()).powf(1.2));
    }

    /// java: private static double coerce0(@Nullable Double number)
    pub fn coerce0(mut number: f64) -> f64 {
        return (if true /* java: number != null: never null here */ { number } else { ((0) as f64) as f64 });
    }

    /// java: public static List<Component> getPowerStats(int accessoryPower, String selectedPower)
    pub fn get_power_stats(mut accessory_power: i32, selected_power: &str) -> Vec<MutableComponent> {
        let mut lines: Vec<MutableComponent> = Vec::new();
        let mut power: Power = compat::lock(&POWERS).get(selected_power).cloned().unwrap_or_else(|| POWER_EMPTY.clone());
        let mut stat_multiplier: f64 = AccessoryPowerCalculator::calculate_stat_multiplier((accessory_power) as f64);
        for stat_id in STAT_ORDER.iter() {
            let mut amount: Option<f64> = None;
            if power.stats().contains_key(stat_id) {
                amount = Some((AccessoryPowerCalculator::coerce0(amount.clone().unwrap()) + (power.stats().get(stat_id).cloned().unwrap() * stat_multiplier)));
            }
            if power.bonus().contains_key(stat_id) {
                amount = Some((AccessoryPowerCalculator::coerce0(amount.clone().unwrap()) + power.bonus().get(stat_id).cloned().unwrap()));
            }
            if let Some(amount) = &amount {
                lines.push(AccessoryPowerCalculator::format_stat_by_id(stat_id, *amount));
            }
        }
        if lines.is_empty() {
            lines.push(component::literal("Unknown stats...").with_style_format(ChatFormatting::Gray));
        }
        return lines.iter().cloned().collect::<Vec<_>>();
    }

    /// java: private static Component formatStatById(String id, double amount)
    pub fn format_stat_by_id(id: &str, mut amount: f64) -> MutableComponent {
        return match id.as_str() {
            "health" => AccessoryPowerCalculator::format_health(amount),
            "defence" => AccessoryPowerCalculator::format_defence(amount),
            "true_defence" => AccessoryPowerCalculator::format_true_defence(amount),
            "strength" => AccessoryPowerCalculator::format_strength(amount),
            "critical_chance" => AccessoryPowerCalculator::format_crit_chance(amount),
            "critical_damage" => AccessoryPowerCalculator::format_crit_damage(amount),
            "attack_speed" => AccessoryPowerCalculator::format_attack_speed(amount),
            "ferocity" => AccessoryPowerCalculator::format_ferocity(amount),
            "intelligence" => AccessoryPowerCalculator::format_intelligence(amount),
            "ability_damage" => AccessoryPowerCalculator::format_ability_damage(amount),
            "walk_speed" => AccessoryPowerCalculator::format_walk_speed(amount),
            "combat_wisdom" => AccessoryPowerCalculator::format_combat_wisdom(amount),
            _ => (component::literal(&AccessoryPowerCalculator::format_stat('?', amount))).with_style_format(ChatFormatting::Gray),
        };
    }

    /// java: public static Component formatHealth(double amount)
    pub fn format_health(mut amount: f64) -> MutableComponent {
        return (component::literal(&AccessoryPowerCalculator::format_stat(sky_block_icons::HEALTH, amount))).with_style_format(ChatFormatting::Red);
    }

    /// java: public static Component formatDefence(double amount)
    pub fn format_defence(mut amount: f64) -> MutableComponent {
        return (component::literal(&AccessoryPowerCalculator::format_stat(sky_block_icons::DEFENSE, amount))).with_style_format(ChatFormatting::Green);
    }

    /// java: public static Component formatTrueDefence(double amount)
    pub fn format_true_defence(mut amount: f64) -> MutableComponent {
        return (component::literal(&AccessoryPowerCalculator::format_stat(sky_block_icons::TRUE_DEFENSE, amount))).with_style_format(ChatFormatting::White);
    }

    /// java: public static Component formatStrength(double amount)
    pub fn format_strength(mut amount: f64) -> MutableComponent {
        return (component::literal(&AccessoryPowerCalculator::format_stat(sky_block_icons::STRENGTH, amount))).with_style_format(ChatFormatting::Red);
    }

    /// java: public static Component formatCritChance(double amount)
    pub fn format_crit_chance(mut amount: f64) -> MutableComponent {
        return (component::literal(&AccessoryPowerCalculator::format_stat(sky_block_icons::CRIT_CHANCE, amount))).with_style_format(ChatFormatting::Blue);
    }

    /// java: public static Component formatCritDamage(double amount)
    pub fn format_crit_damage(mut amount: f64) -> MutableComponent {
        return (component::literal(&AccessoryPowerCalculator::format_stat(sky_block_icons::CRIT_DAMAGE, amount))).with_style_format(ChatFormatting::Blue);
    }

    /// java: public static Component formatAttackSpeed(double amount)
    pub fn format_attack_speed(mut amount: f64) -> MutableComponent {
        return (component::literal(&AccessoryPowerCalculator::format_stat(sky_block_icons::ATTACK_SPEED, amount))).with_style_format(ChatFormatting::Yellow);
    }

    /// java: public static Component formatFerocity(double amount)
    pub fn format_ferocity(mut amount: f64) -> MutableComponent {
        return (component::literal(&AccessoryPowerCalculator::format_stat(sky_block_icons::FEROCITY, amount))).with_style_format(ChatFormatting::Red);
    }

    /// java: public static Component formatIntelligence(double amount)
    pub fn format_intelligence(mut amount: f64) -> MutableComponent {
        return (component::literal(&AccessoryPowerCalculator::format_stat(sky_block_icons::INTELLIGENCE, amount))).with_style_format(ChatFormatting::Aqua);
    }

    /// java: public static Component formatAbilityDamage(double amount)
    pub fn format_ability_damage(mut amount: f64) -> MutableComponent {
        return (component::literal(&AccessoryPowerCalculator::format_stat(sky_block_icons::ABILITY_DAMAGE, amount))).with_style_format(ChatFormatting::Red);
    }

    /// java: public static Component formatWalkSpeed(double amount)
    pub fn format_walk_speed(mut amount: f64) -> MutableComponent {
        return (component::literal(&AccessoryPowerCalculator::format_stat(sky_block_icons::SPEED, amount))).with_style_format(ChatFormatting::White);
    }

    /// java: public static Component formatCombatWisdom(double amount)
    pub fn format_combat_wisdom(mut amount: f64) -> MutableComponent {
        return (component::literal(&AccessoryPowerCalculator::format_stat(sky_block_icons::COMBAT_WISDOM, amount))).with_style_format(ChatFormatting::DarkAqua);
    }

    /// java: private static String formatStat(char symbol, double amount)
    pub fn format_stat(mut symbol: char, mut amount: f64) -> String {
        let mut sign: String = (if amount >= (0 as f64) { "+" } else { "" }).to_string();
        return format!("{}{}{}", sign, formatters::DOUBLE_NUMBERS.get_mut().format(amount), symbol);
    }

}

/// java: private record Power(Map<String, Double> stats, Map<String, Double> bonus)
#[derive(Clone, Debug, PartialEq)]
pub struct Power {
    /// java: Map<String, Double> stats
    pub stats: std::collections::HashMap<String, f64>,
    /// java: Map<String, Double> bonus
    pub bonus: std::collections::HashMap<String, f64>,
}

impl Power {
    /// java: the canonical record constructor
    pub fn new(mut stats: std::collections::HashMap<String, f64>, mut bonus: std::collections::HashMap<String, f64>) -> Self {
        Self { stats, bonus }
    }

    /// java: the record accessor `stats()`
    pub fn stats(&self) -> std::collections::HashMap<String, f64> {
        self.stats.clone()
    }

    /// java: the record accessor `bonus()`
    pub fn bonus(&self) -> std::collections::HashMap<String, f64> {
        self.bonus.clone()
    }

}

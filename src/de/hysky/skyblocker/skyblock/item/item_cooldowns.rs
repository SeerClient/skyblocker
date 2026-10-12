//! Ported from de.hysky.skyblocker.skyblock.item.ItemCooldowns by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::utils::item_utils::ItemUtils;
use mcrust::net::minecraft::world::entity::player::player::Player;
use mcrust::net::minecraft::world::interaction_hand::InteractionHand;
use mcrust::net::minecraft::world::interaction_result::InteractionResult;
use mcrust::net::minecraft::world::level::level::Level;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;
use mcrust::net::minecraft::world::entity::living_entity::LivingEntity as _;

/// java: private static final String JUNGLE_AXE_ID = "JUNGLE_AXE";
pub const JUNGLE_AXE_ID: &str = "JUNGLE_AXE";
/// java: private static final String TREECAPITATOR_ID = "TREECAPITATOR_AXE";
pub const TREECAPITATOR_ID: &str = "TREECAPITATOR_AXE";
/// java: private static final String FIG_AXE_ID = "FIG_AXE";
pub const FIG_AXE_ID: &str = "FIG_AXE";
/// java: private static final String FIGSTONE_ID = "FIGSTONE_AXE";
pub const FIGSTONE_ID: &str = "FIGSTONE_AXE";
/// java: private static final String GRAPPLING_HOOK_ID = "GRAPPLING_HOOK";
pub const GRAPPLING_HOOK_ID: &str = "GRAPPLING_HOOK";
/// java: private static final String ROGUE_SWORD_ID = "ROGUE_SWORD";
pub const ROGUE_SWORD_ID: &str = "ROGUE_SWORD";
/// java: private static final String LEAPING_SWORD_ID = "LEAPING_SWORD";
pub const LEAPING_SWORD_ID: &str = "LEAPING_SWORD";
/// java: private static final String SILK_EDGE_SWORD_ID = "SILK_EDGE_SWORD";
pub const SILK_EDGE_SWORD_ID: &str = "SILK_EDGE_SWORD";
/// java: private static final String GREAT_SPOOK_STAFF_ID = "GREAT_SPOOK_STAFF";
pub const GREAT_SPOOK_STAFF_ID: &str = "GREAT_SPOOK_STAFF";
/// java: private static final String SPIRIT_LEAP_ID = "SPIRIT_LEAP";
pub const SPIRIT_LEAP_ID: &str = "SPIRIT_LEAP";
/// java: private static final String GIANTS_SWORD_ID = "GIANTS_SWORD";
pub const GIANTS_SWORD_ID: &str = "GIANTS_SWORD";
/// java: private static final String SHADOW_FURY_ID = "SHADOW_FURY";
pub const SHADOW_FURY_ID: &str = "SHADOW_FURY";
/// java: private static final String LIVID_DAGGER_ID = "LIVID_DAGGER";
pub const LIVID_DAGGER_ID: &str = "LIVID_DAGGER";
/// java: private static final String INK_WAND_ID = "INK_WAND";
pub const INK_WAND_ID: &str = "INK_WAND";
/// java: private static final List<String> BAT_ARMOR_IDS = List.of("BAT_PERSON_HELMET", "BAT_PERSON_CHESTPLATE", "BAT_PERSON_LEGGINGS", "BAT_PERSON_BOOTS");
pub static BAT_ARMOR_IDS: LazyLock<Vec<String>> = LazyLock::new(|| vec!["BAT_PERSON_HELMET".to_string(), "BAT_PERSON_CHESTPLATE".to_string(), "BAT_PERSON_LEGGINGS".to_string(), "BAT_PERSON_BOOTS".to_string()]);
/// java: private static final Map<String, CooldownEntry> ITEM_COOLDOWNS = new HashMap<>();
pub static ITEM_COOLDOWNS: LazyLock<Mutex<std::collections::HashMap<String, CooldownEntry>>> = LazyLock::new(|| Mutex::new(std::collections::HashMap::new()));

/// java: public class ItemCooldowns
#[derive(Clone)]
pub struct ItemCooldowns {}

impl ItemCooldowns {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        // TODO(fabric2seer): UseItemCallback.EVENT.register: no SeerClient equivalent yet
        //   java: UseItemCallback.EVENT.register(ItemCooldowns::onItemInteract);
    }

    /// java: private static InteractionResult onItemInteract(Player player, Level world, InteractionHand hand)
    pub fn on_item_interact(player: &dyn Player, world: &Level, hand: &InteractionHand) -> InteractionResult {
        if !(SkyblockerConfigManager::get().ui_and_visuals.item_cooldown.enable_item_cooldowns) {
            return InteractionResult::Pass;
        }
        let mut used_item_id: String = player.get_main_hand_item().get_skyblock_id();
        match used_item_id.as_str() {
            FIG_AXE_ID | FIGSTONE_ID | JUNGLE_AXE_ID | TREECAPITATOR_ID => {
                ItemCooldowns::handle_item_cooldown_2(&used_item_id, 1000);
            }
            SILK_EDGE_SWORD_ID | LEAPING_SWORD_ID => {
                ItemCooldowns::handle_item_cooldown_2(&used_item_id, 1000);
            }
            GRAPPLING_HOOK_ID => {
                // TODO(fabric2seer): `Player.fishing` has no Rust field in the port (searched net.minecraft.world.entity.player.Player and its supertypes)
                // rust: ItemCooldowns::handle_item_cooldown(GRAPPLING_HOOK_ID.to_string(), 2000, (true /* java: player.fishing != null: never null here */ && !(ItemCooldowns::is_wearing_bat_armor(player))));
            }
            ROGUE_SWORD_ID | SPIRIT_LEAP_ID | LIVID_DAGGER_ID => {
                ItemCooldowns::handle_item_cooldown_2(&used_item_id, 5000);
            }
            SHADOW_FURY_ID => {
                ItemCooldowns::handle_item_cooldown_2(SHADOW_FURY_ID, 15000);
            }
            INK_WAND_ID | GIANTS_SWORD_ID => {
                ItemCooldowns::handle_item_cooldown_2(&used_item_id, 30000);
            }
            GREAT_SPOOK_STAFF_ID => {
                ItemCooldowns::handle_item_cooldown_2(GREAT_SPOOK_STAFF_ID, 60000);
            }
            _ => {
            }
        }
        return InteractionResult::Pass;
    }

    /// java: private static void handleItemCooldown(String itemId, int cooldownTime, boolean additionalCondition)
    pub fn handle_item_cooldown(mut item_id: String, mut cooldown_time: i32, mut additional_condition: bool) {
        if !(ItemCooldowns::is_on_cooldown_string(&item_id)) && additional_condition {
            compat::lock(&ITEM_COOLDOWNS).insert(item_id.clone(), CooldownEntry::new(cooldown_time));
        }
    }

    /// java: private static void handleItemCooldown(String itemId, int cooldownTime)
    pub fn handle_item_cooldown_2(item_id: &str, mut cooldown_time: i32) {
        ItemCooldowns::handle_item_cooldown(item_id.to_string(), cooldown_time, true);
    }

    /// java: public static boolean isOnCooldown(ItemStack itemStack)
    pub fn is_on_cooldown(item_stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> bool {
        return ItemCooldowns::is_on_cooldown_string(&item_stack.get_skyblock_id());
    }

    /// java: private static boolean isOnCooldown(String itemId)
    pub fn is_on_cooldown_string(item_id: &str) -> bool {
        if compat::lock(&ITEM_COOLDOWNS).contains_key(item_id) {
            let mut cooldown_entry: CooldownEntry = compat::lock(&ITEM_COOLDOWNS).get(item_id).cloned().unwrap();
            if cooldown_entry.is_on_cooldown() {
                return true;
            } else {
                compat::lock(&ITEM_COOLDOWNS).remove(item_id);
                return false;
            }
        }
        return false;
    }

    /// java: public static CooldownEntry getItemCooldownEntry(ItemStack itemStack)
    pub fn get_item_cooldown_entry(item_stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> CooldownEntry {
        return compat::lock(&ITEM_COOLDOWNS).get(&item_stack.get_skyblock_id()).cloned().unwrap();
    }

    /// java: private static boolean isWearingBatArmor(Player player)
    pub fn is_wearing_bat_armor(player: &dyn Player) -> bool {
        for stack in ItemUtils::get_armor(player) {
            let mut item_id: String = stack.get_skyblock_id();
            if !(BAT_ARMOR_IDS.iter().any(|e| *e == item_id.clone())) {
                return false;
            }
        }
        return true;
    }

}

/// java: public record CooldownEntry(int cooldown, long startTime)
#[derive(Clone, Debug, PartialEq)]
pub struct CooldownEntry {
    /// java: int cooldown
    pub cooldown: i32,
    /// java: long startTime
    pub start_time: i64,
}

impl CooldownEntry {
    /// java: public CooldownEntry(int cooldown)
    pub fn new(mut cooldown: i32) -> Self {
        let mut this = Self::new_2(cooldown, compat::millis());
        this
    }

    /// java: the canonical record constructor
    /// (the canonical record constructor is `new_N` here: Rust has no overloading and `new` is the explicit one)
    pub fn new_2(mut cooldown: i32, mut start_time: i64) -> Self {
        Self { cooldown, start_time }
    }

    /// java: the record accessor `cooldown()`
    pub fn cooldown(&self) -> i32 {
        self.cooldown
    }

    /// java: the record accessor `startTime()`
    pub fn start_time(&self) -> i64 {
        self.start_time
    }

    /// java: public boolean isOnCooldown()
    pub fn is_on_cooldown(&self) -> bool {
        return (self.start_time.wrapping_add(self.cooldown as i64) > compat::millis());
    }

    /// java: public long getRemainingCooldown()
    pub fn get_remaining_cooldown(&self) -> i64 {
        let mut time: i64 = (self.start_time.wrapping_add(self.cooldown as i64)).wrapping_sub(compat::millis());
        return time.max((0 as i64));
    }

    /// java: public float getRemainingCooldownPercent()
    pub fn get_remaining_cooldown_percent(&self) -> f32 {
        return (if self.is_on_cooldown() { (((self.get_remaining_cooldown()) as f32) / (self.cooldown as f32)) } else { 0.0f32 });
    }

}

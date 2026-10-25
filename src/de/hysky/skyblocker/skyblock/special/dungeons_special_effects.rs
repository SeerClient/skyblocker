//! Ported from de.hysky.skyblocker.skyblock.special.DungeonsSpecialEffects by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::itemlist::item_repository::ItemRepository;
use crate::de::hysky::skyblocker::skyblock::special::special_effects::SpecialEffects;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final Pattern DUNGEON_CHEST_PATTERN = Pattern.compile("^\\s{3,}(?!.*:)(?:RARE REWARD!\\s+)?(?<item>.+)$");
pub static DUNGEON_CHEST_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("^\\s{3,}(?!.*:)(?:RARE REWARD!\\s+)?(?<item>.+)$"));

/// java: public class DungeonsSpecialEffects
#[derive(Clone)]
pub struct DungeonsSpecialEffects {}

impl DungeonsSpecialEffects {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        // java: ClientReceiveMessageEvents.ALLOW_GAME.register(DungeonsSpecialEffects::display_rare_drop_effect);
        compat::register_allow_game(|message, overlay| DungeonsSpecialEffects::display_rare_drop_effect(message, overlay));
    }

    /// java: private static boolean displayRareDropEffect(Component message, boolean overlay)
    pub fn display_rare_drop_effect(message: &MutableComponent, mut overlay: bool) -> bool {
        if !(Utils::is_on_skyblock()) || overlay || !(SkyblockerConfigManager::get().general.special_effects.rare_dungeon_drop_effects) {
            return true;
        }
        // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut string_form: String = message.get_string();
            let mut matcher: compat::regex::Matcher = DUNGEON_CHEST_PATTERN.matcher(&string_form);
            if matcher.matches() {
                let mut stack: Option<compat::Shared<FlexibleItemStack>> = DungeonsSpecialEffects::get_stack_from_name(matcher.group_named("item").as_deref().unwrap());
                if stack.is_some() && FlexibleItemStack::get_stack(stack.as_ref().unwrap()).is_some() && !(FlexibleItemStack::get_stack_or_throw(stack.as_ref().unwrap()).is_empty()) {
                    SpecialEffects::display_special_effect(&FlexibleItemStack::get_stack_or_throw(stack.as_ref().unwrap()), mcrust::net::minecraft::core::particles::particle_types::portal());
                }
            }
        }));
        if let Err(e) = __result {
            logger().error(&format!("[Skyblocker Special Effects] An unexpected exception was encountered!"));
        }
        return true;
    }

    /// java: private static @Nullable FlexibleItemStack getStackFromName(String itemName)
    pub fn get_stack_from_name(item_name: &str) -> Option<compat::Shared<FlexibleItemStack>> {
        let mut item_id: String = match item_name {
            "Recombobulator 3000" => "RECOMBOBULATOR_3000".to_string(),
            "First Master Star" => "FIRST_MASTER_STAR".to_string(),
            "Second Master Star" => "SECOND_MASTER_STAR".to_string(),
            "Spirit Mask" => "SPIRIT_MASK".to_string(),
            "Third Master Star" => "THIRD_MASTER_STAR".to_string(),
            "Shadow Fury" => "SHADOW_FURY".to_string(),
            "Giant's Sword" => "GIANTS_SWORD".to_string(),
            "Fourth Master Star" => "FOURTH_MASTER_STAR".to_string(),
            "Necron Dye" => "NECRON_DYE".to_string(),
            "Dark Claymore" => "DARK_CLAYMORE".to_string(),
            "Necron's Handle" | "Shiny Necron's Handle" => "NECRON_HANDLE".to_string(),
            "Master Skull - Tier 5" => "MASTER_SKULL_TIER_5".to_string(),
            "Shadow Warp" => "SHADOW_WARP_SCROLL".to_string(),
            "Wither Shield" => "WITHER_SHIELD_SCROLL".to_string(),
            "Implosion" => "IMPLOSION_SCROLL".to_string(),
            "Fifth Master Star" => "FIFTH_MASTER_STAR".to_string(),
            _ => "NONE".to_string(),
        };
        return ItemRepository::get_item_stack(&item_id);
    }

}

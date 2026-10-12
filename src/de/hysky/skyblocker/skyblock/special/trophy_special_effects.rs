//! Ported from de.hysky.skyblocker.skyblock.special.TrophySpecialEffects by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::itemlist::item_repository::ItemRepository;
use crate::de::hysky::skyblocker::skyblock::special::special_effects::SpecialEffects;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final Pattern DIAMOND_PATTERN = Pattern.compile("^NEW DISCOVERY:\\s+(?<item>.+?)\\s+DIAMOND$");
pub static DIAMOND_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("^NEW DISCOVERY:\\s+(?<item>.+?)\\s+DIAMOND$"));

/// java: public class TrophySpecialEffects
#[derive(Clone)]
pub struct TrophySpecialEffects {}

impl TrophySpecialEffects {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        // TODO(fabric2seer): ClientReceiveMessageEvents.ALLOW_GAME.register: no SeerClient equivalent yet
        //   java: ClientReceiveMessageEvents.ALLOW_GAME.register(TrophySpecialEffects::displayRareDropEffect);
    }

    /// java: private static boolean displayRareDropEffect(Component message, boolean overlay)
    pub fn display_rare_drop_effect(message: &MutableComponent, mut overlay: bool) -> bool {
        if !(Utils::is_on_skyblock()) || overlay || !(SkyblockerConfigManager::get().general.special_effects.trophy_drop_effects) {
            return true;
        }
        // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut string_form: String = message.get_string();
            let mut matcher: compat::regex::Matcher = DIAMOND_PATTERN.matcher(&string_form);
            if matcher.matches() {
                let mut stack: Option<compat::Shared<FlexibleItemStack>> = TrophySpecialEffects::get_stack_from_name(matcher.group_named("item").as_deref().unwrap());
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
        let mut item_id: String = match item_name.as_str() {
            "Blobfish" => "BLOBFISH_DIAMOND".to_string(),
            "Flyfish" => "FLYFISH_DIAMOND".to_string(),
            "Golden Fish" => "GOLDEN_FISH_DIAMOND".to_string(),
            "Gusher" => "GUSHER_DIAMOND".to_string(),
            "Karate Fish" => "KARATE_FISH_DIAMOND".to_string(),
            "Lava Horse" => "LAVA_HORSE_DIAMOND".to_string(),
            "Mana Ray" => "MANA_RAY_DIAMOND".to_string(),
            "Moldfin" => "MOLDFIN_DIAMOND".to_string(),
            "Skeleton Fish" => "SKELETON_FISH_DIAMOND".to_string(),
            "Slugfish" => "SLUGFISH_DIAMOND".to_string(),
            "Soul Fish" => "SOUL_FISH_DIAMOND".to_string(),
            "Steaming Hot Flounder" => "STEAMING_HOT_FLOUNDER_DIAMOND".to_string(),
            "Sulphur Skitter" => "SULPHUR_SKITTER_DIAMOND".to_string(),
            "Vanille" => "VANILLE_DIAMOND".to_string(),
            "Volcanic Stonefish" => "VOLCANIC_STONEFISH_DIAMOND".to_string(),
            "Obfuscated-1" => "OBFUSCATED_FISH_1_DIAMOND".to_string(),
            "Obfuscated-2" => "OBFUSCATED_FISH_2_DIAMOND".to_string(),
            "Obfuscated-3" => "OBFUSCATED_FISH_3_DIAMOND".to_string(),
            "Common Frog" => "COMMON_FROG_DIAMOND".to_string(),
            "Exploding Frog" => "EXPLODING_FROG_DIAMOND".to_string(),
            "Leap Frog" => "LEAP_FROG_DIAMOND".to_string(),
            "Wetlands Frog" => "WETLANDS_FROG_DIAMOND".to_string(),
            "Reality Hopper" => "REALITY_HOPPER_DIAMOND".to_string(),
            "Blessed Frog" => "BLESSED_FROG_DIAMOND".to_string(),
            "Bullfrog" => "BULLFROG_DIAMOND".to_string(),
            "Sea Frog" => "SEA_FROG_DIAMOND".to_string(),
            "Cave Frog" => "CAVE_FROG_DIAMOND".to_string(),
            "Highlands Frog" => "HIGHLANDS_FROG_DIAMOND".to_string(),
            "Tree Frog" => "TREE_FROG_DIAMOND".to_string(),
            "Puddle Jumper" => "PUDDLE_JUMPER_DIAMOND".to_string(),
            _ => "NONE".to_string(),
        };
        return ItemRepository::get_item_stack(&item_id);
    }

}

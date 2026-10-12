//! Ported from de.hysky.skyblocker.skyblock.special.RareDropSpecialEffects by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::itemlist::item_repository::ItemRepository;
use crate::de::hysky::skyblocker::skyblock::special::special_effects::SpecialEffects;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use crate::de::hysky::skyblocker::utils::sky_block_icons;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final Pattern MAGIC_FIND_PATTERN = Pattern.compile("^(?!.*:)(?:RARE|VERY RARE|CRAZY RARE|INSANE) DROP!\\s+\\(?(?<item>.+?)\\)?(?:\\s+\\(\\+\\d+%? "+SkyBlockIcons.MAGIC_FIND+" Magic Find
pub static MAGIC_FIND_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile(&format!("^(?!.*:)(?:RARE|VERY RARE|CRAZY RARE|INSANE) DROP!\\s+\\(?(?<item>.+?)\\)?(?:\\s+\\(\\+\\d+%? {} Magic Find\\))?$", sky_block_icons::MAGIC_FIND)));
/// java: private static final Pattern OVERBLOOM_PATTERN = Pattern.compile("^RARE CROP!\\s+(?<crop>[\\w\\s]+)\\s+\\(\\+\\d+(?:\\.\\d+)?" + SkyBlockIcons.OVERBLOOM + "\\).*");
pub static OVERBLOOM_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile(&format!("^RARE CROP!\\s+(?<crop>[\\w\\s]+)\\s+\\(\\+\\d+(?:\\.\\d+)?{}\\).*", sky_block_icons::OVERBLOOM)));

/// java: public class RareDropSpecialEffects
#[derive(Clone)]
pub struct RareDropSpecialEffects {}

impl RareDropSpecialEffects {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        // TODO(fabric2seer): ClientReceiveMessageEvents.ALLOW_GAME.register: no SeerClient equivalent yet
        //   java: ClientReceiveMessageEvents.ALLOW_GAME.register(RareDropSpecialEffects::displayRareDropEffect);
    }

    /// java: private static boolean displayRareDropEffect(Component message, boolean overlay)
    pub fn display_rare_drop_effect(message: &MutableComponent, mut overlay: bool) -> bool {
        if Utils::is_on_skyblock() && SkyblockerConfigManager::get().general.special_effects.rare_drop_effects && !overlay {
            // java: try {..} catch (Exception e) - the body runs inline: a Rust body does not throw checked exceptions (a panic in it propagates)
            {
                let mut string_form: String = message.get_string();
                let mut magic_find_matcher: compat::regex::Matcher = MAGIC_FIND_PATTERN.matcher(&string_form);
                if magic_find_matcher.matches() {
                    RareDropSpecialEffects::trigger_drop_effect(magic_find_matcher.group_named("item").as_deref().unwrap());
                    return true;
                }
                let mut overbloom_matcher: compat::regex::Matcher = OVERBLOOM_PATTERN.matcher(&string_form);
                if overbloom_matcher.matches() {
                    RareDropSpecialEffects::trigger_drop_effect(overbloom_matcher.group_named("crop").as_deref().unwrap());
                }
            }
            // TODO(fabric2seer): the `catch (Exception e)` block is not reached in Rust - the try body returns, breaks or continues, so it cannot run inside `catch_unwind`
            //   java: { //In case there's a regex failure or something else bad happens
            //   java: 				LOGGER.error("[Skyblocker Special Effects] An unexpected exception was encountered: ", e);
            //   java: 			}
        }
        return true;
    }

    /// java: private static void triggerDropEffect(String itemName)
    pub fn trigger_drop_effect(item_name: &str) {
        let mut stack: Option<compat::Shared<FlexibleItemStack>> = RareDropSpecialEffects::get_stack_from_name(item_name);
        if stack.is_some() && FlexibleItemStack::get_stack(stack.as_ref().unwrap()).is_some() && !(FlexibleItemStack::get_stack(stack.as_ref().unwrap()).unwrap().is_empty()) {
            SpecialEffects::display_special_effect(&FlexibleItemStack::get_stack_or_throw(stack.as_ref().unwrap()), mcrust::net::minecraft::core::particles::particle_types::scrape());
        }
    }

    /// java: private static @Nullable FlexibleItemStack getStackFromName(String itemName)
    pub fn get_stack_from_name(item_name: &str) -> Option<compat::Shared<FlexibleItemStack>> {
        let mut item_id: String = match item_name {
            "Enchanted Book (Chimera I)" => "ULTIMATE_CHIMERA;1".to_string(),
            "Fateful Stinger" => "FATEFUL_STINGER".to_string(),
            "Manti-core" => "MANTI_CORE".to_string(),
            "Minos Relic" => "MINOS_RELIC".to_string(),
            "Shimmering Wool" => "SHIMMERING_WOOL".to_string(),
            "Scythe Blade" => "SCYTHE_BLADE".to_string(),
            "Shredded Sinew" => "SHARD_OF_THE_SHREDDED".to_string(),
            "Severed Hand" => "SEVERED_HAND".to_string(),
            "Warden Heart" => "WARDEN_HEART".to_string(),
            "Shriveled Wasp" => "SHRIVELED_WASP".to_string(),
            "Digested Mosquito" => "DIGESTED_MOSQUITO".to_string(),
            "Ensnared Snail" => "ENSNARED_SNAIL".to_string(),
            "Primordial Eye" => "PRIMORDIAL_EYE".to_string(),
            "Overflux Capacitor" => "OVERFLUX_CAPACITOR".to_string(),
            "End Stone Idol" => "END_STONE_IDOL".to_string(),
            "Judgement Core" => "JUDGEMENT_CORE".to_string(),
            "High Class Archfiend Dice" => "HIGH_CLASS_ARCHFIEND_DICE".to_string(),
            "Prince's Crown Jewel" => " PRINCES_CROWN_JEWEL".to_string(),
            "Pocket-sized Igloo" => "POCKET_SIZED_IGLOO".to_string(),
            "Radioactive Vial" => "RADIOACTIVE_VIAL".to_string(),
            "Tiki Mask" => "TIKI_MASK".to_string(),
            "Titanoboa Shed" => "TITANOBOA_SHED".to_string(),
            "Ray of Helios" => "RAY_OF_HELIOS".to_string(),
            _ => "NONE".to_string(),
        };
        return ItemRepository::get_item_stack(&item_id);
    }

}

//! Ported from de.hysky.skyblocker.skyblock.rift.McGrubberUpdater by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::utils::regex_utils::RegexUtils;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::net::minecraft::world::inventory::slot::Slot;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::ContainerScreenHost as _;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final String MOTES_GRUBBER = "Motes Grubber";
pub const MOTES_GRUBBER: &str = "Motes Grubber";
/// java: private static final String CONSUMABLE_ITEMS = "Miscellaneous ➜ Consumable Items";
pub const CONSUMABLE_ITEMS: &str = "Miscellaneous ➜ Consumable Items";
/// java: private static final Pattern ORB_PICKUP = Pattern.compile("ORB! Picked up \\+(?<motes>\\d+) Motes, recovered \\+2ф Rift Time!");
pub static ORB_PICKUP: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("ORB! Picked up \\+(?<motes>\\d+) Motes, recovered \\+2ф Rift Time!"));
/// java: private static final Pattern MOTES_PRICE = Pattern.compile("(?<price>\\d+(\\.\\d)?) Motes");
pub static MOTES_PRICE: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("(?<price>\\d+(\\.\\d)?) Motes"));
/// java: private static final Pattern MCGRUBBER_PROGRESS = Pattern.compile("Total Progress: (?<percent>\\d+)%");
pub static MCGRUBBER_PROGRESS: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("Total Progress: (?<percent>\\d+)%"));

/// java: public class McGrubberUpdater
#[derive(Clone)]
pub struct McGrubberUpdater {}

impl McGrubberUpdater {
    /// java: private McGrubberUpdater()
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        screen_events::BEFORE_INIT.register(Arc::new(|screen: &mut Screen, _width: i32, _height: i32| {
            // java: if (!Utils.isOnSkyblock() || !(screen instanceof ContainerScreen containerScreen)) return;
            //       else if (!SkyblockerConfigManager.get().otherLocations.rift.autoDetectMcGrubber) return;
            if !Utils::is_on_skyblock() || !matches!(compat::minecraft().gui.screen_kind, mcrust::net::minecraft::client::gui::gui::ScreenKind::Container) {
                return;
            } else if !SkyblockerConfigManager::get().other_locations.rift.auto_detect_mc_grubber {
                return;
            }
            // `containerScreen.getMenu()` is the player's open `ChestMenu`, read when the screen is removed
            let title: String = screen.get_title().to_string();
            if Utils::is_in_the_rift() && title == MOTES_GRUBBER {
                // java: ScreenEvents.remove(screen).register(_ -> McGrubberUpdater.fromMotesGrubber(containerScreen));
                screen_events::remove(screen, Arc::new(|_| {
                    let slots: Vec<Slot> = mcrust::mcrust::modding::game::with_chest_menu(|menu| menu.menu.slots.clone()).unwrap_or_default();
                    McGrubberUpdater::from_motes_grubber(&slots);
                }));
            } else if !Utils::is_in_the_rift() && title == CONSUMABLE_ITEMS {
                // java: ScreenEvents.remove(screen).register(_ -> McGrubberUpdater.fromConsumableItems(containerScreen));
                screen_events::remove(screen, Arc::new(|_| {
                    let slots: Vec<Slot> = mcrust::mcrust::modding::game::with_chest_menu(|menu| menu.menu.slots.clone()).unwrap_or_default();
                    McGrubberUpdater::from_consumable_items(&slots);
                }));
            }
        }));
        // java: ClientReceiveMessageEvents.ALLOW_GAME.register(McGrubberUpdater::fromOrbPickup);
        compat::register_allow_game(|text, overlay| McGrubberUpdater::from_orb_pickup(text, overlay));
    }

    /// java: private static void fromMotesGrubber(ContainerScreen screen) - `screen.getMenu().slots`
    pub fn from_motes_grubber(slots: &[Slot]) {
        use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;
        use crate::de::hysky::skyblocker::skyblock::item::tooltip::info::tooltip_info_type;
        use crate::de::hysky::skyblocker::skyblock::item::tooltip::info::data_tooltip_info_type::DataTooltipInfoType as _;
        for slot in slots.iter() {
            // java: if (slot.container != Minecraft.getInstance().player.getInventory()) continue;
            if !matches!(slot.container, mcrust::net::minecraft::world::inventory::slot::SlotContainer::Inventory) {
                continue;
            }
            let item: mcrust::net::minecraft::world::item::item_stack::ItemStack = compat::slot_item(slot);
            let Some(matcher) = crate::de::hysky::skyblocker::utils::item_utils::ItemUtils::get_lore_line_if_match(&item, &MOTES_PRICE) else {
                continue;
            };
            // java: float price = NumberUtils.toFloat(matcher.group("price")); (0 for an unparseable string)
            let price: f32 = matcher.group_named("price").and_then(|p| p.parse::<f32>().ok()).unwrap_or(0.0);
            let internal_id: String = item.get_skyblock_id();
            // Compare against base price to determine mcgrubber stacks
            if tooltip_info_type::MOTES.get_mut().lock().borrow().has_or_null_warning(&internal_id) {
                // x is mcgrubber stacks, y is final price, & z is initial price
                // y = (1 + 0.05x)z -> reorder to solve for x -> x = 20y / z - 20
                let base: i32 = tooltip_info_type::MOTES.get_mut().lock().borrow().get_data().and_then(|data| data.get(&internal_id).copied()).unwrap_or(0);
                // java: Math.round(float) - floor(x + 0.5)
                let mc_grubber_stacks: i32 = ((20.0f32 * price / (base as f32) - 20.0f32) + 0.5f32).floor() as i32;
                if SkyblockerConfigManager::get().other_locations.rift.mc_grubber_stacks != mc_grubber_stacks {
                    SkyblockerConfigManager::update_only(Arc::new(move |config| { config.other_locations.rift.mc_grubber_stacks = mc_grubber_stacks; }));
                }
                break;
            }
        }
    }

    /// java: private static void fromConsumableItems(ContainerScreen screen) - `screen.getMenu().slots`
    pub fn from_consumable_items(slots: &[Slot]) {
        use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;
        for slot in slots.iter() {
            let item: mcrust::net::minecraft::world::item::item_stack::ItemStack = compat::slot_item(slot);
            if item.get_skyblock_id() != "MCGRUBBER_BURGER" {
                continue;
            }
            let Some(matcher) = crate::de::hysky::skyblocker::utils::item_utils::ItemUtils::get_lore_line_if_match(&item, &MCGRUBBER_PROGRESS) else {
                continue;
            };
            // 20% progress per mcgrubber stack
            let mc_grubber_stacks: i32 = RegexUtils::parse_int_from_matcher_matcher_string(&matcher, "percent").wrapping_div(20);
            if SkyblockerConfigManager::get().other_locations.rift.mc_grubber_stacks != mc_grubber_stacks {
                SkyblockerConfigManager::update_only(Arc::new(move |config| { config.other_locations.rift.mc_grubber_stacks = mc_grubber_stacks; }));
            }
            break;
        }
    }

    /// java: private static boolean fromOrbPickup(Component text, boolean overlay)
    pub fn from_orb_pickup(text: &MutableComponent, mut overlay: bool) -> bool {
        if !(SkyblockerConfigManager::get().other_locations.rift.auto_detect_mc_grubber) {
            return true;
        } else if !(Utils::is_on_skyblock()) || !(Utils::is_in_the_rift()) || overlay {
            return true;
        }
        let mut matcher: compat::regex::Matcher = ORB_PICKUP.matcher(&text.get_string());
        if matcher.matches() {
            let mut motes: i32 = RegexUtils::parse_int_from_matcher_matcher_string(&matcher, "motes");
            let mut mc_grubber_stacks: i32 = (motes.wrapping_sub(5)).wrapping_div(60);
            if SkyblockerConfigManager::get().other_locations.rift.mc_grubber_stacks != mc_grubber_stacks {
                SkyblockerConfigManager::update_only(Arc::new(move |mut config| { config.other_locations.rift.mc_grubber_stacks = mc_grubber_stacks; }));
            }
        }
        return true;
    }

}

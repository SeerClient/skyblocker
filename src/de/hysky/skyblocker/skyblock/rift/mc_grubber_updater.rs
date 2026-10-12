//! Ported from de.hysky.skyblocker.skyblock.rift.McGrubberUpdater by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::utils::regex_utils::RegexUtils;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::net::minecraft::client::gui::screens::inventory::container_screen::ContainerScreen;
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
        screen_events::BEFORE_INIT.register(Arc::new(|screen: &mut Screen, width: i32, height: i32| {
            // TODO(fabric2seer): an `instanceof` pattern under `||` / `!`
            //   java: if (!Utils.isOnSkyblock() || !(screen instanceof ContainerScreen containerScreen)) {
            //   java: 				return;
            //   java: 			} else if (!SkyblockerConfigManager.get().otherLocations.rift.autoDetectMcGrubber) {
            //   java: 				return;
            //   java: 			}
            // TODO(fabric2seer): unknown name `containerScreen`
            //   java: if (Utils.isInTheRift() && containerScreen.getTitle().getString().equals(MOTES_GRUBBER)) {
            //   java: 				ScreenEvents.remove(screen).register(_ -> McGrubberUpdater.fromMotesGrubber(containerScreen));
            //   java: 			} else if (!Utils.isInTheRift() && containerScreen.getTitle().getString().equals(CONSUMABLE_ITEMS)) {
            //   java: 				ScreenEvents.remove(screen).register(_ -> McGrubberUpdater.fromConsumableItems(containerScreen));
            //   java: 			}
        }));
        // TODO(fabric2seer): ClientReceiveMessageEvents.ALLOW_GAME.register: no SeerClient equivalent yet
        //   java: ClientReceiveMessageEvents.ALLOW_GAME.register(McGrubberUpdater::fromOrbPickup);
    }

    /// java: private static void fromMotesGrubber(ContainerScreen screen)
    pub fn from_motes_grubber(screen: &ContainerScreen) {
        // TODO(fabric2seer): field `slots` of a value of unknown type
        //   java: for (Slot slot : screen.getMenu().slots) {
        //   java: 			if (slot.container != Minecraft.getInstance().player.getInventory()) continue;
        //   java:
        //   java: 			ItemStack item = slot.getItem();
        //   java: 			Matcher matcher = ItemUtils.getLoreLineIfMatch(item, MOTES_PRICE);
        //   java:
        //   java: 			if (matcher == null) continue;
        //   java:
        //   java: 			float price = NumberUtils.toFloat(matcher.group("price"));
        //   java: 			final String internalID = item.getSkyblockId();
        //   java:
        //   java: 			// Compare against base price to determine mcgrubber stacks
        //   java: 			if (TooltipInfoType.MOTES.hasOrNullWarning(internalID)) {
        //   java: 				// x is mcgrubber stacks, y is final price, & z is initial price
        //   java: 				// y = (1 + 0.05x)z -> reorder to solve for x -> x = 20y / z - 20
        //   java: 				int mcGrubberStacks = Math.round(20 * price / TooltipInfoType.MOTES.getData().getInt(internalID) - 20);
        //   java:
        //   java: 				if (SkyblockerConfigManager.get().otherLocations.rift.mcGrubberStacks != mcGrubberStacks) {
        //   java: 					SkyblockerConfigManager.updateOnly(config -> config.otherLocations.rift.mcGrubberStacks = mcGrubberStacks);
        //   java: 				}
        //   java:
        //   java: 				break;
        //   java: 			}
        //   java: 		}
    }

    /// java: private static void fromConsumableItems(ContainerScreen screen)
    pub fn from_consumable_items(screen: &ContainerScreen) {
        // TODO(fabric2seer): field `slots` of a value of unknown type
        //   java: for (Slot slot : screen.getMenu().slots) {
        //   java: 			ItemStack item = slot.getItem();
        //   java:
        //   java: 			if (!item.getSkyblockId().equals("MCGRUBBER_BURGER")) continue;
        //   java:
        //   java: 			Matcher matcher = ItemUtils.getLoreLineIfMatch(item, MCGRUBBER_PROGRESS);
        //   java:
        //   java: 			if (matcher == null) continue;
        //   java:
        //   java: 			// 20% progress per mcgrubber stack
        //   java: 			int mcGrubberStacks = RegexUtils.parseIntFromMatcher(matcher, "percent") / 20;
        //   java:
        //   java: 			if (SkyblockerConfigManager.get().otherLocations.rift.mcGrubberStacks != mcGrubberStacks) {
        //   java: 				SkyblockerConfigManager.updateOnly(config -> config.otherLocations.rift.mcGrubberStacks = mcGrubberStacks);
        //   java: 			}
        //   java:
        //   java: 			break;
        //   java: 		}
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
                SkyblockerConfigManager::update_only(Arc::new(move |config| { config.other_locations.rift.mc_grubber_stacks = mc_grubber_stacks; }));
            }
        }
        return true;
    }

}

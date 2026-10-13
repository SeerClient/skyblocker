//! Ported from de.hysky.skyblocker.skyblock.quicknav.QuickNav by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::configs::quick_navigation_config::ItemData;
use crate::de::hysky::skyblocker::config::configs::quick_navigation_config::QuickNavItem;
use crate::de::hysky::skyblocker::config::configs::quick_navigation_config::QuickNavigationConfig;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::quicknav::quick_nav_button::QuickNavButton;
use crate::de::hysky::skyblocker::skyblock::quicknav::quick_nav_button::QuickNavButtonBase;
use crate::de::hysky::skyblocker::skyblock::quicknav::quick_nav_confirmation_button::QuickNavConfirmationButton;
use crate::de::hysky::skyblocker::utils::constants;
use crate::de::hysky::skyblocker::utils::datafixer::item_stack_componentization_fixer::ItemStackComponentizationFixer;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::player::local_player::LocalPlayer;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::world::item::items;

/// java: public class QuickNav
#[derive(Clone)]
pub struct QuickNav {}

impl QuickNav {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static List<QuickNavButton> init(String screenTitle)
    pub fn init(screen_title: &str) -> Vec<compat::Shared<dyn QuickNavButton>> {
        let mut buttons: Vec<compat::Shared<dyn QuickNavButton>> = Vec::new();
        let data = &SkyblockerConfigManager::get().quick_nav;
        // java: try {..} catch (CommandSyntaxException e) - an exception here is a panic, caught by `catch_unwind`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if compat::read(&data.button1, |__t| __t.render) {
                buttons.push(QuickNav::parse_button(&data.button1, screen_title, 0));
            }
            if compat::read(&data.button2, |__t| __t.render) {
                buttons.push(QuickNav::parse_button(&data.button2, screen_title, 1));
            }
            if compat::read(&data.button3, |__t| __t.render) {
                buttons.push(QuickNav::parse_button(&data.button3, screen_title, 2));
            }
            if compat::read(&data.button4, |__t| __t.render) {
                buttons.push(QuickNav::parse_button(&data.button4, screen_title, 3));
            }
            if compat::read(&data.button5, |__t| __t.render) {
                buttons.push(QuickNav::parse_button(&data.button5, screen_title, 4));
            }
            if compat::read(&data.button6, |__t| __t.render) {
                buttons.push(QuickNav::parse_button(&data.button6, screen_title, 5));
            }
            if compat::read(&data.button7, |__t| __t.render) {
                buttons.push(QuickNav::parse_button(&data.button7, screen_title, 6));
            }
            if compat::read(&data.button8, |__t| __t.render) {
                buttons.push(QuickNav::parse_button(&data.button8, screen_title, 7));
            }
            if compat::read(&data.button9, |__t| __t.render) {
                buttons.push(QuickNav::parse_button(&data.button9, screen_title, 8));
            }
            if compat::read(&data.button10, |__t| __t.render) {
                buttons.push(QuickNav::parse_button(&data.button10, screen_title, 9));
            }
            if compat::read(&data.button11, |__t| __t.render) {
                buttons.push(QuickNav::parse_button(&data.button11, screen_title, 10));
            }
            if compat::read(&data.button12, |__t| __t.render) {
                buttons.push(QuickNav::parse_button(&data.button12, screen_title, 11));
            }
            if compat::read(&data.button13, |__t| __t.render) {
                buttons.push(QuickNav::parse_button(&data.button13, screen_title, 12));
            }
            if compat::read(&data.button14, |__t| __t.render) {
                buttons.push(QuickNav::parse_button(&data.button14, screen_title, 13));
            }
        }));
        if let Err(e) = __result {
            logger().error(&format!("[Skyblocker] Failed to initialize Quick Nav Button"));
        }
        return buttons;
    }

    /// java: private static QuickNavButton parseButton(QuickNavigationConfig.QuickNavItem buttonInfo, String screenTitle, int id) throws CommandSyntaxException
    pub fn parse_button(button_info: &compat::Shared<QuickNavItem>, screen_title: &str, mut id: i32) -> compat::Shared<dyn QuickNavButton> {
        let mut item_data: Option<ItemData> = compat::read(button_info, |__t| __t.item_data.clone());
        // java: ItemStack stack = itemData != null && itemData.item != null && itemData.components != null ? ItemStackComponentizationFixer.fromComponentsString(itemData.item.toString(), Math.clamp(itemData.count, 1, 99), itemData.components) : new ItemStack(Items.BARRIER);
        // (`Item.toString()` is the port's `Display` - the registered name; the config's `components` is a non-null `String`)
        let mut stack: mcrust::net::minecraft::world::item::item_stack::ItemStack = match item_data.as_ref().and_then(|data| data.item.as_ref().map(|item| (data, item))) {
            Some((data, item)) => ItemStackComponentizationFixer::from_components_string(&item.to_string(), data.count.clamp(1, 99), &data.components),
            None => mcrust::net::minecraft::world::item::item_stack::ItemStack::of_one(items::items().get("barrier")),
        };
        let mut ui_title_matches: bool = false;
        // java: try {..} catch (PatternSyntaxException e) - an exception here is a panic, caught by `catch_unwind`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            ui_title_matches = compat::regex::Pattern::matches(&compat::read(button_info, |__t| __t.ui_title.clone()), &screen_title);
        }));
        if let Err(e) = __result {
            logger().error(&format!("[Skyblocker] Failed to parse Quick Nav Button with regex: {}", compat::read(button_info, |__t| __t.ui_title.clone())));
            if mcrust::mcrust::modding::game::with_player(|_| ()).is_some() {
                mcrust::mcrust::modding::game::with_player(|player| { script_host::send_feedback(&(constants::PREFIX.get_mut())().append((component::literal(&format!("Invalid regex in Quick Nav Button {}!", id.wrapping_add(1)))).with_style_format(ChatFormatting::Red)).to_string()); });
            }
        }
        if compat::read(button_info, |__t| __t.double_click) {
            return (QuickNavConfirmationButton::new(id, ui_title_matches, compat::read(button_info, |__t| __t.click_event.clone()), stack.clone(), compat::read(button_info, |__t| __t.tooltip.clone())) as compat::Shared<dyn QuickNavButton>);
        }
        return (QuickNavButtonBase::new(id, ui_title_matches, compat::read(button_info, |__t| __t.click_event.clone()), stack.clone(), compat::read(button_info, |__t| __t.tooltip.clone())) as compat::Shared<dyn QuickNavButton>);
    }

}

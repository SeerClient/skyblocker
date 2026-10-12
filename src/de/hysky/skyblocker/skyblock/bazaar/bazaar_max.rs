//! Ported from de.hysky.skyblocker.skyblock.bazaar.BazaarMax by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use mcrust::net::minecraft::client::gui::font::text_field_helper::TextFieldHelper;
use mcrust::net::minecraft::world::inventory::slot::Slot;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;

/// java: private static final String BUY_ORDER_QUANTITY = "Buy Order Quantity";
pub const BUY_ORDER_QUANTITY: &str = "Buy Order Quantity";
/// java: private static final String CLICK_TO_SPECIFY = "Click to specify!";
pub const CLICK_TO_SPECIFY: &str = "Click to specify!";
/// java: private static final Pattern MAX_QUANTITY_PATTERN = Pattern.compile("Buy up to ([0-9,]+)x\\.");
pub static MAX_QUANTITY_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("Buy up to ([0-9,]+)x\\."));
/// java: private static final String PAGE_TITLE = "How many do you want\\?";
pub const PAGE_TITLE: &str = "How many do you want\\?";
/// java: public static final BazaarMax INSTANCE = new BazaarMax();
/// (a mutable singleton - `checkMaxValue` writes `lastSeenMax` - reached on the render thread only)
pub static INSTANCE: compat::MainThread<BazaarMax> = compat::MainThread::new(|| BazaarMax::new());

/// java: public final class BazaarMax
#[derive(Clone)]
pub struct BazaarMax {
    /// java: private int lastSeenMax = -1;
    pub last_seen_max: i32,
}

impl BazaarMax {
    /// java: private BazaarMax()
    pub fn new() -> Self {
        let mut this = Self {
            last_seen_max: -1,
        };
        this
    }

    /// java: public void checkMaxValue(@Nullable Slot focusedSlot)
    pub fn check_max_value(&mut self, mut focused_slot: Option<Slot>) {
        let Some(mut focused_slot) = focused_slot.clone() else {
            return;
        };
        let mut has_buy_order_quantity: bool = false;
        let mut has_click_to_specify: bool = false;
        let mut max_quantity_matcher: Option<compat::regex::Matcher> = None;
        // java: ItemStack stack = focusedSlot.getItem(); - the port's slot reads its item through the open menu's
        // containers (`compat::slot_item`)
        let mut stack: mcrust::net::minecraft::world::item::item_stack::ItemStack = compat::slot_item(&focused_slot);
        let mut lines: Vec<String> = stack.skyblocker_get_lore_strings();
        for line in lines.iter() {
            if BUY_ORDER_QUANTITY == line {
                has_buy_order_quantity = true;
                continue;
            }
            if CLICK_TO_SPECIFY == line {
                has_click_to_specify = true;
                continue;
            }
            let mut matcher: compat::regex::Matcher = MAX_QUANTITY_PATTERN.matcher(line);
            if matcher.matches() {
                max_quantity_matcher = Some(matcher.clone());
            }
        }
        if has_buy_order_quantity && has_click_to_specify && max_quantity_matcher.is_some() {
            // java: try { lastSeenMax = Integer.parseInt(maxQuantityMatcher.group(1).replace(",", "")); } catch (NumberFormatException _) {}
            if let Ok(__v) = (max_quantity_matcher.as_mut().unwrap().group(1).unwrap().replace(",", "")).parse::<i32>() {
                self.last_seen_max = __v;
            }
        }
    }

    /// java: public void expandMax(TextFieldHelper signField, String currentLine)
    pub fn expand_max(&self, sign_field: &mut TextFieldHelper, current_line: &str) {
        if self.last_seen_max < 0 {
            return;
        }
        match current_line {
            "max " => {
                // TODO(fabric2seer): `remove_chars_from_cursor` takes 2 argument(s) in the port (`message: &str, count: i32`), Java passes 1
                // rust: sign_field.remove_chars_from_cursor(&-4.to_string());
            }
            "m " | "x " => {
                // TODO(fabric2seer): `remove_chars_from_cursor` takes 2 argument(s) in the port (`message: &str, count: i32`), Java passes 1
                // rust: sign_field.remove_chars_from_cursor(&-2.to_string());
            }
            _ => {
                return;
            }
        }
        // TODO(fabric2seer): the port's `insert_text_at_cursor` ports this Java method with other parameters (message, text, validator for Java's text)
        // TODO(fabric2seer): `insert_text_at_cursor` takes 3 argument(s) in the port (`message: &str, text: &str, validator: impl Fn(&str) -> bool`), Java passes 1
        // rust: sign_field.insert_text_at_cursor(&self.last_seen_max.to_string());
    }

    /// java: public boolean isEnabled()
    pub fn is_enabled(&self) -> bool {
        return SkyblockerConfigManager::get().helpers.bazaar.enable_bazaar_max;
    }

}

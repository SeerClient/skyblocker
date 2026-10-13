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
    ///
    /// The port's `TextFieldHelper` holds no getter / setter of the line (Java's `() -> messages[line]` /
    /// `this::setMessage`) and no validator: it is handed the line and answers the new one. So the edit is made on the
    /// sign screen that owns the three - `expand_max_on` - and this form (the helper alone) answers the new line for
    /// the caller to `setMessage`.
    pub fn expand_max(&self, sign_field: &mut TextFieldHelper, current_line: &str) -> Option<String> {
        self.expand_max_with(sign_field, current_line, |_| true)
    }

    /// `expandMax(screen.signField, currentLine)` on the sign screen: the helper's setter is the screen's `setMessage`
    /// and its validator `font.width(s) <= sign.getMaxTextLineWidth()` (AbstractSignEditScreen:62-68).
    pub fn expand_max_on(&self, screen: &mut mcrust::net::minecraft::client::gui::screens::inventory::abstract_sign_edit_screen::AbstractSignEditScreen) {
        let current_line: String = screen.current_message().to_string();
        let max_width: i32 = screen.max_text_line_width;
        let font = compat::font();
        let Some(sign_field) = screen.sign_field.as_mut() else {
            return;
        };
        if let Some(updated) = self.expand_max_with(sign_field, &current_line, |s| font.width(s) <= max_width) {
            screen.set_message(updated);
        }
    }

    /// The body of `expandMax`: the line after the two helper calls (`None` when it was not changed).
    fn expand_max_with(&self, sign_field: &mut TextFieldHelper, current_line: &str, validator: impl Fn(&str) -> bool) -> Option<String> {
        if self.last_seen_max < 0 {
            return None;
        }
        // java: signField.removeCharsFromCursor(-4) / (-2) - the helper's getter reads the line, its setter writes it back
        let removed: Option<String> = match current_line {
            "max " => sign_field.remove_chars_from_cursor(current_line, -4),
            "m " | "x " => sign_field.remove_chars_from_cursor(current_line, -2),
            _ => {
                return None;
            }
        };
        let line: String = removed.unwrap_or_else(|| current_line.to_string());
        // java: signField.insertText(String.valueOf(lastSeenMax));
        let inserted: Option<String> = sign_field.insert_text_at_cursor(&line, &self.last_seen_max.to_string(), validator);
        Some(inserted.unwrap_or(line))
    }

    /// java: public boolean isEnabled()
    pub fn is_enabled(&self) -> bool {
        return SkyblockerConfigManager::get().helpers.bazaar.enable_bazaar_max;
    }

}

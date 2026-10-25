//! Ported from de.hysky.skyblocker.skyblock.bazaar.BazaarQuickQuantities by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::calculators::sign_calculator::SignCalculator;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::PressLambda;
use mcrust::net::minecraft::client::gui::components::button;
use mcrust::net::minecraft::client::gui::screens::screen;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// `messages[0]` as the last button press wrote it, until the sign screen commits it (`install_sign_commit`).
static PENDING_FIRST_LINE: std::sync::Mutex<Option<String>> = std::sync::Mutex::new(None);

/// java: public class BazaarQuickQuantities
#[derive(Clone)]
pub struct BazaarQuickQuantities {}

impl BazaarQuickQuantities {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static @Nullable Button[] getButtons(int width, String[] messages)
    /// (Java's `@Nullable Button[]`: the ELEMENTS are nullable - the clipboard button is absent unless the clipboard
    /// holds a number)
    pub fn get_buttons(mut width: i32, messages: &mut Vec<String>) -> Option<Vec<Option<AbstractWidget>>> {
        let mut buttons: Vec<Option<AbstractWidget>> = vec![None; 4];
        let config = &SkyblockerConfigManager::get().ui_and_visuals.bazaar_quick_quantities;
        let mut x: i32 = (width.wrapping_div(2)).wrapping_add(50);
        let mut y: i32 = 45;
        // java: each button's action is `_ -> onButtonPressed(messages, config.slotNQuantity)` over the sign screen's own
        // `messages` array.
        // A button's press is handed the sign screen's `Screen` half, not the `AbstractSignEditScreen`, so the action
        // writes into the array as it was lent here, and the value it writes into `messages[0]` reaches the screen's own
        // array where the sign screen commits it (`removed:121-131`, see `install_sign_commit`).
        // The open sign editor's own `messages[0]` is written through `game::with_sign_edit_screen` (see
        // `on_button_pressed_string_string`), so the new first line also shows on the sign before it closes.
        BazaarQuickQuantities::install_sign_commit();
        let __messages = compat::RenderThread::new(std::sync::Arc::new(std::sync::Mutex::new(messages.clone())));
        let (__q1, __q2, __q3) = (config.slot1_quantity, config.slot2_quantity, config.slot3_quantity);
        let __m = __messages.clone();
        buttons[(0) as usize] = Some((button::builder(component::null_to_empty(Some(&__q1.to_string())).get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { BazaarQuickQuantities::on_button_pressed(&mut __m.get().lock().unwrap_or_else(|p| p.into_inner()), __q1); })))).bounds(x, y.wrapping_add(20), 50, 20).build());
        let __m = __messages.clone();
        buttons[(1) as usize] = Some((button::builder(component::null_to_empty(Some(&__q2.to_string())).get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { BazaarQuickQuantities::on_button_pressed(&mut __m.get().lock().unwrap_or_else(|p| p.into_inner()), __q2); })))).bounds(x, y.wrapping_add(40), 50, 20).build());
        let __m = __messages.clone();
        buttons[(2) as usize] = Some((button::builder(component::null_to_empty(Some(&__q3.to_string())).get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { BazaarQuickQuantities::on_button_pressed(&mut __m.get().lock().unwrap_or_else(|p| p.into_inner()), __q3); })))).bounds(x, y.wrapping_add(60), 50, 20).build());
        // java: String clipboard = client.keyboardHandler.getClipboard().replace(",", "");
        let mut clipboard: String = {
            let mut __memory = mcrust::net::minecraft::client::gui::components::edit_box::MemoryClipboard::default();
            match mcrust::net::minecraft::client::minecraft::SdlClipboard::published() {
                Some(mut __glfw) => compat::minecraft().keyboard_handler.get_clipboard(&mut __glfw),
                None => compat::minecraft().keyboard_handler.get_clipboard(&mut __memory),
            }
        }.replace(",", "");
        // java: if (clipboard.length() <= 5 && NumberUtils.isCreatable(clipboard))
        if compat::str_len(&clipboard) <= 5 && compat::number_utils_is_creatable(&clipboard) {
            let mut text: MutableComponent = component::literal(&clipboard);
            // java: text.withStyle(style -> style.withColor(ChatFormatting.AQUA)); - `MutableComponent.withStyle` restyles
            // `text` itself (and answers it): assigned back
            text = text.with_style_fn(|style| style.with_color_format(Some(ChatFormatting::Aqua)));
            let __m = __messages.clone();
            let __clipboard = clipboard.clone();
            buttons[(3) as usize] = Some((button::builder(text.get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { BazaarQuickQuantities::on_button_pressed_string_string(&mut __m.get().lock().unwrap_or_else(|p| p.into_inner()), __clipboard.clone()); })))).bounds(x, y.wrapping_add(80), 50, 20).build());
        }
        return Some(buttons);
    }

    /// The sign screen's `messages[0]` as a button press wrote it: applied to the lines the screen commits
    /// (`AbstractSignEditScreen.removed:121-131` answers them), once.
    fn install_sign_commit() {
        static ONCE: std::sync::Once = std::sync::Once::new();
        ONCE.call_once(|| {
            let hooked = mcrust::mcrust::modding::hook::modify_return_value::<(bool, [String; 4])>("net.minecraft.client.gui.screens.inventory.AbstractSignEditScreen#removed", |(front, mut lines)| {
                if let Some(first) = PENDING_FIRST_LINE.lock().unwrap_or_else(|p| p.into_inner()).take() {
                    lines[0] = first;
                }
                (front, lines)
            });
            if let Err(e) = hooked {
                logger().error(&format!("[Skyblocker Bazaar Quick Quantities] Failed to hook the sign commit: {:?}", e));
            }
        });
    }

    /// java: static void onButtonPressed(String[] messages, int value)
    pub fn on_button_pressed(messages: &mut Vec<String>, mut value: i32) {
        BazaarQuickQuantities::on_button_pressed_string_string(messages, value.to_string());
    }

    /// java: static void onButtonPressed(String[] messages, String value)
    pub fn on_button_pressed_string_string(messages: &mut Vec<String>, mut value: String) {
        messages[(0) as usize] = value.clone();
        // the sign screen's own `messages[0]` (Java's array is the screen's)
        let __line: String = value.clone();
        mcrust::mcrust::modding::game::with_sign_edit_screen(move |screen| {
            screen.messages[0] = __line;
        });
        *PENDING_FIRST_LINE.lock().unwrap_or_else(|p| p.into_inner()) = Some(value.clone());
        let config = &SkyblockerConfigManager::get().ui_and_visuals.bazaar_quick_quantities;
        if config.close_sign_on_use && compat::minecraft().gui.screen().is_some() {
            SignCalculator::calculate(messages[(0) as usize].clone());
            // java: client.gui.screen().onClose(); - the open screen is the sign screen these buttons are on, whose
            // `onClose:116-119` is `onDone:233-235`, `minecraft.gui.setScreen(null)` (which commits the text through
            // `removed`)
            mcrust::mcrust::modding::game::set_screen_null();
        }
    }

}


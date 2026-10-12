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
    pub fn get_buttons(mut width: i32, messages: &mut Vec<String>) -> Option<Vec<AbstractWidget>> {
        let mut buttons: Vec<AbstractWidget> = (0..(4) as usize).map(|_| Default::default()).collect::<Vec<_>>();
        let config = &SkyblockerConfigManager::get().ui_and_visuals.bazaar_quick_quantities;
        let mut x: i32 = (width.wrapping_div(2)).wrapping_add(50);
        let mut y: i32 = 45;
        buttons[(0) as usize] = (button::builder(component::null_to_empty(Some(&config.slot1_quantity.to_string())).get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { BazaarQuickQuantities::on_button_pressed(messages, config.slot1_quantity); })))).bounds(x, y.wrapping_add(20), 50, 20).build();
        buttons[(1) as usize] = (button::builder(component::null_to_empty(Some(&config.slot2_quantity.to_string())).get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { BazaarQuickQuantities::on_button_pressed(messages, config.slot2_quantity); })))).bounds(x, y.wrapping_add(40), 50, 20).build();
        buttons[(2) as usize] = (button::builder(component::null_to_empty(Some(&config.slot3_quantity.to_string())).get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { BazaarQuickQuantities::on_button_pressed(messages, config.slot3_quantity); })))).bounds(x, y.wrapping_add(60), 50, 20).build();
        // TODO(fabric2seer): `get_clipboard` takes 1 argument(s) in the port (`clipboard: &mut dyn ClipboardAccess`), Java passes 0
        // rust: let mut clipboard: String = compat::minecraft().keyboard_handler.get_clipboard().replace(",", "");
        let mut clipboard: String = todo!("fabric2seer: the initialiser of `clipboard` (TODO above)");
        // TODO(fabric2seer): the condition: static call NumberUtils.isCreatable(..)
        //   java: clipboard.length() <= 5 && NumberUtils.isCreatable(clipboard)
        if todo!("fabric2seer: the condition (TODO above)") {
            let mut text: MutableComponent = component::literal(&clipboard);
            text.with_style_fn(|style| style.with_color_format(Some(ChatFormatting::Aqua)));
            buttons[(3) as usize] = (button::builder(text.get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { BazaarQuickQuantities::on_button_pressed_string_string(messages, clipboard.clone()); })))).bounds(x, y.wrapping_add(80), 50, 20).build();
        }
        return Some(buttons);
    }

    /// java: static void onButtonPressed(String[] messages, int value)
    pub fn on_button_pressed(messages: &mut Vec<String>, mut value: i32) {
        BazaarQuickQuantities::on_button_pressed_string_string(messages, value.to_string());
    }

    /// java: static void onButtonPressed(String[] messages, String value)
    pub fn on_button_pressed_string_string(messages: &mut Vec<String>, mut value: String) {
        messages[(0) as usize] = value.clone();
        let config = &SkyblockerConfigManager::get().ui_and_visuals.bazaar_quick_quantities;
        if config.close_sign_on_use && compat::minecraft().gui.screen().is_some() {
            SignCalculator::calculate(messages[(0) as usize].clone());
            // TODO(fabric2seer): `on_close_for_interrupt` takes 12 argument(s) in the port (`gfx: &mut crate::net::minecraft::client::minecraft::Minecraft, screen: &mut Screen, screen_extra: &mut crate::net::minecraft::client::gui::gui::ScreenExtra, options_data: &mut crate::net::minecraft::client::options::OptionsData, open_sign_e ..`), Java passes 0
            // rust: screen::on_close_for_interrupt();
        }
    }

}

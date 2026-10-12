//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.AbstractSignEditScreenMixin by fabric2seer.
//! @Mixin(net.minecraft.client.gui.screens.inventory.AbstractSignEditScreen)

#[allow(unused_imports)]
use crate::prelude::*;

pub static SPEED_INPUT_MARKER: LazyLock<Mutex<String>> = LazyLock::new(|| Mutex::new(String::from("speed cap!")));
pub static INPUT_SIGN_MARKER: LazyLock<Mutex<String>> = LazyLock::new(|| Mutex::new(String::from("^^^^^^^^^^^^^^^")));
pub static ALT_INPUT_SIGN_MARKER: LazyLock<Mutex<String>> = LazyLock::new(|| Mutex::new(String::from("^^^^^^")));
pub static BAZAAR_FLIP_MARKER: LazyLock<Mutex<String>> = LazyLock::new(|| Mutex::new(String::from("^^Flipping^^")));
pub static SET_NAME_MARKER: LazyLock<Mutex<String>> = LazyLock::new(|| Mutex::new(String::from("Enter name")));

/// @Unique
fn is_speed_input_sign(this: &mut mcrust::net::minecraft::client::gui::screens::inventory::abstract_sign_edit_screen::AbstractSignEditScreen) -> bool {
    return (this.messages[(3) as usize] == compat::get_state(&SPEED_INPUT_MARKER));
}

/// @Unique
fn is_input_search_sign(this: &mut mcrust::net::minecraft::client::gui::screens::inventory::abstract_sign_edit_screen::AbstractSignEditScreen) -> bool {
    return ((this.messages[(2) as usize]).ends_with("your") || (this.messages[(2) as usize]).ends_with("query"));
}

/// @Unique
fn is_set_name_sign(this: &mut mcrust::net::minecraft::client::gui::screens::inventory::abstract_sign_edit_screen::AbstractSignEditScreen) -> bool {
    return (this.messages[(2) as usize] == compat::get_state(&SET_NAME_MARKER));
}

/// @Unique
fn is_alt_input_search_sign(this: &mut mcrust::net::minecraft::client::gui::screens::inventory::abstract_sign_edit_screen::AbstractSignEditScreen) -> bool {
    return (this.messages[(2) as usize]).ends_with("your");
}

/// @Unique
fn is_input_sign(this: &mut mcrust::net::minecraft::client::gui::screens::inventory::abstract_sign_edit_screen::AbstractSignEditScreen) -> bool {
    return (this.messages[(1) as usize] == compat::get_state(&INPUT_SIGN_MARKER) && !(is_input_search_sign(this)) || this.messages[(1) as usize] == compat::get_state(&ALT_INPUT_SIGN_MARKER) && !(is_alt_input_search_sign(this)) && !(is_set_name_sign(this)) || this.messages[(1) as usize] == compat::get_state(&BAZAAR_FLIP_MARKER));
}

/// @Inject skyblocker$init
///   java: @Inject(method = "init", at = @At("TAIL"))
/// target: net.minecraft.client.gui.screens.inventory.AbstractSignEditScreen#init  [exists]
/// Rust: net/minecraft/client/gui/screens/inventory/abstract_sign_edit_screen.rs:853 fn init(&mut self, width: i32, height: i32, done_label: &str)
fn skyblocker_init(ci: &mut CallbackInfo<'_>) {
    let Some(this) = compat::detach(ci.this_mut::<mcrust::net::minecraft::client::gui::screens::inventory::abstract_sign_edit_screen::AbstractSignEditScreen>()) else { return; };
    if crate::de::hysky::skyblocker::utils::utils::Utils::is_on_skyblock() {
        let mut config = crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get();
        if is_input_sign(this) && this.messages[(3) as usize] == "to order" && config.ui_and_visuals.bazaar_quick_quantities.enabled {
            let mut buttons: Vec<mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget> = crate::de::hysky::skyblocker::skyblock::bazaar::bazaar_quick_quantities::BazaarQuickQuantities::get_buttons(this.screen.width, &mut this.messages).unwrap();
            for button in buttons.iter() {
                if true /* java: button != null: never null here */ {
                    // TODO(fabric2seer): call to `addRenderableWidget(..)` (not a method of this class)
                    //   java: addRenderableWidget(button);
                }
            }
        }
    }
}

/// @Inject skyblocker$charTyped
///   java: @Inject(method = "charTyped", at = @At("TAIL"))
/// target: net.minecraft.client.gui.screens.inventory.AbstractSignEditScreen#charTyped  [exists]
/// Rust: net/minecraft/client/gui/screens/inventory/abstract_sign_edit_screen.rs:956 fn char_typed(&mut self, c: char, line_fits: impl Fn(&str) -> bool)
fn skyblocker_char_typed(ci: &mut CallbackInfo<'_>) {
    let Some(this) = compat::detach(ci.this_mut::<mcrust::net::minecraft::client::gui::screens::inventory::abstract_sign_edit_screen::AbstractSignEditScreen>()) else { return; };
    // TODO(fabric2seer): the condition: `event`: this argument is not visible to the injector (see the notes above the handler)
    //   java: event.codepoint() == ' ' && Utils.isOnSkyblock()
    //   java: 				&& SkyblockerConfigManager.get().helpers.bazaar.enableBazaarMax
    //   java: 				&& isInputSign() && messages[3].equals("to order")
    if todo!("fabric2seer: the condition (TODO above)") {
        crate::de::hysky::skyblocker::skyblock::bazaar::bazaar_max::INSTANCE.expand_max(&mut this.sign_field, &this.messages[(this.line) as usize]);
    }
}

/// @Inject skyblocker$extractRenderStateSign
///   java: @Inject(method = "extractRenderState", at = @At("HEAD"))
/// target: net.minecraft.client.gui.screens.inventory.AbstractSignEditScreen#extractRenderState  [exists]
/// Rust: net/minecraft/client/gui/screens/inventory/abstract_sign_edit_screen.rs:1209 fn extract_render_state(gl: &crate::lwjgl::opengl::Gl, gfx: &crate::net::minecraft::client::minecraft::Minecraft, state: &AbstractSignEditScreen, gui_w: i32, now_ms: i64, title: &str, screen: &crate::net::minecraft::client::gui::screens::screen::Screen, gui_scale: i32, fb_height: i32)
fn skyblocker_extract_render_state_sign(ci: &mut CallbackInfo<'_>) {
    let Some(this) = compat::detach_ref(ci.arg::<mcrust::net::minecraft::client::gui::screens::inventory::abstract_sign_edit_screen::AbstractSignEditScreen>(2)) else { return; };
    // `this`: this port's extractRenderState is a free function; its `state` argument (index 2) is the AbstractSignEditScreen
    if crate::de::hysky::skyblocker::utils::utils::Utils::is_on_skyblock() {
        let mut config = crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get();
        if is_speed_input_sign(this) && config.general.speed_presets.enable_speed_presets {
            let mut presets = crate::de::hysky::skyblocker::skyblock::speedpreset::speed_presets::SpeedPresets::get_instance();
            if presets.has_preset(&this.messages[(0) as usize]) {
                // TODO(fabric2seer): `graphics`: this argument is not visible to the injector (see the notes above the handler)
                //   java: graphics.centeredText(this.font, Component.literal(String.format("%s » %d", messages[0], presets.getPreset(messages[0]))).withStyle(ChatFormatting.GREEN),
                //   java: 							graphics.guiWidth() / 2, 55, 0xFFFFFFFF);
            }
        } else if is_input_sign(this) && config.ui_and_visuals.input_calculator.enabled {
            // TODO(fabric2seer): `graphics`: this argument is not visible to the injector (see the notes above the handler)
            //   java: SignCalculator.extractCalculator(graphics, messages[0], graphics.guiWidth() / 2, 55);
        }
    }
}

/// @Inject skyblocker$keyPressed
///   java: @Inject(method = "keyPressed", at = @At("HEAD"))
/// target: net.minecraft.client.gui.screens.inventory.AbstractSignEditScreen#keyPressed  [exists]
/// Rust: net/minecraft/client/gui/screens/inventory/abstract_sign_edit_screen.rs:917 fn key_pressed(&mut self, is_up: bool, is_down_or_confirmation: bool)
fn skyblocker_key_pressed(ci: &mut CallbackInfo<'_>) {
    let Some(this) = compat::detach(ci.this_mut::<mcrust::net::minecraft::client::gui::screens::inventory::abstract_sign_edit_screen::AbstractSignEditScreen>()) else { return; };
    // TODO(fabric2seer): the condition: `input`: this argument is not visible to the injector (see the notes above the handler)
    //   java: SkyblockerConfigManager.get().uiAndVisuals.inputCalculator.closeSignsWithEnter
    //   java: 				&& Utils.isOnSkyblock() && isInputSign()
    //   java: 				&& (input.isConfirmation())
    if todo!("fabric2seer: the condition (TODO above)") {
        this.on_close();
    }
}

/// @Inject skyblocker$finishEditing
///   java: @Inject(method = "onDone", at = @At("HEAD"))
/// target: net.minecraft.client.gui.screens.inventory.AbstractSignEditScreen#onDone  [exists]
/// Rust: net/minecraft/client/gui/screens/inventory/abstract_sign_edit_screen.rs:1030 fn on_done(&self)
fn skyblocker_finish_editing(ci: &mut CallbackInfo<'_>) {
    let Some(this) = compat::detach_ref(ci.this::<mcrust::net::minecraft::client::gui::screens::inventory::abstract_sign_edit_screen::AbstractSignEditScreen>()) else { return; };
    let mut config = crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get();
    if crate::de::hysky::skyblocker::utils::utils::Utils::is_on_skyblock() {
        if is_speed_input_sign(this) && config.general.speed_presets.enable_speed_presets {
            let mut presets = crate::de::hysky::skyblocker::skyblock::speedpreset::speed_presets::SpeedPresets::get_instance();
            if presets.has_preset(&this.messages[(0) as usize]) {
                this.messages[(0) as usize] = presets.get_preset(&this.messages[(0) as usize]).to_string();
            }
        } else if is_input_sign(this) && config.ui_and_visuals.input_calculator.enabled {
            let mut is_price: bool = (this.messages[(2) as usize]).contains("price");
            let mut value: String = crate::de::hysky::skyblocker::skyblock::calculators::sign_calculator::SignCalculator::get_new_value(is_price);
            if compat::str_len(&value) >= 15 {
                value = compat::substring2(&value, 0, 15);
            }
            this.messages[(0) as usize] = value.clone();
        }
    }
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    report(hook::register_injector("net.minecraft.client.gui.screens.inventory.AbstractSignEditScreen#init", InjectOptions::at(At::Tail).kind(InjectorKind::Inject).priority(1000), Arc::new(skyblocker_init)));
    report(hook::register_injector("net.minecraft.client.gui.screens.inventory.AbstractSignEditScreen#charTyped", InjectOptions::at(At::Tail).kind(InjectorKind::Inject).priority(1000), Arc::new(skyblocker_char_typed)));
    report(hook::register_injector("net.minecraft.client.gui.screens.inventory.AbstractSignEditScreen#extractRenderState", InjectOptions::at(At::Head).kind(InjectorKind::Inject).priority(1000), Arc::new(skyblocker_extract_render_state_sign)));
    report(hook::register_injector("net.minecraft.client.gui.screens.inventory.AbstractSignEditScreen#keyPressed", InjectOptions::at(At::Head).kind(InjectorKind::Inject).priority(1000), Arc::new(skyblocker_key_pressed)));
    report(hook::register_injector("net.minecraft.client.gui.screens.inventory.AbstractSignEditScreen#onDone", InjectOptions::at(At::Head).kind(InjectorKind::Inject).priority(1000), Arc::new(skyblocker_finish_editing)));
}

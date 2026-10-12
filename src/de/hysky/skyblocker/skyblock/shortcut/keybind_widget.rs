//! Ported from de.hysky.skyblocker.skyblock.shortcut.KeybindWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::shortcut::shortcut_key_binding::ShortcutKeyBinding;
use mcrust::com::mojang::blaze3d::platform::input_constants;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::PressLambda;
use mcrust::net::minecraft::client::gui::components::button::CreateNarration;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::input::input_with_modifiers::InputWithModifiers;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::client::gui::components::events::gui_event_listener::GuiEventListener as _;

/// java: public class KeybindWidget extends Button
pub struct KeybindWidget {
    /// java: extends net.minecraft.client.gui.components.Button
    pub base: AbstractWidget,
    /// java: private final ShortcutKeyBinding keyBinding;
    pub key_binding: Option<compat::Shared<ShortcutKeyBinding>>,
    /// java: private final Runnable updateListener;
    pub update_listener: Option<Arc<dyn Fn()>>,
    /// java: private boolean editing;
    pub editing: bool,
}

impl std::ops::Deref for KeybindWidget {
    type Target = AbstractWidget;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for KeybindWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl KeybindWidget {
    /// java: protected KeybindWidget(ShortcutKeyBinding keyBinding, int x, int y, int width, int height, Component message, Runnable updateListener)
    pub fn new(mut key_binding: compat::Shared<ShortcutKeyBinding>, mut x: i32, mut y: i32, mut width: i32, mut height: i32, mut message: MutableComponent, update_listener: Arc<dyn Fn()>) -> Self {
        let mut this = Self::new_8(key_binding.clone(), x, y, width, height, message.clone(), DEFAULT_NARRATION, update_listener.clone());
        this
    }

    /// java: protected KeybindWidget(ShortcutKeyBinding keyBinding, int x, int y, int width, int height, Component message, CreateNarration narrationSupplier, Runnable updateListener)
    pub fn new_8(mut key_binding: compat::Shared<ShortcutKeyBinding>, mut x: i32, mut y: i32, mut width: i32, mut height: i32, mut message: MutableComponent, mut narration_supplier: CreateNarration, update_listener: Arc<dyn Fn()>) -> Self {
        let mut this = Self::new_9(key_binding.clone(), x, y, width, height, message.clone(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| {

        })), narration_supplier, update_listener.clone());
        this
    }

    /// java: protected KeybindWidget(ShortcutKeyBinding keyBinding, int x, int y, int width, int height, Component message, OnPress onPress, CreateNarration narrationSupplier, Runnable updateListener)
    pub fn new_9(mut key_binding: compat::Shared<ShortcutKeyBinding>, mut x: i32, mut y: i32, mut width: i32, mut height: i32, mut message: MutableComponent, mut on_press: OnPress, mut narration_supplier: CreateNarration, update_listener: Arc<dyn Fn()>) -> Self {
        let __init_key_binding = Some(key_binding.clone());
        let __init_update_listener = Some(update_listener.clone());
        // TODO(fabric2seer): `new Button(..)`: the port has no `Button::new`
        //   java: super(x, y, width, height, message, onPress, narrationSupplier);
        let mut this: Self = todo!("constructor");
        this
    }

    /// java: public boolean isEditing()
    pub fn is_editing(&self) -> bool {
        return self.editing;
    }

    /// java: public boolean stopEditing()
    pub fn stop_editing(&mut self) -> bool {
        let mut was_editing: bool = self.editing;
        self.editing = false;
        return was_editing;
    }

    /// java: @Override public void onPress(InputWithModifiers input)
    pub fn on_press(&mut self, input: &dyn InputWithModifiers) {
        self.editing = true;
        ShortcutKeyBinding::clear_bound_keys(self.key_binding.as_ref().unwrap());
        (self.update_listener.as_ref().unwrap())();
        // TODO(fabric2seer): `Button.onPress` has no Rust method in the port (looked for `on_press` in Button and its supertypes)
        // rust: self.base.on_press(input);
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    pub fn mouse_clicked(&mut self, click: &MouseButtonEvent, mut doubled: bool) -> bool {
        if self.editing {
            // TODO(fabric2seer): field `MOUSE` of a value of unknown type
            //   java: keyBinding.addBoundKey(InputConstants.Type.MOUSE.getOrCreate(click.button()));
            (self.update_listener.as_ref().unwrap())();
            return true;
        }
        // TODO(fabric2seer): `mouse_clicked` takes 3 argument(s) in the port (`button: i32, x: f64, y: f64`), Java passes 2
        // rust: return self.base.mouse_clicked(click.clone(), doubled).is_some();
        return todo!("fabric2seer: see the TODO above");
    }

    /// java: @Override public boolean keyPressed(KeyEvent input)
    pub fn key_pressed(&mut self, input: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        if self.editing {
            if input.is_escape() {
                ShortcutKeyBinding::add_bound_key(self.key_binding.as_ref().unwrap(), mcrust::com::mojang::blaze3d::platform::input_constants::unknown());
            } else {
                // TODO(fabric2seer): the port's `get_key_from_event` ports this Java method with other parameters (key, scancode for Java's event)
                // TODO(fabric2seer): `get_key_from_event` takes 2 argument(s) in the port (`key: i32, scancode: i32`), Java passes 1
                // rust: ShortcutKeyBinding::add_bound_key(self.key_binding.as_ref().unwrap(), input_constants::get_key_from_event(input.clone()));
            }
            (self.update_listener.as_ref().unwrap())();
            return true;
        }
        // TODO(fabric2seer): `key_pressed` takes 3 argument(s) in the port (`_key: i32, _scancode: i32, _modifiers: i32`), Java passes 1
        // TODO(fabric2seer): `key_pressed` is a method of the port's trait `GuiEventListener`, which the port's `AbstractWidget` does not implement
        // rust: return self.base.key_pressed(input.clone());
        return todo!("fabric2seer: see the TODO above");
    }

    /// java: @Override protected void extractContents(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_contents(&self, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        // TODO(fabric2seer): `Button.extractDefaultSprite` has no Rust method in the port (looked for `extract_default_sprite` in Button and its supertypes)
        // rust: self.extract_default_sprite(graphics);
        // TODO(fabric2seer): field `NONE` of a value of unknown type
        //   java: this.extractDefaultLabel(graphics.textRenderer(GuiGraphicsExtractor.HoveredTextEffects.NONE));
    }

}

// NOTE(fabric2seer): extractContents, keyPressed, mouseClicked, onPress override(s) of Button - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them

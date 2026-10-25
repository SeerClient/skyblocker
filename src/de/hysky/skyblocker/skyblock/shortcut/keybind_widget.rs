//! Ported from de.hysky.skyblocker.skyblock.shortcut.KeybindWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::network::chat::component::Component as _;
use crate::de::hysky::skyblocker::skyblock::shortcut::shortcut_key_binding::ShortcutKeyBinding;
use mcrust::com::mojang::blaze3d::platform::input_constants;
use mcrust::mcrust::modding::widgets::ModWidget;
use mcrust::mcrust::modding::widgets::WidgetRenderContext;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::WidgetKind;
use mcrust::net::minecraft::client::gui::components::button::CreateNarration;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::input::key_event::KeyEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public class KeybindWidget extends Button
pub struct KeybindWidget {
    /// java: extends net.minecraft.client.gui.components.Button - the widget half (x, y, size, message, active,
    /// tooltip, narration supplier), lent by the game for each call (`compat::WidgetAdapter`)
    pub base: AbstractWidget,
    /// java: private final ShortcutKeyBinding keyBinding; - the SAME object the list row holds
    pub key_binding: compat::Shared<ShortcutKeyBinding>,
    /// java: private final Runnable updateListener;
    pub update_listener: Arc<dyn Fn()>,
    /// java: private boolean editing;
    pub editing: bool,
    /// `Button.onPress` - the `OnPress` the constructor was given (the no-op `_ -> {}` of the shorter ones)
    pub on_press: Arc<dyn Fn()>,
    /// `extractDefaultSprite` + `extractDefaultLabel`, drawn by the game's own button pass
    pub look: compat::ButtonLook,
    /// `Button.createNarration` when it is a lambda reading live state (`textSupplier -> ..`): called with Java's
    /// `() -> super.createNarrationMessage()` each time the widget is narrated. `None` is the data form the widget
    /// half carries (`base.create_narration`).
    pub narration_supplier: Option<Arc<dyn Fn(&dyn Fn() -> mcrust::net::minecraft::network::chat::formatted_text::FormattedText) -> mcrust::net::minecraft::network::chat::formatted_text::FormattedText>>,
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
    pub fn new(key_binding: compat::Shared<ShortcutKeyBinding>, x: i32, y: i32, width: i32, height: i32, message: MutableComponent, update_listener: Arc<dyn Fn()>) -> compat::Shared<Self> {
        // java: this(keyBinding, x, y, width, height, message, DEFAULT_NARRATION, updateListener);
        Self::new_8(key_binding, x, y, width, height, message, CreateNarration::Default, update_listener)
    }

    /// java: protected KeybindWidget(ShortcutKeyBinding keyBinding, int x, int y, int width, int height, Component message, CreateNarration narrationSupplier, Runnable updateListener)
    pub fn new_8(key_binding: compat::Shared<ShortcutKeyBinding>, x: i32, y: i32, width: i32, height: i32, message: MutableComponent, narration_supplier: CreateNarration, update_listener: Arc<dyn Fn()>) -> compat::Shared<Self> {
        // java: this(keyBinding, x, y, width, height, message, _ -> {}, narrationSupplier, updateListener);
        Self::new_9(key_binding, x, y, width, height, message, Arc::new(|| {}), narration_supplier, update_listener)
    }

    /// java: protected KeybindWidget(ShortcutKeyBinding keyBinding, int x, int y, int width, int height, Component message, OnPress onPress, CreateNarration narrationSupplier, Runnable updateListener)
    pub fn new_9(key_binding: compat::Shared<ShortcutKeyBinding>, x: i32, y: i32, width: i32, height: i32, message: MutableComponent, on_press: Arc<dyn Fn()>, narration_supplier: CreateNarration, update_listener: Arc<dyn Fn()>) -> compat::Shared<Self> {
        // java: super(x, y, width, height, message, onPress, narrationSupplier);
        let mut base = AbstractWidget::new(x, y, width, height, message.get_string(), WidgetKind::Button, OnPress::Inert);
        base.message_text = Some(mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(message));
        base.create_narration = narration_supplier;
        compat::shared(Self { base, key_binding, update_listener, editing: false, on_press, look: compat::ButtonLook::new(), narration_supplier: None })
    }

    /// java: `new KeybindWidget(keyBinding, x, y, width, height, message, textSupplier -> .., updateListener)` - the
    /// eight-argument constructor with a `CreateNarration` lambda that reads live state
    pub fn new_with_narration(
        key_binding: compat::Shared<ShortcutKeyBinding>,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        message: MutableComponent,
        narration_supplier: Arc<dyn Fn(&dyn Fn() -> mcrust::net::minecraft::network::chat::formatted_text::FormattedText) -> mcrust::net::minecraft::network::chat::formatted_text::FormattedText>,
        update_listener: Arc<dyn Fn()>,
    ) -> compat::Shared<Self> {
        let this = Self::new_8(key_binding, x, y, width, height, message, CreateNarration::Default, update_listener);
        compat::write(&this, |t| t.narration_supplier = Some(narration_supplier));
        this
    }

    /// `new KeybindWidget(..)` handed to the game (`List.of(keybindButton, replacement)` - the row's children)
    pub fn widget(this: compat::Shared<Self>) -> AbstractWidget {
        compat::WidgetAdapter::widget(this)
    }

    /// java: public boolean isEditing()
    pub fn is_editing(&self) -> bool {
        return self.editing;
    }

    /// java: public boolean stopEditing()
    pub fn stop_editing(&mut self) -> bool {
        let was_editing: bool = self.editing;
        self.editing = false;
        return was_editing;
    }

    /// java: @Override public void onPress(InputWithModifiers input)
    pub fn on_press(&mut self) {
        self.editing = true;
        compat::write(&self.key_binding, |k| k.clear_bound_keys());
        (self.update_listener)();
        // java: super.onPress(input);
        (self.on_press)();
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    pub fn mouse_clicked(&mut self, click: &MouseButtonEvent, doubled: bool) -> Option<bool> {
        if self.editing {
            compat::write(&self.key_binding, |k| k.add_bound_key(input_constants::Type::Mouse.get_or_create(click.button())));
            (self.update_listener)();
            return Some(true);
        }
        // java: return super.mouseClicked(click, doubled); - the base body (`None`), which reaches `onClick` -> `onPress`
        let _ = doubled;
        return None;
    }

    /// java: @Override public boolean keyPressed(KeyEvent input)
    pub fn key_pressed(&mut self, input: &KeyEvent) -> Option<bool> {
        if self.editing {
            if input.is_escape() {
                // This should never happen because ESC is handled in ShortcutsConfigScreen#keyPressed
                compat::write(&self.key_binding, |k| k.add_bound_key(input_constants::unknown()));
            } else {
                // java: InputConstants.getKey(input)
                compat::write(&self.key_binding, |k| k.add_bound_key(input_constants::get_key_from_event(input.key)));
            }
            (self.update_listener)();
            return Some(true);
        }
        // java: return super.keyPressed(input); - `AbstractButton.keyPressed`, the base body (`None`)
        return None;
    }
}

/// KeybindWidget's widget half (`super`'s fields: x, y, size, message, ...), lent by the game for each call
impl compat::WidgetHalf for KeybindWidget {
    fn widget_half(&mut self) -> &mut AbstractWidget {
        &mut self.base
    }
}

/// KeybindWidget as the game's widget: `KeybindWidget::widget(handle)` where Java hands it on
impl ModWidget for compat::WidgetAdapter<KeybindWidget> {
    /// java: @Override protected void extractContents(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    /// - `extractDefaultSprite(graphics)` + `extractDefaultLabel(..)`
    fn extract_widget_render_state(&mut self, widget: &AbstractWidget, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, _a: f32) {
        self.lend_ref(widget, |__h| compat::write(__h, |t| t.look.draw(widget, context, mouse_x, mouse_y)))
    }

    /// `Button.updateWidgetNarration` - `defaultButtonNarrationText`: `output.add(TITLE, createNarrationMessage())`, then
    /// the usage hint of an active button; `Button.createNarrationMessage` is
    /// `createNarration.createNarrationMessage(() -> super.createNarrationMessage())`
    fn update_widget_narration(&mut self, widget: &AbstractWidget, output: &mut dyn NarrationElementOutput) {
        let Some(supplier) = self.call(|__h| compat::read(__h, |t| t.narration_supplier.clone())) else {
            widget.default_button_narration_text(output);
            return;
        };
        use mcrust::net::minecraft::client::gui::narration::narrated_element_type::NarratedElementType;
        let title = supplier(&|| AbstractWidget::wrap_default_narration_message(widget.get_message_component()));
        output.add_component(NarratedElementType::Title, title);
        if widget.active {
            let key = if widget.is_focused() { "narration.button.usage.focused" } else { "narration.button.usage.hovered" };
            output.add_component(NarratedElementType::Usage, mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(mcrust::net::minecraft::network::chat::component::translatable(key)));
        }
    }

    /// `AbstractButton.onClick` -> `onPress(input)`
    fn on_click(&mut self, widget: &mut AbstractWidget, _event: &MouseButtonEvent, _double_click: bool) {
        self.lend(widget, |__h| compat::write(__h, |t| t.on_press()))
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    fn mouse_clicked(&mut self, widget: &mut AbstractWidget, event: &MouseButtonEvent, double_click: bool) -> bool {
        match self.lend(widget, |__h| compat::write(__h, |t| t.mouse_clicked(event, double_click))) {
            Some(answer) => answer,
            // super.mouseClicked: `AbstractWidget.mouseClicked`'s base body
            None => {
                if !widget.is_active() {
                    return false;
                }
                if self.is_valid_click_button(event.button_info()) && self.is_mouse_over(widget, event.x(), event.y()) {
                    self.on_click(widget, event, double_click);
                    true
                } else {
                    false
                }
            }
        }
    }

    /// java: @Override public boolean keyPressed(KeyEvent input)
    fn key_pressed(&mut self, widget: &mut AbstractWidget, event: &KeyEvent) -> bool {
        match self.lend(widget, |__h| compat::write(__h, |t| t.key_pressed(event))) {
            Some(answer) => answer,
            // super.keyPressed: `AbstractButton.keyPressed:63-72` - an active button presses on Enter / Space / keypad Enter
            None => {
                if mcrust::net::minecraft::client::gui::components::abstract_button::key_activates(widget.active, event.is_selection()) {
                    // `:68`'s `this.playDownSound(Minecraft.getInstance().getSoundManager())`
                    compat::batch_w_play_down_sound();
                    self.lend(widget, |__h| compat::write(__h, |t| t.on_press()));
                    true
                } else {
                    false
                }
            }
        }
    }

    /// `AbstractWidget.handleCursor` - a button's pointer
    fn handle_cursor(&self, widget: &AbstractWidget, cursors: &mcrust::com::mojang::blaze3d::platform::cursor::cursor_types::CursorTypes) -> Option<mcrust::com::mojang::blaze3d::platform::cursor::cursor_type::CursorType> {
        widget.handle_cursor(cursors)
    }
}

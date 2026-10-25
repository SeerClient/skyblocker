//! Ported from de.hysky.skyblocker.utils.render.gui.ItemButtonWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::mcrust::modding::widgets::ModWidget;
use mcrust::mcrust::modding::widgets::WidgetRenderContext;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress as GameOnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::WidgetKind;
use mcrust::net::minecraft::client::gui::components::button::CreateNarration;
use mcrust::net::minecraft::client::gui::components::tooltip::Tooltip;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::input::key_event::KeyEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::network::chat::formatted_text::FormattedText;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: `Button.OnPress` - `void onPress(Button button)`, handed the button that was pressed.
pub type OnPress = Arc<dyn Fn(&mut ItemButtonWidget) + Send + Sync>;

/// java: public class ItemButtonWidget extends Button
pub struct ItemButtonWidget {
    /// java: extends net.minecraft.client.gui.components.Button (x, y, size, message, tooltip, narration)
    pub base: AbstractWidget,
    /// java: Button.onPress (`protected final OnPress onPress`)
    pub on_press: OnPress,
    /// java: private final ItemStack item;
    pub item: mcrust::net::minecraft::world::item::item_stack::ItemStack,
    /// `AbstractButton.extractDefaultSprite` - the game's button sprite pass (see `compat::ButtonLook`)
    pub look: compat::ButtonLook,
}

impl std::ops::Deref for ItemButtonWidget {
    type Target = AbstractWidget;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for ItemButtonWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl ItemButtonWidget {
    /// java: public ItemButtonWidget(int x, int y, int width, int height, ItemStack item, Component message, OnPress onPress, CreateNarration narrationSupplier)
    pub fn new(x: i32, y: i32, width: i32, height: i32, mut item: mcrust::net::minecraft::world::item::item_stack::ItemStack, message: MutableComponent, on_press: OnPress, narration_supplier: CreateNarration) -> Self {
        // java: super(x, y, width, height, message, onPress, narrationSupplier);
        let mut base = AbstractWidget::new(x, y, width, height, message.get_string(), WidgetKind::Button, GameOnPress::Inert);
        base.message_text = Some(FormattedText::from(message.clone()));
        base.create_narration = narration_supplier;
        let mut this = Self {
            base,
            on_press,
            // java: this.item = item.copy();
            item: item.copy(),
            look: compat::ButtonLook::new(),
        };
        // java: setTooltip(Tooltip.create(message));
        this.base.set_tooltip(Tooltip::create(message.get_string()));
        this.base.set_tooltip_text(FormattedText::from(message));
        // java: setTooltipDelay(Duration.ofMillis(250));
        this.base.set_tooltip_delay(250);
        this
    }

    /// java: public ItemButtonWidget(int x, int y, int width, int height, ItemStack item, Component message, OnPress onPress)
    pub fn new_7(x: i32, y: i32, width: i32, height: i32, item: mcrust::net::minecraft::world::item::item_stack::ItemStack, message: MutableComponent, on_press: OnPress) -> Self {
        // java: this(x, y, width, height, item, message, onPress, DEFAULT_NARRATION);
        Self::new(x, y, width, height, item, message, on_press, CreateNarration::Default)
    }

    /// java: public ItemButtonWidget(int x, int y, ItemStack item, Component message, OnPress onPress)
    pub fn new_5(x: i32, y: i32, item: mcrust::net::minecraft::world::item::item_stack::ItemStack, message: MutableComponent, on_press: OnPress) -> Self {
        // java: this(x, y, 20, 20, item, message, onPress, DEFAULT_NARRATION);
        Self::new(x, y, 20, 20, item, message, on_press, CreateNarration::Default)
    }

    /// java: @Override protected void extractContents(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float delta)
    ///
    /// (`widget`: the game's widget the sprite is drawn over - `self.base` when the button is a child the mod draws)
    pub fn extract_contents(&mut self, widget: &AbstractWidget, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, _delta: f32) {
        // java: this.extractDefaultSprite(graphics);
        self.look.draw_sprite(widget, context, mouse_x, mouse_y);
        // java: graphics.item(this.item, getX() + getWidth() / 2 - 8, getY() + getHeight() / 2 - 8);
        context.graphics.item(&self.item, widget.get_x().wrapping_add(widget.get_width().wrapping_div(2)).wrapping_sub(8), widget.get_y().wrapping_add(widget.get_height().wrapping_div(2)).wrapping_sub(8));
    }

    /// java: `AbstractWidget.extractRenderState(graphics, mouseX, mouseY, a)` called by the object that holds this
    /// button as a child (`GardenPlotsWidget`): `:56-62` - if visible, `isHovered` from the mouse, then
    /// `extractWidgetRenderState` (`AbstractButton`: `extractContents`), then the tooltip holder's
    /// `refreshTooltipForNextRenderPass` (handed to the frame through `compat::batch_s_set_tooltip_for_next_frame`).
    pub fn extract_render_state(&mut self, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        if !self.base.visible {
            return;
        }
        self.base.set_hovered_from_extract(mouse_x, mouse_y);
        let widget = self.base.clone();
        self.extract_contents(&widget, context, mouse_x, mouse_y, a);
        let hovered = self.base.is_hovered();
        let focused = self.base.is_focused();
        if self.base.tooltip_holder.refresh_with(compat::millis(), self.base.tooltip.is_some(), hovered, focused, false) {
            if let Some(tooltip) = self.base.tooltip.clone() {
                compat::batch_s_set_tooltip_for_next_frame(vec![tooltip]);
            }
        }
    }

    /// java: `Button.onPress(InputWithModifiers input)` - `this.onPress.onPress(this)`
    pub fn on_press(&mut self) {
        let on_press = self.on_press.clone();
        on_press(self);
    }

    /// java: `AbstractWidget.mouseClicked:109-125` called by the holder's children pass: an active button under the
    /// mouse plays `playDownSound` and runs `onClick` (`AbstractButton.onClick:56-59`: `onPress`).
    pub fn mouse_clicked(&mut self, click: &MouseButtonEvent, _doubled: bool) -> bool {
        if !self.base.is_active() {
            return false;
        }
        if self.base.is_valid_click_button(click.button()) && self.base.is_mouse_over(click.x(), click.y()) {
            compat::batch_w_play_down_sound();
            self.on_press();
            return true;
        }
        false
    }

    /// java: `AbstractButton.keyPressed:61-72` - a selection key presses an active button
    pub fn key_pressed(&mut self, event: &KeyEvent) -> bool {
        if !self.base.is_active() || !event.is_selection() {
            return false;
        }
        compat::batch_w_play_down_sound();
        self.on_press();
        true
    }
}

/// ItemButtonWidget's widget half (`super`'s fields: x, y, size, message, ...), lent by the game for each call
impl compat::WidgetHalf for ItemButtonWidget {
    fn widget_half(&mut self) -> &mut AbstractWidget {
        &mut self.base
    }
}

/// ItemButtonWidget as the game's widget: `compat::WidgetAdapter::widget(handle)` where Java hands it on
impl ModWidget for compat::WidgetAdapter<ItemButtonWidget> {
    /// java: AbstractButton.extractWidgetRenderState - `extractContents(..)`, then `handleCursor(graphics)`
    fn extract_widget_render_state(&mut self, widget: &AbstractWidget, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.lend_ref(widget, |__h| __h.lock().borrow_mut().extract_contents(widget, context, mouse_x, mouse_y, a))
    }

    /// java: AbstractButton.extractWidgetRenderState's `handleCursor(graphics)` - `AbstractWidget.handleCursor:68-72`
    fn handle_cursor(&self, widget: &AbstractWidget, cursors: &mcrust::com::mojang::blaze3d::platform::cursor::cursor_types::CursorTypes) -> Option<mcrust::com::mojang::blaze3d::platform::cursor::cursor_type::CursorType> {
        widget.handle_cursor(cursors)
    }

    /// java: Button.updateWidgetNarration - `this.defaultButtonNarrationText(output)`
    fn update_widget_narration(&mut self, widget: &AbstractWidget, output: &mut dyn NarrationElementOutput) {
        widget.default_button_narration_text(output)
    }

    /// java: AbstractButton.onClick - `this.onPress(event)`
    fn on_click(&mut self, widget: &mut AbstractWidget, _event: &MouseButtonEvent, _double_click: bool) {
        self.lend(widget, |__h| __h.lock().borrow_mut().on_press())
    }

    /// java: AbstractButton.keyPressed - a selection key presses an active button
    fn key_pressed(&mut self, widget: &mut AbstractWidget, event: &KeyEvent) -> bool {
        if !widget.is_active() || !event.is_selection() {
            return false;
        }
        self.lend(widget, |__h| __h.lock().borrow_mut().on_press());
        true
    }

    /// java: `AbstractWidget.playDownSound:166-168` - `UI_BUTTON_CLICK`
    fn play_down_sound(&self) -> Option<&'static mcrust::net::minecraft::sounds::sound_event::SoundEvent> {
        Some(mcrust::net::minecraft::sounds::sound_events::ui_button_click())
    }
}

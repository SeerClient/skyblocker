//! Ported from de.hysky.skyblocker.utils.render.gui.RangedSliderWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::formatters;
use mcrust::com::mojang::blaze3d::platform::cursor::cursor_type::CursorType;
use mcrust::com::mojang::blaze3d::platform::cursor::cursor_types::CursorTypes;
use mcrust::java::text::decimal_format::DecimalFormat;
use mcrust::mcrust::modding::widgets::WidgetRenderContext;
use mcrust::net::minecraft::client::gui::components::abstract_container_widget::AbstractContainerWidget;
use mcrust::net::minecraft::client::gui::components::abstract_slider_button;
use mcrust::net::minecraft::client::gui::components::abstract_slider_button::SliderState;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::input::key_event::KeyEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::formatted_text::FormattedText;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: public class RangedSliderWidget extends AbstractSliderButton
///
/// A mod `AbstractSliderButton` subclass: the object keeps `AbstractWidget`'s box (`base`), `AbstractSliderButton`'s
/// `value` / `canChangeValue` / `dragging` (`state`) and its message, and is handed to the game as
/// `compat::object_widget` (`RangedSliderWidget::widget`); it is drawn with the game's own slider look.
pub struct RangedSliderWidget {
    /// java: extends AbstractSliderButton - AbstractWidget's fields (x, y, width, height, active, visible)
    pub base: AbstractContainerWidget,
    /// AbstractSliderButton's `value`, `canChangeValue`, `dragging`
    pub state: SliderState,
    /// AbstractWidget's `message`
    pub message: FormattedText,
    /// java: private final double min;
    pub min: f64,
    /// java: private final double max;
    pub max: f64,
    /// java: private final Double2ObjectFunction<Component> formatter;
    pub formatter: Arc<dyn Fn(f64) -> MutableComponent>,
    /// java: private final DoubleConsumer callback;
    pub callback: Arc<dyn Fn(f64)>,
    /// java: private final double step;
    pub step: f64,
    /// the game's slider look (`AbstractSliderButton.extractWidgetRenderState`)
    look: compat::SliderLook,
}

impl RangedSliderWidget {
    /// java: public RangedSliderWidget(int x, int y, int width, int height, double defaultValue, double min, double max, double step, Double2ObjectFunction<Component> formatter, DoubleConsumer callback)
    pub fn new(x: i32, y: i32, width: i32, height: i32, default_value: f64, min: f64, max: f64, step: f64, formatter: Arc<dyn Fn(f64) -> MutableComponent>, callback: Arc<dyn Fn(f64)>) -> compat::Shared<Self> {
        // java: super(x, y, width, height, Component.empty(), 0);
        let mut this = Self {
            base: AbstractContainerWidget::new(x, y, width, height, FormattedText::Empty),
            state: SliderState::new(0.0),
            message: FormattedText::Empty,
            min,
            max,
            formatter,
            callback,
            step,
            look: compat::SliderLook::new(),
        };
        // java: setValue(valueToProgress(defaultValue));
        let progress = this.value_to_progress(default_value);
        this.set_value(progress);
        compat::shared(this)
    }

    /// `addRenderableWidget(slider)` / a layout's `addChild(slider)`: the game's widget over this object.
    pub fn widget(this: compat::Shared<Self>) -> AbstractWidget {
        let message: FormattedText = compat::read(&this, |__t| __t.message.clone());
        let mut widget = compat::object_widget(this, message.get_string());
        widget.set_message_text(message);
        widget
    }

    /// java: public static Builder builder()
    pub fn builder() -> Builder {
        return Builder::new();
    }

    /// java: private double roundToNearestStep(double value)
    pub fn round_to_nearest_step(&self, value: f64) -> f64 {
        return (if self.step <= (0 as f64) { value } else { ((compat::java_round_f64((value / self.step)) as f64) * self.step) });
    }

    /// java: private double progressToValue(double progress)
    pub fn progress_to_value(&self, progress: f64) -> f64 {
        return ((progress * (self.max - self.min)) + self.min);
    }

    /// java: private double valueToProgress(double value)
    pub fn value_to_progress(&self, value: f64) -> f64 {
        return ((value - self.min) / (self.max - self.min)).clamp((0 as f64), (1 as f64));
    }

    /// java: @Override protected void updateMessage()
    pub fn update_message(&mut self) {
        // java: setMessage(this.formatter.apply(getValue()));
        let __message = (self.formatter)(self.get_value());
        self.message = FormattedText::from(__message);
    }

    /// java: public double getValue()
    pub fn get_value(&self) -> f64 {
        return self.round_to_nearest_step(self.progress_to_value(self.state.value));
    }

    /// java: @Override protected void applyValue()
    pub fn apply_value(&mut self) {
        // java: callback.accept(getValue());
        (self.callback)(self.get_value());
    }

    /// AbstractSliderButton.setValue(double):132-140 - clamped; `applyValue()` when it changed; `updateMessage()`.
    pub fn set_value(&mut self, new_value: f64) {
        if self.state.set_value(new_value) {
            self.apply_value();
        }
        self.update_message();
    }

    /// AbstractSliderButton.setValueFromMouse:128-130.
    fn set_value_from_mouse(&mut self, event: &MouseButtonEvent) {
        let value = abstract_slider_button::value_from_mouse(event.x(), self.base.get_x(), self.base.get_width());
        self.set_value(value);
    }
}

impl compat::ObjectWidget for RangedSliderWidget {
    fn object_box(&mut self) -> &mut AbstractContainerWidget {
        &mut self.base
    }

    fn object_box_ref(&self) -> &AbstractContainerWidget {
        &self.base
    }

    /// AbstractSliderButton.extractWidgetRenderState - the track, the handle and the message
    fn extract_widget_render_state(&mut self, widget: &AbstractWidget, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, _a: f32) {
        self.look.draw(widget, &self.message, &self.state, context, mouse_x, mouse_y);
    }

    /// AbstractSliderButton.updateWidgetNarration:49-63
    fn update_widget_narration(&mut self, widget: &AbstractWidget, output: &mut dyn NarrationElementOutput) {
        abstract_slider_button::update_widget_narration(output, &self.message.get_string(), self.base.is_active(), widget.is_focused(), self.state.can_change_value);
    }

    /// AbstractSliderButton.onClick:89-93 - `dragging = active; setValueFromMouse(event)`
    fn on_click(&mut self, _widget: &mut AbstractWidget, event: &MouseButtonEvent, _double_click: bool) {
        self.state.dragging = self.base.is_active();
        self.set_value_from_mouse(event);
    }

    /// AbstractSliderButton.onDrag:143-146 - `setValueFromMouse(event)`
    fn on_drag(&mut self, _widget: &mut AbstractWidget, event: &MouseButtonEvent, _dx: f64, _dy: f64) {
        self.set_value_from_mouse(event);
    }

    /// java: @Override public void onRelease(MouseButtonEvent click)
    fn on_release(&mut self, _widget: &mut AbstractWidget, _event: &MouseButtonEvent) {
        // java: super.onRelease(click); - `dragging = false; super.playDownSound(..)`
        // rust: port gap (GAPS_batchH #1) - the release click sound is `SoundManager.play`, which a mod cannot reach
        self.state.on_release();
        // java: this.value = valueToProgress(getValue());
        self.state.value = self.value_to_progress(self.get_value());
    }

    /// AbstractSliderButton.keyPressed:109-126
    fn key_pressed(&mut self, _widget: &mut AbstractWidget, event: &KeyEvent) -> bool {
        if event.is_selection() {
            self.state.toggle_armed();
            return true;
        }
        if self.state.can_change_value && (event.is_left() || event.is_right()) {
            let value = self.state.value + abstract_slider_button::key_step(self.base.get_width(), event.is_left());
            self.set_value(value);
            return true;
        }
        false
    }

    /// AbstractSliderButton.setFocused:95-107 - `canChangeValue` follows the input that focused it
    fn set_focused(&mut self, widget: &mut AbstractWidget, focused: bool) {
        let last_input_type = compat::minecraft().last_input_type;
        widget.set_focused(focused, last_input_type);
        self.state.set_focused(focused, last_input_type);
    }

    /// AbstractSliderButton.playDownSound - empty (the click is the release's)
    fn play_down_sound(&self) -> Option<&'static mcrust::net::minecraft::sounds::sound_event::SoundEvent> {
        None
    }

    /// AbstractSliderButton.handleCursor - RESIZE_EW while dragging
    fn handle_cursor(&self, widget: &AbstractWidget, cursors: &CursorTypes) -> Option<CursorType> {
        abstract_slider_button::cursor_for(cursors, widget.is_hovered(), self.base.is_active(), self.state.dragging)
    }
}

/// java: public static class Builder
///
/// Each setter is Java's `return this;` - the builder is handed on, so the chain keeps writing the one object.
pub struct Builder {
    /// java: private int x, y, width = Button.DEFAULT_WIDTH, height = Button.DEFAULT_HEIGHT;
    pub x: i32,
    /// java: private int x, y, width = Button.DEFAULT_WIDTH, height = Button.DEFAULT_HEIGHT;
    pub y: i32,
    /// java: private int x, y, width = Button.DEFAULT_WIDTH, height = Button.DEFAULT_HEIGHT;
    pub width: i32,
    /// java: private int x, y, width = Button.DEFAULT_WIDTH, height = Button.DEFAULT_HEIGHT;
    pub height: i32,
    /// java: private double value, min = 0, max = 1, step;
    pub value: f64,
    /// java: private double value, min = 0, max = 1, step;
    pub min: f64,
    /// java: private double value, min = 0, max = 1, step;
    pub max: f64,
    /// java: private double value, min = 0, max = 1, step;
    pub step: f64,
    /// java: private Double2ObjectFunction<Component> formatter = d -> Component.literal(Formatters.DOUBLE_NUMBERS.format(d));
    pub formatter: Arc<dyn Fn(f64) -> MutableComponent>,
    /// java: private DoubleConsumer callback = _ -> {};
    pub callback: Arc<dyn Fn(f64)>,
}

impl Builder {
    /// java: private Builder()
    pub fn new() -> Self {
        let mut this = Self {
            x: 0,
            y: 0,
            width: mcrust::net::minecraft::client::gui::components::button::DEFAULT_WIDTH,
            height: mcrust::net::minecraft::client::gui::components::button::DEFAULT_HEIGHT,
            value: 0.0,
            min: ((0) as f64) as f64,
            max: ((1) as f64) as f64,
            step: 0.0,
            formatter: Arc::new(move |d: f64| component::literal(&formatters::DOUBLE_NUMBERS.get_mut().format(d))),
            callback: Arc::new(move |_: f64| {

        }),
        };
        this
    }

    /// java: public Builder position(int x, int y)
    pub fn position(mut self, mut x: i32, mut y: i32) -> Builder {
        self.x = x;
        self.y = y;
        return self;
    }

    /// java: public Builder width(int width)
    pub fn width(mut self, mut width: i32) -> Builder {
        self.width = width;
        return self;
    }

    /// java: public Builder height(int height)
    pub fn height(mut self, mut height: i32) -> Builder {
        self.height = height;
        return self;
    }

    /// java: public Builder minMax(double min, double max)
    pub fn min_max(mut self, mut min: f64, mut max: f64) -> Builder {
        self.min = min;
        self.max = max;
        return self;
    }

    /// java: public Builder step(double step)
    pub fn step(mut self, mut step: f64) -> Builder {
        self.step = step;
        return self;
    }

    /// java: public Builder defaultValue(double value)
    pub fn default_value(mut self, mut value: f64) -> Builder {
        self.value = value;
        return self;
    }

    /// java: public Builder optionFormatter(Component optionName, NumberFormat formatter)
    pub fn option_formatter(self, option_name: &MutableComponent, formatter: &DecimalFormat) -> Builder {
        // java: return optionFormatter(optionName, d -> Component.literal(formatter.format(d)));
        // (the lambda keeps the format object; `DecimalFormat.format` reads only its pattern, so the lambda holds its
        // own copy of that pattern)
        let formatter: DecimalFormat = formatter.clone();
        return self.option_formatter_component_double2_object_function(option_name, &(Arc::new(move |d: f64| component::literal(&formatter.format(d))) as Arc<dyn Fn(f64) -> MutableComponent>));
    }

    /// java: public Builder optionFormatter(Component optionName, Double2ObjectFunction<Component> formatter)
    pub fn option_formatter_component_double2_object_function(self, option_name: &MutableComponent, formatter: &Arc<dyn Fn(f64) -> MutableComponent>) -> Builder {
        // java: return formatter(d -> CommonComponents.optionNameValue(optionName, formatter.apply(d)));
        // (`optionNameValue(Component, Component)` is `Component.translatable("options.generic_value", caption, value)`)
        let option_name: MutableComponent = option_name.clone();
        let formatter: Arc<dyn Fn(f64) -> MutableComponent> = formatter.clone();
        return self.formatter(Arc::new(move |d: f64| component::translatable_args("options.generic_value", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(option_name.clone()), mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from((formatter)(d))])));
    }

    /// java: public Builder formatter(Double2ObjectFunction<Component> formatter)
    pub fn formatter(mut self, formatter: Arc<dyn Fn(f64) -> MutableComponent>) -> Builder {
        self.formatter = formatter;
        return self;
    }

    /// java: public Builder callback(DoubleConsumer callback)
    pub fn callback(mut self, callback: Arc<dyn Fn(f64)>) -> Builder {
        self.callback = callback;
        return self;
    }

    /// java: public RangedSliderWidget build()
    pub fn build(&self) -> compat::Shared<RangedSliderWidget> {
        return RangedSliderWidget::new(self.x, self.y, self.width, self.height, self.value, self.min, self.max, self.step, self.formatter.clone(), self.callback.clone());
    }

}

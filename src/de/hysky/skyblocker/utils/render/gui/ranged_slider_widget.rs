//! Ported from de.hysky.skyblocker.utils.render.gui.RangedSliderWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::formatters;
use mcrust::java::text::decimal_format::DecimalFormat;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: public class RangedSliderWidget extends AbstractSliderButton
pub struct RangedSliderWidget {
    /// java: extends net.minecraft.client.gui.components.AbstractSliderButton
    pub base: AbstractWidget,
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
}

impl std::ops::Deref for RangedSliderWidget {
    type Target = AbstractWidget;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for RangedSliderWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl RangedSliderWidget {
    /// java: public RangedSliderWidget(int x, int y, int width, int height, double defaultValue, double min, double max, double step, Double2ObjectFunction<Component> formatter, DoubleConsumer callback)
    pub fn new(mut x: i32, mut y: i32, mut width: i32, mut height: i32, mut default_value: f64, mut min: f64, mut max: f64, mut step: f64, mut formatter: Arc<dyn Fn(f64) -> MutableComponent>, mut callback: Arc<dyn Fn(f64)>) -> Self {
        let __init_min = min;
        let __init_max = max;
        let __init_step = step;
        let __init_formatter = formatter;
        let __init_callback = callback;
        // TODO(fabric2seer): `new` takes 1 argument(s) in the port (`initial_value: f64`), Java passes 6
        //   java: super(x, y, width, height, Component.empty(), 0);
        let mut this: Self = todo!("constructor");
        // TODO(fabric2seer): `SliderState::set_value` ports `setValue`, but the port's `AbstractWidget` holds no `SliderState` (no Deref, no `base` field) to call it on
        // rust: this.set_value(this.value_to_progress(default_value));
        this
    }

    /// java: public static Builder builder()
    pub fn builder() -> Builder {
        return Builder::new();
    }

    /// java: private double roundToNearestStep(double value)
    pub fn round_to_nearest_step(&self, mut value: f64) -> f64 {
        return (if self.step <= (0 as f64) { value } else { ((compat::java_round_f64((value / self.step)) as f64) * self.step) });
    }

    /// java: private double progressToValue(double progress)
    pub fn progress_to_value(&self, mut progress: f64) -> f64 {
        return ((progress * (self.max - self.min)) + self.min);
    }

    /// java: private double valueToProgress(double value)
    pub fn value_to_progress(&self, mut value: f64) -> f64 {
        return ((value - self.min) / (self.max - self.min)).clamp((0 as f64), (1 as f64));
    }

    /// java: @Override protected void updateMessage()
    pub fn update_message(&mut self) {
        // java: setMessage(this.formatter.apply(getValue()));
        let __message = (self.formatter)(self.get_value());
        self.base.set_message_text(mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(__message));
    }

    /// java: public double getValue()
    pub fn get_value(&self) -> f64 {
        // TODO(fabric2seer): `AbstractSliderButton.value` has no Rust field in the port (searched net.minecraft.client.gui.components.AbstractSliderButton and its supertypes)
        // rust: return self.round_to_nearest_step(self.progress_to_value(self.value));
        return todo!("fabric2seer: see the TODO above");
    }

    /// java: @Override protected void applyValue()
    pub fn apply_value(&mut self) {
        // java: callback.accept(getValue());
        (self.callback)(self.get_value());
    }

    /// java: @Override public void onRelease(MouseButtonEvent click)
    pub fn on_release(&self, click: &MouseButtonEvent) {
        // TODO(fabric2seer): `on_release` takes 0 argument(s) in the port (``), Java passes 1
        // TODO(fabric2seer): `SliderState::on_release` ports `onRelease`, but the port's `AbstractWidget` holds no `SliderState` (no Deref, no `base` field) to call it on
        // rust: self.base.on_release(click.clone());
        // TODO(fabric2seer): `AbstractSliderButton.value` has no Rust field in the port (searched net.minecraft.client.gui.components.AbstractSliderButton and its supertypes)
        // rust: self.value = self.value_to_progress(self.get_value());
    }

}

// NOTE(fabric2seer): applyValue, onRelease, updateMessage override(s) of AbstractSliderButton - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them

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
    pub fn build(&self) -> RangedSliderWidget {
        return RangedSliderWidget::new(self.x, self.y, self.width, self.height, self.value, self.min, self.max, self.step, self.formatter.clone(), self.callback.clone());
    }

}

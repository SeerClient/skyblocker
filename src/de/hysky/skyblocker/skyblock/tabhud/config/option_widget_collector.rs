//! Ported from de.hysky.skyblocker.skyblock.tabhud.config.OptionWidgetCollector by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::formatters;
use crate::de::hysky::skyblocker::utils::location::Location;
use crate::de::hysky::skyblocker::utils::render::gui::ranged_slider_widget::RangedSliderWidget;
use mcrust::java::text::decimal_format::DecimalFormat;
use mcrust::mcrust::modding::widgets::ModWidget;
use mcrust::mcrust::modding::widgets::WidgetRenderContext;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::WidgetKind;
use mcrust::net::minecraft::client::gui::components::cycle_button;
use mcrust::net::minecraft::client::gui::components::cycle_button::Builder;
use mcrust::net::minecraft::client::gui::components::cycle_button::CycleButton;
use mcrust::net::minecraft::client::gui::components::tooltip::Tooltip;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::client::input::mouse_button_info::MouseButtonInfo;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: public record OptionWidgetCollector(List<AbstractWidget> collectorList, Runnable onOptionChange, Screen configScreen, Location editingFor)
pub struct OptionWidgetCollector {
    /// java: List<AbstractWidget> collectorList
    pub collector_list: Vec<AbstractWidget>,
    /// java: Runnable onOptionChange
    pub on_option_change: Arc<dyn Fn()>,
    /// java: Screen configScreen
    pub config_screen: mcrust::net::minecraft::client::gui::screens::screen::Screen,
    /// java: Location editingFor
    pub editing_for: Location,
}

impl OptionWidgetCollector {
    /// java: the canonical record constructor
    pub fn new(mut collector_list: Vec<AbstractWidget>, mut on_option_change: Arc<dyn Fn()>, mut config_screen: mcrust::net::minecraft::client::gui::screens::screen::Screen, mut editing_for: Location) -> Self {
        Self { collector_list, on_option_change, config_screen, editing_for }
    }

    /// java: the record accessor `collectorList()`
    pub fn collector_list(&self) -> Vec<AbstractWidget> {
        self.collector_list.clone()
    }

    /// java: the record accessor `onOptionChange()`
    pub fn on_option_change(&self) -> Arc<dyn Fn()> {
        self.on_option_change.clone()
    }

    /// java: the record accessor `configScreen()`
    pub fn config_screen(&self) -> mcrust::net::minecraft::client::gui::screens::screen::Screen {
        self.config_screen.clone()
    }

    /// java: the record accessor `editingFor()`
    pub fn editing_for(&self) -> Location {
        self.editing_for
    }

    /// java: public <T extends AbstractWidget> T addWidget(T widget)
    pub fn add_widget<T: Clone + 'static>(&mut self, mut widget: T) -> T {
        self.collector_list.push(widget.clone());
        return widget;
    }

    /// java: public void buildDeferred()
    pub fn build_deferred(&mut self) {
        // TODO(fabric2seer): instanceof with a pattern binding outside an `if` condition
        //   java: collectorList.replaceAll(widget -> widget instanceof DeferredWidget deferred ? deferred.get() : widget);
    }

    /// java: public void yesNoButton(Component label, Consumer<Boolean> callback, boolean initialValue, @Nullable Component tooltip)
    pub fn yes_no_button(&mut self, label: &MutableComponent, callback: Arc<dyn Fn(bool)>, mut initial_value: bool, mut tooltip: Option<MutableComponent>) {
        // TODO(fabric2seer): Consumer.andThen
        //   java: Consumer<Boolean> consumer = callback.andThen(_ -> onOptionChange.run());
        let mut consumer: Arc<dyn Fn(bool)> = todo!("fabric2seer: the initialiser of `consumer` (TODO above)");
        let mut builder: Builder = cycle_button::boolean_builder(mcrust::net::minecraft::network::chat::common_components::gui_yes().get_string(), mcrust::net::minecraft::network::chat::common_components::gui_no().get_string(), initial_value);
        if let Some(tooltip) = &tooltip {
            // TODO(fabric2seer): a lambda for a parameter the port does not type as a closure (`net.minecraft.client.OptionInstance.TooltipSupplier<T>`)
            // rust: builder.with_tooltip(|_| Tooltip::create(tooltip.get_string()));
        }
        // TODO(fabric2seer): the port's `create` ports this Java method with other parameters (name, generic_value_format, value_change_listener for Java's x, y, width, height, name, value_change_listener)
        // TODO(fabric2seer): `create` takes 3 argument(s) in the port (`name: String, generic_value_format: String, value_change_listener: OnValueChange`), Java passes 6
        // TODO(fabric2seer): a lambda for a parameter the port does not type as a closure (`net.minecraft.client.gui.components.CycleButton.OnValueChange<T>`)
        // rust: self.add_widget(builder.create(0, 0, 150, 20, label.clone(), |_, value| { (consumer)(value.clone()); }).unwrap());
    }

    /// java: public void yesNoButton(Component label, Consumer<Boolean> callback, boolean initialValue)
    pub fn yes_no_button_3(&mut self, label: &MutableComponent, callback: Arc<dyn Fn(bool)>, mut initial_value: bool) {
        self.yes_no_button(label, callback.clone(), initial_value, None);
    }

    /// java: public <E extends Enum<E>> EnumButtonBuilder<E> enumButton(Class<E> enumClass, Component label, Consumer<E> callback, E initialValue)
    pub fn enum_button<E: 'static>(&mut self, enum_class: &crate::unresolved::Class, label: &MutableComponent, callback: Arc<dyn Fn(E)>, initial_value: &E) -> EnumButtonBuilder<E> {
        // TODO(fabric2seer): Consumer.andThen
        //   java: EnumButtonBuilder<E> builder = new EnumButtonBuilder<>(enumClass, label, callback.andThen(_ -> onOptionChange.run()), initialValue);
        let mut builder: EnumButtonBuilder<E> = todo!("fabric2seer: the initialiser of `builder` (TODO above)");
        self.add_widget(DeferredWidget::new(Arc::new(move || builder.build())));
        return builder;
    }

    /// java: public SliderBuilder slider(Component label, DoubleConsumer callback, double initialValue, double min, double max)
    pub fn slider(&mut self, label: &MutableComponent, callback: &crate::unresolved::DoubleConsumer, mut initial_value: f64, mut min: f64, mut max: f64) -> SliderBuilder {
        let mut builder: SliderBuilder = SliderBuilder::new(label.clone(), callback.clone(), initial_value, min, max);
        self.add_widget(DeferredWidget::new(Arc::new(move || builder.build())));
        return builder;
    }

}

/// java: public static class SliderBuilder
pub struct SliderBuilder {
    /// java: private final Component label;
    pub label: MutableComponent,
    /// java: private final DoubleConsumer callback;
    pub callback: crate::unresolved::DoubleConsumer,
    /// java: private final double initialValue, min, max;
    pub initial_value: f64,
    /// java: private final double initialValue, min, max;
    pub min: f64,
    /// java: private final double initialValue, min, max;
    pub max: f64,
    /// java: private double step = 0.1;
    pub step: f64,
    /// java: private NumberFormat format = Formatters.FLOAT_NUMBERS;
    pub format: DecimalFormat,
    /// java: private @Nullable Component tooltip;
    pub tooltip: Option<MutableComponent>,
}

impl SliderBuilder {
    /// java: private SliderBuilder(Component label, DoubleConsumer callback, double initialValue, double min, double max)
    pub fn new(mut label: MutableComponent, mut callback: crate::unresolved::DoubleConsumer, mut initial_value: f64, mut min: f64, mut max: f64) -> Self {
        let __init_label = label.clone();
        let __init_callback = callback.clone();
        let __init_initial_value = initial_value;
        let __init_min = min;
        let __init_max = max;
        let mut this = Self {
            label: __init_label,
            callback: __init_callback,
            initial_value: __init_initial_value,
            min: __init_min,
            max: __init_max,
            step: 0.1,
            format: formatters::FLOAT_NUMBERS.get_mut().clone(),
            tooltip: None,
        };
        this
    }

    /// java: public SliderBuilder step(double step)
    pub fn step(&mut self, mut step: f64) -> SliderBuilder {
        self.step = step;
        return self.clone();
    }

    /// java: public SliderBuilder format(NumberFormat format)
    pub fn format(&mut self, mut format: DecimalFormat) -> SliderBuilder {
        self.format = format.clone();
        return self.clone();
    }

    /// java: public SliderBuilder tooltip(Component tooltip)
    pub fn tooltip(&mut self, mut tooltip: MutableComponent) -> SliderBuilder {
        self.tooltip = Some(tooltip.clone());
        return self.clone();
    }

    /// java: private RangedSliderWidget build()
    pub fn build(&self) -> RangedSliderWidget {
        let mut slider_widget: RangedSliderWidget = (RangedSliderWidget::builder().min_max(self.min, self.max)).step(self.step).option_formatter(&self.label, &self.format).default_value(self.initial_value).callback(self.callback.clone()).build();
        if self.tooltip.is_some() {
            slider_widget.set_tooltip(Tooltip::create(self.tooltip.as_ref().unwrap().get_string()));
        }
        return slider_widget;
    }

}

/// java: public static class EnumButtonBuilder<E extends Enum<E>>
pub struct EnumButtonBuilder<E> {
    /// java: private final Class<E> enumClass;
    pub enum_class: crate::unresolved::Class,
    /// java: private final Component label;
    pub label: MutableComponent,
    /// java: private final Consumer<E> callback;
    pub callback: Arc<dyn Fn(E)>,
    /// java: private final E initialValue;
    pub initial_value: E,
    /// java: private Function<E, Component> display = e -> Component.literal(e.toString());
    pub display: Arc<dyn Fn(E) -> MutableComponent>,
    /// java: private @Nullable Component tooltip;
    pub tooltip: Option<MutableComponent>,
}

impl<E: Clone + 'static> EnumButtonBuilder<E> {
    /// java: private EnumButtonBuilder(Class<E> enumClass, Component label, Consumer<E> callback, E initialValue)
    pub fn new(mut enum_class: crate::unresolved::Class, mut label: MutableComponent, callback: Arc<dyn Fn(E)>, mut initial_value: E) -> Self {
        let __init_enum_class = enum_class.clone();
        let __init_label = label.clone();
        let __init_callback = callback.clone();
        let __init_initial_value = initial_value.clone();
        let mut this = Self {
            enum_class: __init_enum_class,
            label: __init_label,
            callback: __init_callback,
            initial_value: __init_initial_value,
            display: Arc::new(move |e| component::literal(&compat::jstring(&e))),
            tooltip: None,
        };
        this
    }

    /// java: public EnumButtonBuilder<E> display(Function<E, Component> display)
    pub fn display(&mut self, display: Arc<dyn Fn(E) -> MutableComponent>) -> EnumButtonBuilder<E> {
        self.display = display.clone();
        return self.clone();
    }

    /// java: public EnumButtonBuilder<E> tooltip(Component tooltip)
    pub fn tooltip(&mut self, mut tooltip: MutableComponent) -> EnumButtonBuilder<E> {
        self.tooltip = Some(tooltip.clone());
        return self.clone();
    }

    /// java: private CycleButton<E> build()
    pub fn build(&mut self) -> CycleButton {
        // TODO(fabric2seer): the port's `builder` ports this Java method with other parameters (default_value for Java's value_stringifier, default_value)
        // TODO(fabric2seer): `builder` takes 1 argument(s) in the port (`default_value: String`), Java passes 2
        // rust: let mut builder: Builder = cycle_button::builder(self.display.clone(), self.initial_value.clone());
        let mut builder: Builder = todo!("fabric2seer: the initialiser of `builder` (TODO above)");
        let mut tooltip1: Option<MutableComponent> = self.tooltip.clone();
        if let Some(tooltip1) = &tooltip1 {
            // TODO(fabric2seer): a lambda for a parameter the port does not type as a closure (`net.minecraft.client.OptionInstance.TooltipSupplier<T>`)
            // rust: builder.with_tooltip(|_| Tooltip::create(tooltip1.get_string()));
        }
        // TODO(fabric2seer): `.getEnumConstants(..)` on a Class
        //   java: return builder
        //   java: 					.withValues(enumClass.getEnumConstants())
        //   java: 					.create(label, (_, value) -> callback.accept(value));
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}

/// java: private static class DeferredWidget extends AbstractWidget implements Supplier<AbstractWidget>
#[derive(Clone)]
pub struct DeferredWidget {
    /// java: extends net.minecraft.client.gui.components.AbstractWidget
    pub base: AbstractWidget,
    /// java: private final Supplier<AbstractWidget> supplier;
    pub supplier: Arc<dyn Fn() -> AbstractWidget>,
}

impl std::ops::Deref for DeferredWidget {
    type Target = AbstractWidget;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for DeferredWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl DeferredWidget {
    /// java: private DeferredWidget(Supplier<AbstractWidget> supplier)
    pub fn new(supplier: Arc<dyn Fn() -> AbstractWidget>) -> compat::Shared<Self> {
        let __init_supplier = supplier.clone();
        let __this = compat::shared(Self {
            base: AbstractWidget::new(0, 0, 0, 0, (&component::empty().get_string()).to_string(), WidgetKind::PlainText, OnPress::Inert),
            supplier: __init_supplier,
        });
        __this
    }

    /// java: @Override public AbstractWidget get()
    pub fn get(__this: &compat::Shared<Self>) -> AbstractWidget {
        return ((compat::read(__this, |__t| __t.supplier.clone())))();
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_widget_render_state(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        panic!("UnsupportedOperationException: {}", "Be sure to call OptionWidgetCollector#buildDeferred!");
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput output)
    pub fn update_widget_narration(__this: &compat::Shared<Self>, output: &dyn NarrationElementOutput) {
    }

}

/// DeferredWidget's widget half (`super`'s fields: x, y, size, message, ...), lent by the game for each call
impl compat::WidgetHalf for DeferredWidget {
    fn widget_half(&mut self) -> &mut AbstractWidget {
        &mut self.base
    }
}

/// DeferredWidget as the game's widget: `compat::WidgetAdapter::widget(handle)` where Java hands it on
impl ModWidget for compat::WidgetAdapter<DeferredWidget> {
    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_widget_render_state(&mut self, widget: &AbstractWidget, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.lend_ref(widget, |__h| DeferredWidget::extract_widget_render_state(__h, &context.graphics, mouse_x, mouse_y, a))
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput output)
    fn update_widget_narration(&mut self, widget: &AbstractWidget, output: &mut dyn NarrationElementOutput) {
        self.lend_ref(widget, |__h| DeferredWidget::update_widget_narration(__h, output))
    }

}

// NOTE(fabric2seer): extractWidgetRenderState, get, updateWidgetNarration override(s) of AbstractWidget - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them

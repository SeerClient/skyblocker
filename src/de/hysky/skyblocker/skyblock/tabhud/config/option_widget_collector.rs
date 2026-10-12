//! Ported from de.hysky.skyblocker.skyblock.tabhud.config.OptionWidgetCollector by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::tabhud::config::widgets_configuration_screen::WidgetsConfigurationScreen;
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
use mcrust::net::minecraft::client::gui::components::cycle_button::TooltipSupplier;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::formatted_text::FormattedText;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: public record OptionWidgetCollector(List<AbstractWidget> collectorList, Runnable onOptionChange, Screen configScreen, Location editingFor)
pub struct OptionWidgetCollector {
    /// java: List<AbstractWidget> collectorList
    pub collector_list: Vec<AbstractWidget>,
    /// java: Runnable onOptionChange
    pub on_option_change: Arc<dyn Fn()>,
    /// java: Screen configScreen (the one collector is built by `SidePanelWidget.open`, over its
    /// `WidgetsConfigurationScreen`)
    pub config_screen: compat::Shared<WidgetsConfigurationScreen>,
    /// java: Location editingFor
    pub editing_for: Location,
}

impl OptionWidgetCollector {
    /// java: the canonical record constructor
    pub fn new(collector_list: Vec<AbstractWidget>, on_option_change: Arc<dyn Fn()>, config_screen: compat::Shared<WidgetsConfigurationScreen>, editing_for: Location) -> Self {
        Self { collector_list, on_option_change, config_screen, editing_for }
    }

    /// java: the record accessor `collectorList()` (the list object itself)
    pub fn collector_list(&mut self) -> &mut Vec<AbstractWidget> {
        &mut self.collector_list
    }

    /// java: the record accessor `onOptionChange()`
    pub fn on_option_change(&self) -> Arc<dyn Fn()> {
        self.on_option_change.clone()
    }

    /// java: the record accessor `configScreen()`
    pub fn config_screen(&self) -> compat::Shared<WidgetsConfigurationScreen> {
        self.config_screen.clone()
    }

    /// java: the record accessor `editingFor()`
    pub fn editing_for(&self) -> Location {
        self.editing_for
    }

    /// java: public <T extends AbstractWidget> T addWidget(T widget)
    /// (the list takes the widget; no caller reads the reference Java hands back)
    pub fn add_widget(&mut self, widget: AbstractWidget) {
        self.collector_list.push(widget);
    }

    /// java: public void buildDeferred()
    pub fn build_deferred(&mut self) {
        // java: collectorList.replaceAll(widget -> widget instanceof DeferredWidget deferred ? deferred.get() : widget);
        for widget in self.collector_list.iter_mut() {
            let deferred = widget
                .mod_widget()
                .and_then(|slot| slot.with::<compat::WidgetAdapter<DeferredWidget>, _>(|a| a.object().clone()));
            if let Some(deferred) = deferred {
                *widget = DeferredWidget::get(&deferred);
            }
        }
    }

    /// java: public void yesNoButton(Component label, Consumer<Boolean> callback, boolean initialValue, @Nullable Component tooltip)
    pub fn yes_no_button(&mut self, label: &MutableComponent, callback: Arc<dyn Fn(bool)>, initial_value: bool, tooltip: Option<MutableComponent>) {
        // java: Consumer<Boolean> consumer = callback.andThen(_ -> onOptionChange.run());
        let on_option_change = self.on_option_change.clone();
        let consumer: Arc<dyn Fn(bool)> = Arc::new(move |b| {
            callback(b);
            on_option_change();
        });
        let mut builder: Builder = cycle_button::boolean_builder(mcrust::net::minecraft::network::chat::common_components::gui_yes().get_string(), mcrust::net::minecraft::network::chat::common_components::gui_no().get_string(), initial_value);
        if let Some(tooltip) = &tooltip {
            builder = builder.with_tooltip(TooltipSupplier::Constant(FormattedText::from(tooltip.clone())));
        }
        // java: addWidget(builder.create(label, (_, value) -> consumer.accept(value)));
        // (booleanBuilder's values are [TRUE, FALSE])
        self.add_widget(compat::cycle_button_widget(0, 0, 150, 20, label, builder, move |index| consumer(index == 0)));
    }

    /// java: public void yesNoButton(Component label, Consumer<Boolean> callback, boolean initialValue)
    pub fn yes_no_button_3(&mut self, label: &MutableComponent, callback: Arc<dyn Fn(bool)>, initial_value: bool) {
        self.yes_no_button(label, callback, initial_value, None);
    }

    /// java: public <E extends Enum<E>> EnumButtonBuilder<E> enumButton(Class<E> enumClass, Component label, Consumer<E> callback, E initialValue)
    /// (`enumClass` is handed as its `getEnumConstants()`, the only use Java makes of it)
    pub fn enum_button<E: Clone + std::fmt::Display + 'static>(&mut self, enum_class: Vec<E>, label: &MutableComponent, callback: Arc<dyn Fn(E)>, initial_value: &E) -> EnumButtonBuilder<E> {
        // java: callback.andThen(_ -> onOptionChange.run())
        let on_option_change = self.on_option_change.clone();
        let then: Arc<dyn Fn(E)> = Arc::new(move |e| {
            callback(e);
            on_option_change();
        });
        let builder: EnumButtonBuilder<E> = EnumButtonBuilder::new(enum_class, label.clone(), then, initial_value.clone());
        // java: addWidget(new DeferredWidget(builder::build)); - the method reference keeps the builder OBJECT
        let deferred = builder.clone();
        self.add_widget(compat::WidgetAdapter::widget(DeferredWidget::new(Arc::new(move || deferred.build()))));
        return builder;
    }

    /// java: public SliderBuilder slider(Component label, DoubleConsumer callback, double initialValue, double min, double max)
    pub fn slider(&mut self, label: &MutableComponent, callback: Arc<dyn Fn(f64)>, initial_value: f64, min: f64, max: f64) -> SliderBuilder {
        let builder: SliderBuilder = SliderBuilder::new(label.clone(), callback, initial_value, min, max);
        // java: addWidget(new DeferredWidget(builder::build));
        let deferred = builder.clone();
        self.add_widget(compat::WidgetAdapter::widget(DeferredWidget::new(Arc::new(move || deferred.build()))));
        return builder;
    }

}

/// java: public static class SliderBuilder - a reference to the builder object (Java's `this`: the `DeferredWidget`'s
/// `builder::build` holds the same object, so a setter called after `slider(..)` returns reaches the build)
#[derive(Clone)]
pub struct SliderBuilder {
    pub this: compat::Shared<SliderBuilderFields>,
}

/// The fields of a `SliderBuilder`.
pub struct SliderBuilderFields {
    /// java: private final Component label;
    pub label: MutableComponent,
    /// java: private final DoubleConsumer callback;
    pub callback: Arc<dyn Fn(f64)>,
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
    pub fn new(label: MutableComponent, callback: Arc<dyn Fn(f64)>, initial_value: f64, min: f64, max: f64) -> Self {
        SliderBuilder {
            this: compat::shared(SliderBuilderFields {
                label,
                callback,
                initial_value,
                min,
                max,
                step: 0.1,
                format: formatters::FLOAT_NUMBERS.get_mut().clone(),
                tooltip: None,
            }),
        }
    }

    /// java: public SliderBuilder step(double step)
    pub fn step(&self, step: f64) -> SliderBuilder {
        compat::write(&self.this, |t| t.step = step);
        return self.clone();
    }

    /// java: public SliderBuilder format(NumberFormat format)
    pub fn format(&self, format: DecimalFormat) -> SliderBuilder {
        compat::write(&self.this, |t| t.format = format);
        return self.clone();
    }

    /// java: public SliderBuilder tooltip(Component tooltip)
    pub fn tooltip(&self, tooltip: MutableComponent) -> SliderBuilder {
        compat::write(&self.this, |t| t.tooltip = Some(tooltip));
        return self.clone();
    }

    /// java: private RangedSliderWidget build()
    pub fn build(&self) -> AbstractWidget {
        let (min, max, step, label, format, initial_value, callback, tooltip) = compat::read(&self.this, |t| {
            (t.min, t.max, t.step, t.label.clone(), t.format.clone(), t.initial_value, t.callback.clone(), t.tooltip.clone())
        });
        let mut slider_widget: RangedSliderWidget = RangedSliderWidget::builder().min_max(min, max).step(step).option_formatter(&label, &format).default_value(initial_value).callback(callback).build();
        if let Some(tooltip) = tooltip {
            slider_widget.set_tooltip_text(FormattedText::from(tooltip));
        }
        // rust: port gap - `RangedSliderWidget extends AbstractSliderButton` (utils/, batch A) has no widget adapter, so
        // its widget half is what the list holds (GAPS_batchA #11)
        return slider_widget.base;
    }

}

/// java: public static class EnumButtonBuilder<E extends Enum<E>> - a reference to the builder object (see
/// `SliderBuilder`)
pub struct EnumButtonBuilder<E: 'static> {
    pub this: compat::Shared<EnumButtonBuilderFields<E>>,
}

impl<E: 'static> Clone for EnumButtonBuilder<E> {
    fn clone(&self) -> Self {
        EnumButtonBuilder { this: self.this.clone() }
    }
}

/// The fields of an `EnumButtonBuilder`.
pub struct EnumButtonBuilderFields<E> {
    /// java: private final Class<E> enumClass; (its `getEnumConstants()`)
    pub enum_class: Vec<E>,
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

impl<E: Clone + std::fmt::Display + 'static> EnumButtonBuilder<E> {
    /// java: private EnumButtonBuilder(Class<E> enumClass, Component label, Consumer<E> callback, E initialValue)
    pub fn new(enum_class: Vec<E>, label: MutableComponent, callback: Arc<dyn Fn(E)>, initial_value: E) -> Self {
        EnumButtonBuilder {
            this: compat::shared(EnumButtonBuilderFields {
                enum_class,
                label,
                callback,
                initial_value,
                display: Arc::new(move |e: E| component::literal(&e.to_string())),
                tooltip: None,
            }),
        }
    }

    /// java: public EnumButtonBuilder<E> display(Function<E, Component> display)
    pub fn display(&self, display: Arc<dyn Fn(E) -> MutableComponent>) -> EnumButtonBuilder<E> {
        compat::write(&self.this, |t| t.display = display);
        return self.clone();
    }

    /// java: public EnumButtonBuilder<E> tooltip(Component tooltip)
    pub fn tooltip(&self, tooltip: MutableComponent) -> EnumButtonBuilder<E> {
        compat::write(&self.this, |t| t.tooltip = Some(tooltip));
        return self.clone();
    }

    /// java: private CycleButton<E> build()
    pub fn build(&self) -> AbstractWidget {
        let (enum_constants, label, callback, initial_value, display, tooltip) = compat::read(&self.this, |t| {
            (t.enum_class.clone(), t.label.clone(), t.callback.clone(), t.initial_value.clone(), t.display.clone(), t.tooltip.clone())
        });
        // java: CycleButton.Builder<E> builder = CycleButton.builder(display, initialValue);
        // (the port's cycle button holds each value's display LABEL; the value is mapped back by its index)
        let mut builder: Builder = cycle_button::builder(display(initial_value).get_string());
        let tooltip1: Option<MutableComponent> = tooltip;
        if let Some(tooltip1) = &tooltip1 {
            builder = builder.with_tooltip(TooltipSupplier::Constant(FormattedText::from(tooltip1.clone())));
        }
        // java: return builder.withValues(enumClass.getEnumConstants()).create(label, (_, value) -> callback.accept(value));
        let builder = builder.with_values(enum_constants.iter().map(|e| display(e.clone()).get_string()).collect());
        return compat::cycle_button_widget(0, 0, 150, 20, &label, builder, move |index| callback(enum_constants[index].clone()));
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

//! Ported from de.hysky.skyblocker.skyblock.fancybars.EditBarWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::fancybars::edit_bar_color_popup::EditBarColorPopup;
use crate::de::hysky::skyblocker::skyblock::fancybars::fancy_status_bars::FancyStatusBars;
use crate::de::hysky::skyblocker::skyblock::fancybars::status_bar::Direction;
use crate::de::hysky::skyblocker::skyblock::fancybars::status_bar::IconPosition;
use crate::de::hysky::skyblocker::skyblock::fancybars::status_bar::StatusBar;
use crate::de::hysky::skyblocker::skyblock::fancybars::status_bar::TextPosition;
use crate::de::hysky::skyblocker::skyblock::fancybars::status_bars_config_screen::StatusBarsConfigScreen;
use crate::de::hysky::skyblocker::utils::enum_utils::EnumUtils;
use crate::de::hysky::skyblocker::utils::render::gui_helper::GuiHelper;
use mcrust::java::awt::color::Color;
use mcrust::mcrust::modding::widgets::ModWidget;
use mcrust::mcrust::modding::widgets::WidgetRenderContext;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::active_text_collector;
use mcrust::net::minecraft::client::gui::active_text_collector::ActiveTextCollector;
use mcrust::net::minecraft::client::gui::components::abstract_container_widget::AbstractContainerWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::WidgetKind;
use mcrust::net::minecraft::client::gui::components::events::gui_event_listener::GuiEventListener;
use mcrust::net::minecraft::client::gui::font::Font;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::layouts::layout;
use mcrust::net::minecraft::client::gui::layouts::layout_element::LayoutElement;
use mcrust::net::minecraft::client::gui::layouts::linear_layout::LinearLayout;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::gui::screens::inventory::tooltip::tooltip_render_util;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::client::input::mouse_button_info::MouseButtonInfo;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::util::unit::Unit;
use mcrust::org::joml::matrix3x2f_stack::Matrix3x2fStack;
use mcrust::net::minecraft::client::gui::components::renderable::Renderable as _;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final int BASE_WIDTH = 100;
pub const BASE_WIDTH: i32 = 100;
/// java: private static final int DEFAULT_OPTION_HEIGHT = 11;
pub const DEFAULT_OPTION_HEIGHT: i32 = 11;

/// java: public class EditBarWidget extends AbstractContainerWidget
pub struct EditBarWidget {
    /// java: extends net.minecraft.client.gui.components.AbstractContainerWidget
    pub base: AbstractContainerWidget,
    /// java: private final StringWidget nameWidget;
    pub name_widget: AbstractWidget,
    /// java: private final List<? extends AbstractOption<?>> options;
    pub options: Vec<compat::Shared<dyn AbstractOption<compat::Object>>>,
    /// java: private final int contentsWidth;
    pub contents_width: i32,
    /// java: public int insideMouseX = 0;
    pub inside_mouse_x: i32,
    /// java: public int insideMouseY = 0;
    pub inside_mouse_y: i32,
}

impl std::ops::Deref for EditBarWidget {
    type Target = AbstractContainerWidget;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for EditBarWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl EditBarWidget {
    /// java: public EditBarWidget(int x, int y, StatusBarsConfigScreen parent)
    pub fn new(mut x: i32, mut y: i32, mut parent: compat::Shared<StatusBarsConfigScreen>) -> Self {
        let mut text_renderer: Font = compat::font();
        let __init_name_widget = compat::string_widget(0, 0, -1, 9, component::empty());
        let mut layout: LinearLayout = LinearLayout::vertical();
        // TODO(fabric2seer): static `AbstractScrollArea.defaultSettings` has no Rust counterpart in the port
        //   java: super(x, y, BASE_WIDTH, 110, Component.literal("Edit bar"), AbstractScrollArea.defaultSettings(4));
        let mut this: Self = todo!("constructor");
        // TODO(fabric2seer): a widget put in a layout outside a mod screen's own methods: the port's layouts hold a widget by its index in the screen's widget list
        // rust: layout.add_child(EnumCyclingOption::new(component::translatable("skyblocker.bars.config.icon"), std::any::TypeId::of::<IconPosition>(), Arc::new(move |__a0| ({ let __f = compat::dispatch(&__a0, |__t| __t.get_icon_position()); __f(&__a0) })), Arc::new(move |__a0, __a1| { ({ let __f = compat::dispatch(&__a0, |__t| __t.set_icon_position()); __f(&__a0, __a1.clone()) }); })));
        // TODO(fabric2seer): a widget put in a layout outside a mod screen's own methods: the port's layouts hold a widget by its index in the screen's widget list
        // rust: layout.add_child(EnumCyclingOption::new(component::translatable("skyblocker.bars.config.text"), std::any::TypeId::of::<TextPosition>(), Arc::new(move |__a0| ({ let __f = compat::dispatch(&__a0, |__t| __t.get_text_position()); __f(&__a0) })), Arc::new(move |__a0, __a1| { ({ let __f = compat::dispatch(&__a0, |__t| __t.set_text_position()); __f(&__a0, __a1.clone()) }); })));
        // TODO(fabric2seer): a widget put in a layout outside a mod screen's own methods: the port's layouts hold a widget by its index in the screen's widget list
        // rust: layout.add_child(BooleanOption::new(component::translatable("skyblocker.bars.config.showMax"), Arc::new(move |bar| (if { let __f = compat::dispatch(&bar, |__t| __t.has_max()); __f(&bar) } { Some(compat::read(&bar, |__t| __t.status_bar_base().show_max)) } else { None })), Arc::new(move |bar, show_max| { { let __v = show_max; compat::write(&bar, |__t| __t.status_bar_base_mut().show_max = __v); } })));
        // TODO(fabric2seer): a widget put in a layout outside a mod screen's own methods: the port's layouts hold a widget by its index in the screen's widget list
        // rust: layout.add_child(BooleanOption::new(component::translatable("skyblocker.bars.config.showOverflow"), Arc::new(move |bar| (if { let __f = compat::dispatch(&bar, |__t| __t.has_overflow()); __f(&bar) } { Some(compat::read(&bar, |__t| __t.status_bar_base().show_overflow)) } else { None })), Arc::new(move |bar, show_overflow| { { let __v = show_overflow; compat::write(&bar, |__t| __t.status_bar_base_mut().show_overflow = __v); } })));
        // TODO(fabric2seer): a widget put in a layout outside a mod screen's own methods: the port's layouts hold a widget by its index in the screen's widget list
        // rust: layout.add_child(EnumCyclingOption::new(component::translatable("skyblocker.bars.config.direction"), std::any::TypeId::of::<Direction>(), Arc::new(move |__a0| ({ let __f = compat::dispatch(&__a0, |__t| __t.get_direction()); __f(&__a0) })), Arc::new(move |__a0, __a1| { ({ let __f = compat::dispatch(&__a0, |__t| __t.set_direction()); __f(&__a0, __a1.clone()) }); })));
        // TODO(fabric2seer): a widget put in a layout outside a mod screen's own methods: the port's layouts hold a widget by its index in the screen's widget list
        // rust: layout.add_child(ColorOption::new(component::translatable("skyblocker.bars.config.mainColor"), parent.clone(), Arc::new(move |bar| ({ let __f = compat::dispatch(&bar, |__t| __t.get_colors()); __f(&bar) })[(0) as usize].clone()), Arc::new(move |bar, color| { ({ let __f = compat::dispatch(&bar, |__t| __t.get_colors()); __f(&bar) })[(0) as usize] = color.clone(); })));
        // TODO(fabric2seer): a widget put in a layout outside a mod screen's own methods: the port's layouts hold a widget by its index in the screen's widget list
        // rust: layout.add_child(ColorOption::new(component::translatable("skyblocker.bars.config.overflowColor"), parent.clone(), Arc::new(move |bar| (if { let __f = compat::dispatch(&bar, |__t| __t.has_overflow()); __f(&bar) } { Some(({ let __f = compat::dispatch(&bar, |__t| __t.get_colors()); __f(&bar) })[(1) as usize].clone()) } else { None })), Arc::new(move |bar, color| { ({ let __f = compat::dispatch(&bar, |__t| __t.get_colors()); __f(&bar) })[(1) as usize] = color.clone(); })));
        // TODO(fabric2seer): a widget put in a layout outside a mod screen's own methods: the port's layouts hold a widget by its index in the screen's widget list
        // rust: layout.add_child(ColorOption::new(component::translatable("skyblocker.bars.config.textColor"), parent.clone(), Arc::new(move |__a0| ({ let __f = compat::dispatch(&__a0, |__t| __t.get_text_color()); __f(&__a0) })), Arc::new(move |__a0, __a1| { ({ let __f = compat::dispatch(&__a0, |__t| __t.set_text_color()); __f(&__a0, Some(__a1.clone())) }); })));
        // TODO(fabric2seer): a widget put in a layout outside a mod screen's own methods: the port's layouts hold a widget by its index in the screen's widget list
        // rust: layout.add_child(RunnableOption::new(component::translatable("skyblocker.bars.config.hide"), Arc::new(move |bar| compat::read(&bar, |__t| __t.status_bar_base().enabled)), Arc::new(move |bar| {
        // rust: if compat::read(&bar, |__t| __t.status_bar_base().anchor.clone()).is_some() {
        // rust: compat::read(&parent, |__t| __t.bars.clone()).lock().borrow().bar_positioner.remove_bar_bar_anchor_int_status_bar(compat::read(&bar, |__t| __t.status_bar_base().anchor.clone()).unwrap(), compat::read(&bar, |__t| __t.status_bar_base().grid_y), &bar);
        // rust: }
        // rust: { let __v = false; compat::write(&bar, |__t| __t.status_bar_base_mut().enabled = __v); }
        // rust: FancyStatusBars::update_positions(&compat::read(&parent, |__t| __t.bars.clone()), true);
        // rust: })));
        // TODO(fabric2seer): static call ImmutableList.builder(..)
        //   java: ImmutableList.Builder<AbstractOption<?>> builder = ImmutableList.builder();
        let mut builder: crate::unresolved::Builder = todo!("fabric2seer: the initialiser of `builder` (TODO above)");
        // TODO(fabric2seer): `.add(..)` on a Builder
        //   java: layout.visitWidgets(w -> builder.add((AbstractOption<?>) w));
        // TODO(fabric2seer): `.build(..)` on a Builder
        //   java: options = builder.build();
        this.contents_width = this.options.iter().cloned().map(|__a0| __a0.lock().borrow().expected_width()).max().unwrap_or(BASE_WIDTH);
        // TODO(fabric2seer): a lambda for a parameter the port does not type as a closure (`Consumer<net.minecraft.client.gui.components.AbstractWidget>`)
        // rust: layout::visit_widgets(|w| { w.set_width(this.contents_width); });
        layout.arrange_elements();
        // TODO(fabric2seer): `LayoutElement::set_position` ports `setPosition`, but the port's `LinearLayout` holds no `LayoutElement` (no Deref, no `base` field) to call it on
        // rust: layout.set_position(0, 11);
        this
    }

    /// java: @Override public List<? extends GuiEventListener> children()
    pub fn children(&self) -> Vec<Box<dyn GuiEventListener>> {
        return self.options.clone();
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_widget_render_state(&mut self, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        if self.is_hovered() {
            self.inside_mouse_x = mouse_x;
            self.inside_mouse_y = mouse_y;
        } else {
            let mut i: i32 = mouse_x.wrapping_sub(self.inside_mouse_x);
            let mut j: i32 = mouse_y.wrapping_sub(self.inside_mouse_y);
            if (i.wrapping_mul(i)).wrapping_add(j.wrapping_mul(j)) > 30i32.wrapping_mul(30) {
                self.visible = false;
            }
        }
        let __pose_1 = gui_graphics_extractor::push_pose();
        gui_graphics_extractor::pose_translate((self.get_x()) as f32, (self.get_y()) as f32);
        tooltip_render_util::extract_tooltip_background(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), 0, 0, self.get_width(), self.get_height(), None);
        // TODO(fabric2seer): `extract_render_state` is a method of the port's trait `Renderable`, which the port's `AbstractWidget` does not implement
        // rust: self.name_widget.extract_render_state(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mouse_x, mouse_y, a);
        for option in self.options.iter() {
            // TODO(fabric2seer): `extract_render_state` is a method of the port's trait `Renderable`, which the port's `AbstractWidget` does not implement
            // rust: option.extract_render_state(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mouse_x.wrapping_sub(self.get_x()), mouse_y.wrapping_sub(self.get_y()), a);
        }
        gui_graphics_extractor::pop_pose(__pose_1);
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    pub fn update_widget_narration(&self, builder: &dyn NarrationElementOutput) {
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    pub fn mouse_clicked(&self, click: &MouseButtonEvent, mut doubled: bool) -> bool {
        if !self.visible {
            return false;
        }
        if !self.is_hovered() {
            self.visible = false;
        }
        return { let __s0 = MouseButtonEvent::new((click.x() - (self.get_x() as f64)), (click.y() - (self.get_y() as f64)), click.button_info()); self.base.mouse_clicked(__s0, doubled) };
    }

    /// java: public void setStatusBar(StatusBar statusBar)
    pub fn set_status_bar(&mut self, status_bar: &compat::Shared<dyn StatusBar>) {
        for opt in self.options.iter() {
            { let (__a0,) = (status_bar.clone(),); opt.lock().borrow_mut().update_from_bar(__a0) };
        }
        let mut formatted: MutableComponent = ({ let __f = compat::dispatch(status_bar, |__t| __t.get_name()); __f(status_bar) }).copy().with_style_format(ChatFormatting::Bold);
        self.name_widget.set_message_text(mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(formatted.clone()));
        { let __s0 = (compat::font().width_of_cached(&mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(formatted.clone()))).max(self.contents_width); self.set_width(__s0) };
    }

    /// java: @Override public void setWidth(int width)
    pub fn set_width(&mut self, mut width: i32) {
        self.base.set_width(width);
        for option in self.options.iter_mut() {
            option.set_width(width);
        }
        self.name_widget.set_width(width);
    }

    /// java: @Override protected int contentHeight()
    pub fn content_height(&self) -> i32 {
        return 0;
    }

    /// java: @Override protected double scrollRate()
    pub fn scroll_rate(&self) -> f64 {
        return ((0) as f64) as f64;
    }

}

// NOTE(fabric2seer): children, contentHeight, extractWidgetRenderState, mouseClicked, scrollRate, setWidth, updateWidgetNarration override(s) of AbstractContainerWidget - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them

/// java: public class RunnableOption extends AbstractOption<Unit>
#[derive(Clone)]
pub struct RunnableOption {
    /// java: extends AbstractOption
    pub base: AbstractOptionBase<Unit>,
}

impl std::ops::Deref for RunnableOption {
    type Target = AbstractOptionBase<Unit>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for RunnableOption {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl RunnableOption {
    /// java: public RunnableOption(Component message, Predicate<StatusBar> isEnabled, Consumer<StatusBar> action)
    pub fn new(mut message: MutableComponent, is_enabled: Arc<dyn Fn(compat::Shared<dyn StatusBar>) -> bool>, action: Arc<dyn Fn(compat::Shared<dyn StatusBar>)>) -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            base: AbstractOptionBase::<Unit>::new_3(message.clone(), Arc::new(move |bar| (if (is_enabled)(bar.clone()) { Some(mcrust::net::minecraft::util::unit::Unit::INSTANCE) } else { None })), Arc::new(move |s, _| { (action)(s.clone()); })),
        });
        __this
    }

    /// java: @Override public void onClick(MouseButtonEvent click, boolean doubled)
    pub fn on_click(__this: &compat::Shared<Self>, click: &MouseButtonEvent, mut doubled: bool) {
        // TODO(fabric2seer): `AbstractWidget.onClick` has no Rust method in the port (looked for `on_click` in AbstractWidget and its supertypes)
        // rust: __this.lock().borrow_mut().base.on_click(click, doubled);
        // TODO(fabric2seer): `EditBarWidget.this` (the enclosing instance of an inner class)
        //   java: EditBarWidget.this.visible = false;
        __this.lock().borrow_mut().set_and_update(mcrust::net::minecraft::util::unit::Unit::INSTANCE);
    }

}

impl AbstractOption<mcrust::net::minecraft::util::unit::Unit> for RunnableOption {
    fn abstract_option_base(&self) -> &AbstractOptionBase<mcrust::net::minecraft::util::unit::Unit> {
        &self.base
    }
    fn abstract_option_base_mut(&mut self) -> &mut AbstractOptionBase<mcrust::net::minecraft::util::unit::Unit> {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override protected int extractValue(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_value(&self, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) -> i32 {
        return 0;
    }

    /// java: @Override protected int expectedValueWidth()
    fn expected_value_width(&self) -> i32 {
        return 0;
    }

}

/// RunnableOption's widget half (`super`'s fields: x, y, size, message, ...), lent by the game for each call
impl compat::WidgetHalf for RunnableOption {
    fn widget_half(&mut self) -> &mut AbstractWidget {
        &mut self.base.base
    }
}

/// RunnableOption as the game's widget: `compat::WidgetAdapter::widget(handle)` where Java hands it on
impl ModWidget for compat::WidgetAdapter<RunnableOption> {
    /// java: extractWidgetRenderState - not overridden here (the base class's body has no entry point in the port)
    fn extract_widget_render_state(&mut self, widget: &AbstractWidget, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        let _ = widget;
    }

    /// java: updateWidgetNarration - not overridden here (the base class's body has no entry point in the port)
    fn update_widget_narration(&mut self, widget: &AbstractWidget, output: &mut dyn NarrationElementOutput) {
        let _ = widget;
    }

    /// java: @Override public void onClick(MouseButtonEvent click, boolean doubled)
    fn on_click(&mut self, widget: &mut AbstractWidget, event: &MouseButtonEvent, double_click: bool) {
        self.lend(widget, |__h| RunnableOption::on_click(__h, event, double_click))
    }

}

/// java: private abstract static class AbstractOption<T> extends AbstractWidget
#[derive(Clone)]
pub struct AbstractOptionBase<T> {
    /// java: extends net.minecraft.client.gui.components.AbstractWidget
    pub base: AbstractWidget,
    /// java: protected @Nullable T current;
    pub current: Option<T>,
    /// java: protected final Function<StatusBar, @Nullable T> getter;
    pub getter: Option<Arc<dyn Fn(compat::Shared<dyn StatusBar>) -> Option<T>>>,
    /// java: protected final BiConsumer<StatusBar, T> setter;
    pub setter: Option<Arc<dyn Fn(compat::Shared<dyn StatusBar>, T)>>,
    /// java: protected @Nullable StatusBar activeBar;
    pub active_bar: Option<compat::Shared<dyn StatusBar>>,
}

impl<T: 'static> std::ops::Deref for AbstractOptionBase<T> {
    type Target = AbstractWidget;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<T: 'static> std::ops::DerefMut for AbstractOptionBase<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl<T: Clone + 'static> AbstractOptionBase<T> {
    /// java: private AbstractOption(int height, Component message, Function<StatusBar, @Nullable T> getter, BiConsumer<StatusBar, T> setter)
    pub fn new(mut height: i32, mut message: MutableComponent, getter: Arc<dyn Fn(compat::Shared<dyn StatusBar>) -> Option<T>>, setter: Arc<dyn Fn(compat::Shared<dyn StatusBar>, T)>) -> Self {
        let __init_getter = Some(getter.clone());
        let __init_setter = Some(setter.clone());
        let mut this = Self {
            base: AbstractWidget::new(0, 0, BASE_WIDTH, height, (&message.get_string()).to_string(), WidgetKind::PlainText, OnPress::Inert),
            current: None,
            getter: __init_getter,
            setter: __init_setter,
            active_bar: None,
        };
        this
    }

    /// java: private AbstractOption(Component message, Function<StatusBar, @Nullable T> getter, BiConsumer<StatusBar, T> setter)
    pub fn new_3(mut message: MutableComponent, getter: Arc<dyn Fn(compat::Shared<dyn StatusBar>) -> Option<T>>, setter: Arc<dyn Fn(compat::Shared<dyn StatusBar>, T)>) -> Self {
        let mut this = Self::new(DEFAULT_OPTION_HEIGHT, message.clone(), getter.clone(), setter.clone());
        this
    }

}

/// java: the virtual methods of AbstractOption (implemented by AbstractOptionBase and every subclass)
pub trait AbstractOption<T> {
    fn abstract_option_base(&self) -> &AbstractOptionBase<T>;
    fn abstract_option_base_mut(&mut self) -> &mut AbstractOptionBase<T>;
    fn as_any(&self) -> &dyn std::any::Any;

    /// java: public void updateFromBar(StatusBar statusBar)
    fn update_from_bar(&mut self, mut status_bar: compat::Shared<dyn StatusBar>) {
        let mut apply: Option<T> = (self.abstract_option_base().getter.as_ref().unwrap())(status_bar.clone());
        if let Some(apply) = &apply {
            self.abstract_option_base_mut().current = Some(apply.clone());
            self.abstract_option_base_mut().active = true;
        } else {
            self.abstract_option_base_mut().current = None;
            self.abstract_option_base_mut().active = false;
        }
        self.abstract_option_base_mut().active_bar = Some(status_bar.clone());
    }

    /// java: protected void setAndUpdate(T value)
    fn set_and_update(&mut self, mut value: T) {
        self.abstract_option_base_mut().current = Some(value.clone());
        if self.abstract_option_base().active_bar.is_some() {
            (self.abstract_option_base().setter.as_ref().unwrap())(self.abstract_option_base().active_bar.clone(), value.clone());
        }
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_widget_render_state(&self, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        if self.abstract_option_base().is_mouse_over((mouse_x) as f64, (mouse_y) as f64) {
            graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), self.abstract_option_base().get_x(), self.abstract_option_base().get_y(), self.abstract_option_base().get_right(), self.abstract_option_base().get_bottom(), 0x20FFFFFF);
        }
        let mut font: Font = compat::font();
        // TODO(fabric2seer): the port's `text_renderer` ports this Java method with other parameters (mouse_x, mouse_y for Java's )
        // TODO(fabric2seer): `text_renderer` takes 2 argument(s) in the port (`mouse_x: f64, mouse_y: f64`), Java passes 0
        // rust: let mut text_renderer: Box<dyn ActiveTextCollector> = gui_graphics_extractor::text_renderer();
        let mut text_renderer: Box<dyn ActiveTextCollector> = todo!("fabric2seer: the initialiser of `textRenderer` (TODO above)");
        let mut value_width: i32 = self.extract_value(graphics, mouse_x, mouse_y, a);
        let mut message: MutableComponent = compat::component_of(self.abstract_option_base().get_message_component()).copy().with_color((if self.abstract_option_base().active { mcrust::net::minecraft::util::common_colors::WHITE } else { mcrust::net::minecraft::util::common_colors::GRAY }));
        if font.width_of_cached(&mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(compat::component_of(self.abstract_option_base().get_message_component()))) > ((self.abstract_option_base().get_width()).wrapping_sub(value_width)).wrapping_sub(2) {
            // TODO(fabric2seer): `accept_scrolling_with_default_center` takes 6 argument(s) in the port (`left: i32, right: i32, top: i32, bottom: i32, line_width: i32, time_millis: i64`), Java passes 5
            // rust: active_text_collector::accept_scrolling_with_default_center(message.clone(), (self.abstract_option_base().get_x()).wrapping_add(1), ((self.abstract_option_base().get_right()).wrapping_sub(value_width)).wrapping_sub(1), self.abstract_option_base().get_y(), self.abstract_option_base().get_bottom());
        } else {
            // TODO(fabric2seer): `ActiveTextCollector.accept` has no Rust method in the port (looked for `accept` in ActiveTextCollector and its supertypes)
            // rust: text_renderer.accept((self.abstract_option_base().get_x()).wrapping_add(1), (self.abstract_option_base().get_y()).wrapping_add(1), &message);
        }
    }

    /// java: protected abstract int extractValue(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a);
    fn extract_value(&self, graphics: &mut GuiGraphicsExtractor, mouse_x: i32, mouse_y: i32, a: f32) -> i32;

    /// java: protected abstract int expectedValueWidth();
    fn expected_value_width(&self) -> i32;

    /// java: public final int expectedWidth()
    fn expected_width(&self) -> i32 {
        return (self.expected_value_width().wrapping_add(compat::font().width_of_cached(&mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(compat::component_of(self.abstract_option_base().get_message_component()))))).wrapping_add(2);
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput output)
    fn update_widget_narration(&self, output: &dyn NarrationElementOutput) {
    }

}

/// java: public static class EnumCyclingOption<T extends Enum<T>> extends AbstractOption<T>
#[derive(Clone)]
pub struct EnumCyclingOption<T> {
    /// java: extends AbstractOption
    pub base: AbstractOptionBase<T>,
    /// java: private final T[] values;
    pub values: Vec<T>,
}

impl<T: 'static> std::ops::Deref for EnumCyclingOption<T> {
    type Target = AbstractOptionBase<T>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<T: 'static> std::ops::DerefMut for EnumCyclingOption<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl<T: Clone + 'static> EnumCyclingOption<T> {
    /// java: public EnumCyclingOption(Component message, Class<T> enumClass, Function<StatusBar, @Nullable T> getter, BiConsumer<StatusBar, T> setter)
    pub fn new(mut message: MutableComponent, mut enum_class: crate::unresolved::Class, getter: Arc<dyn Fn(compat::Shared<dyn StatusBar>) -> Option<T>>, setter: Arc<dyn Fn(compat::Shared<dyn StatusBar>, T)>) -> compat::Shared<Self> {
        // TODO(fabric2seer): `.getEnumConstants(..)` on a Class
        //   java: values = enumClass.getEnumConstants();
        let __this = compat::shared(Self {
            base: AbstractOptionBase::<T>::new_3(message.clone(), getter.clone(), setter.clone()),
            values: todo!("fabric2seer: the field value (TODO above)"),
        });
        { let __v = Some(__this.lock().borrow().values[(0) as usize].clone()); compat::write(&__this, |__t| __t.current = __v); }
        __this
    }

    /// java: @Override public void onClick(MouseButtonEvent click, boolean doubled)
    pub fn on_click(__this: &compat::Shared<Self>, click: &MouseButtonEvent, mut doubled: bool) {
        let __r0 = compat::read(__this, |__t| __t.current.clone());
        let __r1 = compat::read(__this, |__t| __t.current.clone());
        { let (__a0,) = ((if __r0.is_some() { EnumUtils::cycle(&__r1.unwrap()) } else { __this.lock().borrow().values[(0) as usize].clone() }),); __this.lock().borrow_mut().set_and_update(__a0) };
        // TODO(fabric2seer): `AbstractWidget.onClick` has no Rust method in the port (looked for `on_click` in AbstractWidget and its supertypes)
        // rust: __this.lock().borrow_mut().base.on_click(click, doubled);
    }

}

impl<T: Clone + 'static> AbstractOption<T> for EnumCyclingOption<T> {
    fn abstract_option_base(&self) -> &AbstractOptionBase<T> {
        &self.base
    }
    fn abstract_option_base_mut(&mut self) -> &mut AbstractOptionBase<T> {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override protected int extractValue(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_value(&self, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) -> i32 {
        let mut text_renderer: Font = compat::font();
        let mut string: String = (if self.current.is_some() { compat::opt_str(&self.current) } else { "???".to_string() });
        let mut value_width: i32 = (text_renderer.width(&string)).wrapping_add(1);
        gui_graphics_extractor::text_string_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), Some(&string), self.get_right().wrapping_sub(value_width), self.get_y().wrapping_add(1), mcrust::net::minecraft::util::common_colors::WHITE, true);
        return value_width;
    }

    /// java: @Override protected int expectedValueWidth()
    fn expected_value_width(&self) -> i32 {
        let mut m: i32 = 0;
        for value in self.values.iter() {
            let mut i: i32 = compat::font().width(&compat::jstring(&value));
            m = m.max(i);
        }
        return m;
    }

}

/// java: public static class BooleanOption extends AbstractOption<Boolean>
#[derive(Clone)]
pub struct BooleanOption {
    /// java: extends AbstractOption
    pub base: AbstractOptionBase<bool>,
}

impl std::ops::Deref for BooleanOption {
    type Target = AbstractOptionBase<bool>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for BooleanOption {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl BooleanOption {
    /// java: public BooleanOption(Component message, Function<StatusBar, @Nullable Boolean> getter, BiConsumer<StatusBar, Boolean> setter)
    pub fn new(mut message: MutableComponent, getter: Arc<dyn Fn(compat::Shared<dyn StatusBar>) -> Option<bool>>, setter: Arc<dyn Fn(compat::Shared<dyn StatusBar>, bool)>) -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            base: AbstractOptionBase::<bool>::new_3(message.clone(), getter.clone(), setter.clone()),
        });
        __this
    }

    /// java: @Override public void onClick(MouseButtonEvent click, boolean doubled)
    pub fn on_click(__this: &compat::Shared<Self>, click: &MouseButtonEvent, mut doubled: bool) {
        let __r0 = compat::read(__this, |__t| __t.current.clone());
        let __r1 = compat::read(__this, |__t| __t.current.clone());
        { let (__a0,) = ((__r0.is_none() || !(__r1)),); __this.lock().borrow_mut().set_and_update(__a0) };
        // TODO(fabric2seer): `AbstractWidget.onClick` has no Rust method in the port (looked for `on_click` in AbstractWidget and its supertypes)
        // rust: __this.lock().borrow_mut().base.on_click(click, doubled);
    }

    /// java: public void setCurrent(boolean current)
    pub fn set_current(__this: &compat::Shared<Self>, mut current: bool) {
        { let __v = Some(current); compat::write(__this, |__t| __t.current = __v); }
    }

}

impl AbstractOption<bool> for BooleanOption {
    fn abstract_option_base(&self) -> &AbstractOptionBase<bool> {
        &self.base
    }
    fn abstract_option_base_mut(&mut self) -> &mut AbstractOptionBase<bool> {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override protected int extractValue(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_value(&self, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) -> i32 {
        GuiHelper::border(graphics, self.get_right().wrapping_sub(10), self.get_y().wrapping_add(1), 9, 9, (if self.active { -1 } else { mcrust::net::minecraft::util::common_colors::GRAY }));
        if self.active && true == self.current.clone().unwrap() {
            graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), self.get_right().wrapping_sub(8), self.get_y().wrapping_add(3), self.get_right().wrapping_sub(3), self.get_y().wrapping_add(8), mcrust::net::minecraft::util::common_colors::WHITE);
        }
        return 10;
    }

    /// java: @Override protected int expectedValueWidth()
    fn expected_value_width(&self) -> i32 {
        return 10;
    }

}

/// BooleanOption's widget half (`super`'s fields: x, y, size, message, ...), lent by the game for each call
impl compat::WidgetHalf for BooleanOption {
    fn widget_half(&mut self) -> &mut AbstractWidget {
        &mut self.base.base
    }
}

/// BooleanOption as the game's widget: `compat::WidgetAdapter::widget(handle)` where Java hands it on
impl ModWidget for compat::WidgetAdapter<BooleanOption> {
    /// java: extractWidgetRenderState - not overridden here (the base class's body has no entry point in the port)
    fn extract_widget_render_state(&mut self, widget: &AbstractWidget, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        let _ = widget;
    }

    /// java: updateWidgetNarration - not overridden here (the base class's body has no entry point in the port)
    fn update_widget_narration(&mut self, widget: &AbstractWidget, output: &mut dyn NarrationElementOutput) {
        let _ = widget;
    }

    /// java: @Override public void onClick(MouseButtonEvent click, boolean doubled)
    fn on_click(&mut self, widget: &mut AbstractWidget, event: &MouseButtonEvent, double_click: bool) {
        self.lend(widget, |__h| BooleanOption::on_click(__h, event, double_click))
    }

}

/// java: public static class ColorOption extends AbstractOption<Color>
pub struct ColorOption {
    /// java: extends AbstractOption
    pub base: AbstractOptionBase<Color>,
    /// java: private final Screen parent;
    pub parent: mcrust::net::minecraft::client::gui::screens::screen::Screen,
}

impl std::ops::Deref for ColorOption {
    type Target = AbstractOptionBase<Color>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for ColorOption {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl ColorOption {
    /// java: public ColorOption(Component message, Screen parent, Function<StatusBar, @Nullable Color> getter, BiConsumer<StatusBar, Color> setter)
    pub fn new(mut message: MutableComponent, mut parent: mcrust::net::minecraft::client::gui::screens::screen::Screen, getter: Arc<dyn Fn(compat::Shared<dyn StatusBar>) -> Option<Color>>, setter: Arc<dyn Fn(compat::Shared<dyn StatusBar>, Color)>) -> compat::Shared<Self> {
        // TODO(fabric2seer): the mod keeps the game's `Screen` (Java holds a reference to the live object); the port's `Screen` is not a value to copy and the port lends the mod no handle to it
        //   java: this.parent = parent;
        let __this = compat::shared(Self {
            base: AbstractOptionBase::<Color>::new_3(message.clone(), getter.clone(), setter.clone()),
            parent: todo!("fabric2seer: the field value (TODO above)"),
        });
        __this
    }

    /// java: @Override public void onClick(MouseButtonEvent click, boolean doubled)
    pub fn on_click(__this: &compat::Shared<Self>, click: &MouseButtonEvent, mut doubled: bool) {
        // TODO(fabric2seer): `AbstractWidget.onClick` has no Rust method in the port (looked for `on_click` in AbstractWidget and its supertypes)
        // rust: __this.lock().borrow_mut().base.on_click(click, doubled);
        // TODO(fabric2seer): `.getRGB(..)` on a T
        //   java: Minecraft.getInstance().gui.setScreen(new EditBarColorPopup(Component.literal("Edit ").append(getMessage()), parent, this::setAndUpdate, current != null ? current.getRGB() : -1));
    }

}

impl AbstractOption<mcrust::java::awt::color::Color> for ColorOption {
    fn abstract_option_base(&self) -> &AbstractOptionBase<mcrust::java::awt::color::Color> {
        &self.base
    }
    fn abstract_option_base_mut(&mut self) -> &mut AbstractOptionBase<mcrust::java::awt::color::Color> {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override protected int extractValue(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_value(&self, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) -> i32 {
        GuiHelper::border(graphics, self.get_right().wrapping_sub(10), self.get_y().wrapping_add(1), 9, 9, (if self.active { -1 } else { mcrust::net::minecraft::util::common_colors::GRAY }));
        // TODO(fabric2seer): `.getRGB(..)` on a T
        //   java: graphics.fill(getRight() - 8, getY() + 3, getRight() - 3, getY() + 8, active && current != null ? current.getRGB() : CommonColors.GRAY);
        return 10;
    }

    /// java: @Override protected int expectedValueWidth()
    fn expected_value_width(&self) -> i32 {
        return 10;
    }

}

/// ColorOption's widget half (`super`'s fields: x, y, size, message, ...), lent by the game for each call
impl compat::WidgetHalf for ColorOption {
    fn widget_half(&mut self) -> &mut AbstractWidget {
        &mut self.base.base
    }
}

/// ColorOption as the game's widget: `compat::WidgetAdapter::widget(handle)` where Java hands it on
impl ModWidget for compat::WidgetAdapter<ColorOption> {
    /// java: extractWidgetRenderState - not overridden here (the base class's body has no entry point in the port)
    fn extract_widget_render_state(&mut self, widget: &AbstractWidget, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        let _ = widget;
    }

    /// java: updateWidgetNarration - not overridden here (the base class's body has no entry point in the port)
    fn update_widget_narration(&mut self, widget: &AbstractWidget, output: &mut dyn NarrationElementOutput) {
        let _ = widget;
    }

    /// java: @Override public void onClick(MouseButtonEvent click, boolean doubled)
    fn on_click(&mut self, widget: &mut AbstractWidget, event: &MouseButtonEvent, double_click: bool) {
        self.lend(widget, |__h| ColorOption::on_click(__h, event, double_click))
    }

}

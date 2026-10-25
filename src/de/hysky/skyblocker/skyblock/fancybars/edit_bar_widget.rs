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
use crate::de::hysky::skyblocker::utils::enum_utils::JavaEnum;
use crate::de::hysky::skyblocker::utils::render::gui_helper::GuiHelper;
use mcrust::java::awt::color::Color;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::active_text_collector;
use mcrust::net::minecraft::client::gui::components::abstract_container_widget::AbstractContainerWidget;
use mcrust::net::minecraft::client::gui::components::abstract_scroll_area::ScrollbarSettings;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::WidgetKind;
use mcrust::net::minecraft::client::gui::font::Font;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::gui::screens::inventory::tooltip::tooltip_render_util;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::formatted_text::FormattedText;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::util::unit::Unit;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final int BASE_WIDTH = 100;
pub const BASE_WIDTH: i32 = 100;
/// java: private static final int DEFAULT_OPTION_HEIGHT = 11;
pub const DEFAULT_OPTION_HEIGHT: i32 = 11;

/// java: public class EditBarWidget extends AbstractContainerWidget
///
/// The screen holds this widget itself (`StatusBarsConfigScreen.editBarWidget`) and draws it and hands it its clicks
/// (Java's `addWidget(editBarWidget)` / `editBarWidget.extractRenderState(..)`); its options are its own children,
/// drawn and clicked here in the widget's own coordinates, as Java's `extractWidgetRenderState` / `mouseClicked` do.
pub struct EditBarWidget {
    /// java: extends net.minecraft.client.gui.components.AbstractContainerWidget
    pub base: AbstractContainerWidget,
    /// java: private final StringWidget nameWidget;
    pub name_widget: AbstractWidget,
    /// java: private final List<? extends AbstractOption<?>> options;
    pub options: Vec<Box<dyn EditOption>>,
    /// java: private final int contentsWidth;
    pub contents_width: i32,
    /// java: public int insideMouseX = 0;
    pub inside_mouse_x: i32,
    /// java: public int insideMouseY = 0;
    pub inside_mouse_y: i32,
    /// java: `EditBarWidget.this.visible = false` from an option's `onClick` (the options are inner objects that
    /// reach this widget: the write is made here after the click)
    pub hide_requested: std::rc::Rc<std::cell::Cell<bool>>,
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
    pub fn new(x: i32, y: i32, parent: compat::Shared<StatusBarsConfigScreen>) -> Self {
        let text_renderer: Font = compat::font();
        // java: super(x, y, BASE_WIDTH, 110, Component.literal("Edit bar"), AbstractScrollArea.defaultSettings(4));
        let base = AbstractContainerWidget::new_with_scrollbar_settings(x, y, BASE_WIDTH, 110, FormattedText::from(component::literal("Edit bar")), ScrollbarSettings::default_settings(4.0));
        // java: nameWidget = new StringWidget(Component.empty(), textRenderer);
        let name_widget = compat::string_widget(0, 0, -1, mcrust::net::minecraft::client::gui::font::LINE_HEIGHT, component::empty());
        let hide_requested: std::rc::Rc<std::cell::Cell<bool>> = Default::default();
        let parent_w = std::sync::Arc::downgrade(&parent);
        let mut options: Vec<Box<dyn EditOption>> = Vec::new();
        // java: layout.addChild(new EnumCyclingOption<>(..icon.., IconPosition.class, StatusBar::getIconPosition, StatusBar::setIconPosition));
        options.push(Box::new(EnumCyclingOption::<IconPosition>::new(component::translatable("skyblocker.bars.config.icon"), Arc::new(|__a0: compat::Shared<dyn StatusBar>| Some({ let __f = compat::dispatch(&__a0, |__t| __t.get_icon_position()); __f(&__a0) })), Arc::new(|__a0: compat::Shared<dyn StatusBar>, __a1: IconPosition| { let __f = compat::dispatch(&__a0, |__t| __t.set_icon_position()); __f(&__a0, __a1) }))));
        // java: layout.addChild(new EnumCyclingOption<>(..text.., TextPosition.class, StatusBar::getTextPosition, StatusBar::setTextPosition));
        options.push(Box::new(EnumCyclingOption::<TextPosition>::new(component::translatable("skyblocker.bars.config.text"), Arc::new(|__a0: compat::Shared<dyn StatusBar>| Some({ let __f = compat::dispatch(&__a0, |__t| __t.get_text_position()); __f(&__a0) })), Arc::new(|__a0: compat::Shared<dyn StatusBar>, __a1: TextPosition| { let __f = compat::dispatch(&__a0, |__t| __t.set_text_position()); __f(&__a0, __a1) }))));
        // java: layout.addChild(new BooleanOption(..showMax.., bar -> bar.hasMax() ? bar.showMax : null, (bar, showMax) -> bar.showMax = showMax));
        options.push(Box::new(BooleanOption::new(component::translatable("skyblocker.bars.config.showMax"), Arc::new(|bar: compat::Shared<dyn StatusBar>| if { let __f = compat::dispatch(&bar, |__t| __t.has_max()); __f(&bar) } { Some(compat::read(&bar, |__t| __t.status_bar_base().show_max)) } else { None }), Arc::new(|bar: compat::Shared<dyn StatusBar>, show_max: bool| compat::write(&bar, |__t| __t.status_bar_base_mut().show_max = show_max)))));
        // java: layout.addChild(new BooleanOption(..showOverflow.., bar -> bar.hasOverflow() ? bar.showOverflow : null, (bar, showOverflow) -> bar.showOverflow = showOverflow));
        options.push(Box::new(BooleanOption::new(component::translatable("skyblocker.bars.config.showOverflow"), Arc::new(|bar: compat::Shared<dyn StatusBar>| if { let __f = compat::dispatch(&bar, |__t| __t.has_overflow()); __f(&bar) } { Some(compat::read(&bar, |__t| __t.status_bar_base().show_overflow)) } else { None }), Arc::new(|bar: compat::Shared<dyn StatusBar>, show_overflow: bool| compat::write(&bar, |__t| __t.status_bar_base_mut().show_overflow = show_overflow)))));
        // java: layout.addChild(new EnumCyclingOption<>(..direction.., Direction.class, StatusBar::getDirection, StatusBar::setDirection));
        options.push(Box::new(EnumCyclingOption::<Direction>::new(component::translatable("skyblocker.bars.config.direction"), Arc::new(|__a0: compat::Shared<dyn StatusBar>| Some({ let __f = compat::dispatch(&__a0, |__t| __t.get_direction()); __f(&__a0) })), Arc::new(|__a0: compat::Shared<dyn StatusBar>, __a1: Direction| { let __f = compat::dispatch(&__a0, |__t| __t.set_direction()); __f(&__a0, __a1) }))));
        // java: layout.addChild(new ColorOption(..mainColor.., parent, bar -> bar.getColors()[0], (bar, color) -> bar.getColors()[0] = color));
        // (`getColors()` answers the bar's own array, which the setter writes)
        options.push(Box::new(ColorOption::new(component::translatable("skyblocker.bars.config.mainColor"), parent_w.clone(), Arc::new(|bar: compat::Shared<dyn StatusBar>| ({ let __f = compat::dispatch(&bar, |__t| __t.get_colors()); __f(&bar) }).first().cloned()), Arc::new(|bar: compat::Shared<dyn StatusBar>, color: Color| compat::write(&bar, |__t| __t.status_bar_base_mut().colors[0] = color)))));
        // java: layout.addChild(new ColorOption(..overflowColor.., parent, bar -> bar.hasOverflow() ? bar.getColors()[1] : null, (bar, color) -> bar.getColors()[1] = color));
        options.push(Box::new(ColorOption::new(component::translatable("skyblocker.bars.config.overflowColor"), parent_w.clone(), Arc::new(|bar: compat::Shared<dyn StatusBar>| if { let __f = compat::dispatch(&bar, |__t| __t.has_overflow()); __f(&bar) } { ({ let __f = compat::dispatch(&bar, |__t| __t.get_colors()); __f(&bar) }).get(1).cloned() } else { None }), Arc::new(|bar: compat::Shared<dyn StatusBar>, color: Color| compat::write(&bar, |__t| __t.status_bar_base_mut().colors[1] = color)))));
        // java: layout.addChild(new ColorOption(..textColor.., parent, StatusBar::getTextColor, StatusBar::setTextColor));
        options.push(Box::new(ColorOption::new(component::translatable("skyblocker.bars.config.textColor"), parent_w.clone(), Arc::new(|__a0: compat::Shared<dyn StatusBar>| { let __f = compat::dispatch(&__a0, |__t| __t.get_text_color()); __f(&__a0) }), Arc::new(|__a0: compat::Shared<dyn StatusBar>, __a1: Color| { let __f = compat::dispatch(&__a0, |__t| __t.set_text_color()); __f(&__a0, Some(__a1)) }))));
        // java: layout.addChild(new RunnableOption(..hide.., bar -> bar.enabled, bar -> {..}));
        let hide_parent = parent_w.clone();
        options.push(Box::new(RunnableOption::new(component::translatable("skyblocker.bars.config.hide"), Arc::new(|bar: compat::Shared<dyn StatusBar>| compat::read(&bar, |__t| __t.status_bar_base().enabled)), Arc::new(move |bar: compat::Shared<dyn StatusBar>| {
            let Some(parent) = hide_parent.upgrade() else { return; };
            let bars = compat::read(&parent, |__t| __t.bars.clone());
            if let Some(anchor) = compat::read(&bar, |__t| __t.status_bar_base().anchor.clone()) {
                let grid_y = compat::read(&bar, |__t| __t.status_bar_base().grid_y);
                compat::write(&bars, |__b| __b.bar_positioner.remove_bar_bar_anchor_int_status_bar(anchor, grid_y, &bar));
            }
            compat::write(&bar, |__t| __t.status_bar_base_mut().enabled = false);
            FancyStatusBars::update_positions(&bars, true);
        }), hide_requested.clone())));
        // java: options = builder.build(); - `layout.visitWidgets(w -> builder.add((AbstractOption<?>) w))`, in order
        // java: contentsWidth = options.stream().mapToInt(AbstractOption::expectedWidth).max().orElse(BASE_WIDTH);
        let contents_width = options.iter().map(|__o| __o.expected_width()).max().unwrap_or(BASE_WIDTH);
        // java: layout.visitWidgets(w -> w.setWidth(contentsWidth)); layout.arrangeElements(); layout.setPosition(0, 11);
        // (a vertical `LinearLayout` with no spacing: the options stacked from (0, 11) in the widget's coordinates)
        let mut y_pos = 11;
        for option in options.iter() {
            option.set_width(contents_width);
            option.set_position(0, y_pos);
            y_pos += option.height();
        }
        EditBarWidget { base, name_widget, options, contents_width, inside_mouse_x: 0, inside_mouse_y: 0, hide_requested }
    }

    /// java: @Override public List<? extends GuiEventListener> children() - `return options;`
    pub fn children(&self) -> &Vec<Box<dyn EditOption>> {
        return &self.options;
    }

    /// AbstractWidget.extractRenderState - `if (!visible) return; isHovered = ..; extractWidgetRenderState(..)`
    pub fn extract_render_state(&mut self, graphics: &mut GuiGraphicsExtractor, mouse_x: i32, mouse_y: i32, a: f32) {
        if !self.visible {
            return;
        }
        self.base.is_hovered = self.base.is_mouse_over(mouse_x as f64, mouse_y as f64);
        self.extract_widget_render_state(graphics, mouse_x, mouse_y, a);
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_widget_render_state(&mut self, graphics: &mut GuiGraphicsExtractor, mouse_x: i32, mouse_y: i32, a: f32) {
        if self.is_hovered() {
            self.inside_mouse_x = mouse_x;
            self.inside_mouse_y = mouse_y;
        } else {
            let i: i32 = mouse_x.wrapping_sub(self.inside_mouse_x);
            let j: i32 = mouse_y.wrapping_sub(self.inside_mouse_y);
            if (i.wrapping_mul(i)).wrapping_add(j.wrapping_mul(j)) > 30i32.wrapping_mul(30) {
                self.visible = false;
            }
        }
        let (x, y) = (self.get_x(), self.get_y());
        let __pose_1 = gui_graphics_extractor::push_pose();
        gui_graphics_extractor::pose_translate(x as f32, y as f32);
        tooltip_render_util::extract_tooltip_background(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), 0, 0, self.get_width(), self.get_height(), None);
        // java: nameWidget.extractRenderState(graphics, mouseX, mouseY, a); - a `StringWidget`: its message at its place
        if let Some(name) = self.name_widget.message_text.clone() {
            gui_graphics_extractor::draw_formatted_text_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &name, self.name_widget.get_x(), self.name_widget.get_y(), mcrust::net::minecraft::util::common_colors::WHITE, true);
        }
        for option in self.options.iter() {
            option.extract_render_state(graphics, mouse_x.wrapping_sub(x), mouse_y.wrapping_sub(y), a, (x, y));
        }
        gui_graphics_extractor::pop_pose(__pose_1);
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    pub fn update_widget_narration(&self, _builder: &dyn NarrationElementOutput) {
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    pub fn mouse_clicked(&mut self, click: &MouseButtonEvent, doubled: bool) -> bool {
        if !self.visible {
            return false;
        }
        if !self.is_hovered() {
            self.visible = false;
        }
        // java: return super.mouseClicked(new MouseButtonEvent(click.x() - getX(), click.y() - getY(), click.buttonInfo()), doubled);
        // - `AbstractContainerWidget.mouseClicked`: the scroll bar (none: `contentHeight()` is 0) and then
        // `ContainerEventHandler.mouseClicked`, the first child that takes the click
        let shifted = MouseButtonEvent::new(click.x() - (self.get_x() as f64), click.y() - (self.get_y() as f64), click.button_info());
        for option in self.options.iter() {
            if option.mouse_clicked(&shifted, doubled) {
                // `EditBarWidget.this.visible = false;` from the option's `onClick`
                if self.hide_requested.replace(false) {
                    self.visible = false;
                }
                return true;
            }
        }
        return false;
    }

    /// java: public void setStatusBar(StatusBar statusBar)
    pub fn set_status_bar(&mut self, status_bar: &compat::Shared<dyn StatusBar>) {
        for opt in self.options.iter() {
            opt.update_from_bar(status_bar.clone());
        }
        let formatted: MutableComponent = ({ let __f = compat::dispatch(status_bar, |__t| __t.get_name()); __f(status_bar) }).copy().with_style_format(ChatFormatting::Bold);
        self.name_widget.set_message_text(FormattedText::from(formatted.clone()));
        { let __s0 = (compat::font().width_of_cached(&FormattedText::from(formatted.clone()))).max(self.contents_width); self.set_width(__s0) };
    }

    /// java: @Override public void setWidth(int width)
    pub fn set_width(&mut self, width: i32) {
        self.base.set_width(width);
        for option in self.options.iter() {
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
        return 0.0;
    }

}

/// One of `EditBarWidget.options` (`AbstractOption<?>`): the calls the widget makes on its children.
pub trait EditOption {
    /// java: public void updateFromBar(StatusBar statusBar)
    fn update_from_bar(&self, status_bar: compat::Shared<dyn StatusBar>);
    /// java: public final int expectedWidth()
    fn expected_width(&self) -> i32;
    /// AbstractWidget.getHeight
    fn height(&self) -> i32;
    /// AbstractWidget.setWidth
    fn set_width(&self, width: i32);
    /// AbstractWidget.setPosition
    fn set_position(&self, x: i32, y: i32);
    /// AbstractWidget.extractRenderState (mouse in the widget's coordinates; `origin` is where they start on screen)
    fn extract_render_state(&self, graphics: &mut GuiGraphicsExtractor, mouse_x: i32, mouse_y: i32, a: f32, origin: (i32, i32));
    /// AbstractWidget.mouseClicked
    fn mouse_clicked(&self, click: &MouseButtonEvent, doubled: bool) -> bool;
}

/// An option class's `onClick(MouseButtonEvent, boolean)` override.
pub trait OptionClick {
    fn on_click(__this: &compat::Shared<Self>, click: &MouseButtonEvent, doubled: bool);
}

macro_rules! edit_option {
    ([$($g:tt)*] $o:ty) => {
impl<$($g)*> EditOption for compat::Shared<$o> {
    fn update_from_bar(&self, status_bar: compat::Shared<dyn StatusBar>) {
        compat::write(self, |__t| __t.update_from_bar(status_bar));
    }

    fn expected_width(&self) -> i32 {
        compat::read(self, |__t| __t.expected_width())
    }

    fn height(&self) -> i32 {
        compat::read(self, |__t| __t.abstract_option_base().get_height())
    }

    fn set_width(&self, width: i32) {
        compat::write(self, |__t| __t.abstract_option_base_mut().set_width(width));
    }

    fn set_position(&self, x: i32, y: i32) {
        compat::write(self, |__t| __t.abstract_option_base_mut().set_position(x, y));
    }

    /// AbstractWidget.extractRenderState:56-62 - `if (visible) { isHovered = ..; extractWidgetRenderState(..) }`
    fn extract_render_state(&self, graphics: &mut GuiGraphicsExtractor, mouse_x: i32, mouse_y: i32, a: f32, origin: (i32, i32)) {
        let visible = compat::read(self, |__t| __t.abstract_option_base().visible);
        if !visible {
            return;
        }
        compat::write(self, |__t| { let __b = &mut __t.abstract_option_base_mut().base; __b.set_hovered_from_extract(mouse_x, mouse_y); });
        compat::read(self, |__t| __t.extract_widget_render_state(graphics, mouse_x, mouse_y, a, origin));
    }

    /// AbstractWidget.mouseClicked:109-125 - an active, visible widget under the mouse runs `onClick`
    fn mouse_clicked(&self, click: &MouseButtonEvent, doubled: bool) -> bool {
        let (active, visible, over) = compat::read(self, |__t| { let __b = __t.abstract_option_base(); (__b.active, __b.visible, __b.is_mouse_over(click.x(), click.y())) });
        if !active || !visible {
            return false;
        }
        if click.button() == 1 && over {
            // java: this.playDownSound(Minecraft.getInstance().getSoundManager());
            compat::batch_w_play_down_sound();
            <$o as OptionClick>::on_click(self, click, doubled);
            return true;
        }
        false
    }
}
    };
}

edit_option!([] RunnableOption);
edit_option!([] BooleanOption);
edit_option!([] ColorOption);
edit_option!([T: Clone + JavaEnum + std::fmt::Display + 'static] EnumCyclingOption<T>);

/// java: public class RunnableOption extends AbstractOption<Unit>
pub struct RunnableOption {
    /// java: extends AbstractOption
    pub base: AbstractOptionBase<Unit>,
    /// `EditBarWidget.this` - its `visible = false` (see `EditBarWidget::hide_requested`)
    pub outer_hide: std::rc::Rc<std::cell::Cell<bool>>,
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
    pub fn new(message: MutableComponent, is_enabled: Arc<dyn Fn(compat::Shared<dyn StatusBar>) -> bool>, action: Arc<dyn Fn(compat::Shared<dyn StatusBar>)>, outer_hide: std::rc::Rc<std::cell::Cell<bool>>) -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            base: AbstractOptionBase::<Unit>::new_3(message.clone(), Arc::new(move |bar| (if (is_enabled)(bar.clone()) { Some(Unit::INSTANCE) } else { None })), Arc::new(move |s, _| { (action)(s.clone()); })),
            outer_hide,
        });
        __this
    }
}

impl OptionClick for RunnableOption {
    /// java: @Override public void onClick(MouseButtonEvent click, boolean doubled)
    fn on_click(__this: &compat::Shared<Self>, _click: &MouseButtonEvent, _doubled: bool) {
        // java: super.onClick(click, doubled); - `AbstractWidget.onClick` is empty
        // java: EditBarWidget.this.visible = false;
        compat::read(__this, |__t| __t.outer_hide.set(true));
        __this.lock().borrow_mut().set_and_update(Unit::INSTANCE);
    }
}

impl AbstractOption<Unit> for RunnableOption {
    fn abstract_option_base(&self) -> &AbstractOptionBase<Unit> {
        &self.base
    }
    fn abstract_option_base_mut(&mut self) -> &mut AbstractOptionBase<Unit> {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override protected int extractValue(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_value(&self, _graphics: &mut GuiGraphicsExtractor, _mouse_x: i32, _mouse_y: i32, _a: f32) -> i32 {
        return 0;
    }

    /// java: @Override protected int expectedValueWidth()
    fn expected_value_width(&self) -> i32 {
        return 0;
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
    pub fn new(height: i32, message: MutableComponent, getter: Arc<dyn Fn(compat::Shared<dyn StatusBar>) -> Option<T>>, setter: Arc<dyn Fn(compat::Shared<dyn StatusBar>, T)>) -> Self {
        // java: super(0, 0, BASE_WIDTH, height, message);
        let mut base = AbstractWidget::new(0, 0, BASE_WIDTH, height, (&message.get_string()).to_string(), WidgetKind::PlainText, OnPress::Inert);
        base.set_message_text(FormattedText::from(message));
        Self {
            base,
            current: None,
            getter: Some(getter),
            setter: Some(setter),
            active_bar: None,
        }
    }

    /// java: private AbstractOption(Component message, Function<StatusBar, @Nullable T> getter, BiConsumer<StatusBar, T> setter)
    pub fn new_3(message: MutableComponent, getter: Arc<dyn Fn(compat::Shared<dyn StatusBar>) -> Option<T>>, setter: Arc<dyn Fn(compat::Shared<dyn StatusBar>, T)>) -> Self {
        Self::new(DEFAULT_OPTION_HEIGHT, message, getter, setter)
    }

}

/// java: the virtual methods of AbstractOption (implemented by AbstractOptionBase and every subclass)
pub trait AbstractOption<T: Clone + 'static> {
    fn abstract_option_base(&self) -> &AbstractOptionBase<T>;
    fn abstract_option_base_mut(&mut self) -> &mut AbstractOptionBase<T>;
    fn as_any(&self) -> &dyn std::any::Any;

    /// java: public void updateFromBar(StatusBar statusBar)
    fn update_from_bar(&mut self, status_bar: compat::Shared<dyn StatusBar>) {
        let apply: Option<T> = (self.abstract_option_base().getter.as_ref().unwrap())(status_bar.clone());
        if let Some(apply) = apply {
            self.abstract_option_base_mut().current = Some(apply);
            self.abstract_option_base_mut().active = true;
        } else {
            self.abstract_option_base_mut().current = None;
            self.abstract_option_base_mut().active = false;
        }
        self.abstract_option_base_mut().active_bar = Some(status_bar.clone());
    }

    /// java: protected void setAndUpdate(T value)
    fn set_and_update(&mut self, value: T) {
        self.abstract_option_base_mut().current = Some(value.clone());
        if let Some(active_bar) = self.abstract_option_base().active_bar.clone() {
            (self.abstract_option_base().setter.as_ref().unwrap())(active_bar, value.clone());
        }
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    ///
    /// (`origin`: where the widget coordinates start on the screen, for the scrolling text's scissor)
    fn extract_widget_render_state(&self, graphics: &mut GuiGraphicsExtractor, mouse_x: i32, mouse_y: i32, a: f32, origin: (i32, i32)) {
        let base = self.abstract_option_base();
        if base.is_mouse_over((mouse_x) as f64, (mouse_y) as f64) {
            graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), base.get_x(), base.get_y(), base.get_right(), base.get_bottom(), 0x20FFFFFF);
        }
        let font: Font = compat::font();
        // java: ActiveTextCollector textRenderer = graphics.textRenderer();
        let value_width: i32 = self.extract_value(graphics, mouse_x, mouse_y, a);
        let message_text: FormattedText = base.message_text.clone().unwrap_or_else(|| FormattedText::of(base.message.clone()));
        // java: Component message = getMessage().copy().withColor(active ? CommonColors.WHITE : CommonColors.GRAY);
        let message: FormattedText = FormattedText::from(compat::component_of(message_text.clone()).copy().with_color(if base.active { mcrust::net::minecraft::util::common_colors::WHITE } else { mcrust::net::minecraft::util::common_colors::GRAY }));
        let gl = mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl();
        let minecraft = &*compat::minecraft();
        if font.width_of_cached(&message_text) > ((base.get_width()).wrapping_sub(value_width)).wrapping_sub(2) {
            // java: textRenderer.acceptScrollingWithDefaultCenter(message, getX() + 1, getRight() - valueWidth - 1, getY(), getBottom());
            let width = font.width_of_cached(&message);
            let placement = active_text_collector::accept_scrolling_with_default_center((base.get_x()).wrapping_add(1), ((base.get_right()).wrapping_sub(value_width)).wrapping_sub(1), base.get_y(), base.get_bottom(), width, mcrust::net::minecraft::util::util::get_millis());
            let scissor = placement.scissor.map(|(l, r, t, b)| (l + origin.0, r + origin.0, t + origin.1, b + origin.1));
            gui_graphics_extractor::draw_clipped_formatted_text(gl, minecraft, placement.x, placement.y, &message, -1, &mcrust::net::minecraft::network::chat::style::Style::EMPTY, scissor, None, minecraft.get_window().get_gui_scale(), minecraft.get_window().get_height());
        } else {
            // java: textRenderer.accept(getX() + 1, getY() + 1, message);
            gui_graphics_extractor::draw_formatted_text_with_drop_shadow(gl, minecraft, &message, (base.get_x()).wrapping_add(1), (base.get_y()).wrapping_add(1), -1, true);
        }
    }

    /// java: protected abstract int extractValue(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a);
    fn extract_value(&self, graphics: &mut GuiGraphicsExtractor, mouse_x: i32, mouse_y: i32, a: f32) -> i32;

    /// java: protected abstract int expectedValueWidth();
    fn expected_value_width(&self) -> i32;

    /// java: public final int expectedWidth()
    fn expected_width(&self) -> i32 {
        let base = self.abstract_option_base();
        let message_text: FormattedText = base.message_text.clone().unwrap_or_else(|| FormattedText::of(base.message.clone()));
        return (self.expected_value_width().wrapping_add(compat::font().width_of_cached(&message_text))).wrapping_add(2);
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput output)
    fn update_widget_narration(&self, _output: &dyn NarrationElementOutput) {
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

impl<T: Clone + JavaEnum + std::fmt::Display + 'static> EnumCyclingOption<T> {
    /// java: public EnumCyclingOption(Component message, Class<T> enumClass, Function<StatusBar, @Nullable T> getter, BiConsumer<StatusBar, T> setter)
    /// (`Class<T>` is the type itself: `T: JavaEnum` answers `getEnumConstants()`)
    pub fn new(message: MutableComponent, getter: Arc<dyn Fn(compat::Shared<dyn StatusBar>) -> Option<T>>, setter: Arc<dyn Fn(compat::Shared<dyn StatusBar>, T)>) -> compat::Shared<Self> {
        // java: values = enumClass.getEnumConstants(); current = values[0];
        let values = T::java_values();
        let mut base = AbstractOptionBase::<T>::new_3(message, getter, setter);
        base.current = Some(values[0].clone());
        compat::shared(Self { base, values })
    }
}

impl<T: Clone + JavaEnum + std::fmt::Display + 'static> OptionClick for EnumCyclingOption<T> {
    /// java: @Override public void onClick(MouseButtonEvent click, boolean doubled)
    fn on_click(__this: &compat::Shared<Self>, _click: &MouseButtonEvent, _doubled: bool) {
        // java: setAndUpdate(current != null ? EnumUtils.cycle(current) : values[0]);
        let next: T = compat::read(__this, |__t| match &__t.current { Some(current) => EnumUtils::cycle(current), None => __t.values[0].clone() });
        __this.lock().borrow_mut().set_and_update(next);
        // java: super.onClick(click, doubled); - `AbstractWidget.onClick` is empty
    }
}

impl<T: Clone + JavaEnum + std::fmt::Display + 'static> AbstractOption<T> for EnumCyclingOption<T> {
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
    fn extract_value(&self, _graphics: &mut GuiGraphicsExtractor, _mouse_x: i32, _mouse_y: i32, _a: f32) -> i32 {
        let text_renderer: Font = compat::font();
        let string: String = (if let Some(current) = &self.current { current.to_string() } else { "???".to_string() });
        let value_width: i32 = (text_renderer.width(&string)).wrapping_add(1);
        gui_graphics_extractor::text_string_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), Some(&string), self.get_right().wrapping_sub(value_width), self.get_y().wrapping_add(1), mcrust::net::minecraft::util::common_colors::WHITE, true);
        return value_width;
    }

    /// java: @Override protected int expectedValueWidth()
    fn expected_value_width(&self) -> i32 {
        let mut m: i32 = 0;
        for value in self.values.iter() {
            let i: i32 = compat::font().width(&value.to_string());
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
    pub fn new(message: MutableComponent, getter: Arc<dyn Fn(compat::Shared<dyn StatusBar>) -> Option<bool>>, setter: Arc<dyn Fn(compat::Shared<dyn StatusBar>, bool)>) -> compat::Shared<Self> {
        compat::shared(Self {
            base: AbstractOptionBase::<bool>::new_3(message, getter, setter),
        })
    }

    /// java: public void setCurrent(boolean current)
    pub fn set_current(__this: &compat::Shared<Self>, current: bool) {
        { let __v = Some(current); compat::write(__this, |__t| __t.current = __v); }
    }

}

impl OptionClick for BooleanOption {
    /// java: @Override public void onClick(MouseButtonEvent click, boolean doubled)
    fn on_click(__this: &compat::Shared<Self>, _click: &MouseButtonEvent, _doubled: bool) {
        // java: setAndUpdate(current == null || !current);
        let next: bool = compat::read(__this, |__t| __t.current.is_none() || !__t.current.unwrap());
        __this.lock().borrow_mut().set_and_update(next);
        // java: super.onClick(click, doubled); - `AbstractWidget.onClick` is empty
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
    fn extract_value(&self, graphics: &mut GuiGraphicsExtractor, _mouse_x: i32, _mouse_y: i32, _a: f32) -> i32 {
        GuiHelper::border(graphics, self.get_right().wrapping_sub(10), self.get_y().wrapping_add(1), 9, 9, (if self.active { -1 } else { mcrust::net::minecraft::util::common_colors::GRAY }));
        if self.active && Some(true) == self.current {
            graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), self.get_right().wrapping_sub(8), self.get_y().wrapping_add(3), self.get_right().wrapping_sub(3), self.get_y().wrapping_add(8), mcrust::net::minecraft::util::common_colors::WHITE);
        }
        return 10;
    }

    /// java: @Override protected int expectedValueWidth()
    fn expected_value_width(&self) -> i32 {
        return 10;
    }

}

/// java: public static class ColorOption extends AbstractOption<Color>
pub struct ColorOption {
    /// java: extends AbstractOption
    pub base: AbstractOptionBase<Color>,
    /// java: private final Screen parent; - the config screen the colour popup is opened over (the game holds it
    /// under the popup)
    pub parent: compat::WeakShared<StatusBarsConfigScreen>,
    /// the object's own shared handle (`this::setAndUpdate` handed to the popup)
    pub self_ref: compat::WeakShared<Self>,
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
    pub fn new(message: MutableComponent, parent: compat::WeakShared<StatusBarsConfigScreen>, getter: Arc<dyn Fn(compat::Shared<dyn StatusBar>) -> Option<Color>>, setter: Arc<dyn Fn(compat::Shared<dyn StatusBar>, Color)>) -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            base: AbstractOptionBase::<Color>::new_3(message, getter, setter),
            parent,
            self_ref: std::sync::Weak::new(),
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        __this
    }
}

impl OptionClick for ColorOption {
    /// java: @Override public void onClick(MouseButtonEvent click, boolean doubled)
    fn on_click(__this: &compat::Shared<Self>, _click: &MouseButtonEvent, _doubled: bool) {
        // java: super.onClick(click, doubled); - `AbstractWidget.onClick` is empty
        // java: Minecraft.getInstance().gui.setScreen(new EditBarColorPopup(Component.literal("Edit ").append(getMessage()), parent, this::setAndUpdate, current != null ? current.getRGB() : -1));
        let (message, current, self_ref) = compat::read(__this, |__t| (__t.message_text.clone().unwrap_or_else(|| FormattedText::of(__t.message.clone())), __t.current.clone(), __t.self_ref.clone()));
        let title = component::literal("Edit ").append(compat::component_of(message));
        let set_color: Arc<dyn Fn(Color)> = {
            let self_ref = compat::RenderThread::new(self_ref);
            Arc::new(move |color: Color| {
                if let Some(this) = self_ref.get().upgrade() {
                    this.lock().borrow_mut().set_and_update(color);
                }
            })
        };
        let initial_color: i32 = current.map(|__c| __c.get_rgb()).unwrap_or(-1);
        let popup = EditBarColorPopup::new(title, mcrust::net::minecraft::client::gui::screens::screen::Screen::new(String::new()), set_color, initial_color);
        mcrust::mcrust::modding::game::set_screen_keeping_parent(Box::new(compat::ScreenAdapter::new(popup)));
    }
}

impl AbstractOption<Color> for ColorOption {
    fn abstract_option_base(&self) -> &AbstractOptionBase<Color> {
        &self.base
    }
    fn abstract_option_base_mut(&mut self) -> &mut AbstractOptionBase<Color> {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override protected int extractValue(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_value(&self, graphics: &mut GuiGraphicsExtractor, _mouse_x: i32, _mouse_y: i32, _a: f32) -> i32 {
        GuiHelper::border(graphics, self.get_right().wrapping_sub(10), self.get_y().wrapping_add(1), 9, 9, (if self.active { -1 } else { mcrust::net::minecraft::util::common_colors::GRAY }));
        // java: graphics.fill(getRight() - 8, getY() + 3, getRight() - 3, getY() + 8, active && current != null ? current.getRGB() : CommonColors.GRAY);
        graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), self.get_right().wrapping_sub(8), self.get_y().wrapping_add(3), self.get_right().wrapping_sub(3), self.get_y().wrapping_add(8), match (&self.current, self.active) { (Some(current), true) => current.get_rgb(), _ => mcrust::net::minecraft::util::common_colors::GRAY });
        return 10;
    }

    /// java: @Override protected int expectedValueWidth()
    fn expected_value_width(&self) -> i32 {
        return 10;
    }

}

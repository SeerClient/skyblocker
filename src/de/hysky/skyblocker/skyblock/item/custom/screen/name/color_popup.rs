//! Ported from de.hysky.skyblocker.skyblock.item.custom.screen.name.ColorPopup by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::render::gui::abstract_popup_screen::AbstractPopupScreen;
use crate::de::hysky::skyblocker::utils::render::gui::abstract_popup_screen::AbstractPopupScreenBase;
use crate::de::hysky::skyblocker::utils::render::gui::argb_text_input::ARGBTextInput;
use crate::de::hysky::skyblocker::utils::render::gui::color_picker_widget::ColorPickerWidget;
use mcrust::mcrust::modding::screens::ModScreen;
use mcrust::mcrust::modding::screens::ScreenRenderContext;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::PressLambda;
use mcrust::net::minecraft::client::gui::components::button;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::layouts::grid_layout::GridLayout;
use mcrust::net::minecraft::client::gui::layouts::grid_layout::RowHelper;
use mcrust::net::minecraft::client::gui::layouts::layout_element::LayoutElement;
use mcrust::net::minecraft::client::gui::layouts::layout_settings::LayoutSettings;
use mcrust::net::minecraft::client::gui::layouts::spacer_element::SpacerElement;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::client::input::mouse_button_info::MouseButtonInfo;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::client::gui::layouts::abstract_layout::AbstractLayout as _;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: public class ColorPopup extends AbstractPopupScreen
pub struct ColorPopup {
    /// java: extends AbstractPopupScreen
    pub base: AbstractPopupScreenBase,
    /// java: private final GridLayout layout = new GridLayout();
    pub layout: GridLayout,
    /// java: private final boolean gradient;
    pub gradient: bool,
    /// java: private final GradientConsumer gradientConsumer;
    pub gradient_consumer: Option<GradientConsumer>,
    /// java: private final IntIntPair currentColor = new IntIntMutablePair(-1, -1);
    pub current_color: (i32, i32),
}

impl std::ops::Deref for ColorPopup {
    type Target = AbstractPopupScreenBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for ColorPopup {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl ColorPopup {
    /// java: private ColorPopup(Screen backgroundScreen, GradientConsumer gradientConsumer, boolean gradient)
    pub fn new(mut background_screen: mcrust::net::minecraft::client::gui::screens::screen::Screen, gradient_consumer: GradientConsumer, mut gradient: bool) -> compat::Shared<Self> {
        // java: super(Component.literal("Color Popup"), backgroundScreen);
        let (_, __this) = AbstractPopupScreenBase::new_in(component::literal("Color Popup"), background_screen, move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                layout: GridLayout::new_default(),
                gradient,
                gradient_consumer: Some(gradient_consumer),
                current_color: (-1, -1),
            });
            ((__h.clone() as compat::Shared<dyn AbstractPopupScreen>), __h)
        });
        // java: layout.defaultCellSetting().alignHorizontallyCenter(); - on the layout's own settings
        compat::write(&__this, |__t| {
            let settings = __t.layout.default_cell_setting();
            *settings = settings.clone().align_horizontally_center();
        });
        __this
    }

    /// java: private ColorPopup(Screen backgroundScreen, IntConsumer consumer)
    pub fn new_2(mut background_screen: mcrust::net::minecraft::client::gui::screens::screen::Screen, consumer: Arc<dyn Fn(i32)>) -> compat::Shared<Self> {
        // java: this(backgroundScreen, ((start, _) -> consumer.accept(start)), false);
        ColorPopup::new(background_screen, Arc::new(move |start: i32, _: i32| consumer(start)), false)
    }

    /// java: public static ColorPopup create(Screen backgroundScreen, IntConsumer colorConsumer)
    pub fn create(background_screen: mcrust::net::minecraft::client::gui::screens::screen::Screen, color_consumer: Arc<dyn Fn(i32)>) -> compat::Shared<ColorPopup> {
        return ColorPopup::new_2(background_screen, color_consumer);
    }

    /// java: public static ColorPopup createGradient(Screen backgroundScreen, GradientConsumer gradientConsumer)
    pub fn create_gradient(background_screen: mcrust::net::minecraft::client::gui::screens::screen::Screen, gradient_consumer: GradientConsumer) -> compat::Shared<ColorPopup> {
        return ColorPopup::new(background_screen, gradient_consumer, true);
    }

    /// `addRenderableWidget(widget)`: the widget joins this screen, and the layout gets its leaf (the port's layouts
    /// hold a widget by its place in the screen)
    fn renderable_element(__this: &compat::Shared<Self>, widget: mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget) -> LayoutElement {
        let (w, h) = (widget.get_width(), widget.get_height());
        LayoutElement::widget(compat::write(__this, |__t| __t.add_renderable_widget(widget)), w, h)
    }

    /// `adder.addChild(child, columnWidth[, settings])` on this popup's `layout` (the port's RowHelper is handed its grid)
    fn grid_add(__this: &compat::Shared<Self>, adder: &mut RowHelper, child: LayoutElement, column_width: i32, settings: Option<LayoutSettings>) {
        compat::write(__this, |__t| match settings {
            None => adder.add_child_width(&mut __t.layout, child, column_width),
            Some(settings) => adder.add_child_spanning(&mut __t.layout, child, column_width, settings),
        })
        .expect("GridLayout.addChild");
    }

    /// java: private void createLayout(GridLayout.RowHelper adder)
    pub fn create_layout(__this: &compat::Shared<Self>, adder: &mut RowHelper) {
        let mut color_picker: compat::Shared<ColorPickerWidget> = ColorPickerWidget::new(0, 0, 200, 100);
        let mut argb: compat::Shared<ARGBTextInput> = ARGBTextInput::new(0, 0, compat::font(), true, false);
        let __color_picker_element: LayoutElement = { let __w = compat::WidgetAdapter::widget(color_picker.clone()); let (__ww, __wh) = (__w.get_width(), __w.get_height()); LayoutElement::widget(compat::write(__this, |__t| __t.add_renderable_widget(__w)), __ww, __wh) };
        let __argb_element: LayoutElement = { let __w = compat::WidgetAdapter::widget(argb.clone()); let (__ww, __wh) = (__w.get_width(), __w.get_height()); LayoutElement::widget(compat::write(__this, |__t| __t.add_renderable_widget(__w)), __ww, __wh) };
        let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        let __color_picker = compat::RenderThread::new(std::sync::Arc::downgrade(&color_picker)); // captured weakly: the two widgets refer to each other
        ARGBTextInput::set_on_change(&argb, Arc::new(move |color| { let Some(__this_1) = __this_w1.get().upgrade() else { return Default::default(); }; {
            if let Some(__color_picker) = __color_picker.get().upgrade() { ColorPickerWidget::set_argb_color(&__color_picker, color); }
            compat::write((&__this_1), |__t| __t.current_color.0 = color); // currentColor.first(color)
        } }));
        let __this_w2 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        let __argb = compat::RenderThread::new(std::sync::Arc::downgrade(&argb)); // captured weakly: the two widgets refer to each other
        ColorPickerWidget::set_on_color_change(&color_picker, Arc::new(move |color, _| { let Some(__this_2) = __this_w2.get().upgrade() else { return Default::default(); }; {
            if let Some(__argb) = __argb.get().upgrade() { ARGBTextInput::set_argb_color(&__argb, color); }
            compat::write((&__this_2), |__t| __t.current_color.0 = color); // currentColor.first(color)
        } }));
        ColorPopup::grid_add(__this, adder, __color_picker_element, 2, None);
        ColorPopup::grid_add(__this, adder, __argb_element, 2, None);
    }

    /// java: private void createLayoutGradient(GridLayout.RowHelper adder)
    pub fn create_layout_gradient(__this: &compat::Shared<Self>, adder: &mut RowHelper) {
        let mut color_picker_start: compat::Shared<ColorPickerWidget> = ColorPickerWidget::new(0, 0, 200, 100);
        let mut argb_start: compat::Shared<ARGBTextInput> = ARGBTextInput::new(0, 0, compat::font(), true, false);
        let mut color_picker_end: compat::Shared<ColorPickerWidget> = ColorPickerWidget::new(0, 0, 200, 100);
        let mut argb_end: compat::Shared<ARGBTextInput> = ARGBTextInput::new(0, 0, compat::font(), true, false);
        let __color_picker_start_element: LayoutElement = { let __w = compat::WidgetAdapter::widget(color_picker_start.clone()); let (__ww, __wh) = (__w.get_width(), __w.get_height()); LayoutElement::widget(compat::write(__this, |__t| __t.add_renderable_widget(__w)), __ww, __wh) };
        let __argb_start_element: LayoutElement = { let __w = compat::WidgetAdapter::widget(argb_start.clone()); let (__ww, __wh) = (__w.get_width(), __w.get_height()); LayoutElement::widget(compat::write(__this, |__t| __t.add_renderable_widget(__w)), __ww, __wh) };
        let __color_picker_end_element: LayoutElement = { let __w = compat::WidgetAdapter::widget(color_picker_end.clone()); let (__ww, __wh) = (__w.get_width(), __w.get_height()); LayoutElement::widget(compat::write(__this, |__t| __t.add_renderable_widget(__w)), __ww, __wh) };
        let __argb_end_element: LayoutElement = { let __w = compat::WidgetAdapter::widget(argb_end.clone()); let (__ww, __wh) = (__w.get_width(), __w.get_height()); LayoutElement::widget(compat::write(__this, |__t| __t.add_renderable_widget(__w)), __ww, __wh) };
        let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        let __color_picker_start = compat::RenderThread::new(std::sync::Arc::downgrade(&color_picker_start)); // captured weakly: the two widgets refer to each other
        ARGBTextInput::set_on_change(&argb_start, Arc::new(move |color| { let Some(__this_1) = __this_w1.get().upgrade() else { return Default::default(); }; {
            if let Some(__color_picker_start) = __color_picker_start.get().upgrade() { ColorPickerWidget::set_argb_color(&__color_picker_start, color); }
            compat::write((&__this_1), |__t| __t.current_color.0 = color); // currentColor.first(color)
        } }));
        let __this_w2 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        let __argb_start = compat::RenderThread::new(std::sync::Arc::downgrade(&argb_start)); // captured weakly: the two widgets refer to each other
        ColorPickerWidget::set_on_color_change(&color_picker_start, Arc::new(move |color, _| { let Some(__this_2) = __this_w2.get().upgrade() else { return Default::default(); }; {
            if let Some(__argb_start) = __argb_start.get().upgrade() { ARGBTextInput::set_argb_color(&__argb_start, color); }
            compat::write((&__this_2), |__t| __t.current_color.0 = color); // currentColor.first(color)
        } }));
        let __this_w3 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        let __color_picker_end = compat::RenderThread::new(std::sync::Arc::downgrade(&color_picker_end)); // captured weakly: the two widgets refer to each other
        ARGBTextInput::set_on_change(&argb_end, Arc::new(move |color| { let Some(__this_3) = __this_w3.get().upgrade() else { return Default::default(); }; {
            if let Some(__color_picker_end) = __color_picker_end.get().upgrade() { ColorPickerWidget::set_argb_color(&__color_picker_end, color); }
            compat::write((&__this_3), |__t| __t.current_color.1 = color); // currentColor.second(color)
        } }));
        let __this_w4 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        let __argb_end = compat::RenderThread::new(std::sync::Arc::downgrade(&argb_end)); // captured weakly: the two widgets refer to each other
        ColorPickerWidget::set_on_color_change(&color_picker_end, Arc::new(move |color, _| { let Some(__this_4) = __this_w4.get().upgrade() else { return Default::default(); }; {
            if let Some(__argb_end) = __argb_end.get().upgrade() { ARGBTextInput::set_argb_color(&__argb_end, color); }
            compat::write((&__this_4), |__t| __t.current_color.1 = color); // currentColor.second(color)
        } }));
        { let __e = ColorPopup::renderable_element(__this, compat::string_widget(0, 0, -1, 9, component::translatable("skyblocker.customItemNames.screen.gradientStart"))); ColorPopup::grid_add(__this, adder, __e, 1, None); }
        { let __e = ColorPopup::renderable_element(__this, compat::string_widget(0, 0, -1, 9, component::translatable("skyblocker.customItemNames.screen.gradientEnd"))); ColorPopup::grid_add(__this, adder, __e, 1, None); }
        ColorPopup::grid_add(__this, adder, __color_picker_start_element, 1, None);
        ColorPopup::grid_add(__this, adder, __color_picker_end_element, 1, None);
        ColorPopup::grid_add(__this, adder, __argb_start_element, 1, None);
        ColorPopup::grid_add(__this, adder, __argb_end_element, 1, None);
    }

}

impl AbstractPopupScreen for ColorPopup {
    fn abstract_popup_screen_base(&self) -> &AbstractPopupScreenBase {
        &self.base
    }
    fn abstract_popup_screen_base_mut(&mut self) -> &mut AbstractPopupScreenBase {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override protected void init() (dispatch: the body for this class)
    fn init(&self) -> fn(&compat::Shared<dyn AbstractPopupScreen>) {
        ColorPopup::init_dyn
    }

    /// java: @Override protected void repositionElements() (dispatch: the body for this class)
    fn reposition_elements(&self) -> fn(&compat::Shared<dyn AbstractPopupScreen>) {
        ColorPopup::reposition_elements_dyn
    }

    /// java: @Override public void extractBackground(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a) (dispatch: the body for this class)
    fn extract_background(&self) -> fn(&compat::Shared<dyn AbstractPopupScreen>, &mut GuiGraphicsExtractor, i32, i32, f32) {
        ColorPopup::extract_background_dyn
    }

}

impl ColorPopup {
    /// java: @Override protected void init()
    pub fn init_impl(__this: &compat::Shared<Self>) {
        let mut adder: RowHelper = __this.lock().borrow().layout.create_row_helper(2);
        { let __e = ColorPopup::renderable_element(__this, compat::string_widget(0, 0, -1, 9, component::translatable("skyblocker.customItemNames.screen.customColorTitle"))); ColorPopup::grid_add(__this, &mut adder, __e, 2, None); }
        if compat::read(__this, |__t| __t.gradient) {
            ColorPopup::create_layout_gradient(__this, &mut adder);
        } else {
            ColorPopup::create_layout(__this, &mut adder);
        }
        ColorPopup::grid_add(__this, &mut adder, SpacerElement::height(15).as_layout_element(), 2, None);
        let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        { let __e = ColorPopup::renderable_element(__this, button::builder(component::translatable("gui.cancel").get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { let Some(__this_1) = __this_w1.get().upgrade() else { return Default::default(); }; compat::lend_screen(&__this_1, __screen, || { ({ let __f = compat::dispatch((&__this_1), |__t| __t.on_close()); __f(&compat::view((&__this_1), |a| a as compat::Shared<dyn AbstractPopupScreen>)) }); }) }))).build()); ColorPopup::grid_add(__this, &mut adder, __e, 1, Some(LayoutSettings::defaults().align_horizontally_right().padding_right(2))); }
        let __this_w2 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        { let __e = ColorPopup::renderable_element(__this, button::builder(component::translatable("gui.done").get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { let Some(__this_2) = __this_w2.get().upgrade() else { return Default::default(); }; compat::lend_screen(&__this_2, __screen, || {
            let (__first, __second) = compat::read((&__this_2), |__t| __t.current_color);
            (compat::read((&__this_2), |__t| __t.gradient_consumer.clone()).unwrap())(__first, __second);
            ({ let __f = compat::dispatch((&__this_2), |__t| __t.on_close()); __f(&compat::view((&__this_2), |a| a as compat::Shared<dyn AbstractPopupScreen>)) });
        }) }))).build()); ColorPopup::grid_add(__this, &mut adder, __e, 1, Some(LayoutSettings::defaults().align_horizontally_left().padding_left(2))); }
        <dyn AbstractPopupScreen>::init_body(&compat::view(__this, |a| a as compat::Shared<dyn AbstractPopupScreen>));
    }

    /// ColorPopup.init as the dispatch of AbstractPopupScreen.init
    pub fn init_dyn(__this: &compat::Shared<dyn AbstractPopupScreen>) {
        Self::init_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override protected void repositionElements()
    pub fn reposition_elements_impl(__this: &compat::Shared<Self>) {
        <dyn AbstractPopupScreen>::reposition_elements_body(&compat::view(__this, |a| a as compat::Shared<dyn AbstractPopupScreen>));
        { __this.lock().borrow_mut().layout.arrange_elements(); let mut __vw = Vec::new(); LayoutElement::Grid(__this.lock().borrow().layout.clone()).visit_widgets(&mut __vw); for (__i, __x, __y) in __vw { compat::write(__this, |__t| __t.widget_mut(__i).set_position(__x, __y)); } };
        // java: layout.setPosition((width - layout.getWidth()) / 2, (height - layout.getHeight()) / 2); - and the widgets
        // follow the layout (the port's layouts hold a widget by its place in the screen)
        compat::write(__this, |__t| {
            let (__width, __height) = (__t.width, __t.height);
            let mut __element = LayoutElement::Grid(__t.layout.clone());
            let (__x, __y) = ((__width.wrapping_sub(__element.get_width())).wrapping_div(2), (__height.wrapping_sub(__element.get_height())).wrapping_div(2));
            __element.set_position(__x, __y);
            let mut __vw = Vec::new();
            __element.visit_widgets(&mut __vw);
            if let LayoutElement::Grid(__grid) = __element { __t.layout = __grid; }
            for (__i, __wx, __wy) in __vw { __t.widget_mut(__i).set_position(__wx, __wy); }
        });
    }

    /// ColorPopup.repositionElements as the dispatch of AbstractPopupScreen.repositionElements
    pub fn reposition_elements_dyn(__this: &compat::Shared<dyn AbstractPopupScreen>) {
        Self::reposition_elements_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public void extractBackground(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_background_impl(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        <dyn AbstractPopupScreen>::extract_background_body(&compat::view(__this, |a| a as compat::Shared<dyn AbstractPopupScreen>), graphics, mouse_x, mouse_y, a);
        AbstractPopupScreenBase::extract_popup_background(graphics, __this.lock().borrow().layout.get_x(), __this.lock().borrow().layout.get_y(), __this.lock().borrow().layout.get_width(), __this.lock().borrow().layout.get_height());
    }

    /// ColorPopup.extractBackground as the dispatch of AbstractPopupScreen.extractBackground
    pub fn extract_background_dyn(__this: &compat::Shared<dyn AbstractPopupScreen>, __p0: &mut GuiGraphicsExtractor, __p1: i32, __p2: i32, __p3: f32) {
        Self::extract_background_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2, __p3)
    }

}

/// ColorPopup's `Screen` half (`super`'s fields and widget tree), lent by the game for each call
impl compat::ScreenHalf for ColorPopup {
    fn screen_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::screens::screen::Screen {
        &mut self.base.base
    }
    fn screen_parts(&mut self) -> (&mut mcrust::net::minecraft::client::gui::screens::screen::Screen, Option<&mut compat::WidgetCache>) {
        let __s = &mut self.base;
        (&mut __s.base, Some(&mut __s.__widgets))
    }
}

/// ColorPopup as the game's screen: `game::set_screen(Box::new(compat::ScreenAdapter::new(..)))`
impl ModScreen for compat::ScreenAdapter<ColorPopup> {
    /// java: AbstractPopupScreen.extractBackground overrides `Screen.extractBackground` without `super`
    fn extract_background_calls_super(&self) -> bool {
        false
    }

    /// java: @Override public void extractBackground(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_background(&mut self, _screen: &mcrust::net::minecraft::client::gui::screens::screen::Screen, context: &mut mcrust::mcrust::modding::screens::ScreenRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.call(|__h| ({ let __f = compat::dispatch(__h, |__t| __t.extract_background()); __f(&compat::view(__h, |a| a as compat::Shared<dyn AbstractPopupScreen>), &mut context.graphics, mouse_x, mouse_y, a) }))
    }

    fn get_title(&self) -> String {
        self.title()
    }

    /// java: @Override protected void init()
    fn init(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        self.lend(screen, |__h| ({ let __f = compat::dispatch(__h, |__t| __t.init()); __f(&compat::view(__h, |a| a as compat::Shared<dyn AbstractPopupScreen>)) }))
    }

    /// java: @Override protected void repositionElements()
    fn reposition_elements(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        self.lend(screen, |__h| ({ let __f = compat::dispatch(__h, |__t| __t.reposition_elements()); __f(&compat::view(__h, |a| a as compat::Shared<dyn AbstractPopupScreen>)) }))
    }

    /// java: Screen.tick (the base body)
    fn tick(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        self.lend(screen, |_| ())
    }

    /// java: @Override public void added()
    fn added(&mut self) {
        self.call(|__h| ({ let __f = compat::dispatch(__h, |__t| __t.added()); __f(&compat::view(__h, |a| a as compat::Shared<dyn AbstractPopupScreen>)) }))
    }

    /// java: Screen.keyPressed (the base body)
    fn key_pressed(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, event: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        self.lend(screen, |_| ());
        if event.is_escape() && self.should_close_on_esc() {
            self.on_close();
            return true;
        }
        false
    }

    /// java: Screen.mouseClicked (the base body)
    fn mouse_clicked(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, x: f64, y: f64, button: i32) -> bool {
        self.lend(screen, |_| false)
    }

    /// java: @Override public void onClose()
    fn on_close(&mut self) {
        self.call(|__h| ({ let __f = compat::dispatch(__h, |__t| __t.on_close()); __f(&compat::view(__h, |a| a as compat::Shared<dyn AbstractPopupScreen>)) }))
    }

}

/// java: @FunctionalInterface public interface GradientConsumer
/// a functional interface: `accept(int start, int end)` - a lambda / method reference is this closure type
pub type GradientConsumer = Arc<dyn Fn(i32, i32)>;

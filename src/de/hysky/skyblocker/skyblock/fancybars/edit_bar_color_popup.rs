//! Ported from de.hysky.skyblocker.skyblock.fancybars.EditBarColorPopup by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::render::gui::abstract_popup_screen::AbstractPopupScreen;
use crate::de::hysky::skyblocker::utils::render::gui::abstract_popup_screen::AbstractPopupScreenBase;
use crate::de::hysky::skyblocker::utils::render::gui::argb_text_input::ARGBTextInput;
use crate::de::hysky::skyblocker::utils::render::gui::color_picker_widget::ColorPickerWidget;
use mcrust::java::awt::color::Color;
use mcrust::mcrust::modding::screens::ModScreen;
use mcrust::mcrust::modding::screens::ScreenRenderContext;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::PressLambda;
use mcrust::net::minecraft::client::gui::components::button;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::layouts::frame_layout;
use mcrust::net::minecraft::client::gui::layouts::layout_element::LayoutElement;
use mcrust::net::minecraft::client::gui::layouts::linear_layout::LinearLayout;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::client::input::mouse_button_info::MouseButtonInfo;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: public class EditBarColorPopup extends AbstractPopupScreen
pub struct EditBarColorPopup {
    /// java: extends AbstractPopupScreen
    pub base: AbstractPopupScreenBase,
    /// java: private final Consumer<Color> setColor;
    pub set_color: Arc<dyn Fn(Color)>,
    /// java: private int currentColor = -1;
    pub current_color: i32,
    /// java: private LinearLayout layout = LinearLayout.vertical();
    pub layout: LinearLayout,
}

impl std::ops::Deref for EditBarColorPopup {
    type Target = AbstractPopupScreenBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for EditBarColorPopup {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl EditBarColorPopup {
    /// java: protected EditBarColorPopup(Component title, Screen backgroundScreen, Consumer<Color> setColor, int initialColor)
    pub fn new(mut title: MutableComponent, mut background_screen: mcrust::net::minecraft::client::gui::screens::screen::Screen, set_color: Arc<dyn Fn(Color)>, mut initial_color: i32) -> compat::Shared<Self> {
        let __init_set_color = set_color.clone();
        let __init_current_color = initial_color;
        // java: super(title, backgroundScreen); (the base keeps a stand-in: the held screen is the one the popup is
        // opened over, see `AbstractPopupScreenBase::new_in`)
        let (_, __this) = AbstractPopupScreenBase::new_in(title, background_screen, move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                set_color: __init_set_color,
                current_color: __init_current_color,
                layout: LinearLayout::vertical(),
            });
            ((__h.clone() as compat::Shared<dyn AbstractPopupScreen>), __h)
        });
        __this
    }

}

impl AbstractPopupScreen for EditBarColorPopup {
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
        EditBarColorPopup::init_dyn
    }

    /// java: @Override public void extractBackground(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a) (dispatch: the body for this class)
    fn extract_background(&self) -> fn(&compat::Shared<dyn AbstractPopupScreen>, &mut GuiGraphicsExtractor, i32, i32, f32) {
        EditBarColorPopup::extract_background_dyn
    }

}

impl EditBarColorPopup {
    /// java: @Override protected void init()
    pub fn init_impl(__this: &compat::Shared<Self>) {
        <dyn AbstractPopupScreen>::init_body(&compat::view(__this, |a| a as compat::Shared<dyn AbstractPopupScreen>));
        { let __v = LinearLayout::vertical(); compat::write(__this, |__t| __t.layout = __v); }
        // java: layout.spacing(8).defaultCellSetting().alignHorizontallyCenter(); - `spacing` answers `this`
        compat::write(__this, |__t| {
            let layout = std::mem::replace(&mut __t.layout, LinearLayout::vertical());
            __t.layout = layout.spacing(8);
            // (the port's `LayoutSettings` setters answer a new value: written back to the cell defaults)
            { let __s = __t.layout.default_cell_setting(); *__s = (*__s).align_horizontally_center(); }
        });
        { let (__q0,) = ({ let __w = compat::string_widget(0, 0, -1, 9, { let (__q0,) = (&mcrust::net::minecraft::network::chat::style::Style::EMPTY.with_bold(Some(true)),); compat::IntoComponent::into_component(compat::read(__this, |__t| __t.title.clone())).with_style(__q0) }); let (__ww, __wh) = (__w.get_width(), __w.get_height()); LayoutElement::widget(compat::write(__this, |__t| __t.add_widget(__w)), __ww, __wh) },); __this.lock().borrow_mut().layout.add_child(__q0) };
        // java: LinearLayout colorLayout = layout.addChild(LinearLayout.horizontal().spacing(4)); - the port's layouts are
        // values, so the child is filled first and added (in the same place among `layout`'s children) once complete
        let mut color_layout: LinearLayout = LinearLayout::horizontal().spacing(4);
        let mut color_picker: compat::Shared<ColorPickerWidget> = ColorPickerWidget::new(0, 0, 200, 100);
        let mut argb: compat::Shared<ARGBTextInput> = ARGBTextInput::new(0, 0, compat::font(), true, false);
        ColorPickerWidget::set_argb_color(&color_picker, compat::read(__this, |__t| __t.current_color));
        ARGBTextInput::set_argb_color(&argb, compat::read(__this, |__t| __t.current_color));
        let __this_1 = compat::RenderThread::new(__this.clone());
        // (the two widgets' callbacks reach each other: each holds the other's Weak, so neither keeps the other alive)
        let color_picker_w = compat::RenderThread::new(std::sync::Arc::downgrade(&color_picker));
        let argb_w = compat::RenderThread::new(std::sync::Arc::downgrade(&argb));
        ARGBTextInput::set_on_change(&argb, Arc::new(move |color| {
            if let Some(color_picker) = color_picker_w.get().upgrade() { ColorPickerWidget::set_argb_color(&color_picker, color); }
            { let __v = color; compat::write(__this_1.get(), |__t| __t.current_color = __v); }
        }));
        let __this_2 = compat::RenderThread::new(__this.clone());
        ColorPickerWidget::set_on_color_change(&color_picker, Arc::new(move |color, _| {
            if let Some(argb) = argb_w.get().upgrade() { ARGBTextInput::set_argb_color(&argb, color); }
            { let __v = color; compat::write(__this_2.get(), |__t| __t.current_color = __v); }
        }));
        color_layout.add_child({ let __w = compat::WidgetAdapter::widget(color_picker.clone()); let (__ww, __wh) = (__w.get_width(), __w.get_height()); LayoutElement::widget(compat::write(__this, |__t| __t.add_widget(__w)), __ww, __wh) });
        color_layout.add_child({ let __w = compat::WidgetAdapter::widget(argb.clone()); let (__ww, __wh) = (__w.get_width(), __w.get_height()); LayoutElement::widget(compat::write(__this, |__t| __t.add_widget(__w)), __ww, __wh) });
        { let (__q0,) = (LayoutElement::Grid(color_layout.wrapped.clone()),); __this.lock().borrow_mut().layout.add_child(__q0) };
        let mut horizontal: LinearLayout = LinearLayout::horizontal();
        let __this_w3 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        let mut button_widget: AbstractWidget = ((button::builder(component::literal("Cancel").get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { let Some(__this_3) = __this_w3.get().upgrade() else { return Default::default(); }; compat::lend_screen(&__this_3, __screen, || { let mut __pidx = compat::WidgetRef::Screen(__index); { ({ let __f = compat::dispatch((&__this_3), |__t| __t.on_close()); __f(&compat::view((&__this_3), |a| a as compat::Shared<dyn AbstractPopupScreen>)) }); } }) })))).width(80)).build();
        horizontal.add_child({ let __w = button_widget.clone(); let (__ww, __wh) = (__w.get_width(), __w.get_height()); LayoutElement::widget(compat::write(__this, |__t| __t.add_widget(__w)), __ww, __wh) });
        let __this_w4 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        horizontal.add_child({ let __w = ((button::builder(component::literal("Done").get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { let Some(__this_4) = __this_w4.get().upgrade() else { return Default::default(); }; compat::lend_screen(&__this_4, __screen, || { let mut __pidx = compat::WidgetRef::Screen(__index); {
            // java: setColor.accept(new Color(currentColor));
            ((compat::read((&__this_4), |__t| __t.set_color.clone())))(Color::from_rgb(compat::read((&__this_4), |__t| __t.current_color)));
            ({ let __f = compat::dispatch((&__this_4), |__t| __t.on_close()); __f(&compat::view((&__this_4), |a| a as compat::Shared<dyn AbstractPopupScreen>)) });
        } }) })))).width(80)).build(); let (__ww, __wh) = (__w.get_width(), __w.get_height()); LayoutElement::widget(compat::write(__this, |__t| __t.add_widget(__w)), __ww, __wh) });
        { let (__q0,) = (LayoutElement::Grid(horizontal.clone().wrapped),); __this.lock().borrow_mut().layout.add_child(__q0) };
        { let mut __vw = Vec::new(); LayoutElement::Grid(__this.lock().borrow().layout.wrapped.clone()).visit_widgets(&mut __vw); for (__i, _, _) in __vw { compat::write(__this, |__t| __t.mark_renderable(__i)); } };
        // java: this.layout.arrangeElements(); FrameLayout.centerInRectangle(layout, this.getRectangle()); - the layout's
        // widgets take their positions from the centred layout
        {
            let mut layout: LinearLayout = compat::read(__this, |__t| __t.layout.clone());
            layout.arrange_elements();
            let rectangle = compat::read(__this, |__t| __t.get_rectangle());
            frame_layout::center_in_rectangle_of(&mut *compat::LayoutBack::grid(&mut layout.wrapped), rectangle);
            let mut __vw = Vec::new();
            LayoutElement::Grid(layout.wrapped.clone()).visit_widgets(&mut __vw);
            for (__i, __x, __y) in __vw {
                compat::write(__this, |__t| __t.widget_mut(__i).set_position(__x, __y));
            }
            compat::write(__this, |__t| __t.layout = layout);
        }
    }

    /// EditBarColorPopup.init as the dispatch of AbstractPopupScreen.init
    pub fn init_dyn(__this: &compat::Shared<dyn AbstractPopupScreen>) {
        Self::init_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public void extractBackground(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_background_impl(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        <dyn AbstractPopupScreen>::extract_background_body(&compat::view(__this, |a| a as compat::Shared<dyn AbstractPopupScreen>), graphics, mouse_x, mouse_y, a);
        AbstractPopupScreenBase::extract_popup_background(graphics, __this.lock().borrow().layout.get_x(), __this.lock().borrow().layout.get_y(), __this.lock().borrow().layout.get_width(), __this.lock().borrow().layout.get_height());
    }

    /// EditBarColorPopup.extractBackground as the dispatch of AbstractPopupScreen.extractBackground
    pub fn extract_background_dyn(__this: &compat::Shared<dyn AbstractPopupScreen>, __p0: &mut GuiGraphicsExtractor, __p1: i32, __p2: i32, __p3: f32) {
        Self::extract_background_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2, __p3)
    }

}

/// EditBarColorPopup's `Screen` half (`super`'s fields and widget tree), lent by the game for each call
impl compat::ScreenHalf for EditBarColorPopup {
    fn screen_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::screens::screen::Screen {
        &mut self.base.base
    }
    fn screen_parts(&mut self) -> (&mut mcrust::net::minecraft::client::gui::screens::screen::Screen, Option<&mut compat::WidgetCache>) {
        let __s = &mut self.base;
        (&mut __s.base, Some(&mut __s.__widgets))
    }
}

/// EditBarColorPopup as the game's screen: `game::set_screen(Box::new(compat::ScreenAdapter::new(..)))`
impl ModScreen for compat::ScreenAdapter<EditBarColorPopup> {
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

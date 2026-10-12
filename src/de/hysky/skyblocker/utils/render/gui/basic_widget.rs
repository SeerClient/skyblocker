//! Ported from de.hysky.skyblocker.utils.render.gui.BasicWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::layouts::layout_element::LayoutElement;
use mcrust::net::minecraft::client::gui::navigation::screen_rectangle::ScreenRectangle;

/// java: public abstract class BasicWidget implements LayoutElement, GuiEventListener, Renderable
#[derive(Clone)]
pub struct BasicWidgetBase {
    /// java: protected int w = 0, h = 0;
    pub w: i32,
    /// java: protected int w = 0, h = 0;
    pub h: i32,
    /// java: protected int x = 0, y = 0;
    pub x: i32,
    /// java: protected int x = 0, y = 0;
    pub y: i32,
    /// java: private boolean focused = false;
    pub focused: bool,
}

impl BasicWidgetBase {
    /// java: the implicit no-argument constructor (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_in<__R>(__wrap: impl FnOnce(Self) -> (compat::Shared<dyn BasicWidget>, __R)) -> (compat::Shared<dyn BasicWidget>, __R) {
        let this = Self {
            w: 0,
            h: 0,
            x: 0,
            y: 0,
            focused: false,
        };
        let (__h, __r) = __wrap(this);
        let __this = __h;
        (__this, __r)
    }

}

/// java: the virtual methods of BasicWidget (implemented by BasicWidgetBase and every subclass)
pub trait BasicWidget: compat::DynAny {
    fn basic_widget_base(&self) -> &BasicWidgetBase;
    fn basic_widget_base_mut(&mut self) -> &mut BasicWidgetBase;
    fn as_any(&self) -> &dyn std::any::Any;

    /// java: public final int getX() (dispatch: the body for this class)
    fn get_x(&self) -> fn(&compat::Shared<dyn BasicWidget>) -> i32 {
        <dyn BasicWidget>::get_x_body
    }

    /// java: public final void setX(int x) (dispatch: the body for this class)
    fn set_x(&self) -> fn(&compat::Shared<dyn BasicWidget>, i32) {
        <dyn BasicWidget>::set_x_body
    }

    /// java: public final int getY() (dispatch: the body for this class)
    fn get_y(&self) -> fn(&compat::Shared<dyn BasicWidget>) -> i32 {
        <dyn BasicWidget>::get_y_body
    }

    /// java: public final void setY(int y) (dispatch: the body for this class)
    fn set_y(&self) -> fn(&compat::Shared<dyn BasicWidget>, i32) {
        <dyn BasicWidget>::set_y_body
    }

    /// java: public final int getWidth() (dispatch: the body for this class)
    fn get_width(&self) -> fn(&compat::Shared<dyn BasicWidget>) -> i32 {
        <dyn BasicWidget>::get_width_body
    }

    /// java: public void setWidth(int width) (dispatch: the body for this class)
    fn set_width(&self) -> fn(&compat::Shared<dyn BasicWidget>, i32) {
        <dyn BasicWidget>::set_width_body
    }

    /// java: public final int getHeight() (dispatch: the body for this class)
    fn get_height(&self) -> fn(&compat::Shared<dyn BasicWidget>) -> i32 {
        <dyn BasicWidget>::get_height_body
    }

    /// java: @Override public void visitWidgets(Consumer<AbstractWidget> consumer) (dispatch: the body for this class)
    fn visit_widgets(&self) -> fn(&compat::Shared<dyn BasicWidget>, Arc<dyn Fn(AbstractWidget)>) {
        <dyn BasicWidget>::visit_widgets_body
    }

    /// java: public void setHeight(int height) (dispatch: the body for this class)
    fn set_height(&self) -> fn(&compat::Shared<dyn BasicWidget>, i32) {
        <dyn BasicWidget>::set_height_body
    }

    /// java: public void setDimensions(int size) (dispatch: the body for this class)
    fn set_dimensions(&self) -> fn(&compat::Shared<dyn BasicWidget>, i32) {
        <dyn BasicWidget>::set_dimensions_body
    }

    /// java: public void setDimensions(int width, int height) (dispatch: the body for this class)
    fn set_dimensions_2(&self) -> fn(&compat::Shared<dyn BasicWidget>, i32, i32) {
        <dyn BasicWidget>::set_dimensions_2_body
    }

    /// java: @Override public void setFocused(boolean focused) (dispatch: the body for this class)
    fn set_focused(&self) -> fn(&compat::Shared<dyn BasicWidget>, bool) {
        <dyn BasicWidget>::set_focused_body
    }

    /// java: @Override public boolean isFocused() (dispatch: the body for this class)
    fn is_focused(&self) -> fn(&compat::Shared<dyn BasicWidget>) -> bool {
        <dyn BasicWidget>::is_focused_body
    }

    /// java: @Override public boolean isMouseOver(double mouseX, double mouseY) (dispatch: the body for this class)
    fn is_mouse_over(&self) -> fn(&compat::Shared<dyn BasicWidget>, f64, f64) -> bool {
        <dyn BasicWidget>::is_mouse_over_body
    }

    /// java: @Override public ScreenRectangle getRectangle() (dispatch: the body for this class)
    fn get_rectangle(&self) -> fn(&compat::Shared<dyn BasicWidget>) -> ScreenRectangle {
        <dyn BasicWidget>::get_rectangle_body
    }

}

impl dyn BasicWidget {
    /// java: public final int getX()
    pub fn get_x_body(__this: &compat::Shared<dyn BasicWidget>) -> i32 {
        return compat::read(__this, |__t| __t.basic_widget_base().x);
    }

    /// java: public final void setX(int x)
    pub fn set_x_body(__this: &compat::Shared<dyn BasicWidget>, mut x: i32) {
        { let __v = x; compat::write(__this, |__t| __t.basic_widget_base_mut().x = __v); }
    }

    /// java: public final int getY()
    pub fn get_y_body(__this: &compat::Shared<dyn BasicWidget>) -> i32 {
        return compat::read(__this, |__t| __t.basic_widget_base().y);
    }

    /// java: public final void setY(int y)
    pub fn set_y_body(__this: &compat::Shared<dyn BasicWidget>, mut y: i32) {
        { let __v = y; compat::write(__this, |__t| __t.basic_widget_base_mut().y = __v); }
    }

    /// java: public final int getWidth()
    pub fn get_width_body(__this: &compat::Shared<dyn BasicWidget>) -> i32 {
        return compat::read(__this, |__t| __t.basic_widget_base().w);
    }

    /// java: public void setWidth(int width)
    pub fn set_width_body(__this: &compat::Shared<dyn BasicWidget>, mut width: i32) {
        { let __v = width; compat::write(__this, |__t| __t.basic_widget_base_mut().w = __v); }
    }

    /// java: public final int getHeight()
    pub fn get_height_body(__this: &compat::Shared<dyn BasicWidget>) -> i32 {
        return compat::read(__this, |__t| __t.basic_widget_base().h);
    }

    /// java: @Override public void visitWidgets(Consumer<AbstractWidget> consumer)
    pub fn visit_widgets_body(__this: &compat::Shared<dyn BasicWidget>, consumer: Arc<dyn Fn(AbstractWidget)>) {
    }

    /// java: public void setHeight(int height)
    pub fn set_height_body(__this: &compat::Shared<dyn BasicWidget>, mut height: i32) {
        { let __v = height; compat::write(__this, |__t| __t.basic_widget_base_mut().h = __v); }
    }

    /// java: public void setDimensions(int size)
    pub fn set_dimensions_body(__this: &compat::Shared<dyn BasicWidget>, mut size: i32) {
        ({ let __f = compat::dispatch(__this, |__t| __t.set_dimensions_2()); __f(__this, size, size) });
    }

    /// java: public void setDimensions(int width, int height)
    pub fn set_dimensions_2_body(__this: &compat::Shared<dyn BasicWidget>, mut width: i32, mut height: i32) {
        { let __v = width; compat::write(__this, |__t| __t.basic_widget_base_mut().w = __v); }
        { let __v = height; compat::write(__this, |__t| __t.basic_widget_base_mut().h = __v); }
    }

    /// java: @Override public void setFocused(boolean focused)
    pub fn set_focused_body(__this: &compat::Shared<dyn BasicWidget>, mut focused: bool) {
        { let __v = focused; compat::write(__this, |__t| __t.basic_widget_base_mut().focused = __v); }
    }

    /// java: @Override public boolean isFocused()
    pub fn is_focused_body(__this: &compat::Shared<dyn BasicWidget>) -> bool {
        return compat::read(__this, |__t| __t.basic_widget_base().focused);
    }

    /// java: @Override public boolean isMouseOver(double mouseX, double mouseY)
    pub fn is_mouse_over_body(__this: &compat::Shared<dyn BasicWidget>, mut mouse_x: f64, mut mouse_y: f64) -> bool {
        return (mouse_x >= (({ let __f = compat::dispatch(__this, |__t| __t.get_x()); __f(__this) }) as f64) && mouse_x <= (({ let __f = compat::dispatch(__this, |__t| __t.get_x()); __f(__this) }).wrapping_add({ let __f = compat::dispatch(__this, |__t| __t.get_width()); __f(__this) }) as f64) && mouse_y >= (({ let __f = compat::dispatch(__this, |__t| __t.get_y()); __f(__this) }) as f64) && mouse_y <= (({ let __f = compat::dispatch(__this, |__t| __t.get_y()); __f(__this) }).wrapping_add({ let __f = compat::dispatch(__this, |__t| __t.get_height()); __f(__this) }) as f64));
    }

    /// java: @Override public ScreenRectangle getRectangle()
    pub fn get_rectangle_body(__this: &compat::Shared<dyn BasicWidget>) -> ScreenRectangle {
        // TODO(fabric2seer): `.getRectangle(..)` on a value of unknown type
        //   java: return LayoutElement.super.getRectangle();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}

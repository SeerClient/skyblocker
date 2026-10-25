//! Ported from de.hysky.skyblocker.utils.render.gui.ColorPickerWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::render::gui_helper::GuiHelper;
use mcrust::com::mojang::blaze3d::platform::cursor::cursor_types::CursorTypes;
use mcrust::mcrust::modding::widgets::ModWidget;
use mcrust::mcrust::modding::widgets::WidgetRenderContext;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::WidgetKind;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::gui::navigation::screen_rectangle::ScreenRectangle;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::client::input::mouse_button_info::MouseButtonInfo;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::util::argb;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final Identifier SV_THUMB_TEXTURE = SkyblockerMod.id("color_picker/sv_thumb");
pub static SV_THUMB_TEXTURE: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("color_picker/sv_thumb"));

/// java: public class ColorPickerWidget extends AbstractWidget
#[derive(Clone)]
pub struct ColorPickerWidget {
    /// java: extends net.minecraft.client.gui.components.AbstractWidget
    pub base: AbstractWidget,
    /// java: private final int[] rainbowColors;
    pub rainbow_colors: Vec<i32>,
    /// java: private final boolean hasAlpha;
    pub has_alpha: bool,
    /// java: private final int alphaMask;
    pub alpha_mask: i32,
    /// java: private double aThumbX = 0;
    pub a_thumb_x: f64,
    /// java: private double hThumbX = 0;
    pub h_thumb_x: f64,
    /// java: private double svThumbX = 0;
    pub sv_thumb_x: f64,
    /// java: private double svThumbY = 0;
    pub sv_thumb_y: f64,
    /// java: private int svColor = 0xFF_FF_00_00;
    pub sv_color: i32,
    /// java: private boolean draggingSV = false;
    pub dragging_sv: bool,
    /// java: private boolean draggingH = false;
    pub dragging_h: bool,
    /// java: private boolean draggingA = false;
    pub dragging_a: bool,
    /// java: private ScreenRectangle svRect;
    pub sv_rect: Option<ScreenRectangle>,
    /// java: private ScreenRectangle hRect;
    pub h_rect: Option<ScreenRectangle>,
    /// java: private ScreenRectangle aRect = ScreenRectangle.empty();
    pub a_rect: ScreenRectangle,
    /// java: private int argbColor = -1;
    pub argb_color: i32,
    /// java: private @Nullable Callback onColorChange = null;
    pub on_color_change: Option<Callback>,
}

impl std::ops::Deref for ColorPickerWidget {
    type Target = AbstractWidget;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for ColorPickerWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl ColorPickerWidget {
    /// java: public ColorPickerWidget(int x, int y, int width, int height)
    pub fn new(mut x: i32, mut y: i32, mut width: i32, mut height: i32) -> compat::Shared<Self> {
        let __this = Self::new_5(x, y, width, height, false);
        __this
    }

    /// java: public ColorPickerWidget(int x, int y, int width, int height, boolean hasAlpha)
    pub fn new_5(mut x: i32, mut y: i32, mut width: i32, mut height: i32, mut has_alpha: bool) -> compat::Shared<Self> {
        let __init_rainbow_colors = ColorPickerWidget::create_rainbow_colors((width.wrapping_div(20)).min(8));
        let __init_has_alpha = has_alpha;
        let __init_alpha_mask = (if has_alpha { 0 } else { 0xFF000000u32 as i32 });
        let __this = compat::shared(Self {
            base: AbstractWidget::new(x, y, width, height, (&component::literal("ColorPicker").get_string()).to_string(), WidgetKind::PlainText, OnPress::Inert),
            rainbow_colors: __init_rainbow_colors,
            has_alpha: __init_has_alpha,
            alpha_mask: __init_alpha_mask,
            a_thumb_x: ((0) as f64) as f64,
            h_thumb_x: ((0) as f64) as f64,
            sv_thumb_x: ((0) as f64) as f64,
            sv_thumb_y: ((0) as f64) as f64,
            sv_color: 0xFFFF0000u32 as i32,
            dragging_sv: false,
            dragging_h: false,
            dragging_a: false,
            sv_rect: None,
            h_rect: None,
            a_rect: ScreenRectangle::empty(),
            argb_color: -1,
            on_color_change: None,
        });
        ColorPickerWidget::update_rects(&__this);
        __this
    }

    /// java: private static int[] createRainbowColors(int samples)
    pub fn create_rainbow_colors(mut samples: i32) -> Vec<i32> {
        let mut rainbow_colors: Vec<i32> = vec![0; (samples) as usize];
        for i in 0..samples {
            rainbow_colors[(i) as usize] = mcrust::java::awt::color::Color::hsb_to_rgb((((i) as f32) / (samples as f32)), 1.0, 1.0);
        }
        return rainbow_colors;
    }

    /// java: @Override public void onRelease(MouseButtonEvent click)
    pub fn on_release(__this: &compat::Shared<Self>, click: &MouseButtonEvent) {
        // java: super.onRelease(click); - `AbstractWidget.onRelease:103-104` is empty
        if (compat::read(__this, |__t| __t.dragging_h) || compat::read(__this, |__t| __t.dragging_sv) || compat::read(__this, |__t| __t.dragging_a)) && compat::read(__this, |__t| __t.on_color_change.clone()).is_some() {
            (compat::read(__this, |__t| __t.on_color_change.clone()).unwrap())((compat::read(__this, |__t| __t.argb_color) | compat::read(__this, |__t| __t.alpha_mask)), true);
        }
        { let __v = false; compat::write(__this, |__t| __t.dragging_h = __v); }
        { let __v = false; compat::write(__this, |__t| __t.dragging_sv = __v); }
        { let __v = false; compat::write(__this, |__t| __t.dragging_a = __v); }
    }

    /// java: private void updateRects()
    pub fn update_rects(__this: &compat::Shared<Self>) {
        let mut y: i32 = __this.lock().borrow().get_bottom();
        if compat::read(__this, |__t| __t.has_alpha) {
            { let __v = ScreenRectangle::new_at((__this.lock().borrow().get_x()).wrapping_add(1), (__this.lock().borrow().get_bottom()).wrapping_sub(9), (__this.lock().borrow().get_width()).wrapping_sub(2), 8); compat::write(__this, |__t| __t.a_rect = __v); }
            y = __this.lock().borrow().a_rect.top();
        }
        { let __v = Some(ScreenRectangle::new_at((__this.lock().borrow().get_x()).wrapping_add(1), (y.wrapping_sub(9)).wrapping_sub(4), (__this.lock().borrow().get_width()).wrapping_sub(2), 8)); compat::write(__this, |__t| __t.h_rect = __v); }
        let mut preview_offset: i32 = 15;
        let mut sv_y: i32 = (__this.lock().borrow().get_y()).wrapping_add(1);
        { let __v = Some(ScreenRectangle::new_at(((__this.lock().borrow().get_x()).wrapping_add(1)).wrapping_add(preview_offset), sv_y, ((__this.lock().borrow().get_width()).wrapping_sub(2)).wrapping_sub(preview_offset), ((__this.lock().borrow().h_rect.as_ref().unwrap().top()).wrapping_sub(sv_y)).wrapping_sub(4))); compat::write(__this, |__t| __t.sv_rect = __v); }
    }

    /// java: @Override public void setX(int x)
    pub fn set_x(__this: &compat::Shared<Self>, mut x: i32) {
        __this.lock().borrow_mut().base.set_x(x);
        ColorPickerWidget::update_rects(__this);
    }

    /// java: @Override public void setY(int y)
    pub fn set_y(__this: &compat::Shared<Self>, mut y: i32) {
        __this.lock().borrow_mut().base.set_y(y);
        ColorPickerWidget::update_rects(__this);
    }

    /// java: @Override public void setWidth(int width)
    pub fn set_width(__this: &compat::Shared<Self>, mut width: i32) {
        __this.lock().borrow_mut().base.set_width(width);
        ColorPickerWidget::update_rects(__this);
    }

    /// java: @Override public void setHeight(int height)
    pub fn set_height(__this: &compat::Shared<Self>, mut height: i32) {
        __this.lock().borrow_mut().base.set_height(height);
        ColorPickerWidget::update_rects(__this);
    }

    /// java: @Override public void setSize(int width, int height)
    pub fn set_size(__this: &compat::Shared<Self>, mut width: i32, mut height: i32) {
        __this.lock().borrow_mut().base.set_size(width, height);
        ColorPickerWidget::update_rects(__this);
    }

    /// java: @Override public void onClick(MouseButtonEvent click, boolean doubled)
    pub fn on_click(__this: &compat::Shared<Self>, click: &MouseButtonEvent, mut doubled: bool) {
        // java: super.onClick(click, doubled); - `AbstractWidget.onClick:100-101` is empty
        let mut i: i32 = ((click.x()) as i32);
        let mut j: i32 = ((click.y()) as i32);
        if __this.lock().borrow().h_rect.as_ref().unwrap().contains_point(i, j) {
            { let __v = true; compat::write(__this, |__t| __t.dragging_h = __v); }
            ColorPickerWidget::on_drag(__this, click, ((0) as f64) as f64, ((0) as f64) as f64);
        }
        if __this.lock().borrow().sv_rect.as_ref().unwrap().contains_point(i, j) {
            { let __v = true; compat::write(__this, |__t| __t.dragging_sv = __v); }
            ColorPickerWidget::on_drag(__this, click, ((0) as f64) as f64, ((0) as f64) as f64);
        }
        if compat::read(__this, |__t| __t.has_alpha) && __this.lock().borrow().a_rect.contains_point(i, j) {
            { let __v = true; compat::write(__this, |__t| __t.dragging_a = __v); }
            ColorPickerWidget::on_drag(__this, click, ((0) as f64) as f64, ((0) as f64) as f64);
        }
    }

    /// java: @Override protected void onDrag(MouseButtonEvent click, double deltaX, double deltaY)
    pub fn on_drag(__this: &compat::Shared<Self>, click: &MouseButtonEvent, mut delta_x: f64, mut delta_y: f64) {
        // java: super.onDrag(click, deltaX, deltaY); - `AbstractWidget.onDrag:106-107` is empty
        if compat::read(__this, |__t| __t.dragging_h) {
            { let __v = (click.x() - (__this.lock().borrow().h_rect.as_ref().unwrap().left() as f64)).clamp((0 as f64), ((__this.lock().borrow().h_rect.as_ref().unwrap().width()).wrapping_sub(1) as f64)); compat::write(__this, |__t| __t.h_thumb_x = __v); }
            { let __v = mcrust::java::awt::color::Color::hsb_to_rgb((((compat::read(__this, |__t| __t.h_thumb_x) / ((__this.lock().borrow().h_rect.as_ref().unwrap().width()).wrapping_sub(1) as f64))) as f32), 1.0, 1.0); compat::write(__this, |__t| __t.sv_color = __v); }
        }
        if compat::read(__this, |__t| __t.dragging_sv) {
            { let __v = (click.x() - (__this.lock().borrow().sv_rect.as_ref().unwrap().left() as f64)).clamp((0 as f64), ((__this.lock().borrow().sv_rect.as_ref().unwrap().width()).wrapping_sub(1) as f64)); compat::write(__this, |__t| __t.sv_thumb_x = __v); }
            { let __v = (click.y() - (__this.lock().borrow().sv_rect.as_ref().unwrap().top() as f64)).clamp((0 as f64), ((__this.lock().borrow().sv_rect.as_ref().unwrap().height()).wrapping_sub(1) as f64)); compat::write(__this, |__t| __t.sv_thumb_y = __v); }
        }
        if compat::read(__this, |__t| __t.dragging_a) {
            { let __v = (click.x() - (__this.lock().borrow().a_rect.left() as f64)).clamp((0 as f64), ((__this.lock().borrow().a_rect.width()).wrapping_sub(1) as f64)); compat::write(__this, |__t| __t.a_thumb_x = __v); }
        }
        if compat::read(__this, |__t| __t.dragging_h) || compat::read(__this, |__t| __t.dragging_sv) || compat::read(__this, |__t| __t.dragging_a) {
            let mut alpha: f32 = (if compat::read(__this, |__t| __t.has_alpha) { (((compat::read(__this, |__t| __t.a_thumb_x)) as f32) / ((__this.lock().borrow().a_rect.width()).wrapping_sub(1) as f32)) } else { 1.0f32 });
            { let __v = argb::color_alpha_f32_rgb(alpha, mcrust::java::awt::color::Color::hsb_to_rgb((((compat::read(__this, |__t| __t.h_thumb_x) / ((__this.lock().borrow().h_rect.as_ref().unwrap().width()).wrapping_sub(1) as f64))) as f32), (((compat::read(__this, |__t| __t.sv_thumb_x) / ((__this.lock().borrow().sv_rect.as_ref().unwrap().width()).wrapping_sub(1) as f64))) as f32), ((((1 as f64) - (compat::read(__this, |__t| __t.sv_thumb_y) / ((__this.lock().borrow().sv_rect.as_ref().unwrap().height()).wrapping_sub(1) as f64)))) as f32))); compat::write(__this, |__t| __t.argb_color = __v); }
            if compat::read(__this, |__t| __t.on_color_change.clone()).is_some() {
                (compat::read(__this, |__t| __t.on_color_change.clone()).unwrap())(compat::read(__this, |__t| __t.argb_color), false);
            }
        }
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float delta)
    pub fn extract_widget_render_state(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut delta: f32) {
        let mut color: i32 = 0x80606060u32 as i32;
        graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), (__this.lock().borrow().h_rect.as_ref().unwrap().left()).wrapping_sub(1), (__this.lock().borrow().h_rect.as_ref().unwrap().top()).wrapping_sub(1), (__this.lock().borrow().h_rect.as_ref().unwrap().right()).wrapping_add(1), (__this.lock().borrow().h_rect.as_ref().unwrap().bottom()).wrapping_add(1), color);
        for i in { 0..(__this.lock().borrow().rainbow_colors.len() as i32) } {
            let mut start_color: i32 = __this.lock().borrow().rainbow_colors[(i) as usize];
            let mut end_color: i32 = __this.lock().borrow().rainbow_colors[((i.wrapping_add(1)).wrapping_rem(__this.lock().borrow().rainbow_colors.len() as i32)) as usize];
            let mut segment_length: f32 = (((__this.lock().borrow().h_rect.as_ref().unwrap().width()) as f32) / ((__this.lock().borrow().rainbow_colors.len() as i32) as f32));
            let mut start_x: f32 = ((__this.lock().borrow().h_rect.as_ref().unwrap().left() as f32) + (segment_length * (i as f32)));
            let mut end_x: f32 = ((__this.lock().borrow().h_rect.as_ref().unwrap().left() as f32) + (segment_length * (i.wrapping_add(1) as f32)));
            GuiHelper::horizontal_gradient(graphics, start_x, ((__this.lock().borrow().h_rect.as_ref().unwrap().top()) as f32) as f32, end_x, ((__this.lock().borrow().h_rect.as_ref().unwrap().bottom()) as f32) as f32, start_color, end_color);
        }
        ColorPickerWidget::draw_thumb(__this, graphics, &compat::read(__this, |__t| __t.h_rect.clone()).unwrap(), ((compat::read(__this, |__t| __t.h_thumb_x)) as i32));
        graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), (__this.lock().borrow().sv_rect.as_ref().unwrap().left()).wrapping_sub(1), (__this.lock().borrow().sv_rect.as_ref().unwrap().top()).wrapping_sub(1), (__this.lock().borrow().sv_rect.as_ref().unwrap().right()).wrapping_add(1), (__this.lock().borrow().sv_rect.as_ref().unwrap().bottom()).wrapping_add(1), color);
        let mut picker_x: i32 = __this.lock().borrow().sv_rect.as_ref().unwrap().left();
        let mut picker_y: i32 = __this.lock().borrow().sv_rect.as_ref().unwrap().top();
        let mut picker_end_x: i32 = __this.lock().borrow().sv_rect.as_ref().unwrap().right();
        let mut picker_end_y: i32 = __this.lock().borrow().sv_rect.as_ref().unwrap().bottom();
        GuiHelper::horizontal_gradient(graphics, (picker_x) as f32, (picker_y) as f32, (picker_end_x) as f32, (picker_end_y) as f32, -1, compat::read(__this, |__t| __t.sv_color));
        graphics.fill_gradient(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), picker_x, picker_y, picker_end_x, picker_end_y, 1, 0xFF000000u32 as i32);
        gui_graphics_extractor::blit_sprite_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), SV_THUMB_TEXTURE.get_mut(), ((__this.lock().borrow().sv_rect.as_ref().unwrap().left()).wrapping_add((compat::read(__this, |__t| __t.sv_thumb_x)) as i32)).wrapping_sub(2), ((__this.lock().borrow().sv_rect.as_ref().unwrap().top()).wrapping_add((compat::read(__this, |__t| __t.sv_thumb_y)) as i32)).wrapping_sub(2), 5, 5);
        if compat::read(__this, |__t| __t.has_alpha) {
            graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), (__this.lock().borrow().a_rect.left()).wrapping_sub(1), (__this.lock().borrow().a_rect.top()).wrapping_sub(1), (__this.lock().borrow().a_rect.right()).wrapping_add(1), (__this.lock().borrow().a_rect.bottom()).wrapping_add(1), color);
            GuiHelper::horizontal_gradient(graphics, ((__this.lock().borrow().a_rect.left()) as f32) as f32, ((__this.lock().borrow().a_rect.top()) as f32) as f32, ((__this.lock().borrow().a_rect.right()) as f32) as f32, ((__this.lock().borrow().a_rect.bottom()) as f32) as f32, mcrust::net::minecraft::util::common_colors::BLACK, mcrust::net::minecraft::util::common_colors::WHITE);
            ColorPickerWidget::draw_thumb(__this, graphics, &compat::read(__this, |__t| __t.a_rect.clone()), ((compat::read(__this, |__t| __t.a_thumb_x)) as i32));
        }
        graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), __this.lock().borrow().get_x(), __this.lock().borrow().get_y(), (__this.lock().borrow().sv_rect.as_ref().unwrap().left()).wrapping_sub(2), (__this.lock().borrow().sv_rect.as_ref().unwrap().bottom()).wrapping_add(1), color);
        graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), (__this.lock().borrow().get_x()).wrapping_add(1), (__this.lock().borrow().get_y()).wrapping_add(1), (__this.lock().borrow().sv_rect.as_ref().unwrap().left()).wrapping_sub(3), __this.lock().borrow().sv_rect.as_ref().unwrap().bottom(), compat::read(__this, |__t| __t.argb_color));
        if __this.lock().borrow().is_hovered() {
            if __this.lock().borrow().sv_rect.as_ref().unwrap().contains_point(mouse_x, mouse_y) || __this.lock().borrow().h_rect.as_ref().unwrap().contains_point(mouse_x, mouse_y) || __this.lock().borrow().a_rect.contains_point(mouse_x, mouse_y) {
                // java: graphics.requestCursor(CursorTypes.POINTING_HAND);
                compat::batch_w_request_cursor(compat::BatchWCursor::PointingHand);
            }
            if compat::read(__this, |__t| __t.dragging_sv) {
                // java: graphics.requestCursor(CursorTypes.CROSSHAIR);
                compat::batch_w_request_cursor(compat::BatchWCursor::Crosshair);
            } else if compat::read(__this, |__t| __t.dragging_h) || compat::read(__this, |__t| __t.dragging_a) {
                // java: graphics.requestCursor(CursorTypes.RESIZE_EW);
                compat::batch_w_request_cursor(compat::BatchWCursor::ResizeEw);
            }
        }
    }

    /// java: private void drawThumb(GuiGraphicsExtractor graphics, ScreenRectangle rect, int thumbX)
    pub fn draw_thumb(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, rect: &ScreenRectangle, mut thumb_x: i32) {
        graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), (rect.left().wrapping_add(thumb_x)).wrapping_sub(1), rect.top(), (rect.left().wrapping_add(thumb_x)).wrapping_add(2), rect.bottom(), mcrust::net::minecraft::util::common_colors::BLACK);
        graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), rect.left().wrapping_add(thumb_x), rect.top().wrapping_sub(1), (rect.left().wrapping_add(thumb_x)).wrapping_add(1), rect.bottom().wrapping_add(1), mcrust::net::minecraft::util::common_colors::BLACK);
        graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), rect.left().wrapping_add(thumb_x), rect.top(), (rect.left().wrapping_add(thumb_x)).wrapping_add(1), rect.bottom(), mcrust::net::minecraft::util::common_colors::WHITE);
    }

    /// java: public int getARGBColor()
    pub fn get_argb_color(__this: &compat::Shared<Self>) -> i32 {
        return compat::read(__this, |__t| __t.argb_color);
    }

    /// java: public void setARGBColor(int argb)
    pub fn set_argb_color(__this: &compat::Shared<Self>, mut argb: i32) {
        { let __v = (argb | compat::read(__this, |__t| __t.alpha_mask)); compat::write(__this, |__t| __t.argb_color = __v); }
        // java: float[] floats = Color.RGBtoHSB((argbColor >> 16) & 0xFF, (argbColor >> 8) & 0xFF, argbColor & 0xFF, null);
        let __argb = compat::read(__this, |__t| __t.argb_color);
        let mut floats: Vec<f32> = mcrust::java::awt::color::Color::rgb_to_hsb((__argb >> 16) & 0xFF, (__argb >> 8) & 0xFF, __argb & 0xFF, None).to_vec();
        ColorPickerWidget::set_hsv(__this, floats[(0) as usize], floats[(1) as usize], floats[(2) as usize]);
        ColorPickerWidget::set_alpha(__this, argb::alpha_float(compat::read(__this, |__t| __t.argb_color)));
    }

    /// java: public void setHSV(float h, float s, float v)
    pub fn set_hsv(__this: &compat::Shared<Self>, mut h: f32, mut s: f32, mut v: f32) {
        { let __v = ((h * ((__this.lock().borrow().h_rect.as_ref().unwrap().width()).wrapping_sub(1) as f32))) as f64; compat::write(__this, |__t| __t.h_thumb_x = __v); }
        { let __v = ((s * ((__this.lock().borrow().sv_rect.as_ref().unwrap().width()).wrapping_sub(1) as f32))) as f64; compat::write(__this, |__t| __t.sv_thumb_x = __v); }
        { let __v = ((((1 as f32) - v) * ((__this.lock().borrow().sv_rect.as_ref().unwrap().height()).wrapping_sub(1) as f32))) as f64; compat::write(__this, |__t| __t.sv_thumb_y = __v); }
        { let __v = mcrust::java::awt::color::Color::hsb_to_rgb((((compat::read(__this, |__t| __t.h_thumb_x) / ((__this.lock().borrow().h_rect.as_ref().unwrap().width()).wrapping_sub(1) as f64))) as f32), 1.0, 1.0); compat::write(__this, |__t| __t.sv_color = __v); }
    }

    /// java: public void setAlpha(float alpha)
    pub fn set_alpha(__this: &compat::Shared<Self>, mut alpha: f32) {
        if !(compat::read(__this, |__t| __t.has_alpha)) {
            return;
        }
        { let __v = ((alpha * ((__this.lock().borrow().a_rect.width()).wrapping_sub(1) as f32))) as f64; compat::write(__this, |__t| __t.a_thumb_x = __v); }
    }

    /// java: public void setOnColorChange(@Nullable Callback onColorChange)
    pub fn set_on_color_change(__this: &compat::Shared<Self>, on_color_change: Callback) {
        { let __v = Some(on_color_change.clone()); compat::write(__this, |__t| __t.on_color_change = __v); }
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    pub fn update_widget_narration(__this: &compat::Shared<Self>, builder: &dyn NarrationElementOutput) {
    }

}

/// ColorPickerWidget's widget half (`super`'s fields: x, y, size, message, ...), lent by the game for each call
impl compat::WidgetHalf for ColorPickerWidget {
    fn widget_half(&mut self) -> &mut AbstractWidget {
        &mut self.base
    }
}

/// ColorPickerWidget as the game's widget: `compat::WidgetAdapter::widget(handle)` where Java hands it on
impl ModWidget for compat::WidgetAdapter<ColorPickerWidget> {
    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float delta)
    fn extract_widget_render_state(&mut self, widget: &AbstractWidget, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.lend_ref(widget, |__h| ColorPickerWidget::extract_widget_render_state(__h, &mut context.graphics, mouse_x, mouse_y, a))
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    fn update_widget_narration(&mut self, widget: &AbstractWidget, output: &mut dyn NarrationElementOutput) {
        self.lend_ref(widget, |__h| ColorPickerWidget::update_widget_narration(__h, output))
    }

    /// java: @Override public void onClick(MouseButtonEvent click, boolean doubled)
    fn on_click(&mut self, widget: &mut AbstractWidget, event: &MouseButtonEvent, double_click: bool) {
        self.lend(widget, |__h| ColorPickerWidget::on_click(__h, event, double_click))
    }

    /// java: @Override public void onRelease(MouseButtonEvent click)
    fn on_release(&mut self, widget: &mut AbstractWidget, event: &MouseButtonEvent) {
        self.lend(widget, |__h| ColorPickerWidget::on_release(__h, event))
    }

    /// java: @Override protected void onDrag(MouseButtonEvent click, double deltaX, double deltaY)
    fn on_drag(&mut self, widget: &mut AbstractWidget, event: &MouseButtonEvent, dx: f64, dy: f64) {
        self.lend(widget, |__h| ColorPickerWidget::on_drag(__h, event, dx, dy))
    }

}

// NOTE(fabric2seer): extractWidgetRenderState, onClick, onDrag, onRelease, setHeight, setSize, setWidth, setX, setY, updateWidgetNarration override(s) of AbstractWidget - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them

/// java: public interface Callback
/// a functional interface: `onColorChange(int color, boolean mouseRelease)` - a lambda / method reference is this closure type
pub type Callback = Arc<dyn Fn(i32, bool)>;

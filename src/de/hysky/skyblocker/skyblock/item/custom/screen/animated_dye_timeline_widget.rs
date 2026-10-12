//! Ported from de.hysky.skyblocker.skyblock.item.custom.screen.AnimatedDyeTimelineWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::item::custom::custom_armor_animated_dyes::AnimatedDye;
use crate::de::hysky::skyblocker::skyblock::item::custom::custom_armor_animated_dyes::Keyframe;
use crate::de::hysky::skyblocker::utils::ok_lab_color::OkLabColor;
use crate::de::hysky::skyblocker::utils::render::gui_helper::GuiHelper;
use mcrust::com::mojang::blaze3d::platform::native_image::NativeImage;
use mcrust::mcrust::modding::widgets::ModWidget;
use mcrust::mcrust::modding::widgets::WidgetRenderContext;
use mcrust::net::minecraft::client::gui::components::abstract_container_widget::AbstractContainerWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::events::gui_event_listener::GuiEventListener;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::client::input::mouse_button_info::MouseButtonInfo;
use mcrust::net::minecraft::client::renderer::texture::dynamic_texture::DynamicTexture;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::client::gui::components::renderable::Renderable as _;

/// java: private static final Identifier GRADIENT_TEXTURE = SkyblockerMod.id("generated/dye_gradient");
pub static GRADIENT_TEXTURE: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("generated/dye_gradient"));
/// java: private static final int HORIZONTAL_MARGIN = 3;
pub const HORIZONTAL_MARGIN: i32 = 3;
/// java: private static final int VERTICAL_MARGIN = 1;
pub const VERTICAL_MARGIN: i32 = 1;

/// java: public class AnimatedDyeTimelineWidget extends AbstractContainerWidget implements Closeable
pub struct AnimatedDyeTimelineWidget {
    /// java: extends net.minecraft.client.gui.components.AbstractContainerWidget
    pub base: AbstractContainerWidget,
    /// java: private DynamicTexture gradientTexture;
    pub gradient_texture: Option<DynamicTexture>,
    /// java: private int textureWidth;
    pub texture_width: i32,
    /// java: private int textureHeight;
    pub texture_height: i32,
    /// java: private final FrameCallback frameCallback;
    pub frame_callback: Option<FrameCallback>,
    /// java: private String uuid = "";
    pub uuid: String,
    /// java: private final ArrayList<KeyframeWidget> keyframes = new ArrayList<>();
    pub keyframes: Vec<compat::Shared<KeyframeWidget>>,
    /// java: private @Nullable AnimatedDyeTimelineWidget.KeyframeWidget focusedFrame = null;
    pub focused_frame: Option<compat::Shared<KeyframeWidget>>,
    /// java: private int deletedIndex = -1;
    pub deleted_index: i32,
}

impl std::ops::Deref for AnimatedDyeTimelineWidget {
    type Target = AbstractContainerWidget;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for AnimatedDyeTimelineWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl AnimatedDyeTimelineWidget {
    /// java: public AnimatedDyeTimelineWidget(int x, int y, int width, int height, FrameCallback frameCallback)
    pub fn new(mut x: i32, mut y: i32, mut width: i32, mut height: i32, frame_callback: FrameCallback) -> compat::Shared<Self> {
        // TODO(fabric2seer): static `AbstractScrollArea.defaultSettings` has no Rust counterpart in the port
        //   java: super(x, y, width, height, Component.literal("Animated Dye Timeline"), AbstractScrollArea.defaultSettings(4));
        let __this: compat::Shared<Self> = todo!("constructor");
        AnimatedDyeTimelineWidget::create_image(&__this, width, height);
        { let __v = Some(frame_callback.clone()); compat::write(&__this, |__t| __t.frame_callback = __v); }
        __this
    }

    /// java: private void createImage(int width, int height)
    pub fn create_image(__this: &compat::Shared<Self>, mut width: i32, mut height: i32) {
        { let __v = Some(DynamicTexture::new_blank("TimelineGradient", width.wrapping_sub(HORIZONTAL_MARGIN.wrapping_mul(2)), height.wrapping_sub(VERTICAL_MARGIN.wrapping_mul(2)), true)); compat::write(__this, |__t| __t.gradient_texture = __v); }
        debug_assert!(true /* java: __this.lock().borrow().gradient_texture.as_ref().unwrap().get_pixels() != null: never null here */);
        { let __v = __this.lock().borrow().gradient_texture.as_ref().unwrap().get_pixels().get_width(); compat::write(__this, |__t| __t.texture_width = __v); }
        { let __v = __this.lock().borrow().gradient_texture.as_ref().unwrap().get_pixels().get_height(); compat::write(__this, |__t| __t.texture_height = __v); }
        compat::minecraft().get_texture_manager().register(GRADIENT_TEXTURE.get_mut().clone(), compat::read(__this, |__t| __t.gradient_texture.clone()).unwrap());
    }

    /// java: public void recreateImage()
    pub fn recreate_image(__this: &compat::Shared<Self>) {
        AnimatedDyeTimelineWidget::create_image(__this, compat::read(__this, |__t| __t.width), compat::read(__this, |__t| __t.height));
        AnimatedDyeTimelineWidget::create_gradient_texture(__this);
    }

    /// java: @Override public void setWidth(int width)
    pub fn set_width(__this: &compat::Shared<Self>, mut width: i32) {
        __this.lock().borrow_mut().base.set_width(width);
        AnimatedDyeTimelineWidget::create_image(__this, width, compat::read(__this, |__t| __t.height));
        AnimatedDyeTimelineWidget::create_gradient_texture(__this);
    }

    /// java: @Override public List<? extends GuiEventListener> children()
    pub fn children(__this: &compat::Shared<Self>) -> Vec<Box<dyn GuiEventListener>> {
        return compat::read(__this, |__t| __t.keyframes.clone());
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_widget_render_state(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        gui_graphics_extractor::blit_identifier_region(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), GRADIENT_TEXTURE.get_mut(), (__this.lock().borrow().get_x()).wrapping_add(HORIZONTAL_MARGIN), (__this.lock().borrow().get_y()).wrapping_add(VERTICAL_MARGIN), ((0) as f32) as f32, ((0) as f32) as f32, (__this.lock().borrow().get_width()).wrapping_sub(HORIZONTAL_MARGIN.wrapping_mul(2)), (__this.lock().borrow().get_height()).wrapping_sub(VERTICAL_MARGIN.wrapping_mul(2)), compat::read(__this, |__t| __t.texture_width), compat::read(__this, |__t| __t.texture_height), compat::read(__this, |__t| __t.texture_width), compat::read(__this, |__t| __t.texture_height));
        for frame in compat::read(__this, |__t| __t.keyframes.clone()) {
            // TODO(fabric2seer): `extract_render_state` is a method of the port's trait `Renderable`, which the port's `AbstractWidget` does not implement
            // rust: let __wa1 = graphics.clone();
            // rust: let __wa2 = mouse_x;
            // rust: let __wa3 = mouse_y;
            // rust: let __wa4 = a;
            // rust: let __o1 = frame.clone();
            // rust: compat::write(&__o1, |__t| __t.extract_render_state(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), __wa2, __wa3, __wa4));
        }
    }

    /// java: @Override public void setFocused(@Nullable GuiEventListener focused)
    pub fn set_focused(__this: &compat::Shared<Self>, mut focused: Option<Box<dyn GuiEventListener>>) {
        { let (__q0,) = (focused.clone(),); __this.lock().borrow_mut().base.set_focused(__q0) };
        if let Some(keyframe) = focused.as_ref().unwrap().as_any().downcast_ref::<compat::Shared<KeyframeWidget>>() {
            (compat::read(__this, |__t| __t.frame_callback.clone()).unwrap())(compat::read(keyframe, |__t| __t.color), compat::read(keyframe, |__t| __t.time));
            { let __v = Some(keyframe.clone()); compat::write(__this, |__t| __t.focused_frame = __v); }
        }
    }

    /// java: public void setAnimatedDye(String uuid)
    pub fn set_animated_dye(__this: &compat::Shared<Self>, mut uuid: String) {
        { let __v = uuid.clone(); compat::write(__this, |__t| __t.uuid = __v); }
        let mut dye: AnimatedDye = SkyblockerConfigManager::get().general.custom_animated_dyes.get(&uuid).cloned().unwrap();
        __this.lock().borrow_mut().keyframes.clear();
        // TODO(fabric2seer): List.ensureCapacity(..)
        //   java: keyframes.ensureCapacity(dye.keyframes().size());
        for i in 0..(dye.keyframes().len() as i32) {
            let keyframe = &dye.keyframes()[(i) as usize];
            let __h0 = KeyframeWidget::new(keyframe.color(), keyframe.time(), (i != 0 && i != (dye.keyframes().len() as i32).wrapping_sub(1)));
            __this.lock().borrow_mut().keyframes.push(__h0);
        }
        AnimatedDyeTimelineWidget::set_focused(__this, Some(__this.lock().borrow().keyframes[0].clone()));
        AnimatedDyeTimelineWidget::create_gradient_texture(__this);
    }

    /// java: private void createGradientTexture()
    pub fn create_gradient_texture(__this: &compat::Shared<Self>) {
        let mut image: Option<NativeImage> = Some(__this.lock().borrow().gradient_texture.as_ref().unwrap().get_pixels().clone());
        debug_assert!(image.is_some());
        let mut l: i64 = compat::millis();
        for i in 0..(__this.lock().borrow().keyframes.len() as i32).wrapping_sub(1) {
            let mut frame: compat::Shared<KeyframeWidget> = __this.lock().borrow().keyframes[(i) as usize].clone();
            let mut next_frame: compat::Shared<KeyframeWidget> = __this.lock().borrow().keyframes[(i.wrapping_add(1)) as usize].clone();
            let mut start_x: i32 = (((((image.as_ref().unwrap().get_width()).wrapping_sub(1) as f32) * compat::read(&frame, |__t| __t.time))) as i32);
            let mut end_x: i32 = (((((image.as_ref().unwrap().get_width()).wrapping_sub(1) as f32) * compat::read(&next_frame, |__t| __t.time))) as i32);
            let mut size: i32 = end_x.wrapping_sub(start_x);
            for x in 0..=size {
                let mut color: i32 = OkLabColor::interpolate(compat::read(&frame, |__t| __t.color), compat::read(&next_frame, |__t| __t.color), (((x) as f32) / (size as f32)));
                for y in 0..image.as_ref().unwrap().get_height() {
                    image.as_mut().unwrap().set_pixel(x.wrapping_add(start_x), y, (color | 0xFF000000u32 as i32));
                }
            }
        }
        let mut v: f64 = (((compat::millis()).wrapping_sub(l) as f64) / 1000.0);
        logger().debug(&format!("Time taken to generate gradient texture: {}s", compat::fstr(v as f64)));
        __this.lock().borrow_mut().gradient_texture.as_mut().unwrap().upload();
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    pub fn mouse_clicked(__this: &compat::Shared<Self>, click: &MouseButtonEvent, mut doubled: bool) -> bool {
        let mut b: bool = { let (__q0, __q1,) = (click.clone(), doubled,); __this.lock().borrow_mut().base.mouse_clicked(__q0, __q1) };
        if b {
            if compat::read(__this, |__t| __t.deleted_index) != -1 {
                AnimatedDyeTimelineWidget::set_focused(__this, Some(__this.lock().borrow().keyframes[(compat::read(__this, |__t| __t.deleted_index)) as usize].clone()));
                { let __v = -1; compat::write(__this, |__t| __t.deleted_index = __v); }
            }
            return true;
        }
        if { let (__q0, __q1,) = (click.x(), click.y(),); __this.lock().borrow().is_mouse_over(__q0, __q1) } {
            let mut mouse_x: f64 = ((click.x() - (__this.lock().borrow().get_x() as f64)) + (HORIZONTAL_MARGIN as f64));
            let mut e: compat::Shared<KeyframeWidget> = KeyframeWidget::new(0xFFFF0000u32 as i32, (((mouse_x / (((__this.lock().borrow().get_width()).wrapping_sub(HORIZONTAL_MARGIN.wrapping_mul(2))).wrapping_sub(1) as f64))) as f32), true);
            let __h0 = e.clone();
            __this.lock().borrow_mut().keyframes.push(__h0);
            AnimatedDyeTimelineWidget::set_focused(__this, Some(e.clone()));
            AnimatedDyeTimelineWidget::data_changed(__this);
            return true;
        }
        return false;
    }

    /// java: public void setColor(int argb)
    pub fn set_color(__this: &compat::Shared<Self>, mut argb: i32) {
        if compat::read(__this, |__t| __t.focused_frame.clone()).is_none() {
            logger().warn("Tried to set color when no frame was focused");
            return;
        }
        { let __v = argb; compat::write(&compat::read(__this, |__t| __t.focused_frame.clone()).unwrap(), |__t| __t.color = __v); }
        AnimatedDyeTimelineWidget::data_changed(__this);
    }

    /// java: private void dataChanged()
    pub fn data_changed(__this: &compat::Shared<Self>) {
        let __h0 = |a, b| { let key = |f| compat::read(f, |__t| __t.time); key(a).total_cmp(&key(b)) };
        __this.lock().borrow_mut().keyframes.sort_by(__h0);
        AnimatedDyeTimelineWidget::create_gradient_texture(__this);
        let mut config_frames: Vec<Keyframe> = (compat::read(__this, |__t| __t.keyframes.clone()).into_iter().map(|keyframe| Keyframe::new(compat::read(&keyframe, |__t| __t.color), compat::read(&keyframe, |__t| __t.time))).collect::<Vec<_>>()).iter().cloned().collect::<Vec<_>>();
        let mut dye: AnimatedDye = SkyblockerConfigManager::get().general.custom_animated_dyes.get(&compat::read(__this, |__t| __t.uuid.clone())).cloned().unwrap();
        let mut new_dye: AnimatedDye = AnimatedDye::new(config_frames.clone(), dye.cycle_back(), dye.delay(), dye.duration());
        let __this_1 = compat::RenderThread::new(__this.clone());
        SkyblockerConfigManager::update_only(Arc::new(move |mut config| { config.general.custom_animated_dyes.insert(compat::read(__this_1.get(), |__t| __t.uuid.clone()), new_dye.clone()); }));
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    pub fn update_widget_narration(__this: &compat::Shared<Self>, builder: &dyn NarrationElementOutput) {
    }

    /// java: @Override protected int contentHeight()
    pub fn content_height(__this: &compat::Shared<Self>) -> i32 {
        return __this.lock().borrow().get_height();
    }

    /// java: @Override protected double scrollRate()
    pub fn scroll_rate(__this: &compat::Shared<Self>) -> f64 {
        return ((0) as f64) as f64;
    }

    /// java: @Override public void close()
    pub fn close(__this: &compat::Shared<Self>) {
        compat::minecraft().get_texture_manager().release(GRADIENT_TEXTURE.get_mut());
    }

}

// NOTE(fabric2seer): children, close, contentHeight, extractWidgetRenderState, mouseClicked, scrollRate, setFocused, setWidth, updateWidgetNarration override(s) of AbstractContainerWidget - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them

/// java: private class KeyframeWidget extends AbstractWidget
#[derive(Clone)]
pub struct KeyframeWidget {
    /// java: extends net.minecraft.client.gui.components.AbstractWidget
    pub base: AbstractWidget,
    /// java: int color;
    pub color: i32,
    /// java: float time;
    pub time: f32,
    /// java: private final boolean draggable;
    pub draggable: bool,
    /// java: private boolean dragging = false;
    pub dragging: bool,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl std::ops::Deref for KeyframeWidget {
    type Target = AbstractWidget;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for KeyframeWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl KeyframeWidget {
    /// java: private KeyframeWidget(int color, float time, boolean draggable)
    pub fn new(mut color: i32, mut time: f32, mut draggable: bool) -> compat::Shared<Self> {
        let __init_draggable = draggable;
        let __init_color = color;
        let __init_time = time;
        // TODO(fabric2seer): `AnimatedDyeTimelineWidget.this` (the enclosing instance of an inner class)
        //   java: super(0, 0, 7, AnimatedDyeTimelineWidget.this.getHeight(), Component.literal("Keyframe"));
        let __this: compat::Shared<Self> = todo!("constructor");
        __this
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_widget_render_state(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), KeyframeWidget::get_x(__this), KeyframeWidget::get_y(__this), (KeyframeWidget::get_x(__this)).wrapping_add(__this.lock().borrow().get_width()), (KeyframeWidget::get_y(__this)).wrapping_add(__this.lock().borrow().get_height()), compat::read(__this, |__t| __t.color));
        GuiHelper::border(graphics, KeyframeWidget::get_x(__this), KeyframeWidget::get_y(__this), __this.lock().borrow().get_width(), __this.lock().borrow().get_height(), (if __this.lock().borrow().is_focused() { -1 } else { mcrust::net::minecraft::util::common_colors::GRAY }));
    }

    /// java: @Override public int getX()
    pub fn get_x(__this: &compat::Shared<Self>) -> i32 {
        // TODO(fabric2seer): `AnimatedDyeTimelineWidget.this` (the enclosing instance of an inner class)
        //   java: AnimatedDyeTimelineWidget parent = AnimatedDyeTimelineWidget.this;
        let mut parent: compat::Shared<AnimatedDyeTimelineWidget> = todo!("fabric2seer: the initialiser of `parent` (TODO above)");
        let __o1 = parent.clone();
        let __o2 = parent.clone();
        let __o3 = parent.clone();
        let __o4 = parent.clone();
        return (((((compat::write(&__o3, |__t| __t.get_x())).wrapping_add(HORIZONTAL_MARGIN) as f32) + (compat::read(__this, |__t| __t.time) * (((compat::write(&__o4, |__t| __t.get_width())).wrapping_sub(HORIZONTAL_MARGIN.wrapping_mul(2))).wrapping_sub(1) as f32)))) as i32).wrapping_sub(3);
    }

    /// java: @Override public int getY()
    pub fn get_y(__this: &compat::Shared<Self>) -> i32 {
        // TODO(fabric2seer): `AnimatedDyeTimelineWidget.this` (the enclosing instance of an inner class)
        //   java: return AnimatedDyeTimelineWidget.this.getY();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override protected void onDrag(MouseButtonEvent click, double offsetX, double offsetY)
    pub fn on_drag(__this: &compat::Shared<Self>, click: &MouseButtonEvent, mut offset_x: f64, mut offset_y: f64) {
        // TODO(fabric2seer): `AbstractWidget.onDrag` has no Rust method in the port (looked for `on_drag` in AbstractWidget and its supertypes)
        // rust: __this.lock().borrow_mut().base.on_drag(click, offset_x, offset_y);
        if !(compat::read(__this, |__t| __t.draggable)) {
            return;
        }
        // TODO(fabric2seer): `AnimatedDyeTimelineWidget.this` (the enclosing instance of an inner class)
        //   java: AnimatedDyeTimelineWidget parent = AnimatedDyeTimelineWidget.this;
        let mut parent: compat::Shared<AnimatedDyeTimelineWidget> = todo!("fabric2seer: the initialiser of `parent` (TODO above)");
        let __o1 = parent.clone();
        let __o2 = parent.clone();
        let mut mouse_x: f64 = ((click.x() - (compat::write(&__o2, |__t| __t.get_x()) as f64)) + (HORIZONTAL_MARGIN as f64));
        let __o3 = parent.clone();
        let mut v: f32 = (((mouse_x / (((compat::write(&__o3, |__t| __t.get_width())).wrapping_sub(HORIZONTAL_MARGIN.wrapping_mul(2))).wrapping_sub(1) as f64))) as f32);
        { let __v = v.clamp((0 as f32), (1 as f32)); compat::write(__this, |__t| __t.time = __v); }
        { let __v = true; compat::write(__this, |__t| __t.dragging = __v); }
    }

    /// java: @Override public void onRelease(MouseButtonEvent click)
    pub fn on_release(__this: &compat::Shared<Self>, click: &MouseButtonEvent) {
        // TODO(fabric2seer): `AbstractWidget.onRelease` has no Rust method in the port (looked for `on_release` in AbstractWidget and its supertypes)
        // rust: __this.lock().borrow_mut().base.on_release(click);
        if compat::read(__this, |__t| __t.dragging) {
            // TODO(fabric2seer): `dataChanged(..)` is a method of the enclosing instance (AnimatedDyeTimelineWidget.this) - an inner class has no outer reference in Rust
            //   java: dataChanged();
        }
    }

    /// java: @Override public boolean keyPressed(KeyEvent input)
    pub fn key_pressed(__this: &compat::Shared<Self>, input: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        if input.key() == 261 {
            KeyframeWidget::delete_this(__this, false);
        }
        // TODO(fabric2seer): `key_pressed` takes 3 argument(s) in the port (`_key: i32, _scancode: i32, _modifiers: i32`), Java passes 1
        // TODO(fabric2seer): `key_pressed` is a method of the port's trait `GuiEventListener`, which the port's `AbstractWidget` does not implement
        // rust: return { let (__q0,) = (input.clone(),); __this.lock().borrow_mut().base.key_pressed(__q0) };
        return todo!("fabric2seer: see the TODO above");
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    pub fn mouse_clicked(__this: &compat::Shared<Self>, click: &MouseButtonEvent, mut doubled: bool) -> bool {
        if click.button() == 1 && { let (__q0, __q1,) = (click.x(), click.y(),); __this.lock().borrow().is_mouse_over(__q0, __q1) } {
            KeyframeWidget::delete_this(__this, true);
            return true;
        }
        // TODO(fabric2seer): `mouse_clicked` takes 3 argument(s) in the port (`button: i32, x: f64, y: f64`), Java passes 2
        // rust: return { let (__q0, __q1,) = (click.clone(), doubled,); __this.lock().borrow_mut().base.mouse_clicked(__q0, __q1) }.is_some();
        return todo!("fabric2seer: see the TODO above");
    }

    /// java: private void deleteThis(boolean mouse)
    pub fn delete_this(__this: &compat::Shared<Self>, mut mouse: bool) {
        if !(compat::read(__this, |__t| __t.draggable)) {
            return;
        }
        // TODO(fabric2seer): `keyframes` is a field of the enclosing instance (AnimatedDyeTimelineWidget.this) - an inner class has no outer reference in Rust
        //   java: int i = keyframes.indexOf(this);
        let mut i: i32 = todo!("fabric2seer: the initialiser of `i` (TODO above)");
        // TODO(fabric2seer): `AnimatedDyeTimelineWidget.this` (the enclosing instance of an inner class)
        //   java: AnimatedDyeTimelineWidget.this.setFocused(keyframes.get(i + 1));
        if mouse {
            // TODO(fabric2seer): `deletedIndex` is a field of the enclosing instance (AnimatedDyeTimelineWidget.this) - an inner class has no outer reference in Rust
            //   java: deletedIndex = i;
        }
        // TODO(fabric2seer): `keyframes` is a field of the enclosing instance (AnimatedDyeTimelineWidget.this) - an inner class has no outer reference in Rust
        //   java: keyframes.remove(this);
        // TODO(fabric2seer): `dataChanged(..)` is a method of the enclosing instance (AnimatedDyeTimelineWidget.this) - an inner class has no outer reference in Rust
        //   java: dataChanged();
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    pub fn update_widget_narration(__this: &compat::Shared<Self>, builder: &dyn NarrationElementOutput) {
    }

}

/// KeyframeWidget's widget half (`super`'s fields: x, y, size, message, ...), lent by the game for each call
impl compat::WidgetHalf for KeyframeWidget {
    fn widget_half(&mut self) -> &mut AbstractWidget {
        &mut self.base
    }
}

/// KeyframeWidget as the game's widget: `compat::WidgetAdapter::widget(handle)` where Java hands it on
impl ModWidget for compat::WidgetAdapter<KeyframeWidget> {
    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_widget_render_state(&mut self, widget: &AbstractWidget, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.lend_ref(widget, |__h| KeyframeWidget::extract_widget_render_state(__h, &mut context.graphics, mouse_x, mouse_y, a))
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    fn update_widget_narration(&mut self, widget: &AbstractWidget, output: &mut dyn NarrationElementOutput) {
        self.lend_ref(widget, |__h| KeyframeWidget::update_widget_narration(__h, output))
    }

    /// java: @Override public void onRelease(MouseButtonEvent click)
    fn on_release(&mut self, widget: &mut AbstractWidget, event: &MouseButtonEvent) {
        self.lend(widget, |__h| KeyframeWidget::on_release(__h, event))
    }

    /// java: @Override protected void onDrag(MouseButtonEvent click, double offsetX, double offsetY)
    fn on_drag(&mut self, widget: &mut AbstractWidget, event: &MouseButtonEvent, dx: f64, dy: f64) {
        self.lend(widget, |__h| KeyframeWidget::on_drag(__h, event, dx, dy))
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    fn mouse_clicked(&mut self, widget: &mut AbstractWidget, event: &MouseButtonEvent, double_click: bool) -> bool {
        self.lend(widget, |__h| KeyframeWidget::mouse_clicked(__h, event, double_click))
    }

    /// java: @Override public boolean keyPressed(KeyEvent input)
    fn key_pressed(&mut self, widget: &mut AbstractWidget, event: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        self.lend(widget, |__h| KeyframeWidget::key_pressed(__h, event))
    }

}

// NOTE(fabric2seer): extractWidgetRenderState, getX, getY, keyPressed, mouseClicked, onDrag, onRelease, updateWidgetNarration override(s) of AbstractWidget - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them

/// java: public interface FrameCallback
/// a functional interface: `onFrameSelected(int color, float time)` - a lambda / method reference is this closure type
pub type FrameCallback = Arc<dyn Fn(i32, f32)>;

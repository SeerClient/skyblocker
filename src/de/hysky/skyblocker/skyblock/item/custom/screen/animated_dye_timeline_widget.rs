//! Ported from de.hysky.skyblocker.skyblock.item.custom.screen.AnimatedDyeTimelineWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::item::custom::custom_armor_animated_dyes::AnimatedDye;
use crate::de::hysky::skyblocker::skyblock::item::custom::custom_armor_animated_dyes::Keyframe;
use crate::de::hysky::skyblocker::utils::ok_lab_color::OkLabColor;
use crate::de::hysky::skyblocker::utils::render::gui_helper::GuiHelper;
use mcrust::com::mojang::blaze3d::platform::input_constants;
use mcrust::mcrust::modding::widgets::WidgetRenderContext;
use mcrust::net::minecraft::client::gui::components::abstract_container_widget::AbstractContainerWidget;
use mcrust::net::minecraft::client::gui::components::abstract_scroll_area::ScrollbarSettings;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::input::key_event::KeyEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::client::renderer::texture::dynamic_texture::DynamicTexture;
use mcrust::net::minecraft::network::chat::formatted_text::FormattedText;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::util::common_colors;

/// java: private static final Identifier GRADIENT_TEXTURE = SkyblockerMod.id("generated/dye_gradient");
pub static GRADIENT_TEXTURE: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("generated/dye_gradient"));
/// java: private static final int HORIZONTAL_MARGIN = 3;
pub const HORIZONTAL_MARGIN: i32 = 3;
/// java: private static final int VERTICAL_MARGIN = 1;
pub const VERTICAL_MARGIN: i32 = 1;

/// java: public interface FrameCallback { void onFrameSelected(int color, float time); }
pub type FrameCallback = Arc<dyn Fn(i32, f32)>;

/// java: public class AnimatedDyeTimelineWidget extends AbstractContainerWidget implements Closeable
pub struct AnimatedDyeTimelineWidget {
    /// java: extends net.minecraft.client.gui.components.AbstractContainerWidget
    pub base: AbstractContainerWidget,
    /// java: private DynamicTexture gradientTexture;
    pub gradient_texture: DynamicTexture,
    /// java: private int textureWidth;
    pub texture_width: i32,
    /// java: private int textureHeight;
    pub texture_height: i32,
    /// java: private final FrameCallback frameCallback;
    pub frame_callback: FrameCallback,
    /// java: private String uuid = "";
    pub uuid: String,
    /// java: private final ArrayList<KeyframeWidget> keyframes = new ArrayList<>();
    pub keyframes: Vec<KeyframeWidget>,
    /// java: private @Nullable AnimatedDyeTimelineWidget.KeyframeWidget focusedFrame = null; (the keyframe's `id`)
    pub focused_frame: Option<u64>,
    /// java: private int deletedIndex = -1;
    pub deleted_index: i32,
    /// `AbstractContainerWidget.getFocused()` - the focused keyframe's `id` (Java holds the object)
    focused: Option<u64>,
    /// the next keyframe's `id`: Java tells keyframes apart by identity
    next_id: u64,
}

impl AnimatedDyeTimelineWidget {
    /// java: public AnimatedDyeTimelineWidget(int x, int y, int width, int height, FrameCallback frameCallback)
    pub fn new(x: i32, y: i32, width: i32, height: i32, frame_callback: FrameCallback) -> compat::Shared<Self> {
        // java: super(x, y, width, height, Component.literal("Animated Dye Timeline"), AbstractScrollArea.defaultSettings(4));
        let base = AbstractContainerWidget::new_with_scrollbar_settings(x, y, width, height, FormattedText::of("Animated Dye Timeline"), ScrollbarSettings::default_settings(4.0));
        // java: createImage(width, height);
        let (gradient_texture, texture_width, texture_height) = AnimatedDyeTimelineWidget::create_image(width, height);
        compat::shared(AnimatedDyeTimelineWidget {
            base,
            gradient_texture,
            texture_width,
            texture_height,
            frame_callback,
            uuid: String::new(),
            keyframes: Vec::new(),
            focused_frame: None,
            deleted_index: -1,
            focused: None,
            next_id: 0,
        })
    }

    /// java: private void createImage(int width, int height) - the texture, its size, registered under `GRADIENT_TEXTURE`
    fn create_image(width: i32, height: i32) -> (DynamicTexture, i32, i32) {
        let gradient_texture = DynamicTexture::new_blank("TimelineGradient", width - HORIZONTAL_MARGIN * 2, height - VERTICAL_MARGIN * 2, true);
        let texture_width = gradient_texture.get_pixels().get_width();
        let texture_height = gradient_texture.get_pixels().get_height();
        // java: Minecraft.getInstance().getTextureManager().register(GRADIENT_TEXTURE, gradientTexture);
        //   (the manager holds the same GPU texture this object writes)
        compat::minecraft().game_renderer.texture_manager_mut().register(GRADIENT_TEXTURE.get().clone(), gradient_texture.texture.clone());
        (gradient_texture, texture_width, texture_height)
    }

    /// The `createImage` fields written onto this object.
    fn create_image_here(&mut self, width: i32, height: i32) {
        let (gradient_texture, texture_width, texture_height) = AnimatedDyeTimelineWidget::create_image(width, height);
        self.gradient_texture = gradient_texture;
        self.texture_width = texture_width;
        self.texture_height = texture_height;
    }

    /// java: public void recreateImage() - Called when the screen has been displayed again after a popup
    pub fn recreate_image(&mut self) {
        let (width, height) = (self.base.width, self.base.height);
        self.create_image_here(width, height);
        self.create_gradient_texture();
    }

    /// java: @Override public void setFocused(@Nullable GuiEventListener focused)
    pub fn set_focused_child(&mut self, focused: Option<u64>) {
        // java: super.setFocused(focused); - AbstractContainerWidget:43-54 (the keyframes' `isFocused()` reads it)
        self.focused = focused;
        if let Some(id) = focused
            && let Some(keyframe) = self.keyframes.iter().find(|k| k.id == id)
        {
            (self.frame_callback)(keyframe.color, keyframe.time);
            self.focused_frame = Some(id);
        }
    }

    /// java: public void setAnimatedDye(String uuid)
    pub fn set_animated_dye(&mut self, uuid: &str) {
        self.uuid = uuid.to_string();
        let dye: AnimatedDye = SkyblockerConfigManager::get().general.custom_animated_dyes.get(uuid).cloned().expect("NullPointerException: the animated dye");
        self.keyframes.clear();
        let keyframes = dye.keyframes();
        self.keyframes.reserve(keyframes.len());
        for i in 0..keyframes.len() {
            let keyframe: &Keyframe = &keyframes[i];
            let frame = self.new_keyframe(keyframe.color(), keyframe.time(), i != 0 && i != keyframes.len() - 1);
            self.keyframes.push(frame);
        }
        // java: setFocused(keyframes.getFirst());
        let first = self.keyframes.first().expect("NoSuchElementException: keyframes.getFirst()").id;
        self.set_focused_child(Some(first));
        self.create_gradient_texture();
    }

    /// java: private void createGradientTexture()
    fn create_gradient_texture(&mut self) {
        let l = std::time::Instant::now();
        let image = self.gradient_texture.get_pixels_mut();
        let image_width = image.get_width();
        let image_height = image.get_height();
        for i in 0..self.keyframes.len().saturating_sub(1) {
            let frame = &self.keyframes[i];
            let next_frame = &self.keyframes[i + 1];
            let start_x: i32 = (((image_width - 1) as f32) * frame.time) as i32;
            let end_x: i32 = (((image_width - 1) as f32) * next_frame.time) as i32;
            let size: i32 = end_x - start_x;
            for x in 0..=size {
                let color: i32 = OkLabColor::interpolate(frame.color, next_frame.color, (x as f32) / (size as f32));
                for y in 0..image_height {
                    image.set_pixel(x + start_x, y, color | 0xFF_00_00_00_u32 as i32);
                }
            }
        }
        let v: f64 = l.elapsed().as_millis() as f64 / 1000.0;
        // java: CustomizeScreen.LOGGER.debug("Time taken to generate gradient texture: {}s", v);
        logger().debug(&format!("Time taken to generate gradient texture: {}s", v));
        self.gradient_texture.upload();
    }

    /// java: public void setColor(int argb)
    pub fn set_color(&mut self, argb: i32) {
        let Some(focused_frame) = self.focused_frame else {
            logger().warn("Tried to set color when no frame was focused");
            return;
        };
        if let Some(frame) = self.keyframes.iter_mut().find(|k| k.id == focused_frame) {
            frame.color = argb;
        }
        self.data_changed();
    }

    /// java: private void dataChanged()
    fn data_changed(&mut self) {
        // java: keyframes.sort(Comparator.comparingDouble(f -> f.time)); - stable, as `List.sort`
        self.keyframes.sort_by(|a, b| (a.time as f64).total_cmp(&(b.time as f64)));
        self.create_gradient_texture();
        let config_frames: Vec<Keyframe> = self.keyframes.iter().map(|keyframe| Keyframe::new(keyframe.color, keyframe.time)).collect();
        let dye: AnimatedDye = SkyblockerConfigManager::get().general.custom_animated_dyes.get(&self.uuid).cloned().expect("NullPointerException: the animated dye");
        let new_dye = AnimatedDye::new(config_frames, dye.cycle_back(), dye.delay(), dye.duration());
        let uuid = self.uuid.clone();
        SkyblockerConfigManager::update_only(Arc::new(move |config: &mut crate::de::hysky::skyblocker::config::skyblocker_config::SkyblockerConfig| {
            config.general.custom_animated_dyes.insert(uuid.clone(), new_dye.clone());
        }));
    }

    /// java: new KeyframeWidget(color, time, draggable)
    fn new_keyframe(&mut self, color: i32, time: f32, draggable: bool) -> KeyframeWidget {
        let id = self.next_id;
        self.next_id += 1;
        KeyframeWidget::new(id, color, time, draggable, self.base.height)
    }

    /// `keyframes.indexOf(..)` by identity.
    fn index_of(&self, id: u64) -> Option<usize> {
        self.keyframes.iter().position(|k| k.id == id)
    }

    /// java: KeyframeWidget.deleteThis(boolean mouse)
    fn delete_keyframe(&mut self, id: u64, mouse: bool) {
        let Some(i) = self.index_of(id) else {
            return;
        };
        if !self.keyframes[i].draggable {
            return;
        }
        // java: AnimatedDyeTimelineWidget.this.setFocused(keyframes.get(i + 1));
        let next = self.keyframes.get(i + 1).expect("IndexOutOfBoundsException: keyframes.get(i + 1)").id;
        self.set_focused_child(Some(next));
        if mouse {
            self.deleted_index = i as i32;
        }
        self.keyframes.remove(i);
        self.data_changed();
    }

    /// `KeyframeWidget.getX()` - from this widget's box
    fn keyframe_x(&self, keyframe: &KeyframeWidget) -> i32 {
        ((self.base.get_x() + HORIZONTAL_MARGIN) as f32 + keyframe.time * ((self.base.get_width() - HORIZONTAL_MARGIN * 2 - 1) as f32)) as i32 - 3
    }

    /// `KeyframeWidget.isMouseOver` - `isActive() && areCoordinatesInRectangle(..)` over its own box
    fn keyframe_is_mouse_over(&self, keyframe: &KeyframeWidget, mouse_x: f64, mouse_y: f64) -> bool {
        let (x, y) = (self.keyframe_x(keyframe), self.base.get_y());
        keyframe.active && mouse_x >= x as f64 && mouse_y >= y as f64 && mouse_x < (x + keyframe.width) as f64 && mouse_y < (y + keyframe.height) as f64
    }

    /// java: KeyframeWidget.mouseClicked(MouseButtonEvent click, boolean doubled)
    fn keyframe_mouse_clicked(&mut self, id: u64, event: &MouseButtonEvent) -> bool {
        let Some(i) = self.index_of(id) else {
            return false;
        };
        let over = self.keyframe_is_mouse_over(&self.keyframes[i], event.x(), event.y());
        if event.button() == input_constants::MOUSE_BUTTON_RIGHT && over {
            self.delete_keyframe(id, true);
            return true;
        }
        // java: super.mouseClicked(click, doubled) - AbstractWidget:109-125 (`onClick` is empty)
        self.keyframes[i].active && event.button() == 1 && over
    }

    /// java: KeyframeWidget.onDrag(MouseButtonEvent click, double offsetX, double offsetY)
    fn keyframe_on_drag(&mut self, id: u64, event: &MouseButtonEvent) {
        let Some(i) = self.index_of(id) else {
            return;
        };
        if !self.keyframes[i].draggable {
            return;
        }
        let mouse_x: f64 = event.x() - self.base.get_x() as f64 + HORIZONTAL_MARGIN as f64;
        let v: f32 = (mouse_x / (self.base.get_width() - HORIZONTAL_MARGIN * 2 - 1) as f64) as f32;
        self.keyframes[i].time = v.clamp(0.0, 1.0);
        self.keyframes[i].dragging = true;
    }

    /// java: KeyframeWidget.onRelease(MouseButtonEvent click)
    fn keyframe_on_release(&mut self, id: u64) {
        if self.index_of(id).is_some_and(|i| self.keyframes[i].dragging) {
            self.data_changed();
        }
    }

    /// java: @Override public void close()
    pub fn close(&mut self) {
        compat::minecraft().game_renderer.texture_manager_mut().release(GRADIENT_TEXTURE.get());
    }
}

impl compat::ObjectWidget for AnimatedDyeTimelineWidget {
    fn object_box(&mut self) -> &mut AbstractContainerWidget {
        &mut self.base
    }

    fn object_box_ref(&self) -> &AbstractContainerWidget {
        &self.base
    }

    /// java: @Override public void setWidth(int width)
    fn set_width(&mut self, width: i32) {
        self.base.set_width(width);
        let (width, height) = (self.base.width, self.base.height);
        self.create_image_here(width, height);
        self.create_gradient_texture();
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_widget_render_state(&mut self, _widget: &AbstractWidget, context: &mut WidgetRenderContext<'_>, _mouse_x: i32, _mouse_y: i32, _a: f32) {
        gui_graphics_extractor::blit_identifier_region(
            context.gl,
            context.minecraft,
            mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(),
            GRADIENT_TEXTURE.get(),
            self.base.get_x() + HORIZONTAL_MARGIN,
            self.base.get_y() + VERTICAL_MARGIN,
            0.0,
            0.0,
            self.base.get_width() - HORIZONTAL_MARGIN * 2,
            self.base.get_height() - VERTICAL_MARGIN * 2,
            self.texture_width,
            self.texture_height,
            self.texture_width,
            self.texture_height,
        );
        // java: for (KeyframeWidget frame : keyframes) frame.extractRenderState(graphics, mouseX, mouseY, a);
        for keyframe in self.keyframes.iter() {
            if !keyframe.visible {
                continue;
            }
            let (x, y, w, h) = (self.keyframe_x(keyframe), self.base.get_y(), keyframe.width, keyframe.height);
            // java: KeyframeWidget.extractWidgetRenderState
            context.graphics.fill(context.gl, x, y, x + w, y + h, keyframe.color);
            let focused = self.focused == Some(keyframe.id);
            GuiHelper::border(&mut context.graphics, x, y, w, h, if focused { -1 } else { common_colors::GRAY });
        }
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    fn mouse_clicked(&mut self, _widget: &mut AbstractWidget, event: &MouseButtonEvent, _double_click: bool) -> bool {
        // java: boolean b = super.mouseClicked(click, doubled); - ContainerEventHandler.mouseClicked:39-55 over the keyframes
        let mut b = false;
        let ids: Vec<u64> = self.keyframes.iter().map(|k| k.id).collect();
        for id in ids {
            if self.keyframe_mouse_clicked(id, event) {
                self.set_focused_child(Some(id));
                if event.button() == 1 {
                    self.base.set_dragging(true);
                }
                b = true;
                break;
            }
        }
        if b {
            if self.deleted_index != -1 {
                let id = self.keyframes.get(self.deleted_index as usize).expect("IndexOutOfBoundsException: keyframes.get(deletedIndex)").id;
                self.set_focused_child(Some(id));
                self.deleted_index = -1;
            }
            return true;
        }
        if self.base.is_mouse_over(event.x(), event.y()) {
            let mouse_x: f64 = event.x() - self.base.get_x() as f64 + HORIZONTAL_MARGIN as f64;
            let time = (mouse_x / (self.base.get_width() - HORIZONTAL_MARGIN * 2 - 1) as f64) as f32;
            let e = self.new_keyframe(0xFFFF0000_u32 as i32, time, true);
            let id = e.id;
            self.keyframes.push(e);
            self.set_focused_child(Some(id));
            self.data_changed();
            return true;
        }
        false
    }

    /// ContainerEventHandler.mouseReleased:57-66 - the focused keyframe's `mouseReleased` (`onRelease`)
    fn mouse_released(&mut self, _widget: &mut AbstractWidget, event: &MouseButtonEvent) -> bool {
        if event.button() == 1 && self.base.is_dragging() {
            self.base.set_dragging(false);
            if let Some(id) = self.focused {
                // AbstractWidget.mouseReleased:127-135
                self.keyframe_on_release(id);
                return true;
            }
        }
        false
    }

    /// ContainerEventHandler.mouseDragged:68-71 - the focused keyframe's `mouseDragged` (`onDrag`)
    fn mouse_dragged(&mut self, _widget: &mut AbstractWidget, event: &MouseButtonEvent, _dx: f64, _dy: f64) -> bool {
        if let Some(id) = self.focused
            && self.base.is_dragging()
            && event.button() == 1
        {
            self.keyframe_on_drag(id, event);
            return true;
        }
        false
    }

    /// ContainerEventHandler.keyPressed:83-85 - the focused keyframe's `keyPressed`
    fn key_pressed(&mut self, _widget: &mut AbstractWidget, event: &KeyEvent) -> bool {
        let Some(id) = self.focused else {
            return false;
        };
        // java: KeyframeWidget.keyPressed - `if (input.key() == KEY_DELETE) deleteThis(false); return super.keyPressed(input);`
        if event.key() == input_constants::KEY_DELETE {
            self.delete_keyframe(id, false);
        }
        false
    }
}

/// java: private class KeyframeWidget extends AbstractWidget
///
/// A child of the timeline that only the timeline draws and hands input to (`children()` is `keyframes`); its `getX()`
/// / `getY()` are the timeline's (`keyframe_x`).
#[derive(Clone, Debug)]
pub struct KeyframeWidget {
    /// the object's identity (Java compares keyframes by reference)
    pub id: u64,
    /// java: int color;
    pub color: i32,
    /// java: float time;
    pub time: f32,
    /// java: private final boolean draggable;
    pub draggable: bool,
    /// java: private boolean dragging = false;
    pub dragging: bool,
    /// AbstractWidget's `width` - 7
    pub width: i32,
    /// AbstractWidget's `height` - the timeline's height when the keyframe was made
    pub height: i32,
    /// AbstractWidget's `active`
    pub active: bool,
    /// AbstractWidget's `visible`
    pub visible: bool,
}

impl KeyframeWidget {
    /// java: private KeyframeWidget(int color, float time, boolean draggable) - `super(0, 0, 7, AnimatedDyeTimelineWidget.this.getHeight(), Component.literal("Keyframe"))`
    fn new(id: u64, color: i32, time: f32, draggable: bool, height: i32) -> KeyframeWidget {
        KeyframeWidget { id, color, time, draggable, dragging: false, width: 7, height, active: true, visible: true }
    }
}

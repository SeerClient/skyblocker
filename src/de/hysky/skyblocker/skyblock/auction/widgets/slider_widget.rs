//! Ported from de.hysky.skyblocker.skyblock.auction.widgets.SliderWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::auction::slot_click_handler::SlotClickHandler;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::WidgetKind;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::client::input::mouse_button_info::MouseButtonInfo;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: public class SliderWidget<E extends Enum<E> & SliderWidget.OptionInfo> extends AbstractWidget
#[derive(Clone)]
pub struct SliderWidgetBase<E> {
    /// java: extends net.minecraft.client.gui.components.AbstractWidget
    pub base: AbstractWidget,
    /// java: private final SlotClickHandler clickSlot;
    pub click_slot: SlotClickHandler,
    /// java: private int button = InputConstants.MOUSE_BUTTON_LEFT;
    pub button: i32,
    /// java: private int slotId = -1;
    pub slot_id: i32,
    /// java: protected E current;
    pub current: E,
    /// java: float posProgress;
    pub pos_progress: f32,
}

impl<E: 'static> std::ops::Deref for SliderWidgetBase<E> {
    type Target = AbstractWidget;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<E: 'static> std::ops::DerefMut for SliderWidgetBase<E> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl<E: Clone + OptionInfo + 'static> SliderWidgetBase<E> {
    /// java: public SliderWidget(int x, int y, int width, int height, Component message, SlotClickHandler clickSlot, E defaultOption)
    pub fn new(mut x: i32, mut y: i32, mut width: i32, mut height: i32, mut message: MutableComponent, mut click_slot: SlotClickHandler, mut default_option: E) -> Self {
        let __init_click_slot = click_slot.clone();
        let __init_current = default_option.clone();
        let mut this = Self {
            base: AbstractWidget::new(x, y, width, height, (&message.get_string()).to_string(), WidgetKind::PlainText, OnPress::Inert),
            click_slot: __init_click_slot,
            button: 0,
            slot_id: -1,
            current: __init_current,
            pos_progress: 0.0,
        };
        // java: posProgress = current.getOffset();
        this.pos_progress = this.current.get_offset() as f32;
        this
    }

}

/// java: the virtual methods of SliderWidget (implemented by SliderWidgetBase and every subclass)
pub trait SliderWidget<E: Clone + OptionInfo + 'static> {
    fn slider_widget_base(&self) -> &SliderWidgetBase<E>;
    fn slider_widget_base_mut(&mut self) -> &mut SliderWidgetBase<E>;
    fn as_any(&self) -> &dyn std::any::Any;

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float delta)
    fn extract_widget_render_state(&mut self, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut delta: f32) {
        let offset: f32 = self.slider_widget_base().current.get_offset() as f32;
        if self.slider_widget_base().pos_progress < offset {
            self.slider_widget_base_mut().pos_progress += delta * 5.0;
            if self.slider_widget_base().pos_progress > offset {
                self.slider_widget_base_mut().pos_progress = offset;
            }
        } else if self.slider_widget_base().pos_progress > offset {
            self.slider_widget_base_mut().pos_progress -= delta * 5.0;
            if self.slider_widget_base().pos_progress < offset {
                self.slider_widget_base_mut().pos_progress = offset;
            }
        }
        let __pose_1 = gui_graphics_extractor::push_pose();
        gui_graphics_extractor::pose_translate((self.slider_widget_base().get_x()) as f32, (self.slider_widget_base().get_y()) as f32);
        let current: E = self.slider_widget_base().current.clone();
        let pos_progress: f32 = self.slider_widget_base().pos_progress;
        // java: int x = current.isVertical() ? 0 : Math.round(posProgress); int y = current.isVertical() ? Math.round(posProgress) : 0;
        let mut x: i32 = if current.is_vertical() { 0 } else { compat::java_round_f32(pos_progress) };
        let mut y: i32 = if current.is_vertical() { compat::java_round_f32(pos_progress) } else { 0 };
        let option_size: Vec<i32> = current.get_option_size();
        let mut option_width: i32 = option_size[0];
        let mut option_height: i32 = option_size[1];
        let gl = mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl();
        let minecraft = &*compat::minecraft();
        let _ = graphics;
        gui_graphics_extractor::blit_sprite_identifier(gl, minecraft, mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), &current.get_back_texture(), 0, 0, self.slider_widget_base().get_width(), self.slider_widget_base().get_height());
        if self.slider_widget_base().is_hovered() {
            gui_graphics_extractor::blit_sprite_identifier(gl, minecraft, mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), &current.get_hover_texture(), x, y, option_width, option_height);
        } else {
            gui_graphics_extractor::blit_sprite_identifier(gl, minecraft, mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), &current.get_option_texture(), x, y, option_width, option_height);
        }
        gui_graphics_extractor::pop_pose(__pose_1);
    }

    /// java: @Override public void onClick(MouseButtonEvent click, boolean doubled)
    fn on_click(&self, click: &MouseButtonEvent, mut doubled: bool) {
        if self.slider_widget_base().slot_id == -1 {
            return;
        }
        // 26.3: the mouse button is SDL-numbered (left 1, right 3); the slot click takes the container button
        // (left 0, right 1) - `AbstractContainerScreen.getContainerClickButton:287-293`.
        (self.slider_widget_base().click_slot)(self.slider_widget_base().slot_id, mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::get_container_click_button(self.slider_widget_base().button));
        // java: super.onClick(click, doubled) - AbstractWidget.onClick:100-101 is an empty body, nothing to run
    }

    /// java: @Override protected boolean isValidClickButton(MouseButtonInfo input)
    fn is_valid_click_button(&mut self, input: &MouseButtonInfo) -> bool {
        self.slider_widget_base_mut().button = input.button();
        // java: return super.isValidClickButton(input) || button == InputConstants.MOUSE_BUTTON_RIGHT;
        return (self.slider_widget_base().base.is_valid_click_button(input.button()) || self.slider_widget_base().button == mcrust::com::mojang::blaze3d::platform::input_constants::MOUSE_BUTTON_RIGHT);
    }

    /// java: public void setSlotId(int slotId)
    fn set_slot_id(&mut self, mut slot_id: i32) {
        self.slider_widget_base_mut().slot_id = slot_id;
    }

    /// java: public void setCurrent(E current)
    fn set_current(&mut self, mut current: E) {
        self.slider_widget_base_mut().current = current.clone();
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    fn update_widget_narration(&self, builder: &dyn NarrationElementOutput) {
    }

}

impl<E: Clone + OptionInfo + 'static> SliderWidget<E> for SliderWidgetBase<E> {
    fn slider_widget_base(&self) -> &SliderWidgetBase<E> {
        self
    }
    fn slider_widget_base_mut(&mut self) -> &mut SliderWidgetBase<E> {
        self
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}

/// java: public interface OptionInfo - implemented by the enums a slider cycles through (`E extends Enum<E> & OptionInfo`), so
/// its methods are asked of the constant itself
pub trait OptionInfo {
    /// java: boolean isVertical();
    fn is_vertical(&self) -> bool;

    /// java: int getOffset();
    fn get_offset(&self) -> i32;

    /// java: int[] getOptionSize();
    fn get_option_size(&self) -> Vec<i32>;

    /// java: Identifier getOptionTexture();
    fn get_option_texture(&self) -> Identifier;

    /// java: Identifier getBackTexture();
    fn get_back_texture(&self) -> Identifier;

    /// java: Identifier getHoverTexture();
    fn get_hover_texture(&self) -> Identifier;
}

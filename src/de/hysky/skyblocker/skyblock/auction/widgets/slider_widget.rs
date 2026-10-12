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
    pub click_slot: compat::Shared<dyn SlotClickHandler>,
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

impl<E: Clone + 'static> SliderWidgetBase<E> {
    /// java: public SliderWidget(int x, int y, int width, int height, Component message, SlotClickHandler clickSlot, E defaultOption)
    pub fn new(mut x: i32, mut y: i32, mut width: i32, mut height: i32, mut message: MutableComponent, mut click_slot: compat::Shared<dyn SlotClickHandler>, mut default_option: E) -> Self {
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
        // TODO(fabric2seer): `.getOffset(..)` on a E
        //   java: posProgress = current.getOffset();
        this
    }

}

/// java: the virtual methods of SliderWidget (implemented by SliderWidgetBase and every subclass)
pub trait SliderWidget<E> {
    fn slider_widget_base(&self) -> &SliderWidgetBase<E>;
    fn slider_widget_base_mut(&mut self) -> &mut SliderWidgetBase<E>;
    fn as_any(&self) -> &dyn std::any::Any;

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float delta)
    fn extract_widget_render_state(&mut self, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut delta: f32) {
        // TODO(fabric2seer): `.getOffset(..)` on a E
        //   java: if (posProgress < current.getOffset()) {
        //   java: 			posProgress += delta * 5;
        //   java: 			if (posProgress > current.getOffset()) posProgress = current.getOffset();
        //   java: 		} else if (posProgress > current.getOffset()) {
        //   java: 			posProgress -= delta * 5;
        //   java: 			if (posProgress < current.getOffset()) posProgress = current.getOffset();
        //   java: 		}
        let __pose_1 = gui_graphics_extractor::push_pose();
        gui_graphics_extractor::pose_translate((self.slider_widget_base().get_x()) as f32, (self.slider_widget_base().get_y()) as f32);
        // TODO(fabric2seer): `.isVertical(..)` on a E
        //   java: int x = current.isVertical() ? 0 : Math.round(posProgress);
        let mut x: i32 = todo!("fabric2seer: the initialiser of `x` (TODO above)");
        // TODO(fabric2seer): `.isVertical(..)` on a E
        //   java: int y = current.isVertical() ? Math.round(posProgress) : 0;
        let mut y: i32 = todo!("fabric2seer: the initialiser of `y` (TODO above)");
        // TODO(fabric2seer): `.getOptionSize(..)` on a E
        //   java: int optionWidth = current.getOptionSize()[0];
        let mut option_width: i32 = todo!("fabric2seer: the initialiser of `optionWidth` (TODO above)");
        // TODO(fabric2seer): `.getOptionSize(..)` on a E
        //   java: int optionHeight = current.getOptionSize()[1];
        let mut option_height: i32 = todo!("fabric2seer: the initialiser of `optionHeight` (TODO above)");
        // TODO(fabric2seer): `.getBackTexture(..)` on a E
        //   java: graphics.blitSprite(RenderPipelines.GUI_TEXTURED, current.getBackTexture(), 0, 0, getWidth(), getHeight());
        if self.slider_widget_base().is_hovered() {
            // TODO(fabric2seer): `.getHoverTexture(..)` on a E
            //   java: graphics.blitSprite(RenderPipelines.GUI_TEXTURED, current.getHoverTexture(), x, y, optionWidth, optionHeight);
        } else {
            // TODO(fabric2seer): `.getOptionTexture(..)` on a E
            //   java: graphics.blitSprite(RenderPipelines.GUI_TEXTURED, current.getOptionTexture(), x, y, optionWidth, optionHeight);
        }
        gui_graphics_extractor::pop_pose(__pose_1);
    }

    /// java: @Override public void onClick(MouseButtonEvent click, boolean doubled)
    fn on_click(&self, click: &MouseButtonEvent, mut doubled: bool) {
        if self.slider_widget_base().slot_id == -1 {
            return;
        }
        ({ let __r = &self.slider_widget_base().click_slot; let __f = compat::dispatch(__r, |__t| __t.click()); __f(__r, self.slider_widget_base().slot_id, self.slider_widget_base().button) });
        // TODO(fabric2seer): `AbstractWidget.onClick` has no Rust method in the port (looked for `on_click` in AbstractWidget and its supertypes)
        // rust: self.slider_widget_base().on_click(click, doubled);
    }

    /// java: @Override protected boolean isValidClickButton(MouseButtonInfo input)
    fn is_valid_click_button(&mut self, input: &MouseButtonInfo) -> bool {
        self.slider_widget_base_mut().button = input.button();
        // TODO(fabric2seer): `AbstractWidget.isValidClickButton` has no Rust method in the port (looked for `is_valid_click_button` in AbstractWidget and its supertypes)
        // rust: return (self.slider_widget_base().is_valid_click_button(input) || self.slider_widget_base().button == 1);
        return todo!("fabric2seer: see the TODO above");
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

impl<E: Clone + 'static> SliderWidget<E> for SliderWidgetBase<E> {
    fn slider_widget_base(&self) -> &SliderWidgetBase<E> {
        &self
    }
    fn slider_widget_base_mut(&mut self) -> &mut SliderWidgetBase<E> {
        &mut self
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}

/// java: public interface OptionInfo
pub trait OptionInfo: compat::DynAny {
    fn as_any(&self) -> &dyn std::any::Any;
    /// java: boolean isVertical(); (dispatch: the body for this class)
    fn is_vertical(&self) -> fn(&compat::Shared<dyn OptionInfo>) -> bool;

    /// java: int getOffset(); (dispatch: the body for this class)
    fn get_offset(&self) -> fn(&compat::Shared<dyn OptionInfo>) -> i32;

    /// java: int[] getOptionSize(); (dispatch: the body for this class)
    fn get_option_size(&self) -> fn(&compat::Shared<dyn OptionInfo>) -> Vec<i32>;

    /// java: Identifier getOptionTexture(); (dispatch: the body for this class)
    fn get_option_texture(&self) -> fn(&compat::Shared<dyn OptionInfo>) -> Identifier;

    /// java: Identifier getBackTexture(); (dispatch: the body for this class)
    fn get_back_texture(&self) -> fn(&compat::Shared<dyn OptionInfo>) -> Identifier;

    /// java: Identifier getHoverTexture(); (dispatch: the body for this class)
    fn get_hover_texture(&self) -> fn(&compat::Shared<dyn OptionInfo>) -> Identifier;

}

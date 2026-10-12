//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.widgets.BasicInfoBoxWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::WidgetKind;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final Identifier BACKGROUND = SkyblockerMod.id("profile_viewer2/basic_background");
pub static BACKGROUND: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("profile_viewer2/basic_background"));

/// java: public sealed class BasicInfoBoxWidget extends AbstractWidget permits AccessoryPowerWidget, CroesusWidget, DailyRunsWidget, DungeonRunsWidget, DungeonSecretsWidget, FloorRunsWidget, SkillsInfoBoxWidget, TuningSlotWidget
#[derive(Clone)]
pub struct BasicInfoBoxWidgetBase {
    /// java: extends net.minecraft.client.gui.components.AbstractWidget
    pub base: AbstractWidget,
}

impl std::ops::Deref for BasicInfoBoxWidgetBase {
    type Target = AbstractWidget;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for BasicInfoBoxWidgetBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl BasicInfoBoxWidgetBase {
    /// java: public BasicInfoBoxWidget(int width, int height) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_in<__R>(mut width: i32, mut height: i32, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn BasicInfoBoxWidget>, __R)) -> (compat::Shared<dyn BasicInfoBoxWidget>, __R) {
        let this = Self {
            base: AbstractWidget::new(0, 0, width, height, (&component::empty().get_string()).to_string(), WidgetKind::PlainText, OnPress::Inert),
        };
        let (__h, __r) = __wrap(this);
        let __this = __h;
        { let __v = false; compat::write(&__this, |__t| __t.basic_info_box_widget_base_mut().active = __v); }
        (__this, __r)
    }

    /// java: public BasicInfoBoxWidget(int width, int height)
    pub fn new(mut width: i32, mut height: i32) -> compat::Shared<Self> {
        Self::new_in(width, height, |this| {
            let __h = compat::shared(this);
            ((__h.clone() as compat::Shared<dyn BasicInfoBoxWidget>), __h)
        }).1
    }

}

/// java: the virtual methods of BasicInfoBoxWidget (implemented by BasicInfoBoxWidgetBase and every subclass)
pub trait BasicInfoBoxWidget: compat::DynAny {
    fn basic_info_box_widget_base(&self) -> &BasicInfoBoxWidgetBase;
    fn basic_info_box_widget_base_mut(&mut self) -> &mut BasicInfoBoxWidgetBase;
    fn as_any(&self) -> &dyn std::any::Any;

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a) (dispatch: the body for this class)
    fn extract_widget_render_state(&self) -> fn(&compat::Shared<dyn BasicInfoBoxWidget>, &mut GuiGraphicsExtractor, i32, i32, f32) {
        <dyn BasicInfoBoxWidget>::extract_widget_render_state_body
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput output) (dispatch: the body for this class)
    fn update_widget_narration(&self) -> fn(&compat::Shared<dyn BasicInfoBoxWidget>, &dyn NarrationElementOutput) {
        <dyn BasicInfoBoxWidget>::update_widget_narration_body
    }

    /// java: @Override public boolean shouldTakeFocusAfterInteraction() (dispatch: the body for this class)
    fn should_take_focus_after_interaction(&self) -> fn(&compat::Shared<dyn BasicInfoBoxWidget>) -> bool {
        <dyn BasicInfoBoxWidget>::should_take_focus_after_interaction_body
    }

}

impl dyn BasicInfoBoxWidget {
    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_widget_render_state_body(__this: &compat::Shared<dyn BasicInfoBoxWidget>, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        gui_graphics_extractor::blit_sprite_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), BACKGROUND.get_mut(), __this.lock().borrow().basic_info_box_widget_base().get_x(), __this.lock().borrow().basic_info_box_widget_base().get_y(), __this.lock().borrow().basic_info_box_widget_base().get_width(), __this.lock().borrow().basic_info_box_widget_base().get_height());
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput output)
    pub fn update_widget_narration_body(__this: &compat::Shared<dyn BasicInfoBoxWidget>, output: &dyn NarrationElementOutput) {
    }

    /// java: @Override public boolean shouldTakeFocusAfterInteraction()
    pub fn should_take_focus_after_interaction_body(__this: &compat::Shared<dyn BasicInfoBoxWidget>) -> bool {
        return false;
    }

}

impl BasicInfoBoxWidget for BasicInfoBoxWidgetBase {
    fn basic_info_box_widget_base(&self) -> &BasicInfoBoxWidgetBase {
        self
    }
    fn basic_info_box_widget_base_mut(&mut self) -> &mut BasicInfoBoxWidgetBase {
        self
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}

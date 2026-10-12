//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.widgets.ProfileViewerWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::abstract_profile_viewer_screen;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::WidgetKind;
use mcrust::net::minecraft::client::gui::font::Font;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: protected static final int CONTENT_PADDING = AbstractProfileViewerScreen.PADDING;
pub static CONTENT_PADDING: LazyLock<i32> = LazyLock::new(|| abstract_profile_viewer_screen::PADDING);
/// java: protected static final int ITEM_SIZE = GuiRenderer.DEFAULT_ITEM_SIZE;
pub static ITEM_SIZE: LazyLock<i32> = LazyLock::new(|| mcrust::net::minecraft::client::gui::render::gui_renderer::DEFAULT_ITEM_SIZE);

/// java: public abstract sealed class ProfileViewerWidget extends AbstractWidget permits PageTabWidget
#[derive(Clone)]
pub struct ProfileViewerWidgetBase {
    /// java: extends net.minecraft.client.gui.components.AbstractWidget
    pub base: AbstractWidget,
    /// java: private final boolean padToContent;
    pub pad_to_content: bool,
    /// java: private int relativeX;
    pub relative_x: i32,
    /// java: private int relativeY;
    pub relative_y: i32,
}

impl std::ops::Deref for ProfileViewerWidgetBase {
    type Target = AbstractWidget;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for ProfileViewerWidgetBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl ProfileViewerWidgetBase {
    /// java: protected ProfileViewerWidget(int x, int y, int width, int height, Component message) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_in<__R>(mut x: i32, mut y: i32, mut width: i32, mut height: i32, mut message: MutableComponent, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn ProfileViewerWidget>, __R)) -> (compat::Shared<dyn ProfileViewerWidget>, __R) {
        let (__this, __r) = Self::new_6_in(x, y, width, height, true, message.clone(), __wrap);
        (__this, __r)
    }

    /// java: protected ProfileViewerWidget(int x, int y, int width, int height, boolean padToContent, Component message) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_6_in<__R>(mut x: i32, mut y: i32, mut width: i32, mut height: i32, mut pad_to_content: bool, mut message: MutableComponent, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn ProfileViewerWidget>, __R)) -> (compat::Shared<dyn ProfileViewerWidget>, __R) {
        let __init_pad_to_content = pad_to_content;
        let __init_relative_x = x;
        let __init_relative_y = y;
        let this = Self {
            base: AbstractWidget::new(0, 0, width, height, (&message.get_string()).to_string(), WidgetKind::PlainText, OnPress::Inert),
            pad_to_content: __init_pad_to_content,
            relative_x: __init_relative_x,
            relative_y: __init_relative_y,
        };
        let (__h, __r) = __wrap(this);
        let __this = __h;
        { let __v = false; compat::write(&__this, |__t| __t.profile_viewer_widget_base_mut().active = __v); }
        (__this, __r)
    }

    /// java: protected static Font getFont()
    pub fn get_font() -> Font {
        return compat::font();
    }

}

/// java: the virtual methods of ProfileViewerWidget (implemented by ProfileViewerWidgetBase and every subclass)
pub trait ProfileViewerWidget: compat::DynAny {
    fn profile_viewer_widget_base(&self) -> &ProfileViewerWidgetBase;
    fn profile_viewer_widget_base_mut(&mut self) -> &mut ProfileViewerWidgetBase;
    fn as_any(&self) -> &dyn std::any::Any;

    /// java: protected final int getRelativeX() (dispatch: the body for this class)
    fn get_relative_x(&self) -> fn(&compat::Shared<dyn ProfileViewerWidget>) -> i32 {
        <dyn ProfileViewerWidget>::get_relative_x_body
    }

    /// java: protected final void setRelativeX(int relativeX) (dispatch: the body for this class)
    fn set_relative_x(&self) -> fn(&compat::Shared<dyn ProfileViewerWidget>, i32) {
        <dyn ProfileViewerWidget>::set_relative_x_body
    }

    /// java: protected final int getRelativeY() (dispatch: the body for this class)
    fn get_relative_y(&self) -> fn(&compat::Shared<dyn ProfileViewerWidget>) -> i32 {
        <dyn ProfileViewerWidget>::get_relative_y_body
    }

    /// java: protected final void setRelativeY(int relativeY) (dispatch: the body for this class)
    fn set_relative_y(&self) -> fn(&compat::Shared<dyn ProfileViewerWidget>, i32) {
        <dyn ProfileViewerWidget>::set_relative_y_body
    }

    /// java: public final void updatePosition(int backgroundX, int backgroundY) (dispatch: the body for this class)
    fn update_position(&self) -> fn(&compat::Shared<dyn ProfileViewerWidget>, i32, i32) {
        <dyn ProfileViewerWidget>::update_position_body
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput output) (dispatch: the body for this class)
    fn update_widget_narration(&self) -> fn(&compat::Shared<dyn ProfileViewerWidget>, &dyn NarrationElementOutput) {
        <dyn ProfileViewerWidget>::update_widget_narration_body
    }

}

impl dyn ProfileViewerWidget {
    /// java: protected final int getRelativeX()
    pub fn get_relative_x_body(__this: &compat::Shared<dyn ProfileViewerWidget>) -> i32 {
        return compat::read(__this, |__t| __t.profile_viewer_widget_base().relative_x);
    }

    /// java: protected final void setRelativeX(int relativeX)
    pub fn set_relative_x_body(__this: &compat::Shared<dyn ProfileViewerWidget>, mut relative_x: i32) {
        { let __v = relative_x; compat::write(__this, |__t| __t.profile_viewer_widget_base_mut().relative_x = __v); }
    }

    /// java: protected final int getRelativeY()
    pub fn get_relative_y_body(__this: &compat::Shared<dyn ProfileViewerWidget>) -> i32 {
        return compat::read(__this, |__t| __t.profile_viewer_widget_base().relative_y);
    }

    /// java: protected final void setRelativeY(int relativeY)
    pub fn set_relative_y_body(__this: &compat::Shared<dyn ProfileViewerWidget>, mut relative_y: i32) {
        { let __v = relative_y; compat::write(__this, |__t| __t.profile_viewer_widget_base_mut().relative_y = __v); }
    }

    /// java: public final void updatePosition(int backgroundX, int backgroundY)
    pub fn update_position_body(__this: &compat::Shared<dyn ProfileViewerWidget>, mut background_x: i32, mut background_y: i32) {
        let __r0 = compat::read(__this, |__t| __t.profile_viewer_widget_base().relative_x);
        let __r1 = compat::read(__this, |__t| __t.profile_viewer_widget_base().pad_to_content);
        { let (__q0,) = ((background_x.wrapping_add(__r0)).wrapping_add(if __r1 { *CONTENT_PADDING } else { 0 }),); __this.lock().borrow_mut().profile_viewer_widget_base_mut().set_x(__q0) };
        let __r0 = compat::read(__this, |__t| __t.profile_viewer_widget_base().relative_y);
        let __r1 = compat::read(__this, |__t| __t.profile_viewer_widget_base().pad_to_content);
        { let (__q0,) = ((background_y.wrapping_add(__r0)).wrapping_add(if __r1 { *CONTENT_PADDING } else { 0 }),); __this.lock().borrow_mut().profile_viewer_widget_base_mut().set_y(__q0) };
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput output)
    pub fn update_widget_narration_body(__this: &compat::Shared<dyn ProfileViewerWidget>, output: &dyn NarrationElementOutput) {
    }

}

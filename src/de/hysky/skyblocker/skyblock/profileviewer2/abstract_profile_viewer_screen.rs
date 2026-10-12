//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.AbstractProfileViewerScreen by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final Identifier BACKGROUND = SkyblockerMod.id("textures/gui/profile_viewer2/base_plate.png");
pub static BACKGROUND: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("textures/gui/profile_viewer2/base_plate.png"));
/// java: public static final int BACKGROUND_WIDTH = 353;
pub const BACKGROUND_WIDTH: i32 = 353;
/// java: public static final int BACKGROUND_HEIGHT = 199;
pub const BACKGROUND_HEIGHT: i32 = 199;
/// java: public static final int PADDING = 8;
pub const PADDING: i32 = 8;
/// java: public static final int CONTENT_WIDTH = BACKGROUND_WIDTH - PADDING * 2;
pub static CONTENT_WIDTH: LazyLock<i32> = LazyLock::new(|| BACKGROUND_WIDTH.wrapping_sub(PADDING.wrapping_mul(2)));
/// java: public static final int CONTENT_HEIGHT = BACKGROUND_HEIGHT - PADDING * 2;
pub static CONTENT_HEIGHT: LazyLock<i32> = LazyLock::new(|| BACKGROUND_HEIGHT.wrapping_sub(PADDING.wrapping_mul(2)));

/// java: public abstract sealed class AbstractProfileViewerScreen extends Screen permits ErrorProfileViewerScreen, LoadingProfileViewerScreen, ProfileViewerScreen
pub struct AbstractProfileViewerScreenBase {
    /// java: extends net.minecraft.client.gui.screens.Screen
    pub base: mcrust::net::minecraft::client::gui::screens::screen::Screen,
    /// the screen's widgets as this object last saw them (see `compat::WidgetCache`)
    pub __widgets: compat::WidgetCache,
}

impl std::ops::Deref for AbstractProfileViewerScreenBase {
    type Target = mcrust::net::minecraft::client::gui::screens::screen::Screen;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for AbstractProfileViewerScreenBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl AbstractProfileViewerScreenBase {
    /// java: protected AbstractProfileViewerScreen(Component title) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_in<__R>(mut title: MutableComponent, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn AbstractProfileViewerScreen>, __R)) -> (compat::Shared<dyn AbstractProfileViewerScreen>, __R) {
        let this = Self {
            base: mcrust::net::minecraft::client::gui::screens::screen::Screen::new(title.get_string()),
            __widgets: Default::default(),
        };
        let (__h, __r) = __wrap(this);
        let __this = __h;
        (__this, __r)
    }

}

/// java: the virtual methods of AbstractProfileViewerScreen (implemented by AbstractProfileViewerScreenBase and every subclass)
pub trait AbstractProfileViewerScreen: compat::DynAny {
    fn abstract_profile_viewer_screen_base(&self) -> &AbstractProfileViewerScreenBase;
    fn abstract_profile_viewer_screen_base_mut(&mut self) -> &mut AbstractProfileViewerScreenBase;
    fn as_any(&self) -> &dyn std::any::Any;

    /// java: protected final int getBackgroundX() (dispatch: the body for this class)
    fn get_background_x(&self) -> fn(&compat::Shared<dyn AbstractProfileViewerScreen>) -> i32 {
        <dyn AbstractProfileViewerScreen>::get_background_x_body
    }

    /// java: protected final int getBackgroundY() (dispatch: the body for this class)
    fn get_background_y(&self) -> fn(&compat::Shared<dyn AbstractProfileViewerScreen>) -> i32 {
        <dyn AbstractProfileViewerScreen>::get_background_y_body
    }

    /// java: @Override public void extractRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a) (dispatch: the body for this class)
    fn extract_render_state(&self) -> fn(&compat::Shared<dyn AbstractProfileViewerScreen>, &GuiGraphicsExtractor, i32, i32, f32) {
        <dyn AbstractProfileViewerScreen>::extract_render_state_body
    }

}

impl dyn AbstractProfileViewerScreen {
    /// java: protected final int getBackgroundX()
    pub fn get_background_x_body(__this: &compat::Shared<dyn AbstractProfileViewerScreen>) -> i32 {
        return ((compat::read(__this, |__t| __t.abstract_profile_viewer_screen_base().width)).wrapping_div(2)).wrapping_sub(BACKGROUND_WIDTH.wrapping_div(2));
    }

    /// java: protected final int getBackgroundY()
    pub fn get_background_y_body(__this: &compat::Shared<dyn AbstractProfileViewerScreen>) -> i32 {
        return (((compat::read(__this, |__t| __t.abstract_profile_viewer_screen_base().height)).wrapping_div(2)).wrapping_sub(BACKGROUND_HEIGHT.wrapping_div(2))).wrapping_add(5);
    }

    /// java: @Override public void extractRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_render_state_body(__this: &compat::Shared<dyn AbstractProfileViewerScreen>, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        ();
        gui_graphics_extractor::blit_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), BACKGROUND.get_mut(), ({ let __f = compat::dispatch(__this, |__t| __t.get_background_x()); __f(__this) }), ({ let __f = compat::dispatch(__this, |__t| __t.get_background_y()); __f(__this) }), 0.0f32, 0.0f32, BACKGROUND_WIDTH, BACKGROUND_HEIGHT, BACKGROUND_WIDTH, BACKGROUND_HEIGHT);
    }

}

//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.LoadingProfileViewerScreen by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::abstract_profile_viewer_screen;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::abstract_profile_viewer_screen::AbstractProfileViewerScreen;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::abstract_profile_viewer_screen::AbstractProfileViewerScreenBase;
use mcrust::mcrust::modding::screens::ModScreen;
use mcrust::mcrust::modding::screens::ScreenRenderContext;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::screens::loading_dots_text;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::client::input::mouse_button_info::MouseButtonInfo;
use mcrust::net::minecraft::network::chat::component;

/// java: public final class LoadingProfileViewerScreen extends AbstractProfileViewerScreen
pub struct LoadingProfileViewerScreen {
    /// java: extends AbstractProfileViewerScreen
    pub base: AbstractProfileViewerScreenBase,
    /// java: private final long start = System.currentTimeMillis();
    pub start: i64,
}

impl std::ops::Deref for LoadingProfileViewerScreen {
    type Target = AbstractProfileViewerScreenBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for LoadingProfileViewerScreen {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl LoadingProfileViewerScreen {
    /// java: protected LoadingProfileViewerScreen(String name)
    pub fn new(mut name: String) -> compat::Shared<Self> {
        let (_, __this) = AbstractProfileViewerScreenBase::new_in(component::literal("Skyblocker Profile Viewer - Loading"), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                start: compat::millis(),
            });
            ((__h.clone() as compat::Shared<dyn AbstractProfileViewerScreen>), __h)
        });
        __this
    }

}

impl AbstractProfileViewerScreen for LoadingProfileViewerScreen {
    fn abstract_profile_viewer_screen_base(&self) -> &AbstractProfileViewerScreenBase {
        &self.base
    }
    fn abstract_profile_viewer_screen_base_mut(&mut self) -> &mut AbstractProfileViewerScreenBase {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void extractRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a) (dispatch: the body for this class)
    fn extract_render_state(&self) -> fn(&compat::Shared<dyn AbstractProfileViewerScreen>, &GuiGraphicsExtractor, i32, i32, f32) {
        LoadingProfileViewerScreen::extract_render_state_dyn
    }

}

impl LoadingProfileViewerScreen {
    /// java: @Override public void extractRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_render_state_impl(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        <dyn AbstractProfileViewerScreen>::extract_render_state_body(&compat::view(__this, |a| a as compat::Shared<dyn AbstractProfileViewerScreen>), graphics, mouse_x, mouse_y, a);
        let mut centre_x: i32 = ({ let __f = compat::dispatch(__this, |__t| __t.get_background_x()); __f(&compat::view(__this, |a| a as compat::Shared<dyn AbstractProfileViewerScreen>)) }).wrapping_add((abstract_profile_viewer_screen::BACKGROUND_WIDTH).wrapping_div(2));
        let mut centre_y: i32 = ({ let __f = compat::dispatch(__this, |__t| __t.get_background_y()); __f(&compat::view(__this, |a| a as compat::Shared<dyn AbstractProfileViewerScreen>)) }).wrapping_add((abstract_profile_viewer_screen::BACKGROUND_HEIGHT).wrapping_div(2));
        let mut time_loading: i64 = (compat::millis()).wrapping_sub(compat::read(__this, |__t| __t.start));
        gui_graphics_extractor::centered_text_string(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), "Loading", centre_x, centre_y.wrapping_sub(mcrust::net::minecraft::client::gui::font::LINE_HEIGHT), mcrust::net::minecraft::util::common_colors::WHITE);
        gui_graphics_extractor::centered_text_string(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), loading_dots_text::get(time_loading), centre_x, centre_y.wrapping_add(mcrust::net::minecraft::client::gui::font::LINE_HEIGHT), mcrust::net::minecraft::util::common_colors::WHITE);
    }

    /// LoadingProfileViewerScreen.extractRenderState as the dispatch of AbstractProfileViewerScreen.extractRenderState
    pub fn extract_render_state_dyn(__this: &compat::Shared<dyn AbstractProfileViewerScreen>, __p0: &GuiGraphicsExtractor, __p1: i32, __p2: i32, __p3: f32) {
        Self::extract_render_state_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2, __p3)
    }

}

/// LoadingProfileViewerScreen's `Screen` half (`super`'s fields and widget tree), lent by the game for each call
impl compat::ScreenHalf for LoadingProfileViewerScreen {
    fn screen_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::screens::screen::Screen {
        &mut self.base.base
    }
    fn screen_parts(&mut self) -> (&mut mcrust::net::minecraft::client::gui::screens::screen::Screen, Option<&mut compat::WidgetCache>) {
        let __s = &mut self.base;
        (&mut __s.base, Some(&mut __s.__widgets))
    }
}

/// LoadingProfileViewerScreen as the game's screen: `game::set_screen(Box::new(compat::ScreenAdapter::new(..)))`
impl ModScreen for compat::ScreenAdapter<LoadingProfileViewerScreen> {
    fn get_title(&self) -> String {
        self.title()
    }

    /// java: Screen.tick (the base body)
    fn tick(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        self.lend(screen, |_| ())
    }

    /// java: @Override public void extractRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_render_state(&mut self, screen: &mcrust::net::minecraft::client::gui::screens::screen::Screen, context: &mut ScreenRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.lend_ref(screen, |__h| ({ let __f = compat::dispatch(__h, |__t| __t.extract_render_state()); __f(&compat::view(__h, |a| a as compat::Shared<dyn AbstractProfileViewerScreen>), &context.graphics, mouse_x, mouse_y, a) }))
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

}

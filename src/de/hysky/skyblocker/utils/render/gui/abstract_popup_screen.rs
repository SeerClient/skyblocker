//! Ported from de.hysky.skyblocker.utils.render.gui.AbstractPopupScreen by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::render::gui::filtered_edit_box::FilteredEditBox;
use crate::de::hysky::skyblocker::utils::render::gui::filtered_edit_box::FilteredEditBoxBase;
use mcrust::net::minecraft::client::gui::components::edit_box::EditBox;
use mcrust::net::minecraft::client::gui::font::Font;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::client::gui::components::renderable::Renderable as _;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final Identifier BACKGROUND_TEXTURE = Identifier.withDefaultNamespace("popup/background");
pub static BACKGROUND_TEXTURE: compat::MainThread<Identifier> = compat::MainThread::new(|| Identifier::with_default_namespace("popup/background"));

/// java: public class AbstractPopupScreen extends Screen
pub struct AbstractPopupScreenBase {
    /// java: extends net.minecraft.client.gui.screens.Screen
    pub base: mcrust::net::minecraft::client::gui::screens::screen::Screen,
    /// java: public final Screen backgroundScreen;
    pub background_screen: mcrust::net::minecraft::client::gui::screens::screen::Screen,
    /// the screen's widgets as this object last saw them (see `compat::WidgetCache`)
    pub __widgets: compat::WidgetCache,
}

impl std::ops::Deref for AbstractPopupScreenBase {
    type Target = mcrust::net::minecraft::client::gui::screens::screen::Screen;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for AbstractPopupScreenBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl AbstractPopupScreenBase {
    /// java: protected AbstractPopupScreen(Component title, Screen backgroundScreen) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_in<__R>(mut title: MutableComponent, mut background_screen: mcrust::net::minecraft::client::gui::screens::screen::Screen, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn AbstractPopupScreen>, __R)) -> (compat::Shared<dyn AbstractPopupScreen>, __R) {
        // TODO(fabric2seer): the mod keeps the game's `Screen` (Java holds a reference to the live object); the port's `Screen` is not a value to copy and the port lends the mod no handle to it
        //   java: this.backgroundScreen = backgroundScreen;
        let this = Self {
            base: mcrust::net::minecraft::client::gui::screens::screen::Screen::new(title.get_string()),
            __widgets: Default::default(),
            background_screen: todo!("fabric2seer: the field value (TODO above)"),
        };
        let (__h, __r) = __wrap(this);
        let __this = __h;
        (__this, __r)
    }

    /// java: protected AbstractPopupScreen(Component title, Screen backgroundScreen)
    pub fn new(mut title: MutableComponent, mut background_screen: mcrust::net::minecraft::client::gui::screens::screen::Screen) -> compat::Shared<Self> {
        Self::new_in(title, background_screen, |this| {
            let __h = compat::shared(this);
            ((__h.clone() as compat::Shared<dyn AbstractPopupScreen>), __h)
        }).1
    }

    /// java: public static void extractPopupBackground(GuiGraphicsExtractor graphics, int x, int y, int width, int height)
    pub fn extract_popup_background(graphics: &GuiGraphicsExtractor, mut x: i32, mut y: i32, mut width: i32, mut height: i32) {
        gui_graphics_extractor::blit_sprite_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), BACKGROUND_TEXTURE.get_mut(), x.wrapping_sub(18), y.wrapping_sub(18), width.wrapping_add(36), height.wrapping_add(36));
    }

}

/// java: the virtual methods of AbstractPopupScreen (implemented by AbstractPopupScreenBase and every subclass)
pub trait AbstractPopupScreen: compat::DynAny {
    fn abstract_popup_screen_base(&self) -> &AbstractPopupScreenBase;
    fn abstract_popup_screen_base_mut(&mut self) -> &mut AbstractPopupScreenBase;
    fn as_any(&self) -> &dyn std::any::Any;

    /// java: @Override public void onClose() (dispatch: the body for this class)
    fn on_close(&self) -> fn(&compat::Shared<dyn AbstractPopupScreen>) {
        <dyn AbstractPopupScreen>::on_close_body
    }

    /// java: @Override public void extractBackground(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a) (dispatch: the body for this class)
    fn extract_background(&self) -> fn(&compat::Shared<dyn AbstractPopupScreen>, &mut GuiGraphicsExtractor, i32, i32, f32) {
        <dyn AbstractPopupScreen>::extract_background_body
    }

    /// java: @Override protected void init() (dispatch: the body for this class)
    fn init(&self) -> fn(&compat::Shared<dyn AbstractPopupScreen>) {
        <dyn AbstractPopupScreen>::init_body
    }

    /// java: @Override protected void repositionElements() (dispatch: the body for this class)
    fn reposition_elements(&self) -> fn(&compat::Shared<dyn AbstractPopupScreen>) {
        <dyn AbstractPopupScreen>::reposition_elements_body
    }

    /// java: @Override public void added() (dispatch: the body for this class)
    fn added(&self) -> fn(&compat::Shared<dyn AbstractPopupScreen>) {
        <dyn AbstractPopupScreen>::added_body
    }

}

impl dyn AbstractPopupScreen {
    /// java: @Override public void onClose()
    pub fn on_close_body(__this: &compat::Shared<dyn AbstractPopupScreen>) {
        mcrust::mcrust::modding::game::set_screen_to_parent();
    }

    /// java: @Override public void extractBackground(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_background_body(__this: &compat::Shared<dyn AbstractPopupScreen>, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        // TODO(fabric2seer): `Screen.extractBackground` has no Rust method in the port (looked for `extract_background` in Screen and its supertypes)
        // rust: __this.lock().borrow().abstract_popup_screen_base().background_screen.extract_background(graphics, -1, -1, a);
        gui_graphics_extractor::next_stratum();
        // TODO(fabric2seer): `extract_render_state` is a method of the port's trait `Renderable`, which the port's `Screen` does not implement
        // rust: { let (__q0, __q1, __q2, __q3, __q4,) = (mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), -1, -1, a,); __this.lock().borrow_mut().abstract_popup_screen_base_mut().background_screen.extract_render_state(__q0, __q1, __q2, __q3, __q4) };
        gui_graphics_extractor::next_stratum();
        // TODO(fabric2seer): `Screen.extractTransparentBackground` has no Rust method in the port (looked for `extract_transparent_background` in Screen and its supertypes)
        // rust: __this.lock().borrow().abstract_popup_screen_base().extract_transparent_background(graphics);
    }

    /// java: @Override protected void init()
    pub fn init_body(__this: &compat::Shared<dyn AbstractPopupScreen>) {
        ();
        ({ let __f = compat::dispatch(__this, |__t| __t.reposition_elements()); __f(__this) });
    }

    /// java: @Override protected void repositionElements()
    pub fn reposition_elements_body(__this: &compat::Shared<dyn AbstractPopupScreen>) {
        // TODO(fabric2seer): `resize` takes 3 argument(s) in the port (`width: i32, height: i32, build: impl FnOnce(&mut Screen)`), Java passes 2
        // rust: let __r0 = compat::read(__this, |__t| __t.abstract_popup_screen_base().width);
        // rust: let __r1 = compat::read(__this, |__t| __t.abstract_popup_screen_base().height);
        // rust: { let (__q0, __q1,) = (__r0, __r1,); __this.lock().borrow_mut().abstract_popup_screen_base_mut().background_screen.resize(__q0, __q1) };
    }

    /// java: @Override public void added()
    pub fn added_body(__this: &compat::Shared<dyn AbstractPopupScreen>) {
        ();
        __this.lock().borrow_mut().abstract_popup_screen_base_mut().background_screen.clear_focus();
    }

}

impl AbstractPopupScreen for AbstractPopupScreenBase {
    fn abstract_popup_screen_base(&self) -> &AbstractPopupScreenBase {
        &self
    }
    fn abstract_popup_screen_base_mut(&mut self) -> &mut AbstractPopupScreenBase {
        &mut self
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}

/// java: public static class EnterConfirmTextFieldWidget extends FilteredEditBox
#[derive(Clone)]
pub struct EnterConfirmTextFieldWidget {
    /// java: extends FilteredEditBox
    pub base: FilteredEditBoxBase,
    /// java: private final Runnable onEnter;
    pub on_enter: Option<Arc<dyn Fn()>>,
}

impl std::ops::Deref for EnterConfirmTextFieldWidget {
    type Target = FilteredEditBoxBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for EnterConfirmTextFieldWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl EnterConfirmTextFieldWidget {
    /// java: public EnterConfirmTextFieldWidget(Font font, int width, int height, Component text, Runnable onEnter)
    pub fn new(mut font: Font, mut width: i32, mut height: i32, mut text: MutableComponent, on_enter: Arc<dyn Fn()>) -> compat::Shared<Self> {
        let __this = Self::new_7(compat::font(), 0, 0, width, height, text.clone(), on_enter.clone());
        __this
    }

    /// java: public EnterConfirmTextFieldWidget(Font font, int x, int y, int width, int height, Component text, Runnable onEnter)
    pub fn new_7(mut font: Font, mut x: i32, mut y: i32, mut width: i32, mut height: i32, mut text: MutableComponent, on_enter: Arc<dyn Fn()>) -> compat::Shared<Self> {
        let __this = Self::new_8(compat::font(), x, y, width, height, None, text.clone(), on_enter.clone());
        __this
    }

    /// java: public EnterConfirmTextFieldWidget(Font font, int x, int y, int width, int height, @Nullable EditBox copyFrom, Component text, Runnable onEnter)
    pub fn new_8(mut font: Font, mut x: i32, mut y: i32, mut width: i32, mut height: i32, mut copy_from: Option<EditBox>, mut text: MutableComponent, on_enter: Arc<dyn Fn()>) -> compat::Shared<Self> {
        let __init_on_enter = Some(on_enter.clone());
        let (_, __this) = FilteredEditBoxBase::new_7_in(compat::font(), x, y, width, height, copy_from.clone(), text.clone(), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                on_enter: __init_on_enter,
            });
            ((__h.clone() as compat::Shared<dyn FilteredEditBox>), __h)
        });
        __this
    }

    /// java: @Override public boolean keyPressed(KeyEvent input)
    pub fn key_pressed(__this: &compat::Shared<Self>, input: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        // TODO(fabric2seer): `key_pressed` takes 5 argument(s) in the port (`splitter: &StringSplitter<W>, event: KeyEvent, focused: bool, active: bool, clipboard: &mut dyn ClipboardAccess`), Java passes 1
        if !(__this.lock().borrow_mut().base.key_pressed(input)) {
            if input.is_confirmation() {
                ((compat::read(__this, |__t| __t.on_enter.clone()).unwrap()))();
                return true;
            }
        } else {
            return true;
        }
        return false;
    }

}

impl FilteredEditBox for EnterConfirmTextFieldWidget {
    fn filtered_edit_box_base(&self) -> &FilteredEditBoxBase {
        &self.base
    }
    fn filtered_edit_box_base_mut(&mut self) -> &mut FilteredEditBoxBase {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}

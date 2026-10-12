//! Ported from de.hysky.skyblocker.utils.render.gui.BasicToast by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::client::gui::components::toasts::toast_manager::ToastManager;
use mcrust::net::minecraft::client::gui::font::Font;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::util::formatted_char_sequence::FormattedCharSequence;
use mcrust::net::minecraft::util::formatted_char_sequence::Seq;
use mcrust::net::minecraft::client::gui::components::toasts::toast::Token;
use mcrust::net::minecraft::client::gui::components::toasts::toast::Visibility;
use mcrust::net::minecraft::network::chat::formatted_text::FormattedText;

/// java: private static final Identifier TEXTURE = SkyblockerMod.id("notification");
pub static TEXTURE: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("notification"));

/// java: public class BasicToast implements Toast
pub struct BasicToast {
    /// java: private final long displayDuration;
    pub display_duration: i64,
    /// java: private final @Nullable ItemStack icon;
    pub icon: Option<mcrust::net::minecraft::world::item::item_stack::ItemStack>,
    /// java: private final List<FormattedCharSequence> lines;
    /// (the port's `Font.split` answers each line as the `FormattedText` its draws take)
    pub lines: Vec<FormattedText>,
    /// java: private final int width;
    pub width: i32,
    /// java: private final Object token;
    /// (the port's `Toast.getToken` answers its closed set of tokens, `Token`; see GAPS_batchA.md)
    pub token: Token,
    /// java: private Visibility visibility = Visibility.SHOW;
    pub visibility: Visibility,
}

impl BasicToast {
    /// java: public BasicToast(Component message, long displayDuration, @Nullable ItemStack icon, Object token)
    pub fn new(mut message: MutableComponent, mut display_duration: i64, mut icon: Option<mcrust::net::minecraft::world::item::item_stack::ItemStack>, token: Token) -> Self {
        let mut text_renderer: Font = compat::font();
        let __init_lines = text_renderer.split(&mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(message.clone()), 200);
        let __init_display_duration = display_duration;
        let __init_icon = icon.clone();
        let __init_token = token;
        let mut this = Self {
            display_duration: __init_display_duration,
            icon: __init_icon,
            lines: __init_lines,
            width: 0,
            token: __init_token,
            // java: private Visibility visibility = Visibility.SHOW;
            visibility: Visibility::Show,
        };
        this.width = (this.lines.iter().cloned().map(|__a0| text_renderer.width_of_cached(&__a0)).max().unwrap_or(200)).wrapping_add(if icon.is_none() { 10 } else { 30 });
        this
    }

    /// java: public BasicToast(Component message, long displayDuration, @Nullable ItemStack icon)
    pub fn new_3(mut message: MutableComponent, mut display_duration: i64, mut icon: Option<mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> Self {
        let mut this = Self::new(message.clone(), display_duration, icon.clone(), Token::NoToken);
        this
    }

    /// java: @Override public Visibility getWantedVisibility()
    pub fn get_wanted_visibility(&self) -> Visibility {
        return self.visibility;
    }

    /// java: @Override public void update(ToastManager manager, long time)
    pub fn update(&mut self, manager: &ToastManager, mut time: i64) {
        if time > self.display_duration {
            // java: visibility = Visibility.HIDE;
            self.visibility = Visibility::Hide;
        }
    }

    /// java: @Override public void extractRenderState(GuiGraphicsExtractor graphics, Font textRenderer, long startTime)
    pub fn extract_render_state(&self, graphics: &GuiGraphicsExtractor, text_renderer: &Font, mut start_time: i64) {
        gui_graphics_extractor::blit_sprite_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), TEXTURE.get_mut(), 0, 0, self.width(), self.height());
        let mut offset: i32;
        if self.icon.is_some() {
            graphics.fake_item(self.icon.as_ref().unwrap(), 4, 4);
            offset = 20;
        } else {
            offset = 0;
        }
        for i in 0..(self.lines.len() as i32) {
            gui_graphics_extractor::draw_formatted_char_sequence_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &self.lines[(i) as usize], 4i32.wrapping_add(offset), 8i32.wrapping_add(i.wrapping_mul(12)), -1, false);
        }
    }

    /// java: @Override public int height()
    pub fn height(&self) -> i32 {
        return (8i32.wrapping_add(4)).wrapping_add(((self.lines.len() as i32).max(1)).wrapping_mul(12));
    }

    /// java: @Override public int width()
    pub fn width(&self) -> i32 {
        return self.width;
    }

    /// java: @Override public Object getToken()
    pub fn get_token(&self) -> Token {
        return self.token;
    }

}

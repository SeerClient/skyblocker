//! Ported from de.hysky.skyblocker.compatibility.ModernUICompatibility by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::client::gui::font::Font;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::util::formatted_char_sequence::FormattedCharSequence;
use mcrust::net::minecraft::util::formatted_char_sequence::Seq;
use mcrust::org::joml::matrix3x2f_stack::Matrix3x2fStack;

/// java: public static final boolean MODERNUI_ENABLED = FabricLoader.getInstance().isModLoaded("modernui");
pub static MODERNUI_ENABLED: LazyLock<bool> = LazyLock::new(|| mcrust::mcrust::modding::loader::is_mod_loaded("modernui"));
/// java: private static final boolean IS_TEXT_ENGINE_ENABLED = isTextEngineEnabled();
pub static IS_TEXT_ENGINE_ENABLED: LazyLock<bool> = LazyLock::new(|| ModernUICompatibility::is_text_engine_enabled());

/// java: public class ModernUICompatibility
#[derive(Clone)]
pub struct ModernUICompatibility {}

impl ModernUICompatibility {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: private static boolean isTextEngineEnabled()
    pub fn is_text_engine_enabled() -> bool {
        if !(*MODERNUI_ENABLED) {
            return false;
        }
        // TODO(fabric2seer): static call MethodHandles.publicLookup(..)
        //   java: MethodHandles.Lookup lookup = MethodHandles.publicLookup();
        let mut lookup: crate::unresolved::Lookup = todo!("fabric2seer: the initialiser of `lookup` (TODO above)");
        let mut clazz: crate::unresolved::Class;
        // java: try {..} catch (ClassNotFoundException e) - an exception here is a panic, caught by `catch_unwind`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            // TODO(fabric2seer): static call Class.forName(..)
            //   java: clazz = Class.forName("icyllis.modernui.mc.ModernUIMod");
        }));
        if let Err(e) = __result {
            logger().error(&format!("[Skyblocker ModernUI Compat] Could not find icyllis.modernui.mc.ModernUIMod"));
            return true;
        }
        // java: try {..} catch (NoSuchMethodException | IllegalAccessException e) catch (Throwable e) - the body runs inline: a Rust body does not throw checked exceptions (a panic in it propagates)
        {
            // TODO(fabric2seer): not translated (expected ';' at 'methodHandle' (offset 1095))
            //   java: MethodHandle methodHandle = lookup.findStatic(
            //   java: 					clazz,
            //   java: 					"isTextEngineEnabled",
            //   java: 					MethodType.methodType(boolean.class));
            // TODO(fabric2seer): unknown name `methodHandle`
            //   java: return Boolean.TRUE.equals(methodHandle.invoke());
        }
        // TODO(fabric2seer): the `catch (NoSuchMethodException | IllegalAccessException e)` block is not reached in Rust - the try body returns, breaks or continues, so it cannot run inside `catch_unwind`
        //   java: {
        //   java: 			LOGGER.error("[Skyblocker ModernUI Compat] Could not find isTextEngineEnabled method", e);
        //   java: 			return true;
        //   java: 		}
        // TODO(fabric2seer): the `catch (Throwable e)` block is not reached in Rust - the try body returns, breaks or continues, so it cannot run inside `catch_unwind`
        //   java: {
        //   java: 			LOGGER.error("[Skyblocker ModernUI Compat] Could not invoke isTextEngineEnabled", e);
        //   java: 			return true;
        //   java: 		}
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public static boolean extractOutlinedText(GuiGraphicsExtractor context, FormattedCharSequence text, FormattedCharSequence outlineText, int x, int y, int color, int outlineColor)
    pub fn extract_outlined_text(context: &GuiGraphicsExtractor, text: &Seq, outline_text: &Seq, mut x: i32, mut y: i32, mut color: i32, mut outline_color: i32) -> bool {
        if !(*IS_TEXT_ENGINE_ENABLED) {
            return false;
        }
        let mut offset: f32 = 0.5f32;
        let mut text_renderer: Font = compat::font();
        // the port draws a `FormattedText` where Java draws the `FormattedCharSequence`
        let __outline_text = sequence_text(&**outline_text);
        let __text = sequence_text(&**text);
        let __pose_1 = gui_graphics_extractor::push_pose();
        gui_graphics_extractor::pose_translate((offset) as f32, (0) as f32);
        gui_graphics_extractor::draw_formatted_char_sequence_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &__outline_text, x, y, outline_color, false);
        gui_graphics_extractor::pop_pose(__pose_1);
        let __pose_2 = gui_graphics_extractor::push_pose();
        gui_graphics_extractor::pose_translate((offset) as f32, (offset) as f32);
        gui_graphics_extractor::draw_formatted_char_sequence_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &__outline_text, x, y, outline_color, false);
        gui_graphics_extractor::pop_pose(__pose_2);
        let __pose_3 = gui_graphics_extractor::push_pose();
        gui_graphics_extractor::pose_translate((offset) as f32, ((-offset)) as f32);
        gui_graphics_extractor::draw_formatted_char_sequence_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &__outline_text, x, y, outline_color, false);
        gui_graphics_extractor::pop_pose(__pose_3);
        let __pose_4 = gui_graphics_extractor::push_pose();
        gui_graphics_extractor::pose_translate(((-offset)) as f32, (0) as f32);
        gui_graphics_extractor::draw_formatted_char_sequence_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &__outline_text, x, y, outline_color, false);
        gui_graphics_extractor::pop_pose(__pose_4);
        let __pose_5 = gui_graphics_extractor::push_pose();
        gui_graphics_extractor::pose_translate(((-offset)) as f32, (offset) as f32);
        gui_graphics_extractor::draw_formatted_char_sequence_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &__outline_text, x, y, outline_color, false);
        gui_graphics_extractor::pop_pose(__pose_5);
        let __pose_6 = gui_graphics_extractor::push_pose();
        gui_graphics_extractor::pose_translate(((-offset)) as f32, ((-offset)) as f32);
        gui_graphics_extractor::draw_formatted_char_sequence_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &__outline_text, x, y, outline_color, false);
        gui_graphics_extractor::pop_pose(__pose_6);
        let __pose_7 = gui_graphics_extractor::push_pose();
        gui_graphics_extractor::pose_translate((0) as f32, (offset) as f32);
        gui_graphics_extractor::draw_formatted_char_sequence_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &__outline_text, x, y, outline_color, false);
        gui_graphics_extractor::pop_pose(__pose_7);
        let __pose_8 = gui_graphics_extractor::push_pose();
        gui_graphics_extractor::pose_translate((0) as f32, ((-offset)) as f32);
        gui_graphics_extractor::draw_formatted_char_sequence_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &__outline_text, x, y, outline_color, false);
        gui_graphics_extractor::pop_pose(__pose_8);
        gui_graphics_extractor::draw_formatted_char_sequence_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &__text, x, y, color, false);
        return true;
    }

}

/// The port draws a `FormattedText` where Java draws a `FormattedCharSequence`: the sequence's styled
/// characters, gathered back into runs of one style each.
fn sequence_text(seq: &dyn FormattedCharSequence) -> mcrust::net::minecraft::network::chat::formatted_text::FormattedText {
    use mcrust::net::minecraft::network::chat::formatted_text::FormattedText;
    use mcrust::net::minecraft::network::chat::style::Style;
    let mut runs: Vec<(Style, String)> = Vec::new();
    seq.accept(&mut |_position: i32, style: &Style, codepoint: u32| {
        if let Some(c) = char::from_u32(codepoint) {
            match runs.last_mut() {
                Some((s, text)) if s == style => text.push(c),
                _ => runs.push((style.clone(), c.to_string())),
            }
        }
        true
    });
    FormattedText::composite(runs.into_iter().map(|(style, text)| FormattedText::of_styled(text, style)).collect())
}

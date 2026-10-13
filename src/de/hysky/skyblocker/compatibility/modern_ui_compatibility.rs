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
        //   java: MethodHandles.Lookup lookup = MethodHandles.publicLookup();
        let lookup: crate::unresolved::Lookup = public_lookup();
        // java: try { clazz = Class.forName("icyllis.modernui.mc.ModernUIMod"); } catch (ClassNotFoundException e) - a panic, caught by `catch_unwind`
        let clazz: crate::unresolved::Class = match std::panic::catch_unwind(|| class_for_name("icyllis.modernui.mc.ModernUIMod")) {
            Ok(clazz) => clazz,
            Err(e) => {
                logger().error(&format!("[Skyblocker ModernUI Compat] Could not find icyllis.modernui.mc.ModernUIMod {}", compat::panic_message(&e)));
                return true;
            }
        };
        // java: try {..} catch (NoSuchMethodException | IllegalAccessException e) {..} catch (Throwable e) {..}
        //   java: MethodHandle methodHandle = lookup.findStatic(clazz, "isTextEngineEnabled", MethodType.methodType(boolean.class));
        let method_handle: crate::unresolved::MethodHandle = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| find_method(&lookup, &clazz, "isTextEngineEnabled"))) {
            Ok(handle) => handle,
            Err(e) => {
                logger().error(&format!("[Skyblocker ModernUI Compat] Could not find isTextEngineEnabled method {}", compat::panic_message(&e)));
                return true;
            }
        };
        //   java: return Boolean.TRUE.equals(methodHandle.invoke());
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| invoke(&method_handle))) {
            Ok(answer) => answer.downcast_ref::<bool>().copied() == Some(true),
            Err(e) => {
                logger().error(&format!("[Skyblocker ModernUI Compat] Could not invoke isTextEngineEnabled {}", compat::panic_message(&e)));
                true
            }
        }
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

/// `Class.forName(name)` - another mod's JVM class.
///
/// rust: port gap - SeerClient runs no JVM: there are no Java mod classes to load or reflect on (`Class.forName`,
/// `MethodHandles.Lookup.findStatic/findVirtual`, `Field.get`, `MethodHandle.invoke`), so the lookup fails as
/// `ClassNotFoundException` does in Java for an absent class (GAPS_batchF, GAPS_M #11). Every caller only gets here
/// after `FabricLoader.isModLoaded(..)` answered true for a SeerClient mod of that id.
fn class_for_name(name: &str) -> crate::unresolved::Class {
    panic!("ClassNotFoundException: {name}");
}

/// `MethodHandles.publicLookup()` - see `class_for_name` (the lookup exists; nothing can be found through it).
fn public_lookup() -> crate::unresolved::Lookup {
    crate::unresolved::Lookup
}

/// `lookup.findStatic(clazz, name, type)` / `findVirtual(..)` - see `class_for_name`.
fn find_method(_lookup: &crate::unresolved::Lookup, _clazz: &crate::unresolved::Class, name: &str) -> crate::unresolved::MethodHandle {
    panic!("NoSuchMethodException: {name}");
}

/// `methodHandle.invoke(..)` - see `class_for_name` (no handle can have been found).
fn invoke(_handle: &crate::unresolved::MethodHandle) -> compat::Object {
    panic!("WrongMethodTypeException: no JVM method behind the handle");
}

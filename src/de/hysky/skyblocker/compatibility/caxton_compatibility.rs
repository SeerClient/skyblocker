//! Ported from de.hysky.skyblocker.compatibility.CaxtonCompatibility by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::com::mojang::blaze3d::pipeline::render_pipeline::RenderPipeline;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::util::formatted_char_sequence::FormattedCharSequence;
use mcrust::net::minecraft::util::formatted_char_sequence::Seq;

/// java: public static final boolean CAXTON_ENABLED = FabricLoader.getInstance().isModLoaded("caxton");
pub static CAXTON_ENABLED: LazyLock<bool> = LazyLock::new(|| mcrust::mcrust::modding::loader::is_mod_loaded("caxton"));
/// java: private static final @Nullable MethodHandle HANDLE = createHandle();
pub static HANDLE: compat::MainThread<Option<crate::unresolved::MethodHandle>> = compat::MainThread::new(|| CaxtonCompatibility::create_handle());
/// java: private static boolean errored = false;
pub static ERRORED: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));

/// java: public final class CaxtonCompatibility
#[derive(Clone)]
pub struct CaxtonCompatibility {}

impl CaxtonCompatibility {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: private static @Nullable MethodHandle createHandle()
    pub fn create_handle() -> Option<crate::unresolved::MethodHandle> {
        if !(*CAXTON_ENABLED) {
            return None;
        }
        //   java: MethodHandles.Lookup lookup = MethodHandles.publicLookup();
        let lookup: crate::unresolved::Lookup = compat::jvm_public_lookup();
        // java: try { clazz = Class.forName("xyz.flirora.caxton.render.Voepfxo"); } catch (ClassNotFoundException e) - a panic, caught by `catch_unwind`
        let clazz: crate::unresolved::Class = match std::panic::catch_unwind(|| compat::jvm_class_for_name("xyz.flirora.caxton.render.Voepfxo")) {
            Ok(clazz) => clazz,
            Err(e) => {
                logger().error(&format!("[Skyblocker Caxton Compat] Could not find xyz.flirora.caxton.render.Voepfxo {}", compat::panic_message(&e)));
                return None;
            }
        };
        // java: try {..} catch (NoSuchMethodException | IllegalAccessException e) - a panic, caught by `catch_unwind`
        //   java: return lookup.findStatic(clazz, "drawText4Way",
        //   java: 		MethodType.methodType(void.class, GuiGraphicsExtractor.class, Font.class, FormattedCharSequence.class, float.class, float.class, int.class, int.class));
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| compat::jvm_find_method(&lookup, &clazz, "drawText4Way"))) {
            Ok(handle) => Some(handle),
            Err(e) => {
                logger().error(&format!("[Skyblocker Caxton Compat] Could not find drawText4Way method {}", compat::panic_message(&e)));
                None
            }
        }
    }

    /// java: private static Optional<RenderPipeline> getCaxtonPipeline(String name)
    pub fn get_caxton_pipeline(name: &str) -> Option<RenderPipeline> {
        if !(*CAXTON_ENABLED) {
            return None;
        }
        // java: try { clazz = Class.forName("xyz.flirora.caxton.render.CaxtonShaders"); } catch (ClassNotFoundException e) - a panic, caught by `catch_unwind`
        let clazz: crate::unresolved::Class = match std::panic::catch_unwind(|| compat::jvm_class_for_name("xyz.flirora.caxton.render.CaxtonShaders")) {
            Ok(clazz) => clazz,
            Err(e) => {
                logger().error(&format!("[Skyblocker Caxton Compat] Could not find xyz.flirora.caxton.render.CaxtonShaders {}", compat::panic_message(&e)));
                return None;
            }
        };
        // java: try {..} catch (IllegalAccessException | NoSuchFieldException e) - a panic, caught by `catch_unwind`
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| -> Option<RenderPipeline> {
            //   java: return Optional.of((RenderPipeline) clazz.getField(name).get(null));
            // `clazz.getField(name)` - see `class_for_name` (no class can have been found to read a field of)
            let _ = &clazz;
            panic!("NoSuchFieldException: {}", name)
        })) {
            Ok(__v) => return __v,
            Err(e) => {
                logger().error(&format!("[Skyblocker Caxton Compat] Could not find {} shader {}", name, compat::panic_message(&e)));
                return None;
            }
        }
    }

    /// java: public static boolean drawOutlinedText(GuiGraphicsExtractor graphics, FormattedCharSequence text, float x, float y, int color, int outlineColor)
    pub fn draw_outlined_text(graphics: &GuiGraphicsExtractor, text: &Seq, mut x: f32, mut y: f32, mut color: i32, mut outline_color: i32) -> bool {
        if HANDLE.get_mut().is_none() || compat::get_state(&ERRORED) {
            return false;
        }
        // java: try {..} catch (Throwable e) - an exception here is a panic, caught by `catch_unwind`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            //   java: HANDLE.invoke(graphics, Minecraft.getInstance().font, text, x, y, color, outlineColor);
            if let Some(handle) = HANDLE.get().as_ref() {
                let _ = (graphics, text, x, y, color, outline_color);
                compat::jvm_invoke(handle);
            }
        }));
        if let Err(e) = __result {
            logger().error(&format!("[Skyblocker Caxton Compat] Could not invoke drawText4Way"));
            compat::set_state(&ERRORED, true);
            return false;
        }
        return true;
    }

    /// java: public static Optional<RenderPipeline> getSeeThroughTextPipeline()
    pub fn get_see_through_text_pipeline() -> Option<RenderPipeline> {
        return CaxtonCompatibility::get_caxton_pipeline("TEXT_SEETHROUGH");
    }

    /// java: public static Optional<RenderPipeline> getTextPipeline()
    pub fn get_text_pipeline() -> Option<RenderPipeline> {
        return CaxtonCompatibility::get_caxton_pipeline("TEXT");
    }

}


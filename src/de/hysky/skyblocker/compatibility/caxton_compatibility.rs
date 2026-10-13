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
        let lookup: crate::unresolved::Lookup = public_lookup();
        // java: try { clazz = Class.forName("xyz.flirora.caxton.render.Voepfxo"); } catch (ClassNotFoundException e) - a panic, caught by `catch_unwind`
        let clazz: crate::unresolved::Class = match std::panic::catch_unwind(|| class_for_name("xyz.flirora.caxton.render.Voepfxo")) {
            Ok(clazz) => clazz,
            Err(e) => {
                logger().error(&format!("[Skyblocker Caxton Compat] Could not find xyz.flirora.caxton.render.Voepfxo {}", compat::panic_message(&e)));
                return None;
            }
        };
        // java: try {..} catch (NoSuchMethodException | IllegalAccessException e) - a panic, caught by `catch_unwind`
        //   java: return lookup.findStatic(clazz, "drawText4Way",
        //   java: 		MethodType.methodType(void.class, GuiGraphicsExtractor.class, Font.class, FormattedCharSequence.class, float.class, float.class, int.class, int.class));
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| find_method(&lookup, &clazz, "drawText4Way"))) {
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
        let clazz: crate::unresolved::Class = match std::panic::catch_unwind(|| class_for_name("xyz.flirora.caxton.render.CaxtonShaders")) {
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
                invoke(handle);
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

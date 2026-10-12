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
        // TODO(fabric2seer): static call MethodHandles.publicLookup(..)
        //   java: MethodHandles.Lookup lookup = MethodHandles.publicLookup();
        let mut lookup: crate::unresolved::Lookup = todo!("fabric2seer: the initialiser of `lookup` (TODO above)");
        let mut clazz: crate::unresolved::Class;
        // java: try {..} catch (ClassNotFoundException e) - an exception here is a panic, caught by `catch_unwind`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            // TODO(fabric2seer): static call Class.forName(..)
            //   java: clazz = Class.forName("xyz.flirora.caxton.render.Voepfxo");
        }));
        if let Err(e) = __result {
            logger().error(&format!("[Skyblocker Caxton Compat] Could not find xyz.flirora.caxton.render.Voepfxo"));
            return None;
        }
        // java: try {..} catch (NoSuchMethodException | IllegalAccessException e) - an exception here is a panic, caught by `catch_unwind`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            // TODO(fabric2seer): not translated (expected '.' at ',' (offset 1272))
            //   java: return lookup.findStatic(
            //   java: 					clazz,
            //   java: 					"drawText4Way",
            //   java: 					MethodType.methodType(void.class, GuiGraphicsExtractor.class, Font.class, FormattedCharSequence.class, float.class, float.class, int.class, int.class));
        }));
        if let Err(e) = __result {
            logger().error(&format!("[Skyblocker Caxton Compat] Could not find drawText4Way method"));
            return None;
        }
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: private static Optional<RenderPipeline> getCaxtonPipeline(String name)
    pub fn get_caxton_pipeline(name: &str) -> Option<RenderPipeline> {
        if !(*CAXTON_ENABLED) {
            return None;
        }
        let mut clazz: crate::unresolved::Class;
        // java: try {..} catch (ClassNotFoundException e) - an exception here is a panic, caught by `catch_unwind`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            // TODO(fabric2seer): static call Class.forName(..)
            //   java: clazz = Class.forName("xyz.flirora.caxton.render.CaxtonShaders");
        }));
        if let Err(e) = __result {
            logger().error(&format!("[Skyblocker Caxton Compat] Could not find xyz.flirora.caxton.render.CaxtonShaders"));
            return None;
        }
        // java: try {..} catch (IllegalAccessException | NoSuchFieldException e) - an exception here is a panic, caught by `catch_unwind`; the body's `return` is the closure's value
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| -> Option<RenderPipeline> {
            // java: return Optional.of((RenderPipeline) clazz.getField(name).get(null));
            // rust: port gap - reflection into another mod's classes (`Class.getField(..).get(null)`) has no Rust
            // counterpart; it fails as `NoSuchFieldException` would (GAPS_batchF.md)
            panic!("NoSuchFieldException: {}", name)
        })) {
            Ok(__v) => return __v,
            Err(e) => {
                logger().error(&format!("[Skyblocker Caxton Compat] Could not find {} shader", name));
                return None;
            }
        }
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public static boolean drawOutlinedText(GuiGraphicsExtractor graphics, FormattedCharSequence text, float x, float y, int color, int outlineColor)
    pub fn draw_outlined_text(graphics: &GuiGraphicsExtractor, text: &Seq, mut x: f32, mut y: f32, mut color: i32, mut outline_color: i32) -> bool {
        if HANDLE.get_mut().is_none() || compat::get_state(&ERRORED) {
            return false;
        }
        // java: try {..} catch (Throwable e) - an exception here is a panic, caught by `catch_unwind`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            // TODO(fabric2seer): `.invoke(..)` on a MethodHandle
            //   java: HANDLE.invoke(graphics, Minecraft.getInstance().font, text, x, y, color, outlineColor);
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

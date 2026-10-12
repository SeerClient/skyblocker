//! Ported from de.hysky.skyblocker.compatibility.IrisCompatibility by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::render::skyblocker_render_pipelines;
use mcrust::com::mojang::blaze3d::pipeline::render_pipeline::RenderPipeline;

/// java: private static final boolean IRIS_ENABLED = FabricLoader.getInstance().isModLoaded("iris");
pub static IRIS_ENABLED: LazyLock<bool> = LazyLock::new(|| mcrust::mcrust::modding::loader::is_mod_loaded("iris"));
/// java: private static final String IRIS_API_CLASS = "net.irisshaders.iris.api.v0.IrisApi";
pub const IRIS_API_CLASS: &str = "net.irisshaders.iris.api.v0.IrisApi";
/// java: private static final String IRIS_PROGRAM_CLASS = "net.irisshaders.iris.api.v0.IrisProgram";
pub const IRIS_PROGRAM_CLASS: &str = "net.irisshaders.iris.api.v0.IrisProgram";
/// java: private static final MethodHandle GET_IRIS_API = getIrisApiHandle();
pub static GET_IRIS_API: compat::MainThread<crate::unresolved::MethodHandle> = compat::MainThread::new(|| IrisCompatibility::get_iris_api_handle().unwrap());
/// java: private static final MethodHandle REGISTER_PIPELINE = getRegisterPipelineHandle();
pub static REGISTER_PIPELINE: compat::MainThread<crate::unresolved::MethodHandle> = compat::MainThread::new(|| IrisCompatibility::get_register_pipeline_handle().unwrap());
/// java: private static final MethodHandle GET_IRIS_PROGRAM = getIrisProgramHandle();
pub static GET_IRIS_PROGRAM: compat::MainThread<crate::unresolved::MethodHandle> = compat::MainThread::new(|| IrisCompatibility::get_iris_program_handle().unwrap());

/// java: public class IrisCompatibility
#[derive(Clone)]
pub struct IrisCompatibility {}

impl IrisCompatibility {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static void assignPipelines()
    pub fn assign_pipelines() {
        if *IRIS_ENABLED {
            IrisCompatibility::assign_pipeline(skyblocker_render_pipelines::FILLED_INSTANCED.get_mut(), "BASIC");
            IrisCompatibility::assign_pipeline(skyblocker_render_pipelines::FILLED_THROUGH_WALLS_INSTANCED.get_mut(), "BASIC");
            IrisCompatibility::assign_pipeline(&mcrust::net::minecraft::client::renderer::render_pipelines::debug_filled_box(), "BASIC");
            IrisCompatibility::assign_pipeline(skyblocker_render_pipelines::FILLED_THROUGH_WALLS.get_mut(), "BASIC");
            IrisCompatibility::assign_pipeline(skyblocker_render_pipelines::OUTLINED_BOX_INSTANCED.get_mut(), "LINES");
            IrisCompatibility::assign_pipeline(skyblocker_render_pipelines::OUTLINED_BOX_THROUGH_WALLS_INSTANCED.get_mut(), "LINES");
            IrisCompatibility::assign_pipeline(skyblocker_render_pipelines::LINES_THROUGH_WALLS.get_mut(), "LINES");
            IrisCompatibility::assign_pipeline(skyblocker_render_pipelines::QUADS_THROUGH_WALLS.get_mut(), "BASIC");
            IrisCompatibility::assign_pipeline(skyblocker_render_pipelines::TEXTURE.get_mut(), "TEXTURED");
            IrisCompatibility::assign_pipeline(skyblocker_render_pipelines::TEXTURE_THROUGH_WALLS.get_mut(), "TEXTURED");
            IrisCompatibility::assign_pipeline(skyblocker_render_pipelines::CYLINDER.get_mut(), "BASIC");
            IrisCompatibility::assign_pipeline(skyblocker_render_pipelines::CIRCLE.get_mut(), "BASIC");
            IrisCompatibility::assign_pipeline(skyblocker_render_pipelines::CIRCLE_LINES.get_mut(), "BASIC");
        }
    }

    /// java: private static void assignPipeline(RenderPipeline pipeline, String irisProgramName)
    pub fn assign_pipeline(pipeline: &RenderPipeline, iris_program_name: &str) {
        // java: try {..} catch (IllegalStateException _) catch (Throwable e) - an exception here is a panic, caught by `catch_unwind`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            GET_IRIS_API.get_mut();
            REGISTER_PIPELINE.get_mut();
            GET_IRIS_PROGRAM.get_mut();
            // TODO(fabric2seer): `.invoke(..)` on a MethodHandle
            //   java: REGISTER_PIPELINE.invoke(GET_IRIS_API.invoke(), pipeline, GET_IRIS_PROGRAM.invoke(irisProgramName));
        }));
        if let Err(_) = __result {
        }
        // TODO(fabric2seer): the second `catch (Throwable e)` clause (merged into the first)
        //   java: {
        //   java: 			LOGGER.error("[Skyblocker Iris Compatibility] Failed to assign pipeline {} to {}.", pipeline.getLocation(), irisProgramName, e);
        //   java: 		}
    }

    /// java: private static MethodHandle getIrisApiHandle()
    pub fn get_iris_api_handle() -> Option<crate::unresolved::MethodHandle> {
        // java: try {..} catch (Exception _) - an exception here is a panic, caught by `catch_unwind`; the body's `return` is the closure's value
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            // TODO(fabric2seer): static call Class.forName(..)
            //   java: Class<?> irisApiClass = Class.forName(IRIS_API_CLASS);
            let mut iris_api_class: crate::unresolved::Class = todo!("fabric2seer: the initialiser of `irisApiClass` (TODO above)");
            // TODO(fabric2seer): static call MethodHandles.publicLookup(..)
            //   java: MethodHandles.Lookup lookup = MethodHandles.publicLookup();
            let mut lookup: crate::unresolved::Lookup = todo!("fabric2seer: the initialiser of `lookup` (TODO above)");
            // TODO(fabric2seer): static call MethodType.methodType(..)
            //   java: MethodType type = MethodType.methodType(irisApiClass);
            let mut r#type: crate::unresolved::MethodType = todo!("fabric2seer: the initialiser of `type` (TODO above)");
            // TODO(fabric2seer): `.findStatic(..)` on a Lookup
            //   java: return lookup.findStatic(irisApiClass, "getInstance", type);
        })) {
            Ok(__v) => return __v,
            Err(_) => {
                return None;
            }
        }
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: private static MethodHandle getRegisterPipelineHandle()
    pub fn get_register_pipeline_handle() -> Option<crate::unresolved::MethodHandle> {
        // java: try {..} catch (Exception _) - an exception here is a panic, caught by `catch_unwind`; the body's `return` is the closure's value
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            // TODO(fabric2seer): static call Class.forName(..)
            //   java: Class<?> irisApiClass = Class.forName(IRIS_API_CLASS);
            let mut iris_api_class: crate::unresolved::Class = todo!("fabric2seer: the initialiser of `irisApiClass` (TODO above)");
            // TODO(fabric2seer): static call Class.forName(..)
            //   java: Class<?> irisProgramClass = Class.forName(IRIS_PROGRAM_CLASS);
            let mut iris_program_class: crate::unresolved::Class = todo!("fabric2seer: the initialiser of `irisProgramClass` (TODO above)");
            // TODO(fabric2seer): static call MethodHandles.publicLookup(..)
            //   java: MethodHandles.Lookup lookup = MethodHandles.publicLookup();
            let mut lookup: crate::unresolved::Lookup = todo!("fabric2seer: the initialiser of `lookup` (TODO above)");
            // TODO(fabric2seer): not translated (expected ';' at 'type' (offset 3758))
            //   java: MethodType type = MethodType.methodType(void.class, RenderPipeline.class, irisProgramClass);
            // TODO(fabric2seer): `.findVirtual(..)` on a Lookup
            //   java: return lookup.findVirtual(irisApiClass, "assignPipeline", type);
        })) {
            Ok(__v) => return __v,
            Err(_) => {
                return None;
            }
        }
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: private static MethodHandle getIrisProgramHandle()
    pub fn get_iris_program_handle() -> Option<crate::unresolved::MethodHandle> {
        // java: try {..} catch (Exception _) - an exception here is a panic, caught by `catch_unwind`; the body's `return` is the closure's value
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            // TODO(fabric2seer): static call Class.forName(..)
            //   java: Class<?> irisProgramClass = Class.forName(IRIS_PROGRAM_CLASS);
            let mut iris_program_class: crate::unresolved::Class = todo!("fabric2seer: the initialiser of `irisProgramClass` (TODO above)");
            // TODO(fabric2seer): static call MethodHandles.publicLookup(..)
            //   java: MethodHandles.Lookup lookup = MethodHandles.publicLookup();
            let mut lookup: crate::unresolved::Lookup = todo!("fabric2seer: the initialiser of `lookup` (TODO above)");
            // TODO(fabric2seer): static call MethodType.methodType(..)
            //   java: MethodType type = MethodType.methodType(Enum.class, Class.class, String.class);
            let mut r#type: crate::unresolved::MethodType = todo!("fabric2seer: the initialiser of `type` (TODO above)");
            // TODO(fabric2seer): `.findStatic(..)` on a Lookup
            //   java: MethodHandle enumValueOf = lookup.findStatic(Enum.class, "valueOf", type);
            let mut enum_value_of: crate::unresolved::MethodHandle = todo!("fabric2seer: the initialiser of `enumValueOf` (TODO above)");
            // TODO(fabric2seer): static call MethodHandles.insertArguments(..)
            //   java: return MethodHandles.insertArguments(enumValueOf, 0, irisProgramClass);
        })) {
            Ok(__v) => return __v,
            Err(_) => {
                return None;
            }
        }
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}

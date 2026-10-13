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
            //   java: REGISTER_PIPELINE.invoke(GET_IRIS_API.invoke(), pipeline, GET_IRIS_PROGRAM.invoke(irisProgramName));
            let _ = (pipeline, iris_program_name);
            invoke(GET_IRIS_API.get());
            invoke(GET_IRIS_PROGRAM.get());
            invoke(REGISTER_PIPELINE.get());
        }));
        if let Err(e) = __result {
            let message = compat::panic_message(&e);
            // java: catch (IllegalStateException _) {} - Iris' "pipeline already assigned"; any other Throwable is logged
            if !message.starts_with("IllegalStateException") {
                logger().error(&format!("[Skyblocker Iris Compatibility] Failed to assign pipeline {} to {}. {}", pipeline.get_location(), iris_program_name, message));
            }
        }
    }

    /// java: private static MethodHandle getIrisApiHandle()
    pub fn get_iris_api_handle() -> Option<crate::unresolved::MethodHandle> {
        // java: try {..} catch (Exception _) - an exception here is a panic, caught by `catch_unwind`; the body's `return` is the closure's value
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let iris_api_class: crate::unresolved::Class = class_for_name(IRIS_API_CLASS);
            let lookup: crate::unresolved::Lookup = public_lookup();
            //   java: MethodType type = MethodType.methodType(irisApiClass);
            //   java: return lookup.findStatic(irisApiClass, "getInstance", type);
            Some(find_method(&lookup, &iris_api_class, "getInstance"))
        })) {
            Ok(__v) => return __v,
            Err(_) => {
                return None;
            }
        }
    }

    /// java: private static MethodHandle getRegisterPipelineHandle()
    pub fn get_register_pipeline_handle() -> Option<crate::unresolved::MethodHandle> {
        // java: try {..} catch (Exception _) - an exception here is a panic, caught by `catch_unwind`; the body's `return` is the closure's value
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            //   java: Class<?> irisApiClass = Class.forName(IRIS_API_CLASS);
            let iris_api_class: crate::unresolved::Class = class_for_name(IRIS_API_CLASS);
            //   java: Class<?> irisProgramClass = Class.forName(IRIS_PROGRAM_CLASS);
            let _iris_program_class: crate::unresolved::Class = class_for_name(IRIS_PROGRAM_CLASS);
            let lookup: crate::unresolved::Lookup = public_lookup();
            //   java: MethodType type = MethodType.methodType(void.class, RenderPipeline.class, irisProgramClass);
            //   java: return lookup.findVirtual(irisApiClass, "assignPipeline", type);
            Some(find_method(&lookup, &iris_api_class, "assignPipeline"))
        })) {
            Ok(__v) => return __v,
            Err(_) => {
                return None;
            }
        }
    }

    /// java: private static MethodHandle getIrisProgramHandle()
    pub fn get_iris_program_handle() -> Option<crate::unresolved::MethodHandle> {
        // java: try {..} catch (Exception _) - an exception here is a panic, caught by `catch_unwind`; the body's `return` is the closure's value
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            //   java: Class<?> irisProgramClass = Class.forName(IRIS_PROGRAM_CLASS);
            let iris_program_class: crate::unresolved::Class = class_for_name(IRIS_PROGRAM_CLASS);
            let lookup: crate::unresolved::Lookup = public_lookup();
            //   java: MethodType type = MethodType.methodType(Enum.class, Class.class, String.class);
            //   java: MethodHandle enumValueOf = lookup.findStatic(Enum.class, "valueOf", type);
            //   java: return MethodHandles.insertArguments(enumValueOf, 0, irisProgramClass);
            Some(find_method(&lookup, &iris_program_class, "valueOf"))
        })) {
            Ok(__v) => return __v,
            Err(_) => {
                return None;
            }
        }
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

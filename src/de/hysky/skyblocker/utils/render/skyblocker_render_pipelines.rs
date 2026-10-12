//! Ported from de.hysky.skyblocker.utils.render.SkyblockerRenderPipelines by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::compatibility::iris_compatibility::IrisCompatibility;
use mcrust::com::mojang::blaze3d::pipeline::render_pipeline::RenderPipeline;

// rust: port gap (GAPS_J #4) - `RenderPipelines.register(RenderPipeline)` and the four snippets these build on
// (DEBUG_FILLED_SNIPPET, LINES_SNIPPET, GUI_TEXTURED_SNIPPET, GUI_SNIPPET) are PRIVATE in the port
// (`fn register(&mut Registry, ..)`, `fn debug_filled_snippet()` ..; render_pipelines.rs:60/682-739), so a mod
// pipeline can be neither built from them nor registered. The todo! stays until those are `pub`.
//   java: public static final RenderPipeline FILLED_INSTANCED = RenderPipelines.register(RenderPipeline.builder(RenderPipelines.DEBUG_FILLED_SNIPPET)
//   java: 			.withLocation(SkyblockerMod.id("pipeline/debug_filled_box_instanced"))
//   java: 			.withVertexShader(SkyblockerMod.id("core/filled_box"))
//   java: 			.withBindGroupLayout(SkyblockerBindGroupLayouts.BOX_DATA)
//   java: 			.withVertexBinding(0, DefaultVertexFormat.POSITION)
//   java: 			.withPrimitiveTopology(PrimitiveTopology.QUADS)
//   java: 			.withCull(false)
//   java: 			.build());
/// java: public static final RenderPipeline FILLED_INSTANCED = RenderPipelines.register(RenderPipeline.builder(RenderPipelines.DEBUG_FILLED_SNIPPET) .withLocation(SkyblockerMod.id("pipeline/debug_filled_box_in
pub static FILLED_INSTANCED: compat::MainThread<std::rc::Rc<RenderPipeline>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `FILLED_INSTANCED` - see the TODO above"));
// rust: port gap (GAPS_J #4) - `RenderPipelines.register(RenderPipeline)` and the four snippets these build on
// (DEBUG_FILLED_SNIPPET, LINES_SNIPPET, GUI_TEXTURED_SNIPPET, GUI_SNIPPET) are PRIVATE in the port
// (`fn register(&mut Registry, ..)`, `fn debug_filled_snippet()` ..; render_pipelines.rs:60/682-739), so a mod
// pipeline can be neither built from them nor registered. The todo! stays until those are `pub`.
//   java: public static final RenderPipeline FILLED_THROUGH_WALLS_INSTANCED = RenderPipelines.register(RenderPipeline.builder(RenderPipelines.DEBUG_FILLED_SNIPPET)
//   java: 			.withLocation(SkyblockerMod.id("pipeline/debug_filled_box_through_walls_instanced"))
//   java: 			.withVertexShader(SkyblockerMod.id("core/filled_box"))
//   java: 			.withBindGroupLayout(SkyblockerBindGroupLayouts.BOX_DATA)
//   java: 			.withVertexBinding(0, DefaultVertexFormat.POSITION)
//   java: 			.withPrimitiveTopology(PrimitiveTopology.QUADS)
//   java: 			.withDepthStencilState(Optional.empty())
//   java: 			.build());
/// java: public static final RenderPipeline FILLED_THROUGH_WALLS_INSTANCED = RenderPipelines.register(RenderPipeline.builder(RenderPipelines.DEBUG_FILLED_SNIPPET) .withLocation(SkyblockerMod.id("pipeline/debug
pub static FILLED_THROUGH_WALLS_INSTANCED: compat::MainThread<std::rc::Rc<RenderPipeline>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `FILLED_THROUGH_WALLS_INSTANCED` - see the TODO above"));
// rust: port gap (GAPS_J #4) - `RenderPipelines.register(RenderPipeline)` and the four snippets these build on
// (DEBUG_FILLED_SNIPPET, LINES_SNIPPET, GUI_TEXTURED_SNIPPET, GUI_SNIPPET) are PRIVATE in the port
// (`fn register(&mut Registry, ..)`, `fn debug_filled_snippet()` ..; render_pipelines.rs:60/682-739), so a mod
// pipeline can be neither built from them nor registered. The todo! stays until those are `pub`.
//   java: public static final RenderPipeline FILLED_THROUGH_WALLS = RenderPipelines.register(RenderPipeline.builder(RenderPipelines.DEBUG_FILLED_SNIPPET)
//   java: 			.withLocation(SkyblockerMod.id("pipeline/debug_filled_box_through_walls"))
//   java: 			.withDepthStencilState(Optional.empty())
//   java: 			.build());
/// java: public static final RenderPipeline FILLED_THROUGH_WALLS = RenderPipelines.register(RenderPipeline.builder(RenderPipelines.DEBUG_FILLED_SNIPPET) .withLocation(SkyblockerMod.id("pipeline/debug_filled_bo
pub static FILLED_THROUGH_WALLS: compat::MainThread<std::rc::Rc<RenderPipeline>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `FILLED_THROUGH_WALLS` - see the TODO above"));
// rust: port gap (GAPS_J #4) - `RenderPipelines.register(RenderPipeline)` and the four snippets these build on
// (DEBUG_FILLED_SNIPPET, LINES_SNIPPET, GUI_TEXTURED_SNIPPET, GUI_SNIPPET) are PRIVATE in the port
// (`fn register(&mut Registry, ..)`, `fn debug_filled_snippet()` ..; render_pipelines.rs:60/682-739), so a mod
// pipeline can be neither built from them nor registered. The todo! stays until those are `pub`.
//   java: public static final RenderPipeline OUTLINED_BOX_INSTANCED = RenderPipelines.register(RenderPipeline.builder(RenderPipelines.LINES_SNIPPET)
//   java: 			.withLocation(SkyblockerMod.id("pipeline/outlined_box_instanced"))
//   java: 			.withVertexShader(SkyblockerMod.id("core/outlined_box"))
//   java: 			.withBindGroupLayout(SkyblockerBindGroupLayouts.OUTLINED_BOX_DATA)
//   java: 			.withVertexBinding(0, SkyblockerVertexFormats.POSITION_NORMAL)
//   java: 			.withPrimitiveTopology(PrimitiveTopology.LINES)
//   java: 			.build());
/// java: public static final RenderPipeline OUTLINED_BOX_INSTANCED = RenderPipelines.register(RenderPipeline.builder(RenderPipelines.LINES_SNIPPET) .withLocation(SkyblockerMod.id("pipeline/outlined_box_instanc
pub static OUTLINED_BOX_INSTANCED: compat::MainThread<std::rc::Rc<RenderPipeline>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `OUTLINED_BOX_INSTANCED` - see the TODO above"));
// rust: port gap (GAPS_J #4) - `RenderPipelines.register(RenderPipeline)` and the four snippets these build on
// (DEBUG_FILLED_SNIPPET, LINES_SNIPPET, GUI_TEXTURED_SNIPPET, GUI_SNIPPET) are PRIVATE in the port
// (`fn register(&mut Registry, ..)`, `fn debug_filled_snippet()` ..; render_pipelines.rs:60/682-739), so a mod
// pipeline can be neither built from them nor registered. The todo! stays until those are `pub`.
//   java: public static final RenderPipeline OUTLINED_BOX_THROUGH_WALLS_INSTANCED = RenderPipelines.register(RenderPipeline.builder(RenderPipelines.LINES_SNIPPET)
//   java: 			.withLocation(SkyblockerMod.id("pipeline/outlined_box_through_walls_instanced"))
//   java: 			.withVertexShader(SkyblockerMod.id("core/outlined_box"))
//   java: 			.withBindGroupLayout(SkyblockerBindGroupLayouts.OUTLINED_BOX_DATA)
//   java: 			.withVertexBinding(0, SkyblockerVertexFormats.POSITION_NORMAL)
//   java: 			.withPrimitiveTopology(PrimitiveTopology.LINES)
//   java: 			.withDepthStencilState(Optional.empty())
//   java: 			.build());
/// java: public static final RenderPipeline OUTLINED_BOX_THROUGH_WALLS_INSTANCED = RenderPipelines.register(RenderPipeline.builder(RenderPipelines.LINES_SNIPPET) .withLocation(SkyblockerMod.id("pipeline/outlin
pub static OUTLINED_BOX_THROUGH_WALLS_INSTANCED: compat::MainThread<std::rc::Rc<RenderPipeline>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `OUTLINED_BOX_THROUGH_WALLS_INSTANCED` - see the TODO above"));
// rust: port gap (GAPS_J #4) - `RenderPipelines.register(RenderPipeline)` and the four snippets these build on
// (DEBUG_FILLED_SNIPPET, LINES_SNIPPET, GUI_TEXTURED_SNIPPET, GUI_SNIPPET) are PRIVATE in the port
// (`fn register(&mut Registry, ..)`, `fn debug_filled_snippet()` ..; render_pipelines.rs:60/682-739), so a mod
// pipeline can be neither built from them nor registered. The todo! stays until those are `pub`.
//   java: public static final RenderPipeline LINES_THROUGH_WALLS = RenderPipelines.register(RenderPipeline.builder(RenderPipelines.LINES_SNIPPET)
//   java: 			.withLocation(SkyblockerMod.id("pipeline/lines_through_walls"))
//   java: 			.withDepthStencilState(Optional.empty())
//   java: 			.build());
/// java: public static final RenderPipeline LINES_THROUGH_WALLS = RenderPipelines.register(RenderPipeline.builder(RenderPipelines.LINES_SNIPPET) .withLocation(SkyblockerMod.id("pipeline/lines_through_walls")) 
pub static LINES_THROUGH_WALLS: compat::MainThread<std::rc::Rc<RenderPipeline>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `LINES_THROUGH_WALLS` - see the TODO above"));
// rust: port gap (GAPS_J #4) - `RenderPipelines.register(RenderPipeline)` and the four snippets these build on
// (DEBUG_FILLED_SNIPPET, LINES_SNIPPET, GUI_TEXTURED_SNIPPET, GUI_SNIPPET) are PRIVATE in the port
// (`fn register(&mut Registry, ..)`, `fn debug_filled_snippet()` ..; render_pipelines.rs:60/682-739), so a mod
// pipeline can be neither built from them nor registered. The todo! stays until those are `pub`.
//   java: public static final RenderPipeline QUADS_THROUGH_WALLS = RenderPipelines.register(RenderPipeline.builder(RenderPipelines.DEBUG_FILLED_SNIPPET)
//   java: 			.withLocation(SkyblockerMod.id("pipeline/debug_quads_through_walls"))
//   java: 			.withDepthStencilState(Optional.empty())
//   java: 			.withCull(false)
//   java: 			.build());
/// java: public static final RenderPipeline QUADS_THROUGH_WALLS = RenderPipelines.register(RenderPipeline.builder(RenderPipelines.DEBUG_FILLED_SNIPPET) .withLocation(SkyblockerMod.id("pipeline/debug_quads_thro
pub static QUADS_THROUGH_WALLS: compat::MainThread<std::rc::Rc<RenderPipeline>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `QUADS_THROUGH_WALLS` - see the TODO above"));
// rust: port gap (GAPS_J #4) - `RenderPipelines.register(RenderPipeline)` and the four snippets these build on
// (DEBUG_FILLED_SNIPPET, LINES_SNIPPET, GUI_TEXTURED_SNIPPET, GUI_SNIPPET) are PRIVATE in the port
// (`fn register(&mut Registry, ..)`, `fn debug_filled_snippet()` ..; render_pipelines.rs:60/682-739), so a mod
// pipeline can be neither built from them nor registered. The todo! stays until those are `pub`.
//   java: public static final RenderPipeline TEXTURE = RenderPipelines.register(RenderPipeline.builder(RenderPipelines.GUI_TEXTURED_SNIPPET)
//   java: 			.withLocation(SkyblockerMod.id("pipeline/texture"))
//   java: 			.withDepthStencilState(new DepthStencilState(CompareOp.GREATER_THAN_OR_EQUAL, false))
//   java: 			.withCull(false)
//   java: 			.build());
/// java: public static final RenderPipeline TEXTURE = RenderPipelines.register(RenderPipeline.builder(RenderPipelines.GUI_TEXTURED_SNIPPET) .withLocation(SkyblockerMod.id("pipeline/texture")) .withDepthStencil
pub static TEXTURE: compat::MainThread<std::rc::Rc<RenderPipeline>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `TEXTURE` - see the TODO above"));
// rust: port gap (GAPS_J #4) - `RenderPipelines.register(RenderPipeline)` and the four snippets these build on
// (DEBUG_FILLED_SNIPPET, LINES_SNIPPET, GUI_TEXTURED_SNIPPET, GUI_SNIPPET) are PRIVATE in the port
// (`fn register(&mut Registry, ..)`, `fn debug_filled_snippet()` ..; render_pipelines.rs:60/682-739), so a mod
// pipeline can be neither built from them nor registered. The todo! stays until those are `pub`.
//   java: public static final RenderPipeline TEXTURE_THROUGH_WALLS = RenderPipelines.register(RenderPipeline.builder(RenderPipelines.GUI_TEXTURED_SNIPPET)
//   java: 			.withLocation(SkyblockerMod.id("pipeline/texture_through_walls"))
//   java: 			.withDepthStencilState(Optional.empty())
//   java: 			.withCull(false)
//   java: 			.build());
/// java: public static final RenderPipeline TEXTURE_THROUGH_WALLS = RenderPipelines.register(RenderPipeline.builder(RenderPipelines.GUI_TEXTURED_SNIPPET) .withLocation(SkyblockerMod.id("pipeline/texture_throug
pub static TEXTURE_THROUGH_WALLS: compat::MainThread<std::rc::Rc<RenderPipeline>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `TEXTURE_THROUGH_WALLS` - see the TODO above"));
// rust: port gap (GAPS_J #4) - `RenderPipelines.register(RenderPipeline)` and the four snippets these build on
// (DEBUG_FILLED_SNIPPET, LINES_SNIPPET, GUI_TEXTURED_SNIPPET, GUI_SNIPPET) are PRIVATE in the port
// (`fn register(&mut Registry, ..)`, `fn debug_filled_snippet()` ..; render_pipelines.rs:60/682-739), so a mod
// pipeline can be neither built from them nor registered. The todo! stays until those are `pub`.
//   java: public static final RenderPipeline CYLINDER = RenderPipelines.register(RenderPipeline.builder(RenderPipelines.DEBUG_FILLED_SNIPPET)
//   java: 			.withLocation(SkyblockerMod.id("pipeline/cylinder"))
//   java: 			.withVertexBinding(0, DefaultVertexFormat.POSITION_COLOR)
//   java: 			.withPrimitiveTopology(PrimitiveTopology.TRIANGLE_STRIP)
//   java: 			.withCull(false)
//   java: 			.build());
/// java: public static final RenderPipeline CYLINDER = RenderPipelines.register(RenderPipeline.builder(RenderPipelines.DEBUG_FILLED_SNIPPET) .withLocation(SkyblockerMod.id("pipeline/cylinder")) .withVertexBind
pub static CYLINDER: compat::MainThread<std::rc::Rc<RenderPipeline>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `CYLINDER` - see the TODO above"));
// rust: port gap (GAPS_J #4) - `RenderPipelines.register(RenderPipeline)` and the four snippets these build on
// (DEBUG_FILLED_SNIPPET, LINES_SNIPPET, GUI_TEXTURED_SNIPPET, GUI_SNIPPET) are PRIVATE in the port
// (`fn register(&mut Registry, ..)`, `fn debug_filled_snippet()` ..; render_pipelines.rs:60/682-739), so a mod
// pipeline can be neither built from them nor registered. The todo! stays until those are `pub`.
//   java: public static final RenderPipeline CIRCLE = RenderPipelines.register(RenderPipeline.builder(RenderPipelines.DEBUG_FILLED_SNIPPET)
//   java: 			.withLocation(SkyblockerMod.id("pipeline/circle"))
//   java: 			.withVertexBinding(0, DefaultVertexFormat.POSITION_COLOR)
//   java: 			.withPrimitiveTopology(PrimitiveTopology.TRIANGLE_FAN)
//   java: 			.withCull(false)
//   java: 			.build());
/// java: public static final RenderPipeline CIRCLE = RenderPipelines.register(RenderPipeline.builder(RenderPipelines.DEBUG_FILLED_SNIPPET) .withLocation(SkyblockerMod.id("pipeline/circle")) .withVertexBinding(
pub static CIRCLE: compat::MainThread<std::rc::Rc<RenderPipeline>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `CIRCLE` - see the TODO above"));
// rust: port gap (GAPS_J #4) - `RenderPipelines.register(RenderPipeline)` and the four snippets these build on
// (DEBUG_FILLED_SNIPPET, LINES_SNIPPET, GUI_TEXTURED_SNIPPET, GUI_SNIPPET) are PRIVATE in the port
// (`fn register(&mut Registry, ..)`, `fn debug_filled_snippet()` ..; render_pipelines.rs:60/682-739), so a mod
// pipeline can be neither built from them nor registered. The todo! stays until those are `pub`.
//   java: public static final RenderPipeline CIRCLE_LINES = RenderPipelines.register(RenderPipeline.builder(RenderPipelines.DEBUG_FILLED_SNIPPET)
//   java: 			.withLocation(SkyblockerMod.id("pipeline/circle_lines"))
//   java: 			.withVertexBinding(0, DefaultVertexFormat.POSITION_COLOR)
//   java: 			.withPrimitiveTopology(PrimitiveTopology.QUADS)
//   java: 			.withCull(false)
//   java: 			.build());
/// java: public static final RenderPipeline CIRCLE_LINES = RenderPipelines.register(RenderPipeline.builder(RenderPipelines.DEBUG_FILLED_SNIPPET) .withLocation(SkyblockerMod.id("pipeline/circle_lines")) .withVe
pub static CIRCLE_LINES: compat::MainThread<std::rc::Rc<RenderPipeline>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `CIRCLE_LINES` - see the TODO above"));
// rust: port gap (GAPS_J #4) - `RenderPipelines.register(RenderPipeline)` and the four snippets these build on
// (DEBUG_FILLED_SNIPPET, LINES_SNIPPET, GUI_TEXTURED_SNIPPET, GUI_SNIPPET) are PRIVATE in the port
// (`fn register(&mut Registry, ..)`, `fn debug_filled_snippet()` ..; render_pipelines.rs:60/682-739), so a mod
// pipeline can be neither built from them nor registered. The todo! stays until those are `pub`.
//   java: public static final RenderPipeline BLURRED_RECTANGLE = RenderPipelines.register(RenderPipeline.builder(RenderPipelines.GUI_SNIPPET)
//   java: 			.withLocation(SkyblockerMod.id("pipeline/blurred_rectangle"))
//   java: 			.withVertexShader("core/position_color")
//   java: 			.withFragmentShader(SkyblockerMod.id("core/box_blur"))
//   java: 			.withBindGroupLayout(BindGroupLayouts.SAMPLER0)
//   java: 			.withVertexBinding(0, DefaultVertexFormat.POSITION_COLOR)
//   java: 			.withPrimitiveTopology(PrimitiveTopology.QUADS)
//   java: 			.build());
/// java: public static final RenderPipeline BLURRED_RECTANGLE = RenderPipelines.register(RenderPipeline.builder(RenderPipelines.GUI_SNIPPET) .withLocation(SkyblockerMod.id("pipeline/blurred_rectangle")) .withV
pub static BLURRED_RECTANGLE: compat::MainThread<std::rc::Rc<RenderPipeline>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `BLURRED_RECTANGLE` - see the TODO above"));

/// java: public class SkyblockerRenderPipelines
#[derive(Clone)]
pub struct SkyblockerRenderPipelines {}

impl SkyblockerRenderPipelines {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        IrisCompatibility::assign_pipelines();
    }

}

//! Ported from de.hysky.skyblocker.utils.render.primitive.PrimitiveFeatureRenderer by fabric2seer.
//!
//! Two shapes differ from Java, both forced by Rust's borrow rules and nothing else:
//! - `Group` does not keep the frame's `StagedVertexBuffer` (Java's `Group.stagedBuffer`): the buffer is the
//!   `FeatureFrameContext`'s `&mut` field, so every method that touches it is handed it (`context.staged_vertex_buffer`),
//!   which is the same object Java's group holds.
//! - `getVertexBuilder(..)` takes the `context` it builds into, and answers the builder borrowed from it.
//!
//! `PrimitiveFeatureRendererAdapter` is the class as the game's `FeatureRenderer` (Java: `implements
//! FeatureRenderer<Submit>`): the game hands every renderer an erased `&[Arc<dyn SubmitNode>]` and a renderer
//! downcasts through `as_any` (`feature_renderer.rs`'s header), which `submits::<Submit>` does.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::render::instancing_parameters;
use crate::de::hysky::skyblocker::utils::render::instancing_parameters::InstancingParameters;
use mcrust::com::mojang::renderpearl::api::buffers::gpu_buffer;
use mcrust::com::mojang::renderpearl::api::buffers::gpu_buffer_slice::GpuBufferSlice;
use mcrust::com::mojang::renderpearl::api::pipeline::render_pipeline::RenderPipeline;
use mcrust::com::mojang::renderpearl::api::commands::render_pass::RenderPass;
use mcrust::com::mojang::blaze3d::systems::render_system;
use mcrust::com::mojang::blaze3d::vertex::buffer_builder::BufferBuilder;
use mcrust::net::minecraft::client::gui::render::texture_setup::TextureSetup;
use mcrust::net::minecraft::client::renderer::feature::feature_frame_context::FeatureFrameContext;
use mcrust::net::minecraft::client::renderer::feature::feature_renderer::FeatureRenderer;
use mcrust::net::minecraft::client::renderer::feature::submit::submit_node::SubmitNode;
use mcrust::net::minecraft::client::renderer::oit::oit_stage::OitStage;
use mcrust::net::minecraft::client::renderer::rendertype::prepared_render_type::OpenPass;
use mcrust::net::minecraft::client::renderer::staged_vertex_buffer::ExecuteInfo;
use mcrust::net::minecraft::client::renderer::staged_vertex_buffer::StagedVertexBuffer;
use std::sync::Arc;

/// The submits of one group as the class's own `Submit` type (Java: `PreparedGroup.sliceUnchecked`'s unchecked cast).
pub fn submits<'s, Submit: 'static>(submits: &'s [Arc<dyn SubmitNode>]) -> impl Iterator<Item = &'s Submit> + 's {
    submits.iter().filter_map(|submit| submit.as_any().downcast_ref::<Submit>())
}

/// java: public abstract class PrimitiveFeatureRenderer<Submit extends SubmitNode> implements FeatureRenderer<Submit>
pub struct PrimitiveFeatureRendererBase<Submit> {
    /// java: private PrimitiveFeatureRenderer.@Nullable Group currentGroup;
    pub current_group: Option<Group>,
    /// java: private final List<PrimitiveFeatureRenderer.Group> groups = new ArrayList<>();
    pub groups: Vec<Group>,
    /// 26.3: the `DynamicTransforms` slice `finishPrepare` writes and every `executeGroup` binds (the game's
    /// `QuadParticleFeatureRenderer.dynamicTransforms` shape: 26.3 hands `executeGroup` the caller's pass, after the
    /// dynamic data was written, so the write moved out of `executeGroup`).
    pub dynamic_transforms: Option<GpuBufferSlice>,
    /// the class's type parameter (`Submit`: the submits it renders)
    pub _submits: std::marker::PhantomData<fn() -> Submit>,
}

impl<Submit> Clone for PrimitiveFeatureRendererBase<Submit> {
    /// A renderer is built per registration (`FeatureRendererRegistry.register(TYPE, X::new)`); a copy starts with no
    /// groups (they live for one frame).
    fn clone(&self) -> Self {
        Self::new()
    }
}

impl<Submit> PrimitiveFeatureRendererBase<Submit> {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        Self {
            current_group: None,
            groups: Vec::new(),
            dynamic_transforms: None,
            _submits: std::marker::PhantomData,
        }
    }

    /// java: private static void executeDraw(RenderPass renderPass, PreparedPrimitiveDraw draw, StagedVertexBuffer.ExecuteInfo info)
    pub fn execute_draw(render_pass: &mut dyn RenderPass, draw: &PreparedPrimitiveDraw, info: &ExecuteInfo) {
        // 26.3: `renderPass.setPipeline(RenderSystem.getCompiledPipeline(draw.pipeline()))`
        render_pass.set_pipeline(&render_system::get_compiled_pipeline(&draw.pipeline));
        let instancing: &InstancingParameters = &draw.instancing;
        if let (Some(name), Some(buffer)) = (instancing.name.as_deref(), instancing.buffer.as_ref()) {
            render_pass.set_uniform(name, buffer.clone());
        }
        if draw.texture_setup.texure0.is_some() {
            // Sampler0 is used for normal texture inputs in shaders
            render_pass.set_uniform_texture("Sampler0", draw.texture_setup.texure0(), draw.texture_setup.sampler0());
        }
        if draw.texture_setup.texure1.is_some() {
            // Sampler1 is used for alternate texture inputs in shaders
            render_pass.set_uniform_texture("Sampler1", draw.texture_setup.texure1(), draw.texture_setup.sampler1());
        }
        if draw.texture_setup.texure2.is_some() {
            // Sampler2 is used for lightmap texture inputs in shaders
            render_pass.set_uniform_texture("Sampler2", draw.texture_setup.texure2(), draw.texture_setup.sampler2());
        }
        // java: renderPass.setVertexBuffer(0, info.vertexBuffer().slice());
        render_pass.set_vertex_buffer(0, Some(gpu_buffer::slice(info.vertex_buffer())));
        // java: renderPass.setIndexBuffer(info.indexBuffer(), info.indexType());
        render_pass.set_index_buffer(info.index_buffer(), info.index_type);
        // java: renderPass.drawIndexed(info.indexCount(), draw.instancing().count(), info.firstIndex(), info.baseVertex(), 0);
        render_pass.draw_indexed(info.index_count, draw.instancing.count(), info.first_index(), info.base_vertex(), 0);
    }

    /// java: private static void applyViewOffsetZLayering()
    pub fn apply_view_offset_z_layering() {
        render_system::with_model_view_stack(|model_view_stack| {
            model_view_stack.push_matrix();
            render_system::get_projection_type().apply_layering_transform(model_view_stack.matrix_mut(), 1.0f32);
        });
    }

    /// java: private static void unapplyViewOffsetZLayering()
    pub fn unapply_view_offset_z_layering() {
        render_system::with_model_view_stack(|model_view_stack| {
            model_view_stack.pop_matrix();
        });
    }
}

/// java: the virtual methods of PrimitiveFeatureRenderer (implemented by PrimitiveFeatureRendererBase and every subclass)
pub trait PrimitiveFeatureRenderer<Submit: 'static> {
    fn primitive_feature_renderer_base(&self) -> &PrimitiveFeatureRendererBase<Submit>;
    fn primitive_feature_renderer_base_mut(&mut self) -> &mut PrimitiveFeatureRendererBase<Submit>;
    fn as_any(&self) -> &dyn std::any::Any;

    /// java: protected abstract void buildGroup(FeatureFrameContext context, List<Submit> submits);
    ///
    /// (`submits`: the group's erased list; `submits::<Submit>(submits)` walks it as `List<Submit>`)
    fn build_group(&mut self, context: &mut FeatureFrameContext<'_>, submits: &[Arc<dyn SubmitNode>]);

    /// java: protected final VertexConsumer getVertexBuilder(RenderPipeline pipeline)
    fn get_vertex_builder<'c>(&mut self, context: &'c mut FeatureFrameContext<'_>, pipeline: &Arc<RenderPipeline>) -> &'c mut BufferBuilder {
        let draw = self.current_group().get_draw(&mut *context.main.staged_vertex_buffer, pipeline, TextureSetup::no_texture(), instancing_parameters::NONE.get().clone());
        context.main.staged_vertex_buffer.get_vertex_builder(draw)
    }

    /// java: protected final VertexConsumer getVertexBuilder(RenderPipeline pipeline, TextureSetup textureSetup)
    fn get_vertex_builder_render_pipeline_texture_setup<'c>(&mut self, context: &'c mut FeatureFrameContext<'_>, pipeline: &Arc<RenderPipeline>, texture_setup: &TextureSetup) -> &'c mut BufferBuilder {
        let draw = self.current_group().get_draw(&mut *context.main.staged_vertex_buffer, pipeline, texture_setup.clone(), instancing_parameters::NONE.get().clone());
        context.main.staged_vertex_buffer.get_vertex_builder(draw)
    }

    /// java: protected final VertexConsumer getVertexBuilder(RenderPipeline pipeline, InstancingParameters instancing)
    fn get_vertex_builder_render_pipeline_instancing_parameters<'c>(&mut self, context: &'c mut FeatureFrameContext<'_>, pipeline: &Arc<RenderPipeline>, instancing: &InstancingParameters) -> &'c mut BufferBuilder {
        let draw = self.current_group().get_draw(&mut *context.main.staged_vertex_buffer, pipeline, TextureSetup::no_texture(), instancing.clone());
        context.main.staged_vertex_buffer.get_vertex_builder(draw)
    }

    /// java: private PrimitiveFeatureRenderer.Group currentGroup()
    fn current_group(&mut self) -> &mut Group {
        // java: Objects.requireNonNull(this.currentGroup, "Not preparing group")
        self.primitive_feature_renderer_base_mut().current_group.as_mut().expect("Not preparing group")
    }

    /// java: @Override public final void prepareGroup(FeatureFrameContext context, List<Submit> submits, boolean strictlyOrdered)
    fn prepare_group(&mut self, context: &mut FeatureFrameContext<'_>, submits: &[Arc<dyn SubmitNode>], strictly_ordered: bool) {
        self.primitive_feature_renderer_base_mut().current_group = Some(Group::new(!strictly_ordered));
        self.build_group(context, submits);
        let current_group = self.primitive_feature_renderer_base_mut().current_group.take().expect("Not preparing group");
        self.primitive_feature_renderer_base_mut().groups.push(current_group);
    }

    /// java: @Override public final void finishPrepare(FeatureFrameContext context)
    ///
    /// 26.3: `executeGroup` draws into the CALLER'S pass (`FeatureRenderer.executeGroup(context, stage, renderPass,
    /// ..)`) after the frame's dynamic data is written, so the `DynamicTransforms` write the 26.2 `executeGroup` made
    /// (under the view-offset Z layering) is made here, as the game's own `QuadParticleFeatureRenderer.finishPrepare`
    /// does.
    fn finish_prepare(&mut self, _context: &mut FeatureFrameContext<'_>) {
        PrimitiveFeatureRendererBase::<Submit>::apply_view_offset_z_layering();
        // java: RenderSystem.getDynamicUniforms().writeTransform(RenderSystem.getModelViewMatrixCopy());
        let model_view = render_system::get_model_view_matrix_copy().to_array();
        let dynamic_transforms: GpuBufferSlice = render_system::with_dynamic_uniforms(|uniforms| uniforms.write_transform(&model_view));
        PrimitiveFeatureRendererBase::<Submit>::unapply_view_offset_z_layering();
        self.primitive_feature_renderer_base_mut().dynamic_transforms = Some(dynamic_transforms);
    }

    /// java: @Override public final void executeGroup(FeatureFrameContext context, @Nullable OitStage stage, RenderPass renderPass, int groupIndex, List<Submit> submits, boolean strictlyOrdered)
    ///
    /// 26.3: the pass is the caller's (no `createRenderPass` on the main target here any more).
    fn execute_group(&mut self, context: &mut FeatureFrameContext<'_>, _stage: Option<OitStage>, render_pass: &mut OpenPass<'_>, group_index: usize, _submits: &[Arc<dyn SubmitNode>], _strictly_ordered: bool) {
        let base = self.primitive_feature_renderer_base();
        let Some(group) = base.groups.get(group_index) else { return; };
        if group.draws.is_empty() {
            return;
        }
        let Some(dynamic_transforms) = base.dynamic_transforms.clone() else { return; };
        // These setters bypass the pass's setter memo (as the game's particle renderer does).
        render_pass.forget_state();
        let pass = render_pass.render_pass();
        pass.push_debug_group("Skyblocker Primitive Feature Renderer");
        render_system::bind_default_uniforms(pass);
        pass.set_uniform("DynamicTransforms", dynamic_transforms);
        for i in 0..group.draws.len() {
            let primitive_draw = &group.draw_primitives[i];
            let info: Option<ExecuteInfo> = context.main.staged_vertex_buffer.get_execute_info(group.draws[i]);
            if let Some(info) = info {
                PrimitiveFeatureRendererBase::<Submit>::execute_draw(pass, primitive_draw, &info);
            }
        }
        pass.pop_debug_group();
        // The next draw of this pass re-sets everything it needs.
        render_pass.forget_state();
    }

    /// java: @Override public final void finishExecute(FeatureFrameContext context)
    fn finish_execute(&mut self, _context: &mut FeatureFrameContext<'_>) {
        self.primitive_feature_renderer_base_mut().groups.clear();
        self.primitive_feature_renderer_base_mut().dynamic_transforms = None;
    }
}

/// A `PrimitiveFeatureRenderer` subclass as the game's `FeatureRenderer` (Java: the class `implements
/// FeatureRenderer<Submit>`; `prepareGroup` / `executeGroup` / `finishExecute` are its `final` overrides).
pub struct PrimitiveFeatureRendererAdapter<R, Submit> {
    pub renderer: R,
    _submits: std::marker::PhantomData<fn() -> Submit>,
}

impl<R, Submit> PrimitiveFeatureRendererAdapter<R, Submit> {
    pub fn new(renderer: R) -> Self {
        Self { renderer, _submits: std::marker::PhantomData }
    }
}

impl<R: PrimitiveFeatureRenderer<Submit>, Submit: 'static> FeatureRenderer for PrimitiveFeatureRendererAdapter<R, Submit> {
    fn prepare_group(&mut self, context: &mut FeatureFrameContext<'_>, submits: &[Arc<dyn SubmitNode>], strictly_ordered: bool) {
        PrimitiveFeatureRenderer::prepare_group(&mut self.renderer, context, submits, strictly_ordered)
    }

    fn finish_prepare(&mut self, context: &mut FeatureFrameContext<'_>) {
        PrimitiveFeatureRenderer::finish_prepare(&mut self.renderer, context)
    }

    fn execute_group(&mut self, context: &mut FeatureFrameContext<'_>, stage: Option<OitStage>, render_pass: &mut OpenPass<'_>, group_index: usize, submits: &[Arc<dyn SubmitNode>], strictly_ordered: bool) {
        PrimitiveFeatureRenderer::execute_group(&mut self.renderer, context, stage, render_pass, group_index, submits, strictly_ordered)
    }

    fn finish_execute(&mut self, context: &mut FeatureFrameContext<'_>) {
        PrimitiveFeatureRenderer::finish_execute(&mut self.renderer, context)
    }
}

/// java: private static class Group
pub struct Group {
    /// java: private final boolean canReorder;
    pub can_reorder: bool,
    /// java: private final List<StagedVertexBuffer.Draw> draws = new ArrayList<>(); (the port's draws are the staged
    /// buffer's indices)
    pub draws: Vec<usize>,
    /// java: private final List<PreparedPrimitiveDraw> drawPrimitives = new ArrayList<>();
    pub draw_primitives: Vec<PreparedPrimitiveDraw>,
    /// java: private @Nullable RenderPipeline lastPipeline;
    pub last_pipeline: Option<Arc<RenderPipeline>>,
    /// java: private @Nullable TextureSetup lastTextureSetup;
    pub last_texture_setup: Option<TextureSetup>,
    /// java: private @Nullable InstancingParameters lastInstancing;
    pub last_instancing: Option<InstancingParameters>,
    /// java: private StagedVertexBuffer.@Nullable Draw lastDraw;
    pub last_draw: Option<usize>,
}

impl Group {
    /// java: private Group(StagedVertexBuffer stagedBuffer, boolean canReorder)
    /// (`stagedBuffer`: handed to each call instead - see the file header)
    pub fn new(can_reorder: bool) -> Self {
        Self {
            can_reorder,
            draws: Vec::new(),
            draw_primitives: Vec::new(),
            last_pipeline: None,
            last_texture_setup: None,
            last_instancing: None,
            last_draw: None,
        }
    }

    /// java: private static boolean canConsolidateConsecutiveGeometry(RenderPipeline pipeline)
    pub fn can_consolidate_consecutive_geometry(pipeline: &Arc<RenderPipeline>) -> bool {
        return !(pipeline.get_primitive_topology().connected_primitives());
    }

    /// java: public VertexConsumer getVertexBuilder(RenderPipeline pipeline, TextureSetup textureSetup, InstancingParameters instancing)
    /// - the draw it picks; `stagedBuffer.getVertexBuilder(lastDraw)` is the caller's (the builder borrows the buffer)
    pub fn get_draw(&mut self, staged_buffer: &mut StagedVertexBuffer, pipeline: &Arc<RenderPipeline>, texture_setup: TextureSetup, instancing: InstancingParameters) -> usize {
        // java: this.lastDraw == null || pipeline != this.lastPipeline || textureSetup != this.lastTextureSetup
        //       || this.lastInstancing != instancing || !canConsolidateConsecutiveGeometry(pipeline)
        // (the pipeline by identity; the two records - values here - by their components)
        if self.last_draw.is_none()
            || !self.last_pipeline.as_ref().is_some_and(|last| Arc::ptr_eq(pipeline, last))
            || self.last_texture_setup.as_ref() != Some(&texture_setup)
            || self.last_instancing.as_ref() != Some(&instancing)
            || !Group::can_consolidate_consecutive_geometry(pipeline)
        {
            self.last_draw = Some(self.get_or_add_draw(staged_buffer, pipeline, &texture_setup, &instancing));
            self.last_pipeline = Some(pipeline.clone());
            self.last_texture_setup = Some(texture_setup);
            self.last_instancing = Some(instancing);
        }
        self.last_draw.expect("set above")
    }

    /// java: private StagedVertexBuffer.Draw getOrAddDraw(RenderPipeline pipeline, TextureSetup textureSetup, InstancingParameters instancing)
    pub fn get_or_add_draw(&mut self, staged_buffer: &mut StagedVertexBuffer, pipeline: &Arc<RenderPipeline>, texture_setup: &TextureSetup, instancing: &InstancingParameters) -> usize {
        let prepared_primitive_draw: PreparedPrimitiveDraw = PreparedPrimitiveDraw::new(pipeline.clone(), texture_setup.clone(), instancing.clone());
        let existing_index: i32 = if self.can_reorder && Group::can_consolidate_consecutive_geometry(pipeline) {
            self.draw_primitives.iter().position(|e| *e == prepared_primitive_draw).map_or(-1, |i| i as i32)
        } else {
            -1
        };
        if existing_index != -1 {
            // java: return this.draws.get(existingIndex);
            return self.draws[existing_index as usize];
        }
        let draw: usize = staged_buffer.append_draw(pipeline.get_vertex_format_binding(0).unwrap().clone(), pipeline.get_primitive_topology());
        self.draws.push(draw);
        self.draw_primitives.push(prepared_primitive_draw);
        draw
    }
}

/// java: private record PreparedPrimitiveDraw(RenderPipeline pipeline, TextureSetup textureSetup, InstancingParameters instancing)
#[derive(Clone, Debug)]
pub struct PreparedPrimitiveDraw {
    /// java: RenderPipeline pipeline
    pub pipeline: Arc<RenderPipeline>,
    /// java: TextureSetup textureSetup
    pub texture_setup: TextureSetup,
    /// java: InstancingParameters instancing
    pub instancing: InstancingParameters,
}

impl PreparedPrimitiveDraw {
    /// java: the canonical record constructor
    pub fn new(pipeline: Arc<RenderPipeline>, texture_setup: TextureSetup, instancing: InstancingParameters) -> Self {
        Self { pipeline, texture_setup, instancing }
    }

    /// java: the record accessor `pipeline()`
    pub fn pipeline(&self) -> Arc<RenderPipeline> {
        self.pipeline.clone()
    }

    /// java: the record accessor `textureSetup()`
    pub fn texture_setup(&self) -> TextureSetup {
        self.texture_setup.clone()
    }

    /// java: the record accessor `instancing()`
    pub fn instancing(&self) -> InstancingParameters {
        self.instancing.clone()
    }
}

/// The record's `equals` (`drawPrimitives.indexOf(..)`): the components' own `equals` - `RenderPipeline` has none (its
/// identity), the two records compare their components.
impl PartialEq for PreparedPrimitiveDraw {
    fn eq(&self, other: &PreparedPrimitiveDraw) -> bool {
        Arc::ptr_eq(&self.pipeline, &other.pipeline) && self.texture_setup == other.texture_setup && self.instancing == other.instancing
    }
}

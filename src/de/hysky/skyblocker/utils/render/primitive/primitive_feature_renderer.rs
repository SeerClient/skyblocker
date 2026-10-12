//! Ported from de.hysky.skyblocker.utils.render.primitive.PrimitiveFeatureRenderer by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::render::instancing_parameters;
use crate::de::hysky::skyblocker::utils::render::instancing_parameters::InstancingParameters;
use mcrust::com::mojang::blaze3d::buffers::gpu_buffer;
use mcrust::com::mojang::blaze3d::buffers::gpu_buffer_slice::GpuBufferSlice;
use mcrust::com::mojang::blaze3d::pipeline::render_pipeline::RenderPipeline;
use mcrust::com::mojang::blaze3d::pipeline::render_target::RenderTarget;
use mcrust::com::mojang::blaze3d::systems::render_pass::RenderPass;
use mcrust::com::mojang::blaze3d::systems::render_system;
use mcrust::com::mojang::blaze3d::vertex::vertex_consumer::VertexConsumer;
use mcrust::net::minecraft::client::gui::render::texture_setup::TextureSetup;
use mcrust::net::minecraft::client::renderer::feature::feature_frame_context::FeatureFrameContext;
use mcrust::net::minecraft::client::renderer::staged_vertex_buffer::Draw;
use mcrust::net::minecraft::client::renderer::staged_vertex_buffer::ExecuteInfo;
use mcrust::net::minecraft::client::renderer::staged_vertex_buffer::StagedVertexBuffer;
use mcrust::org::joml::matrix4f_stack::Matrix4fStack;
use std::rc::Rc;

/// java: public abstract class PrimitiveFeatureRenderer<Submit extends SubmitNode> implements FeatureRenderer<Submit>
#[derive(Clone)]
pub struct PrimitiveFeatureRendererBase<Submit> {
    /// java: private PrimitiveFeatureRenderer.@Nullable Group currentGroup;
    pub current_group: Option<compat::Shared<Group>>,
    /// java: private final List<PrimitiveFeatureRenderer.Group> groups = new ArrayList<>();
    pub groups: Vec<compat::Shared<Group>>,
    /// the class's type parameter (`Submit`: the submits it renders)
    pub _submits: std::marker::PhantomData<fn() -> Submit>,
}

impl<Submit: Clone + 'static> PrimitiveFeatureRendererBase<Submit> {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            current_group: None,
            groups: Vec::new(),
            _submits: std::marker::PhantomData,
        };
        this
    }

    /// java: private static void executeDraw(RenderPass renderPass, PreparedPrimitiveDraw draw, StagedVertexBuffer.ExecuteInfo info)
    pub fn execute_draw(render_pass: &mut RenderPass, draw: &PreparedPrimitiveDraw, info: &ExecuteInfo) {
        render_pass.set_pipeline(&draw.pipeline());
        let mut instancing: InstancingParameters = draw.instancing();
        if instancing.name().is_some() && instancing.buffer().is_some() {
            render_pass.set_uniform(instancing.name().as_deref().unwrap(), instancing.buffer().unwrap());
        }
        if draw.texture_setup.texure0().is_some() {
            render_pass.bind_texture("Sampler0", draw.texture_setup.texure0(), draw.texture_setup.sampler0());
        }
        if draw.texture_setup.texure1().is_some() {
            render_pass.bind_texture("Sampler1", draw.texture_setup.texure1(), draw.texture_setup.sampler1());
        }
        if draw.texture_setup.texure2().is_some() {
            render_pass.bind_texture("Sampler2", draw.texture_setup.texure2(), draw.texture_setup.sampler2());
        }
        // TODO(fabric2seer): the port's `slice` ports this Java method with other parameters (buffer for Java's )
        // TODO(fabric2seer): `slice` takes 1 argument(s) in the port (`buffer: &Rc<dyn GpuBuffer>`), Java passes 0
        // rust: render_pass.set_vertex_buffer((0) as usize, Some(gpu_buffer::slice()));
        // TODO(fabric2seer): `ExecuteInfo.indexType` has no Rust method in the port (looked for `index_type` in StagedVertexBuffer and JavaSrc)
        // rust: render_pass.set_index_buffer(info.index_buffer().clone(), info.index_type());
        // TODO(fabric2seer): `ExecuteInfo.indexCount` has no Rust method in the port (looked for `index_count` in StagedVertexBuffer and JavaSrc)
        // rust: render_pass.draw_indexed(info.index_count(), draw.instancing().count(), info.first_index(), info.base_vertex(), 0);
    }

    /// java: private static void applyViewOffsetZLayering()
    pub fn apply_view_offset_z_layering() {
        // TODO(fabric2seer): `with_model_view_stack` takes 1 argument(s) in the port (`f: impl FnOnce(&mut crate::org::joml::matrix4f_stack::Matrix4fStack) -> R`), Java passes 0
        // rust: let mut model_view_stack: Matrix4fStack = render_system::with_model_view_stack();
        let mut model_view_stack: Matrix4fStack = todo!("fabric2seer: the initialiser of `modelViewStack` (TODO above)");
        model_view_stack.push_matrix();
        render_system::get_projection_type().apply_layering_transform(&mut model_view_stack, 1.0f32);
    }

    /// java: private static void unapplyViewOffsetZLayering()
    pub fn unapply_view_offset_z_layering() {
        // TODO(fabric2seer): `with_model_view_stack` takes 1 argument(s) in the port (`f: impl FnOnce(&mut crate::org::joml::matrix4f_stack::Matrix4fStack) -> R`), Java passes 0
        // rust: render_system::with_model_view_stack().pop_matrix();
    }

}

/// java: the virtual methods of PrimitiveFeatureRenderer (implemented by PrimitiveFeatureRendererBase and every subclass)
pub trait PrimitiveFeatureRenderer<Submit> {
    fn primitive_feature_renderer_base(&self) -> &PrimitiveFeatureRendererBase<Submit>;
    fn primitive_feature_renderer_base_mut(&mut self) -> &mut PrimitiveFeatureRendererBase<Submit>;
    fn as_any(&self) -> &dyn std::any::Any;

    /// java: protected abstract void buildGroup(FeatureFrameContext context, List<Submit> submits);
    fn build_group(&self, context: &FeatureFrameContext, submits: &Vec<Submit>);

    /// java: protected final VertexConsumer getVertexBuilder(RenderPipeline pipeline)
    fn get_vertex_builder(&self, pipeline: &Rc<RenderPipeline>) -> Box<dyn VertexConsumer> {
        return Group::get_vertex_builder(&self.current_group(), pipeline.clone(), TextureSetup::no_texture(), instancing_parameters::NONE.get_mut().clone());
    }

    /// java: protected final VertexConsumer getVertexBuilder(RenderPipeline pipeline, TextureSetup textureSetup)
    fn get_vertex_builder_render_pipeline_texture_setup(&self, pipeline: &Rc<RenderPipeline>, texture_setup: &TextureSetup) -> Box<dyn VertexConsumer> {
        return Group::get_vertex_builder(&self.current_group(), pipeline.clone(), texture_setup.clone(), instancing_parameters::NONE.get_mut().clone());
    }

    /// java: protected final VertexConsumer getVertexBuilder(RenderPipeline pipeline, InstancingParameters instancing)
    fn get_vertex_builder_render_pipeline_instancing_parameters(&self, pipeline: &Rc<RenderPipeline>, instancing: &InstancingParameters) -> Box<dyn VertexConsumer> {
        return Group::get_vertex_builder(&self.current_group(), pipeline.clone(), TextureSetup::no_texture(), instancing.clone());
    }

    /// java: private PrimitiveFeatureRenderer.Group currentGroup()
    fn current_group(&self) -> compat::Shared<Group> {
        return self.primitive_feature_renderer_base().current_group.clone().unwrap();
    }

    /// java: @Override public final void prepareGroup(FeatureFrameContext context, List<Submit> submits, boolean strictlyOrdered)
    fn prepare_group(&mut self, context: &FeatureFrameContext, submits: &Vec<Submit>, mut strictly_ordered: bool) {
        // TODO(fabric2seer): the mod keeps the game's `StagedVertexBuffer` (Java holds a reference to the live object); the port's `StagedVertexBuffer` is not a value to copy and the port lends the mod no handle to it
        // rust: self.primitive_feature_renderer_base_mut().current_group = Some(Group::new(context.staged_vertex_buffer().clone(), !strictly_ordered));
        self.build_group(context, submits);
        // java: this.groups.add(this.currentGroup);
        let current_group: compat::Shared<Group> = self.primitive_feature_renderer_base().current_group.clone().unwrap();
        self.primitive_feature_renderer_base_mut().groups.push(current_group);
        self.primitive_feature_renderer_base_mut().current_group = None;
    }

    /// java: @Override public final void executeGroup(FeatureFrameContext context, int groupIndex, List<Submit> submits, boolean strictlyOrdered)
    fn execute_group(&self, context: &FeatureFrameContext, mut group_index: i32, submits: &Vec<Submit>, mut strictly_ordered: bool) {
        let mut group: compat::Shared<Group> = self.primitive_feature_renderer_base().groups[(group_index) as usize].clone();
        PrimitiveFeatureRendererBase::<Submit>::apply_view_offset_z_layering();
        let main_target = compat::minecraft().game_renderer.main_render_target();
        // TODO(fabric2seer): `with_dynamic_uniforms` takes 1 argument(s) in the port (`f: impl FnOnce(&mut DynamicUniforms) -> R`), Java passes 0
        // rust: let mut dynamic_transforms: GpuBufferSlice = render_system::with_dynamic_uniforms().write_transform(&render_system::get_model_view_matrix_copy());
        let mut dynamic_transforms: GpuBufferSlice = todo!("fabric2seer: the initialiser of `dynamicTransforms` (TODO above)");
        {
            // TODO(fabric2seer): a lambda for a parameter the port does not type as a closure (`Supplier<String>`)
            // rust: let mut render_pass: RenderPass = render_system::get_device().create_command_encoder().create_render_pass_depth(|| "Skyblocker Primitive Feature Renderer".to_string(), main_target.get_color_texture_view().unwrap(), None, (if main_target.use_depth { main_target.get_depth_texture_view() } else { None }), None);
            let mut render_pass: RenderPass = todo!("fabric2seer: the initialiser of `renderPass` (TODO above)");
            render_system::bind_default_uniforms(&mut render_pass);
            render_pass.set_uniform("DynamicTransforms", dynamic_transforms.clone());
            for i in 0..(group.lock().borrow().draws.len() as i32) {
                let primitive_draw = &group.lock().borrow().draw_primitives[(i) as usize];
                // TODO(fabric2seer): `StagedVertexBuffer.getExecuteInfo` has no Rust method in the port (looked for `get_execute_info` in StagedVertexBuffer and its supertypes)
                // rust: let mut info: Option<ExecuteInfo> = Some(context.staged_vertex_buffer().get_execute_info(&group.lock().borrow().draws[(i) as usize]));
                let mut info: ExecuteInfo = todo!("fabric2seer: the initialiser of `info` (TODO above)");
                if true /* java: info != null: never null here */ {
                    PrimitiveFeatureRendererBase::<Submit>::execute_draw(&mut render_pass, primitive_draw, &info);
                }
            }
        }
        PrimitiveFeatureRendererBase::<Submit>::unapply_view_offset_z_layering();
    }

    /// java: @Override public final void finishExecute(FeatureFrameContext context)
    fn finish_execute(&mut self, context: &FeatureFrameContext) {
        self.primitive_feature_renderer_base_mut().groups.clear();
    }

}

/// java: private static class Group
pub struct Group {
    /// java: private final StagedVertexBuffer stagedBuffer;
    pub staged_buffer: StagedVertexBuffer,
    /// java: private final boolean canReorder;
    pub can_reorder: bool,
    /// java: private final List<StagedVertexBuffer.Draw> draws = new ArrayList<>(); (the port's draws are the staged
    /// buffer's indices)
    pub draws: Vec<usize>,
    /// java: private final List<PreparedPrimitiveDraw> drawPrimitives = new ArrayList<>();
    pub draw_primitives: Vec<PreparedPrimitiveDraw>,
    /// java: private @Nullable RenderPipeline lastPipeline;
    pub last_pipeline: Option<Rc<RenderPipeline>>,
    /// java: private @Nullable TextureSetup lastTextureSetup;
    pub last_texture_setup: Option<TextureSetup>,
    /// java: private @Nullable InstancingParameters lastInstancing;
    pub last_instancing: Option<InstancingParameters>,
    /// java: private StagedVertexBuffer.@Nullable Draw lastDraw;
    pub last_draw: Option<usize>,
}

impl Group {
    /// java: private Group(StagedVertexBuffer stagedBuffer, boolean canReorder)
    pub fn new(mut staged_buffer: StagedVertexBuffer, mut can_reorder: bool) -> compat::Shared<Self> {
        // TODO(fabric2seer): the mod keeps the game's `StagedVertexBuffer` (Java holds a reference to the live object); the port's `StagedVertexBuffer` is not a value to copy and the port lends the mod no handle to it
        //   java: this.stagedBuffer = stagedBuffer;
        let __init_can_reorder = can_reorder;
        let __this = compat::shared(Self {
            staged_buffer: todo!("fabric2seer: the field value (TODO above)"),
            can_reorder: __init_can_reorder,
            draws: Vec::new(),
            draw_primitives: Vec::new(),
            last_pipeline: None,
            last_texture_setup: None,
            last_instancing: None,
            last_draw: None,
        });
        __this
    }

    /// java: private static boolean canConsolidateConsecutiveGeometry(RenderPipeline pipeline)
    pub fn can_consolidate_consecutive_geometry(pipeline: &Rc<RenderPipeline>) -> bool {
        return !(pipeline.get_primitive_topology().connected_primitives());
    }

    /// java: public VertexConsumer getVertexBuilder(RenderPipeline pipeline, TextureSetup textureSetup, InstancingParameters instancing)
    pub fn get_vertex_builder(__this: &compat::Shared<Self>, mut pipeline: Rc<RenderPipeline>, mut texture_setup: TextureSetup, mut instancing: InstancingParameters) -> Box<dyn VertexConsumer> {
        // java: this.lastDraw == null || pipeline != this.lastPipeline || textureSetup != this.lastTextureSetup
        //       || this.lastInstancing != instancing || !canConsolidateConsecutiveGeometry(pipeline)
        // (the pipeline by identity; the two records - values here - by their components)
        if compat::read(__this, |__t| __t.last_draw).is_none()
            || !compat::read(__this, |__t| __t.last_pipeline.as_ref().is_some_and(|last| Rc::ptr_eq(&pipeline, last)))
            || compat::read(__this, |__t| __t.last_texture_setup.as_ref() != Some(&texture_setup))
            || compat::read(__this, |__t| __t.last_instancing.as_ref() != Some(&instancing))
            || !(Group::can_consolidate_consecutive_geometry(&pipeline)) {
            { let __v = Some(Group::get_or_add_draw(__this, &pipeline, &texture_setup, &instancing)); compat::write(__this, |__t| __t.last_draw = __v); }
            { let __v = Some(pipeline.clone()); compat::write(__this, |__t| __t.last_pipeline = __v); }
            { let __v = Some(texture_setup.clone()); compat::write(__this, |__t| __t.last_texture_setup = __v); }
            { let __v = Some(instancing.clone()); compat::write(__this, |__t| __t.last_instancing = __v); }
        }
        // TODO(fabric2seer): `StagedVertexBuffer.getVertexBuilder` has no Rust method in the port (looked for `get_vertex_builder` in StagedVertexBuffer and its supertypes)
        // rust: let __h0 = &compat::read(__this, |__t| __t.last_draw.clone()).unwrap();
        // rust: return __this.lock().borrow().staged_buffer.get_vertex_builder(__h0);
        return todo!("fabric2seer: see the TODO above");
    }

    /// java: private StagedVertexBuffer.Draw getOrAddDraw(RenderPipeline pipeline, TextureSetup textureSetup, InstancingParameters instancing)
    pub fn get_or_add_draw(__this: &compat::Shared<Self>, pipeline: &Rc<RenderPipeline>, texture_setup: &TextureSetup, instancing: &InstancingParameters) -> usize {
        let mut prepared_primitive_draw: PreparedPrimitiveDraw = PreparedPrimitiveDraw::new(pipeline.clone(), texture_setup.clone(), instancing.clone());
        let mut existing_index: i32 = (if compat::read(__this, |__t| __t.can_reorder) && Group::can_consolidate_consecutive_geometry(pipeline) { compat::read(__this, |__t| __t.draw_primitives.iter().position(|e| *e == prepared_primitive_draw).map_or(-1, |i| i as i32)) } else { -1 });
        if existing_index != -1 {
            // java: return this.draws.get(existingIndex);
            return compat::read(__this, |__t| __t.draws[(existing_index) as usize]);
        }
        let mut draw: usize = { let (__q0, __q1,) = (pipeline.get_vertex_format_binding((0) as usize).unwrap().clone(), pipeline.get_primitive_topology(),); __this.lock().borrow_mut().staged_buffer.append_draw(__q0, __q1) };
        // java: this.draws.add(draw);
        __this.lock().borrow_mut().draws.push(draw);
        let __h0 = prepared_primitive_draw.clone();
        __this.lock().borrow_mut().draw_primitives.push(__h0);
        return draw;
    }

}

/// java: private record PreparedPrimitiveDraw(RenderPipeline pipeline, TextureSetup textureSetup, InstancingParameters instancing)
#[derive(Clone, Debug)]
pub struct PreparedPrimitiveDraw {
    /// java: RenderPipeline pipeline
    pub pipeline: Rc<RenderPipeline>,
    /// java: TextureSetup textureSetup
    pub texture_setup: TextureSetup,
    /// java: InstancingParameters instancing
    pub instancing: InstancingParameters,
}

impl PreparedPrimitiveDraw {
    /// java: the canonical record constructor
    pub fn new(mut pipeline: Rc<RenderPipeline>, mut texture_setup: TextureSetup, mut instancing: InstancingParameters) -> Self {
        Self { pipeline, texture_setup, instancing }
    }

    /// java: the record accessor `pipeline()`
    pub fn pipeline(&self) -> Rc<RenderPipeline> {
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
        Rc::ptr_eq(&self.pipeline, &other.pipeline) && self.texture_setup == other.texture_setup && self.instancing == other.instancing
    }
}

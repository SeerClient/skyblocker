//! Ported from de.hysky.skyblocker.utils.render.primitive.TexturedQuadFeatureRenderer by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_feature_renderer;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_feature_renderer::PrimitiveFeatureRenderer;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_feature_renderer::PrimitiveFeatureRendererBase;
use crate::de::hysky::skyblocker::utils::render::skyblocker_render_pipelines;
use crate::de::hysky::skyblocker::utils::render::state::textured_quad_render_state::TexturedQuadRenderState;
use mcrust::com::mojang::blaze3d::systems::render_system;
use mcrust::com::mojang::blaze3d::textures::filter_mode::FilterMode;
use mcrust::com::mojang::blaze3d::vertex::vertex_consumer::VertexConsumer;
use mcrust::net::minecraft::client::gui::render::texture_setup::TextureSetup;
use mcrust::net::minecraft::client::renderer::feature::feature_frame_context::FeatureFrameContext;
use mcrust::net::minecraft::client::renderer::feature::feature_renderer_type;
use mcrust::net::minecraft::client::renderer::feature::feature_renderer_type::FeatureRendererType;
use mcrust::net::minecraft::client::renderer::feature::submit::submit_node::SubmitNode;
use mcrust::net::minecraft::client::renderer::state::level::camera_render_state::CameraRenderState;
use mcrust::org::joml::matrix4f::Matrix4f;

/// java: public static final FeatureRendererType<Submit> TYPE = FeatureRendererType.create("Skyblocker Textured Quad");
pub static TYPE: compat::MainThread<FeatureRendererType> = compat::MainThread::new(|| feature_renderer_type::create("Skyblocker Textured Quad"));

/// java: public class TexturedQuadFeatureRenderer extends PrimitiveFeatureRenderer<TexturedQuadFeatureRenderer.Submit>
#[derive(Clone)]
pub struct TexturedQuadFeatureRenderer {
    /// java: extends PrimitiveFeatureRenderer
    pub base: PrimitiveFeatureRendererBase<Submit>,
}

impl std::ops::Deref for TexturedQuadFeatureRenderer {
    type Target = PrimitiveFeatureRendererBase<Submit>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for TexturedQuadFeatureRenderer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl TexturedQuadFeatureRenderer {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            base: PrimitiveFeatureRendererBase::<Submit>::new(),
        };
        this
    }

}

impl PrimitiveFeatureRenderer<crate::de::hysky::skyblocker::utils::render::primitive::textured_quad_feature_renderer::Submit> for TexturedQuadFeatureRenderer {
    fn primitive_feature_renderer_base(&self) -> &PrimitiveFeatureRendererBase<crate::de::hysky::skyblocker::utils::render::primitive::textured_quad_feature_renderer::Submit> {
        &self.base
    }
    fn primitive_feature_renderer_base_mut(&mut self) -> &mut PrimitiveFeatureRendererBase<crate::de::hysky::skyblocker::utils::render::primitive::textured_quad_feature_renderer::Submit> {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override protected void buildGroup(FeatureFrameContext context, List<Submit> submits)
    fn build_group(&mut self, context: &mut FeatureFrameContext<'_>, submits: &[std::sync::Arc<dyn SubmitNode>]) {
        for submit in primitive_feature_renderer::submits::<Submit>(submits) {
            for state in submit.states.iter() {
                // java: TextureSetup textureSetup = TextureSetup.singleTexture(context.textureManager().getTexture(state.texture()).getTextureView(), RenderSystem.getSamplerCache().getClampToEdge(FilterMode.NEAREST));
                let texture_view = context.main.texture_manager.get_texture(context.shared.resource_manager, &state.texture()).get_texture_view().clone();
                let texture_setup: TextureSetup = TextureSetup::single_texture(texture_view, render_system::with_sampler_cache(|cache| cache.get_clamp_to_edge(FilterMode::Nearest)));
                let builder = self.get_vertex_builder_render_pipeline_texture_setup(context, &(if submit.through_walls() { skyblocker_render_pipelines::TEXTURE_THROUGH_WALLS.get_mut().clone() } else { skyblocker_render_pipelines::TEXTURE.get_mut().clone() }), &texture_setup);
                let mut position_matrix: Matrix4f = (Matrix4f::new().translate((((state.pos().x() - submit.camera.pos.x())) as f32), (((state.pos().y() - submit.camera.pos.y())) as f32), (((state.pos().z() - submit.camera.pos.z())) as f32))).rotate_quat(&submit.camera.orientation);
                { builder.add_vertex_matrix(&position_matrix, ((state.render_offset().x()) as f32), ((state.render_offset().y()) as f32), ((state.render_offset().z()) as f32)); builder.set_uv((1) as f32, ((1 as f32) - state.texture_height())); builder.set_color_f32(state.shader_colour()[(0) as usize], state.shader_colour()[(1) as usize], state.shader_colour()[(2) as usize], state.alpha()); };
                { builder.add_vertex_matrix(&position_matrix, ((state.render_offset().x()) as f32), (((state.render_offset().y()) as f32) + state.height()), ((state.render_offset().z()) as f32)); builder.set_uv((1) as f32, (1) as f32); builder.set_color_f32(state.shader_colour()[(0) as usize], state.shader_colour()[(1) as usize], state.shader_colour()[(2) as usize], state.alpha()); };
                { builder.add_vertex_matrix(&position_matrix, (((state.render_offset().x()) as f32) + state.width()), (((state.render_offset().y()) as f32) + state.height()), ((state.render_offset().z()) as f32)); builder.set_uv(((1 as f32) - state.texture_width()), (1) as f32); builder.set_color_f32(state.shader_colour()[(0) as usize], state.shader_colour()[(1) as usize], state.shader_colour()[(2) as usize], state.alpha()); };
                { builder.add_vertex_matrix(&position_matrix, (((state.render_offset().x()) as f32) + state.width()), ((state.render_offset().y()) as f32), ((state.render_offset().z()) as f32)); builder.set_uv(((1 as f32) - state.texture_width()), ((1 as f32) - state.texture_height())); builder.set_color_f32(state.shader_colour()[(0) as usize], state.shader_colour()[(1) as usize], state.shader_colour()[(2) as usize], state.alpha()); };
            }
        }
    }

}

/// java: public record Submit(List<TexturedQuadRenderState> states, CameraRenderState camera, boolean throughWalls) implements SubmitNode
#[derive(Clone, Debug)]
pub struct Submit {
    /// java: List<TexturedQuadRenderState> states
    pub states: Vec<TexturedQuadRenderState>,
    /// java: CameraRenderState camera
    pub camera: CameraRenderState,
    /// java: boolean throughWalls
    pub through_walls: bool,
}

impl Submit {
    /// java: the canonical record constructor
    pub fn new(mut states: Vec<TexturedQuadRenderState>, mut camera: CameraRenderState, mut through_walls: bool) -> Self {
        Self { states, camera, through_walls }
    }

    /// java: the record accessor `states()`
    pub fn states(&self) -> Vec<TexturedQuadRenderState> {
        self.states.clone()
    }

    /// java: the record accessor `camera()`
    pub fn camera(&self) -> CameraRenderState {
        self.camera.clone()
    }

    /// java: the record accessor `throughWalls()`
    pub fn through_walls(&self) -> bool {
        self.through_walls
    }

    /// java: @Override public FeatureRendererType<? extends SubmitNode> featureType()
    pub fn feature_type(&self) -> FeatureRendererType {
        return TYPE.get_mut().clone();
    }

}

/// java: `implements SubmitNode` - the record's `featureType()`; `as_any` is the downcast `FeatureRenderer` reads it back through
impl SubmitNode for Submit {
    fn feature_type(&self) -> FeatureRendererType {
        *TYPE.get()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

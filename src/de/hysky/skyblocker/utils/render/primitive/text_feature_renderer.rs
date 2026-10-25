//! Ported from de.hysky.skyblocker.utils.render.primitive.TextFeatureRenderer by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_feature_renderer;
use crate::de::hysky::skyblocker::compatibility::caxton_compatibility::CaxtonCompatibility;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_feature_renderer::PrimitiveFeatureRenderer;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_feature_renderer::PrimitiveFeatureRendererBase;
use crate::de::hysky::skyblocker::utils::render::state::text_render_state::TextRenderState;
use mcrust::com::mojang::blaze3d::pipeline::render_pipeline::RenderPipeline;
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
use mcrust::net::minecraft::client::gui::font::SheetGlyph;
use mcrust::net::minecraft::client::gui::font::glyphs::baked_sheet_glyph;

/// java: public static final FeatureRendererType<Submit> TYPE = FeatureRendererType.create("Skyblocker Text");
pub static TYPE: compat::MainThread<FeatureRendererType> = compat::MainThread::new(|| feature_renderer_type::create("Skyblocker Text"));
/// java: private static final @Nullable RenderPipeline CAXTON_SEE_THROUGH = CaxtonCompatibility.getSeeThroughTextPipeline().orElse(null);
pub static CAXTON_SEE_THROUGH: compat::MainThread<Option<std::sync::Arc<RenderPipeline>>> = compat::MainThread::new(|| CaxtonCompatibility::get_see_through_text_pipeline().map(std::sync::Arc::new));
/// java: private static final @Nullable RenderPipeline CAXTON_NORMAL = CaxtonCompatibility.getTextPipeline().orElse(null);
pub static CAXTON_NORMAL: compat::MainThread<Option<std::sync::Arc<RenderPipeline>>> = compat::MainThread::new(|| CaxtonCompatibility::get_text_pipeline().map(std::sync::Arc::new));

/// java: public class TextFeatureRenderer extends PrimitiveFeatureRenderer<TextFeatureRenderer.Submit>
#[derive(Clone)]
pub struct TextFeatureRenderer {
    /// java: extends PrimitiveFeatureRenderer
    pub base: PrimitiveFeatureRendererBase<Submit>,
}

impl std::ops::Deref for TextFeatureRenderer {
    type Target = PrimitiveFeatureRendererBase<Submit>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for TextFeatureRenderer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl TextFeatureRenderer {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            base: PrimitiveFeatureRendererBase::<Submit>::new(),
        };
        this
    }

    /// java: the anonymous `Font.GlyphVisitor.acceptRenderable(TextRenderable renderable)` up to `renderable.render(..)`:
    /// the texture setup and the builder for the renderable's sheet.
    pub fn accept_renderable<'c>(&mut self, context: &'c mut FeatureFrameContext<'_>, through_walls: bool, renderable: &SheetGlyph) -> &'c mut mcrust::com::mojang::blaze3d::vertex::buffer_builder::BufferBuilder {
        // java: TextureSetup textureSetup = TextureSetup.singleTextureWithLightmap(renderable.textureView(), RenderSystem.getSamplerCache().getClampToEdge(FilterMode.NEAREST));
        let texture_view = context.main.texture_manager.get_texture(context.shared.resource_manager, &renderable.sheet).get_texture_view().clone();
        let texture_setup: TextureSetup = TextureSetup::single_texture_with_lightmap(texture_view, render_system::with_sampler_cache(|cache| cache.get_clamp_to_edge(FilterMode::Nearest)), Some(context.main.lightmap.clone()));
        // This is a bit of a weird workaround to know if the intensity pipelines should be used instead of the normal ones.
        // java: renderable.guiPipeline() == RenderPipelines.GUI_TEXT_GRAYSCALE - a grayscale (TrueType) sheet: `!colored`
        let pipeline = TextFeatureRenderer::get_pipeline(through_walls, !renderable.colored);
        self.get_vertex_builder_render_pipeline_texture_setup(context, &pipeline, &texture_setup)
    }

    /// `Font.getGlyph:84-90` - an obfuscated glyph's random stand-in, as Java's `PreparedText` resolved it (the port
    /// resolves glyphs at visit time, so the visitor draws the random one the game's text renderer would).
    fn get_glyph(font: &mcrust::net::minecraft::client::gui::font::Font<'_>, placement: &mcrust::net::minecraft::client::gui::font::GlyphPlacement) -> mcrust::net::minecraft::client::gui::glyph_source::RandomGlyph {
        thread_local! {
            static OBFUSCATION_RANDOM: std::cell::RefCell<mcrust::net::minecraft::util::random_source::LegacyRandomSource> =
                std::cell::RefCell::new(mcrust::net::minecraft::util::random_source::LegacyRandomSource::new_unique());
        }
        if placement.obfuscated && placement.codepoint != 32 {
            let target_width = mcrust::net::minecraft::util::mth::ceil_f32(placement.source.glyph_info(font.set, placement.codepoint).advance);
            return OBFUSCATION_RANDOM.with(|random| placement.source.random_codepoint(font.set, &mut random.borrow_mut(), target_width));
        }
        mcrust::net::minecraft::client::gui::glyph_source::RandomGlyph::Original
    }

    /// `glyph.createGlyph(x, y, color, shadowColor, style, boldOffset, shadowOffset)` (`BakedSheetGlyph.GlyphInstance`) for a
    /// placed glyph; a sprite glyph is drawn unsheared and single (`PlainTextRenderable.render`).
    fn glyph_instance(placement: &mcrust::net::minecraft::client::gui::font::GlyphPlacement) -> baked_sheet_glyph::GlyphInstance {
        use mcrust::net::minecraft::client::gui::glyph_source::GlyphSource;
        let sprite = matches!(placement.source, GlyphSource::AtlasSprite(_) | GlyphSource::PlayerSprite(_));
        baked_sheet_glyph::GlyphInstance {
            x: placement.x,
            y: placement.y,
            color: placement.color,
            shadow_color: placement.shadow_color,
            italic: placement.italic && !sprite,
            bold: placement.bold && !sprite,
            bold_offset: placement.bold_offset,
            shadow_offset: placement.shadow_offset,
        }
    }

    /// `createEffect(x0, y0, x1, y1, depth, color, shadowColor, shadowOffset)` (`BakedSheetGlyph.EffectInstance`).
    fn effect_instance(effect: &mcrust::net::minecraft::client::gui::font::Effect) -> baked_sheet_glyph::EffectInstance {
        baked_sheet_glyph::EffectInstance {
            x0: effect.x0,
            y0: effect.y0,
            x1: effect.x1,
            y1: effect.y1,
            depth: effect.depth,
            color: effect.color,
            shadow_color: effect.shadow_color,
            shadow_offset: effect.shadow_offset,
        }
    }

    /// java: private static RenderPipeline getPipeline(boolean seeThrough, boolean greyscale)
    pub fn get_pipeline(mut see_through: bool, mut greyscale: bool) -> std::sync::Arc<RenderPipeline> {
        if see_through {
            return (if CAXTON_SEE_THROUGH.get_mut().is_some() { CAXTON_SEE_THROUGH.get_mut().clone() } else { Some((if greyscale { mcrust::net::minecraft::client::renderer::render_pipelines::text_grayscale_see_through() } else { mcrust::net::minecraft::client::renderer::render_pipelines::text_see_through() })) }).unwrap();
        } else {
            return (if CAXTON_NORMAL.get_mut().is_some() { CAXTON_NORMAL.get_mut().clone() } else { Some((if greyscale { mcrust::net::minecraft::client::renderer::render_pipelines::text_grayscale() } else { mcrust::net::minecraft::client::renderer::render_pipelines::text() })) }).unwrap();
        }
    }

}

impl PrimitiveFeatureRenderer<crate::de::hysky::skyblocker::utils::render::primitive::text_feature_renderer::Submit> for TextFeatureRenderer {
    fn primitive_feature_renderer_base(&self) -> &PrimitiveFeatureRendererBase<crate::de::hysky::skyblocker::utils::render::primitive::text_feature_renderer::Submit> {
        &self.base
    }
    fn primitive_feature_renderer_base_mut(&mut self) -> &mut PrimitiveFeatureRendererBase<crate::de::hysky::skyblocker::utils::render::primitive::text_feature_renderer::Submit> {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override protected void buildGroup(FeatureFrameContext context, List<Submit> submits)
    fn build_group(&mut self, context: &mut FeatureFrameContext<'_>, submits: &[std::sync::Arc<dyn SubmitNode>]) {
        for submit in primitive_feature_renderer::submits::<Submit>(submits) {
            for state in submit.states.iter() {
                let mut position_matrix: Matrix4f = ((Matrix4f::new().translate((((state.pos().x() - submit.camera.pos.x())) as f32), (((state.pos().y() - submit.camera.pos.y())) as f32), (((state.pos().z() - submit.camera.pos.z())) as f32))).rotate_quat(&submit.camera.orientation)).scale(state.scale(), (-state.scale()), state.scale());
                // java: state.glyphs().visit(new Font.GlyphVisitor() { acceptRenderable(renderable) { .. } });
                // `PreparedText.visit` hands over the background (none: `prepareText(.., 0)`), every glyph, then every
                // effect; each `renderable` is the port's sheet glyph, resolved from the frame's font set.
                let Some(font) = context.shared.font else {
                    continue;
                };
                let glyphs = &state.glyphs;
                for placement in &glyphs.glyphs {
                    let random_glyph = TextFeatureRenderer::get_glyph(font, placement);
                    let Some(sheet_glyph) = font.resolve_random(context.main.texture_manager, Some(context.main.atlas_manager), &placement.source, placement.codepoint, random_glyph) else {
                        continue;
                    };
                    let instance = TextFeatureRenderer::glyph_instance(placement);
                    let builder = self.accept_renderable(context, submit.through_walls, &sheet_glyph);
                    baked_sheet_glyph::render_glyph_instance(&sheet_glyph.glyph, &instance, &position_matrix, builder, mcrust::net::minecraft::util::light_coords_util::FULL_BRIGHT, false);
                    if let Some(overlay) = &sheet_glyph.overlay {
                        baked_sheet_glyph::render_glyph_instance(overlay, &instance, &position_matrix, builder, mcrust::net::minecraft::util::light_coords_util::FULL_BRIGHT, false);
                    }
                }
                if !glyphs.effects.is_empty() {
                    if let Some(white) = font.effect(context.main.texture_manager) {
                        for effect in &glyphs.effects {
                            let instance = TextFeatureRenderer::effect_instance(effect);
                            let builder = self.accept_renderable(context, submit.through_walls, &white);
                            baked_sheet_glyph::render_effect_instance(&white.glyph, &instance, &position_matrix, builder, mcrust::net::minecraft::util::light_coords_util::FULL_BRIGHT);
                        }
                    }
                }
            }
        }
    }

}

/// java: public record Submit(List<TextRenderState> states, CameraRenderState camera, boolean throughWalls) implements SubmitNode
#[derive(Clone, Debug)]
pub struct Submit {
    /// java: List<TextRenderState> states
    pub states: Vec<TextRenderState>,
    /// java: CameraRenderState camera
    pub camera: CameraRenderState,
    /// java: boolean throughWalls
    pub through_walls: bool,
}

impl Submit {
    /// java: the canonical record constructor
    pub fn new(mut states: Vec<TextRenderState>, mut camera: CameraRenderState, mut through_walls: bool) -> Self {
        Self { states, camera, through_walls }
    }

    /// java: the record accessor `states()`
    pub fn states(&self) -> Vec<TextRenderState> {
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

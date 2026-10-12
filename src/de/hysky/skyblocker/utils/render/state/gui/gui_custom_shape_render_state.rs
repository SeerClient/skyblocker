//! Ported from de.hysky.skyblocker.utils.render.state.gui.GuiCustomShapeRenderState by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::com::mojang::blaze3d::pipeline::render_pipeline::RenderPipeline;
use mcrust::com::mojang::blaze3d::vertex::vertex_consumer::VertexConsumer;
use mcrust::net::minecraft::client::gui::navigation::screen_rectangle::ScreenRectangle;
use mcrust::net::minecraft::client::gui::render::texture_setup::TextureSetup;
use mcrust::org::joml::matrix3x2f::Matrix3x2f;
use mcrust::org::joml::vector2f::Vector2f;

/// java: public record GuiCustomShapeRenderState( RenderPipeline pipeline, TextureSetup textureSetup, Matrix3x2f matrix, List<Vector2f> inputVertices, int color, @Nullable ScreenRectangle scissorArea, @Nullable ScreenRectangle bounds) implements Gui
#[derive(Clone, Debug)]
pub struct GuiCustomShapeRenderState {
    /// java: RenderPipeline pipeline
    pub pipeline: RenderPipeline,
    /// java: TextureSetup textureSetup
    pub texture_setup: TextureSetup,
    /// java: Matrix3x2f matrix
    pub matrix: Matrix3x2f,
    /// java: List<Vector2f> inputVertices
    pub input_vertices: Vec<Vector2f>,
    /// java: int color
    pub color: i32,
    /// java: ScreenRectangle scissorArea
    pub scissor_area: Option<ScreenRectangle>,
    /// java: ScreenRectangle bounds
    pub bounds: Option<ScreenRectangle>,
}

impl GuiCustomShapeRenderState {
    /// java: public GuiCustomShapeRenderState( RenderPipeline pipeline, TextureSetup textureSetup, Matrix3x2f pose, List<Vector2f> inputVertices, int color, @Nullable ScreenRectangle scissorArea )
    pub fn new(mut pipeline: RenderPipeline, mut texture_setup: TextureSetup, mut pose: Matrix3x2f, mut input_vertices: Vec<Vector2f>, mut color: i32, mut scissor_area: Option<ScreenRectangle>) -> Self {
        let mut this = Self::new_canonical(pipeline.clone(), texture_setup.clone(), pose.clone(), input_vertices.clone(), color, scissor_area.clone(), GuiCustomShapeRenderState::create_bounds(&input_vertices, &pose, scissor_area.clone()));
        this
    }

    /// java: the canonical record constructor
    pub fn new_canonical(mut pipeline: RenderPipeline, mut texture_setup: TextureSetup, mut matrix: Matrix3x2f, mut input_vertices: Vec<Vector2f>, mut color: i32, mut scissor_area: Option<ScreenRectangle>, mut bounds: Option<ScreenRectangle>) -> Self {
        Self { pipeline, texture_setup, matrix, input_vertices, color, scissor_area, bounds }
    }

    /// java: the record accessor `pipeline()`
    pub fn pipeline(&self) -> RenderPipeline {
        self.pipeline.clone()
    }

    /// java: the record accessor `textureSetup()`
    pub fn texture_setup(&self) -> TextureSetup {
        self.texture_setup.clone()
    }

    /// java: the record accessor `matrix()`
    pub fn matrix(&self) -> Matrix3x2f {
        self.matrix.clone()
    }

    /// java: the record accessor `inputVertices()`
    pub fn input_vertices(&self) -> Vec<Vector2f> {
        self.input_vertices.clone()
    }

    /// java: the record accessor `color()`
    pub fn color(&self) -> i32 {
        self.color
    }

    /// java: the record accessor `scissorArea()`
    pub fn scissor_area(&self) -> Option<ScreenRectangle> {
        self.scissor_area.clone()
    }

    /// java: the record accessor `bounds()`
    pub fn bounds(&self) -> Option<ScreenRectangle> {
        self.bounds.clone()
    }

    /// java: @Nullable private static ScreenRectangle createBounds(List<Vector2f> inputVertices, Matrix3x2f pose, @Nullable ScreenRectangle scissorArea)
    pub fn create_bounds(input_vertices: &Vec<Vector2f>, pose: &Matrix3x2f, mut scissor_area: Option<ScreenRectangle>) -> Option<ScreenRectangle> {
        let mut x0: i32 = (input_vertices.iter().cloned().map(|vertex| ((vertex.x) as i32)).collect::<Vec<_>>()).iter().max().cloned().unwrap();
        let mut x1: i32 = (input_vertices.iter().cloned().map(|vertex| ((vertex.x) as i32)).collect::<Vec<_>>()).iter().min().cloned().unwrap();
        let mut y0: i32 = (input_vertices.iter().cloned().map(|vertex| ((vertex.y) as i32)).collect::<Vec<_>>()).iter().max().cloned().unwrap();
        let mut y1: i32 = (input_vertices.iter().cloned().map(|vertex| ((vertex.y) as i32)).collect::<Vec<_>>()).iter().min().cloned().unwrap();
        let mut screen_rect: ScreenRectangle = (ScreenRectangle::new_at(x0, y0, x1.wrapping_sub(x0), y1.wrapping_sub(y0))).transform_max_bounds(pose);
        return (if scissor_area.is_some() { scissor_area.as_ref().unwrap().intersection(screen_rect.clone()) } else { Some(screen_rect.clone()) });
    }

    /// java: @Override public void buildVertices(VertexConsumer vertexConsumer)
    pub fn build_vertices(&self, vertex_consumer: &mut dyn VertexConsumer) {
        for vector in self.input_vertices.iter() {
            { vertex_consumer.add_vertex_with_2d_pose(&self.matrix(), vector.x(), vector.y()); vertex_consumer.set_color(self.color); &mut *vertex_consumer };
        }
    }

}

//! Ported from de.hysky.skyblocker.utils.render.state.gui.GuiHorizontalGradientRenderState by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::com::mojang::blaze3d::pipeline::render_pipeline::RenderPipeline;
use mcrust::com::mojang::blaze3d::vertex::vertex_consumer::VertexConsumer;
use mcrust::net::minecraft::client::gui::navigation::screen_rectangle::ScreenRectangle;
use mcrust::net::minecraft::client::gui::render::texture_setup::TextureSetup;
use mcrust::org::joml::matrix3x2f::Matrix3x2f;

/// java: public record GuiHorizontalGradientRenderState( RenderPipeline pipeline, TextureSetup textureSetup, Matrix3x2f matrix, int startX, int startY, int endX, int endY, int colorStart, int colorEnd, @Nullable ScreenRectangle scissorArea, @Nullabl
#[derive(Clone, Debug)]
pub struct GuiHorizontalGradientRenderState {
    /// java: RenderPipeline pipeline
    pub pipeline: RenderPipeline,
    /// java: TextureSetup textureSetup
    pub texture_setup: TextureSetup,
    /// java: Matrix3x2f matrix
    pub matrix: Matrix3x2f,
    /// java: int startX
    pub start_x: i32,
    /// java: int startY
    pub start_y: i32,
    /// java: int endX
    pub end_x: i32,
    /// java: int endY
    pub end_y: i32,
    /// java: int colorStart
    pub color_start: i32,
    /// java: int colorEnd
    pub color_end: i32,
    /// java: ScreenRectangle scissorArea
    pub scissor_area: Option<ScreenRectangle>,
    /// java: ScreenRectangle bounds
    pub bounds: Option<ScreenRectangle>,
}

impl GuiHorizontalGradientRenderState {
    /// java: public GuiHorizontalGradientRenderState( RenderPipeline pipeline, TextureSetup textureSetup, Matrix3x2f pose, int startX, int startY, int endX, int endY, int colorStart, int colorEnd, @Nullable Screen
    pub fn new(mut pipeline: RenderPipeline, mut texture_setup: TextureSetup, mut pose: Matrix3x2f, mut start_x: i32, mut start_y: i32, mut end_x: i32, mut end_y: i32, mut color_start: i32, mut color_end: i32, mut scissor_area: Option<ScreenRectangle>) -> Self {
        let mut this = Self::new_canonical(pipeline.clone(), texture_setup.clone(), pose.clone(), start_x, start_y, end_x, end_y, color_start, color_end, scissor_area.clone(), GuiHorizontalGradientRenderState::create_bounds(start_x, start_y, end_x, end_y, &pose, scissor_area.clone()));
        this
    }

    /// java: the canonical record constructor
    pub fn new_canonical(mut pipeline: RenderPipeline, mut texture_setup: TextureSetup, mut matrix: Matrix3x2f, mut start_x: i32, mut start_y: i32, mut end_x: i32, mut end_y: i32, mut color_start: i32, mut color_end: i32, mut scissor_area: Option<ScreenRectangle>, mut bounds: Option<ScreenRectangle>) -> Self {
        Self { pipeline, texture_setup, matrix, start_x, start_y, end_x, end_y, color_start, color_end, scissor_area, bounds }
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

    /// java: the record accessor `startX()`
    pub fn start_x(&self) -> i32 {
        self.start_x
    }

    /// java: the record accessor `startY()`
    pub fn start_y(&self) -> i32 {
        self.start_y
    }

    /// java: the record accessor `endX()`
    pub fn end_x(&self) -> i32 {
        self.end_x
    }

    /// java: the record accessor `endY()`
    pub fn end_y(&self) -> i32 {
        self.end_y
    }

    /// java: the record accessor `colorStart()`
    pub fn color_start(&self) -> i32 {
        self.color_start
    }

    /// java: the record accessor `colorEnd()`
    pub fn color_end(&self) -> i32 {
        self.color_end
    }

    /// java: the record accessor `scissorArea()`
    pub fn scissor_area(&self) -> Option<ScreenRectangle> {
        self.scissor_area.clone()
    }

    /// java: the record accessor `bounds()`
    pub fn bounds(&self) -> Option<ScreenRectangle> {
        self.bounds.clone()
    }

    /// java: @Nullable private static ScreenRectangle createBounds(int x0, int y0, int x1, int y1, Matrix3x2f pose, @Nullable ScreenRectangle scissorArea)
    pub fn create_bounds(mut x0: i32, mut y0: i32, mut x1: i32, mut y1: i32, pose: &Matrix3x2f, mut scissor_area: Option<ScreenRectangle>) -> Option<ScreenRectangle> {
        let mut screen_rect: ScreenRectangle = (ScreenRectangle::new_at(x0, y0, x1.wrapping_sub(x0), y1.wrapping_sub(y0))).transform_max_bounds(pose);
        return (if scissor_area.is_some() { scissor_area.as_ref().unwrap().intersection(screen_rect.clone()) } else { Some(screen_rect.clone()) });
    }

    /// java: @Override public void buildVertices(VertexConsumer vertices)
    pub fn build_vertices(&self, vertices: &mut dyn VertexConsumer) {
        { vertices.add_vertex_with_2d_pose(&self.matrix(), (self.start_x()) as f32, (self.start_y()) as f32); vertices.set_color(self.color_start()); &mut *vertices };
        { vertices.add_vertex_with_2d_pose(&self.matrix(), (self.start_x()) as f32, (self.end_y()) as f32); vertices.set_color(self.color_start()); &mut *vertices };
        { vertices.add_vertex_with_2d_pose(&self.matrix(), (self.end_x()) as f32, (self.end_y()) as f32); vertices.set_color(self.color_end()); &mut *vertices };
        { vertices.add_vertex_with_2d_pose(&self.matrix(), (self.end_x()) as f32, (self.start_y()) as f32); vertices.set_color(self.color_end()); &mut *vertices };
    }

}

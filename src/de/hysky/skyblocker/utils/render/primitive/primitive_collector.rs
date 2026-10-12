//! Ported from de.hysky.skyblocker.utils.render.primitive.PrimitiveCollector by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::render::primitive::vanilla_renderer::VanillaRenderer;
use mcrust::net::minecraft::core::block_pos::BlockPos;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::world::level::block::state::block_state::BlockState;
use mcrust::net::minecraft::world::phys::aabb::AABB;
use mcrust::net::minecraft::world::phys::vec3::Vec3;

/// java: public interface PrimitiveCollector
pub trait PrimitiveCollector: compat::DynAny {
    fn as_any(&self) -> &dyn std::any::Any;
    /// java: <S> void submitVanilla(S state, VanillaRenderer<S> renderer);
    fn submit_vanilla<S: 'static>(&mut self, state: &S, renderer: &compat::Shared<dyn VanillaRenderer<S>>) where Self: Sized;

    /// java: void submitFilledBoxWithBeaconBeam(BlockPos pos, float[] colourComponents, float alpha, boolean throughWalls); (dispatch: the body for this class)
    fn submit_filled_box_with_beacon_beam(&self) -> fn(&compat::Shared<dyn PrimitiveCollector>, &BlockPos, &Vec<f32>, f32, bool);

    /// java: void submitFilledBoxWithBeaconBeam(AABB box, float[] colourComponents, float alpha, boolean throughWalls); (dispatch: the body for this class)
    fn submit_filled_box_with_beacon_beam_aabb_float_float_boolean(&self) -> fn(&compat::Shared<dyn PrimitiveCollector>, &AABB, &Vec<f32>, f32, bool);

    /// java: void submitFilledBox(BlockPos pos, float[] colourComponents, float alpha, boolean throughWalls); (dispatch: the body for this class)
    fn submit_filled_box(&self) -> fn(&compat::Shared<dyn PrimitiveCollector>, &BlockPos, &Vec<f32>, f32, bool);

    /// java: void submitFilledBox(Vec3 pos, Vec3 dimensions, float[] colourComponents, float alpha, boolean throughWalls); (dispatch: the body for this class)
    fn submit_filled_box_5(&self) -> fn(&compat::Shared<dyn PrimitiveCollector>, &Vec3, &Vec3, &Vec<f32>, f32, bool);

    /// java: void submitFilledBox(AABB box, float[] colourComponents, float alpha, boolean throughWalls); (dispatch: the body for this class)
    fn submit_filled_box_aabb_float_float_boolean(&self) -> fn(&compat::Shared<dyn PrimitiveCollector>, &AABB, &Vec<f32>, f32, bool);

    /// java: void submitOutlinedBox(BlockPos pos, float[] colourComponents, float lineWidth, boolean throughWalls); (dispatch: the body for this class)
    fn submit_outlined_box(&self) -> fn(&compat::Shared<dyn PrimitiveCollector>, &BlockPos, &Vec<f32>, f32, bool);

    /// java: void submitOutlinedBox(AABB box, float[] colourComponents, float lineWidth, boolean throughWalls); (dispatch: the body for this class)
    fn submit_outlined_box_aabb_float_float_boolean(&self) -> fn(&compat::Shared<dyn PrimitiveCollector>, &AABB, &Vec<f32>, f32, bool);

    /// java: void submitOutlinedBox(AABB box, float[] colourComponents, float alpha, float lineWidth, boolean throughWalls); (dispatch: the body for this class)
    fn submit_outlined_box_5(&self) -> fn(&compat::Shared<dyn PrimitiveCollector>, &AABB, &Vec<f32>, f32, f32, bool);

    /// java: void submitLinesFromPoints(Vec3[] points, float[] colourComponents, float alpha, float lineWidth, boolean throughWalls); (dispatch: the body for this class)
    fn submit_lines_from_points(&self) -> fn(&compat::Shared<dyn PrimitiveCollector>, &Vec<Vec3>, &Vec<f32>, f32, f32, bool);

    /// java: void submitLineFromCursor(Vec3 point, float[] colourComponents, float alpha, float lineWidth); (dispatch: the body for this class)
    fn submit_line_from_cursor(&self) -> fn(&compat::Shared<dyn PrimitiveCollector>, &Vec3, &Vec<f32>, f32, f32);

    /// java: void submitQuad(Vec3[] points, float[] colourComponents, float alpha, boolean throughWalls); (dispatch: the body for this class)
    fn submit_quad(&self) -> fn(&compat::Shared<dyn PrimitiveCollector>, &Vec<Vec3>, &Vec<f32>, f32, bool);

    /// java: void submitTexturedQuad(Vec3 pos, float width, float height, float textureWidth, float textureHeight, Vec3 renderOffset, Identifier texture, float[] shaderColour, float alpha, boolean throughWalls); (dispatch: the body for this class)
    fn submit_textured_quad(&self) -> fn(&compat::Shared<dyn PrimitiveCollector>, &Vec3, f32, f32, f32, f32, &Vec3, &Identifier, &Vec<f32>, f32, bool);

    /// java: void submitBlockHologram(BlockPos pos, BlockState state, float alpha); (dispatch: the body for this class)
    fn submit_block_hologram(&self) -> fn(&compat::Shared<dyn PrimitiveCollector>, &BlockPos, &Arc<BlockState>, f32);

    /// java: void submitText(Component text, Vec3 pos, boolean throughWalls); (dispatch: the body for this class)
    fn submit_text(&self) -> fn(&compat::Shared<dyn PrimitiveCollector>, &MutableComponent, &Vec3, bool);

    /// java: void submitText(Component text, Vec3 pos, float scale, boolean throughWalls); (dispatch: the body for this class)
    fn submit_text_4(&self) -> fn(&compat::Shared<dyn PrimitiveCollector>, &MutableComponent, &Vec3, f32, bool);

    /// java: void submitText(Component text, Vec3 pos, float scale, float yOffset, boolean throughWalls); (dispatch: the body for this class)
    fn submit_text_5(&self) -> fn(&compat::Shared<dyn PrimitiveCollector>, &MutableComponent, &Vec3, f32, f32, bool);

    /// java: void submitCylinder(Vec3 centre, float radius, float height, int segments, int colour); (dispatch: the body for this class)
    fn submit_cylinder(&self) -> fn(&compat::Shared<dyn PrimitiveCollector>, &Vec3, f32, f32, i32, i32);

    /// java: void submitSphere(Vec3 centre, float radius, int segments, int rings, int colour); (dispatch: the body for this class)
    fn submit_sphere(&self) -> fn(&compat::Shared<dyn PrimitiveCollector>, &Vec3, f32, i32, i32, i32);

    /// java: void submitFilledCircle(Vec3 centre, float radius, int segments, int colour); (dispatch: the body for this class)
    fn submit_filled_circle(&self) -> fn(&compat::Shared<dyn PrimitiveCollector>, &Vec3, f32, i32, i32);

    /// java: void submitOutlinedCircle(Vec3 centre, float radius, float thickness, int segments, int colour); (dispatch: the body for this class)
    fn submit_outlined_circle(&self) -> fn(&compat::Shared<dyn PrimitiveCollector>, &Vec3, f32, f32, i32, i32);

}

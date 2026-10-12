//! Ported from de.hysky.skyblocker.utils.render.gui.pip.GuiEquipmentRenderer by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::render::state::gui::gui_equipment_render_state::GuiEquipmentRenderState;
use mcrust::com::mojang::blaze3d::platform::lighting::Lighting;
use mcrust::com::mojang::blaze3d::vertex::pose_stack::PoseStack;
use mcrust::net::minecraft::client::gui::render::pip::picture_in_picture_renderer::PictureInPictureRenderer;
use mcrust::net::minecraft::client::renderer::submit_node_collector::SubmitNodeCollector;
use mcrust::com::mojang::math::axis::Axis as _;

/// java: public class GuiEquipmentRenderer<S> extends PictureInPictureRenderer<GuiEquipmentRenderState<S>>
pub struct GuiEquipmentRenderer<S> {
    /// java: extends net.minecraft.client.gui.render.pip.PictureInPictureRenderer
    pub base: PictureInPictureRenderer,
    /// the class's type parameter (`S`: the equipment model's render state)
    pub _state: std::marker::PhantomData<fn() -> S>,
}

impl<S: 'static> std::ops::Deref for GuiEquipmentRenderer<S> {
    type Target = PictureInPictureRenderer;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<S: 'static> std::ops::DerefMut for GuiEquipmentRenderer<S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl<S: Clone + 'static> GuiEquipmentRenderer<S> {
    /// java: private GuiEquipmentRenderer(PictureInPictureRendererRegistry.Context context)
    pub fn new(mut context: crate::unresolved::Context) -> Self {
        // TODO(fabric2seer): `new` takes 1 argument(s) in the port (`simple_name: &str`), Java passes 0
        //   java:
        let mut this: Self = todo!("constructor");
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        // TODO(fabric2seer): Fabric API `PictureInPictureRendererRegistry.register` has no SeerClient equivalent
        //   java: PictureInPictureRendererRegistry.register(GuiEquipmentRenderer::new);
    }

    /// java: @SuppressWarnings("unchecked") @Override public Class<GuiEquipmentRenderState<S>> getRenderStateClass()
    pub fn get_render_state_class(&self) -> std::any::TypeId {
        // java: return (Class<GuiEquipmentRenderState<S>>) (Object) GuiEquipmentRenderState.class; - the class object
        // is the port's `TypeId`
        return std::any::TypeId::of::<GuiEquipmentRenderState<S>>();
    }

    /// java: @Override protected void renderToTexture(GuiEquipmentRenderState<S> state, PoseStack matrices, SubmitNodeCollector submitNodeCollector)
    pub fn render_to_texture(&self, state: &GuiEquipmentRenderState<S>, matrices: &mut PoseStack, submit_node_collector: &mut dyn SubmitNodeCollector) {
        let client = compat::minecraft();
        matrices.push_pose();
        matrices.translate((0) as f32, (state.offset() / state.scale()), (0) as f32);
        matrices.mul_pose_quat(&mcrust::com::mojang::math::axis::XN.rotation_degrees((-5) as f32));
        matrices.mul_pose_quat(&mcrust::com::mojang::math::axis::YN.rotation_degrees(state.rotation()));
        // TODO(fabric2seer): field `ENTITY_IN_UI` of a value of unknown type
        //   java: client.gameRenderer.lighting().setupFor(Lighting.Entry.ENTITY_IN_UI);
        state.equipment_renderer().render_layers(state.layer_type(), &state.asset_key(), state.model(), state.state() as std::rc::Rc<dyn std::any::Any>, &state.stack(), matrices, submit_node_collector, mcrust::net::minecraft::util::light_coords_util::FULL_BRIGHT, 0);
        matrices.pop_pose();
    }

    /// java: @Override protected float getTranslateY(int height, int guiScale)
    pub fn get_translate_y(&self, mut height: i32, mut gui_scale: i32) -> f32 {
        return ((height.wrapping_div(2)) as f32) as f32;
    }

    /// java: @Override protected String getTextureLabel()
    pub fn get_texture_label(&self) -> String {
        return "skyblocker equipment renderer".to_string();
    }

}

// NOTE(fabric2seer): getRenderStateClass, getTextureLabel, getTranslateY, renderToTexture override(s) of PictureInPictureRenderer - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them

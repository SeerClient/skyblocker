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

impl<S: 'static> GuiEquipmentRenderer<S> {
    /// java: private GuiEquipmentRenderer(PictureInPictureRendererRegistry.Context context) {} - the base's texture state
    pub fn new() -> Self {
        Self {
            base: PictureInPictureRenderer::new("GuiEquipmentRenderer"),
            _state: std::marker::PhantomData,
        }
    }

    /// java: @Init public static void init()
    pub fn init() {
        // java: PictureInPictureRendererRegistry.register(GuiEquipmentRenderer::new);
        // The renderer draws the states of its class (`getRenderStateClass`); Java's class object is one for every `S`,
        // the port's `TypeId` is one per `S`, so it is registered for the state the mod builds (`TrimElementButton`'s
        // `AvatarRenderState`).
        compat::leftovers_register_picture_in_picture_renderer::<GuiEquipmentRenderState<mcrust::net::minecraft::client::renderer::entity::state::avatar_render_state::AvatarRenderState>>(
            "skyblocker equipment renderer",
            |height, gui_scale| GuiEquipmentRenderer::<mcrust::net::minecraft::client::renderer::entity::state::avatar_render_state::AvatarRenderState>::new().get_translate_y(height, gui_scale),
            |state, matrices, submit_node_collector, lighting| GuiEquipmentRenderer::<mcrust::net::minecraft::client::renderer::entity::state::avatar_render_state::AvatarRenderState>::new().render_to_texture(state, matrices, submit_node_collector, lighting),
        );
    }

    /// java: @SuppressWarnings("unchecked") @Override public Class<GuiEquipmentRenderState<S>> getRenderStateClass()
    pub fn get_render_state_class(&self) -> std::any::TypeId {
        // java: return (Class<GuiEquipmentRenderState<S>>) (Object) GuiEquipmentRenderState.class; - the class object
        // is the port's `TypeId`
        return std::any::TypeId::of::<GuiEquipmentRenderState<S>>();
    }

    /// java: @Override protected void renderToTexture(GuiEquipmentRenderState<S> state, PoseStack matrices, SubmitNodeCollector submitNodeCollector)
    ///
    /// (`lighting`: `client.gameRenderer.lighting()`, handed in as the game's own pip renderers take it)
    pub fn render_to_texture(&self, state: &GuiEquipmentRenderState<S>, matrices: &mut PoseStack, submit_node_collector: &mut dyn SubmitNodeCollector, lighting: &Lighting) {
        matrices.push_pose();
        matrices.translate((0) as f32, (state.offset() / state.scale()), (0) as f32);
        matrices.rotate(&mcrust::com::mojang::math::axis::XN.rotation_degrees((-5) as f32));
        matrices.rotate(&mcrust::com::mojang::math::axis::YN.rotation_degrees(state.rotation()));
        // java: client.gameRenderer.lighting().setupFor(Lighting.Entry.ENTITY_IN_UI);
        lighting.setup_for(mcrust::com::mojang::blaze3d::platform::lighting::Entry::EntityInUi);
        state.equipment_renderer().render_layers(state.layer_type(), &state.asset_key(), state.model(), state.state() as std::sync::Arc<dyn std::any::Any>, &state.stack(), matrices, submit_node_collector, mcrust::net::minecraft::util::light_coords_util::FULL_BRIGHT, 0);
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

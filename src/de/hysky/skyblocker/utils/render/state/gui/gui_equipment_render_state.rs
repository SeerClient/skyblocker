//! Ported from de.hysky.skyblocker.utils.render.state.gui.GuiEquipmentRenderState by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::client::gui::navigation::screen_rectangle::ScreenRectangle;
use mcrust::net::minecraft::client::model::model::Model;
use mcrust::net::minecraft::client::renderer::entity::layers::equipment_layer_renderer::EquipmentLayerRenderer;
use mcrust::net::minecraft::client::resources::model::equipment_client_info::LayerType;
use mcrust::net::minecraft::resources::resource_key::ResourceKey;
use mcrust::net::minecraft::world::item::equipment::equipment_asset::EquipmentAsset;

/// java: public record GuiEquipmentRenderState<S>( EquipmentLayerRenderer equipmentRenderer, EquipmentClientInfo.LayerType layerType, ResourceKey<EquipmentAsset> assetKey, Model<S> model, S state, ItemStack stack, int x0, int y0, int x1, int y1, flo
pub struct GuiEquipmentRenderState<S> {
    /// java: EquipmentLayerRenderer equipmentRenderer
    pub equipment_renderer: EquipmentLayerRenderer,
    /// java: EquipmentClientInfo.LayerType layerType
    pub layer_type: LayerType,
    /// java: ResourceKey<EquipmentAsset> assetKey
    pub asset_key: ResourceKey,
    /// java: Model<S> model (the port's model handle - what `EquipmentLayerRenderer.renderLayers` takes)
    pub model: std::rc::Rc<std::cell::RefCell<dyn mcrust::net::minecraft::client::renderer::feature::model_feature_renderer::AnimatedModel>>,
    /// java: S state (a reference to the render state, as Java's record holds it)
    pub state: std::rc::Rc<S>,
    /// java: ItemStack stack
    pub stack: mcrust::net::minecraft::world::item::item_stack::ItemStack,
    /// java: int x0
    pub x0: i32,
    /// java: int y0
    pub y0: i32,
    /// java: int x1
    pub x1: i32,
    /// java: int y1
    pub y1: i32,
    /// java: float rotation
    pub rotation: f32,
    /// java: float scale
    pub scale: f32,
    /// java: float offset
    pub offset: f32,
    /// java: ScreenRectangle scissorArea
    pub scissor_area: Option<ScreenRectangle>,
    /// java: ScreenRectangle bounds
    pub bounds: Option<ScreenRectangle>,
}

impl<S: Clone + 'static> GuiEquipmentRenderState<S> {
    /// java: public GuiEquipmentRenderState( EquipmentLayerRenderer equipmentRenderer, EquipmentClientInfo.LayerType layerType, ResourceKey<EquipmentAsset> assetKey, Model<S> model, S state, ItemStack stack, int x
    pub fn new(mut equipment_renderer: EquipmentLayerRenderer, mut layer_type: LayerType, mut asset_key: ResourceKey, mut model: std::rc::Rc<std::cell::RefCell<dyn mcrust::net::minecraft::client::renderer::feature::model_feature_renderer::AnimatedModel>>, mut state: std::rc::Rc<S>, mut stack: mcrust::net::minecraft::world::item::item_stack::ItemStack, mut x1: i32, mut y1: i32, mut x2: i32, mut y2: i32, mut rotation: f32, mut scale: f32, mut offset: f32, mut scissor_area: Option<ScreenRectangle>) -> Self {
        // java: this(equipmentRenderer, layerType, assetKey, model, state, stack, x1, y1, x2, y2, rotation, scale, offset, scissorArea, PictureInPictureRenderState.getBounds(x1, y1, x2, y2, scissorArea));
        let bounds = mcrust::net::minecraft::client::renderer::state::gui::pip::picture_in_picture_render_state::get_bounds(x1, y1, x2, y2, scissor_area.clone());
        Self::new_canonical(equipment_renderer, layer_type, asset_key, model, state, stack, x1, y1, x2, y2, rotation, scale, offset, scissor_area, bounds)
    }

    /// java: the canonical record constructor
    pub fn new_canonical(mut equipment_renderer: EquipmentLayerRenderer, mut layer_type: LayerType, mut asset_key: ResourceKey, mut model: std::rc::Rc<std::cell::RefCell<dyn mcrust::net::minecraft::client::renderer::feature::model_feature_renderer::AnimatedModel>>, mut state: std::rc::Rc<S>, mut stack: mcrust::net::minecraft::world::item::item_stack::ItemStack, mut x0: i32, mut y0: i32, mut x1: i32, mut y1: i32, mut rotation: f32, mut scale: f32, mut offset: f32, mut scissor_area: Option<ScreenRectangle>, mut bounds: Option<ScreenRectangle>) -> Self {
        Self { equipment_renderer, layer_type, asset_key, model, state, stack, x0, y0, x1, y1, rotation, scale, offset, scissor_area, bounds }
    }

    /// java: the record accessor `equipmentRenderer()`
    pub fn equipment_renderer(&self) -> &EquipmentLayerRenderer {
        &self.equipment_renderer
    }

    /// java: the record accessor `layerType()`
    pub fn layer_type(&self) -> LayerType {
        self.layer_type
    }

    /// java: the record accessor `assetKey()`
    pub fn asset_key(&self) -> ResourceKey {
        self.asset_key.clone()
    }

    /// java: the record accessor `model()`
    pub fn model(&self) -> std::rc::Rc<std::cell::RefCell<dyn mcrust::net::minecraft::client::renderer::feature::model_feature_renderer::AnimatedModel>> {
        self.model.clone()
    }

    /// java: the record accessor `state()`
    pub fn state(&self) -> std::rc::Rc<S> {
        self.state.clone()
    }

    /// java: the record accessor `stack()`
    pub fn stack(&self) -> mcrust::net::minecraft::world::item::item_stack::ItemStack {
        self.stack.clone()
    }

    /// java: the record accessor `x0()`
    pub fn x0(&self) -> i32 {
        self.x0
    }

    /// java: the record accessor `y0()`
    pub fn y0(&self) -> i32 {
        self.y0
    }

    /// java: the record accessor `x1()`
    pub fn x1(&self) -> i32 {
        self.x1
    }

    /// java: the record accessor `y1()`
    pub fn y1(&self) -> i32 {
        self.y1
    }

    /// java: the record accessor `rotation()`
    pub fn rotation(&self) -> f32 {
        self.rotation
    }

    /// java: the record accessor `scale()`
    pub fn scale(&self) -> f32 {
        self.scale
    }

    /// java: the record accessor `offset()`
    pub fn offset(&self) -> f32 {
        self.offset
    }

    /// java: the record accessor `scissorArea()`
    pub fn scissor_area(&self) -> Option<ScreenRectangle> {
        self.scissor_area.clone()
    }

    /// java: the record accessor `bounds()`
    pub fn bounds(&self) -> Option<ScreenRectangle> {
        self.bounds.clone()
    }

}

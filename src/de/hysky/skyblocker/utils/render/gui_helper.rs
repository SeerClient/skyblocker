//! Ported from de.hysky.skyblocker.utils.render.GuiHelper by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use std::rc::Rc;
use crate::de::hysky::skyblocker::compatibility::caxton_compatibility::CaxtonCompatibility;
use crate::de::hysky::skyblocker::compatibility::modern_ui_compatibility;
use crate::de::hysky::skyblocker::compatibility::modern_ui_compatibility::ModernUICompatibility;
use crate::de::hysky::skyblocker::utils::render::skyblocker_render_pipelines;
use crate::de::hysky::skyblocker::utils::render::state::gui::gui_custom_shape_render_state::GuiCustomShapeRenderState;
use crate::de::hysky::skyblocker::utils::render::state::gui::gui_equipment_render_state::GuiEquipmentRenderState;
use crate::de::hysky::skyblocker::utils::render::state::gui::gui_horizontal_gradient_render_state::GuiHorizontalGradientRenderState;
use crate::de::hysky::skyblocker::utils::render::state::gui::gui_outlined_text_render_state::GuiOutlinedTextRenderState;
use crate::de::hysky::skyblocker::utils::render::texture_pool::TexturePool;
use mcrust::com::mojang::blaze3d::gpu_format::GpuFormat;
use mcrust::com::mojang::blaze3d::pipeline::render_target::RenderTarget;
use mcrust::com::mojang::blaze3d::systems::render_system;
use mcrust::com::mojang::blaze3d::textures::filter_mode::FilterMode;
use mcrust::com::mojang::blaze3d::textures::gpu_sampler::GpuSampler;
use mcrust::com::mojang::blaze3d::textures::gpu_texture_view::GpuTextureView;
use mcrust::java::awt::color::Color;
use mcrust::java::util::uuid::Uuid;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::render::texture_setup::TextureSetup;
use mcrust::net::minecraft::client::minecraft;
use mcrust::net::minecraft::client::model::model::Model;
use mcrust::net::minecraft::client::renderer::entity::layers::equipment_layer_renderer::EquipmentLayerRenderer;
use mcrust::net::minecraft::client::resources::model::equipment_client_info::LayerType;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::resources::resource_key::ResourceKey;
use mcrust::net::minecraft::util::argb;
use mcrust::net::minecraft::util::formatted_char_sequence::FormattedCharSequence;
use mcrust::net::minecraft::util::formatted_char_sequence::Seq;
use mcrust::net::minecraft::world::entity::player::player_skin::PlayerSkin;
use mcrust::net::minecraft::world::item::component::resolvable_profile::ResolvableProfile;
use mcrust::net::minecraft::world::item::equipment::equipment_asset::EquipmentAsset;
use mcrust::org::joml::vector2f::Vector2f;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final TexturePool BLIT_TEXTURE_POOL = TexturePool.create("Blit Pool", 4, GpuTexture.USAGE_TEXTURE_BINDING | GpuTexture.USAGE_COPY_DST, GpuFormat.RGBA8_UNORM);
pub static BLIT_TEXTURE_POOL: compat::MainThread<TexturePool> = compat::MainThread::new(|| TexturePool::create("Blit Pool", 4, (mcrust::com::mojang::blaze3d::textures::gpu_texture::USAGE_TEXTURE_BINDING | mcrust::com::mojang::blaze3d::textures::gpu_texture::USAGE_COPY_DST), &GpuFormat::Rgba8Unorm));
/// java: private static int blitIndexForFrame = -1;
pub static BLIT_INDEX_FOR_FRAME: LazyLock<Mutex<i32>> = LazyLock::new(|| Mutex::new(-1));

/// java: public class GuiHelper
#[derive(Clone)]
pub struct GuiHelper {}

impl GuiHelper {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static void nineSliceColored(GuiGraphicsExtractor graphics, Identifier texture, int x, int y, int width, int height, int argb)
    pub fn nine_slice_colored(graphics: &GuiGraphicsExtractor, texture: &Identifier, mut x: i32, mut y: i32, mut width: i32, mut height: i32, mut argb: i32) {
        gui_graphics_extractor::blit_sprite_identifier_with_color(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), texture, x, y, width, height, argb);
    }

    /// java: public static void nineSliceColored(GuiGraphicsExtractor graphics, Identifier texture, int x, int y, int width, int height, Color color)
    pub fn nine_slice_colored_gui_graphics_extractor_identifier_int_int_int_int_color(graphics: &GuiGraphicsExtractor, texture: &Identifier, mut x: i32, mut y: i32, mut width: i32, mut height: i32, color: &Color) {
        // TODO(fabric2seer): `.getAlpha(..)` on a Color
        //   java: nineSliceColored(graphics, texture, x, y, width, height, ARGB.color(color.getAlpha(), color.getRed(), color.getGreen(), color.getBlue()));
    }

    /// java: public static void horizontalGradient(GuiGraphicsExtractor graphics, float startX, float startY, float endX, float endY, int colorStart, int colorEnd)
    pub fn horizontal_gradient(graphics: &GuiGraphicsExtractor, mut start_x: f32, mut start_y: f32, mut end_x: f32, mut end_y: f32, mut color_start: i32, mut color_end: i32) {
        // TODO(fabric2seer): `pose` (a posestack) used as a value
        //   java: graphics.guiRenderState.addGuiElement(new GuiHorizontalGradientRenderState(RenderPipelines.GUI, TextureSetup.noTexture(), new Matrix3x2f(graphics.pose()), (int) startX, (int) startY, (int) endX, (int) endY, colorStart, colorEnd, graphics.scissorStack.peek()));
    }

    /// java: public static void border(GuiGraphicsExtractor context, int x, int y, int width, int height, int color)
    pub fn border(context: &mut GuiGraphicsExtractor, mut x: i32, mut y: i32, mut width: i32, mut height: i32, mut color: i32) {
        context.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), x, y, x.wrapping_add(width), y.wrapping_add(1), color);
        context.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), x, (y.wrapping_add(height)).wrapping_sub(1), x.wrapping_add(width), y.wrapping_add(height), color);
        context.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), x, y.wrapping_add(1), x.wrapping_add(1), (y.wrapping_add(height)).wrapping_sub(1), color);
        context.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), (x.wrapping_add(width)).wrapping_sub(1), y.wrapping_add(1), x.wrapping_add(width), (y.wrapping_add(height)).wrapping_sub(1), color);
    }

    /// java: public static void customShape(GuiGraphicsExtractor graphics, List<Vector2f> vertices, int color)
    pub fn custom_shape(graphics: &GuiGraphicsExtractor, vertices: &Vec<Vector2f>, mut color: i32) {
        // TODO(fabric2seer): `pose` (a posestack) used as a value
        //   java: graphics.guiRenderState.addGuiElement(new GuiCustomShapeRenderState(RenderPipelines.GUI, TextureSetup.noTexture(), new Matrix3x2f(graphics.pose()), vertices, color, graphics.scissorStack.peek()));
    }

    /// java: public static void playerHead(GuiGraphicsExtractor graphics, int x, int y, int size, UUID uuid)
    pub fn player_head(graphics: &GuiGraphicsExtractor, mut x: i32, mut y: i32, mut size: i32, uuid: &Uuid) {
        // TODO(fabric2seer): `.map(..)` on a Result
        //   java: PlayerSkin texture = CLIENT.playerSkinRenderCache().lookup(ResolvableProfile.createUnresolved(uuid))
        //   java: 				.getNow(Optional.empty())
        //   java: 				.map(PlayerSkinRenderCache.RenderInfo::playerSkin)
        //   java: 				.orElseGet(() -> DefaultPlayerSkin.get(uuid));
        let mut texture: PlayerSkin = todo!("fabric2seer: the initialiser of `texture` (TODO above)");
        // TODO(fabric2seer): static `PlayerFaceExtractor.extractRenderState` has no Rust counterpart in the port
        // rust: PlayerFaceExtractor::extract_render_state(graphics.clone(), texture.clone(), x, y, size);
    }

    /// java: public static <S> void equipment(GuiGraphicsExtractor graphics, EquipmentLayerRenderer equipmentRenderer, EquipmentClientInfo.LayerType layerType, ResourceKey<EquipmentAsset> assetKey, Model<S> model,
    pub fn equipment<S: Clone + 'static>(graphics: &GuiGraphicsExtractor, equipment_renderer: &EquipmentLayerRenderer, layer_type: &LayerType, asset_key: &ResourceKey, model: &Model, state: &S, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, mut x1: i32, mut y1: i32, mut x2: i32, mut y2: i32, mut rotation: f32, mut scale: f32, mut offset: f32) {
        // TODO(fabric2seer): the mod keeps the game's `EquipmentLayerRenderer` (Java holds a reference to the live object); the port's `EquipmentLayerRenderer` is not a value to copy and the port lends the mod no handle to it
        // TODO(fabric2seer): `GuiGraphicsExtractor.scissorStack` has no Rust field in the port (searched net.minecraft.client.gui.GuiGraphicsExtractor and its supertypes)
        // rust: let mut render_state: GuiEquipmentRenderState<S> = GuiEquipmentRenderState::new(equipment_renderer.clone(), *layer_type, asset_key.clone(), model.clone(), state.clone(), stack.clone(), x1, y1, x2, y2, rotation, scale, offset, graphics.scissor_stack.peek());
        let mut render_state: GuiEquipmentRenderState<S> = todo!("fabric2seer: the initialiser of `renderState` (TODO above)");
        // TODO(fabric2seer): `GuiGraphicsExtractor.guiRenderState` has no Rust field in the port (searched net.minecraft.client.gui.GuiGraphicsExtractor and its supertypes)
        // rust: graphics.gui_render_state.add_pictures_in_picture_state(render_state.clone());
    }

    /// java: public static void outlinedText(GuiGraphicsExtractor graphics, Component text, int x, int y, int color, int outlineColor)
    pub fn outlined_text(graphics: &GuiGraphicsExtractor, text: &MutableComponent, mut x: i32, mut y: i32, mut color: i32, mut outline_color: i32) {
        let mut ordered_text: Seq = text.get_visual_order_text();
        GuiHelper::outlined_text_7(graphics, &ordered_text, &(if *modern_ui_compatibility::MODERNUI_ENABLED { component::literal(&text.get_string()).get_visual_order_text() } else { ordered_text.clone() }), x, y, color, outline_color);
    }

    /// java: public static void outlinedText(GuiGraphicsExtractor graphics, FormattedCharSequence text, int x, int y, int color, int outlineColor)
    pub fn outlined_text_gui_graphics_extractor_formatted_char_sequence_int_int_int_int(graphics: &GuiGraphicsExtractor, text: &Seq, mut x: i32, mut y: i32, mut color: i32, mut outline_color: i32) {
        GuiHelper::outlined_text_7(graphics, text, text, x, y, color, outline_color);
    }

    /// java: private static void outlinedText(GuiGraphicsExtractor graphics, FormattedCharSequence text, FormattedCharSequence outlineText, int x, int y, int color, int outlineColor)
    pub fn outlined_text_7(graphics: &GuiGraphicsExtractor, text: &Seq, outline_text: &Seq, mut x: i32, mut y: i32, mut color: i32, mut outline_color: i32) {
        if CaxtonCompatibility::draw_outlined_text(graphics, text, (x) as f32, (y) as f32, color, outline_color) {
            return;
        }
        if ModernUICompatibility::extract_outlined_text(graphics, text, outline_text, x, y, color, outline_color) {
            return;
        }
        // TODO(fabric2seer): `pose` (a posestack) used as a value
        //   java: GuiOutlinedTextRenderState renderState = new GuiOutlinedTextRenderState(CLIENT.font, text, new Matrix3x2f(graphics.pose()), x, y, color, outlineColor, false, false, graphics.scissorStack.peek());
        let mut render_state: GuiOutlinedTextRenderState = todo!("fabric2seer: the initialiser of `renderState` (TODO above)");
        // TODO(fabric2seer): `GuiGraphicsExtractor.guiRenderState` has no Rust field in the port (searched net.minecraft.client.gui.GuiGraphicsExtractor and its supertypes)
        // rust: graphics.gui_render_state.add_text(render_state.clone());
    }

    /// java: public static void blurredRectangle(GuiGraphicsExtractor graphics, int x0, int y0, int x1, int y1, int radius)
    pub fn blurred_rectangle(graphics: &GuiGraphicsExtractor, mut x0: i32, mut y0: i32, mut x1: i32, mut y1: i32, mut radius: i32) {
        if compat::get_state(&BLIT_INDEX_FOR_FRAME) == -1 {
            let main_render_target = compat::minecraft().game_renderer.main_render_target();
            let mut required_width: i32 = main_render_target.target.width;
            let mut required_height: i32 = main_render_target.target.height;
            compat::set_state(&BLIT_INDEX_FOR_FRAME, BLIT_TEXTURE_POOL.get_mut().get_next_available_index(required_width, required_height));
        }
        let mut blit_texture_view: Rc<dyn GpuTextureView> = BLIT_TEXTURE_POOL.get_mut().get_texture_view(compat::get_state(&BLIT_INDEX_FOR_FRAME));
        // TODO(fabric2seer): `with_sampler_cache` takes 1 argument(s) in the port (`f: impl FnOnce(&SamplerCache) -> R`), Java passes 0
        // rust: let mut sampler: Box<dyn GpuSampler> = render_system::with_sampler_cache().get_clamp_to_edge(FilterMode::Linear);
        let mut sampler: Box<dyn GpuSampler> = todo!("fabric2seer: the initialiser of `sampler` (TODO above)");
        let mut vertex_colour: i32 = argb::color_rgb(radius, 255, 255);
        // TODO(fabric2seer): `GuiGraphicsExtractorInvoker.innerFill` has no Rust method in the port (looked for `inner_fill` in GuiGraphicsExtractorInvoker and JavaSrc)
        // rust: graphics.inner_fill(skyblocker_render_pipelines::BLURRED_RECTANGLE.get_mut(), &TextureSetup::single_texture(blit_texture_view.clone(), sampler.clone()), x0, y0, x1, y1, vertex_colour, None);
    }

    /// java: public static void updateScreenBlitTexture()
    pub fn update_screen_blit_texture() {
        if compat::get_state(&BLIT_INDEX_FOR_FRAME) != -1 {
            let main_render_target = compat::minecraft().game_renderer.main_render_target();
            let mut required_width: i32 = main_render_target.target.width;
            let mut required_height: i32 = main_render_target.target.height;
            let mut blit_texture_view: Rc<dyn GpuTextureView> = BLIT_TEXTURE_POOL.get_mut().get_texture_view(compat::get_state(&BLIT_INDEX_FOR_FRAME));
            render_system::get_device().create_command_encoder().copy_texture_to_texture(main_render_target.target.get_color_texture().expect("NullPointerException: color texture").as_ref(), blit_texture_view.texture().as_ref(), 0, 0, 0, 0, 0, required_width, required_height);
            compat::set_state(&BLIT_INDEX_FOR_FRAME, -1);
        }
    }

    /// java: public static boolean pointIsInArea(double x, double y, double x1, double y1, double x2, double y2)
    pub fn point_is_in_area(mut x: f64, mut y: f64, mut x1: f64, mut y1: f64, mut x2: f64, mut y2: f64) -> bool {
        return (x >= x1 && x <= x2 && y >= y1 && y <= y2);
    }

    /// java: public static boolean hasShiftDown()
    pub fn has_shift_down() -> bool {
        return minecraft::has_shift_down();
    }

    /// java: public static void close()
    pub fn close() {
        BLIT_TEXTURE_POOL.get_mut().close();
    }

}

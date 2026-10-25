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
        // java: nineSliceColored(graphics, texture, x, y, width, height, ARGB.color(color.getAlpha(), color.getRed(), color.getGreen(), color.getBlue()));
        GuiHelper::nine_slice_colored(graphics, texture, x, y, width, height, argb::color_argb(color.get_alpha(), color.get_red(), color.get_green(), color.get_blue()));
    }

    /// java: public static void horizontalGradient(GuiGraphicsExtractor graphics, float startX, float startY, float endX, float endY, int colorStart, int colorEnd)
    pub fn horizontal_gradient(graphics: &GuiGraphicsExtractor, mut start_x: f32, mut start_y: f32, mut end_x: f32, mut end_y: f32, mut color_start: i32, mut color_end: i32) {
        // java: graphics.guiRenderState.addGuiElement(new GuiHorizontalGradientRenderState(RenderPipelines.GUI, TextureSetup.noTexture(), new Matrix3x2f(graphics.pose()), (int) startX, (int) startY, (int) endX, (int) endY, colorStart, colorEnd, graphics.scissorStack.peek()));
        let _ = graphics;
        let state = GuiHorizontalGradientRenderState::new(mcrust::net::minecraft::client::renderer::render_pipelines::gui(), mcrust::net::minecraft::client::gui::render::texture_setup::TextureSetup::no_texture(), gui_graphics_extractor::current_pose(), start_x as i32, start_y as i32, end_x as i32, end_y as i32, color_start, color_end, gui_graphics_extractor::peek_scissor());
        gui_graphics_extractor::with_gui_render_state(|__s| __s.add_gui_element(Box::new(state)));
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
        // java: graphics.guiRenderState.addGuiElement(new GuiCustomShapeRenderState(RenderPipelines.GUI, TextureSetup.noTexture(), new Matrix3x2f(graphics.pose()), vertices, color, graphics.scissorStack.peek()));
        let _ = graphics;
        let state = GuiCustomShapeRenderState::new(mcrust::net::minecraft::client::renderer::render_pipelines::gui(), mcrust::net::minecraft::client::gui::render::texture_setup::TextureSetup::no_texture(), gui_graphics_extractor::current_pose(), vertices.clone(), color, gui_graphics_extractor::peek_scissor());
        gui_graphics_extractor::with_gui_render_state(|__s| __s.add_gui_element(Box::new(state)));
    }

    /// java: public static void playerHead(GuiGraphicsExtractor graphics, int x, int y, int size, UUID uuid)
    pub fn player_head(graphics: &GuiGraphicsExtractor, mut x: i32, mut y: i32, mut size: i32, uuid: &Uuid) {
        // java: PlayerSkin texture = CLIENT.playerSkinRenderCache().lookup(ResolvableProfile.createUnresolved(uuid))
        // java: 				.getNow(Optional.empty())
        // java: 				.map(PlayerSkinRenderCache.RenderInfo::playerSkin)
        // java: 				.orElseGet(() -> DefaultPlayerSkin.get(uuid));
        // `Minecraft.playerSkinRenderCache()` is `game_renderer.player_skin_render_cache` (set once
        // the game has loaded). The port's future answers `Option<RenderInfo>`, so `getNow(empty)`
        // is `get_now(Some(None))`; an exceptionally completed lookup (Java's CompletionException)
        // reads as absent. `createUnresolved(UUID)` cannot fail in Java.
        let mut texture: PlayerSkin = compat::minecraft()
            .game_renderer
            .player_skin_render_cache
            .as_ref()
            .zip(ResolvableProfile::create_unresolved_id(*uuid).ok())
            .and_then(|(cache, profile)| cache.lookup(&profile).get_now(Some(None)).ok().flatten().flatten())
            .map(|info| info.player_skin().clone())
            .unwrap_or_else(|| mcrust::net::minecraft::client::resources::default_player_skin::get_skin(uuid));
        // java: PlayerFaceExtractor.extractRenderState(graphics, texture, x, y, size);
        // = extractRenderState(graphics, skin.body().texturePath(), x, y, size, true, false, -1); the
        // port's takes the texture the id names (batch B pattern 25).
        {
            use mcrust::net::minecraft::core::client_asset::Texture as _;
            let client = compat::minecraft();
            if let Some(face) = client.game_renderer.texture_manager().get_texture_or_request(texture.body().texture_path()) {
                mcrust::net::minecraft::client::gui::components::player_face_extractor::extract_render_state(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*client, face, x, y, size, true, false, -1);
            }
        }
    }

    /// java: public static <S> void equipment(GuiGraphicsExtractor graphics, EquipmentLayerRenderer equipmentRenderer, EquipmentClientInfo.LayerType layerType, ResourceKey<EquipmentAsset> assetKey, Model<S> model,
    pub fn equipment<S: Clone + 'static>(graphics: &GuiGraphicsExtractor, equipment_renderer: &EquipmentLayerRenderer, layer_type: &LayerType, asset_key: &ResourceKey, model: &Model, state: &S, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, mut x1: i32, mut y1: i32, mut x2: i32, mut y2: i32, mut rotation: f32, mut scale: f32, mut offset: f32) {
        // java: GuiEquipmentRenderState<S> renderState = new GuiEquipmentRenderState<>(equipmentRenderer, layerType, assetKey, model, state, stack, x1, y1, x2, y2, rotation, scale, offset, graphics.scissorStack.peek());
        // java: graphics.guiRenderState.addPicturesInPictureState(renderState);
        // BLOCKED (cross-package, `skyblock/item/custom/screen/trim_element_button.rs`): the state and its renderer are ported (`GuiEquipmentRenderState` implements
        // `PictureInPictureRenderState`; `GuiEquipmentRenderer` registers through `compat::leftovers_register_picture_in_picture_renderer`),
        // but this signature hands a `Model` VALUE where `EquipmentLayerRenderer.renderLayers` (and so the state) takes the
        // shared `Rc<RefCell<dyn AnimatedModel>>` the model is posed through, and the only caller (`TrimElementButton.Pattern`)
        // never reaches here: its `equipmentModelData` is baked from `Minecraft.getEntityModels()`, which the port does not
        // lend a mod (GAPS_batchE). The statement lands with that model handle.
        let _ = (graphics, equipment_renderer, layer_type, asset_key, model, state, stack, x1, y1, x2, y2, rotation, scale, offset);
    }

    /// java: public static void outlinedText(GuiGraphicsExtractor graphics, Component text, int x, int y, int color, int outlineColor)
    pub fn outlined_text(graphics: &GuiGraphicsExtractor, text: &MutableComponent, mut x: i32, mut y: i32, mut color: i32, mut outline_color: i32) {
        let mut ordered_text: Seq = text.get_visual_order_text();
        GuiHelper::outlined_text_7(graphics, &ordered_text, &(if *modern_ui_compatibility::MODERNUI_ENABLED { component::literal(&text.get_string()).get_visual_order_text() } else { ordered_text.clone() }), x, y, color, outline_color);
    }

    /// PERF (port-only, same draw): `outlinedText(graphics, text, x, y, color, outlineColor)` above, answering the
    /// resolved record it added when a caller may keep it: neither compat arm can draw (no Caxton, no ModernUI -
    /// then `outlinedText`'s outline sequence IS the text's) and the resolve was cacheable. A caller that draws
    /// the same text at the same place in the same colours, under the same
    /// `gui_graphics_extractor::gui_text_record_epoch`, hands the record back through `add_kept_outlined_text`
    /// instead of building the outline (8 glyph passes) and the text again.
    pub fn outlined_text_kept(graphics: &GuiGraphicsExtractor, text: &MutableComponent, x: i32, y: i32, color: i32, outline_color: i32) -> Option<Rc<mcrust::net::minecraft::client::renderer::state::gui::gui_text_render_state::PreparedText>> {
        if !GuiHelper::outlined_text_keepable() {
            GuiHelper::outlined_text(graphics, text, x, y, color, outline_color);
            return None;
        }
        let ordered_text: Seq = text.get_visual_order_text();
        // `outlinedText(graphics, ordered, ordered, ..)`'s last statements: both compat arms answer false here.
        let render_state = GuiOutlinedTextRenderState::new(&compat::font(), ordered_text, gui_graphics_extractor::current_pose(), x, y, color, outline_color, false, false, gui_graphics_extractor::peek_scissor());
        let kept = render_state.cacheable.then(|| render_state.base.ensure_prepared_shared().clone());
        gui_graphics_extractor::with_gui_render_state(|__s| __s.add_text(render_state.base));
        kept
    }

    /// PERF (port-only): whether `outlined_text_kept` can answer a record at all - neither Caxton nor ModernUI is
    /// loaded (each draws the outline its own way).
    pub fn outlined_text_keepable() -> bool {
        !*crate::de::hysky::skyblocker::compatibility::caxton_compatibility::CAXTON_ENABLED && !*modern_ui_compatibility::MODERNUI_ENABLED
    }

    /// PERF (port-only, same draw): the `addText` `outlined_text_kept` made, with a record it answered - the live
    /// pose and scissor, as the constructor takes them (`GuiTextRenderState::new_shared` computes the bounds as
    /// `new` does).
    pub fn add_kept_outlined_text(prepared: &Rc<mcrust::net::minecraft::client::renderer::state::gui::gui_text_render_state::PreparedText>) {
        let state = mcrust::net::minecraft::client::renderer::state::gui::gui_text_render_state::GuiTextRenderState::new_shared(prepared.clone(), gui_graphics_extractor::current_pose(), gui_graphics_extractor::peek_scissor());
        gui_graphics_extractor::with_gui_render_state(|__s| __s.add_text(state));
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
        // java: GuiOutlinedTextRenderState renderState = new GuiOutlinedTextRenderState(CLIENT.font, text, new Matrix3x2f(graphics.pose()), x, y, color, outlineColor, false, false, graphics.scissorStack.peek());
        // java: graphics.guiRenderState.addText(renderState);
        let render_state = GuiOutlinedTextRenderState::new(&compat::font(), text.clone(), gui_graphics_extractor::current_pose(), x, y, color, outline_color, false, false, gui_graphics_extractor::peek_scissor());
        // (the game's `addText` takes its own state: the subclass hands it the base it prepared - its
        // `ensurePrepared()` record)
        gui_graphics_extractor::with_gui_render_state(|__s| __s.add_text(render_state.base));
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
        // java: GpuSampler sampler = RenderSystem.getSamplerCache().getClampToEdge(FilterMode.LINEAR);
        let mut sampler: Rc<dyn GpuSampler> = render_system::with_sampler_cache(|cache| cache.get_clamp_to_edge(FilterMode::Linear));
        let mut vertex_colour: i32 = argb::color_rgb(radius, 255, 255);
        // java: ((GuiGraphicsExtractorInvoker) graphics).invokeInnerFill(SkyblockerRenderPipelines.BLURRED_RECTANGLE, TextureSetup.singleTexture(blitTextureView, sampler), x0, y0, x1, y1, vertexColour, null);
        // `GuiGraphicsExtractor.innerFill:217-232` is private in the port; the invoker runs its body,
        // which is public API: `addGuiElement(new ColoredRectangleRenderState(pipeline, textureSetup,
        // new Matrix3x2f(pose), x0, y0, x1, y1, color1, color2 != null ? color2 : color1, scissorStack.peek()))`.
        let _ = graphics;
        let pipeline = skyblocker_render_pipelines::BLURRED_RECTANGLE.get().clone();
        let texture_setup = TextureSetup::single_texture(blit_texture_view.clone(), sampler.clone());
        let element = mcrust::net::minecraft::client::renderer::state::gui::colored_rectangle_render_state::ColoredRectangleRenderState::new(pipeline, texture_setup, gui_graphics_extractor::current_pose(), x0, y0, x1, y1, vertex_colour, vertex_colour, gui_graphics_extractor::peek_scissor());
        gui_graphics_extractor::with_gui_render_state(|__s| __s.add_gui_element(Box::new(element)));
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

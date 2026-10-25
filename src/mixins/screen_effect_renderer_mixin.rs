//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.ScreenEffectRendererMixin by fabric2seer.
//! @Mixin(net.minecraft.client.renderer.ScreenEffectRenderer)

#[allow(unused_imports)]
use crate::prelude::*;

/// @Unique
fn configure_flame_height_internal(y: f32) -> f32 {
    return (y - (0.5f32 - (((crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().ui_and_visuals.flame_overlay.flame_height) as f32) / 200.0f32)));
}

/// @ModifyArg configureFlameHeight1
///   java: @ModifyArg(method = "buildFireQuad", index = 4, at = @At(value = "INVOKE", target = "Lnet/minecraft/client/renderer/ScreenEffectRenderer;buildSpriteQuad(Lcom/mojang/blaze3d/vertex/VertexConsumer;Lorg/joml/Matrix4f;Lnet/minecraft/client/renderer/texture/TextureAtlasSprite;FFFFFI)V"))
/// target: net.minecraft.client.renderer.ScreenEffectRenderer#buildSpriteQuad  [exists]
/// within: net.minecraft.client.renderer.ScreenEffectRenderer#buildFireQuad
/// Rust: net/minecraft/client/renderer/screen_effect_renderer.rs:650 fn build_sprite_quad(builder: &mut dyn VertexConsumer, pose: &Matrix4f, sprite: &TextureAtlasSprite, x0: f32, y0: f32, x1: f32, y1: f32, z: f32, color: i32)
fn configure_flame_height1(v: &f32) -> f32 {
    let mut y = *v;
    return configure_flame_height_internal(y);
}

/// @ModifyArg configureFlameHeight2
///   java: @ModifyArg(method = "buildFireQuad", index = 6, at = @At(value = "INVOKE", target = "Lnet/minecraft/client/renderer/ScreenEffectRenderer;buildSpriteQuad(Lcom/mojang/blaze3d/vertex/VertexConsumer;Lorg/joml/Matrix4f;Lnet/minecraft/client/renderer/texture/TextureAtlasSprite;FFFFFI)V"))
/// target: net.minecraft.client.renderer.ScreenEffectRenderer#buildSpriteQuad  [exists]
/// within: net.minecraft.client.renderer.ScreenEffectRenderer#buildFireQuad
/// Rust: net/minecraft/client/renderer/screen_effect_renderer.rs:650 fn build_sprite_quad(builder: &mut dyn VertexConsumer, pose: &Matrix4f, sprite: &TextureAtlasSprite, x0: f32, y0: f32, x1: f32, y1: f32, z: f32, color: i32)
fn configure_flame_height2(v: &f32) -> f32 {
    let mut y = *v;
    return configure_flame_height_internal(y);
}

/// @ModifyArg configureFlameOpacity
///   java: @ModifyArg(method = "buildFireQuad", index = 8, at = @At(value = "INVOKE", target = "Lnet/minecraft/client/renderer/ScreenEffectRenderer;buildSpriteQuad(Lcom/mojang/blaze3d/vertex/VertexConsumer;Lorg/joml/Matrix4f;Lnet/minecraft/client/renderer/texture/TextureAtlasSprite;FFFFFI)V"))
/// target: net.minecraft.client.renderer.ScreenEffectRenderer#buildSpriteQuad  [exists]
/// within: net.minecraft.client.renderer.ScreenEffectRenderer#buildFireQuad
/// Rust: net/minecraft/client/renderer/screen_effect_renderer.rs:650 fn build_sprite_quad(builder: &mut dyn VertexConsumer, pose: &Matrix4f, sprite: &TextureAtlasSprite, x0: f32, y0: f32, x1: f32, y1: f32, z: f32, color: i32)
fn configure_flame_opacity(v: &i32) -> i32 {
    let mut colour = *v;
    let mut opacity: f32 = mcrust::net::minecraft::util::argb::alpha_float(colour);
    let mut modified_opacity: f32 = (opacity - (0.8f32 - (((crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().ui_and_visuals.flame_overlay.flame_opacity) as f32) / 125.0f32)));
    return mcrust::net::minecraft::util::argb::color_alpha_f32_rgb(modified_opacity, colour);
}

/// `@Shadow @Nullable private ItemStack itemActivationItem` of the `renderItemActivationAnimation` frame the
/// `submit` call below is made in. A call-site injector is handed the CALL's receiver and arguments, so the frame's
/// receiver field is read at that method's HEAD (`ScreenEffectRenderer::get_item_activation_item`) and kept for the
/// one call it contains (render thread only).
static ITEM_ACTIVATION_ITEM: compat::MainThread<Option<ItemStack>> = compat::MainThread::new(|| None);

fn skyblocker_capture_item_activation_item(ci: &mut CallbackInfo<'_>) {
    let _ = ci;
    // 26.3: the item moved off `ScreenEffectRenderer` into `LocalPlayer.itemActivation()`.
    let item = mcrust::mcrust::modding::game::with_player(|player| player.item_activation().item().map(|item| item.duplicate())).flatten();
    *ITEM_ACTIVATION_ITEM.get_mut() = item;
}

/// @Inject skyblocker$renderSpecialEffectsItemName
///   java: @Inject(method = "renderItemActivationAnimation", at = @At(value = "INVOKE", target = "Lnet/minecraft/client/renderer/item/ItemStackRenderState;submit(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/SubmitNodeCollector;III)V"))
/// target: net.minecraft.client.renderer.item.ItemStackRenderState#submit  [exists]
/// within: net.minecraft.client.renderer.ScreenEffectRenderer#renderItemActivationAnimation
/// Rust: net/minecraft/client/renderer/item/item_stack_render_state.rs fn submit(&self, pose_stack: &mut PoseStack, submit_node_collector: &mut dyn SubmitNodeCollector, light_coords: i32, overlay_coords: i32, outline_color: i32)
/// The method's `poseStack` and `submitNodeCollector` are exactly the call's arguments 0 and 1, as transformed at the
/// call; `itemActivationItem` is the frame's (see `ITEM_ACTIVATION_ITEM`).
fn skyblocker_render_special_effects_item_name(ci: &mut CallbackInfo<'_>) {
    use mcrust::net::minecraft::client::renderer::ordered_submit_node_collector::OrderedSubmitNodeCollector as _;
    // Render item names for all special effect items
    let Some(item_activation_item) = ITEM_ACTIVATION_ITEM.get().as_ref().map(|item| item.duplicate()) else { return; };
    //   java: if (SpecialEffects.ITEM.get() != itemActivationItem) return;
    if !crate::de::hysky::skyblocker::skyblock::special::special_effects::ITEM.get().is_same_instance(&item_activation_item) {
        return;
    }
    //   java: Component name = itemActivationItem.getHoverName();
    let name = item_activation_item.get_hover_name();
    //   java: int x = -minecraft.font.width(name) / 2;
    let x: i32 = -compat::font().width_of(&name) / 2;
    let Some(pose_stack) = compat::hook_arg_mut::<mcrust::com::mojang::blaze3d::vertex::pose_stack::PoseStack>(ci, 0) else { return; };
    let Some(submit_node_collector) = compat::hook_arg_mut::<dyn mcrust::net::minecraft::client::renderer::submit_node_collector::SubmitNodeCollector>(ci, 1) else { return; };
    //   java: poseStack.pushPose();
    pose_stack.push_pose();
    //   java: poseStack.scale(-0.03125f, -0.03125f, 0.03125f);
    pose_stack.scale(-0.03125f32, -0.03125f32, 0.03125f32);
    //   java: submitNodeCollector.submitText(poseStack, x, 16f, name.getVisualOrderText(), false, Font.DisplayMode.POLYGON_OFFSET, 0xF000F0, 0xFFFFFFFF, 0, 0);
    submit_node_collector.submit_text(
        pose_stack,
        x as f32,
        16.0f32,
        &name,
        false,
        mcrust::net::minecraft::client::gui::font::DisplayMode::PolygonOffset,
        0xF000F0,
        0xFFFFFFFFu32 as i32,
        0,
        0,
    );
    //   java: poseStack.popPose();
    pose_stack.pop_pose();
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    report(hook::call_site("net.minecraft.client.renderer.ScreenEffectRenderer#buildFireQuad", "net.minecraft.client.renderer.ScreenEffectRenderer#buildSpriteQuad").priority(1000).modify_arg::<f32>(4, configure_flame_height1));
    report(hook::call_site("net.minecraft.client.renderer.ScreenEffectRenderer#buildFireQuad", "net.minecraft.client.renderer.ScreenEffectRenderer#buildSpriteQuad").priority(1000).modify_arg::<f32>(6, configure_flame_height2));
    report(hook::call_site("net.minecraft.client.renderer.ScreenEffectRenderer#buildFireQuad", "net.minecraft.client.renderer.ScreenEffectRenderer#buildSpriteQuad").priority(1000).modify_arg::<i32>(8, configure_flame_opacity));
    report(hook::register_injector("net.minecraft.client.renderer.ScreenEffectRenderer#renderItemActivationAnimation", InjectOptions::at(At::Head).kind(InjectorKind::Inject).priority(1000), Arc::new(skyblocker_capture_item_activation_item)));
    report(hook::call_site("net.minecraft.client.renderer.ScreenEffectRenderer#renderItemActivationAnimation", "net.minecraft.client.renderer.item.ItemStackRenderState#submit").priority(1000).inject(Shift::Before, skyblocker_render_special_effects_item_name));
}

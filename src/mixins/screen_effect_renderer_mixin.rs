//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.ScreenEffectRendererMixin by fabric2seer.
//! @Mixin(net.minecraft.client.renderer.ScreenEffectRenderer)

#[allow(unused_imports)]
use crate::prelude::*;

/// @Unique
fn configure_flame_height_internal(this: &mut mcrust::net::minecraft::client::renderer::screen_effect_renderer::ScreenEffectRenderer, y: f32) -> f32 {
    return (y - (0.5f32 - (((crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().ui_and_visuals.flame_overlay.flame_height) as f32) / 200.0f32)));
}

/// @ModifyArg configureFlameHeight1
///   java: @ModifyArg(method = "buildFireQuad", index = 4, at = @At(value = "INVOKE", target = "Lnet/minecraft/client/renderer/ScreenEffectRenderer;buildSpriteQuad(Lcom/mojang/blaze3d/vertex/VertexConsumer;Lorg/joml/Matrix4f;Lnet/minecraft/client/renderer/texture/TextureAtlasSprite;FFFFFI)V"))
/// target: net.minecraft.client.renderer.ScreenEffectRenderer#buildSpriteQuad  [exists]
/// within: net.minecraft.client.renderer.ScreenEffectRenderer#buildFireQuad
/// Rust: net/minecraft/client/renderer/screen_effect_renderer.rs:650 fn build_sprite_quad(builder: &mut dyn VertexConsumer, pose: &Matrix4f, sprite: &TextureAtlasSprite, x0: f32, y0: f32, x1: f32, y1: f32, z: f32, color: i32)
fn configure_flame_height1(v: &f32) -> f32 {
    let mut y = v.clone();
    return configure_flame_height_internal(y);
}

/// @ModifyArg configureFlameHeight2
///   java: @ModifyArg(method = "buildFireQuad", index = 6, at = @At(value = "INVOKE", target = "Lnet/minecraft/client/renderer/ScreenEffectRenderer;buildSpriteQuad(Lcom/mojang/blaze3d/vertex/VertexConsumer;Lorg/joml/Matrix4f;Lnet/minecraft/client/renderer/texture/TextureAtlasSprite;FFFFFI)V"))
/// target: net.minecraft.client.renderer.ScreenEffectRenderer#buildSpriteQuad  [exists]
/// within: net.minecraft.client.renderer.ScreenEffectRenderer#buildFireQuad
/// Rust: net/minecraft/client/renderer/screen_effect_renderer.rs:650 fn build_sprite_quad(builder: &mut dyn VertexConsumer, pose: &Matrix4f, sprite: &TextureAtlasSprite, x0: f32, y0: f32, x1: f32, y1: f32, z: f32, color: i32)
fn configure_flame_height2(v: &f32) -> f32 {
    let mut y = v.clone();
    return configure_flame_height_internal(y);
}

/// @ModifyArg configureFlameOpacity
///   java: @ModifyArg(method = "buildFireQuad", index = 8, at = @At(value = "INVOKE", target = "Lnet/minecraft/client/renderer/ScreenEffectRenderer;buildSpriteQuad(Lcom/mojang/blaze3d/vertex/VertexConsumer;Lorg/joml/Matrix4f;Lnet/minecraft/client/renderer/texture/TextureAtlasSprite;FFFFFI)V"))
/// target: net.minecraft.client.renderer.ScreenEffectRenderer#buildSpriteQuad  [exists]
/// within: net.minecraft.client.renderer.ScreenEffectRenderer#buildFireQuad
/// Rust: net/minecraft/client/renderer/screen_effect_renderer.rs:650 fn build_sprite_quad(builder: &mut dyn VertexConsumer, pose: &Matrix4f, sprite: &TextureAtlasSprite, x0: f32, y0: f32, x1: f32, y1: f32, z: f32, color: i32)
fn configure_flame_opacity(v: &i32) -> i32 {
    let mut colour = v.clone();
    let mut opacity: f32 = mcrust::net::minecraft::util::argb::alpha_float(colour);
    let mut modified_opacity: f32 = (opacity - (0.8f32 - (((crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().ui_and_visuals.flame_overlay.flame_opacity) as f32) / 125.0f32)));
    return mcrust::net::minecraft::util::argb::color_alpha_f32_rgb(modified_opacity, colour);
}

/// @Inject skyblocker$renderSpecialEffectsItemName
///   java: @Inject(method = "renderItemActivationAnimation", at = @At(value = "INVOKE", target = "Lnet/minecraft/client/renderer/item/ItemStackRenderState;submit(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/SubmitNodeCollector;III)V"))
/// target: net.minecraft.client.renderer.item.ItemStackRenderState#submit  [exists]
/// within: net.minecraft.client.renderer.ScreenEffectRenderer#renderItemActivationAnimation
/// Rust: net/minecraft/client/renderer/item/item_stack_render_state.rs:1173 fn submit(&self, pose_stack: &mut crate::com::mojang::blaze3d::vertex::pose_stack::PoseStack, submit_node_collector: &mut dyn crate::net::minecraft::client::renderer::submit_node_collector::SubmitNodeCollector, light_coords: i32, overlay_coords: i32, outline_color: i32)
/// note: @Inject at INVOKE: a call-site injector (`hook::call_site`) - `ci` is the CALL's (its arguments; cancel() skips that call, not the rest of the method)
fn skyblocker_render_special_effects_item_name(ci: &mut CallbackInfo<'_>) {
    // TODO(fabric2seer): the condition: `.get(..)` on a WeakReference
    //   java: SpecialEffects.ITEM.get() != itemActivationItem
    if todo!("fabric2seer: the condition (TODO above)") {
        return;
    }
    // TODO(fabric2seer): `this` (the hooked object) is not available in this callback
    //   java: Component name = itemActivationItem.getHoverName();
    let mut name: mcrust::net::minecraft::network::chat::mutable_component::MutableComponent = todo!("fabric2seer: the initialiser of `name` (TODO above)");
    // TODO(fabric2seer): `this` (the hooked object) is not available in this callback
    //   java: int x = -minecraft.font.width(name) / 2;
    let mut x: i32 = todo!("fabric2seer: the initialiser of `x` (TODO above)");
    // TODO(fabric2seer): `poseStack`: this argument is not visible to the injector (see the notes above the handler)
    //   java: poseStack.pushPose();
    // TODO(fabric2seer): `poseStack`: this argument is not visible to the injector (see the notes above the handler)
    //   java: poseStack.scale(-0.03125f, -0.03125f, 0.03125f);
    // TODO(fabric2seer): `submitNodeCollector`: this argument is not visible to the injector (see the notes above the handler)
    //   java: submitNodeCollector.submitText(poseStack, x, 16f, name.getVisualOrderText(), false, Font.DisplayMode.POLYGON_OFFSET, 0xF000F0, 0xFFFFFFFF, 0, 0);
    // TODO(fabric2seer): `poseStack`: this argument is not visible to the injector (see the notes above the handler)
    //   java: poseStack.popPose();
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    report(hook::call_site("net.minecraft.client.renderer.ScreenEffectRenderer#buildFireQuad", "net.minecraft.client.renderer.ScreenEffectRenderer#buildSpriteQuad").priority(1000).modify_arg::<f32>(4, configure_flame_height1));
    report(hook::call_site("net.minecraft.client.renderer.ScreenEffectRenderer#buildFireQuad", "net.minecraft.client.renderer.ScreenEffectRenderer#buildSpriteQuad").priority(1000).modify_arg::<f32>(6, configure_flame_height2));
    report(hook::call_site("net.minecraft.client.renderer.ScreenEffectRenderer#buildFireQuad", "net.minecraft.client.renderer.ScreenEffectRenderer#buildSpriteQuad").priority(1000).modify_arg::<i32>(8, configure_flame_opacity));
    report(hook::call_site("net.minecraft.client.renderer.ScreenEffectRenderer#renderItemActivationAnimation", "net.minecraft.client.renderer.item.ItemStackRenderState#submit").priority(1000).inject(Shift::Before, skyblocker_render_special_effects_item_name));
}

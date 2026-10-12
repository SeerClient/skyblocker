//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.EntityRendererMixin by fabric2seer.
//! @Mixin(net.minecraft.client.renderer.entity.EntityRenderer)

#[allow(unused_imports)]
use crate::prelude::*;

/// @Inject skyblocker$mobBoundingBox
///   java: @Inject(method = "extractRenderState", at = @At(value = "TAIL"))
/// target: net.minecraft.client.renderer.entity.EntityRenderer#extractRenderState  [exists]
/// Rust: net/minecraft/client/renderer/entity/entity_renderer.rs:658 fn extract_entity_render_state(entity: &dyn crate::net::minecraft::world::entity::entity::Entity, state: &mut crate::net::minecraft::client::renderer::entity::state::entity_render_state::EntityRenderState, partial_ticks: f32, get_brightness: &dyn Fn( crate::net::minecraft::world::level::light_layer:
fn skyblocker_mob_bounding_box(ci: &mut CallbackInfo<'_>) {
    // TODO(fabric2seer): `entity` is Rust argument 0: `&dyn crate::net::minecraft::world::entity::entity::Entity` - generic or borrowed with a lifetime; read it with `unsafe { ci.arg_unchecked::<..>(0) }`
    let partial_ticks: f32 = ci.arg::<f32>(2).copied().unwrap_or_default();
    if crate::de::hysky::skyblocker::skyblock::entity::mob_bounding_boxes::MobBoundingBoxes::should_draw_mob_bounding_box(&entity) {
        crate::de::hysky::skyblocker::skyblock::entity::mob_bounding_boxes::MobBoundingBoxes::submit_box2_be_rendered(&crate::de::hysky::skyblocker::utils::boxes::Boxes::lerp_entity_bounding_box(&entity, partial_ticks), &crate::de::hysky::skyblocker::skyblock::entity::mob_bounding_boxes::MobBoundingBoxes::get_box_color(&entity));
        return;
    }
    if crate::de::hysky::skyblocker::skyblock::slayers::slayer_manager::SlayerManager::should_glow(&entity, crate::de::hysky::skyblocker::config::configs::slayers_config::HighlightSlayerEntities::Hitbox) {
        // TODO(fabric2seer): `.getRGB(..)` on a Color
        //   java: float[] color = ColorUtils.getFloatComponents(SkyblockerConfigManager.get().slayers.highlightColor.getRGB());
        let mut color: Vec<f32> = todo!("fabric2seer: the initialiser of `color` (TODO above)");
        crate::de::hysky::skyblocker::skyblock::entity::mob_bounding_boxes::MobBoundingBoxes::submit_box2_be_rendered(&crate::de::hysky::skyblocker::utils::boxes::Boxes::lerp_entity_bounding_box(&entity, partial_ticks), &color);
    }
}

/// @Inject skyblocker$movePlayerRenderPos
///   java: @Inject(method = "extractRenderState", at = @At(value = "TAIL"))
/// target: net.minecraft.client.renderer.entity.EntityRenderer#extractRenderState  [exists]
/// Rust: net/minecraft/client/renderer/entity/entity_renderer.rs:658 fn extract_entity_render_state(entity: &dyn crate::net::minecraft::world::entity::entity::Entity, state: &mut crate::net::minecraft::client::renderer::entity::state::entity_render_state::EntityRenderState, partial_ticks: f32, get_brightness: &dyn Fn( crate::net::minecraft::world::level::light_layer:
fn skyblocker_move_player_render_pos(ci: &mut CallbackInfo<'_>) {
    // TODO(fabric2seer): `entity` is Rust argument 0: `&dyn crate::net::minecraft::world::entity::entity::Entity` - generic or borrowed with a lifetime; read it with `unsafe { ci.arg_unchecked::<..>(0) }`
    let Some(state) = compat::detach(ci.arg_mut::<mcrust::net::minecraft::client::renderer::entity::state::entity_render_state::EntityRenderState>(1)) else { return; };
    let client = compat::minecraft();
    // TODO(fabric2seer): the condition: `Minecraft.player` held as a value: the port lends the player only inside `game::with_player(|player| ..)` (a mod must never lock it)
    //   java: entity == client.player && !client.options.getCameraType().isFirstPerson()
    if todo!("fabric2seer: the condition (TODO above)") {
        let mut pos: mcrust::net::minecraft::world::phys::vec3::Vec3;
        if crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().ui_and_visuals.smooth_aote.predictive {
            pos = crate::de::hysky::skyblocker::skyblock::teleport::predictive_smooth_aote::PredictiveSmoothAOTE::get_interpolated_player_pos().unwrap();
        } else {
            pos = crate::de::hysky::skyblocker::skyblock::teleport::responsive_smooth_aote::ResponsiveSmoothAOTE::get_interpolated_player_pos(nil).unwrap();
        }
        if true /* java: pos != null: never null here */ {
            state.x = pos.x;
            state.y = pos.y;
            state.z = pos.z;
        }
    }
}

/// @ModifyReturnValue skyblocker$applyCustomName
///   java: @ModifyReturnValue(method = "getNameTag", at = @At("RETURN"))
/// target: net.minecraft.client.renderer.entity.EntityRenderer#getNameTag  [exists]
/// Rust: net/minecraft/client/renderer/entity/entity_renderer.rs:1218 fn get_name_tag(entity: &E, level: Option<&crate::net::minecraft::client::multiplayer::client_level::ClientLevel>)
fn skyblocker_apply_custom_name(ci: &mut CallbackInfo<'_>) {
    // TODO(fabric2seer): read the return value typed: ci.return_value::<T>() (Component)
    // TODO(fabric2seer): unknown name `entity`
    //   java: Component customName = entity.skyblocker$getCustomName();
    let mut custom_name: mcrust::net::minecraft::network::chat::mutable_component::MutableComponent = todo!("fabric2seer: the initialiser of `customName` (TODO above)");
    ci.set_return_value((if true /* java: custom_name != null: never null here */ { custom_name.clone() } else { original.clone() }));
    return;
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    report(hook::register_injector("net.minecraft.client.renderer.entity.EntityRenderer#extractRenderState", InjectOptions::at(At::Tail).kind(InjectorKind::Inject).priority(1000), Arc::new(skyblocker_mob_bounding_box)));
    report(hook::register_injector("net.minecraft.client.renderer.entity.EntityRenderer#extractRenderState", InjectOptions::at(At::Tail).kind(InjectorKind::Inject).priority(1000), Arc::new(skyblocker_move_player_render_pos)));
    report(hook::register_injector("net.minecraft.client.renderer.entity.EntityRenderer#getNameTag", InjectOptions::at(At::Return).kind(InjectorKind::ModifyReturnValue).priority(1000), Arc::new(skyblocker_apply_custom_name)));
}

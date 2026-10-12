//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.EntityRendererMixin by fabric2seer.
//! @Mixin(net.minecraft.client.renderer.entity.EntityRenderer)

#[allow(unused_imports)]
use crate::prelude::*;

use crate::de::hysky::skyblocker::injected::skyblocker_entity::SkyblockerEntity;
use mcrust::net::minecraft::world::entity::entity::Entity;

/// @Inject skyblocker$mobBoundingBox
///   java: @Inject(method = "extractRenderState", at = @At(value = "TAIL"))
/// target: net.minecraft.client.renderer.entity.EntityRenderer#extractRenderState  [exists]
/// Rust: net/minecraft/client/renderer/entity/entity_renderer.rs:658 fn extract_entity_render_state(entity: &dyn crate::net::minecraft::world::entity::entity::Entity, state: &mut crate::net::minecraft::client::renderer::entity::state::entity_render_state::EntityRenderState, partial_ticks: f32, get_brightness: &dyn Fn( crate::net::minecraft::world::level::light_layer:
fn skyblocker_mob_bounding_box(ci: &mut CallbackInfo<'_>) {
    // `@Local(name = "entity")` / `@Local(name = "partialTicks")` are the port's arguments 0 and 2.
    let Some(entity) = compat::hook_arg::<dyn Entity>(ci, 0) else { return; };
    let partial_ticks: f32 = ci.arg::<f32>(2).copied().unwrap_or_default();
    if crate::de::hysky::skyblocker::skyblock::entity::mob_bounding_boxes::MobBoundingBoxes::should_draw_mob_bounding_box(entity) {
        crate::de::hysky::skyblocker::skyblock::entity::mob_bounding_boxes::MobBoundingBoxes::submit_box2_be_rendered(&crate::de::hysky::skyblocker::utils::boxes::Boxes::lerp_entity_bounding_box(entity, partial_ticks), &crate::de::hysky::skyblocker::skyblock::entity::mob_bounding_boxes::MobBoundingBoxes::get_box_color(entity));
        return;
    }
    if crate::de::hysky::skyblocker::skyblock::slayers::slayer_manager::SlayerManager::should_glow(entity, crate::de::hysky::skyblocker::config::configs::slayers_config::HighlightSlayerEntities::Hitbox) {
        let color: Vec<f32> = crate::de::hysky::skyblocker::utils::color_utils::ColorUtils::get_float_components(crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().slayers.highlight_color.get_rgb());
        crate::de::hysky::skyblocker::skyblock::entity::mob_bounding_boxes::MobBoundingBoxes::submit_box2_be_rendered(&crate::de::hysky::skyblocker::utils::boxes::Boxes::lerp_entity_bounding_box(entity, partial_ticks), &color);
    }
}

/// @Inject skyblocker$movePlayerRenderPos
///   java: @Inject(method = "extractRenderState", at = @At(value = "TAIL"))
/// target: net.minecraft.client.renderer.entity.EntityRenderer#extractRenderState  [exists]
/// Rust: net/minecraft/client/renderer/entity/entity_renderer.rs:658 fn extract_entity_render_state(entity: &dyn crate::net::minecraft::world::entity::entity::Entity, state: &mut crate::net::minecraft::client::renderer::entity::state::entity_render_state::EntityRenderState, partial_ticks: f32, get_brightness: &dyn Fn( crate::net::minecraft::world::level::light_layer:
fn skyblocker_move_player_render_pos(ci: &mut CallbackInfo<'_>) {
    let Some(entity) = compat::hook_arg::<dyn Entity>(ci, 0) else { return; };
    let partial_ticks: f32 = ci.arg::<f32>(2).copied().unwrap_or_default();
    let Some(state) = compat::detach(ci.arg_mut::<mcrust::net::minecraft::client::renderer::entity::state::entity_render_state::EntityRenderState>(1)) else { return; };
    // `entity == client.player`: the one `LocalPlayer` in the level IS `Minecraft.player` (the game holds him while his
    // state is extracted, so he is recognised by class rather than through `with_player`).
    let is_client_player = Entity::as_any(entity).is_some_and(|any| any.is::<mcrust::net::minecraft::client::player::local_player::LocalPlayer>());
    if is_client_player && !mcrust::net::minecraft::client::options::get_camera_type().is_first_person() {
        let pos: Option<mcrust::net::minecraft::world::phys::vec3::Vec3>;
        if crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().ui_and_visuals.smooth_aote.predictive {
            pos = crate::de::hysky::skyblocker::skyblock::teleport::predictive_smooth_aote::PredictiveSmoothAOTE::get_interpolated_player_pos();
        } else {
            pos = crate::de::hysky::skyblocker::skyblock::teleport::responsive_smooth_aote::ResponsiveSmoothAOTE::get_interpolated_player_pos(partial_ticks);
        }
        if let Some(pos) = pos {
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
    // `T entity` is the port's GENERIC argument 0, handed over as its concrete class: the custom name is an
    // attachment every entity can carry, and the classes the mod writes it on are armor stands (and any entity a
    // caller hands in as `dyn Entity`).
    let custom_name: Option<mcrust::net::minecraft::network::chat::mutable_component::MutableComponent> =
        if let Some(entity) = compat::hook_arg::<mcrust::net::minecraft::world::entity::decoration::armor_stand::ArmorStand>(ci, 0) {
            entity.skyblocker_get_custom_name()
        } else if let Some(entity) = compat::hook_arg::<dyn Entity>(ci, 0) {
            entity.skyblocker_get_custom_name()
        } else {
            None
        };
    let Some(original) = ci.take_return_value::<Option<std::rc::Rc<mcrust::net::minecraft::network::chat::formatted_text::FormattedText>>>() else { return; };
    ci.set_return_value(match custom_name {
        Some(custom_name) => Some(std::rc::Rc::new(mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(custom_name))),
        None => original,
    });
    return;
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    report(hook::register_injector("net.minecraft.client.renderer.entity.EntityRenderer#extractRenderState", InjectOptions::at(At::Tail).kind(InjectorKind::Inject).priority(1000), Arc::new(skyblocker_mob_bounding_box)));
    report(hook::register_injector("net.minecraft.client.renderer.entity.EntityRenderer#extractRenderState", InjectOptions::at(At::Tail).kind(InjectorKind::Inject).priority(1000), Arc::new(skyblocker_move_player_render_pos)));
    report(hook::register_injector("net.minecraft.client.renderer.entity.EntityRenderer#getNameTag", InjectOptions::at(At::Return).kind(InjectorKind::ModifyReturnValue).priority(1000), Arc::new(skyblocker_apply_custom_name)));
}

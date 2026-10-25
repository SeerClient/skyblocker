//! Ported from the mixin net.azureaaron.renderchest.mixins.EntityRendererMixin (Render Chest 1.0.3).
//! @Mixin(net.minecraft.client.renderer.entity.EntityRenderer)
//!
//! ```java
//! @Inject(method = "extractRenderState", at = @At("TAIL"))
//! private void renderChest$customGlow(CallbackInfo ci, @Local(name = "entity") Entity entity,
//!         @Local(name = "state") EntityRenderState state) {
//!     int customGlowColour = CustomGlowCallback.EVENT.invoker().applyGlowColour(entity, state);
//!     boolean removeGlow = customGlowColour == GlowConstants.REMOVE_GLOW;
//!     if (customGlowColour != GlowConstants.NO_GLOW) {
//!         // If the entity has the vanilla glow then change it's colour otherwise apply the custom glow
//!         if (entity.isCurrentlyGlowing()) {
//!             state.outlineColor = removeGlow ? EntityRenderState.NO_OUTLINE : ARGB.opaque(customGlowColour);
//!         } else if (!removeGlow) {
//!             state.setData(RenderStateKeys.ENTITY_CUSTOM_GLOW_COLOUR, ARGB.opaque(customGlowColour));
//!         }
//!     }
//! }
//! ```
//!
//! The vanilla-glow arm is whole. The custom-glow arm stores the colour as render-state data that the rest of the
//! library (its custom outline phase in `SubmitNodeCollection`, the depth-tested `custom_outline` pipelines, the
//! glow-depth texture copy, the `PreparedRenderType` depth swap) draws from; none of that has a place in the game
//! (GAPS_S.md #9): the arm evaluates its condition and has no store to write.

use crate::net::azureaaron::renderchest::api::custom_glow_callback;
use crate::net::azureaaron::renderchest::api::glow_constants::GlowConstants;
use crate::net::azureaaron::renderchest::r#impl::render_state_keys;
use mcrust::mcrust::modding::hook::{self, At, CallbackInfo};
use mcrust::net::minecraft::client::renderer::entity::state::entity_render_state::{self, EntityRenderState};
use mcrust::net::minecraft::util::argb;
use mcrust::net::minecraft::world::entity::entity::Entity;

/// @Inject renderChest$customGlow
fn render_chest_custom_glow(ci: &mut CallbackInfo<'_>) {
    // @Local(name = "entity") Entity entity - the hooked method's first parameter (`&dyn Entity`)
    let Some(entity) = ci.arg::<dyn Entity>(0).map(|e| e as *const dyn Entity) else {
        return;
    };
    // SAFETY: the argument is alive for the whole hooked call and is only read; the pointer only separates this
    // borrow of `ci` from the `&mut` one below.
    let entity: &dyn Entity = unsafe { &*entity };
    // @Local(name = "state") EntityRenderState state - the second parameter (`&mut EntityRenderState`)
    let Some(state) = ci.arg_mut::<EntityRenderState>(1) else {
        return;
    };

    let custom_glow_colour = custom_glow_callback::invoker()(entity, state);
    let remove_glow = custom_glow_colour == GlowConstants::REMOVE_GLOW;
    if custom_glow_colour != GlowConstants::NO_GLOW {
        // If the entity has the vanilla glow then change it's colour otherwise apply the custom glow
        if entity.base().is_currently_glowing() {
            state.outline_color = if remove_glow { entity_render_state::NO_OUTLINE } else { argb::opaque(custom_glow_colour) };
        } else if !remove_glow {
            // java: state.setData(RenderStateKeys.ENTITY_CUSTOM_GLOW_COLOUR, ARGB.opaque(customGlowColour)); - no store
            // (see the header)
            let _ = (render_state_keys::ENTITY_CUSTOM_GLOW_COLOUR.debug_name, argb::opaque(custom_glow_colour));
        }
    }
}

/// The hooked method `render_chest_custom_glow` is injected into.
const EXTRACT_RENDER_STATE: &str = "net.minecraft.client.renderer.entity.EntityRenderer#extractRenderState";

/// OPTIMIZED BUT NOT FULLY VANILLA (perf round 1, P01-hooks-core; port-only, same result): the injector's pre-filter
/// (`InjectOptions::when`). `render_chest_custom_glow` returns at its FIRST statement whenever `ci.arg::<dyn Entity>(0)`
/// is `None` - and it is `None` on every call while the hooked method's `entity` parameter is not typed-readable
/// (`#[mcrust_hook]` marks a `&dyn Trait` parameter lifetime-carrying, so `CallbackInfo::arg` refuses it), which
/// `hook::target_arg_readable` reads off the hooked method's static description. So this answers false exactly when
/// the callback would have been a no-op, and true (the callback decides, as before) until that is known. A `Some`
/// answer never changes and is cached; the callback itself is unchanged. With it every injector on the target is
/// guarded, so a call none of them wants never leaves the hooked method (first person, outside dungeons).
fn entity_arg_may_be_readable() -> bool {
    use std::sync::atomic::{AtomicU8, Ordering};
    /// 0 = not known yet, 1 = readable, 2 = never readable.
    static ANSWER: AtomicU8 = AtomicU8::new(0);
    match ANSWER.load(Ordering::Relaxed) {
        1 => true,
        2 => false,
        _ => match hook::target_arg_readable(EXTRACT_RENDER_STATE, 0) {
            Some(readable) => {
                ANSWER.store(if readable { 1 } else { 2 }, Ordering::Relaxed);
                readable
            }
            None => true,
        },
    }
}

/// Mixins apply before the game runs: `RenderChest::register_mixins` (Skyblocker's pre-launch) calls this.
pub fn register() {
    // OPTIMIZED BUT NOT FULLY VANILLA (perf round 1, P01-hooks-core): registered with the exact guard above.
    // `MCRUST_VANILLA_RENDERCHEST_GUARD=1` (read once, here) registers it unguarded, as it was:
    //     hook::inject("net.minecraft.client.renderer.entity.EntityRenderer#extractRenderState", At::Tail, render_chest_custom_glow)
    let vanilla = std::env::var("MCRUST_VANILLA_RENDERCHEST_GUARD").is_ok_and(|v| v == "1");
    let registered = if vanilla {
        hook::inject(EXTRACT_RENDER_STATE, At::Tail, render_chest_custom_glow)
    } else {
        hook::inject_with(
            EXTRACT_RENDER_STATE,
            hook::InjectOptions::at(At::Tail).when(entity_arg_may_be_readable),
            render_chest_custom_glow,
        )
    };
    if let Err(e) = registered {
        mcrust::mcrust::modding::log::warn_once(
            "render-chest-entity-renderer",
            &format!("[Render Chest] EntityRendererMixin: {e:?}"),
        );
    }
}

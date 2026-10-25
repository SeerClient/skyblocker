//! Ported from net.azureaaron.renderchest.api.CustomGlowCallback (Render Chest 1.0.3).
//!
//! Callback that is fired for each entity extracted for level rendering to optionally modify its glowing effect.
//!
//! The exact behaviour of the callback will vary as follows:
//! - **When an entity has no vanilla glow:** This will apply Render Chest's custom glow effect which does not show
//!   through walls.
//! - **When an entity has vanilla glow:** This will change the colour of the vanilla glow (and thus show through
//!   walls).
//!
//! The colour returned by a listener must be in the ARGB format. Any colour with a non-opaque alpha value is
//! considered a reserved value by the implementation and may enable special behaviour.
//!
//! Special Return Values:
//! - `GlowConstants::NO_GLOW`: return this as the colour if the entity should not have any glow applied.
//! - `GlowConstants::REMOVE_GLOW`: return this as the colour if the entity should be stripped of any glow it may have.
//!
//! Aside from that it is up to users as to whether they want to perform any additional caching of whether an entity
//! should glow or not.

use crate::net::azureaaron::renderchest::api::glow_constants::GlowConstants;
use mcrust::mcrust::modding::events::Event;
use mcrust::net::minecraft::client::renderer::entity::state::entity_render_state::EntityRenderState;
use mcrust::net::minecraft::world::entity::entity::Entity;

/// java: @FunctionalInterface public interface CustomGlowCallback { int applyGlowColour(Entity entity, EntityRenderState state); }
pub type CustomGlowCallback = dyn Fn(&dyn Entity, &EntityRenderState) -> i32 + Send + Sync;

/// java: Event<CustomGlowCallback> EVENT = EventFactory.createArrayBacked(CustomGlowCallback.class, callbacks -> ..)
pub static EVENT: Event<CustomGlowCallback> = Event::new();

/// `EVENT.invoker()` - the factory's lambda over the listeners: the first colour that is not `NO_GLOW`, else `NO_GLOW`
pub fn invoker() -> impl Fn(&dyn Entity, &EntityRenderState) -> i32 {
    // PERF (port-only, same result): the event's cached listener slice, not a fresh `Vec` (this runs once per
    // extracted entity, per frame).
    let callbacks = EVENT.snapshot();
    move |entity: &dyn Entity, state: &EntityRenderState| {
        for callback in callbacks.iter() {
            let colour = callback(entity, state);
            if colour != GlowConstants::NO_GLOW {
                return colour;
            }
        }
        GlowConstants::NO_GLOW
    }
}

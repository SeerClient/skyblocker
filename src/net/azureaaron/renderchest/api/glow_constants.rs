//! Ported from net.azureaaron.renderchest.api.GlowConstants (Render Chest 1.0.3).
//!
//! Contains constants used with `CustomGlowCallback`.

/// java: public final class GlowConstants
pub struct GlowConstants;

impl GlowConstants {
    /// java: public static final int NO_GLOW = EntityRenderState.NO_OUTLINE;
    ///
    /// Indicates that the handler does not want to apply any glow to the given entity.
    pub const NO_GLOW: i32 = mcrust::net::minecraft::client::renderer::entity::state::entity_render_state::NO_OUTLINE;

    /// java: public static final int REMOVE_GLOW = Integer.MAX_VALUE;
    ///
    /// Indicates that the handler wants to strip the entity of any glow it may have (custom or vanilla).
    pub const REMOVE_GLOW: i32 = i32::MAX;
}

//! Ported from de.hysky.skyblocker.utils.render.primitive.VanillaRenderer by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::client::renderer::state::level::level_render_state::LevelRenderState;
use mcrust::net::minecraft::client::renderer::submit_node_collector::SubmitNodeCollector;

/// java: public interface VanillaRenderer<S>
pub trait VanillaRenderer<S> {
    fn as_any(&self) -> &dyn std::any::Any;
    /// java: void submitVanilla(S state, LevelRenderState levelState, SubmitNodeCollector submitNodeCollector);
    fn submit_vanilla(&self, state: &S, level_state: &LevelRenderState, submit_node_collector: &dyn SubmitNodeCollector);

}

//! Ported from de.hysky.skyblocker.utils.render.state.EmptyRenderState by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: public static final EmptyRenderState INSTANCE = new EmptyRenderState();
pub static INSTANCE: LazyLock<EmptyRenderState> = LazyLock::new(|| EmptyRenderState::new());

/// java: public final class EmptyRenderState
#[derive(Clone)]
pub struct EmptyRenderState {}

impl EmptyRenderState {
    /// java: private EmptyRenderState()
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

}

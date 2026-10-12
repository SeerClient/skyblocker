//! Ported from de.hysky.skyblocker.utils.render.state.SkyblockerRenderState by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: public class SkyblockerRenderState
#[derive(Clone)]
pub struct SkyblockerRenderState {
    /// java: public boolean enabled;
    pub enabled: bool,
}

impl SkyblockerRenderState {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            enabled: false,
        };
        this
    }

}

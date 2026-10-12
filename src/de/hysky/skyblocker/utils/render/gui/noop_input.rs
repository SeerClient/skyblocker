//! Ported from de.hysky.skyblocker.utils.render.gui.NoopInput by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::client::input::input_with_modifiers::InputWithModifiers;

/// java: public static final InputWithModifiers INSTANCE = new NoopInput();
pub static INSTANCE: compat::MainThread<Box<dyn InputWithModifiers>> = compat::MainThread::new(|| NoopInput::new());

/// java: public record NoopInput() implements InputWithModifiers
#[derive(Clone, Debug, PartialEq)]
pub struct NoopInput {}

impl NoopInput {
    /// java: the canonical record constructor
    pub fn new() -> Self {
        Self {  }
    }

    /// java: @Override public int input()
    pub fn input(&self) -> i32 {
        return 0;
    }

    /// java: @Override public int modifiers()
    pub fn modifiers(&self) -> i32 {
        return 0;
    }

}

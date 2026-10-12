//! Ported from de.hysky.skyblocker.utils.command.CommandUtils by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::com::mojang::brigadier::command::Command;

/// java: public static final Command<FabricClientCommandSource> noOp = _ -> -1;
pub static NO_OP: compat::MainThread<Command> = compat::MainThread::new(|| Arc::new(move |_| -1));

/// java: public final class CommandUtils
#[derive(Clone)]
pub struct CommandUtils {}

impl CommandUtils {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

}

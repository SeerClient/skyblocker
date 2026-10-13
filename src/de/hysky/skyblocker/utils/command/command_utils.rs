//! Ported from de.hysky.skyblocker.utils.command.CommandUtils by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::com::mojang::brigadier::command::Command;
use mcrust::com::mojang::brigadier::context::command_context::CommandContext;
use mcrust::mcrust::modding::events::client_command_registration_callback::FabricClientCommandSource;

/// java: public static final Command<FabricClientCommandSource> noOp = _ -> -1;
/// (a `LazyLock`: the command is read on the threads that build command nodes - the executor's - as well as the
/// render thread)
pub static NO_OP: LazyLock<Command<FabricClientCommandSource>> = LazyLock::new(|| Arc::new(move |_: &CommandContext<FabricClientCommandSource>| Ok(-1)));

/// `CommandUtils.noOp` - the one command (`NO_OP`)
pub fn no_op() -> Command<FabricClientCommandSource> {
    NO_OP.clone()
}

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

//! Ported from de.hysky.skyblocker.skyblock.dungeon.terminal.TerminalSolver by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;

/// java: public sealed interface TerminalSolver permits ColorTerminal, LightsOnTerminal, OrderTerminal, SameColorTerminal, StartsWithTerminal
pub trait TerminalSolver: compat::DynAny {
    fn as_any(&self) -> &dyn std::any::Any;
    /// java: default boolean shouldBlockIncorrectClicks() (dispatch: the body for this class)
    fn should_block_incorrect_clicks(&self) -> fn(&compat::Shared<dyn TerminalSolver>) -> bool {
        <dyn TerminalSolver>::should_block_incorrect_clicks_body
    }

}

impl dyn TerminalSolver {
    /// java: default boolean shouldBlockIncorrectClicks()
    pub fn should_block_incorrect_clicks_body(__this: &compat::Shared<dyn TerminalSolver>) -> bool {
        return SkyblockerConfigManager::get().dungeons.terminals.block_incorrect_clicks;
    }

}

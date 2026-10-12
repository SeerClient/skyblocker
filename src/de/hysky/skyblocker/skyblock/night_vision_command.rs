//! Ported from de.hysky.skyblocker.skyblock.NightVisionCommand by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use mcrust::com::mojang::brigadier::command_dispatcher::CommandDispatcher;
use mcrust::com::mojang::brigadier::context::command_context::CommandContext;
use mcrust::net::minecraft::commands::command_build_context::CommandBuildContext;

/// java: public class NightVisionCommand
#[derive(Clone)]
pub struct NightVisionCommand {}

impl NightVisionCommand {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        client_command_registration_callback::EVENT.register(Arc::new(
            |dispatcher: &mut client_command_registration_callback::ClientCommandDispatcher| {
                // TODO(fabric2seer): a command tree this tool cannot read (arguments, redirects, requires) - SeerClient commands receive the words after the name in `ctx.args`
                //   java: dispatcher.register(literal(SkyblockerMod.NAMESPACE)
                //   java: 				.then(literal("nightVision")
                //   java: 						.then(argument("strength", IntegerArgumentType.integer(0, 100))
                //   java: 								.executes(context -> NightVisionCommand.writeStrength(context, IntegerArgumentType.getInteger(context, "strength"))))
                //   java: 						.then(literal("off").executes(context -> NightVisionCommand.writeStrength(context, 0)))
                //   java: 						.then(literal("full").executes(context -> NightVisionCommand.writeStrength(context, 100)))));
            },
        ));
    }

    /// java: private static void register(CommandDispatcher<FabricClientCommandSource> dispatcher, CommandBuildContext access)
    pub fn register(dispatcher: &CommandDispatcher<crate::unresolved::FabricClientCommandSource>, access: &CommandBuildContext) {
        // TODO(fabric2seer): a command builder outside dispatcher.register(..)
        //   java: dispatcher.register(literal(SkyblockerMod.NAMESPACE)
        //   java: 				.then(literal("nightVision")
        //   java: 						.then(argument("strength", IntegerArgumentType.integer(0, 100))
        //   java: 								.executes(context -> NightVisionCommand.writeStrength(context, IntegerArgumentType.getInteger(context, "strength"))))
        //   java: 						.then(literal("off").executes(context -> NightVisionCommand.writeStrength(context, 0)))
        //   java: 						.then(literal("full").executes(context -> NightVisionCommand.writeStrength(context, 100)))));
    }

    /// java: private static int writeStrength(CommandContext<FabricClientCommandSource> context, int strength)
    pub fn write_strength(context: &CommandContext<crate::unresolved::FabricClientCommandSource>, mut strength: i32) -> i32 {
        SkyblockerConfigManager::update(Arc::new(move |mut config| { config.ui_and_visuals.night_vision_strength = strength; }));
        // TODO(fabric2seer): `.sendFeedback(..)` on a FabricClientCommandSource
        //   java: context.getSource().sendFeedback(Component.translatable("skyblocker.nightVision.success", strength));
        return 1;
    }

}

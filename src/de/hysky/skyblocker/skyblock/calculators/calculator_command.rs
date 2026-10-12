//! Ported from de.hysky.skyblocker.skyblock.calculators.CalculatorCommand by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::calculator::Calculator;
use crate::de::hysky::skyblocker::utils::constants;
use crate::de::hysky::skyblocker::utils::formatters;
use mcrust::com::mojang::brigadier::command_dispatcher::CommandDispatcher;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::commands::command_build_context::CommandBuildContext;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public class CalculatorCommand
#[derive(Clone)]
pub struct CalculatorCommand {}

impl CalculatorCommand {
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
                //   java: 				.then(literal("calculate")
                //   java: 						.then(argument("equation", StringArgumentType.greedyString())
                //   java: 								.executes(context -> doCalculation(getString(context, "equation")))
                //   java: 						)
                //   java: 				)
                //   java: 		);
            },
        ));
    }

    /// java: private static void calculate(CommandDispatcher<FabricClientCommandSource> dispatcher, CommandBuildContext registryAccess)
    pub fn calculate(dispatcher: &CommandDispatcher<crate::unresolved::FabricClientCommandSource>, registry_access: &CommandBuildContext) {
        // TODO(fabric2seer): a command builder outside dispatcher.register(..)
        //   java: dispatcher.register(literal(SkyblockerMod.NAMESPACE)
        //   java: 				.then(literal("calculate")
        //   java: 						.then(argument("equation", StringArgumentType.greedyString())
        //   java: 								.executes(context -> doCalculation(getString(context, "equation")))
        //   java: 						)
        //   java: 				)
        //   java: 		);
    }

    /// java: private static int doCalculation(String calculation)
    pub fn do_calculation(calculation: &str) -> i32 {
        let mut text: MutableComponent = (constants::PREFIX.get_mut())();
        // java: try {..} catch (Calculator.CalculatorException e) - an exception here is a panic, caught by `catch_unwind`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            text.append(component::literal(calculation).with_style_format(ChatFormatting::Aqua));
            text.append_str(" = ");
            text.append(component::literal(&formatters::DOUBLE_NUMBERS.get_mut().format(Calculator::calculate(calculation.to_string()))).with_style_format(ChatFormatting::Green));
        }));
        if let Err(e) = __result {
            text.append(component::translatable("skyblocker.config.uiAndVisuals.inputCalculator.invalidEquation").with_style_format(ChatFormatting::Red));
            // TODO(fabric2seer): field `args` of a exception
            //   java: text.append(Component.literal(": ").append(Component.translatable(e.getMessage(), e.args)).withStyle(ChatFormatting.RED));
        }
        if false /* java: compat::minecraft() == null: never null here */ || mcrust::mcrust::modding::game::with_player(|_| ()).is_none() {
            return 0;
        }
        mcrust::mcrust::modding::game::with_player(|player| { script_host::send_feedback(&text.to_string()); });
        return 1;
    }

}

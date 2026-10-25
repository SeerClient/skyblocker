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
        // java: ClientCommandRegistrationCallback.EVENT.register(CalculatorCommand::calculate);
        client_command_registration_callback::EVENT.register(Arc::new(
            |dispatcher: &mut client_command_registration_callback::ClientCommandDispatcher| {
                CalculatorCommand::calculate(dispatcher.dispatcher(), dispatcher.build_context());
            },
        ));
    }

    /// java: private static void calculate(CommandDispatcher<FabricClientCommandSource> dispatcher, CommandBuildContext registryAccess)
    pub fn calculate(dispatcher: &CommandDispatcher<client_command_registration_callback::FabricClientCommandSource>, registry_access: Option<&CommandBuildContext>) {
        //   java: dispatcher.register(literal(SkyblockerMod.NAMESPACE).then(literal("calculate")
        //   java:     .then(argument("equation", StringArgumentType.greedyString()).executes(context -> doCalculation(getString(context, "equation"))))));
        use mcrust::com::mojang::brigadier::arguments::string_argument_type::StringArgumentType;
        use mcrust::com::mojang::brigadier::builder::argument_builder::ArgumentBuilder as _;
        let _ = registry_access;
        let mut equation = client_command_registration_callback::argument("equation", Arc::new(StringArgumentType::greedy_string()));
        equation.executes(Arc::new(|context: &mcrust::com::mojang::brigadier::context::command_context::CommandContext<client_command_registration_callback::FabricClientCommandSource>| {
            Ok(CalculatorCommand::do_calculation(&StringArgumentType::get_string(context, "equation")))
        }));
        let mut calculate = client_command_registration_callback::literal("calculate");
        calculate.then(&equation);
        let mut root = client_command_registration_callback::literal(crate::entry::skyblocker_mod::NAMESPACE);
        root.then(&calculate);
        dispatcher.register(&root);
    }

    /// java: private static int doCalculation(String calculation)
    pub fn do_calculation(calculation: &str) -> i32 {
        let mut text: MutableComponent = (constants::PREFIX.get_mut())();
        // java: try { text.append(calculation (aqua)); text.append(" = "); text.append(format(Calculator.calculate(calculation)) (green)); }
        // catch (Calculator.CalculatorException e) {..} - the exception is a panic of `calculate`, caught by `catch_unwind`;
        // the two appends before it run either way, as they do in Java (`append` answers the component: assigned back)
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| Calculator::calculate(calculation.to_string())));
        text = text.append(component::literal(calculation).with_style_format(ChatFormatting::Aqua));
        text = text.append_str(" = ");
        if let Ok(__value) = &__result {
            text = text.append(component::literal(&formatters::DOUBLE_NUMBERS.get_mut().format(__value.clone())).with_style_format(ChatFormatting::Green));
        }
        if let Err(e) = __result {
            // (only a `CalculatorException` is caught: any other panic propagates)
            let exception = crate::de::hysky::skyblocker::utils::calculator::CalculatorException::of(&*e).cloned();
            let Some(exception) = exception else {
                std::panic::resume_unwind(e);
            };
            text = text.append(component::translatable("skyblocker.config.uiAndVisuals.inputCalculator.invalidEquation").with_style_format(ChatFormatting::Red));
            // java: text.append(Component.literal(": ").append(Component.translatable(e.getMessage(), e.args)).withStyle(ChatFormatting.RED));
            text = text.append(component::literal(": ").append(exception.to_component()).with_style_format(ChatFormatting::Red));
        }
        if false /* java: compat::minecraft() == null: never null here */ || mcrust::mcrust::modding::game::with_player(|_| ()).is_none() {
            return 0;
        }
        mcrust::mcrust::modding::game::with_player(|player| { script_host::send_feedback(&text.to_string()); });
        return 1;
    }

}

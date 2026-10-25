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
                NightVisionCommand::register(dispatcher);
            },
        ));
    }

    /// java: private static void register(CommandDispatcher<FabricClientCommandSource> dispatcher, CommandBuildContext access)
    /// (`access` is not read)
    pub fn register(dispatcher: &mut client_command_registration_callback::ClientCommandDispatcher) {
        use mcrust::com::mojang::brigadier::arguments::integer_argument_type::IntegerArgumentType;
        use mcrust::com::mojang::brigadier::builder::argument_builder::ArgumentBuilder as _;
        // java: dispatcher.register(literal(SkyblockerMod.NAMESPACE).then(literal("nightVision")
        //     .then(argument("strength", IntegerArgumentType.integer(0, 100)).executes(context -> NightVisionCommand.writeStrength(context, IntegerArgumentType.getInteger(context, "strength"))))
        //     .then(literal("off").executes(context -> NightVisionCommand.writeStrength(context, 0)))
        //     .then(literal("full").executes(context -> NightVisionCommand.writeStrength(context, 100)))));
        let mut strength = client_command_registration_callback::argument("strength", Arc::new(IntegerArgumentType::integer_range(0, 100)));
        strength.executes(Arc::new(|context: &CommandContext<mcrust::mcrust::modding::events::client_command_registration_callback::FabricClientCommandSource>| Ok(NightVisionCommand::write_strength(context, IntegerArgumentType::get_integer(context, "strength")))));
        let mut off = client_command_registration_callback::literal("off");
        off.executes(Arc::new(|context: &CommandContext<mcrust::mcrust::modding::events::client_command_registration_callback::FabricClientCommandSource>| Ok(NightVisionCommand::write_strength(context, 0))));
        let mut full = client_command_registration_callback::literal("full");
        full.executes(Arc::new(|context: &CommandContext<mcrust::mcrust::modding::events::client_command_registration_callback::FabricClientCommandSource>| Ok(NightVisionCommand::write_strength(context, 100))));
        let mut night_vision = client_command_registration_callback::literal("nightVision");
        night_vision.then(&strength);
        night_vision.then(&off);
        night_vision.then(&full);
        let mut namespace = client_command_registration_callback::literal(crate::entry::skyblocker_mod::NAMESPACE);
        namespace.then(&night_vision);
        dispatcher.dispatcher().register(&namespace);
    }

    /// java: private static int writeStrength(CommandContext<FabricClientCommandSource> context, int strength)
    pub fn write_strength(context: &CommandContext<mcrust::mcrust::modding::events::client_command_registration_callback::FabricClientCommandSource>, mut strength: i32) -> i32 {
        let _ = context;
        SkyblockerConfigManager::update(Arc::new(move |mut config| { config.ui_and_visuals.night_vision_strength = strength; }));
        // java: context.getSource().sendFeedback(Component.translatable("skyblocker.nightVision.success", strength));
        compat::batch_w_send_feedback(&mcrust::net::minecraft::network::chat::component::translatable_args("skyblocker.nightVision.success", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of(strength.to_string())]));
        return 1;
    }

}

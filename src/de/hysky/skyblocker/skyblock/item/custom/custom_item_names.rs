//! Ported from de.hysky.skyblocker.skyblock.item.custom.CustomItemNames by fabric2seer.

#[allow(unused_imports)]
use mcrust::net::minecraft::world::entity::living_entity::LivingEntity as _;
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::item::custom::screen::customize_screen::CustomizeScreen;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler::SchedulerBase;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::com::mojang::brigadier::command_dispatcher::CommandDispatcher;
use mcrust::net::minecraft::commands::command_build_context::CommandBuildContext;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::chat::style::Style;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: public class CustomItemNames
#[derive(Clone)]
pub struct CustomItemNames {}

impl CustomItemNames {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        // java: ClientCommandRegistrationCallback.EVENT.register(CustomItemNames::registerCommands);
        client_command_registration_callback::EVENT.register(Arc::new(
            |dispatcher: &mut client_command_registration_callback::ClientCommandDispatcher| {
                CustomItemNames::register_commands(dispatcher.dispatcher(), dispatcher.build_context());
            },
        ));
    }

    /// java: private static void registerCommands(CommandDispatcher<FabricClientCommandSource> dispatcher, CommandBuildContext registryAccess)
    pub fn register_commands(dispatcher: &CommandDispatcher<crate::unresolved::FabricClientCommandSource>, registry_access: Option<&CommandBuildContext>) {
        //   java: dispatcher.register(ClientCommands.literal("skyblocker").then(ClientCommands.literal("custom").then(ClientCommands.literal("renameItem")
        //   java:     .executes(context -> openScreen(context.getSource()))
        //   java:     .then(ClientCommands.argument("textComponent", ComponentArgument.textComponent(registryAccess))
        //   java:         .executes(context -> renameItem(context.getSource(), context.getArgument("textComponent", Component.class))))
        //   java:     // greedy string will only consume the arg if the text component parsing fails.
        //   java:     .then(ClientCommands.argument("basicText", StringArgumentType.greedyString())
        //   java:         .executes(context -> renameItem(context.getSource(), Component.nullToEmpty(context.getArgument("basicText", String.class))))))));
        use mcrust::com::mojang::brigadier::builder::argument_builder::ArgumentBuilder as _;
        use mcrust::com::mojang::brigadier::arguments::string_argument_type::StringArgumentType;
        type Ctx = mcrust::com::mojang::brigadier::context::command_context::CommandContext<crate::unresolved::FabricClientCommandSource>;
        let mut rename_item = client_command_registration_callback::literal("renameItem");
        rename_item.executes(Arc::new(|context: &Ctx| Ok(CustomItemNames::open_screen(context.get_source()))));
        // (the registry access is the client's once the server's command tree arrived; the text-component form needs it)
        if let Some(registry_access) = registry_access {
            let mut text_component = client_command_registration_callback::argument("textComponent", Arc::new(mcrust::net::minecraft::commands::arguments::component_argument::ComponentArgument::text_component(registry_access)));
            text_component.executes(Arc::new(|context: &Ctx| {
                let text = context.get_argument::<mcrust::net::minecraft::network::chat::formatted_text::FormattedText>("textComponent").cloned().expect("getArgument: textComponent");
                Ok(CustomItemNames::rename_item(context.get_source(), compat::component_of(text)))
            }));
            rename_item.then(&text_component);
        }
        let mut basic_text = client_command_registration_callback::argument("basicText", Arc::new(StringArgumentType::greedy_string()));
        basic_text.executes(Arc::new(|context: &Ctx| Ok(CustomItemNames::rename_item(context.get_source(), mcrust::net::minecraft::network::chat::component::null_to_empty(context.get_argument::<String>("basicText").map(|s| s.as_str()))))));
        rename_item.then(&basic_text);
        let mut custom = client_command_registration_callback::literal("custom");
        custom.then(&rename_item);
        let mut root = client_command_registration_callback::literal("skyblocker");
        root.then(&custom);
        dispatcher.register(&root);
    }

    /// java: private static int openScreen(FabricClientCommandSource source)
    pub fn open_screen(source: &crate::unresolved::FabricClientCommandSource) -> i32 {
        if !(Utils::is_on_skyblock()) {
            // java: source.sendError(Constants.PREFIX.get().append(Component.translatable("skyblocker.customItemNames.notOnSkyblock")));
            compat::batch_w_send_error(&(crate::de::hysky::skyblocker::utils::constants::PREFIX.get())().append(mcrust::net::minecraft::network::chat::component::translatable("skyblocker.customItemNames.notOnSkyblock")));
            return 0;
        }
        // java: ItemStack handStack = source.getPlayer().getMainHandItem(); (the source's player is the client's)
        let mut hand_stack: mcrust::net::minecraft::world::item::item_stack::ItemStack = mcrust::mcrust::modding::game::with_player(|player| player.get_main_hand_item().clone()).unwrap_or_else(mcrust::net::minecraft::world::item::item_stack::empty);
        if hand_stack.is_empty() {
            // java: source.sendError(Constants.PREFIX.get().append(Component.translatable("skyblocker.customItemNames.noItem")));
            compat::batch_w_send_error(&(crate::de::hysky::skyblocker::utils::constants::PREFIX.get())().append(mcrust::net::minecraft::network::chat::component::translatable("skyblocker.customItemNames.noItem")));
            return 0;
        }
        if hand_stack.get_uuid().is_empty() {
            // java: source.sendError(Constants.PREFIX.get().append(Component.translatable("skyblocker.customItemNames.noItemUuid")));
            compat::batch_w_send_error(&(crate::de::hysky::skyblocker::utils::constants::PREFIX.get())().append(mcrust::net::minecraft::network::chat::component::translatable("skyblocker.customItemNames.noItemUuid")));
            return 0;
        }
        SchedulerBase::queue_open_screen_screen(Box::new(compat::ScreenAdapter::new(CustomizeScreen::new(true))));
        return 1;
    }

    /// java: @SuppressWarnings("SameReturnValue") private static int renameItem(FabricClientCommandSource source, Component text)
    pub fn rename_item(source: &crate::unresolved::FabricClientCommandSource, mut text: MutableComponent) -> i32 {
        if Utils::is_on_skyblock() {
            // java: String itemUuid = source.getPlayer().getMainHandItem().getUuid(); (the source's player is the client's)
            let mut item_uuid: String = mcrust::mcrust::modding::game::with_player(|player| player.get_main_hand_item().get_uuid()).unwrap_or_default();
            if !item_uuid.is_empty() {
                // Set italic to false if it hasn't been changed (or was already false):
                // ((MutableComponent) text).setStyle(currentStyle.withItalic(currentStyle.isItalic())) - the update's
                // lambda runs once, at once, so the component is restyled before it is handed in
                let current_style = text.get_style().clone();
                let text = text.set_style(current_style.with_italic(Some(current_style.is_italic())));
                SkyblockerConfigManager::update(Arc::new(move |mut config| {
                    let custom_item_names = &mut config.general.custom_item_names;
                    custom_item_names.insert(item_uuid.clone(), text.clone());
                }));
                // java: source.sendFeedback(Constants.PREFIX.get().append(Component.translatable("skyblocker.customItemNames.added")));
                compat::batch_w_send_feedback(&(crate::de::hysky::skyblocker::utils::constants::PREFIX.get())().append(mcrust::net::minecraft::network::chat::component::translatable("skyblocker.customItemNames.added")));
            } else {
                // java: source.sendError(Constants.PREFIX.get().append(Component.translatable("skyblocker.customItemNames.noItemUuid")));
                compat::batch_w_send_error(&(crate::de::hysky::skyblocker::utils::constants::PREFIX.get())().append(mcrust::net::minecraft::network::chat::component::translatable("skyblocker.customItemNames.noItemUuid")));
            }
        } else {
            // java: source.sendError(Constants.PREFIX.get().append(Component.translatable("skyblocker.customItemNames.notOnSkyblock")));
            compat::batch_w_send_error(&(crate::de::hysky::skyblocker::utils::constants::PREFIX.get())().append(mcrust::net::minecraft::network::chat::component::translatable("skyblocker.customItemNames.notOnSkyblock")));
        }
        return 1;
    }

}

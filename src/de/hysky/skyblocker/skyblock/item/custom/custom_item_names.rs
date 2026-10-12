//! Ported from de.hysky.skyblocker.skyblock.item.custom.CustomItemNames by fabric2seer.

#[allow(unused_imports)]
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
        client_command_registration_callback::EVENT.register(Arc::new(
            |dispatcher: &mut client_command_registration_callback::ClientCommandDispatcher| {
                dispatcher.register("skyblocker", |ctx| {
                    #[allow(unreachable_code)]
                    Ok(())
                });
            },
        ));
    }

    /// java: private static void registerCommands(CommandDispatcher<FabricClientCommandSource> dispatcher, CommandBuildContext registryAccess)
    pub fn register_commands(dispatcher: &CommandDispatcher<crate::unresolved::FabricClientCommandSource>, registry_access: &CommandBuildContext) {
        // TODO(fabric2seer): a command builder outside dispatcher.register(..)
        //   java: dispatcher.register(ClientCommands.literal("skyblocker")
        //   java: 				.then(ClientCommands.literal("custom")
        //   java: 						.then(ClientCommands.literal("renameItem")
        //   java: 								.executes(context -> openScreen(context.getSource()))
        //   java: 								.then(ClientCommands.argument("textComponent", ComponentArgument.textComponent(registryAccess))
        //   java: 										.executes(context -> renameItem(context.getSource(), context.getArgument("textComponent", Component.class))))
        //   java: 								// greedy string will only consume the arg if the text component parsing fails.
        //   java: 								.then(ClientCommands.argument("basicText", StringArgumentType.greedyString())
        //   java: 										.executes(context -> renameItem(context.getSource(), Component.nullToEmpty(context.getArgument("basicText", String.class))))))));
    }

    /// java: private static int openScreen(FabricClientCommandSource source)
    pub fn open_screen(source: &crate::unresolved::FabricClientCommandSource) -> i32 {
        if !(Utils::is_on_skyblock()) {
            // TODO(fabric2seer): `.sendError(..)` on a FabricClientCommandSource
            //   java: source.sendError(Constants.PREFIX.get().append(Component.translatable("skyblocker.customItemNames.notOnSkyblock")));
            return 0;
        }
        // TODO(fabric2seer): `.getPlayer(..)` on a FabricClientCommandSource
        //   java: ItemStack handStack = source.getPlayer().getMainHandItem();
        let mut hand_stack: mcrust::net::minecraft::world::item::item_stack::ItemStack = todo!("fabric2seer: the initialiser of `handStack` (TODO above)");
        if hand_stack.is_empty() {
            // TODO(fabric2seer): `.sendError(..)` on a FabricClientCommandSource
            //   java: source.sendError(Constants.PREFIX.get().append(Component.translatable("skyblocker.customItemNames.noItem")));
            return 0;
        }
        if hand_stack.get_uuid().is_empty() {
            // TODO(fabric2seer): `.sendError(..)` on a FabricClientCommandSource
            //   java: source.sendError(Constants.PREFIX.get().append(Component.translatable("skyblocker.customItemNames.noItemUuid")));
            return 0;
        }
        SchedulerBase::queue_open_screen_screen(Box::new(compat::ScreenAdapter::new(CustomizeScreen::new(true))));
        return 1;
    }

    /// java: @SuppressWarnings("SameReturnValue") private static int renameItem(FabricClientCommandSource source, Component text)
    pub fn rename_item(source: &crate::unresolved::FabricClientCommandSource, mut text: MutableComponent) -> i32 {
        if Utils::is_on_skyblock() {
            // TODO(fabric2seer): `.getPlayer(..)` on a FabricClientCommandSource
            //   java: String itemUuid = source.getPlayer().getMainHandItem().getUuid();
            let mut item_uuid: String = todo!("fabric2seer: the initialiser of `itemUuid` (TODO above)");
            if !item_uuid.is_empty() {
                // Set italic to false if it hasn't been changed (or was already false):
                // ((MutableComponent) text).setStyle(currentStyle.withItalic(currentStyle.isItalic())) - the update's
                // lambda runs once, at once, so the component is restyled before it is handed in
                let current_style = text.get_style();
                let text = text.set_style(current_style.with_italic(Some(current_style.is_italic())));
                SkyblockerConfigManager::update(Arc::new(move |mut config| {
                    let custom_item_names = &mut config.general.custom_item_names;
                    custom_item_names.insert(item_uuid.clone(), text.clone());
                }));
                // TODO(fabric2seer): `.sendFeedback(..)` on a FabricClientCommandSource
                //   java: source.sendFeedback(Constants.PREFIX.get().append(Component.translatable("skyblocker.customItemNames.added")));
            } else {
                // TODO(fabric2seer): `.sendError(..)` on a FabricClientCommandSource
                //   java: source.sendError(Constants.PREFIX.get().append(Component.translatable("skyblocker.customItemNames.noItemUuid")));
            }
        } else {
            // TODO(fabric2seer): `.sendError(..)` on a FabricClientCommandSource
            //   java: source.sendError(Constants.PREFIX.get().append(Component.translatable("skyblocker.customItemNames.notOnSkyblock")));
        }
        return 1;
    }

}

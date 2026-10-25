//! Ported from de.hysky.skyblocker.skyblock.item.custom.CustomArmorDyeColors by fabric2seer.

#[allow(unused_imports)]
use mcrust::net::minecraft::world::entity::living_entity::LivingEntity as _;
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::com::mojang::brigadier::command_dispatcher::CommandDispatcher;
use mcrust::net::minecraft::commands::command_build_context::CommandBuildContext;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;

/// java: public class CustomArmorDyeColors
#[derive(Clone)]
pub struct CustomArmorDyeColors {}

impl CustomArmorDyeColors {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        // java: ClientCommandRegistrationCallback.EVENT.register(CustomArmorDyeColors::registerCommands);
        client_command_registration_callback::EVENT.register(Arc::new(
            |dispatcher: &mut client_command_registration_callback::ClientCommandDispatcher| {
                CustomArmorDyeColors::register_commands(dispatcher.dispatcher(), dispatcher.build_context());
            },
        ));
    }

    /// java: private static void registerCommands(CommandDispatcher<FabricClientCommandSource> dispatcher, CommandBuildContext registryAccess)
    pub fn register_commands(dispatcher: &CommandDispatcher<crate::unresolved::FabricClientCommandSource>, registry_access: Option<&CommandBuildContext>) {
        //   java: dispatcher.register(ClientCommands.literal("skyblocker").then(ClientCommands.literal("custom").then(ClientCommands.literal("dyeColor")
        //   java:     .executes(context -> customizeDyeColor(context.getSource(), Integer.MIN_VALUE))
        //   java:     .then(ClientCommands.argument("hexCode", ColorArgumentType.hex()).executes(context -> customizeDyeColor(context.getSource(), ColorArgumentType.getIntFromHex(context, "hexCode")))))));
        use mcrust::com::mojang::brigadier::builder::argument_builder::ArgumentBuilder as _;
        use crate::de::hysky::skyblocker::utils::command::argumenttypes::color::color_argument_type::ColorArgumentType;
        type Ctx = mcrust::com::mojang::brigadier::context::command_context::CommandContext<crate::unresolved::FabricClientCommandSource>;
        let _ = registry_access;
        let mut hex_code = client_command_registration_callback::argument("hexCode", Arc::new(ColorArgumentType::hex()));
        hex_code.executes(Arc::new(|context: &Ctx| Ok(CustomArmorDyeColors::customize_dye_color(context.get_source(), ColorArgumentType::get_int_from_hex(context, "hexCode")))));
        let mut dye_color = client_command_registration_callback::literal("dyeColor");
        dye_color.executes(Arc::new(|context: &Ctx| Ok(CustomArmorDyeColors::customize_dye_color(context.get_source(), i32::MIN))));
        dye_color.then(&hex_code);
        let mut custom = client_command_registration_callback::literal("custom");
        custom.then(&dye_color);
        let mut root = client_command_registration_callback::literal("skyblocker");
        root.then(&custom);
        dispatcher.register(&root);
    }

    /// java: @SuppressWarnings("SameReturnValue") private static int customizeDyeColor(FabricClientCommandSource source, int color)
    pub fn customize_dye_color(source: &crate::unresolved::FabricClientCommandSource, mut color: i32) -> i32 {
        // java: ItemStack heldItem = source.getPlayer().getMainHandItem(); (the source's player is the client's)
        let mut held_item: mcrust::net::minecraft::world::item::item_stack::ItemStack = mcrust::mcrust::modding::game::with_player(|player| player.get_main_hand_item().clone()).unwrap_or_else(mcrust::net::minecraft::world::item::item_stack::empty);
        if Utils::is_on_skyblock() && true /* java: held_item != null: never null here */ {
            if held_item.is_in_tag(&mcrust::net::minecraft::tags::item_tags::cauldron_can_remove_dye()) {
                let mut item_uuid: String = held_item.get_uuid();
                if !item_uuid.is_empty() {
                    let custom_dye_colors = &SkyblockerConfigManager::get().general.custom_dye_colors;
                    if color == i32::MIN {
                        if custom_dye_colors.contains_key(&item_uuid) {
                            SkyblockerConfigManager::update(Arc::new(move |mut config| { config.general.custom_dye_colors.remove(&item_uuid); }));
                            // java: source.sendFeedback(Constants.PREFIX.get().append(Component.translatable("skyblocker.customDyeColors.removed")));
                            compat::batch_w_send_feedback(&(crate::de::hysky::skyblocker::utils::constants::PREFIX.get())().append(mcrust::net::minecraft::network::chat::component::translatable("skyblocker.customDyeColors.removed")));
                        } else {
                            // java: source.sendFeedback(Constants.PREFIX.get().append(Component.translatable("skyblocker.customDyeColors.neverHad")));
                            compat::batch_w_send_feedback(&(crate::de::hysky::skyblocker::utils::constants::PREFIX.get())().append(mcrust::net::minecraft::network::chat::component::translatable("skyblocker.customDyeColors.neverHad")));
                        }
                    } else {
                        SkyblockerConfigManager::update(Arc::new(move |mut config| { config.general.custom_dye_colors.insert(item_uuid.clone(), color); }));
                        // java: source.sendFeedback(Constants.PREFIX.get().append(Component.translatable("skyblocker.customDyeColors.added")));
                        compat::batch_w_send_feedback(&(crate::de::hysky::skyblocker::utils::constants::PREFIX.get())().append(mcrust::net::minecraft::network::chat::component::translatable("skyblocker.customDyeColors.added")));
                    }
                } else {
                    // java: source.sendError(Constants.PREFIX.get().append(Component.translatable("skyblocker.customDyeColors.noItemUuid").withColor(TextColor.RED)));
                    compat::batch_w_send_error(&(crate::de::hysky::skyblocker::utils::constants::PREFIX.get())().append(mcrust::net::minecraft::network::chat::component::translatable("skyblocker.customDyeColors.noItemUuid").with_color(mcrust::net::minecraft::network::chat::text_color::TextColor::RED.get_value())));
                }
            } else {
                // java: source.sendError(Constants.PREFIX.get().append(Component.translatable("skyblocker.customDyeColors.notDyeable").withColor(TextColor.RED)));
                compat::batch_w_send_error(&(crate::de::hysky::skyblocker::utils::constants::PREFIX.get())().append(mcrust::net::minecraft::network::chat::component::translatable("skyblocker.customDyeColors.notDyeable").with_color(mcrust::net::minecraft::network::chat::text_color::TextColor::RED.get_value())));
                return 1;
            }
        } else {
            // java: source.sendError(Constants.PREFIX.get().append(Component.translatable("skyblocker.customDyeColors.unableToSetColor").withColor(TextColor.RED)));
            compat::batch_w_send_error(&(crate::de::hysky::skyblocker::utils::constants::PREFIX.get())().append(mcrust::net::minecraft::network::chat::component::translatable("skyblocker.customDyeColors.unableToSetColor").with_color(mcrust::net::minecraft::network::chat::text_color::TextColor::RED.get_value())));
        }
        return 1;
    }

}

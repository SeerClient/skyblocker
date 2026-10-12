//! Ported from de.hysky.skyblocker.skyblock.item.custom.CustomArmorDyeColors by fabric2seer.

#[allow(unused_imports)]
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
        //   java: 						.then(ClientCommands.literal("dyeColor")
        //   java: 								.executes(context -> customizeDyeColor(context.getSource(), Integer.MIN_VALUE))
        //   java: 								.then(ClientCommands.argument("hexCode", ColorArgumentType.hex())
        //   java: 										.executes(context -> customizeDyeColor(context.getSource(), ColorArgumentType.getIntFromHex(context, "hexCode")))))));
    }

    /// java: @SuppressWarnings("SameReturnValue") private static int customizeDyeColor(FabricClientCommandSource source, int color)
    pub fn customize_dye_color(source: &crate::unresolved::FabricClientCommandSource, mut color: i32) -> i32 {
        // TODO(fabric2seer): `.getPlayer(..)` on a FabricClientCommandSource
        //   java: ItemStack heldItem = source.getPlayer().getMainHandItem();
        let mut held_item: mcrust::net::minecraft::world::item::item_stack::ItemStack = todo!("fabric2seer: the initialiser of `heldItem` (TODO above)");
        if Utils::is_on_skyblock() && true /* java: held_item != null: never null here */ {
            if held_item.is_in_tag(&mcrust::net::minecraft::tags::item_tags::cauldron_can_remove_dye()) {
                let mut item_uuid: String = held_item.get_uuid();
                if !item_uuid.is_empty() {
                    let custom_dye_colors = &SkyblockerConfigManager::get().general.custom_dye_colors;
                    if color == i32::MIN {
                        if custom_dye_colors.contains_key(&item_uuid) {
                            SkyblockerConfigManager::update(Arc::new(move |mut config| { config.general.custom_dye_colors.remove(&item_uuid); }));
                            // TODO(fabric2seer): `.sendFeedback(..)` on a FabricClientCommandSource
                            //   java: source.sendFeedback(Constants.PREFIX.get().append(Component.translatable("skyblocker.customDyeColors.removed")));
                        } else {
                            // TODO(fabric2seer): `.sendFeedback(..)` on a FabricClientCommandSource
                            //   java: source.sendFeedback(Constants.PREFIX.get().append(Component.translatable("skyblocker.customDyeColors.neverHad")));
                        }
                    } else {
                        SkyblockerConfigManager::update(Arc::new(move |mut config| { config.general.custom_dye_colors.insert(item_uuid.clone(), color); }));
                        // TODO(fabric2seer): `.sendFeedback(..)` on a FabricClientCommandSource
                        //   java: source.sendFeedback(Constants.PREFIX.get().append(Component.translatable("skyblocker.customDyeColors.added")));
                    }
                } else {
                    // TODO(fabric2seer): `.sendError(..)` on a FabricClientCommandSource
                    //   java: source.sendError(Constants.PREFIX.get().append(Component.translatable("skyblocker.customDyeColors.noItemUuid").withColor(TextColor.RED)));
                }
            } else {
                // TODO(fabric2seer): `.sendError(..)` on a FabricClientCommandSource
                //   java: source.sendError(Constants.PREFIX.get().append(Component.translatable("skyblocker.customDyeColors.notDyeable").withColor(TextColor.RED)));
                return 1;
            }
        } else {
            // TODO(fabric2seer): `.sendError(..)` on a FabricClientCommandSource
            //   java: source.sendError(Constants.PREFIX.get().append(Component.translatable("skyblocker.customDyeColors.unableToSetColor").withColor(TextColor.RED)));
        }
        return 1;
    }

}

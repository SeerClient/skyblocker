//! Ported from de.hysky.skyblocker.skyblock.item.ItemProtection by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::item::hotbar_slot_lock::HotbarSlotLock;
use crate::de::hysky::skyblocker::utils::constants;
use crate::de::hysky::skyblocker::utils::item_utils::ItemUtils;
use crate::de::hysky::skyblocker::utils::location::Location;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::com::mojang::brigadier::command_dispatcher::CommandDispatcher;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::key_mapping;
use mcrust::net::minecraft::client::player::local_player::LocalPlayer;
use mcrust::net::minecraft::commands::command_build_context::CommandBuildContext;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::world::entity::decoration::item_frame::ItemFrame;
use mcrust::net::minecraft::world::entity::entity::Entity;
use mcrust::net::minecraft::world::entity::player::player::Player;
use mcrust::net::minecraft::world::interaction_hand::InteractionHand;
use mcrust::net::minecraft::world::interaction_result::InteractionResult;
use mcrust::net::minecraft::world::inventory::abstract_container_menu::AbstractContainerMenu;
use mcrust::net::minecraft::world::inventory::slot::Slot;
use mcrust::net::minecraft::world::level::level::Level;
use mcrust::net::minecraft::world::phys::entity_hit_result::EntityHitResult;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;
use mcrust::net::minecraft::network::chat::component::Component as _;
use mcrust::net::minecraft::world::entity::living_entity::LivingEntity as _;
use mcrust::net::minecraft::world::level::collision_getter::CollisionGetter as _;

/// java: public static final Identifier ITEM_PROTECTION_TEX = SkyblockerMod.id("item_protection");
pub static ITEM_PROTECTION_TEX: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("item_protection"));
/// java: private static final Component NO_UUID_TEXT = Component.translatable("skyblocker.itemProtection.noItemUuid").withColor(TextColor.RED);
pub static NO_UUID_TEXT: compat::MainThread<MutableComponent> = compat::MainThread::new(|| component::translatable("skyblocker.itemProtection.noItemUuid").with_color_text(mcrust::net::minecraft::network::chat::text_color::TextColor::RED));
/// java: private static final Component UNABLE_TO_PROTECT_TEXT = Component.translatable("skyblocker.itemProtection.unableToProtect").withColor(TextColor.RED);
pub static UNABLE_TO_PROTECT_TEXT: compat::MainThread<MutableComponent> = compat::MainThread::new(|| component::translatable("skyblocker.itemProtection.unableToProtect").with_color_text(mcrust::net::minecraft::network::chat::text_color::TextColor::RED));
/// java: public static KeyMapping itemProtection;
pub static ITEM_PROTECTION: compat::MainThread<Option<mcrust::net::minecraft::client::key_mapping::KeyMapping>> = compat::MainThread::new(|| None);

/// java: public class ItemProtection
#[derive(Clone)]
pub struct ItemProtection {}

impl ItemProtection {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        key_binding_helper::register_key_binding(KeyMapping::new("key.skyblocker.itemProtection", 86, Category::Misc));
        client_command_registration_callback::EVENT.register(Arc::new(
            |dispatcher: &mut client_command_registration_callback::ClientCommandDispatcher| {
                dispatcher.register("skyblocker", |ctx| {
                    if ctx.args.first().map(|s| &**s) == Some("protectItem") {
                        ItemProtection::protect_my_item(ctx);
                        return Ok(());
                    }
                    #[allow(unreachable_code)]
                    Ok(())
                });
            },
        ));
        // TODO(fabric2seer): UseEntityCallback.EVENT.register: no SeerClient equivalent yet
        //   java: UseEntityCallback.EVENT.register(ItemProtection::onEntityInteract);
    }

    /// java: public static void drawSlotIcon(GuiGraphicsExtractor graphics, int slotX, int slotY)
    pub fn draw_slot_icon(graphics: &GuiGraphicsExtractor, mut slot_x: i32, mut slot_y: i32) {
        gui_graphics_extractor::blit_sprite_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), ITEM_PROTECTION_TEX.get_mut(), slot_x, slot_y, 16, 16);
    }

    /// java: public static boolean isItemProtected(@Nullable ItemStack stack)
    pub fn is_item_protected(mut stack: Option<mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> bool {
        let Some(mut stack) = stack.clone() else {
            return false;
        };
        let mut item_uuid: String = stack.get_uuid();
        return SkyblockerConfigManager::get().general.protected_items.contains(&item_uuid);
    }

    /// java: private static void registerCommand(CommandDispatcher<FabricClientCommandSource> dispatcher, CommandBuildContext registryAccess)
    pub fn register_command(dispatcher: &CommandDispatcher<crate::unresolved::FabricClientCommandSource>, registry_access: &CommandBuildContext) {
        // TODO(fabric2seer): a command builder outside dispatcher.register(..)
        //   java: dispatcher.register(ClientCommands.literal("skyblocker")
        //   java: 				.then(ClientCommands.literal("protectItem")
        //   java: 						.executes(context -> protectMyItem(context.getSource()))));
    }

    /// java: private static int protectMyItem(FabricClientCommandSource source)
    /// `source` is the port's client command source (`ClientCommandContext`: its feedback goes to chat as text).
    pub fn protect_my_item(source: &client_command_registration_callback::ClientCommandContext<'_>) -> i32 {
        // ItemStack heldItem = source.getPlayer().getMainHandItem();
        let mut held_item: mcrust::net::minecraft::world::item::item_stack::ItemStack = {
            use mcrust::net::minecraft::world::entity::living_entity::LivingEntity as _;
            mcrust::mcrust::modding::game::with_player(|player| player.get_main_hand_item().clone()).unwrap_or_else(mcrust::net::minecraft::world::item::item_stack::empty)
        };
        if Utils::is_on_skyblock() {
            let mut item_uuid: String = held_item.get_uuid();
            if !item_uuid.is_empty() {
                if !(SkyblockerConfigManager::get().general.protected_items.contains(&item_uuid)) {
                    SkyblockerConfigManager::update(Arc::new(move |mut config| { config.general.protected_items.insert(item_uuid.clone()); }));
                    source.send_feedback(&(constants::PREFIX.get_mut())().append(ItemProtection::get_added_message(&held_item)).get_string());
                } else {
                    SkyblockerConfigManager::update(Arc::new(move |mut config| { config.general.protected_items.remove(&item_uuid); }));
                    source.send_feedback(&(constants::PREFIX.get_mut())().append(component::translatable_args("skyblocker.itemProtection.removed", vec![held_item.get_hover_name()])).get_string());
                }
            } else {
                source.send_feedback(&(constants::PREFIX.get_mut())().append(NO_UUID_TEXT.get().clone()).get_string());
            }
        } else {
            source.send_feedback(&(constants::PREFIX.get_mut())().append(UNABLE_TO_PROTECT_TEXT.get().clone()).get_string());
        }
        return 1;
    }

    /// java: public static void handleKeyPressed(ItemStack heldItem)
    pub fn handle_key_pressed(held_item: &mcrust::net::minecraft::world::item::item_stack::ItemStack) {
        let mut notify_configuration: bool = SkyblockerConfigManager::get().general.item_protection.display_chat_notification;
        if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() {
            return;
        }
        if !(Utils::is_on_skyblock()) {
            mcrust::mcrust::modding::game::with_player(|player| { script_host::send_feedback(&(constants::PREFIX.get_mut())().append(UNABLE_TO_PROTECT_TEXT.get_mut().clone()).to_string()); });
            return;
        }
        if held_item.is_empty() {
            mcrust::mcrust::modding::game::with_player(|player| { script_host::send_feedback(&(constants::PREFIX.get_mut())().append(NO_UUID_TEXT.get_mut().clone()).to_string()); });
            return;
        }
        let mut item_uuid: String = held_item.get_uuid();
        if !item_uuid.is_empty() {
            if !(SkyblockerConfigManager::get().general.protected_items.contains(&item_uuid)) {
                SkyblockerConfigManager::update(Arc::new(move |mut config| { config.general.protected_items.insert(item_uuid.clone()); }));
                if notify_configuration {
                    mcrust::mcrust::modding::game::with_player(|player| { script_host::send_feedback(&(constants::PREFIX.get_mut())().append(ItemProtection::get_added_message(held_item)).to_string()); });
                }
            } else {
                SkyblockerConfigManager::update(Arc::new(move |mut config| { config.general.protected_items.remove(&item_uuid); }));
                if notify_configuration {
                    mcrust::mcrust::modding::game::with_player(|player| { script_host::send_feedback(&(constants::PREFIX.get_mut())().append(component::translatable_args("skyblocker.itemProtection.removed", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(compat::component_of(held_item.get_hover_name()))])).to_string()); });
                }
            }
        } else {
            mcrust::mcrust::modding::game::with_player(|player| { script_host::send_feedback(&(constants::PREFIX.get_mut())().append(NO_UUID_TEXT.get_mut().clone()).to_string()); });
        }
    }

    /// java: private static Component getAddedMessage(ItemStack heldItem)
    pub fn get_added_message(held_item: &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> MutableComponent {
        return (component::translatable_args("skyblocker.itemProtection.added", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(compat::component_of(held_item.get_hover_name()))])).append_str(" ").append(component::translatable("skyblocker.itemProtection.added.safer").with_style_format(ChatFormatting::Italic));
    }

    /// java: public static void handleHotbarKeyPressed(LocalPlayer player)
    pub fn handle_hotbar_key_pressed(player: &LocalPlayer) {
        // the port's key mappings are reached by name
        while key_mapping::consume_click("key.skyblocker.itemProtection") {
            let held_item = player.get_main_hand_item();
            ItemProtection::handle_key_pressed(held_item);
        }
    }

    /// java: public static boolean isPersonalStorage(String screenTitle)
    pub fn is_personal_storage(screen_title: &str) -> bool {
        return (screen_title == "Storage" || screen_title.starts_with("Storage (") || screen_title == "Rift Storage" || screen_title.starts_with("Rift Storage (") || screen_title.starts_with("Ender Chest") || screen_title.starts_with("Chest") || screen_title.starts_with("Trapped Chest") || screen_title.contains("Backpack") && screen_title.contains("(Slot #") || screen_title.starts_with("Chest Storage") || screen_title.starts_with("Medium Shelves") || screen_title.starts_with("Wood Chest") || screen_title.starts_with("Diamond Chest") || screen_title.starts_with("Emerald Chest") || screen_title.starts_with("Iron Chest") || screen_title.starts_with("Gold Chest") || screen_title.starts_with("Lapis Chest") || screen_title.starts_with("Redstone Chest") || screen_title.starts_with("Endstone Chest") || screen_title.starts_with("Skull Chest") || screen_title.starts_with("Weapon Rack") || screen_title.starts_with("Armor Stand") || screen_title.contains("Stats & Equipment") || screen_title.contains("Equipment Sets") || screen_title.contains("Armor Sets") || screen_title.contains("Loadouts") || screen_title.contains("Pets"));
    }

    /// java: public static boolean isNpcSellMenu(AbstractContainerMenu menu)
    pub fn is_npc_sell_menu(menu: &AbstractContainerMenu) -> bool {
        for slot in menu.slots.iter() {
            // java: ItemStack stack = slot.getItem(); - the menu is the player's open one (its containers are the open menu's)
            let mut stack: mcrust::net::minecraft::world::item::item_stack::ItemStack = compat::slot_item(slot);
            if stack.is_empty() {
                continue;
            }
            let mut name: String = compat::component_of(stack.get_hover_name()).get_string();
            if name == "Sell Item" || name == "Sell Inventory" {
                return true;
            }
            if ItemUtils::get_lore_line_if(&stack, Arc::new(move |text| text.contains("buyback"))).is_some() {
                return true;
            }
        }
        return false;
    }

    /// java: public static boolean isNpcSellButton(Slot slot)
    pub fn is_npc_sell_button(slot: &Slot) -> bool {
        // java: String name = slot.getItem().getHoverName().getString();
        let stack: mcrust::net::minecraft::world::item::item_stack::ItemStack = compat::slot_item(slot);
        let mut name: String = compat::component_of(stack.get_hover_name()).get_string();
        // java: return name.equals("Sell Item") || name.equals("Sell Inventory") || ItemUtils.getLoreLineIf(slot.getItem(), text -> text.contains("buyback")) != null;
        return name == "Sell Item" || name == "Sell Inventory" || ItemUtils::get_lore_line_if(&stack, Arc::new(move |text| text.contains("buyback"))).is_some();
    }

    /// java: private static InteractionResult onEntityInteract(Player playerEntity, Level world, InteractionHand hand, Entity entity, @Nullable EntityHitResult entityHitResult)
    pub fn on_entity_interact(player_entity: &dyn Player, world: &Level, hand: &InteractionHand, entity: &dyn Entity, mut entity_hit_result: Option<EntityHitResult>) -> InteractionResult {
        if !(Utils::is_on_skyblock()) || !world.is_client_side_level() {
            return InteractionResult::Pass;
        }
        let mut location: Location = Utils::get_location();
        if !(location == Location::PrivateIsland || location == Location::Garden) {
            return InteractionResult::Pass;
        }
        if let Some(item_frame) = entity.as_any().unwrap_or(&() as &dyn std::any::Any).downcast_ref::<ItemFrame>() && item_frame.get_item().is_empty() {
            if ItemProtection::is_item_protected(Some(player_entity.get_item_in_hand(*hand).clone())) || HotbarSlotLock::is_locked(player_entity.player().get_inventory().get_selected_slot()) {
                return InteractionResult::Fail;
            }
        }
        return InteractionResult::Pass;
    }

}

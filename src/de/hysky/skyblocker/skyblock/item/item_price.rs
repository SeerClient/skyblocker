//! Ported from de.hysky.skyblocker.skyblock.item.ItemPrice by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::events::item_price_update_event;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::info::tooltip_info_type;
use crate::de::hysky::skyblocker::skyblock::itemlist::item_repository::ItemRepository;
use crate::de::hysky::skyblocker::utils::constants;
use crate::de::hysky::skyblocker::utils::enchanted_book_utils::EnchantedBookUtils;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use crate::de::hysky::skyblocker::utils::scheduler::message_scheduler;
use crate::de::hysky::skyblocker::utils::scheduler::message_scheduler::MessageScheduler;
use mcrust::java::util::concurrent::completable_future::CompletableFuture;
use mcrust::net::minecraft::chat_formatting;
use mcrust::net::minecraft::client::player::local_player::LocalPlayer;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::world::inventory::slot::Slot;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::info::data_tooltip_info_type::DataTooltipInfoType as _;
use mcrust::net::minecraft::network::chat::component::Component as _;

//   java: public static final KeyMapping ITEM_PRICE_LOOKUP = KeyMappingHelper.registerKeyMapping(new KeyMapping(
//   java: 			"key.skyblocker.itemPriceLookup",
//   java: 			InputConstants.KEY_F6,
//   java: 			SkyblockerMod.KEYBINDING_CATEGORY
//   java: 	));
/// java: public static final KeyMapping ITEM_PRICE_LOOKUP = KeyMappingHelper.registerKeyMapping(new KeyMapping( "key.skyblocker.itemPriceLookup", InputConstants.KEY_F6, SkyblockerMod.KEYBINDING_CATEGORY ));
pub static ITEM_PRICE_LOOKUP: compat::MainThread<mcrust::net::minecraft::client::key_mapping::KeyMapping> = compat::MainThread::new(|| key_binding_helper::register_key_binding(mcrust::net::minecraft::client::key_mapping::KeyMapping::new("key.skyblocker.itemPriceLookup", mcrust::com::mojang::blaze3d::platform::input_constants::KEY_F6, *crate::entry::skyblocker_mod::KEYBINDING_CATEGORY.get())));

/// java: public class ItemPrice
#[derive(Clone)]
pub struct ItemPrice {}

impl ItemPrice {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @SuppressWarnings("UnstableApiUsage") //For the Javadoc reference. @Init public static void init()
    pub fn init() {
        // the class's static initialiser runs when `init` first touches the class: the key mapping registers then
        let _ = ITEM_PRICE_LOOKUP.get();
        client_command_registration_callback::EVENT.register(Arc::new(
            |dispatcher: &mut client_command_registration_callback::ClientCommandDispatcher| {
                //   java: dispatcher.register(ClientCommands.literal(SkyblockerMod.NAMESPACE)
                //   java: 					.then(ClientCommands.literal("refreshPrices").executes(context -> {
                //   java: 						refreshItemPrices(context.getSource().getPlayer());
                //   java: 						return Command.SINGLE_SUCCESS;
                //   java: 					})));
                use mcrust::com::mojang::brigadier::builder::argument_builder::ArgumentBuilder as _;
                let mut refresh_prices = client_command_registration_callback::literal("refreshPrices");
                refresh_prices.executes(Arc::new(|_: &mcrust::com::mojang::brigadier::context::command_context::CommandContext<client_command_registration_callback::FabricClientCommandSource>| {
                    // java: context.getSource().getPlayer()
                    mcrust::mcrust::modding::game::with_player(|player| ItemPrice::refresh_item_prices(player));
                    Ok(1)
                }));
                let mut root = client_command_registration_callback::literal(crate::entry::skyblocker_mod::NAMESPACE);
                root.then(&refresh_prices);
                dispatcher.dispatcher().register(&root);
            },
        ));
    }

    /// java: public static void itemPriceLookup(LocalPlayer player, Slot slot)
    pub fn item_price_lookup(player: &LocalPlayer, slot: &Slot) {
        // java: ItemStack stack = slot.getItem();
        let mut stack: mcrust::net::minecraft::world::item::item_stack::ItemStack = compat::slot_item(slot);
        ItemPrice::item_price_lookup_local_player_item_stack(player, &stack);
    }

    /// java: public static void itemPriceLookup(LocalPlayer player, ItemStack stack)
    pub fn item_price_lookup_local_player_item_stack(player: &LocalPlayer, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack) {
        let mut skyblock_api_id: String = stack.get_skyblock_api_id();
        let mut flexible: Option<compat::Shared<FlexibleItemStack>> = ItemRepository::get_item_stack(&stack.get_neu_name());
        let mut neu_stack: Option<mcrust::net::minecraft::world::item::item_stack::ItemStack> = (if flexible.is_none() { None } else { Some(FlexibleItemStack::get_stack_or_throw(flexible.as_ref().unwrap())) });
        if neu_stack.is_some() && !(neu_stack.as_ref().unwrap().is_empty()) {
            let mut item_name: String = chat_formatting::strip_formatting(&compat::component_of(neu_stack.as_ref().unwrap().get_hover_name()).get_string());
            if stack.get_skyblock_id() == "PET" {
                item_name = compat::regex::replace_first(&item_name, "\\[Lvl \\d+ ➡ \\d+] ", "");
            }
            if item_name == "Enchanted Book" {
                item_name = EnchantedBookUtils::get_enchant_name_from_lore(&stack.skyblocker_get_lore_strings());
            }
            item_name = chat_formatting::strip_formatting(&item_name);
            if tooltip_info_type::BAZAAR.get_mut().lock().borrow().has_or_null_warning(&skyblock_api_id) {
                MessageScheduler::send_message_after_cooldown(message_scheduler::INSTANCE.get_mut(), &format!("/bz {}", item_name), true);
                return;
            } else if tooltip_info_type::LOWEST_BINS.get_mut().lock().borrow().has_or_null_warning(&skyblock_api_id) {
                MessageScheduler::send_message_after_cooldown(message_scheduler::INSTANCE.get_mut(), &format!("/auctionsearch {}", item_name), true);
                return;
            }
        }
        script_host::send_feedback(&(constants::PREFIX.get_mut())().append(component::translatable("skyblocker.config.helpers.itemPrice.itemPriceLookupFailed")).to_string());
    }

    /// java: private static void refreshItemPrices(LocalPlayer player)
    pub fn refresh_item_prices(player: &LocalPlayer) {
        script_host::send_feedback(&(constants::PREFIX.get_mut())().append(component::translatable("skyblocker.config.helpers.itemPrice.refreshingItemPrices")).to_string());
        // CompletableFuture.allOf(Stream.of(NPC, BAZAAR, LOWEST_BINS, ONE_DAY_AVERAGE, THREE_DAY_AVERAGE)
        //   .map(DataTooltipInfoType::downloadIfEnabled).toArray(CompletableFuture[]::new))
        let downloads = vec![
            compat::write(tooltip_info_type::NPC.get(), |t| t.download_if_enabled()),
            compat::write(tooltip_info_type::BAZAAR.get(), |t| t.download_if_enabled()),
            compat::write(tooltip_info_type::LOWEST_BINS.get(), |t| t.download_if_enabled()),
            compat::write(tooltip_info_type::ONE_DAY_AVERAGE.get(), |t| t.download_if_enabled()),
            compat::write(tooltip_info_type::THREE_DAY_AVERAGE.get(), |t| t.download_if_enabled()),
        ];
        // .thenRunAsync(() -> {..}, Minecraft.getInstance()).exceptionally(e -> {..; return null;})
        CompletableFuture::all_of(&downloads)
            .then_run_async(
                || {
                    (item_price_update_event::on_price_update_invoker())();
                    script_host::send_feedback(&(constants::PREFIX.get_mut())().append(component::translatable("skyblocker.config.helpers.itemPrice.refreshedItemPrices")).to_string());
                },
                std::sync::Arc::new(|__t: Box<dyn FnOnce() + Send>| crate::compat::execute(__t)),
            )
            .exceptionally(|e| {
                logger().error(&format!("[Skyblocker Item Price] Failed to refresh item prices: {}", e));
                script_host::send_feedback(&(constants::PREFIX.get_mut())().append(component::translatable("skyblocker.config.helpers.itemPrice.itemPriceRefreshFailed")).to_string());
            });
    }

}

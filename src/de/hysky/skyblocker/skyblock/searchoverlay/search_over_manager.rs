//! Ported from de.hysky.skyblocker.skyblock.searchoverlay.SearchOverManager by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::configs::ui_and_visuals_config::SearchOverlay;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::info::tooltip_info_type;
use crate::de::hysky::skyblocker::skyblock::itemlist::item_repository::ItemRepository;
use crate::de::hysky::skyblocker::skyblock::museum::donation::Donation;
use crate::de::hysky::skyblocker::skyblock::museum::museum_item_cache;
use crate::de::hysky::skyblocker::skyblock::museum::museum_item_cache::MuseumItemCache;
use crate::de::hysky::skyblocker::skyblock::searchoverlay::overlay_screen::OverlayScreen;
use crate::de::hysky::skyblocker::utils::bazaar_product::BazaarProduct;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use crate::de::hysky::skyblocker::utils::neu_repo_manager::NEURepoManager;
use crate::de::hysky::skyblocker::utils::scheduler::message_scheduler;
use crate::de::hysky::skyblocker::utils::scheduler::message_scheduler::MessageScheduler;
use mcrust::com::mojang::brigadier::command_dispatcher::CommandDispatcher;
use mcrust::java::text::decimal_format_symbols::Locale;
use mcrust::net::minecraft::commands::command_build_context::CommandBuildContext;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::protocol::game::serverbound_sign_update_packet::ServerboundSignUpdatePacket;
use mcrust::net::minecraft::world::level::block::entity::sign_block_entity::SignBlockEntity;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::info::data_tooltip_info_type::DataTooltipInfoType as _;
use mcrust::net::minecraft::network::chat::component::Component as _;
use mcrust::net::minecraft::util::thread::task_scheduler::TaskScheduler as _;

/// java: private static final String PET_NAME_START = "[Lvl {LVL}] ";
pub const PET_NAME_START: &str = "[Lvl {LVL}] ";
/// java: private static @Nullable SignBlockEntity sign = null;
///
/// The level's own block entity (the same object Java keeps), as the open-sign-editor arm hands it on.
pub static SIGN: compat::MainThread<Option<std::sync::Arc<dyn mcrust::net::minecraft::world::level::block::entity::block_entity::BlockEntityObj>>> = compat::MainThread::new(|| None);
/// java: private static boolean signFront = true;
pub static SIGN_FRONT: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(true));
/// java: private static boolean isCommand;
pub static IS_COMMAND: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));
/// java: protected static SearchLocation location = SearchLocation.NONE;
pub static LOCATION: LazyLock<Mutex<SearchLocation>> = LazyLock::new(|| Mutex::new(SearchLocation::None));
/// java: protected static String search = "";
pub static SEARCH: LazyLock<Mutex<String>> = LazyLock::new(|| Mutex::new("".to_string()));
/// java: protected static Boolean maxPetLevel = false;
pub static MAX_PET_LEVEL: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));
/// java: protected static int dungeonStars = 0;
pub static DUNGEON_STARS: LazyLock<Mutex<i32>> = LazyLock::new(|| Mutex::new(0));
/// java: private static HashSet<String> bazaarItems = new HashSet<>();
pub static BAZAAR_ITEMS: LazyLock<Mutex<std::collections::HashSet<String>>> = LazyLock::new(|| Mutex::new(std::collections::HashSet::new()));
/// java: private static HashSet<String> auctionItems = new HashSet<>();
pub static AUCTION_ITEMS: LazyLock<Mutex<std::collections::HashSet<String>>> = LazyLock::new(|| Mutex::new(std::collections::HashSet::new()));
/// java: private static HashSet<String> auctionPets = new HashSet<>();
pub static AUCTION_PETS: LazyLock<Mutex<std::collections::HashSet<String>>> = LazyLock::new(|| Mutex::new(std::collections::HashSet::new()));
/// java: private static HashSet<String> level200Pets = new HashSet<>();
pub static LEVEL200_PETS: LazyLock<Mutex<std::collections::HashSet<String>>> = LazyLock::new(|| Mutex::new(std::collections::HashSet::new()));
/// java: private static HashSet<String> starableItems = new HashSet<>();
pub static STARABLE_ITEMS: LazyLock<Mutex<std::collections::HashSet<String>>> = LazyLock::new(|| Mutex::new(std::collections::HashSet::new()));
/// java: private static HashMap<String, String> namesToNeuId = new HashMap<>();
pub static NAMES_TO_NEU_ID: LazyLock<Mutex<std::collections::HashMap<String, String>>> = LazyLock::new(|| Mutex::new(std::collections::HashMap::new()));
/// java: public static String[] suggestionsArray = {};
pub static SUGGESTIONS_ARRAY: LazyLock<Mutex<Vec<String>>> = LazyLock::new(|| Mutex::new(vec![]));

/// java: public class SearchOverManager
#[derive(Clone)]
pub struct SearchOverManager {}

impl SearchOverManager {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        //   java: ClientCommandRegistrationCallback.EVENT.register(SearchOverManager::registerSearchCommands);
        client_command_registration_callback::EVENT.register(Arc::new(
            |dispatcher: &mut client_command_registration_callback::ClientCommandDispatcher| {
                SearchOverManager::register_search_commands(dispatcher);
            },
        ));
        NEURepoManager::run_async_after_load(Arc::new(move || { SearchOverManager::load_items(); }));
    }

    /// `literal(name).requires(_ -> Utils.isOnSkyblock()).executes(_ -> startCommand(isAuction, ""))`, optionally
    /// `.then(argument("item", StringArgumentType.greedyString())[.suggests(suggestions)].executes(context ->
    /// startCommand(isAuction, StringArgumentType.getString(context, "item"))))`
    fn search_command(name: &str, is_auction: bool, with_item: bool, suggests: Option<mcrust::com::mojang::brigadier::suggestion::suggestion_provider::SuggestionProvider<client_command_registration_callback::FabricClientCommandSource>>) -> mcrust::com::mojang::brigadier::builder::literal_argument_builder::LiteralArgumentBuilder<client_command_registration_callback::FabricClientCommandSource> {
        use mcrust::com::mojang::brigadier::arguments::string_argument_type::StringArgumentType;
        use mcrust::com::mojang::brigadier::builder::argument_builder::ArgumentBuilder as _;
        use mcrust::com::mojang::brigadier::context::command_context::CommandContext;
        type Source = client_command_registration_callback::FabricClientCommandSource;
        let mut literal = client_command_registration_callback::literal(name);
        literal.requires(Arc::new(|_: &Source| crate::de::hysky::skyblocker::utils::utils::Utils::is_on_skyblock()));
        if with_item {
            let mut item = client_command_registration_callback::argument("item", Arc::new(StringArgumentType::greedy_string()));
            if suggests.is_some() {
                item.suggests(suggests);
            }
            item.executes(Arc::new(move |context: &CommandContext<Source>| Ok(SearchOverManager::start_command(is_auction, &StringArgumentType::get_string(context, "item")))));
            literal.then(&item);
        } else {
            literal.executes(Arc::new(move |_: &CommandContext<Source>| Ok(SearchOverManager::start_command(is_auction, ""))));
        }
        literal
    }

    /// java: private static void registerSearchCommands(CommandDispatcher<FabricClientCommandSource> dispatcher, CommandBuildContext registryAccess)
    pub fn register_search_commands(dispatcher: &mut client_command_registration_callback::ClientCommandDispatcher) {
        use mcrust::com::mojang::brigadier::builder::argument_builder::ArgumentBuilder as _;
        if SkyblockerConfigManager::get().ui_and_visuals.search_overlay.enable_commands {
            //   java: dispatcher.register(literal("ahs").requires(_ -> Utils.isOnSkyblock()).executes(_ -> startCommand(true, "")));
            dispatcher.dispatcher().register(&SearchOverManager::search_command("ahs", true, false, None));
            //   java: dispatcher.register(literal("ahsearch").requires(_ -> Utils.isOnSkyblock()).executes(_ -> startCommand(true, "")));
            dispatcher.dispatcher().register(&SearchOverManager::search_command("ahsearch", true, false, None));
            //   java: dispatcher.register(literal("bzs").requires(_ -> Utils.isOnSkyblock()).executes(_ -> startCommand(false, "")));
            dispatcher.dispatcher().register(&SearchOverManager::search_command("bzs", false, false, None));
            //   java: dispatcher.register(literal("ahs").requires(..).then(argument("item", StringArgumentType.greedyString()).executes(context -> startCommand(true, StringArgumentType.getString(context, "item")))));
            dispatcher.dispatcher().register(&SearchOverManager::search_command("ahs", true, true, None));
            //   java: dispatcher.register(literal("ahsearch").requires(..).then(argument("item", ..).executes(context -> startCommand(true, ..))));
            dispatcher.dispatcher().register(&SearchOverManager::search_command("ahsearch", true, true, None));
            if SkyblockerConfigManager::get().ui_and_visuals.search_overlay.command_autocomplete {
                //   java: dispatcher.register(literal("bzs").requires(..).then(argument("item", ..).suggests(AuctionBazaarAutocomplete::suggestBzs).executes(context -> startCommand(false, ..))));
                let suggest_bzs: mcrust::com::mojang::brigadier::suggestion::suggestion_provider::SuggestionProvider<client_command_registration_callback::FabricClientCommandSource> =
                    Arc::new(|context: &mcrust::com::mojang::brigadier::context::command_context::CommandContext<client_command_registration_callback::FabricClientCommandSource>, builder: &mut mcrust::com::mojang::brigadier::suggestion::suggestions_builder::SuggestionsBuilder| Ok(crate::de::hysky::skyblocker::skyblock::auction_bazaar_autocomplete::AuctionBazaarAutocomplete::suggest_bzs(context, builder)));
                dispatcher.dispatcher().register(&SearchOverManager::search_command("bzs", false, true, Some(suggest_bzs)));
            } else {
                //   java: dispatcher.register(literal("bzs").requires(..).then(argument("item", ..).executes(context -> startCommand(false, ..))));
                dispatcher.dispatcher().register(&SearchOverManager::search_command("bzs", false, true, None));
            }
        }
        if !(crate::de::hysky::skyblocker::debug::debug::Debug::debug_enabled()) {
            return;
        }
        //   java: dispatcher.register(literal(SkyblockerMod.NAMESPACE).then(literal("debug").then(literal("reloadSearchOverManager").executes(ctx -> {
        //   java: 			ctx.getSource().sendFeedback(Constants.PREFIX.get().append(Component.literal("Reloading Search Overlay Manager items data").withColor(TextColor.AQUA)));
        //   java: 			SearchOverManager.loadItems();
        //   java: 			ctx.getSource().sendFeedback(Constants.PREFIX.get().append(Component.literal("Reloaded Search Overlay Manager items data").withColor(TextColor.AQUA)));
        //   java: 			return Command.SINGLE_SUCCESS;
        //   java: 		}))));
        let mut reload = client_command_registration_callback::literal("reloadSearchOverManager");
        reload.executes(Arc::new(|_: &mcrust::com::mojang::brigadier::context::command_context::CommandContext<client_command_registration_callback::FabricClientCommandSource>| {
            let prefix = crate::de::hysky::skyblocker::utils::constants::PREFIX.get().clone();
            client_command_registration_callback::send_feedback(&prefix().append(mcrust::net::minecraft::network::chat::component::literal("Reloading Search Overlay Manager items data").with_color_text(mcrust::net::minecraft::network::chat::text_color::TextColor::AQUA)).get_string());
            SearchOverManager::load_items();
            client_command_registration_callback::send_feedback(&prefix().append(mcrust::net::minecraft::network::chat::component::literal("Reloaded Search Overlay Manager items data").with_color_text(mcrust::net::minecraft::network::chat::text_color::TextColor::AQUA)).get_string());
            Ok(1)
        }));
        let mut debug = client_command_registration_callback::literal("debug");
        debug.then(&reload);
        let mut root = client_command_registration_callback::literal(crate::entry::skyblocker_mod::NAMESPACE);
        root.then(&debug);
        dispatcher.dispatcher().register(&root);
    }

    /// java: private static int startCommand(boolean isAuction, String itemName)
    pub fn start_command(mut is_auction: bool, item_name: &str) -> i32 {
        compat::set_state(&IS_COMMAND, true);
        compat::set_state(&LOCATION, (if is_auction { SearchLocation::Auction } else { SearchLocation::Bazaar }));
        compat::set_state(&SEARCH, "".to_string());
        compat::set_state(&SUGGESTIONS_ARRAY, vec![]);
        if !item_name.is_empty() {
            SearchOverManager::update_search(item_name.to_string());
        }
        //   java: CLIENT.schedule(() -> CLIENT.gui.setScreen(new OverlayScreen()));
        // (`Minecraft.schedule` queues the task for the client loop: the port's `minecraft::execute`)
        mcrust::net::minecraft::client::minecraft::execute(Box::new(|_minecraft| mcrust::mcrust::modding::game::set_screen(Box::new(compat::ScreenAdapter::new(OverlayScreen::new())))));
        return 1;
    }

    /// java: private static void loadItems()
    pub fn load_items() {
        let mut bazaar_items: std::collections::HashSet<String> = std::collections::HashSet::new();
        let mut auction_items: std::collections::HashSet<String> = std::collections::HashSet::new();
        let mut auction_pets: std::collections::HashSet<String> = std::collections::HashSet::new();
        let mut level200_pets: std::collections::HashSet<String> = std::collections::HashSet::new();
        let mut starable_items: std::collections::HashSet<String> = std::collections::HashSet::new();
        let mut names_to_neu_id: std::collections::HashMap<String, String> = std::collections::HashMap::new();
        // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if tooltip_info_type::BAZAAR.get_mut().lock().borrow().get_data().is_none() {
                tooltip_info_type::BAZAAR.get_mut().lock().borrow_mut().run();
            }
            let mut products: std::collections::HashMap<String, BazaarProduct> = tooltip_info_type::BAZAAR.get_mut().lock().borrow().get_data().unwrap();
            for entry in products.iter().map(|(k, v)| (k.clone(), v.clone())) {
                let product = &entry.1;
                let mut id: String = product.id();
                let mut sell_volume: i32 = product.sell_volume();
                if sell_volume == 0 {
                    continue;
                }
                if id.starts_with("ENCHANTMENT_") && ItemRepository::get_bazaar_stocks().contains_key(&id) {
                    let mut neu_id: String = ItemRepository::get_bazaar_stocks().get(&id).cloned().unwrap();
                    let Some(mut neu_item) = NEURepoManager::get_item_by_neu_id(&neu_id) else {
                        continue;
                    };
                    //   java: String name = ChatFormatting.stripFormatting(EnchantedBookUtils.getEnchantNameFromLore(neuItem.getLore()));
                    let mut name: String = mcrust::net::minecraft::chat_formatting::strip_formatting(&crate::de::hysky::skyblocker::utils::enchanted_book_utils::EnchantedBookUtils::get_enchant_name_from_lore(neu_item.get_lore()));
                    bazaar_items.insert(name.clone());
                    names_to_neu_id.insert(name.clone(), neu_id.clone());
                    continue;
                }
                if id.starts_with("SHARD_") && ItemRepository::get_bazaar_stocks().contains_key(&id) {
                    id = ItemRepository::get_bazaar_stocks().get(&id).cloned().unwrap();
                }
                let mut neu_item: Option<Arc<crate::io::github::moulberry::repo::data::neu_item::NEUItem>> = NEURepoManager::get_item_by_neu_id(&id);
                if let Some(neu_item) = &neu_item {
                    //   java: String name = ChatFormatting.stripFormatting(neuItem.getDisplayName());
                    let mut name: String = mcrust::net::minecraft::chat_formatting::strip_formatting(neu_item.get_display_name());
                    bazaar_items.insert(name.clone());
                    names_to_neu_id.insert(name.clone(), id.clone());
                }
            }
        }));
        if let Err(e) = __result {
            logger().error(&format!("[Skyblocker] Failed to load bazaar item list! "));
        }
        // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            //   java: Set<@NEUId String> essenceCosts = NEURepoManager.getConstants().getEssenceCost().getCosts().keySet();
            let essence_costs: std::collections::HashSet<String> = NEURepoManager::get_constants().get_essence_cost().get_costs().keys().cloned().collect();
            if tooltip_info_type::THREE_DAY_AVERAGE.get_mut().lock().borrow().get_data().is_none() {
                tooltip_info_type::THREE_DAY_AVERAGE.get_mut().lock().borrow_mut().run();
            }
            //   java: for (Object2DoubleMap.Entry<String> entry : TooltipInfoType.THREE_DAY_AVERAGE.getData().object2DoubleEntrySet()) {
            let three_day_average: std::collections::HashMap<String, f64> = tooltip_info_type::THREE_DAY_AVERAGE.get_mut().lock().borrow().get_data().expect("NullPointerException: THREE_DAY_AVERAGE.getData()");
            for (key, _) in three_day_average.iter() {
                let mut id: String = key.clone();
                id = crate::de::hysky::skyblocker::utils::item_utils::ItemUtils::get_neu_id_from_api_id(&id);
                //look up in NEU repo.
                let parts: Vec<String> = compat::regex::split(&id, "[+-]");
                id = parts[0].clone();
                let neu_item: Option<Arc<crate::io::github::moulberry::repo::data::neu_item::NEUItem>> = NEURepoManager::get_item_by_neu_id(&id);
                if let Some(neu_item) = &neu_item {
                    let mut name: String = mcrust::net::minecraft::chat_formatting::strip_formatting(neu_item.get_display_name());
                    //add names that are pets to the list of pets to work with the lvl 100 button
                    if name.starts_with(PET_NAME_START) {
                        name = name.replace(PET_NAME_START, "");
                        let pet_name: String = name.to_lowercase();
                        auction_pets.insert(pet_name.clone());
                        if parts.len() >= 2 && parts[1] == "200" {
                            level200_pets.insert(pet_name.clone());
                        }
                    }
                    //if it has essence cost add to starable items
                    if essence_costs.contains(neu_item.get_skyblock_item_id()) {
                        starable_items.insert(name.to_lowercase());
                    }
                    auction_items.insert(name.clone());
                    names_to_neu_id.insert(name.clone(), id.clone());
                }
            }
        }));
        if let Err(e) = __result {
            logger().error(&format!("[Skyblocker] Failed to load auction house item list! "));
        }
        compat::set_state(&BAZAAR_ITEMS, bazaar_items.clone());
        compat::set_state(&AUCTION_ITEMS, auction_items.clone());
        compat::set_state(&AUCTION_PETS, auction_pets.clone());
        compat::set_state(&LEVEL200_PETS, level200_pets.clone());
        compat::set_state(&STARABLE_ITEMS, starable_items.clone());
        compat::set_state(&NAMES_TO_NEU_ID, names_to_neu_id.clone());
    }

    /// java: public static HashSet<String> getBazaarItems()
    pub fn get_bazaar_items() -> std::collections::HashSet<String> {
        return compat::get_state(&BAZAAR_ITEMS);
    }

    /// java: public static HashSet<String> getAuctionItems()
    pub fn get_auction_items() -> std::collections::HashSet<String> {
        return compat::get_state(&AUCTION_ITEMS);
    }

    /// java: public static void updateSign(SignBlockEntity sign, boolean front, SearchLocation location)
    pub fn update_sign(sign: std::sync::Arc<dyn mcrust::net::minecraft::world::level::block::entity::block_entity::BlockEntityObj>, mut front: bool, mut location: SearchLocation) {
        compat::set_state(&SIGN_FRONT, front);
        //   java: SearchOverManager.sign = sign; - the same object
        *SIGN.get_mut() = Some(sign);
        compat::set_state(&IS_COMMAND, false);
        compat::set_state(&LOCATION, location);
        if SkyblockerConfigManager::get().ui_and_visuals.search_overlay.keep_previous_searches {
            let mut messages: Vec<MutableComponent> = SIGN.get_mut().as_ref().unwrap().as_sign().expect("ClassCastException: SignBlockEntity").get_text(compat::sign_slot(compat::get_state(&SIGN_FRONT))).get_messages(compat::minecraft().is_text_filtering_enabled()).iter().cloned().map(compat::component_of).collect::<Vec<_>>();
            compat::set_state(&SEARCH, (messages[(0) as usize]).get_string());
            if !(((messages[(1) as usize]).get_string()).is_empty()) {
                if !(compat::lock(&SEARCH).ends_with(" ")) {
                    compat::set_state(&SEARCH, format!("{} ", compat::get_state(&SEARCH)));
                }
                compat::set_state(&SEARCH, format!("{}{}", compat::get_state(&SEARCH), (messages[(1) as usize]).get_string()));
            }
        } else {
            compat::set_state(&SEARCH, "".to_string());
        }
        compat::set_state(&SUGGESTIONS_ARRAY, vec![]);
    }

    /// java: @SuppressWarnings("incomplete-switch") protected static void updateSearch(String newValue)
    pub fn update_search(mut new_value: String) {
        compat::set_state(&SEARCH, new_value.clone());
        let mut total_suggestions: i32 = SkyblockerConfigManager::get().ui_and_visuals.search_overlay.max_suggestions;
        if compat::is_blank(&new_value) || total_suggestions == 0 {
            return;
        }
        match compat::get_state(&LOCATION) {
            SearchLocation::Auction => {
                compat::set_state(&SUGGESTIONS_ARRAY, SearchOverManager::update_suggestions(&compat::get_state(&AUCTION_ITEMS), total_suggestions));
            }
            SearchLocation::Bazaar => {
                compat::set_state(&SUGGESTIONS_ARRAY, SearchOverManager::update_suggestions(&compat::get_state(&BAZAAR_ITEMS), total_suggestions));
            }
            SearchLocation::Museum => {
                compat::set_state(&SUGGESTIONS_ARRAY, SearchOverManager::update_suggestions(&SearchOverManager::get_museum_suggestions(), total_suggestions));
            }
            _ => {}
        }
    }

    /// java: private static Set<String> getMuseumSuggestions()
    pub fn get_museum_suggestions() -> std::collections::HashSet<String> {
        // java: Map<Boolean, List<Donation>> types = MuseumItemCache.MUSEUM_DONATIONS.stream().collect(Collectors.partitioningBy(Donation::isSet));
        let (sets_donations, items_donations): (Vec<compat::Shared<Donation>>, Vec<compat::Shared<Donation>>) = museum_item_cache::MUSEUM_DONATIONS.get().iter().cloned().partition(|__d| Donation::is_set(__d));
        // java: types.get(false).stream().map(Donation::getId).filter(MuseumItemCache::hasItemInMuseum).map(ItemRepository::getItemStack)
        //   .filter(Objects::nonNull).map(FlexibleItemStack::getStackOrThrow).map(ItemStack::getHoverName).map(Component::getString)
        let items = items_donations.iter().map(|__a0| Donation::get_id(__a0)).filter(|__a0| MuseumItemCache::has_item_in_museum(__a0)).filter_map(|__a0| ItemRepository::get_item_stack(&__a0)).map(|__a0| FlexibleItemStack::get_stack_or_throw(&__a0)).map(|__a0| compat::component_of(__a0.get_hover_name())).map(|__a0| __a0.get_string());
        // java: types.get(true).stream().map(Donation::getId).filter(MuseumItemCache::hasItemInMuseum).map(MuseumItemCache.ARMOR_NAMES::get)
        // (a set id with no armor name maps to null in Java, which the set then holds; a Rust set of names cannot)
        let sets = sets_donations.iter().map(|__a0| Donation::get_id(__a0)).filter(|__a0| MuseumItemCache::has_item_in_museum(__a0)).filter_map(|__a0| compat::lock(&museum_item_cache::ARMOR_NAMES).get(&__a0).cloned());
        // java: return Streams.concat(items, sets).collect(Collectors.toSet());
        return items.chain(sets).collect::<std::collections::HashSet<String>>();
    }

    /// java: protected static String[] updateSuggestions(Set<String> items, int totalSuggestions)
    pub fn update_suggestions(items: &std::collections::HashSet<String>, mut total_suggestions: i32) -> Vec<String> {
        // java: .sorted(Comparator.comparing(SearchOverManager::shouldFrontLoad, Comparator.reverseOrder()))
        return compat::sorted_by(items.iter().cloned(), |a: &String, b: &String| SearchOverManager::should_front_load(b).cmp(&SearchOverManager::should_front_load(a))).filter(|item| item.to_lowercase().contains(&compat::lock(&SEARCH).to_lowercase())).take((total_suggestions) as usize).collect::<Vec<_>>();
    }

    /// java: private static boolean shouldFrontLoad(String name)
    pub fn should_front_load(name: &str) -> bool {
        if compat::get_state(&LOCATION) != SearchLocation::Auction {
            return false;
        }
        if !(compat::lock(&AUCTION_PETS).contains(&name.to_lowercase())) {
            return false;
        }
        return ((((compat::str_len(&compat::lock(&SEARCH))) as f64) / (compat::str_len(&name) as f64)) > 0.5);
    }

    /// java: protected static String getSuggestion(int index)
    pub fn get_suggestion(mut index: i32) -> String {
        // one guard for the whole statement: two `compat::lock(&SUGGESTIONS_ARRAY)` temporaries in one
        // expression both live to its end, and the second waits for the first (a self-deadlock)
        #[allow(unused_mut)] let mut __guard_suggestions_array = compat::lock(&SUGGESTIONS_ARRAY);
        let __cond = ((*__guard_suggestions_array).len() as i32) > index && true /* java: (*__guard_suggestions_array)[(index) as usize] != null: never null here */;
        drop(__guard_suggestions_array);
        if __cond {
            return compat::lock(&SUGGESTIONS_ARRAY)[(index) as usize].clone();
        } else {
            return "".to_string();
        }
    }

    /// java: protected static String getSuggestionId(int index)
    pub fn get_suggestion_id(mut index: i32) -> String {
        let mut name: String = SearchOverManager::get_suggestion(index);
        return SearchOverManager::get_item_id(name.clone());
    }

    /// java: private static String getItemId(String name)
    pub fn get_item_id(mut name: String) -> String {
        if name.is_empty() {
            return "".to_string();
        }
        if compat::get_state(&LOCATION) != SearchLocation::Museum || !(compat::lock(&museum_item_cache::ARMOR_NAMES).values().any(|v| *v == name.clone())) {
            return compat::lock(&NAMES_TO_NEU_ID).entry(name.clone()).or_insert_with_key(|k| (|str| ItemRepository::get_items_stream().filter(|stack| (compat::component_of(FlexibleItemStack::get_stack_or_throw(stack).get_hover_name()).get_string() == str)).map(|__a0| __a0.lock().borrow().get_neu_name()).next().unwrap_or("".to_string()))(k.clone())).to_string();
        }
        let mut armor_id: String = "".to_string();
        for pair in compat::lock(&museum_item_cache::ARMOR_NAMES).iter().map(|(k, v)| (k.clone(), v.clone())) {
            if pair.1 == name {
                armor_id = pair.0.clone();
                break;
            }
        }
        return compat::lock(&museum_item_cache::ARMOR_TO_ID).get(&armor_id).cloned().unwrap_or_else(|| compat::lock(&NAMES_TO_NEU_ID).get(&name).cloned().unwrap());
    }

    /// java: protected static String getHistory(int index)
    pub fn get_history(mut index: i32) -> String {
        let config = &SkyblockerConfigManager::get().ui_and_visuals.search_overlay;
        let mut history: Vec<String>;
        match compat::get_state(&LOCATION) {
            SearchLocation::Auction => {
                history = config.auction_history.clone();
            }
            SearchLocation::Bazaar => {
                history = config.bazaar_history.clone();
            }
            SearchLocation::Museum => {
                history = config.museum_history.clone();
            }
            _ => {
                return "".to_string();
            }
        }
        if (history.len() as i32) > index {
            return history[(index) as usize].clone();
        }
        return "".to_string();
    }

    /// java: protected static String getHistoryId(int index)
    pub fn get_history_id(mut index: i32) -> String {
        return SearchOverManager::get_item_id(SearchOverManager::get_history(index));
    }

    /// java: protected static void removeHistoryItem(int index)
    pub fn remove_history_item(mut index: i32) {
        SkyblockerConfigManager::update_only(Arc::new(move |full_config| {
            let config = &full_config.ui_and_visuals.search_overlay;
            let mut history: Vec<String>;
            match compat::get_state(&LOCATION) {
                SearchLocation::Auction => {
                    history = config.auction_history.clone();
                }
                SearchLocation::Bazaar => {
                    history = config.bazaar_history.clone();
                }
                SearchLocation::Museum => {
                    history = config.museum_history.clone();
                }
                _ => {
                    return;
                }
            }
            if (history.len() as i32) > index {
                history.remove((index) as usize);
            }
        }));
    }

    /// java: private static List<String> addToHistory(List<String> history, String search, int historyLength)
    pub fn add_to_history(mut history: Vec<String>, mut search: String, mut history_length: i32) -> Vec<String> {
        if history.is_empty() || !(history.iter().any(|e| *e == search.clone())) {
            // java: history.addFirst(search);
            history.insert(0, search.clone());
            if (history.len() as i32) > history_length {
                history = history[(0) as usize..(history_length) as usize].to_vec();
            }
        } else {
            compat::remove_item(&mut history, &search.clone());
            // java: history.addFirst(search);
            history.insert(0, search.clone());
        }
        return history;
    }

    /// java: @SuppressWarnings("incomplete-switch") private static void saveHistory()
    pub fn save_history() {
        SkyblockerConfigManager::update_only(Arc::new(move |mut full_config| {
            let config = &mut full_config.ui_and_visuals.search_overlay;
            match compat::get_state(&LOCATION) {
                SearchLocation::Auction => {
                    config.auction_history = SearchOverManager::add_to_history(config.auction_history.clone(), compat::get_state(&SEARCH), config.history_length);
                }
                SearchLocation::Bazaar => {
                    config.bazaar_history = SearchOverManager::add_to_history(config.bazaar_history.clone(), compat::get_state(&SEARCH), config.history_length);
                }
                SearchLocation::Museum => {
                    config.museum_history = SearchOverManager::add_to_history(config.museum_history.clone(), compat::get_state(&SEARCH), config.history_length);
                }
                _ => {}
            }
        }));
    }

    /// java: protected static void pushSearch()
    pub fn push_search() {
        if !(compat::lock(&SEARCH).is_empty()) {
            SearchOverManager::save_history();
        }
        SkyblockerConfigManager::update(Arc::new(move |_| {

        }));
        if compat::get_state(&LOCATION) == SearchLocation::Auction {
            SearchOverManager::add_extras();
        }
        if !(compat::get_state(&IS_COMMAND)) && compat::lock(&SEARCH).to_lowercase().starts_with("null") {
            compat::set_state(&SEARCH, format!("\"{}\"", compat::get_state(&SEARCH)));
        }
        if compat::get_state(&IS_COMMAND) {
            SearchOverManager::push_command();
        } else {
            SearchOverManager::push_sign();
        }
    }

    /// java: private static void addExtras()
    pub fn add_extras() {
        if compat::get_state(&MAX_PET_LEVEL) {
            let mut lc_search: String = compat::lock(&SEARCH).to_lowercase();
            if compat::lock(&AUCTION_PETS).contains(&lc_search) {
                let mut max_level: i32 = (if compat::lock(&LEVEL200_PETS).contains(&lc_search) { 200 } else { 100 });
                compat::set_state(&SEARCH, format!("[Lvl {}] {}", max_level, compat::get_state(&SEARCH)));
            }
        } else {
            if compat::lock(&AUCTION_PETS).contains(&compat::lock(&SEARCH).to_lowercase()) {
                compat::set_state(&SEARCH, format!("] {}", compat::get_state(&SEARCH)));
            }
        }
        if compat::get_state(&DUNGEON_STARS) > 0 && compat::lock(&STARABLE_ITEMS).contains(&compat::lock(&SEARCH).to_lowercase()) {
            let mut star_string: String = " ".to_string();
            { star_string.push_str(&"✪".repeat((0i32.max((compat::get_state(&DUNGEON_STARS)).min(5))) as usize)); };
            match compat::get_state(&DUNGEON_STARS) {
                6 => {
                    { star_string.push_str("➊"); };
                }
                7 => {
                    { star_string.push_str("➋"); };
                }
                8 => {
                    { star_string.push_str("➌"); };
                }
                9 => {
                    { star_string.push_str("➍"); };
                }
                10 => {
                    { star_string.push_str("➎"); };
                }
                _ => {}
            }
            compat::set_state(&SEARCH, format!("{}{}", compat::get_state(&SEARCH), star_string.clone()));
        }
    }

    /// java: private static void pushCommand()
    pub fn push_command() {
        if compat::lock(&SEARCH).is_empty() {
            return;
        }
        let mut command: String;
        match compat::get_state(&LOCATION) {
            SearchLocation::Auction => {
                command = format!("/auctionsearch {}", compat::get_state(&SEARCH));
            }
            SearchLocation::Bazaar => {
                command = format!("/bz {}", compat::get_state(&SEARCH));
            }
            _ => {
                return;
            }
        }
        MessageScheduler::send_message_after_cooldown(message_scheduler::INSTANCE.get_mut(), &command, true);
    }

    /// java: private static void pushSign()
    pub fn push_sign() {
        let mut split: (String, String) = SearchOverManager::split_string(&compat::get_state(&SEARCH));
        if mcrust::mcrust::modding::game::with_player(|_| ()).is_some() && SIGN.get_mut().is_some() {
            let mut messages: Vec<MutableComponent> = SIGN.get_mut().as_ref().unwrap().as_sign().expect("ClassCastException: SignBlockEntity").get_text(compat::sign_slot(compat::get_state(&SIGN_FRONT))).get_messages(compat::minecraft().is_text_filtering_enabled()).iter().cloned().map(compat::component_of).collect::<Vec<_>>();
            //   java: CLIENT.player.connection.send(new ServerboundSignUpdatePacket(sign.getBlockPos(), signFront, split.left(), split.right(), messages[2].getString(), messages[3].getString()));
            // (`connection.send` from the client thread: the play connection's own hand-off, `client_advancements::send`)
            let __pos = SIGN.get_mut().as_ref().unwrap().as_sign().expect("ClassCastException: SignBlockEntity").get_block_pos();
            let __packet = mcrust::net::minecraft::network::protocol::game::serverbound_sign_update_packet::ServerboundSignUpdatePacket::new(__pos, vec![split.0.clone(), split.1.clone(), (messages[(2) as usize]).get_string(), (messages[(3) as usize]).get_string()], compat::sign_slot(compat::get_state(&SIGN_FRONT)));
            if let Some(__sinks) = mcrust::net::minecraft::client::multiplayer::client_packet_listener::published_play_sinks() {
                mcrust::net::minecraft::client::multiplayer::client_advancements::send(&__sinks.connection, __packet);
            }
        }
    }

    /// java: public static Pair<String, String> splitString(String s)
    pub fn split_string(s: &str) -> (String, String) {
        if compat::str_len(&s) <= 15 {
            return (s.to_string(), "".to_string());
        }
        let mut index: i32 = compat::last_index_of_from(&s, ' ', 15);
        if index == -1 {
            return (compat::substring2(&s, 0, 15), "".to_string());
        }
        return (compat::substring2(&s, 0, index), compat::trim(&(compat::substring2(&s, index.wrapping_add(1), (index.wrapping_add(16)).min(compat::str_len(&s))))));
    }

}

/// java: public enum SearchLocation
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SearchLocation {
    None,
    Auction,
    Bazaar,
    Museum,
}

impl SearchLocation {
    pub const VALUES: [SearchLocation; 4] = [SearchLocation::None, SearchLocation::Auction, SearchLocation::Bazaar, SearchLocation::Museum];

    /// java: values()
    pub fn values() -> Vec<SearchLocation> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            SearchLocation::None => "NONE",
            SearchLocation::Auction => "AUCTION",
            SearchLocation::Bazaar => "BAZAAR",
            SearchLocation::Museum => "MUSEUM",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> SearchLocation {
        match name {
            "NONE" => SearchLocation::None,
            "AUCTION" => SearchLocation::Auction,
            "BAZAAR" => SearchLocation::Bazaar,
            "MUSEUM" => SearchLocation::Museum,
            _ => panic!("IllegalArgumentException: No enum constant SearchLocation.{name}"),
        }
    }

}

impl std::fmt::Display for SearchLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

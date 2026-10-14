//! Ported from de.hysky.skyblocker.skyblock.museum.MuseumItemCache by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::events::skyblock_events;
use crate::de::hysky::skyblocker::skyblock::museum::donation::Donation;
use crate::de::hysky::skyblocker::skyblock::museum::museum_manager::MuseumManager;
use crate::de::hysky::skyblocker::skyblock::museum::museum_utils::MuseumUtils;
use crate::de::hysky::skyblocker::skyblock::museum::price_data::PriceData;
use crate::de::hysky::skyblocker::utils::data::profiled_data::ProfiledData;
use crate::de::hysky::skyblocker::utils::neu_repo_manager::NEURepoManager;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use crate::entry::skyblocker_mod;
use mcrust::com::mojang::brigadier::command_dispatcher::CommandDispatcher;
use mcrust::com::mojang::serialization::codec;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::com::mojang::serialization::codecs::record_codec_builder;
use mcrust::com::mojang::util::undashed_uuid;
use mcrust::java::util::concurrent::completable_future::CompletableFuture;
use mcrust::java::util::uuid::Uuid;
use mcrust::net::minecraft::commands::command_build_context::CommandBuildContext;
use mcrust::net::minecraft::core::non_null_list::NonNullList;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::hover_event::HoverEvent;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::world::inventory::slot::Slot;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;
use crate::de::hysky::skyblocker::utils::data::json_data::JsonData as _;
use crate::de::hysky::skyblocker::utils::constants;
use mcrust::com::mojang::brigadier::builder::argument_builder::ArgumentBuilder as _;
use mcrust::com::mojang::brigadier::context::command_context::CommandContext;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final String ERROR_LOG_TEMPLATE = "[Skyblocker] Failed to refresh museum item data for profile {}";
pub const ERROR_LOG_TEMPLATE: &str = "[Skyblocker] Failed to refresh museum item data for profile {}";
/// java: private static final int CONFIRM_DONATION_BUTTON_SLOT = 20;
pub const CONFIRM_DONATION_BUTTON_SLOT: i32 = 20;
/// java: private static final String CONSTANTS_MUSEUM_DATA = "constants/museum.json";
pub const CONSTANTS_MUSEUM_DATA: &str = "constants/museum.json";
/// java: private static final Path CACHE_FILE = SkyblockerMod.CONFIG_DIR.resolve("museum_item_cache.json");
pub static CACHE_FILE: compat::MainThread<std::path::PathBuf> = compat::MainThread::new(|| skyblocker_mod::CONFIG_DIR.get_mut().join("museum_item_cache.json"));
/// java: private static final ProfiledData<ProfileMuseumData> MUSEUM_ITEM_CACHE = new ProfiledData<>(CACHE_FILE, ProfileMuseumData.CODEC);
pub static MUSEUM_ITEM_CACHE: compat::MainThread<compat::Shared<ProfiledData<ProfileMuseumData>>> = compat::MainThread::new(|| ProfiledData::new(CACHE_FILE.get_mut().clone(), PROFILE_MUSEUM_DATA_CODEC.get_mut().clone()));
/// java: public static final String DONATION_CONFIRMATION_SCREEN_TITLE = "Confirm Donation";
pub const DONATION_CONFIRMATION_SCREEN_TITLE: &str = "Confirm Donation";
/// java: public static final Map<String, String> ARMOR_NAMES = Object2ObjectMaps.synchronize(new Object2ObjectArrayMap<>());
/// (a synchronized map: `Mutex`. Java's `Object2ObjectArrayMap` keeps insertion order, which a `HashMap` does not -
/// `SearchOverManager` iterates it; see GAPS_J)
pub static ARMOR_NAMES: LazyLock<Mutex<std::collections::HashMap<String, String>>> = LazyLock::new(|| Mutex::new(std::collections::HashMap::new()));
/// java: public static final Map<String, String> ARMOR_TO_ID = Object2ObjectMaps.synchronize(new Object2ObjectArrayMap<>());
pub static ARMOR_TO_ID: LazyLock<Mutex<std::collections::HashMap<String, String>>> = LazyLock::new(|| Mutex::new(std::collections::HashMap::new()));
/// java: private static final Map<String, String> MAPPED_IDS = Object2ObjectMaps.synchronize(new Object2ObjectArrayMap<>());
pub static MAPPED_IDS: LazyLock<Mutex<std::collections::HashMap<String, String>>> = LazyLock::new(|| Mutex::new(std::collections::HashMap::new()));
/// java: public static Set<String> MUSEUM_CATEGORIES = Set.of();
/// (assigned `itemCategories.keySet()` - the key set of Gson's insertion-ordered map, so an ordered list of unique keys)
pub static MUSEUM_CATEGORIES: LazyLock<Mutex<Vec<String>>> = LazyLock::new(|| Mutex::new(Vec::new()));
/// java: public static final ObjectList<Donation> MUSEUM_DONATIONS = ObjectLists.synchronize(new ObjectArrayList<>());
pub static MUSEUM_DONATIONS: compat::MainThread<Vec<compat::Shared<Donation>>> = compat::MainThread::new(|| Vec::new());
/// java: private static final ObjectList<ObjectArrayList<String>> ORDERED_UPGRADES = ObjectLists.synchronize(new ObjectArrayList<>());
pub static ORDERED_UPGRADES: LazyLock<Mutex<Vec<Vec<String>>>> = LazyLock::new(|| Mutex::new(Vec::new()));
/// java: private static final int CURRENT_DATA_VERSION = 1;
pub const CURRENT_DATA_VERSION: i32 = 1;
/// java: private static final Supplier<ProfileMuseumData> EMPTY = () -> new ProfileMuseumData(0L, new ObjectOpenHashSet<>(), CURRENT_DATA_VERSION);
pub static PROFILE_MUSEUM_DATA_EMPTY: compat::MainThread<Arc<dyn Fn() -> ProfileMuseumData>> = compat::MainThread::new(|| Arc::new(move || ProfileMuseumData::new(0, std::collections::HashSet::new(), CURRENT_DATA_VERSION)));
/// java: private static final long TIME_BETWEEN_RESYNCING_ALLOWED = 600_000L;
pub const PROFILE_MUSEUM_DATA_TIME_BETWEEN_RESYNCING_ALLOWED: i64 = 600000;
/// java: private static final Codec<ProfileMuseumData> CODEC = RecordCodecBuilder.create(instance -> instance.group(
/// java: 		Codec.LONG.fieldOf("lastResync").forGetter(ProfileMuseumData::lastResync),
/// java: 		Codec.STRING.listOf().xmap(ObjectOpenHashSet::new, ObjectArrayList::new).fieldOf("collectedItemIds").forGetter(ProfileMuseumData::collectedItemIds),
/// java: 		Codec.INT.optionalFieldOf("dataVersion", 0).forGetter(ProfileMuseumData::dataVersion)
/// java: ).apply(instance, ProfileMuseumData::new));
pub static PROFILE_MUSEUM_DATA_CODEC: compat::MainThread<CodecRef<ProfileMuseumData>> = compat::MainThread::new(|| {
    record_codec_builder::create(record_codec_builder::apply3(
        record_codec_builder::of(|o: &ProfileMuseumData| o.last_resync(), codec::field_of(mcrust::com::mojang::serialization::codec::long_codec(), "lastResync")),
        record_codec_builder::of(
            |o: &ProfileMuseumData| o.collected_item_ids(),
            codec::field_of(
                codec::xmap(
                    codec::list(mcrust::com::mojang::serialization::codec::string_codec()),
                    |list: Vec<String>| list.into_iter().collect::<std::collections::HashSet<String>>(),
                    |set: &std::collections::HashSet<String>| set.iter().cloned().collect::<Vec<String>>(),
                ),
                "collectedItemIds",
            ),
        ),
        record_codec_builder::of(|o: &ProfileMuseumData| o.data_version(), codec::optional_field_of(mcrust::com::mojang::serialization::codec::int_codec(), "dataVersion", 0)),
        move |__a0, __a1, __a2| ProfileMuseumData::new(__a0.clone(), __a1.clone(), __a2.clone()),
    ))
});

/// java: public class MuseumItemCache
#[derive(Clone)]
pub struct MuseumItemCache {}

impl MuseumItemCache {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        MuseumItemCache::load_museum_items();
        // java: ClientLifecycleEvents.CLIENT_STARTED.register(_ -> MUSEUM_ITEM_CACHE.load()); - RETURN of `Minecraft`'s
        // constructor (as `WidgetManager.init` hooks it)
        // rust: a RETURN injector registered from inside `Minecraft#<init>` (the client entrypoints run there) is not in
        // that call's injector snapshot, so it never fired; `compat::execute` runs it before the first client tick, which
        // is where Fabric fires CLIENT_STARTED (early in `Minecraft.run`).
        compat::execute(|| {
            let _ = MUSEUM_ITEM_CACHE.get_mut().lock().borrow_mut().load();
        });
        // java: ClientCommandRegistrationCallback.EVENT.register(MuseumItemCache::registerCommands);
        client_command_registration_callback::EVENT.register(Arc::new(
            |dispatcher: &mut client_command_registration_callback::ClientCommandDispatcher| {
                MuseumItemCache::register_commands(dispatcher.dispatcher(), dispatcher.build_context());
            },
        ));
        skyblock_events::PROFILE_CHANGE.register(Arc::new(move |_, _| { MuseumItemCache::on_profile_change(); }));
    }

    /// java: private static void registerCommands(CommandDispatcher<FabricClientCommandSource> dispatcher, CommandBuildContext registryAccess)
    /// (`registryAccess` is the port's `Option`: absent before the first join)
    pub fn register_commands(dispatcher: &CommandDispatcher<client_command_registration_callback::FabricClientCommandSource>, registry_access: Option<&CommandBuildContext>) {
        let _ = registry_access;
        // java: dispatcher.register(literal(SkyblockerMod.NAMESPACE)
        // java: 		.then(literal("museum")
        // java: 				.then(literal("resync")
        // java: 						.executes(context -> {
        // java: 							FabricClientCommandSource source = context.getSource();
        // java: 							Component text = tryResync(source);
        // java: 							source.sendFeedback(Constants.PREFIX.get().append(text));
        // java: 							return Command.SINGLE_SUCCESS;
        // java: 						}))));
        {
            let mut resync = client_command_registration_callback::literal("resync");
            resync.executes(Arc::new(|context: &CommandContext<client_command_registration_callback::FabricClientCommandSource>| {
                let source = context.get_source();
                let text: MutableComponent = MuseumItemCache::try_resync(source);
                // rust: port gap (GAPS_batchD #17) - `sendFeedback(Component)`: the port's feedback is text-only, the
                // hover event of a failure message is dropped
                mcrust::mcrust::modding::script_host::send_feedback(&(constants::PREFIX.get_mut())().append(text).get_string());
                Ok(1)
            }));
            let mut museum = client_command_registration_callback::literal("museum");
            museum.then(&resync);
            let mut root = client_command_registration_callback::literal(crate::entry::skyblocker_mod::NAMESPACE);
            root.then(&museum);
            dispatcher.register(&root);
        }
        if !(crate::de::hysky::skyblocker::debug::debug::Debug::debug_enabled()) {
            return;
        }
        // java: dispatcher.register(literal(SkyblockerMod.NAMESPACE).then(literal("debug").then(literal("reloadMuseumItems")
        // java: 				.executes(ctx -> {
        // java: 					ctx.getSource().sendFeedback(Component.literal("Reloading..."));
        // java: 					loadMuseumItems();
        // java: 					ctx.getSource().sendFeedback(Component.literal("Reloaded!"));
        // java: 					return Command.SINGLE_SUCCESS;
        // java: 				}))
        // java: 		)
        // java: );
        {
            let mut reload = client_command_registration_callback::literal("reloadMuseumItems");
            reload.executes(Arc::new(|_ctx: &CommandContext<client_command_registration_callback::FabricClientCommandSource>| {
                mcrust::mcrust::modding::script_host::send_feedback("Reloading...");
                MuseumItemCache::load_museum_items();
                mcrust::mcrust::modding::script_host::send_feedback("Reloaded!");
                Ok(1)
            }));
            let mut debug = client_command_registration_callback::literal("debug");
            debug.then(&reload);
            let mut root = client_command_registration_callback::literal(crate::entry::skyblocker_mod::NAMESPACE);
            root.then(&debug);
            dispatcher.register(&root);
        }
    }

    /// java: public static void loadMuseumItems()
    pub fn load_museum_items() {
        NEURepoManager::run_async_after_load(Arc::new(move || {
            compat::lock(&ARMOR_NAMES).clear();
            compat::lock(&ARMOR_TO_ID).clear();
            compat::lock(&MAPPED_IDS).clear();
            MUSEUM_DONATIONS.get_mut().clear();
            compat::lock(&ORDERED_UPGRADES).clear();
            compat::set_state(&MUSEUM_CATEGORIES, Vec::new());
            let Some(mut file_path) = NEURepoManager::file(CONSTANTS_MUSEUM_DATA) else {
                return;
            };
            {
                // java: try (BufferedReader reader = Files.newBufferedReader(filePath.getFsPath())) {
                // rust: port gap (GAPS_batchH, GAPS_J #5) - `NEURepoFile.getFsPath()`: the NEU repo file is a
                // `crate::unresolved` placeholder with no path. The reader's text is what `JsonParser.parseReader` reads.
                let mut reader: String = todo!("fabric2seer: Files.newBufferedReader(filePath.getFsPath()) - NEURepoFile is unported");
                // java: try {..} catch (NoSuchFileException _) catch (IOException e) - an exception here is a panic, caught by `catch_unwind`
                let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    // java: JsonObject json = JsonParser.parseReader(reader).getAsJsonObject();
                    let mut json: serde_json::Map<String, serde_json::Value> = compat::json::get_as_json_object(&serde_json::from_str::<serde_json::Value>(&reader).expect("JsonSyntaxException"));
                    // (`asMap()` is Gson's insertion-ordered view: the maps stay `serde_json::Map`s, which keep that order)
                    // java: Map<String, JsonElement> setExceptions = json.get("set_exceptions").getAsJsonObject().asMap();
                    let mut set_exceptions: serde_json::Map<String, serde_json::Value> = compat::json::get_as_json_object(&json.get("set_exceptions").cloned().expect("java.lang.NullPointerException"));
                    // java: Map<String, JsonElement> mappedIds = json.get("mapped_ids").getAsJsonObject().asMap();
                    let mut mapped_ids: serde_json::Map<String, serde_json::Value> = compat::json::get_as_json_object(&json.get("mapped_ids").cloned().expect("java.lang.NullPointerException"));
                    // java: Map<String, JsonElement> itemToXp = json.get("itemToXp").getAsJsonObject().asMap();
                    let mut item_to_xp: serde_json::Map<String, serde_json::Value> = compat::json::get_as_json_object(&json.get("itemToXp").cloned().expect("java.lang.NullPointerException"));
                    // java: Map<String, JsonElement> setsToItems = json.get("sets_to_items").getAsJsonObject().asMap();
                    let mut sets_to_items: serde_json::Map<String, serde_json::Value> = compat::json::get_as_json_object(&json.get("sets_to_items").cloned().expect("java.lang.NullPointerException"));
                    // java: Map<String, JsonElement> children = json.get("children").getAsJsonObject().asMap();
                    let mut children: serde_json::Map<String, serde_json::Value> = compat::json::get_as_json_object(&json.get("children").cloned().expect("java.lang.NullPointerException"));
                    // java: Map<String, JsonElement> armorToId = json.get("armor_to_id").getAsJsonObject().asMap();
                    let mut armor_to_id: serde_json::Map<String, serde_json::Value> = compat::json::get_as_json_object(&json.get("armor_to_id").cloned().expect("java.lang.NullPointerException"));
                    for (s, json_element) in mapped_ids.iter() {
                        compat::lock(&MAPPED_IDS).insert(s.to_string(), compat::json::get_as_string(&json_element));
                    }
                    // java: Map<String, JsonElement> itemCategories = json.get("items").getAsJsonObject().asMap();
                    let mut item_categories: serde_json::Map<String, serde_json::Value> = compat::json::get_as_json_object(&json.get("items").cloned().expect("java.lang.NullPointerException"));
                    // java: MUSEUM_CATEGORIES = itemCategories.keySet();
                    compat::set_state(&MUSEUM_CATEGORIES, item_categories.keys().cloned().collect::<Vec<_>>());
                    for (category, elem) in item_categories.iter() {
                        let mut array: Vec<serde_json::Value> = compat::json::get_as_json_array(&elem);
                        // java: `return` inside itemCategories.forEach((category, elem) -> {..}) - the next entry
                        if category == "special" {
                            continue;
                        }
                        for element in array.iter() {
                            let mut item_id: String = compat::json::get_as_string(&element);
                            let mut set: Vec<(String, Option<compat::Shared<PriceData>>)> = Vec::new();
                            if armor_to_id.contains_key(&item_id) {
                                let mut is_equipment: bool = true;
                                for json_element in compat::json::get_as_json_array(&sets_to_items.get(&item_id).cloned().expect("java.lang.NullPointerException")).iter() {
                                    if is_equipment {
                                        is_equipment = MuseumUtils::is_equipment(&compat::json::get_as_string(&json_element));
                                    }
                                    set.push((compat::json::get_as_string(&json_element), None));
                                }
                                // java: String realId = setExceptions.entrySet().stream()
                                // java: 		.filter(e -> e.getValue().getAsString().equals(itemID))
                                // java: 		.map(Map.Entry::getKey)
                                // java: 		.findFirst()
                                // java: 		.orElse(itemID);
                                let mut real_id: String = set_exceptions.iter().filter(|(_, value)| compat::json::get_as_string(*value) == item_id).map(|(key, _)| key.clone()).next().unwrap_or_else(|| item_id.clone());
                                compat::lock(&ARMOR_NAMES).insert(item_id.clone(), MuseumUtils::format_armor_name(&real_id, is_equipment));
                            }
                            let mut item_xp: i32 = 0;
                            if item_to_xp.contains_key(&item_id) {
                                item_xp = compat::json::get_as_int(&item_to_xp.get(&item_id).cloned().unwrap());
                            }
                            let mut upgrades: Vec<String> = MuseumItemCache::get_upgrades(&children, &item_id);
                            if !upgrades.is_empty() {
                                // java: Optional<ObjectArrayList<String>> matchingUpgrade = ORDERED_UPGRADES.stream()
                                // java: 		.filter(orderedUpgrade -> orderedUpgrade.contains(itemID) || !Collections.disjoint(orderedUpgrade, upgrades))
                                // java: 		.findFirst();
                                // (the matching list is mutated IN `ORDERED_UPGRADES`: its index, not a copy)
                                let mut ordered_upgrades = compat::lock(&ORDERED_UPGRADES);
                                let mut matching_upgrade: Option<usize> = ordered_upgrades.iter().position(|ordered_upgrade| ordered_upgrade.contains(&item_id) || ordered_upgrade.iter().any(|e| upgrades.contains(e)));
                                if let Some(matching_upgrade) = matching_upgrade {
                                    let mut ordered_upgrade: &mut Vec<String> = &mut ordered_upgrades[matching_upgrade];
                                    if (ordered_upgrade.len() as i32) <= (upgrades.len() as i32) {
                                        ordered_upgrade.clear();
                                        ordered_upgrade.push(item_id.clone());
                                        ordered_upgrade.extend(upgrades.iter().cloned());
                                    }
                                } else {
                                    let mut new_upgrade: Vec<String> = Vec::new();
                                    new_upgrade.push(item_id.clone());
                                    new_upgrade.extend(upgrades.iter().cloned());
                                    ordered_upgrades.push(new_upgrade);
                                }
                            }
                            MUSEUM_DONATIONS.get_mut().push(Donation::new(category.to_string(), item_id.clone(), set.clone(), item_xp));
                        }
                    }
                    for donation in MUSEUM_DONATIONS.get_mut().iter() {
                        for list in compat::get_state(&ORDERED_UPGRADES) {
                            let mut armor_index: i32 = list.iter().position(|e| *e == Donation::get_id(donation)).map_or(-1, |i| i as i32);
                            if armor_index > 0 {
                                {
                                    let mut i: i32 = armor_index.wrapping_sub(1);
                                    while i >= 0 {
                                        Donation::add_downgrade(donation, list[(i) as usize].clone());
                                        i -= 1;
                                    }
                                }
                            }
                        }
                    }
                    for (set_id, display_id_object) in armor_to_id.iter() {
                        compat::lock(&ARMOR_TO_ID).insert(set_id.to_string(), compat::json::get_as_string(&display_id_object));
                    }
                    logger().info("[Skyblocker] Loaded museum data");
                }));
                if let Err(e) = __result {
                    // java: catch (NoSuchFileException _) {} catch (IOException e) { LOGGER.error("[Skyblocker] Failed to load donations data", e); }
                    // (the reader's IO errors are the gap above; any other exception propagates to `runAsyncAfterLoad`'s
                    // handler in Java - logged here)
                    logger().error(&format!("[Skyblocker] Failed to load donations data {}", compat::panic_message(&e)));
                }
            }
        }));
    }

    /// java: public static List<String> getUpgrades(Map<String, JsonElement> children, String item)
    /// (`children` is Gson's insertion-ordered `asMap()` view - a `serde_json::Map` keeps that order, which
    /// `findFirst` reads)
    pub fn get_upgrades(children: &serde_json::Map<String, serde_json::Value>, item: &str) -> Vec<String> {
        let mut upgrades: Vec<String> = Vec::new();
        let mut current_item: String = item.to_string();
        loop {
            let mut final_current_item: String = current_item.clone();
            // java: String parentItem = children.entrySet().stream().filter(e -> e.getValue().getAsString().equals(finalCurrentItem))
            //   .map(Map.Entry::getKey).findFirst().orElse(null);
            let parent_item: Option<String> = children.iter().filter(|(_, value)| compat::json::get_as_string(*value) == final_current_item).map(|(key, _)| key.clone()).next();
            let Some(parent_item) = parent_item else {
                break;
            };
            upgrades.push(parent_item.clone());
            current_item = parent_item.clone();
        }
        return upgrades;
    }

    /// java: public static List<Donation> getDonations()
    pub fn get_donations() -> Vec<compat::Shared<Donation>> {
        let mut uncontributed_items: Vec<compat::Shared<Donation>> = Vec::new();
        if ProfiledData::contains_key(MUSEUM_ITEM_CACHE.get_mut()) {
            // java: ObjectOpenHashSet<String> items = MUSEUM_ITEM_CACHE.get().collectedItemIds();
            let mut items: std::collections::HashSet<String> = ProfiledData::get(MUSEUM_ITEM_CACHE.get_mut()).expect("NullPointerException").collected_item_ids;
            for donation in MUSEUM_DONATIONS.get_mut().iter_mut() {
                if !(items.contains(&Donation::get_id(donation))) {
                    if Donation::is_set(donation) && items.iter().cloned().any(|i| Donation::get_set(donation).into_iter().any(|p| (p.0 == i))) {
                        continue;
                    }
                    Donation::set_price_data(donation);
                    uncontributed_items.push(donation.clone());
                }
            }
            for donation in uncontributed_items.iter() {
                Donation::set_discount(donation, Donation::get_downgrades(donation).into_iter().filter(|_| Donation::is_craftable(donation)).filter(|downgrade| !uncontributed_items.iter().cloned().any(|item| (Donation::get_id(&item) == *downgrade))).map(|downgrade| (downgrade.clone(), MuseumUtils::get_set_craft_cost(&downgrade))).next());
            }
            uncontributed_items.sort_by(|a, b| { let key = |__a0| Donation::get_id(__a0); compat::compare_to(&key(a), &key(b)).cmp(&0) });
        }
        return uncontributed_items;
    }

    /// java: public static void handleClick(Slot ignoredSlot, int slotId, NonNullList<Slot> slots)
    /// (`NonNullList<Slot> slots` is the menu's slot list, the port's `Vec<Slot>`: lent as a slice)
    pub fn handle_click(ignored_slot: &Slot, mut slot_id: i32, slots: &[Slot]) {
        if slot_id == CONFIRM_DONATION_BUTTON_SLOT {
            let mut profile_id: String = Utils::get_profile_id();
            if !profile_id.is_empty() {
                for i in 0..17 {
                    // java: ItemStack stack = slots.get(i).getItem(); - the port's slot reads its item through the open
                    // menu's containers (`compat::slot_item`)
                    let mut stack: mcrust::net::minecraft::world::item::item_stack::ItemStack = compat::slot_item(&slots[(i) as usize]);
                    if !stack.is_empty() {
                        let mut item_id: String = stack.get_skyblock_id();
                        if !item_id.is_empty() {
                            // java: ProfileMuseumData data = MUSEUM_ITEM_CACHE.computeIfAbsent(ProfileMuseumData.EMPTY);
                            // (the port's `computeIfAbsent` answers a COPY of the cached record: its set is changed on the
                            // copy below and the copy is put back, which is the cached object Java changes)
                            let mut data: ProfileMuseumData = ProfiledData::compute_if_absent(MUSEUM_ITEM_CACHE.get_mut(), PROFILE_MUSEUM_DATA_EMPTY.get_mut().clone());
                            if compat::lock(&MAPPED_IDS).contains_key(&item_id) {
                                item_id = compat::lock(&MAPPED_IDS).get(&item_id).cloned().unwrap();
                            }
                            let mut set_id: Option<String> = MuseumUtils::get_set_id(&item_id);
                            let mut donation: Option<compat::Shared<Donation>> = MuseumManager::get_donation((if set_id.is_some() { set_id.clone() } else { Some(item_id.clone()) }).as_deref().unwrap());
                            data.collected_item_ids.insert(item_id.clone());
                            if let Some(set_id) = &set_id {
                                data.collected_item_ids.insert(set_id.to_string());
                            }
                            if donation.is_some() && !(Donation::get_downgrades(donation.as_ref().unwrap()).is_empty()) {
                                for downgrade in Donation::get_downgrades(donation.as_ref().unwrap()) {
                                    if Donation::is_set(donation.as_ref().unwrap()) {
                                        let mut pieces: Vec<String> = MuseumUtils::get_pieces_by_set_id(&downgrade);
                                        data.collected_item_ids.extend(pieces.iter().cloned());
                                    }
                                    data.collected_item_ids.insert(downgrade.clone());
                                }
                            }
                            ProfiledData::put(MUSEUM_ITEM_CACHE.get_mut(), data);
                        }
                    }
                }
                MUSEUM_ITEM_CACHE.get_mut().lock().borrow().save();
            }
        }
    }

    /// java: private static void updateData4ProfileMember(UUID uuid, String profileId, @Nullable FabricClientCommandSource source)
    ///
    /// The request and its JSON run on the worker (`VIRTUAL_THREAD_EXECUTOR`); the cache, the donations and the
    /// feedback are render-thread statics (`compat::MainThread`), so the rest of the try block runs there
    /// (`game::execute`), in Java's order (batch B pattern 27).
    pub fn update_data4_profile_member(mut uuid: Uuid, mut profile_id: String, mut source: Option<client_command_registration_callback::FabricClientCommandSource>) {
        if MUSEUM_DONATIONS.get_mut().is_empty() {
            return;
        }
        // (only `source != null` is read of the source - the feedback goes through the port's command feedback)
        let has_source: bool = source.is_some();
        drop(source);
        CompletableFuture::<()>::run_async(move || {
            // java: try (ApiResponse response = Http.sendHypixelRequest("skyblock/museum", "?profile=" + profileId)) {
            // java: 	if (response.ok()) {
            // java: 		JsonObject profileData = JsonParser.parseString(response.content()).getAsJsonObject();
            // java: 		JsonObject members = profileData.getAsJsonObject("members");
            // java: 		String uuidString = UndashedUuid.toString(uuid);
            // java: 		if (members.has(uuidString)) {
            // java: 			JsonObject memberData = members.get(uuidString).getAsJsonObject();
            // java: 			Map<String, JsonElement> donatedSets = memberData.get("items").getAsJsonObject().asMap();
            // the worker's part: `Ok(Ok(Some(donatedSets)))` / `Ok(Ok(None))` (no member) / `Ok(Err(response))` (non-200) /
            // `Err(exception)`
            let worker: Result<Result<Option<serde_json::Map<String, serde_json::Value>>, crate::de::hysky::skyblocker::utils::http::ApiResponse>, String> = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let mut response: crate::de::hysky::skyblocker::utils::http::ApiResponse = crate::de::hysky::skyblocker::utils::http::Http::send_hypixel_request("skyblock/museum", &format!("?profile={}", profile_id));
                if response.ok() {
                    let mut profile_data: serde_json::Map<String, serde_json::Value> = compat::json::get_as_json_object(&serde_json::from_str::<serde_json::Value>(&response.content()).expect("JsonSyntaxException"));
                    let mut members: serde_json::Map<String, serde_json::Value> = compat::json::get_as_json_object_of(&profile_data, "members");
                    let mut uuid_string: String = undashed_uuid::to_string(&uuid);
                    if compat::json::has(&members, &uuid_string) {
                        let mut member_data: serde_json::Map<String, serde_json::Value> = compat::json::get_as_json_object(&members.get(&uuid_string).cloned().unwrap());
                        let mut donated_sets: serde_json::Map<String, serde_json::Value> = compat::json::get_as_json_object(&member_data.get("items").cloned().expect("java.lang.NullPointerException"));
                        Ok(Some(donated_sets))
                    } else {
                        Ok(None)
                    }
                } else {
                    Err(response)
                }
            })).map_err(|e| compat::panic_message(&e));
            crate::compat::execute(move || {
                let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    match worker {
                        Ok(Ok(Some(donated_sets))) => {
                            let mut item_ids: std::collections::HashSet<String> = std::collections::HashSet::new();
                            for (s, _) in donated_sets.iter() {
                                let mut donation: Option<compat::Shared<Donation>> = MUSEUM_DONATIONS.get_mut().iter().cloned().filter(|d| (Donation::get_id(d) == *s)).next();
                                if let Some(value) = donation.clone() {
                                    item_ids.extend(Donation::get_downgrades(&value).into_iter());
                                }
                                if donation.is_some() {
                                    if Donation::is_set(&donation.clone().unwrap()) {
                                        item_ids.extend(Donation::get_set(&donation.clone().unwrap()).into_iter().map(|__a0| __a0.0.clone()).collect::<Vec<_>>().into_iter());
                                        for downgrade in Donation::get_downgrades(&donation.clone().unwrap()) {
                                            item_ids.extend(MuseumUtils::get_pieces_by_set_id(&downgrade).into_iter());
                                        }
                                    } else {
                                        item_ids.insert(Donation::get_id(&donation.clone().unwrap()).replace("STARRED_", ""));
                                        item_ids.extend(Donation::get_downgrades(&donation.clone().unwrap()).into_iter());
                                    }
                                }
                            }
                            ProfiledData::put_3(MUSEUM_ITEM_CACHE.get_mut(), &uuid, profile_id.clone(), ProfileMuseumData::new(compat::millis(), item_ids.clone(), CURRENT_DATA_VERSION));
                            MUSEUM_ITEM_CACHE.get_mut().lock().borrow().save();
                            if has_source {
                                // java: source.sendFeedback(Constants.PREFIX.get().append(Component.translatable("skyblocker.museum.resyncSuccess")));
                                mcrust::mcrust::modding::script_host::send_feedback(&(constants::PREFIX.get_mut())().append(component::translatable("skyblocker.museum.resyncSuccess")).get_string());
                            }
                            logger().info(&format!("[Skyblocker] Successfully updated museum item cache for profile {}", profile_id));
                        }
                        Ok(Ok(None)) => {
                            // If the player's Museum API is disabled
                            MuseumItemCache::put_empty(uuid.clone(), profile_id.clone());
                            if has_source {
                                // java: source.sendFeedback(Constants.PREFIX.get().append(Component.translatable("skyblocker.museum.resyncFailure", Component.translatable("skyblocker.museum.resyncFailure.apiDisabled")))
                                // java: 		.withStyle(style -> style.withHoverEvent(new HoverEvent.ShowText(Component.translatable("skyblocker.museum.resyncFailure.apiDisabled.@Tooltip")))));
                                // rust: port gap (GAPS_batchD #17) - the feedback is text-only: the hover event is dropped
                                mcrust::mcrust::modding::script_host::send_feedback(&(constants::PREFIX.get_mut())().append(component::translatable_args("skyblocker.museum.resyncFailure", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::translatable("skyblocker.museum.resyncFailure.apiDisabled"))])).get_string());
                            }
                            // java: LOGGER.warn(ERROR_LOG_TEMPLATE + " because the Museum API is disabled!", profileId);
                            logger().warn(&format!("[Skyblocker] Failed to refresh museum item data for profile {} because the Museum API is disabled!", profile_id));
                        }
                        Ok(Err(response)) => {
                            // If the request returns a non 200 status code
                            MuseumItemCache::put_empty(uuid.clone(), profile_id.clone());
                            if has_source {
                                mcrust::mcrust::modding::script_host::send_feedback(&(constants::PREFIX.get_mut())().append(component::translatable_args("skyblocker.museum.resyncFailure", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::translatable("skyblocker.museum.resyncFailure.unknownError"))])).get_string());
                            }
                            // java: LOGGER.error(ERROR_LOG_TEMPLATE + " because a non 200 status code was encountered! Response: {}", profileId, response);
                            // (`response` is the record's toString)
                            logger().error(&format!("[Skyblocker] Failed to refresh museum item data for profile {} because a non 200 status code was encountered! Response: ApiResponse[content={}, statusCode={}, cacheStatuses=[{}], age={}]", profile_id, response.content, response.status_code, response.cache_statuses.join(", "), response.age));
                        }
                        Err(exception) => {
                            panic!("{}", exception);
                        }
                    }
                }));
                if let Err(e) = __result {
                    // java: catch (Exception e) - If an exception was somehow thrown
                    MuseumItemCache::put_empty(uuid.clone(), profile_id.clone());
                    if has_source {
                        mcrust::mcrust::modding::script_host::send_feedback(&(constants::PREFIX.get_mut())().append(component::translatable_args("skyblocker.museum.resyncFailure", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::translatable("skyblocker.museum.resyncFailure.unknownError"))])).get_string());
                    }
                    // java: LOGGER.error(ERROR_LOG_TEMPLATE, profileId, e);
                    logger().error(&format!("[Skyblocker] Failed to refresh museum item data for profile {} {}", profile_id, compat::panic_message(&e)));
                }
            });
        }, compat::executor);
    }

    /// java: private static void putEmpty(UUID uuid, String profileId)
    pub fn put_empty(mut uuid: Uuid, mut profile_id: String) {
        ProfiledData::compute_if_absent_3(MUSEUM_ITEM_CACHE.get_mut(), &uuid, profile_id.clone(), Arc::new(move || ProfileMuseumData::new(compat::millis(), std::collections::HashSet::from([]), CURRENT_DATA_VERSION)));
        MUSEUM_ITEM_CACHE.get_mut().lock().borrow().save();
    }

    /// java: private static Component tryResync(FabricClientCommandSource source)
    pub fn try_resync(source: &client_command_registration_callback::FabricClientCommandSource) -> MutableComponent {
        let mut profile_id: String = Utils::get_profile_id();
        if !(MUSEUM_ITEM_CACHE.get_mut().lock().borrow().is_loaded()) || profile_id.is_empty() {
            return (component::translatable_args("skyblocker.museum.resyncFailure", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::translatable("skyblocker.museum.resyncFailure.profile"))])).with_style_fn(|style| style.with_hover_event(Some(HoverEvent::new_show_text(mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::translatable("skyblocker.museum.resyncFailure.profile.@Tooltip"))))));
        }
        if MUSEUM_DONATIONS.get_mut().is_empty() {
            return (component::translatable_args("skyblocker.museum.resyncFailure", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::translatable("skyblocker.museum.resyncFailure.itemRepo"))])).with_style_fn(|style| style.with_hover_event(Some(HoverEvent::new_show_text(mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::translatable("skyblocker.museum.resyncFailure.itemRepo.@Tooltip"))))));
        }
        // java: if (MUSEUM_ITEM_CACHE.containsKey() && !MUSEUM_ITEM_CACHE.get().canResync()) return Component.translatable("skyblocker.museum.cannotResync");
        if ProfiledData::contains_key(MUSEUM_ITEM_CACHE.get_mut()) && !ProfiledData::get(MUSEUM_ITEM_CACHE.get_mut()).expect("NullPointerException").can_resync() {
            return component::translatable("skyblocker.museum.cannotResync");
        }
        MuseumItemCache::update_data4_profile_member(Utils::get_uuid(), profile_id.clone(), Some(source.clone()));
        return component::translatable("skyblocker.museum.attemptingResync");
    }

    /// java: public static void onProfileChange()
    pub fn on_profile_change() {
        let mut uuid: Uuid = Utils::get_uuid();
        // java: MUSEUM_ITEM_CACHE.isLoaded() && (!MUSEUM_ITEM_CACHE.containsKey() || MUSEUM_ITEM_CACHE.get().needsUpdate())
        if MUSEUM_ITEM_CACHE.get_mut().lock().borrow().is_loaded() && (!ProfiledData::contains_key(MUSEUM_ITEM_CACHE.get_mut()) || ProfiledData::get(MUSEUM_ITEM_CACHE.get_mut()).expect("NullPointerException").needs_update()) {
            MuseumItemCache::update_data4_profile_member(uuid.clone(), Utils::get_profile_id(), None);
        }
    }

    /// java: public static boolean hasItemInMuseum(String id)
    pub fn has_item_in_museum(id: &str) -> bool {
        // java: return MUSEUM_ITEM_CACHE.containsKey() && MUSEUM_ITEM_CACHE.get().collectedItemIds().contains(id.replace("STARRED_", ""));
        return ProfiledData::contains_key(MUSEUM_ITEM_CACHE.get_mut()) && ProfiledData::get(MUSEUM_ITEM_CACHE.get_mut()).expect("NullPointerException").collected_item_ids.contains(&id.replace("STARRED_", ""));
    }

}

/// java: private record ProfileMuseumData(long lastResync, ObjectOpenHashSet<String> collectedItemIds, int dataVersion)
#[derive(Clone, Debug, PartialEq)]
pub struct ProfileMuseumData {
    /// java: long lastResync
    pub last_resync: i64,
    /// java: ObjectOpenHashSet<String> collectedItemIds
    pub collected_item_ids: std::collections::HashSet<String>,
    /// java: int dataVersion
    pub data_version: i32,
}

impl ProfileMuseumData {
    /// java: the canonical record constructor
    pub fn new(mut last_resync: i64, mut collected_item_ids: std::collections::HashSet<String>, mut data_version: i32) -> Self {
        Self { last_resync, collected_item_ids, data_version }
    }

    /// java: the record accessor `lastResync()`
    pub fn last_resync(&self) -> i64 {
        self.last_resync
    }

    /// java: the record accessor `collectedItemIds()`
    pub fn collected_item_ids(&self) -> std::collections::HashSet<String> {
        self.collected_item_ids.clone()
    }

    /// java: the record accessor `dataVersion()`
    pub fn data_version(&self) -> i32 {
        self.data_version
    }

    /// java: private boolean canResync()
    pub fn can_resync(&self) -> bool {
        return (self.last_resync.wrapping_add(PROFILE_MUSEUM_DATA_TIME_BETWEEN_RESYNCING_ALLOWED) < compat::millis());
    }

    /// java: private boolean needsUpdate()
    pub fn needs_update(&self) -> bool {
        return (self.data_version < CURRENT_DATA_VERSION);
    }

}

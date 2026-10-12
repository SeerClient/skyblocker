//! Ported from de.hysky.skyblocker.skyblock.itemlist.ItemRepository by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::compatibility::jei::jei_compatibility;
use crate::de::hysky::skyblocker::events::skyblock_events;
use crate::de::hysky::skyblocker::skyblock::itemlist::item_stack_builder::ItemStackBuilder;
use crate::de::hysky::skyblocker::skyblock::itemlist::recipes::recipe_item_stack_cache;
use crate::de::hysky::skyblocker::skyblock::itemlist::recipes::skyblock_recipe::SkyblockRecipe;
use crate::de::hysky::skyblocker::skyblock::itemlist::stack_overlays::StackOverlays;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use crate::de::hysky::skyblocker::utils::item_utils::ItemUtils;
use crate::de::hysky::skyblocker::utils::neu_repo_manager::NEURepoManager;
use mcrust::java::util::concurrent::completable_future::CompletableFuture;
use mcrust::net::minecraft::client::minecraft;
use mcrust::net::minecraft::network::protocol::game::clientbound_update_recipes_packet::ClientboundUpdateRecipesPacket;
use mcrust::net::minecraft::world::item::crafting::selectable_recipe::SelectableRecipe;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;
use mcrust::net::minecraft::network::protocol::game::client_game_packet_listener::ClientGamePacketListener as _;

/// java: private static final List<FlexibleItemStack> items = new ArrayList<>();
pub static ITEMS: compat::MainThread<Vec<compat::Shared<FlexibleItemStack>>> = compat::MainThread::new(|| Vec::new());
/// java: private static final Map<String, FlexibleItemStack> itemsMap = new HashMap<>();
pub static ITEMS_MAP: compat::MainThread<std::collections::HashMap<String, compat::Shared<FlexibleItemStack>>> = compat::MainThread::new(|| std::collections::HashMap::new());
/// java: private static final List<SkyblockRecipe> recipes = new ArrayList<>();
pub static RECIPES: compat::MainThread<Vec<compat::Shared<dyn SkyblockRecipe>>> = compat::MainThread::new(|| Vec::new());
/// java: private static final HashMap<String, @NEUId String> bazaarStocks = new HashMap<>();
pub static BAZAAR_STOCKS: LazyLock<Mutex<std::collections::HashMap<String, String>>> = LazyLock::new(|| Mutex::new(std::collections::HashMap::new()));
/// java: private static final List<AfterImportTask> afterImportTasks = new CopyOnWriteArrayList<>();
pub static AFTER_IMPORT_TASKS: compat::MainThread<Vec<AfterImportTask>> = compat::MainThread::new(|| Vec::new());
/// java: private static boolean itemsImported = false;
pub static ITEMS_IMPORTED: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));
/// java: private static boolean filesImported = false;
pub static FILES_IMPORTED: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));

/// java: public class ItemRepository
#[derive(Clone)]
pub struct ItemRepository {}

impl ItemRepository {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        NEURepoManager::run_async_after_load(Arc::new(move || { ItemStackBuilder::load_pet_nums(); }));
        NEURepoManager::run_async_after_load(Arc::new(move || { ItemRepository::import_item_files(); }));
        NEURepoManager::run_async_after_load(Arc::new(move || { ItemRepository::load_bazaar_stocks(); }));
        ItemRepository::run_async_after_import(Arc::new(move || { ItemRepository::handle_recipe_synchronization(); }));
        skyblock_events::JOIN.register(Arc::new(move || { ItemRepository::handle_recipe_synchronization(); }));
    }

    /// java: private static void handleRecipeSynchronization()
    pub fn handle_recipe_synchronization() {
        let client = compat::minecraft();
        if mcrust::mcrust::modding::game::level().is_none() || minecraft::get_connection(client).is_none() {
            return;
        }
        // TODO(fabric2seer): `.empty(..)` on a value of unknown type
        //   java: ClientboundUpdateRecipesPacket packet = new ClientboundUpdateRecipesPacket(Map.of(), SelectableRecipe.SingleInputSet.empty());
        let mut packet: ClientboundUpdateRecipesPacket = todo!("fabric2seer: the initialiser of `packet` (TODO above)");
        // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            mcrust::mcrust::modding::game::execute(move || {
                minecraft::get_connection(client).unwrap().handle_update_recipes(&packet);
                if *jei_compatibility::JEI_LOADED {
                    crate::entry::skyblocker_jei_plugin::trick_jei_into_loading_recipes();
                }
            });
        }));
        if let Err(e) = __result {
            logger().info(&format!("[Skyblocker Item Repo] recipe sync error"));
        }
    }

    /// java: private static void importItemFiles()
    pub fn import_item_files() {
        compat::set_state(&ITEMS_IMPORTED, false);
        compat::set_state(&FILES_IMPORTED, false);
        ITEMS.get_mut().clear();
        ITEMS_MAP.get_mut().clear();
        RECIPES.get_mut().clear();
        StackOverlays::load_overlays();
        NEURepoManager::for_each_item(Arc::new(move |__a0| { ItemRepository::load_item(&__a0); }));
        StackOverlays::clean_up_overlays();
        ITEMS.get_mut().sort_by(|a, b| (|a, b| (|a, b| { let key = |stack| compat::regex::replace_all(&stack.lock().borrow().get_skyblock_id(), ".\\d+$", ""); key(a).cmp(&key(b)) })(a, b).then_with(|| (|a, b| { let key = |stack| compat::str_len(&stack.lock().borrow().get_skyblock_id()); key(a).cmp(&key(b)) })(a, b)))(a, b).then_with(|| (|a, b| { let key = |__a0| __a0.lock().borrow().get_skyblock_id(); key(a).cmp(&key(b)) })(a, b)));
        compat::set_state(&ITEMS_IMPORTED, true);
        NEURepoManager::for_each_item(Arc::new(move |__a0| { ItemRepository::load_recipes(&__a0); }));
        recipe_item_stack_cache::CACHE.get_mut().clear();
        compat::set_state(&FILES_IMPORTED, true);
        for task in AFTER_IMPORT_TASKS.get_mut().iter() {
            // TODO(fabric2seer): CompletableFuture.runAsync of a non-lambda
            //   java: CompletableFuture.runAsync(..).exceptionally(..);
        }
    }

    /// java: private static void loadItem(NEUItem item)
    pub fn load_item(item: &crate::unresolved::NEUItem) {
        // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut stack: compat::Shared<FlexibleItemStack> = ItemStackBuilder::from_neu_item(item);
            StackOverlays::apply_overlay(item, &stack);
            // TODO(fabric2seer): the condition: `is` is not a method of FlexibleItemStack
            //   java: stack.is(Items.ENCHANTED_BOOK) && stack.getSkyblockId().contains(";")
            if todo!("fabric2seer: the condition (TODO above)") {
                ItemUtils::get_custom_data(&stack).put_string("id", "ENCHANTED_BOOK");
            }
            ITEMS.get_mut().push(stack.clone());
            // TODO(fabric2seer): `.getSkyblockItemId(..)` on a NEUItem
            //   java: itemsMap.put(item.getSkyblockItemId(), stack);
        }));
        if let Err(e) = __result {
            // TODO(fabric2seer): `.getSkyblockItemId(..)` on a NEUItem
            //   java: LOGGER.error("[Skyblocker Item Repo Loader] Failed to load item, please report this! Skyblock Id: {}", item.getSkyblockItemId(), e);
        }
    }

    /// java: private static void loadRecipes(NEUItem item)
    pub fn load_recipes(item: &crate::unresolved::NEUItem) {
        // TODO(fabric2seer): `.getRecipes(..)` on a NEUItem
        //   java: item.getRecipes().stream().map(ItemRepository::toSkyblockRecipe).filter(Objects::nonNull).forEach(recipes::add);
    }

    /// java: private static void loadBazaarStocks()
    pub fn load_bazaar_stocks() {
        compat::lock(&BAZAAR_STOCKS).clear();
        // TODO(fabric2seer): `.getBazaarStocks(..)` on a NEUConstants
        //   java: NEURepoManager.getConstants().getBazaarStocks().getStocks().forEach((String neuId, String skyblockId) -> bazaarStocks.put(skyblockId, neuId));
    }

    /// java: public static @Nullable String getWikiLink(String neuId)
    pub fn get_wiki_link(neu_id: &str) -> Option<String> {
        let mut item: Option<crate::unresolved::NEUItem> = NEURepoManager::get_item_by_neu_id(neu_id);
        // TODO(fabric2seer): the condition: `.getInfo(..)` on a NEUItem
        //   java: item == null || item.getInfo() == null || item.getInfo().isEmpty()
        if todo!("fabric2seer: the condition (TODO above)") {
            return None;
        }
        // TODO(fabric2seer): `.getInfo(..)` on a NEUItem
        //   java: List<String> info = item.getInfo();
        let mut info: Vec<String> = todo!("fabric2seer: the initialiser of `info` (TODO above)");
        let mut wiki_link0: String = info[0].clone();
        let mut wiki_link1: String = (if (info.len() as i32) > 1 { info[(1) as usize].clone() } else { "".to_string() });
        let mut wiki_domain: String = ItemRepository::get_wiki_link_0();
        if wiki_link0.starts_with(&wiki_domain) {
            return Some(wiki_link0);
        } else if wiki_link1.starts_with(&wiki_domain) {
            return Some(wiki_link1);
        }
        return None;
    }

    /// java: public static String getWikiLink()
    pub fn get_wiki_link_0() -> String {
        return "https://hypixelskyblock.minecraft.wiki".to_string();
    }

    /// java: public static List<SkyblockRecipe> getRecipesAndUsages(ItemStack stack)
    pub fn get_recipes_and_usages(stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> Vec<compat::Shared<dyn SkyblockRecipe>> {
        return ItemRepository::get_recipes(stack).chain(ItemRepository::get_usages(stack)).collect::<Vec<_>>();
    }

    /// java: public static boolean filesImported()
    pub fn files_imported() -> bool {
        return compat::get_state(&FILES_IMPORTED);
    }

    /// java: public static List<FlexibleItemStack> getItems()
    pub fn get_items() -> Vec<compat::Shared<FlexibleItemStack>> {
        return (if compat::get_state(&ITEMS_IMPORTED) { ITEMS.get_mut().clone() } else { vec![] });
    }

    /// java: public static Stream<FlexibleItemStack> getItemsStream()
    pub fn get_items_stream() -> Box<dyn Iterator<Item = compat::Shared<FlexibleItemStack>>> {
        return Box::new((if compat::get_state(&ITEMS_IMPORTED) { ITEMS.get_mut().iter().cloned() } else { std::iter::empty() }));
    }

    /// java: public static Map<String, @NEUId String> getBazaarStocks()
    pub fn get_bazaar_stocks() -> std::collections::HashMap<String, String> {
        return compat::get_state(&BAZAAR_STOCKS);
    }

    /// java: public static @Nullable FlexibleItemStack getItemStack(String neuId)
    pub fn get_item_stack(neu_id: &str) -> Option<compat::Shared<FlexibleItemStack>> {
        return (if compat::get_state(&ITEMS_IMPORTED) { ITEMS_MAP.get_mut().get(neu_id).cloned() } else { None });
    }

    /// java: @Contract("_, !null -> !null") public static FlexibleItemStack getItemStack(String neuId, FlexibleItemStack defaultStack)
    pub fn get_item_stack_2(neu_id: &str, default_stack: &compat::Shared<FlexibleItemStack>) -> compat::Shared<FlexibleItemStack> {
        let mut stack: Option<compat::Shared<FlexibleItemStack>> = ItemRepository::get_item_stack(neu_id);
        return (if stack.is_some() { stack.clone() } else { Some(default_stack.clone()) }).unwrap();
    }

    /// java: public static Supplier<FlexibleItemStack> getItemStackSupplier(String neuId)
    pub fn get_item_stack_supplier(neu_id: &str) -> Arc<dyn Fn() -> compat::Shared<FlexibleItemStack>> {
        return Arc::new(move || ITEMS_MAP.get_mut().get(neu_id).cloned());
    }

    /// java: public static Stream<SkyblockRecipe> getRecipesStream()
    pub fn get_recipes_stream() -> Box<dyn Iterator<Item = compat::Shared<dyn SkyblockRecipe>>> {
        return Box::new((if compat::get_state(&FILES_IMPORTED) { RECIPES.get_mut().iter().cloned() } else { std::iter::empty() }));
    }

    /// java: public static Stream<SkyblockRecipe> getRecipes(ItemStack stack)
    pub fn get_recipes(stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> Box<dyn Iterator<Item = compat::Shared<dyn SkyblockRecipe>> + '_> {
        return Box::new(NEURepoManager::get_recipes().get(&stack.get_neu_name()).cloned().unwrap_or_else(|| std::collections::HashSet::from([])).into_iter().map(|__a0| ItemRepository::to_skyblock_recipe(&__a0)).filter(|__a0| true));
    }

    /// java: public static Stream<SkyblockRecipe> getUsages(ItemStack stack)
    pub fn get_usages(stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> Box<dyn Iterator<Item = compat::Shared<dyn SkyblockRecipe>> + '_> {
        return Box::new(NEURepoManager::get_usages().get(&stack.get_neu_name()).cloned().unwrap_or_else(|| std::collections::HashSet::from([])).into_iter().map(|__a0| ItemRepository::to_skyblock_recipe(&__a0)).filter(|__a0| true));
    }

    /// java: private static @Nullable SkyblockRecipe toSkyblockRecipe(NEURecipe neuRecipe)
    pub fn to_skyblock_recipe(neu_recipe: &crate::unresolved::NEURecipe) -> Option<compat::Shared<dyn SkyblockRecipe>> {
        // TODO(fabric2seer): a pattern switch expression
        //   java: return switch (neuRecipe) {
        //   java: 			case NEUCraftingRecipe craftingRecipe -> new SkyblockCraftingRecipe(craftingRecipe);
        //   java: 			case NEUForgeRecipe forgeRecipe -> new SkyblockForgeRecipe(forgeRecipe);
        //   java: 			case NEUNpcShopRecipe shopRecipe -> new SkyblockNpcShopRecipe(shopRecipe);
        //   java: 			case NEUKatUpgradeRecipe katUpgradeRecipe -> new SkyblockKatUpgradeRecipe(katUpgradeRecipe);
        //   java: 			case null, default -> null;
        //   java: 		};
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public static void runAsyncAfterImport(Runnable runnable)
    pub fn run_async_after_import(runnable: Arc<dyn Fn()>) {
        ItemRepository::run_after_import(runnable.clone(), true);
    }

    /// java: public static void runAfterImport(Runnable runnable, boolean async)
    pub fn run_after_import(runnable: Arc<dyn Fn()>, mut r#async: bool) {
        if compat::get_state(&FILES_IMPORTED) {
            if r#async {
                // TODO(fabric2seer): CompletableFuture.runAsync of a non-lambda
                //   java: CompletableFuture.runAsync(runnable, SkyblockerMod.VIRTUAL_THREAD_EXECUTOR).exceptionally(e -> {
                //   java: 					LOGGER.error("[Skyblocker Item Repo Loader] Encountered unknown exception while running after import task", e);
                //   java: 					return null;
                //   java: 				});
            } else {
                // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
                let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    (runnable)();
                }));
                if let Err(e) = __result {
                    logger().error(&format!("[Skyblocker Item Repo Loader] Encountered unknown exception while running after import task"));
                }
            }
        }
        AFTER_IMPORT_TASKS.get_mut().push(AfterImportTask::new(runnable.clone(), r#async));
    }

}

/// java: private record AfterImportTask(Runnable runnable, boolean async)
#[derive(Clone, Debug)]
pub struct AfterImportTask {
    /// java: Runnable runnable
    pub runnable: Arc<dyn Fn()>,
    /// java: boolean async
    pub r#async: bool,
}

impl AfterImportTask {
    /// java: the canonical record constructor
    pub fn new(mut runnable: Arc<dyn Fn()>, mut r#async: bool) -> Self {
        Self { runnable, r#async }
    }

    /// java: the record accessor `runnable()`
    pub fn runnable(&self) -> Arc<dyn Fn()> {
        self.runnable.clone()
    }

    /// java: the record accessor `async()`
    pub fn r#async(&self) -> bool {
        self.r#async
    }

}

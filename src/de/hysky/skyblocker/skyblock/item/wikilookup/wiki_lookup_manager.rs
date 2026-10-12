//! Ported from de.hysky.skyblocker.skyblock.item.wikilookup.WikiLookupManager by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::item::wikilookup::enchantment_book_item_lookup;
use crate::de::hysky::skyblocker::skyblock::item::wikilookup::event_calendar_lookup;
use crate::de::hysky::skyblocker::skyblock::item::wikilookup::pet_item_lookup;
use crate::de::hysky::skyblocker::skyblock::item::wikilookup::regular_item_lookup;
use crate::de::hysky::skyblocker::skyblock::item::wikilookup::visitor_lookup;
use crate::de::hysky::skyblocker::skyblock::item::wikilookup::wiki_lookup::WikiLookup;
use crate::de::hysky::skyblocker::skyblock::itemlist::item_repository::ItemRepository;
use crate::de::hysky::skyblocker::utils::constants;
use crate::entry::skyblocker_mod;
use mcrust::com::mojang::datafixers::util::either::Either;
use mcrust::java::text::decimal_format_symbols::Locale;
use mcrust::java::util::concurrent::completable_future::CompletableFuture;
use mcrust::net::minecraft::client::key_mapping;
use mcrust::net::minecraft::util::util;
use mcrust::net::minecraft::world::entity::player::player::Player;
use mcrust::net::minecraft::world::inventory::slot::Slot;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: public static KeyMapping independentWikiLookup;
pub static INDEPENDENT_WIKI_LOOKUP: compat::MainThread<Option<mcrust::net::minecraft::client::key_mapping::KeyMapping>> = compat::MainThread::new(|| None);
/// java: private static final WikiLookup[] LOOKUPS = new WikiLookup[] { VisitorLookup.INSTANCE, PetItemLookup.INSTANCE, EnchantmentBookItemLookup.INSTANCE, EventCalendarLookup.INSTANCE, // Always in the last R
pub static LOOKUPS: compat::MainThread<Vec<compat::Shared<dyn WikiLookup>>> = compat::MainThread::new(|| vec![(visitor_lookup::INSTANCE.clone() as compat::Shared<dyn WikiLookup>), (pet_item_lookup::INSTANCE.clone() as compat::Shared<dyn WikiLookup>), (enchantment_book_item_lookup::INSTANCE.clone() as compat::Shared<dyn WikiLookup>), (event_calendar_lookup::INSTANCE.clone() as compat::Shared<dyn WikiLookup>), (regular_item_lookup::INSTANCE.clone() as compat::Shared<dyn WikiLookup>)]);

/// java: public final class WikiLookupManager
#[derive(Clone)]
pub struct WikiLookupManager {}

impl WikiLookupManager {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        key_binding_helper::register_key_binding(KeyMapping::new("key.skyblocker.wikiLookup.independent", 290, Category::Misc));
    }

    /// java: public static String getKeysText()
    pub fn get_keys_text() -> String {
        // TODO(fabric2seer): `get_translated_key_message` takes 2 argument(s) in the port (`name: &str, key_name: &dyn Fn(i32, i32) -> Option<String>`), Java passes 0
        // rust: let mut independent: Option<String> = (if INDEPENDENT_WIKI_LOOKUP.get_mut().as_ref().unwrap().is_unbound() { None } else { Some(compat::component_of(key_mapping::get_translated_key_message()).get_string()) });
        let mut independent: String = todo!("fabric2seer: the initialiser of `independent` (TODO above)");
        if false /* java: independent == null: never null here */ {
            return "".to_string();
        }
        return independent.to_uppercase();
    }

    /// java: public static boolean handleWikiLookup(Either<Slot, ItemStack> either, Player player, KeyEvent input)
    pub fn handle_wiki_lookup(either: &Either<Slot, mcrust::net::minecraft::world::item::item_stack::ItemStack>, player: &dyn Player, input: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        return WikiLookupManager::handle_wiki_lookup_4(None, either, player, input);
    }

    /// java: public static boolean handleWikiLookup(@Nullable String title, Either<Slot, ItemStack> either, Player player, KeyEvent input)
    pub fn handle_wiki_lookup_4(mut title: Option<String>, either: &Either<Slot, mcrust::net::minecraft::world::item::item_stack::ItemStack>, player: &dyn Player, input: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        if SkyblockerConfigManager::get().general.wiki_lookup.enable_wiki_lookup {
            if INDEPENDENT_WIKI_LOOKUP.get_mut().as_ref().unwrap().matches(input) {
                WikiLookupManager::open_wiki_3(title.clone(), either, player);
                return true;
            }
        }
        return false;
    }

    /// java: public static void openWiki(ItemStack itemStack, Player player)
    pub fn open_wiki(item_stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, player: &dyn Player) {
        WikiLookupManager::open_wiki_3(None, &Either::right(item_stack.clone()), player);
    }

    /// java: public static void openWiki(@Nullable String title, Either<Slot, ItemStack> either, Player player)
    pub fn open_wiki_3(mut title: Option<String>, either: &Either<Slot, mcrust::net::minecraft::world::item::item_stack::ItemStack>, player: &dyn Player) {
        for lookup in LOOKUPS.get_mut().iter() {
            if { let __f = compat::dispatch(lookup, |__t| __t.can_search()); __f(lookup, title.clone(), either) } {
                let mut item_stack: mcrust::net::minecraft::world::item::item_stack::ItemStack = WikiLookupManager::map_either_to_item_stack(either);
                ({ let __f = compat::dispatch(lookup, |__t| __t.open()); __f(lookup, &item_stack, player) });
                break;
            }
        }
    }

    /// java: public static void openWikiLinkName(String name, Player player)
    pub fn open_wiki_link_name(name: &str, player: &dyn Player) {
        let mut wiki_link: String = format!("{}/{}", ItemRepository::get_wiki_link_0(), name);
        WikiLookupManager::open_wiki_link(&wiki_link, player);
    }

    /// java: public static void openWikiLink(String wikiLink, Player player)
    pub fn open_wiki_link(wiki_link: &str, player: &dyn Player) {
        (CompletableFuture::<()>::run_async(move || (|| { util::open_uri(wiki_link); })(), compat::executor)).exceptionally(|e| {
            logger().error(&format!("[Skyblocker] Error while retrieving wiki article: {}", wiki_link));
            script_host::send_feedback(&(constants::PREFIX.get_mut())().append_str("Error while retrieving wiki article, see logs...").to_string());
            return Default::default(); // java: return null
        });
    }

    /// java: public static ItemStack mapEitherToItemStack(Either<Slot, ItemStack> either)
    pub fn map_either_to_item_stack(either: &Either<Slot, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> mcrust::net::minecraft::world::item::item_stack::ItemStack {
        // TODO(fabric2seer): `.getItem(..)` on a L
        //   java: return either.right().orElseGet(() -> either.mapLeft(Slot::getItem).left().orElse(ItemStack.EMPTY));
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}

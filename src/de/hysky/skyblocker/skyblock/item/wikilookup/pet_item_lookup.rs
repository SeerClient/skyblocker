//! Ported from de.hysky.skyblocker.skyblock.item.wikilookup.PetItemLookup by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::item::pet_info::PetInfo;
use crate::de::hysky::skyblocker::skyblock::item::wikilookup::wiki_lookup;
use crate::de::hysky::skyblocker::skyblock::item::wikilookup::wiki_lookup::WikiLookup;
use crate::de::hysky::skyblocker::skyblock::item::wikilookup::wiki_lookup_manager::WikiLookupManager;
use mcrust::com::mojang::datafixers::util::either::Either;
use mcrust::net::minecraft::world::entity::player::player::Player;
use mcrust::net::minecraft::world::inventory::slot::Slot;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final Pattern PET_ITEM_NAME = Pattern.compile("^\\[Lvl \\d+] (?<name>.+)$");
pub static PET_ITEM_NAME: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("^\\[Lvl \\d+] (?<name>.+)$"));
/// java: private static final Function<String, Matcher> PET_MATCHER = itemName -> PET_ITEM_NAME.matcher(itemName + " Pet");
pub static PET_MATCHER: compat::MainThread<Arc<dyn Fn(String) -> compat::regex::Matcher>> = compat::MainThread::new(|| Arc::new(move |item_name| PET_ITEM_NAME.matcher(&format!("{} Pet", item_name))));
/// java: private static final Predicate<ItemStack> PET_ITEM_FILTER = itemStack -> { String itemName = itemStack.getHoverName().getString(); PetInfo petInfo = itemStack.getPetInfo(); // Filter only items that h
pub static PET_ITEM_FILTER: compat::MainThread<Arc<dyn Fn(mcrust::net::minecraft::world::item::item_stack::ItemStack) -> bool>> = compat::MainThread::new(|| Arc::new(move |item_stack| {
    let mut item_name: String = compat::component_of(item_stack.get_hover_name()).get_string();
    let mut pet_info: PetInfo = item_stack.get_pet_info();
    return (!pet_info.is_empty() || compat::regex::Pattern::matches(&PET_ITEM_NAME.pattern().to_string(), &item_name));
}));
/// java: public static final PetItemLookup INSTANCE = new PetItemLookup();
pub static INSTANCE: LazyLock<compat::Shared<PetItemLookup>> = LazyLock::new(|| PetItemLookup::new());

/// java: public class PetItemLookup implements WikiLookup
#[derive(Clone)]
pub struct PetItemLookup {}

impl PetItemLookup {
    /// java: private PetItemLookup()
    pub fn new() -> compat::Shared<Self> {
        let __this = compat::shared(Self {
        });
        __this
    }

    /// java: private static void lookupPetItem(Matcher matcher, Player player)
    pub fn lookup_pet_item(matcher: &compat::regex::Matcher, player: &dyn Player) {
        if matcher.matches() {
            let mut pet_name: String = (wiki_lookup::REPLACING_FUNCTION.get_mut())(compat::trim(&matcher.group_named("name").unwrap()));
            WikiLookupManager::open_wiki_link_name(&pet_name, player);
        }
    }

}

impl WikiLookup for PetItemLookup {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void open(ItemStack itemStack, Player player) (dispatch: the body for this class)
    fn open(&self) -> fn(&compat::Shared<dyn WikiLookup>, &mcrust::net::minecraft::world::item::item_stack::ItemStack, &dyn Player) {
        PetItemLookup::open_dyn
    }

    /// java: @Override public boolean canSearch(@Nullable String title, Either<Slot, ItemStack> either) (dispatch: the body for this class)
    fn can_search(&self) -> fn(&compat::Shared<dyn WikiLookup>, Option<String>, &Either<Slot, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> bool {
        PetItemLookup::can_search_dyn
    }

}

impl PetItemLookup {
    /// java: @Override public void open(ItemStack itemStack, Player player)
    pub fn open_impl(__this: &compat::Shared<Self>, item_stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, player: &dyn Player) {
        let mut item_name: String = compat::component_of(item_stack.get_hover_name()).get_string();
        let mut pet_info: PetInfo = item_stack.get_pet_info();
        PetItemLookup::lookup_pet_item(&(PET_MATCHER.get_mut())(pet_info.name().unwrap_or(item_name.clone())), player);
    }

    /// PetItemLookup.open as the dispatch of WikiLookup.open
    pub fn open_dyn(__this: &compat::Shared<dyn WikiLookup>, __p0: &mcrust::net::minecraft::world::item::item_stack::ItemStack, __p1: &dyn Player) {
        Self::open_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1)
    }

    /// java: @Override public boolean canSearch(@Nullable String title, Either<Slot, ItemStack> either)
    pub fn can_search_impl(__this: &compat::Shared<Self>, mut title: Option<String>, either: &Either<Slot, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> bool {
        let mut item_stack: mcrust::net::minecraft::world::item::item_stack::ItemStack = WikiLookupManager::map_either_to_item_stack(either);
        return (PET_ITEM_FILTER.get_mut())(item_stack.clone());
    }

    /// PetItemLookup.canSearch as the dispatch of WikiLookup.canSearch
    pub fn can_search_dyn(__this: &compat::Shared<dyn WikiLookup>, __p0: Option<String>, __p1: &Either<Slot, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> bool {
        Self::can_search_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1)
    }

}

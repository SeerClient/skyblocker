//! Ported from de.hysky.skyblocker.skyblock.item.wikilookup.VisitorLookup by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::item::wikilookup::wiki_lookup;
use crate::de::hysky::skyblocker::skyblock::item::wikilookup::wiki_lookup::WikiLookup;
use crate::de::hysky::skyblocker::skyblock::item::wikilookup::wiki_lookup_manager::WikiLookupManager;
use mcrust::com::mojang::datafixers::util::either::Either;
use mcrust::net::minecraft::world::entity::player::player::Player;
use mcrust::net::minecraft::world::inventory::slot::Slot;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: public static final VisitorLookup INSTANCE = new VisitorLookup();
pub static INSTANCE: LazyLock<compat::Shared<VisitorLookup>> = LazyLock::new(|| VisitorLookup::new());
/// java: private static final Pattern VISITORS_LOGBOOK_PATTERN = Pattern.compile("^(?:\\(\\d+\\/\\d+\\) )?Visitor's Logbook$");
pub static VISITORS_LOGBOOK_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("^(?:\\(\\d+\\/\\d+\\) )?Visitor's Logbook$"));

/// java: public class VisitorLookup implements WikiLookup
#[derive(Clone)]
pub struct VisitorLookup {}

impl VisitorLookup {
    /// java: private VisitorLookup()
    pub fn new() -> compat::Shared<Self> {
        let __this = compat::shared(Self {
        });
        __this
    }

}

impl WikiLookup for VisitorLookup {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void open(ItemStack itemStack, Player player) (dispatch: the body for this class)
    fn open(&self) -> fn(&compat::Shared<dyn WikiLookup>, &mcrust::net::minecraft::world::item::item_stack::ItemStack, &dyn Player) {
        VisitorLookup::open_dyn
    }

    /// java: @Override public boolean canSearch(@Nullable String title, Either<Slot, ItemStack> either) (dispatch: the body for this class)
    fn can_search(&self) -> fn(&compat::Shared<dyn WikiLookup>, Option<String>, &Either<Slot, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> bool {
        VisitorLookup::can_search_dyn
    }

}

impl VisitorLookup {
    /// java: @Override public void open(ItemStack itemStack, Player player)
    pub fn open_impl(__this: &compat::Shared<Self>, item_stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, player: &dyn Player) {
        let mut item_name: String = (wiki_lookup::REPLACING_FUNCTION.get_mut())(compat::component_of(item_stack.get_hover_name()).get_string());
        WikiLookupManager::open_wiki_link_name(&item_name, player);
    }

    /// VisitorLookup.open as the dispatch of WikiLookup.open
    pub fn open_dyn(__this: &compat::Shared<dyn WikiLookup>, __p0: &mcrust::net::minecraft::world::item::item_stack::ItemStack, __p1: &dyn Player) {
        Self::open_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1)
    }

    /// java: @Override public boolean canSearch(@Nullable String title, Either<Slot, ItemStack> either)
    pub fn can_search_impl(__this: &compat::Shared<Self>, mut title: Option<String>, either: &Either<Slot, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> bool {
        let mut optional: Option<Slot> = either.left_value().unwrap();
        if optional.is_none() {
            return false;
        }
        let mut slot: Slot = optional.clone().unwrap();
        if slot.index <= 9 || slot.index >= 44 {
            return false;
        }
        // TODO(fabric2seer): `get_item` takes 1 argument(s) in the port (`containers: &'a Containers<'_>`), Java passes 0
        if slot.get_item().is_item(mcrust::net::minecraft::world::item::items::stained_glass_pane().black()) {
            return false;
        }
        // TODO(fabric2seer): static call StringUtils.isNotEmpty(..)
        //   java: return StringUtils.isNotEmpty(title) && VISITORS_LOGBOOK_PATTERN.matcher(title).matches();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// VisitorLookup.canSearch as the dispatch of WikiLookup.canSearch
    pub fn can_search_dyn(__this: &compat::Shared<dyn WikiLookup>, __p0: Option<String>, __p1: &Either<Slot, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> bool {
        Self::can_search_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1)
    }

}

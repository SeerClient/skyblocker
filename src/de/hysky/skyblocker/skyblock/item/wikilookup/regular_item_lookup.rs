//! Ported from de.hysky.skyblocker.skyblock.item.wikilookup.RegularItemLookup by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::item::wikilookup::wiki_lookup::WikiLookup;
use crate::de::hysky::skyblocker::skyblock::item::wikilookup::wiki_lookup_manager::WikiLookupManager;
use crate::de::hysky::skyblocker::skyblock::itemlist::item_repository::ItemRepository;
use crate::de::hysky::skyblocker::utils::constants;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::world::entity::player::player::Player;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;

/// java: public static final RegularItemLookup INSTANCE = new RegularItemLookup();
pub static INSTANCE: LazyLock<compat::Shared<RegularItemLookup>> = LazyLock::new(|| RegularItemLookup::new());

/// java: public class RegularItemLookup implements WikiLookup
#[derive(Clone)]
pub struct RegularItemLookup {}

impl RegularItemLookup {
    /// java: private RegularItemLookup()
    pub fn new() -> compat::Shared<Self> {
        let __this = compat::shared(Self {
        });
        __this
    }

    /// java: private static void noArticleFound(Player player)
    pub fn no_article_found(player: &dyn Player) {
        script_host::send_feedback(&(constants::PREFIX.get_mut())().append(component::translatable("skyblocker.wikiLookup.noArticleFound.independent")).to_string());
    }

}

impl WikiLookup for RegularItemLookup {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void open(ItemStack itemStack, Player player) (dispatch: the body for this class)
    fn open(&self) -> fn(&compat::Shared<dyn WikiLookup>, &mcrust::net::minecraft::world::item::item_stack::ItemStack, &dyn Player) {
        RegularItemLookup::open_dyn
    }

}

impl RegularItemLookup {
    /// java: @Override public void open(ItemStack itemStack, Player player)
    pub fn open_impl(__this: &compat::Shared<Self>, item_stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, player: &dyn Player) {
        let mut neu_id: String = item_stack.get_neu_name();
        // TODO(fabric2seer): the condition: static call StringUtils.isNotEmpty(..)
        //   java: StringUtils.isNotEmpty(..)
        if todo!("fabric2seer: the condition (TODO above)") {
            let mut wiki_link: Option<String> = ItemRepository::get_wiki_link(&neu_id);
            if let Some(wiki_link) = &wiki_link {
                WikiLookupManager::open_wiki_link(wiki_link, player);
                return;
            }
        }
        RegularItemLookup::no_article_found(player);
    }

    /// RegularItemLookup.open as the dispatch of WikiLookup.open
    pub fn open_dyn(__this: &compat::Shared<dyn WikiLookup>, __p0: &mcrust::net::minecraft::world::item::item_stack::ItemStack, __p1: &dyn Player) {
        Self::open_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1)
    }

}

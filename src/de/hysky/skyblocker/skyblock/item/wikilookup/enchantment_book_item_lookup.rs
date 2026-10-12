//! Ported from de.hysky.skyblocker.skyblock.item.wikilookup.EnchantmentBookItemLookup by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::item::wikilookup::wiki_lookup;
use crate::de::hysky::skyblocker::skyblock::item::wikilookup::wiki_lookup::WikiLookup;
use crate::de::hysky::skyblocker::skyblock::item::wikilookup::wiki_lookup_manager::WikiLookupManager;
use crate::de::hysky::skyblocker::utils::item_utils::ItemUtils;
use mcrust::com::mojang::datafixers::util::either::Either;
use mcrust::net::minecraft::nbt::tag::Tag;
use mcrust::net::minecraft::world::entity::player::player::Player;
use mcrust::net::minecraft::world::inventory::slot::Slot;
use mcrust::net::minecraft::world::item::items;

/// java: private static final Predicate<ItemStack> ENCHANTMENT_BOOK_FILTER = itemStack -> { CompoundTag nbt = ItemUtils.getCustomData(itemStack); if (itemStack.is(Items.ENCHANTED_BOOK) && nbt.contains("enchant
pub static ENCHANTMENT_BOOK_FILTER: compat::MainThread<Arc<dyn Fn(mcrust::net::minecraft::world::item::item_stack::ItemStack) -> bool>> = compat::MainThread::new(|| Arc::new(move |item_stack| {
    let mut nbt: Tag = ItemUtils::get_custom_data(&item_stack);
    if item_stack.is_item(&items::items().get("enchanted_book")) && nbt.contains("enchantments") {
        let mut enchantments: Tag = nbt.get_compound_or_empty("enchantments");
        return ((enchantments.key_set().len() as i32) == 1);
    }
    return false;
}));
/// java: public static final EnchantmentBookItemLookup INSTANCE = new EnchantmentBookItemLookup();
pub static INSTANCE: LazyLock<compat::Shared<EnchantmentBookItemLookup>> = LazyLock::new(|| EnchantmentBookItemLookup::new());

/// java: public class EnchantmentBookItemLookup implements WikiLookup
#[derive(Clone)]
pub struct EnchantmentBookItemLookup {}

impl EnchantmentBookItemLookup {
    /// java: private EnchantmentBookItemLookup()
    pub fn new() -> compat::Shared<Self> {
        let __this = compat::shared(Self {
        });
        __this
    }

}

impl WikiLookup for EnchantmentBookItemLookup {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void open(ItemStack itemStack, Player player) (dispatch: the body for this class)
    fn open(&self) -> fn(&compat::Shared<dyn WikiLookup>, &mcrust::net::minecraft::world::item::item_stack::ItemStack, &dyn Player) {
        EnchantmentBookItemLookup::open_dyn
    }

    /// java: @Override public boolean canSearch(@Nullable String title, Either<Slot, ItemStack> either) (dispatch: the body for this class)
    fn can_search(&self) -> fn(&compat::Shared<dyn WikiLookup>, Option<String>, &Either<Slot, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> bool {
        EnchantmentBookItemLookup::can_search_dyn
    }

}

impl EnchantmentBookItemLookup {
    /// java: @Override public void open(ItemStack itemStack, Player player)
    pub fn open_impl(__this: &compat::Shared<Self>, item_stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, player: &dyn Player) {
        let mut nbt: Tag = ItemUtils::get_custom_data(item_stack);
        let mut enchantments: Tag = nbt.get_compound_or_empty("enchantments");
        // TODO(fabric2seer): static call Iterables.getFirst(..)
        //   java: String firstEnchantment = Iterables.getFirst(enchantments.keySet(), null)
        //   java: 				.replace("ultimate_", "") // Stripped out ultimate prefix
        //   java: 				.replace("_", " ").trim();
        let mut first_enchantment: String = todo!("fabric2seer: the initialiser of `firstEnchantment` (TODO above)");
        // TODO(fabric2seer): static call WordUtils.capitalizeFully(..)
        //   java: String enchantment = REPLACING_FUNCTION.apply(WordUtils.capitalizeFully(firstEnchantment + " enchantment"));
        let mut enchantment: String = todo!("fabric2seer: the initialiser of `enchantment` (TODO above)");
        WikiLookupManager::open_wiki_link_name(&enchantment, player);
    }

    /// EnchantmentBookItemLookup.open as the dispatch of WikiLookup.open
    pub fn open_dyn(__this: &compat::Shared<dyn WikiLookup>, __p0: &mcrust::net::minecraft::world::item::item_stack::ItemStack, __p1: &dyn Player) {
        Self::open_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1)
    }

    /// java: @Override public boolean canSearch(@Nullable String title, Either<Slot, ItemStack> either)
    pub fn can_search_impl(__this: &compat::Shared<Self>, mut title: Option<String>, either: &Either<Slot, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> bool {
        let mut item_stack: mcrust::net::minecraft::world::item::item_stack::ItemStack = WikiLookupManager::map_either_to_item_stack(either);
        return (ENCHANTMENT_BOOK_FILTER.get_mut())(item_stack.clone());
    }

    /// EnchantmentBookItemLookup.canSearch as the dispatch of WikiLookup.canSearch
    pub fn can_search_dyn(__this: &compat::Shared<dyn WikiLookup>, __p0: Option<String>, __p1: &Either<Slot, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> bool {
        Self::can_search_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1)
    }

}

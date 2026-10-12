//! Ported from de.hysky.skyblocker.skyblock.museum.ItemSorter by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::museum::donation::Donation;
use crate::de::hysky::skyblocker::skyblock::museum::museum_manager::MuseumManager;
use crate::de::hysky::skyblocker::skyblock::museum::price_data::PriceData;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::components::tooltip::Tooltip;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::world::item::items;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final Consumer<List<Donation>> SORT_BY_LOWESTBIN = donations -> { donations.forEach(donation -> updateDonationData(donation, false)); donations.sort(ItemSorter::compareEffectivePrices);
pub static SORT_BY_LOWESTBIN: compat::MainThread<Arc<dyn Fn(Vec<compat::Shared<Donation>>)>> = compat::MainThread::new(|| Arc::new(move |donations| {
    for donation in donations.iter() {
        ItemSorter::update_donation_data(donation.clone(), false);
    }
    donations.sort_by(|__a0, __a1| (ItemSorter::compare_effective_prices(__a0, __a1)).cmp(&0));
}));
/// java: private static final Consumer<List<Donation>> SORT_BY_CRAFTCOST = donations -> { donations.forEach(donation -> updateDonationData(donation, true)); donations.sort(ItemSorter::compareEffectivePrices); 
pub static SORT_BY_CRAFTCOST: compat::MainThread<Arc<dyn Fn(Vec<compat::Shared<Donation>>)>> = compat::MainThread::new(|| Arc::new(move |donations| {
    for donation in donations.iter() {
        ItemSorter::update_donation_data(donation.clone(), true);
    }
    donations.sort_by(|__a0, __a1| (ItemSorter::compare_effective_prices(__a0, __a1)).cmp(&0));
}));
/// java: private static final Consumer<List<Donation>> SORT_BY_XP_PER_COIN = donations -> { donations.forEach(donation -> updateDonationData(donation, true)); donations.sort(ItemSorter::compareCoinsPerXP); };
pub static SORT_BY_XP_PER_COIN: compat::MainThread<Arc<dyn Fn(Vec<compat::Shared<Donation>>)>> = compat::MainThread::new(|| Arc::new(move |donations| {
    for donation in donations.iter() {
        ItemSorter::update_donation_data(donation.clone(), true);
    }
    donations.sort_by(|__a0, __a1| (ItemSorter::compare_coins_per_xp(__a0, __a1)).cmp(&0));
}));

/// java: public class ItemSorter
#[derive(Clone)]
pub struct ItemSorter {
    /// java: private SortMode currentSortMode = SortMode.LOWEST_BIN;
    pub current_sort_mode: SortMode,
}

impl ItemSorter {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            current_sort_mode: SortMode::LowestBin,
        };
        this
    }

    /// java: public static void updateDonationData(Donation donation, boolean useCraftCost)
    pub fn update_donation_data(mut donation: compat::Shared<Donation>, mut use_craft_cost: bool) {
        let mut downgrades: Vec<String> = Donation::get_downgrades(&donation);
        let mut discount: Option<(String, f64)> = Donation::get_discount(&donation);
        let mut will_count_for: Vec<compat::Shared<Donation>> = downgrades.iter().cloned().map(|__a0| MuseumManager::get_donation(&__a0)).filter(|__a0| true).collect::<Vec<_>>();
        // TODO(fabric2seer): List.addFirst(..)
        //   java: willCountFor.addFirst(donation);
        let mut total_xp: i32 = will_count_for.iter().cloned().map(|__a0| Donation::get_xp(&__a0)).sum::<i32>();
        let mut l_bin_price: f64 = PriceData::get_l_bin_price(&Donation::get_price_data(&donation));
        let mut raw_craft_cost: f64 = (if Donation::is_craftable(&donation) { PriceData::get_craft_cost(&Donation::get_price_data(&donation)) } else { ((0) as f64) as f64 });
        let mut craft_cost: f64 = (if discount.is_some() { (raw_craft_cost - discount.1) } else { raw_craft_cost });
        let mut effective_price: f64 = (if use_craft_cost { (if craft_cost > (0 as f64) { (if l_bin_price == (0 as f64) { craft_cost } else { craft_cost.min(l_bin_price) }) } else { l_bin_price }) } else { (if l_bin_price > (0 as f64) { l_bin_price } else { craft_cost }) });
        let mut ratio: f64 = (if total_xp > 0 && effective_price > (0 as f64) { (effective_price / (total_xp as f64)) } else { ((0) as f64) as f64 });
        if Donation::is_set(&donation) {
            for pair in Donation::get_set(&donation) {
                PriceData::set_effective_price(&pair.1, (if effective_price == craft_cost { PriceData::get_craft_cost(&pair.1) } else { PriceData::get_l_bin_price(&pair.1) }));
            }
        }
        PriceData::set_effective_price(&Donation::get_price_data(&donation), effective_price);
        Donation::set_xp_coins_ratio(&donation, ratio);
        Donation::set_total_xp(&donation, total_xp);
        Donation::set_counts_towards(&donation, will_count_for.iter().cloned().map(|d| (Donation::get_id(&d), Donation::get_xp(&d))).collect::<Vec<_>>());
    }

    /// java: private static int compareEffectivePrices(Donation a, Donation b)
    pub fn compare_effective_prices(a: &compat::Shared<Donation>, b: &compat::Shared<Donation>) -> i32 {
        let mut price_a: f64 = PriceData::get_effective_price(&Donation::get_price_data(a));
        let mut price_b: f64 = PriceData::get_effective_price(&Donation::get_price_data(b));
        if price_a <= (0 as f64) && price_b <= (0 as f64) {
            return compat::ord_int(Donation::get_total_xp(b).cmp(&Donation::get_total_xp(a)));
        }
        if price_a <= (0 as f64) {
            return 1;
        }
        if price_b <= (0 as f64) {
            return -1;
        }
        return compat::ord_int(price_a.total_cmp(&price_b));
    }

    /// java: private static int compareCoinsPerXP(Donation a, Donation b)
    pub fn compare_coins_per_xp(a: &compat::Shared<Donation>, b: &compat::Shared<Donation>) -> i32 {
        let mut xp_per_coin_a: f64 = Donation::get_xp_coins_ratio(a);
        let mut xp_per_coin_b: f64 = Donation::get_xp_coins_ratio(b);
        if xp_per_coin_a == (0 as f64) && xp_per_coin_b == (0 as f64) {
            return compat::ord_int(Donation::get_total_xp(b).cmp(&Donation::get_total_xp(a)));
        }
        if xp_per_coin_a == (0 as f64) {
            return 1;
        }
        if xp_per_coin_b == (0 as f64) {
            return -1;
        }
        return compat::ord_int(xp_per_coin_a.total_cmp(&xp_per_coin_b));
    }

    /// java: public void cycleSortMode(List<Donation> donations)
    pub fn cycle_sort_mode(&mut self, donations: &Vec<compat::Shared<Donation>>) {
        self.current_sort_mode = SortMode::values()[((self.current_sort_mode.ordinal().wrapping_add(1)).wrapping_rem(SortMode::values().len() as i32)) as usize];
        self.apply_sort(donations);
    }

    /// java: public void applySort(List<Donation> donations)
    pub fn apply_sort(&self, donations: &Vec<compat::Shared<Donation>>) {
        (self.current_sort_mode.get_sort_function())(donations.clone());
    }

    /// java: public ItemStack getCurrentSortingItem()
    pub fn get_current_sorting_item(&self) -> mcrust::net::minecraft::world::item::item_stack::ItemStack {
        return self.current_sort_mode.get_associated_item();
    }

    /// java: public Tooltip getTooltip()
    pub fn get_tooltip(&self) -> Tooltip {
        let mut tooltip: MutableComponent = (component::translatable("skyblocker.museum.hud.sorter").append_str("\n\n")).with_style_format(ChatFormatting::Green).append(self.get_sort_text(SortMode::LowestBin)).append(self.get_sort_text(SortMode::CraftCost)).append(self.get_sort_text(SortMode::CoinsPerXp)).append_str("\n").append(component::translatable("skyblocker.museum.hud.sorter.switch").with_style_format(ChatFormatting::Yellow));
        return Tooltip::create(tooltip.get_string());
    }

    /// java: private Component getSortText(SortMode mode)
    pub fn get_sort_text(&self, mut mode: SortMode) -> MutableComponent {
        let mut is_current: bool = (mode == self.current_sort_mode);
        return (component::literal((if is_current { "➤ " } else { "  " }))).append(mode.get_display_name()).append_str("\n").with_style_format((if is_current { ChatFormatting::Aqua } else { ChatFormatting::Gray }));
    }

}

/// java: public enum SortMode
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SortMode {
    LowestBin,
    CraftCost,
    CoinsPerXp,
}

impl SortMode {
    pub const VALUES: [SortMode; 3] = [SortMode::LowestBin, SortMode::CraftCost, SortMode::CoinsPerXp];

    /// java: values()
    pub fn values() -> Vec<SortMode> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            SortMode::LowestBin => "LOWEST_BIN",
            SortMode::CraftCost => "CRAFT_COST",
            SortMode::CoinsPerXp => "COINS_PER_XP",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> SortMode {
        match name {
            "LOWEST_BIN" => SortMode::LowestBin,
            "CRAFT_COST" => SortMode::CraftCost,
            "COINS_PER_XP" => SortMode::CoinsPerXp,
            _ => panic!("IllegalArgumentException: No enum constant SortMode.{name}"),
        }
    }

    /// the constant's constructor data (Java keeps it in the constant's fields)
    pub fn fields(self) -> &'static SortModeFields {
        static FIELDS: LazyLock<Vec<SortModeFields>> = LazyLock::new(|| vec![
            SortModeFields::new(mcrust::net::minecraft::world::item::item_stack::ItemStack::of_one(items::items().get("gold_ingot")), SORT_BY_LOWESTBIN.get_mut().clone(), component::translatable("skyblocker.museum.hud.sorter.lBin")),
            SortModeFields::new(mcrust::net::minecraft::world::item::item_stack::ItemStack::of_one(items::items().get("crafting_table")), SORT_BY_CRAFTCOST.get_mut().clone(), component::translatable("skyblocker.museum.hud.sorter.craftCost")),
            SortModeFields::new(mcrust::net::minecraft::world::item::item_stack::ItemStack::of_one(items::items().get("experience_bottle")), SORT_BY_XP_PER_COIN.get_mut().clone(), component::translatable("skyblocker.museum.hud.sorter.ratio")),
        ]);
        &FIELDS[self as usize]
    }

    /// java: public ItemStack getAssociatedItem()
    pub fn get_associated_item(&self) -> mcrust::net::minecraft::world::item::item_stack::ItemStack {
        return self.fields().associated_item.clone();
    }

    /// java: public Component getDisplayName()
    pub fn get_display_name(&self) -> MutableComponent {
        return self.fields().display_name.clone();
    }

    /// java: public Consumer<List<Donation>> getSortFunction()
    pub fn get_sort_function(&self) -> Arc<dyn Fn(Vec<compat::Shared<Donation>>)> {
        return self.fields().sort_function.clone();
    }

}

impl std::fmt::Display for SortMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// the constructor data of each SortMode constant
pub struct SortModeFields {
    pub associated_item: mcrust::net::minecraft::world::item::item_stack::ItemStack,
    pub sort_function: Arc<dyn Fn(Vec<compat::Shared<Donation>>)>,
    pub display_name: MutableComponent,
}

impl SortModeFields {
    fn new(mut item: mcrust::net::minecraft::world::item::item_stack::ItemStack, function: Arc<dyn Fn(Vec<compat::Shared<Donation>>)>, mut display_name: MutableComponent) -> Self {
        let mut this = Self {
            associated_item: Default::default(),
            sort_function: Default::default(),
            display_name: Default::default(),
        };
        this.associated_item = item.clone();
        this.sort_function = function.clone();
        this.display_name = display_name.clone();
        this
    }

}

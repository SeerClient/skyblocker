//! Ported from de.hysky.skyblocker.skyblock.museum.ItemFilter by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::museum::donation::Donation;
use crate::de::hysky::skyblocker::skyblock::museum::museum_item_cache;
use mcrust::java::text::decimal_format_symbols::Locale;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::components::tooltip::Tooltip;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: public class ItemFilter
#[derive(Clone)]
pub struct ItemFilter {
    /// java: private final List<String> categories = new ArrayList<>();
    pub categories: Vec<String>,
    /// java: private int filterIndex = 0;
    pub filter_index: i32,
}

impl ItemFilter {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            categories: Vec::new(),
            filter_index: 0,
        };
        this
    }

    /// java: public void updateCategories()
    pub fn update_categories(&mut self) {
        self.categories.clear();
        self.categories.push("All".to_string());
        self.categories.extend(compat::lock(&museum_item_cache::MUSEUM_CATEGORIES).iter().cloned());
        compat::remove_item(&mut self.categories, &"special".to_string());
        self.filter_index = self.filter_index.min((self.categories.len() as i32).wrapping_sub(1));
    }

    /// java: public void cycleFilterMode(List<Donation> items, List<Donation> filteredList)
    pub fn cycle_filter_mode(&mut self, items: &Vec<compat::Shared<Donation>>, filtered_list: &mut Vec<compat::Shared<Donation>>) {
        if self.categories.is_empty() {
            return;
        }
        self.filter_index = (self.filter_index.wrapping_add(1)).wrapping_rem(self.categories.len() as i32);
        self.apply_filter(items, filtered_list);
    }

    /// java: public void applyFilter(List<Donation> items, List<Donation> filteredList)
    pub fn apply_filter(&self, items: &Vec<compat::Shared<Donation>>, filtered_list: &mut Vec<compat::Shared<Donation>>) {
        filtered_list.clear();
        if self.filter_index == 0 {
            filtered_list.extend(items.iter().cloned());
        } else {
            for __a0 in items.iter().cloned().filter(|d| (Donation::get_category(d) == self.categories[(self.filter_index) as usize])) {
                filtered_list.push(__a0.clone());
            }
        }
    }

    /// java: public Tooltip getTooltip()
    pub fn get_tooltip(&self) -> Tooltip {
        let mut tooltip: MutableComponent = (component::translatable("skyblocker.museum.hud.filter").append_str("\n\n")).with_style_format(ChatFormatting::Green);
        let mut i: i32 = 0;
        for category in self.categories.iter() {
            let mut category_name: String = (if compat::str_len(&category) < 2 { category.to_string() } else { format!("{}{}", (compat::substring2(&category, 0, 1)).to_uppercase(), compat::substring(&category, 1)) });
            // java: tooltip.append(..) - appends in place
            tooltip = tooltip.append(self.get_filter_text(i, &component::literal(&category_name)));
            i = i.wrapping_add(1);
        }
        tooltip = (tooltip.append_str("\n")).append(component::translatable("skyblocker.museum.hud.filter.switch").with_style_format(ChatFormatting::Yellow));
        return Tooltip::create(tooltip.get_string());
    }

    /// java: private Component getFilterText(int index, Component category)
    pub fn get_filter_text(&self, mut index: i32, category: &MutableComponent) -> MutableComponent {
        let mut is_current: bool = (index == self.filter_index);
        return (component::literal((if is_current { "➤ " } else { "  " }))).append(category.clone()).append_str("\n").with_style_format((if is_current { ChatFormatting::Aqua } else { ChatFormatting::Gray }));
    }

}

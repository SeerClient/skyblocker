//! Ported from de.hysky.skyblocker.skyblock.chat.SackMessagePrice by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::adders::line_smoothener::LineSmoothener;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::info::tooltip_info_type;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::item_tooltip::ItemTooltip;
use crate::de::hysky::skyblocker::utils::bazaar_product::BazaarProduct;
use crate::de::hysky::skyblocker::utils::neu_repo_manager::NEURepoManager;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::info::data_tooltip_info_type::DataTooltipInfoType as _;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final Pattern ITEM_COUNT_PATTERN = Pattern.compile("([-+][\\d,]+)");
pub static ITEM_COUNT_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("([-+][\\d,]+)"));

/// java: public class SackMessagePrice
#[derive(Clone)]
pub struct SackMessagePrice {}

impl SackMessagePrice {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        // TODO(fabric2seer): ClientReceiveMessageEvents.MODIFY_GAME.register: no SeerClient equivalent yet
        //   java: ClientReceiveMessageEvents.MODIFY_GAME.register(SackMessagePrice::onMessage);
    }

    /// java: private static Component onMessage(Component original, boolean overlay)
    pub fn on_message(original: &MutableComponent, mut overlay: bool) -> MutableComponent {
        if overlay {
            return original.clone();
        }
        let mut string: String = original.get_string();
        if !(string.starts_with("[Sacks] ")) {
            return original.clone();
        }
        let mut copy: MutableComponent = SackMessagePrice::deep_copy(original);
        let mut list_list: Vec<Vec<MutableComponent>> = SackMessagePrice::get_hover_event_siblings(&copy);
        if list_list.is_empty() {
            return original.clone();
        }
        for text_list in list_list.iter_mut() {
            let mut items: std::collections::HashMap<String, i32> = SackMessagePrice::parse_items(text_list);
            if items.is_empty() {
                logger().warn(&format!("No items found in sack message: `{}`", original.get_string()));
                return original.clone();
            }
            let mut npc_price: f64 = ((0) as f64) as f64;
            let mut bazaar_buy_price: f64 = ((0) as f64) as f64;
            let mut bazaar_sell_price: f64 = ((0) as f64) as f64;
            for entry in items.iter().map(|(k, v)| (k.clone(), v.clone())) {
                let mut item_name: String = entry.0.clone();
                let mut count: i32 = entry.1;
                let Some(mut neu_id) = SackMessagePrice::get_neu_id(&item_name) else {
                    logger().warn(&format!("Failed to find NEU ID for item: `{}`. This item will not be priced.", item_name));
                    continue;
                };
                let mut npc_data: Option<std::collections::HashMap<String, f64>> = tooltip_info_type::NPC.get_mut().lock().borrow().get_data();
                if let Some(npc_data) = &npc_data {
                    npc_price = (npc_price + (npc_data.get(&neu_id).cloned().unwrap_or_else(|| ((0) as f64) as f64) * (count as f64)));
                } else {
                    logger().warn(&format!("No NPC data found for item: `{}`", neu_id));
                }
                let mut bazaar_data: Option<std::collections::HashMap<String, BazaarProduct>> = tooltip_info_type::BAZAAR.get_mut().lock().borrow().get_data();
                if let Some(bazaar_data) = &bazaar_data {
                    let mut item_data: Option<BazaarProduct> = bazaar_data.get(&neu_id).cloned();
                    if let Some(item_data) = &item_data {
                        let mut buy_price: Option<f64> = item_data.buy_price();
                        if buy_price.is_some() {
                            bazaar_buy_price = (bazaar_buy_price + (buy_price.clone().unwrap() * (count as f64)));
                        }
                        let mut sell_price: Option<f64> = item_data.sell_price();
                        if sell_price.is_some() {
                            bazaar_sell_price = (bazaar_sell_price + (sell_price.clone().unwrap() * (count as f64)));
                        }
                    } else {
                        logger().warn(&format!("No item data found for item `{}` in bazaar price data.", neu_id));
                    }
                }
            }
            text_list.push(component::literal(mcrust::net::minecraft::network::chat::common_components::NEW_LINE));
            text_list.push(LineSmoothener::create_smooth_line());
            text_list.push(component::literal(mcrust::net::minecraft::network::chat::common_components::NEW_LINE));
            text_list.push(component::empty().append(component::literal("NPC Sell Price: ").with_style_format(ChatFormatting::Yellow)).append((if npc_price > (0 as f64) { ItemTooltip::get_coins_message(npc_price, 1) } else { component::literal("No data").with_style_format(ChatFormatting::Red) })));
            text_list.push(component::literal(mcrust::net::minecraft::network::chat::common_components::NEW_LINE));
            text_list.push(component::empty().append(component::literal("Bazaar Buy Price: ").with_style_format(ChatFormatting::Gold)).append((if bazaar_buy_price > (0 as f64) { ItemTooltip::get_coins_message(bazaar_buy_price, 1) } else { component::literal("No data").with_style_format(ChatFormatting::Red) })));
            text_list.push(component::literal(mcrust::net::minecraft::network::chat::common_components::NEW_LINE));
            text_list.push(component::empty().append(component::literal("Bazaar Sell Price: ").with_style_format(ChatFormatting::Gold)).append((if bazaar_sell_price > (0 as f64) { ItemTooltip::get_coins_message(bazaar_sell_price, 1) } else { component::literal("No data").with_style_format(ChatFormatting::Red) })));
        }
        return copy;
    }

    /// java: private static MutableComponent deepCopy(Component text)
    pub fn deep_copy(text: &MutableComponent) -> MutableComponent {
        let mut copy: MutableComponent = text.plain_copy();
        // TODO(fabric2seer): not translated (expected ')' at '(' (offset 6049))
        //   java: if (text.getStyle().getHoverEvent() instanceof ShowText(Component showText)) {
        //   java: 			// DO NOT simplify to `text.getStyle().withHoverEvent(new ShowText(showText);`,
        //   java: 			// Style.withHoverEvent(hoverEvent) checks if the given value is equal to the current one, and since our text clone is equal by value, it will not create a new style.
        //   java: 			// This means the original hover event will still be used which is immutable, and our list modifications will fail with an UnsupportedOperationException in #onMessage.
        //   java: 			copy.setStyle(Style.EMPTY.withHoverEvent(new ShowText(deepCopy(showText))).applyTo(text.getStyle()));
        //   java: 		} else copy.setStyle(text.getStyle());
        for sibling in text.get_siblings().iter() {
            copy = copy.append(SackMessagePrice::deep_copy(sibling));
        }
        return copy;
    }

    /// java: private static @Nullable String getNeuId(String itemName)
    pub fn get_neu_id(item_name: &str) -> Option<String> {
        //   java: return NEURepoManager.getItemByName(itemName)
        //   java: 				.stream()
        //   java: 				.filter(Objects::nonNull)
        //   java: 				.findFirst()
        //   java: 				.map(NEUItem::getSkyblockItemId)
        //   java: 				.orElseGet(() -> {
        //   java: 					LOGGER.warn("Failed to find the NEU item ID for item: {}", itemName);
        //   java: 					return null; // This won't be used to calculate the price
        //   java: 				});
        match crate::de::hysky::skyblocker::utils::neu_repo_manager::NEURepoManager::get_item_by_name(item_name).into_iter().flatten().next() {
            // rust: port gap - `NEUItem.getSkyblockItemId` (the NEU repo parser is a `crate::unresolved` placeholder,
            // GAPS_batchB L1; the repo never loads, so no item is found and this arm is not reached)
            Some(_item) => todo!("rust: port gap - NEUItem.getSkyblockItemId"),
            None => {
                logger().warn(&format!("Failed to find the NEU item ID for item: {}", item_name));
                return None; // This won't be used to calculate the price
            }
        }
    }

    /// java: private static ObjectArrayList<List<Component>> getHoverEventSiblings(Component text)
    pub fn get_hover_event_siblings(text: &MutableComponent) -> Vec<Vec<MutableComponent>> {
        let mut list_list: Vec<Vec<MutableComponent>> = Vec::new();
        for sibling in text.get_siblings().iter() {
            // TODO(fabric2seer): not translated (expected ')' at '(' (offset 7402))
            //   java: if (sibling.getStyle().getHoverEvent() instanceof ShowText(Component hoverText)
            //   java: 					&& hoverText.getContents() instanceof LiteralContents(String rootContent) // Only match the root content since we only need the root content.
            //   java: 					&& Strings.CS.startsWithAny(rootContent, "Added items:", "Removed items:")) {
            //   java: 				listList.add(hoverText.getSiblings());
            //   java: 			}
        }
        return list_list;
    }

    /// java: @SuppressWarnings("ConstantValue") // It's much easier to read this way. private static Object2IntArrayMap<String> parseItems(List<Component> texts)
    pub fn parse_items(texts: &Vec<MutableComponent>) -> std::collections::HashMap<String, i32> {
        let mut items: std::collections::HashMap<String, i32> = std::collections::HashMap::new();
        let mut last_count: Option<i32> = None;
        let mut last_item_name: Option<String> = None;
        for text in texts.iter() {
            // TODO(fabric2seer): not translated (expected ')' at '(' (offset 8583))
            //   java: if (text.getContents() instanceof LiteralContents(String content)) { // Only match the root content since we only need the root content.
            //   java: 				if (content.equals("\n\n")) break; // End of items list, we can stop parsing here - NOTE: This has to come before the isBlank check, otherwise it will be skipped.
            //   java: 				if (content.isBlank()) continue; // This includes \n lines, which we don't want to try and parse as item names or counts
            //   java:
            //   java: 				String trimmed = content.trim();
            //   java: 				if (lastCount == null && lastItemName == null) { // Initial state, item count comes first
            //   java: 					Matcher matcher = ITEM_COUNT_PATTERN.matcher(trimmed);
            //   java: 					OptionalInt count = RegexUtils.findIntFromMatcher(matcher);
            //   java: 					if (count.isEmpty()) {
            //   java: 						LOGGER.error("Failed to parse item count from text content: `{}`", trimmed);
            //   java: 						return new Object2IntArrayMap<>(); // Something went wrong, so we panic and not modify the text.
            //   java: 					}
            //   java: 					lastCount = count.getAsInt();
            //   java: 				} else if (lastCount != null && lastItemName == null) { // The item name comes next
            //   java: 					lastItemName = trimmed;
            //   java: 				} else if (lastCount != null && lastItemName != null) { // Then comes the sack name, which we can ignore but this iteration can still be used for finalizing the cycle
            //   java: 					items.put(lastItemName, lastCount.intValue());
            //   java: 					lastCount = null;
            //   java: 					lastItemName = null;
            //   java: 				}
            //   java: 			}
        }
        return items;
    }

}

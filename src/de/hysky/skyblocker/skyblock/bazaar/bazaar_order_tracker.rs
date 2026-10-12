//! Ported from de.hysky.skyblocker.skyblock.bazaar.BazaarOrderTracker by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::simple_tooltip_adder::SimpleTooltipAdder;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::simple_tooltip_adder::SimpleTooltipAdderBase;
use crate::de::hysky::skyblocker::utils::container::container_matcher::ContainerMatcher;
use crate::de::hysky::skyblocker::utils::container::regex_container_matcher::RegexContainerMatcher;
use crate::de::hysky::skyblocker::utils::container::regex_container_matcher::RegexContainerMatcherBase;
use crate::de::hysky::skyblocker::utils::container::tooltip_adder::TooltipAdder;
use crate::de::hysky::skyblocker::utils::item_utils::ItemUtils;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::screens::inventory::container_screen::ContainerScreen;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::world::inventory::slot::Slot;
use mcrust::net::minecraft::world::item::items;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;
use mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::ContainerScreenHost as _;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final Pattern ORDER_AMOUNT_PATTERN = Pattern.compile("(?:Order|Offer) amount: ([0-9,]+)x");
pub static ORDER_AMOUNT_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("(?:Order|Offer) amount: ([0-9,]+)x"));
/// java: private static final Pattern UNIT_PRICE_PATTERN = Pattern.compile("Price per unit: ([0-9,.]+) coins");
pub static UNIT_PRICE_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("Price per unit: ([0-9,.]+) coins"));
/// java: private static final String CREATE_BUY_ORDER_TEXT = "Create Buy Order";
pub const CREATE_BUY_ORDER_TEXT: &str = "Create Buy Order";
/// java: private static final String CREATE_SELL_OFFER_TEXT = "Create Sell Offer";
pub const CREATE_SELL_OFFER_TEXT: &str = "Create Sell Offer";
/// java: private static final Pattern ORDER_PATTERN = Pattern.compile("- ([0-9,.]+) coins? each \\| ([0-9,]+)x (in|from) ([0-9,]+) (order|offer)s?");
pub static ORDER_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("- ([0-9,.]+) coins? each \\| ([0-9,]+)x (in|from) ([0-9,]+) (order|offer)s?"));
/// java: public static final BazaarOrderTracker INSTANCE = new BazaarOrderTracker();
pub static INSTANCE: compat::MainThread<compat::Shared<BazaarOrderTracker>> = compat::MainThread::new(|| BazaarOrderTracker::new());

/// java: public class BazaarOrderTracker extends SimpleTooltipAdder
#[derive(Clone)]
pub struct BazaarOrderTracker {
    /// java: extends SimpleTooltipAdder
    pub base: SimpleTooltipAdderBase,
    /// java: private final Int2ObjectMap<Order> orders = new Int2ObjectOpenHashMap<>();
    pub orders: std::collections::HashMap<i32, Order>,
}

impl std::ops::Deref for BazaarOrderTracker {
    type Target = SimpleTooltipAdderBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for BazaarOrderTracker {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl BazaarOrderTracker {
    /// java: private BazaarOrderTracker()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = SimpleTooltipAdderBase::new_in(".* ➜ .*".to_string(), 0, move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                orders: std::collections::HashMap::new(),
            });
            ((__h.clone() as compat::Shared<dyn SimpleTooltipAdder>), __h)
        });
        __this
    }

    /// java: void clearOrders()
    pub fn clear_orders(__this: &compat::Shared<Self>) {
        __this.lock().borrow_mut().orders.clear();
    }

    /// java: void processOrder(ItemStack stack, int slotId)
    pub fn process_order(__this: &compat::Shared<Self>, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, mut slot_id: i32) {
        let mut matchers: Vec<compat::regex::Matcher> = ItemUtils::get_lore_line_if_match_item_stack_pattern(stack, &vec![UNIT_PRICE_PATTERN.clone(), ORDER_AMOUNT_PATTERN.clone()]);
        if (matchers.len() as i32) < 2 {
            return;
        }
        let mut skyblock_id: String = stack.get_skyblock_id();
        let mut unit_price: f64 = (((matchers[(0) as usize]).group(1).unwrap()).replace(",", "")).trim().parse::<f64>().expect("NumberFormatException");
        let mut amount: i32 = (((matchers[(1) as usize]).group(1).unwrap()).replace(",", "")).parse::<i32>().expect("NumberFormatException");
        let __h0 = Order::new(skyblock_id.clone(), unit_price, amount, (slot_id < 18));
        __this.lock().borrow_mut().orders.insert(slot_id, __h0);
    }

    /// java: private void addOrderMarker(List<Component> lines, boolean sell)
    pub fn add_order_marker(__this: &compat::Shared<Self>, lines: &mut Vec<MutableComponent>, mut sell: bool) {
        // java: if (!(client.screen instanceof ContainerScreen screen)) return;
        if !matches!(compat::minecraft().gui.screen_kind, mcrust::net::minecraft::client::gui::gui::ScreenKind::Container) {
            return;
        }
        // java: String skyblockId = screen.getMenu().slots.get(13).getItem().getSkyblockId(); - the screen's menu is the player's open one
        let mut skyblock_id: String = compat::with_open_menu(|menu, containers| menu.slots[13].get_item(containers).duplicate()).unwrap_or_else(mcrust::net::minecraft::world::item::item_stack::empty).get_skyblock_id();
        let mut your_orders: Vec<Order> = compat::sorted_by(__this.lock().borrow().orders.values().cloned().filter(|o| (o.sell() == sell)).filter(|o| (o.skyblock_id() == skyblock_id)), |a, b| { let key = |__a0: &Order| __a0.unit_price(); key(a).total_cmp(&key(b)) }).collect::<Vec<_>>();
        if your_orders.is_empty() {
            return;
        }
        if !sell {
            your_orders = your_orders.iter().rev().cloned().collect::<Vec<_>>();
        }
        {
            let mut i: i32 = 0;
            let mut your_orders_index: i32 = 0;
            while i < (lines.len() as i32) && your_orders_index < (your_orders.len() as i32) {
                'continue_1: {
                    let line = &lines[(i) as usize];
                    let mut matcher: compat::regex::Matcher = ORDER_PATTERN.matcher(&line.get_string());
                    if !matcher.matches() {
                        break 'continue_1;
                    }
                    let mut unit_price: f64 = (matcher.group(1).unwrap().replace(",", "")).trim().parse::<f64>().expect("NumberFormatException");
                    while your_orders_index < (your_orders.len() as i32) && if sell { ((your_orders[(your_orders_index) as usize]).unit_price() < unit_price) } else { ((your_orders[(your_orders_index) as usize]).unit_price() > unit_price) } {
                        your_orders_index += 1;
                    }
                    let mut your_orders_count: i32 = 0;
                    let mut your_orders_amount: i32 = 0;
                    while your_orders_index < (your_orders.len() as i32) && (your_orders[(your_orders_index) as usize]).unit_price() == unit_price {
                        your_orders_count += 1;
                        your_orders_amount = your_orders_amount.wrapping_add((your_orders[(your_orders_index) as usize]).amount());
                        your_orders_index += 1;
                    }
                    if your_orders_count == 0 {
                        break 'continue_1;
                    }
                    lines.insert(({ i += 1; i }) as usize, (component::literal("  - ")).with_style_format(ChatFormatting::DarkGray).append((component::translatable_args("skyblocker.config.helpers.bazaar.orderTrackerTooltip", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::literal(&your_orders_amount.to_string()).with_style_format(ChatFormatting::Green)), mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::literal(&your_orders_count.to_string()).with_style_format(ChatFormatting::White))])).with_style_format(ChatFormatting::Gray)));
                }
                i += 1;
            }
        }
    }

}

impl SimpleTooltipAdder for BazaarOrderTracker {
    fn simple_tooltip_adder_base(&self) -> &SimpleTooltipAdderBase {
        &self.base
    }
    fn simple_tooltip_adder_base_mut(&mut self) -> &mut SimpleTooltipAdderBase {
        &mut self.base
    }

}

impl RegexContainerMatcher for BazaarOrderTracker {
    fn regex_container_matcher_base(&self) -> &RegexContainerMatcherBase {
        &self.base.base
    }
    fn regex_container_matcher_base_mut(&mut self) -> &mut RegexContainerMatcherBase {
        &mut self.base.base
    }

}

impl ContainerMatcher for BazaarOrderTracker {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public boolean isEnabled() (dispatch: the body for this class)
    fn is_enabled(&self) -> fn(&compat::Shared<dyn ContainerMatcher>) -> bool {
        BazaarOrderTracker::is_enabled_dyn
    }

    /// java: @Override public boolean test(Screen screen) (dispatch: the body for this class)
    fn test(&self) -> fn(&compat::Shared<dyn ContainerMatcher>, &mcrust::net::minecraft::client::gui::screens::screen::Screen) -> bool {
        <dyn RegexContainerMatcher>::test_from::<Self>
    }

}

impl TooltipAdder for BazaarOrderTracker {
    /// java: @Override public void addToTooltip(@Nullable Slot focusedSlot, ItemStack stack, List<Component> lines) (dispatch: the body for this class)
    fn add_to_tooltip(&self) -> fn(&compat::Shared<dyn TooltipAdder>, Option<Slot>, &mcrust::net::minecraft::world::item::item_stack::ItemStack, &mut Vec<MutableComponent>) {
        BazaarOrderTracker::add_to_tooltip_dyn
    }

    /// java: @Override public int getPriority() (dispatch: the body for this class)
    fn get_priority(&self) -> fn(&compat::Shared<dyn TooltipAdder>) -> i32 {
        <dyn SimpleTooltipAdder>::get_priority_from::<Self>
    }

}

impl BazaarOrderTracker {
    /// java: @Override public boolean isEnabled()
    pub fn is_enabled_impl(__this: &compat::Shared<Self>) -> bool {
        return SkyblockerConfigManager::get().helpers.bazaar.enable_order_tracker;
    }

    /// BazaarOrderTracker.isEnabled as the dispatch of ContainerMatcher.isEnabled
    pub fn is_enabled_dyn(__this: &compat::Shared<dyn ContainerMatcher>) -> bool {
        Self::is_enabled_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public void addToTooltip(@Nullable Slot focusedSlot, ItemStack stack, List<Component> lines)
    pub fn add_to_tooltip_impl(__this: &compat::Shared<Self>, mut focused_slot: Option<Slot>, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, lines: &mut Vec<MutableComponent>) {
        if stack.is_item(&items::items().get("filled_map")) && CREATE_BUY_ORDER_TEXT == compat::component_of(stack.get_hover_name()).get_string() {
            BazaarOrderTracker::add_order_marker(__this, lines, false);
        } else if stack.is_item(&items::items().get("map")) && CREATE_SELL_OFFER_TEXT == compat::component_of(stack.get_hover_name()).get_string() {
            BazaarOrderTracker::add_order_marker(__this, lines, true);
        }
    }

    /// BazaarOrderTracker.addToTooltip as the dispatch of TooltipAdder.addToTooltip
    pub fn add_to_tooltip_dyn(__this: &compat::Shared<dyn TooltipAdder>, __p0: Option<Slot>, __p1: &mcrust::net::minecraft::world::item::item_stack::ItemStack, __p2: &mut Vec<MutableComponent>) {
        Self::add_to_tooltip_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2)
    }

}

/// java: private record Order(String skyblockId, double unitPrice, int amount, boolean sell)
#[derive(Clone, Debug, PartialEq)]
pub struct Order {
    /// java: String skyblockId
    pub skyblock_id: String,
    /// java: double unitPrice
    pub unit_price: f64,
    /// java: int amount
    pub amount: i32,
    /// java: boolean sell
    pub sell: bool,
}

impl Order {
    /// java: the canonical record constructor
    pub fn new(mut skyblock_id: String, mut unit_price: f64, mut amount: i32, mut sell: bool) -> Self {
        Self { skyblock_id, unit_price, amount, sell }
    }

    /// java: the record accessor `skyblockId()`
    pub fn skyblock_id(&self) -> String {
        self.skyblock_id.clone()
    }

    /// java: the record accessor `unitPrice()`
    pub fn unit_price(&self) -> f64 {
        self.unit_price
    }

    /// java: the record accessor `amount()`
    pub fn amount(&self) -> i32 {
        self.amount
    }

    /// java: the record accessor `sell()`
    pub fn sell(&self) -> bool {
        self.sell
    }

}

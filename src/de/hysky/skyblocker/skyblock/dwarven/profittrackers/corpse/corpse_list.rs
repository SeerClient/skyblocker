//! Ported from de.hysky.skyblocker.skyblock.dwarven.profittrackers.corpse.CorpseList by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::dwarven::corpse_type::CorpseType;
use crate::de::hysky::skyblocker::skyblock::dwarven::profittrackers::corpse::corpse_loot::CorpseLoot;
use crate::de::hysky::skyblocker::skyblock::dwarven::profittrackers::corpse::corpse_profit_tracker;
use crate::de::hysky::skyblocker::skyblock::dwarven::profittrackers::corpse::reward::Reward;
use crate::de::hysky::skyblocker::skyblock::dwarven::profittrackers::corpse::reward_list::AbstractEntry;
use crate::de::hysky::skyblocker::skyblock::dwarven::profittrackers::corpse::reward_list::AbstractEntryBase;
use crate::de::hysky::skyblocker::skyblock::dwarven::profittrackers::corpse::reward_list::EmptyEntry;
use crate::de::hysky::skyblocker::skyblock::dwarven::profittrackers::corpse::reward_list::SingleEntry;
use crate::de::hysky::skyblocker::skyblock::dwarven::profittrackers::corpse::reward_list::number_format_instance;
use crate::de::hysky::skyblocker::skyblock::itemlist::item_repository::ItemRepository;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use crate::de::hysky::skyblocker::utils::formatters;
use crate::de::hysky::skyblocker::utils::render::gui_helper::GuiHelper;
use mcrust::mcrust::modding::widgets::ModList;
use mcrust::mcrust::modding::widgets::ModListEntry;
use mcrust::mcrust::modding::widgets::ModSelectionList;
use mcrust::mcrust::modding::widgets::WidgetRenderContext;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::components::abstract_selection_list::AbstractSelectionList;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::string_widget;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::client::gui::components::renderable::Renderable as _;

/// java: private static final int BORDER_COLOR = 0xFF6C7086;
pub const BORDER_COLOR: i32 = 0xFF6C7086u32 as i32;
/// java: private static final int INNER_MARGIN = 2;
pub const INNER_MARGIN: i32 = 2;

/// java: public class CorpseList extends ContainerObjectSelectionList<RewardList.AbstractEntry>
pub struct CorpseList {
    /// java: extends net.minecraft.client.gui.components.ContainerObjectSelectionList
    pub base: ModSelectionList,
}

impl std::ops::Deref for CorpseList {
    type Target = AbstractSelectionList;
    fn deref(&self) -> &Self::Target {
        &self.base.list
    }
}

impl std::ops::DerefMut for CorpseList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base.list
    }
}

impl CorpseList {
    /// java: public CorpseList(Minecraft client, int width, int height, int entryHeight, List<CorpseLoot> lootList)
    pub fn new(client: &mut mcrust::net::minecraft::client::minecraft::Minecraft, mut width: i32, mut height: i32, mut entry_height: i32, mut loot_list: Vec<compat::Shared<CorpseLoot>>) -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            base: mcrust::mcrust::modding::widgets::ModSelectionList::new(mcrust::mcrust::modding::widgets::ListKind::Container, width, height, 0, entry_height),
        });
        if loot_list.is_empty() {
            CorpseList::add_empty_entry(&__this);
            CorpseList::add_empty_entry(&__this);
            CorpseList::add_empty_entry(&__this);
            let __entry = (SingleEntry::new_2(component::literal("Your corpse history list is empty :(").with_style_format(ChatFormatting::Red), false) as compat::Shared<dyn AbstractEntry>);
            compat::write(&__this, |__t| { let __l = &mut __t.base; (__l.add_entry(Box::new(compat::EntryAdapter::new(__entry))) as i32) });
            return __this.clone();
        }
        for i in 0..(loot_list.len() as i32) {
            let mut loot: compat::Shared<CorpseLoot> = loot_list[(i) as usize].clone();
            let mut r#type: CorpseType = CorpseLoot::corpse_type(&loot);
            let __entry = (SingleEntry::new(component::literal(&format!("{} Corpse", compat::capitalize_fully(r#type.name()))).with_style_format(r#type.fields().color)) as compat::Shared<dyn AbstractEntry>);
            compat::write(&__this, |__t| { let __l = &mut __t.base; (__l.add_entry(Box::new(compat::EntryAdapter::new(__entry))) as i32) });
            // java: addEntry(new RewardList.SingleEntry(Component.literal(Formatters.DATE_FORMATTER.format(LocalDateTime.ofInstant(loot.timestamp(), ZoneId.systemDefault()))).withStyle(ChatFormatting.LIGHT_PURPLE)));
            // (a `LocalDateTime` carries no instant, so the formatter's own zone does not apply: the system zone's wall time)
            {
                let __millis: i64 = match CorpseLoot::timestamp(&loot).duration_since(std::time::UNIX_EPOCH) { Ok(d) => d.as_millis() as i64, Err(e) => -(e.duration().as_millis() as i64) };
                let __date = mcrust::java::time::zoned_date_time::ZonedDateTime::of_instant(mcrust::java::time::instant::Instant::of_epoch_milli(__millis), mcrust::java::time::zone_id::ZoneId::system_default());
                let __entry = (SingleEntry::new(component::literal(&crate::de::hysky::skyblocker::utils::formatters::DATE_FORMATTER.get_mut().format(__date)).with_style_format(ChatFormatting::LightPurple)) as compat::Shared<dyn AbstractEntry>);
                compat::write(&__this, |__t| { let __l = &mut __t.base; (__l.add_entry(Box::new(compat::EntryAdapter::new(__entry))) as i32) });
            }
            let mut entries: Vec<compat::Shared<Reward>> = CorpseLoot::rewards(&loot);
            for reward in entries.iter() {
                let mut item_name: MutableComponent = CorpseList::get_item_name(&Reward::item_id(reward));
                if corpse_profit_tracker::PRICELESS_ITEMS.iter().any(|e| *e == Reward::item_id(reward)) {
                    let __entry = (MultiEntry::new2(item_name.clone(), Reward::amount(reward)) as compat::Shared<dyn AbstractEntry>);
                    compat::write(&__this, |__t| { let __l = &mut __t.base; (__l.add_entry(Box::new(compat::EntryAdapter::new(__entry))) as i32) });
                } else {
                    let __entry = (MultiEntry::new(item_name.clone(), Reward::amount(reward), Reward::price_per_unit(reward)) as compat::Shared<dyn AbstractEntry>);
                    compat::write(&__this, |__t| { let __l = &mut __t.base; (__l.add_entry(Box::new(compat::EntryAdapter::new(__entry))) as i32) });
                }
            }
            if r#type != CorpseType::Lapis && r#type != CorpseType::Unknown {
                let __entry = (MultiEntry::new4((r#type).get_key_price(), true) as compat::Shared<dyn AbstractEntry>);
                compat::write(&__this, |__t| { let __l = &mut __t.base; (__l.add_entry(Box::new(compat::EntryAdapter::new(__entry))) as i32) });
            }
            if CorpseLoot::is_price_data_complete(&loot) {
                let __entry = (MultiEntry::new_1(CorpseLoot::profit(&loot)) as compat::Shared<dyn AbstractEntry>);
                compat::write(&__this, |__t| { let __l = &mut __t.base; (__l.add_entry(Box::new(compat::EntryAdapter::new(__entry))) as i32) });
            } else {
                let __entry = (SingleEntry::new((component::literal("Price data incomplete, can't calculate profit")).with_style_format(ChatFormatting::Red)) as compat::Shared<dyn AbstractEntry>);
                compat::write(&__this, |__t| { let __l = &mut __t.base; (__l.add_entry(Box::new(compat::EntryAdapter::new(__entry))) as i32) });
            }
            if i < (loot_list.len() as i32).wrapping_sub(1) {
                CorpseList::add_empty_entry(&__this);
                CorpseList::add_empty_entry(&__this);
            }
        }
        __this
    }

    /// java: public static Component getItemName(String itemId)
    pub fn get_item_name(item_id: &str) -> MutableComponent {
        return match item_id {
            corpse_profit_tracker::GLACITE_POWDER => component::literal("Glacite Powder").with_style_format(ChatFormatting::Aqua),
            corpse_profit_tracker::OPAL_CRYSTAL => component::literal("Opal Crystal").with_style_format(ChatFormatting::White),
            corpse_profit_tracker::ONYX_CRYSTAL => component::literal("Onyx Crystal").with_style_format(ChatFormatting::DarkGray),
            corpse_profit_tracker::AQUAMARINE_CRYSTAL => component::literal("Aquamarine Crystal").with_style_format(ChatFormatting::Blue),
            corpse_profit_tracker::PERIDOT_CRYSTAL => component::literal("Peridot Crystal").with_style_format(ChatFormatting::DarkGreen),
            corpse_profit_tracker::CITRINE_CRYSTAL => component::literal("Citrine Crystal").with_style_format(ChatFormatting::DarkRed),
            corpse_profit_tracker::RUBY_CRYSTAL => component::literal("Ruby Crystal").with_style_format(ChatFormatting::Red),
            corpse_profit_tracker::JASPER_CRYSTAL => component::literal("Jasper Crystal").with_style_format(ChatFormatting::LightPurple),
            corpse_profit_tracker::ENCHANTMENT_ICE_COLD_1 => component::literal("Enchanted Book (Ice Cold I)").with_style_format(ChatFormatting::White),
            _ => 'yield_1: {
                let mut item_stack: Option<compat::Shared<FlexibleItemStack>> = ItemRepository::get_item_stack(item_id);
                if let Some(item_stack) = &item_stack {
                } else {
                    logger().error(&format!("Item stack for item ID {} is null", item_id));
                    break 'yield_1 component::empty();
                }
                break 'yield_1 compat::component_of(FlexibleItemStack::get_stack_or_throw(item_stack.as_ref().unwrap()).get_hover_name());
            }
        };
    }

    /// java: private void addEmptyEntry()
    pub fn add_empty_entry(__this: &compat::Shared<Self>) {
        let __entry = (EmptyEntry::new() as compat::Shared<dyn AbstractEntry>);
        compat::write(__this, |__t| { let __l = &mut __t.base; (__l.add_entry(Box::new(compat::EntryAdapter::new(__entry))) as i32) });
    }

    /// java: @Override public int getRowWidth()
    pub fn get_row_width(__this: &compat::Shared<Self>) -> i32 {
        return 500i32.min((__this.lock().borrow().get_width()).wrapping_sub(24));
    }

    /// java: @Override public void refreshScrollAmount()
    pub fn refresh_scroll_amount(__this: &compat::Shared<Self>) {
        compat::write(__this, |__t| __t.base.list.refresh_scroll_amount());
        // `entry.getX()` / `entry.getY()`: the row's box in the list (lent to this call), not the row object's copy,
        // which is only brought up to date when the row is drawn.
        let (__entries, __w, __h) = compat::write(__this, |__t| {
            let __l = &mut __t.base;
            let __entries = (0..__l.entry_count()).filter_map(|__i| compat::entry_at::<dyn AbstractEntry>(__l, __i).map(|__e| (__e, __l.list.entry(__i).get_x(), __l.list.entry(__i).get_y()))).collect::<Vec<_>>();
            (__entries, __l.list.width, __l.list.height)
        });
        for (entry, __x, __y) in __entries {
            ({ let __f = compat::dispatch(&entry, |__t| __t.reposition_elements()); __f(&entry, __x, __y, __w, __h) });
        }
    }

}

/// CorpseList's list (the port's `ModSelectionList`) until the game takes it (`compat::attach_list`)
impl compat::ListHalf for CorpseList {
    fn list_half(&mut self) -> &mut ModSelectionList {
        &mut self.base
    }
}

/// CorpseList's overrides of the list's own methods, asked by the port's list bodies
impl ModList for compat::ListClass<CorpseList> {
    /// java: @Override public int getRowWidth()
    fn get_row_width(&self, list: &AbstractSelectionList, super_answer: i32) -> i32 {
        self.ask(list, super_answer, |__h| CorpseList::get_row_width(__h))
    }

}

// NOTE(fabric2seer): getRowWidth, refreshScrollAmount override(s) of ContainerObjectSelectionList - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them

/// java: public static class MultiEntry extends RewardList.AbstractEntry
pub struct MultiEntry {
    /// java: extends AbstractEntry
    pub base: AbstractEntryBase,
    /// java: protected @Nullable StringWidget itemName;
    pub item_name: Option<compat::WidgetRef>,
    /// java: protected @Nullable StringWidget amount = null;
    pub amount: Option<compat::WidgetRef>,
    /// java: protected @Nullable StringWidget totalPrice;
    pub total_price: Option<compat::WidgetRef>,
    /// java: protected @Nullable StringWidget pricePerUnit = null;
    pub price_per_unit: Option<compat::WidgetRef>,
}

impl std::ops::Deref for MultiEntry {
    type Target = AbstractEntryBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for MultiEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl MultiEntry {
    /// java: public MultiEntry(Component itemName, int amount, double pricePerUnit)
    pub fn new(mut item_name: MutableComponent, mut amount: i32, mut price_per_unit: f64) -> compat::Shared<Self> {
        let __init_item_name = (Some(compat::string_widget(0, 0, -1, 9, item_name.clone()))).map(compat::WidgetRef::value);
        let __init_amount = (Some(compat::string_widget(0, 0, -1, 9, (component::literal(&format!("x{}", formatters::INTEGER_NUMBERS.get_mut().format((amount) as f64)))).with_style_format(ChatFormatting::Aqua)))).map(compat::WidgetRef::value);
        let __init_total_price = Some(compat::WidgetRef::value(compat::string_widget(0, 0, -1, 9, component::literal(&format!("{} Coins", number_format_instance().format((amount as f64) * price_per_unit))).with_style_format(ChatFormatting::Gold))));
        let __init_price_per_unit = Some(compat::WidgetRef::value(compat::string_widget(0, 0, -1, 9, component::literal(&format!("{} each", number_format_instance().format(price_per_unit))).with_style_format(ChatFormatting::Gray))));
        let (_, __this) = AbstractEntryBase::new_in(move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                item_name: __init_item_name,
                amount: __init_amount,
                total_price: __init_total_price,
                price_per_unit: __init_price_per_unit,
            });
            ((__h.clone() as compat::Shared<dyn AbstractEntry>), __h)
        });
        compat::write(&__this, |__t| { let __s = &mut *__t; crate::de::hysky::skyblocker::skyblock::dwarven::profittrackers::corpse::reward_list::set_children(&mut __s.base, vec![__s.item_name.as_mut().unwrap(), __s.amount.as_mut().unwrap(), __s.total_price.as_mut().unwrap(), __s.price_per_unit.as_mut().unwrap()]) });
        __this
    }

    /// java: public MultiEntry(Component itemName, int amount)
    pub fn new2(mut item_name: MutableComponent, mut amount: i32) -> compat::Shared<Self> {
        let __init_item_name = (Some(compat::string_widget(0, 0, -1, 9, item_name.clone()))).map(compat::WidgetRef::value);
        let __init_amount = (Some(compat::string_widget(0, 0, -1, 9, (component::literal(&format!("x{}", formatters::INTEGER_NUMBERS.get_mut().format((amount) as f64)))).with_style_format(ChatFormatting::Aqua)))).map(compat::WidgetRef::value);
        let (_, __this) = AbstractEntryBase::new_in(move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                item_name: __init_item_name,
                amount: __init_amount,
                total_price: None,
                price_per_unit: None,
            });
            ((__h.clone() as compat::Shared<dyn AbstractEntry>), __h)
        });
        compat::write(&__this, |__t| { let __s = &mut *__t; crate::de::hysky::skyblocker::skyblock::dwarven::profittrackers::corpse::reward_list::set_children(&mut __s.base, vec![__s.item_name.as_mut().unwrap(), __s.amount.as_mut().unwrap()]) });
        __this
    }

    /// java: public MultiEntry(double profit)
    pub fn new_1(mut profit: f64) -> compat::Shared<Self> {
        let __init_item_name = (Some(compat::string_widget(0, 0, -1, 9, component::literal("Total Profit").with_style_formats(&[ChatFormatting::Bold, ChatFormatting::Gold])))).map(compat::WidgetRef::value);
        let __init_total_price = Some(compat::WidgetRef::value(compat::string_widget(0, 0, -1, 9, component::literal(&format!("{} Coins", number_format_instance().format(profit))).with_style_format(if profit > 0.0 { ChatFormatting::Green } else { ChatFormatting::Red }))));
        let (_, __this) = AbstractEntryBase::new_in(move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                item_name: __init_item_name,
                amount: None,
                total_price: __init_total_price,
                price_per_unit: None,
            });
            ((__h.clone() as compat::Shared<dyn AbstractEntry>), __h)
        });
        compat::write(&__this, |__t| { let __s = &mut *__t; crate::de::hysky::skyblocker::skyblock::dwarven::profittrackers::corpse::reward_list::set_children(&mut __s.base, vec![__s.item_name.as_mut().unwrap(), __s.total_price.as_mut().unwrap()]) });
        __this
    }

    /// java: public MultiEntry(double keyPrice, boolean isKey)
    pub fn new4(mut key_price: f64, mut is_key: bool) -> compat::Shared<Self> {
        let (_, __this) = AbstractEntryBase::new_in(move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                item_name: None,
                amount: None,
                total_price: None,
                price_per_unit: None,
            });
            ((__h.clone() as compat::Shared<dyn AbstractEntry>), __h)
        });
        if !is_key {
            panic!("IllegalArgumentException: {}", "This constructor is only for key entries");
        }
        { let __v = (Some(compat::string_widget(0, 0, -1, 9, component::literal("Key Price").with_style_formats(&[ChatFormatting::Red, ChatFormatting::Bold])))).map(compat::WidgetRef::value); compat::write(&__this, |__t| { let __s = &mut *__t; __s.item_name = __v; }); }
        { let __v = (Some(compat::string_widget(0, 0, -1, 9, component::literal("x1").with_style_format(ChatFormatting::Aqua)))).map(compat::WidgetRef::value); compat::write(&__this, |__t| { let __s = &mut *__t; __s.amount = __v; }); }
        { let __v = Some(compat::WidgetRef::value(compat::string_widget(0, 0, -1, 9, component::literal(&format!("-{} Coins", number_format_instance().format(key_price))).with_style_format(ChatFormatting::Red)))); compat::write(&__this, |__t| { let __s = &mut *__t; __s.total_price = __v; }); }
        compat::write(&__this, |__t| { let __s = &mut *__t; crate::de::hysky::skyblocker::skyblock::dwarven::profittrackers::corpse::reward_list::set_children(&mut __s.base, vec![__s.item_name.as_mut().unwrap(), __s.amount.as_mut().unwrap(), __s.total_price.as_mut().unwrap()]) });
        __this
    }

    /// java: private static void position(StringWidget widget, int start, int width, int y)
    pub fn position(widget: &mut AbstractWidget, mut start: i32, mut width: i32, mut y: i32) {
        widget.set_max_width(width, string_widget::TextOverflow::Scrolling);
        // `widget.getWidth()` is `StringWidget.getWidth`: `min(maxWidth, font.width(message))` once a max width is set.
        let text_width = compat::font().width_of(&widget.get_message_component());
        let widget_width = string_widget::width(widget.max_width, text_width);
        widget.set_x(start.wrapping_add((width.wrapping_sub(widget_width)).wrapping_div(2)));
        widget.set_y(y);
    }

}

impl AbstractEntry for MultiEntry {
    fn abstract_entry_base(&self) -> &AbstractEntryBase {
        &self.base
    }
    fn abstract_entry_base_mut(&mut self) -> &mut AbstractEntryBase {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void extractContent(GuiGraphicsExtractor graphics, int mouseX, int mouseY, boolean hovered, float a) (dispatch: the body for this class)
    fn extract_content(&self) -> fn(&compat::Shared<dyn AbstractEntry>, &mut GuiGraphicsExtractor, i32, i32, bool, f32) {
        MultiEntry::extract_content_dyn
    }

    /// java: @Override public void repositionElements(int x, int y, int width, int entryHeight) (dispatch: the body for this class)
    fn reposition_elements(&self) -> fn(&compat::Shared<dyn AbstractEntry>, i32, i32, i32, i32) {
        MultiEntry::reposition_elements_dyn
    }

}

impl MultiEntry {
    /// java: @Override public void extractContent(GuiGraphicsExtractor graphics, int mouseX, int mouseY, boolean hovered, float a)
    pub fn extract_content_impl(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut hovered: bool, mut a: f32) {
        let mut x: i32 = __this.lock().borrow().get_x();
        let mut y: i32 = __this.lock().borrow().get_y();
        let mut entry_width: i32 = __this.lock().borrow().get_width();
        let mut entry_height: i32 = __this.lock().borrow().get_height();
        GuiHelper::border(graphics, x, y, entry_width, entry_height.wrapping_add(1), BORDER_COLOR);
        GuiHelper::border(graphics, x.wrapping_add(entry_width.wrapping_div(3)), y, (entry_width.wrapping_div(6)).wrapping_add(2), entry_height.wrapping_add(1), BORDER_COLOR);
        GuiHelper::border(graphics, x.wrapping_add(entry_width.wrapping_div(2)), y, entry_width.wrapping_div(4), entry_height.wrapping_add(1), BORDER_COLOR);
        let mut entry_y: i32 = y.wrapping_add(INNER_MARGIN);
        if compat::read(__this, |__t| __t.item_name.is_some()) {
            let __wa1 = entry_y;
            compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.item_name.as_mut().unwrap(); __s.base.__widgets.with(&mut __s.base.__row, __r, |__w| __w.set_y(__wa1)) });
            // java: itemName.setMaxWidth(entryWidth / 3 - 2 * INNER_MARGIN, StringWidget.TextOverflow.SCROLLING);
            let __max_width = (entry_width.wrapping_div(3)).wrapping_sub(2i32.wrapping_mul(INNER_MARGIN));
            compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.item_name.as_mut().unwrap(); __s.base.__widgets.with(&mut __s.base.__row, __r, |__w| __w.set_max_width(__max_width, mcrust::net::minecraft::client::gui::components::string_widget::TextOverflow::Scrolling)) });
            // java: itemName.extractRenderState(graphics, mouseX, mouseY, a); - the row's widgets are drawn by the game's widget
            // pass right after this body (`context.extract_widgets`), at the position set above
        }
        if compat::read(__this, |__t| __t.amount.is_some()) {
            let __wa7 = entry_y;
            compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.amount.as_mut().unwrap(); __s.base.__widgets.with(&mut __s.base.__row, __r, |__w| __w.set_y(__wa7)) });
            // java: amount.extractRenderState(graphics, mouseX, mouseY, a); - the row's widgets are drawn by the game's widget
            // pass right after this body (`context.extract_widgets`), at the position set above
        }
        if compat::read(__this, |__t| __t.total_price.is_some()) {
            let __wa12 = entry_y;
            compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.total_price.as_mut().unwrap(); __s.base.__widgets.with(&mut __s.base.__row, __r, |__w| __w.set_y(__wa12)) });
            // java: totalPrice.extractRenderState(graphics, mouseX, mouseY, a); - the row's widgets are drawn by the game's widget
            // pass right after this body (`context.extract_widgets`), at the position set above
        }
        if compat::read(__this, |__t| __t.price_per_unit.is_some()) {
            let __wa17 = entry_y;
            compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.price_per_unit.as_mut().unwrap(); __s.base.__widgets.with(&mut __s.base.__row, __r, |__w| __w.set_y(__wa17)) });
            // java: pricePerUnit.extractRenderState(graphics, mouseX, mouseY, a); - the row's widgets are drawn by the game's widget
            // pass right after this body (`context.extract_widgets`), at the position set above
        }
    }

    /// MultiEntry.extractContent as the dispatch of AbstractEntry.extractContent
    pub fn extract_content_dyn(__this: &compat::Shared<dyn AbstractEntry>, __p0: &mut GuiGraphicsExtractor, __p1: i32, __p2: i32, __p3: bool, __p4: f32) {
        Self::extract_content_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2, __p3, __p4)
    }

    /// java: @Override public void repositionElements(int x, int y, int width, int entryHeight)
    pub fn reposition_elements_impl(__this: &compat::Shared<Self>, mut x: i32, mut y: i32, mut width: i32, mut entry_height: i32) {
        if compat::read(__this, |__t| __t.item_name.is_some()) {
            let __wa1 = x.wrapping_add(INNER_MARGIN);
            compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.item_name.as_mut().unwrap(); __s.base.__widgets.with(&mut __s.base.__row, __r, |__w| __w.set_x(__wa1)) });
        }
        let mut entry_width: i32 = __this.lock().borrow().get_width();
        if compat::read(__this, |__t| __t.amount.is_some()) {
            compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.amount.as_mut().unwrap(); __s.base.__widgets.with(&mut __s.base.__row, __r, |__w| MultiEntry::position(__w, (x.wrapping_add(entry_width.wrapping_div(3))).wrapping_add(INNER_MARGIN), (entry_width.wrapping_div(6)).wrapping_sub(2i32.wrapping_mul(INNER_MARGIN)), y)) });
        }
        if compat::read(__this, |__t| __t.total_price.is_some()) {
            compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.total_price.as_mut().unwrap(); __s.base.__widgets.with(&mut __s.base.__row, __r, |__w| MultiEntry::position(__w, (x.wrapping_add(entry_width.wrapping_div(2))).wrapping_add(INNER_MARGIN), (entry_width.wrapping_div(4)).wrapping_sub(2i32.wrapping_mul(INNER_MARGIN)), y)) });
        }
        if compat::read(__this, |__t| __t.price_per_unit.is_some()) {
            compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.price_per_unit.as_mut().unwrap(); __s.base.__widgets.with(&mut __s.base.__row, __r, |__w| MultiEntry::position(__w, (x.wrapping_add((3i32.wrapping_mul(entry_width)).wrapping_div(4))).wrapping_add(INNER_MARGIN), (entry_width.wrapping_div(4)).wrapping_sub(2i32.wrapping_mul(INNER_MARGIN)), y)) });
        }
    }

    /// MultiEntry.repositionElements as the dispatch of AbstractEntry.repositionElements
    pub fn reposition_elements_dyn(__this: &compat::Shared<dyn AbstractEntry>, __p0: i32, __p1: i32, __p2: i32, __p3: i32) {
        Self::reposition_elements_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2, __p3)
    }

}

/// MultiEntry's row box and its widgets, lent by its `compat::EntryAdapter` for each call
impl compat::RowHalf for MultiEntry {
    fn entry_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry {
        &mut self.base.base
    }
    fn row_parts(&mut self) -> (&mut mcrust::net::minecraft::client::gui::screens::screen::Screen, &mut compat::WidgetCache) {
        let __s = &mut self.base;
        (&mut __s.__row, &mut __s.__widgets)
    }
}

/// MultiEntry as a row of the game's list (`addEntry` attaches it)
impl ModListEntry for compat::EntryAdapter<MultiEntry> {
    /// java: @Override public void extractContent(GuiGraphicsExtractor graphics, int mouseX, int mouseY, boolean hovered, float a)
    fn extract_content(&mut self, list: &mut ModSelectionList, index: usize, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, hovered: bool, a: f32) {
        self.lend(Some(list.list.entry(index)), |__h| ({ let __f = compat::dispatch(__h, |__t| __t.extract_content()); __f(&compat::view(__h, |a| a as compat::Shared<dyn AbstractEntry>), &mut context.graphics, mouse_x, mouse_y, hovered, a) }));
        // the row's widgets (Java's `child.extractRenderState(..)` over `children()`)
        context.extract_widgets(&mut self.row, mouse_x, mouse_y);
    }

    fn children(&mut self) -> Option<&mut mcrust::net::minecraft::client::gui::screens::screen::Screen> {
        Some(&mut self.row)
    }

}

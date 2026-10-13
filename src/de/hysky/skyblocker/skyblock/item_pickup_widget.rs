//! Ported from de.hysky.skyblocker.skyblock.ItemPickupWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::events::skyblock_events;
use crate::de::hysky::skyblocker::skyblock::itemlist::item_repository::ItemRepository;
use crate::de::hysky::skyblocker::skyblock::tabhud::config::option_widget_collector::OptionWidgetCollector;
use crate::de::hysky::skyblocker::skyblock::tabhud::util::ico;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::element::Element;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::element_collector::ElementCollector;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::separator_element::SeparatorElement;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element_based_widget::ElementBasedWidget;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element_based_widget::ElementBasedWidgetBase;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::HudWidget;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::HudWidgetBase;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::Information;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use crate::de::hysky::skyblocker::utils::formatters;
use crate::de::hysky::skyblocker::utils::formatters::Formatters;
use crate::de::hysky::skyblocker::utils::item_utils::ItemUtils;
use crate::de::hysky::skyblocker::utils::json_value_input::JsonValueInput;
use crate::de::hysky::skyblocker::utils::json_value_output::JsonValueOutput;
use crate::de::hysky::skyblocker::utils::neu_repo_manager::NEURepoManager;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler::Scheduler;
use mcrust::net::minecraft::chat_formatting;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::hover_event::HoverEvent;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::world::item::items;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final int LOBBY_CHANGE_DELAY = 60;
pub const LOBBY_CHANGE_DELAY: i32 = 60;
/// java: private static final String SACKS_MESSAGE_START = "[Sacks]";
pub const SACKS_MESSAGE_START: &str = "[Sacks]";
/// java: private static final Pattern CHANGE_REGEX = Pattern.compile("([+-])([\\d,]+) (.+) \\((.+)\\)");
pub static CHANGE_REGEX: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("([+-])([\\d,]+) (.+) \\((.+)\\)"));
/// java: private static @Nullable ItemPickupWidget instance;
pub static INSTANCE: compat::MainThread<Option<compat::Shared<ItemPickupWidget>>> = compat::MainThread::new(|| None);

/// java: @RegisterWidget public class ItemPickupWidget extends ElementBasedWidget
#[derive(Clone)]
pub struct ItemPickupWidget {
    /// java: extends ElementBasedWidget
    pub base: ElementBasedWidgetBase,
    /// java: private boolean changingLobby;
    pub changing_lobby: bool,
    /// java: private boolean sackNotifications;
    pub sack_notifications: bool,
    /// java: private boolean splitNotifications;
    pub split_notifications: bool,
    /// java: private boolean showItemName = true;
    pub show_item_name: bool,
    /// java: private float lifetime = 3;
    pub lifetime: f32,
    /// java: private final Object2ObjectOpenHashMap<String, ChangeData> addedCount = new Object2ObjectOpenHashMap<>();
    pub added_count: std::collections::HashMap<String, ChangeData>,
    /// java: private final Object2ObjectOpenHashMap<String, ChangeData> removedCount = new Object2ObjectOpenHashMap<>();
    pub removed_count: std::collections::HashMap<String, ChangeData>,
    /// java: private final Object2ObjectOpenHashMap<String, ChangeData> addedSackCount = new Object2ObjectOpenHashMap<>();
    pub added_sack_count: std::collections::HashMap<String, ChangeData>,
    /// java: private final Object2ObjectOpenHashMap<String, ChangeData> removedSackCount = new Object2ObjectOpenHashMap<>();
    pub removed_sack_count: std::collections::HashMap<String, ChangeData>,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl std::ops::Deref for ItemPickupWidget {
    type Target = ElementBasedWidgetBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for ItemPickupWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl ItemPickupWidget {
    /// java: public ItemPickupWidget()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = ElementBasedWidgetBase::new_in(component::literal("Items"), mcrust::net::minecraft::network::chat::text_color::TextColor::AQUA.get_value(), Information::new(Some("item_pickup".to_string()), Some(component::literal("Item Pickup"))), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                self_ref: std::sync::Weak::new(),
                changing_lobby: false,
                sack_notifications: false,
                split_notifications: false,
                show_item_name: true,
                lifetime: ((3) as f32) as f32,
                added_count: std::collections::HashMap::new(),
                removed_count: std::collections::HashMap::new(),
                added_sack_count: std::collections::HashMap::new(),
                removed_sack_count: std::collections::HashMap::new(),
            });
            compat::write(&__h, |__t| __t.self_ref = std::sync::Arc::downgrade(&__h));
            ((__h.clone() as compat::Shared<dyn ElementBasedWidget>), __h)
        });
        *INSTANCE.get_mut() = Some(__this.clone());
        // TODO(fabric2seer): ClientReceiveMessageEvents.ALLOW_GAME.register: no SeerClient equivalent yet
        //   java: ClientReceiveMessageEvents.ALLOW_GAME.register(instance::onChatMessage);
        let __this_j = compat::RenderThread::new(__this.clone());
        client_play_connection_events::JOIN.register(Arc::new(move |listener: &mut ClientPacketListener| {
            { let __v = true; compat::write(&(*__this_j.get()), |__t| __t.changing_lobby = __v); }
        }));
        let __this_2 = compat::RenderThread::new(__this.clone());
        skyblock_events::LOCATION_CHANGE.register(Arc::new(move |_| { let __this_1 = compat::RenderThread::new((*__this_2.get()).clone()); ({ let __r = scheduler::INSTANCE.get_mut(); let __f = compat::dispatch(__r, |__t| __t.schedule()); __f(__r, Arc::new(move || { { let __v = false; compat::write(&(*__this_1.get()), |__t| __t.changing_lobby = __v); } }), LOBBY_CHANGE_DELAY) }); }));
        __this
    }

    /// java: public static ItemPickupWidget getInstance()
    pub fn get_instance() -> compat::Shared<ItemPickupWidget> {
        return INSTANCE.get_mut().clone().unwrap();
    }

    /// java: private static FlexibleItemStack getItem(String itemName)
    pub fn get_item(item_name: &str) -> compat::Shared<FlexibleItemStack> {
        if NEURepoManager::is_loading() || !(ItemRepository::files_imported()) {
            return ItemUtils::get_named_placeholder(item_name);
        }
        // rust: port gap - `NEUItem.getSkyblockItemId` (the NEU repo parser is a `crate::unresolved` placeholder,
        // GAPS_batchB L1): the item found by name cannot be resolved to its id, so the placeholder is answered
        let _item = NEURepoManager::get_item_by_name(item_name).into_iter().flatten().next();
        return ItemUtils::get_named_placeholder(item_name);
        // TODO(fabric2seer): `.getSkyblockItemId(..)` on a NEUItem
        //   java: return NEURepoManager.getItemByName(itemName)
        //   java: 				.stream()
        //   java: 				.filter(Objects::nonNull)
        //   java: 				.findFirst()
        //   java: 				.map(NEUItem::getSkyblockItemId)
        //   java: 				.map(ItemRepository::getItemStack)
        //   java: 				.orElseGet(() -> ItemUtils.getNamedPlaceholder(itemName));
    }

    /// java: @SuppressWarnings("SameReturnValue") private boolean onChatMessage(Component message, boolean overlay)
    pub fn on_chat_message(__this: &compat::Shared<Self>, message: &MutableComponent, mut overlay: bool) -> bool {
        if !(chat_formatting::strip_formatting(&message.get_string()).starts_with(SACKS_MESSAGE_START)) {
            return true;
        }
        if !(compat::read(__this, |__t| __t.sack_notifications)) {
            return true;
        }
        let mut hover_event: Option<HoverEvent> = message.get_siblings()[0].get_style().get_hover_event().cloned();
        if hover_event.is_none() || hover_event.as_ref().unwrap().action() != mcrust::net::minecraft::network::chat::hover_event::Action::ShowText {
            return true;
        }
        // `((HoverEvent.ShowText) hoverEvent).value().getString()`
        let mut hover_message: String = hover_event.as_ref().unwrap().value_of_show_text().unwrap().get_string();
        let mut split: bool = compat::read(__this, |__t| __t.split_notifications);
        let mut matcher: compat::regex::Matcher = CHANGE_REGEX.matcher(&chat_formatting::strip_formatting(&hover_message));
        while matcher.find() {
            let mut item: mcrust::net::minecraft::world::item::item_stack::ItemStack = FlexibleItemStack::get_stack_or_throw(&ItemPickupWidget::get_item(matcher.group(3).as_deref().unwrap()));
            let mut count: i32 = Formatters::parse_number(matcher.group(2).as_deref().unwrap()).int_value();
            if matcher.group(1).as_deref() == Some(&*("+")) {
                ItemPickupWidget::update_count(__this, &mut (if split { compat::read(__this, |__t| __t.added_sack_count.clone()) } else { compat::read(__this, |__t| __t.added_count.clone()) }), &item, count);
            } else if matcher.group(1).as_deref() == Some(&*("-")) {
                ItemPickupWidget::update_count(__this, &mut (if split { compat::read(__this, |__t| __t.removed_sack_count.clone()) } else { compat::read(__this, |__t| __t.removed_count.clone()) }), &item, count);
            }
        }
        return true;
    }

    /// java: private @Nullable String checkNextItem(ChangeData entry)
    pub fn check_next_item(__this: &compat::Shared<Self>, entry: &ChangeData) -> Option<String> {
        let mut expiration_time: i64 = entry.last_change.wrapping_add(((compat::read(__this, |__t| __t.lifetime) * (1000 as f32))) as i64);
        if expiration_time < compat::millis() {
            return None;
        }
        return Some((if compat::read(__this, |__t| __t.show_item_name) { format!("{} ", compat::component_of(entry.item.get_hover_name()).get_string()) } else { " ".to_string() }));
    }

    /// java: public void onItemPickup(int slot, ItemStack newStack)
    pub fn on_item_pickup(__this: &compat::Shared<Self>, mut slot: i32, mut new_stack: mcrust::net::minecraft::world::item::item_stack::ItemStack) {
        if compat::read(__this, |__t| __t.changing_lobby) || mcrust::mcrust::modding::game::with_player(|_| ()).is_none() {
            return;
        }
        if compat::minecraft().gui.screen().is_some() {
            return;
        }
        if slot < 9 || slot >= 45 {
            return;
        }
        if slot >= 36 {
            slot = slot.wrapping_sub(36);
        }
        if slot == 8 {
            return;
        }
        let mut old_stack: mcrust::net::minecraft::world::item::item_stack::ItemStack = mcrust::mcrust::modding::game::with_player(|player| player.base.base.get_inventory().get_non_equipment_items()[(slot) as usize].clone()).unwrap();
        let mut count_diff: i32 = new_stack.get_count().wrapping_sub(old_stack.get_count());
        if std::ptr::addr_eq(std::sync::Arc::as_ptr(&new_stack.get_item()), std::sync::Arc::as_ptr(&items::items().get("air"))) {
            if std::ptr::addr_eq(std::sync::Arc::as_ptr(&old_stack.get_item()), std::sync::Arc::as_ptr(&items::items().get("air"))) {
                return;
            }
            ItemPickupWidget::update_count(__this, &mut compat::read(__this, |__t| __t.removed_count.clone()), &old_stack, old_stack.get_count());
            return;
        }
        new_stack = new_stack.copy();
        if count_diff > 0 {
            compat::write(__this, |__t| ItemPickupWidget::update_count(__this, &mut __t.added_count, &new_stack, count_diff));
        } else if count_diff < 0 {
            compat::write(__this, |__t| ItemPickupWidget::update_count(__this, &mut __t.removed_count, &new_stack, count_diff));
        }
    }

    /// java: private void updateCount(Object2ObjectOpenHashMap<String, ChangeData> map, ItemStack stack, int count)
    pub fn update_count(__this: &compat::Shared<Self>, map: &mut std::collections::HashMap<String, ChangeData>, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, mut count: i32) {
        let mut neu_id: String = stack.get_neu_name();
        if neu_id.is_empty() {
            //   java: neuId = stack.getHoverName().toString();
            neu_id = compat::component_of(stack.get_hover_name()).to_string();
        }
        //   java: map.compute(neuId, (_, existing) -> {
        //   java: 			int existingCount = existing == null ? 0 : existing.amount;
        //   java: 			return new ChangeData(stack, existingCount + count, System.currentTimeMillis());
        //   java: 		});
        let existing_count: i32 = map.get(&neu_id).map_or(0, |existing| existing.amount);
        map.insert(neu_id, ChangeData::new(stack.duplicate(), existing_count.wrapping_add(count), compat::millis()));
    }

}

impl ElementBasedWidget for ItemPickupWidget {
    fn element_based_widget_base(&self) -> &ElementBasedWidgetBase {
        &self.base
    }
    fn element_based_widget_base_mut(&mut self) -> &mut ElementBasedWidgetBase {
        &mut self.base
    }

    /// java: @Override public boolean shouldUpdateBeforeRendering() (dispatch: the body for this class)
    fn should_update_before_rendering(&self) -> fn(&compat::Shared<dyn ElementBasedWidget>) -> bool {
        ItemPickupWidget::should_update_before_rendering_dyn
    }

    /// java: @Override public void updateContent() (dispatch: the body for this class)
    fn update_content(&self) -> fn(&compat::Shared<dyn ElementBasedWidget>) {
        ItemPickupWidget::update_content_dyn
    }

    /// java: @Override protected void updateConfigContent(ElementCollector collector) (dispatch: the body for this class)
    fn update_config_content(&self) -> fn(&compat::Shared<dyn ElementBasedWidget>, &compat::Shared<dyn ElementCollector>) {
        ItemPickupWidget::update_config_content_dyn
    }

}

impl HudWidget for ItemPickupWidget {
    fn hud_widget_base(&self) -> &HudWidgetBase {
        &self.base.base
    }
    fn hud_widget_base_mut(&mut self) -> &mut HudWidgetBase {
        &mut self.base.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void getOptionWidgets(OptionWidgetCollector collector) (dispatch: the body for this class)
    fn get_option_widgets(&self) -> fn(&compat::Shared<dyn HudWidget>, &mut OptionWidgetCollector) {
        ItemPickupWidget::get_option_widgets_dyn
    }

    /// java: @Override public void load(JsonValueInput input) (dispatch: the body for this class)
    fn load(&self) -> fn(&compat::Shared<dyn HudWidget>, &JsonValueInput) {
        ItemPickupWidget::load_dyn
    }

    /// java: @Override public void save(JsonValueOutput output) (dispatch: the body for this class)
    fn save(&self) -> fn(&compat::Shared<dyn HudWidget>, &mut JsonValueOutput) {
        ItemPickupWidget::save_dyn
    }

    /// java: @Override public boolean shouldRender() (dispatch: the body for this class)
    fn should_render(&self) -> fn(&compat::Shared<dyn HudWidget>) -> bool {
        ItemPickupWidget::should_render_dyn
    }

    /// java: @Override protected final void extractWidgetRenderState(GuiGraphicsExtractor context, float delta) (dispatch: the body for this class)
    fn extract_widget_render_state(&self) -> fn(&compat::Shared<dyn HudWidget>, &mut GuiGraphicsExtractor, f32) {
        <dyn ElementBasedWidget>::extract_widget_render_state_from::<Self>
    }

    /// java: @Override protected void extractWidgetRenderStateForConfig(GuiGraphicsExtractor graphics, float delta) (dispatch: the body for this class)
    fn extract_widget_render_state_for_config(&self) -> fn(&compat::Shared<dyn HudWidget>, &mut GuiGraphicsExtractor, f32) {
        <dyn ElementBasedWidget>::extract_widget_render_state_for_config_from::<Self>
    }

    /// java: @Override public void updateConfigPreview() (dispatch: the body for this class)
    fn update_config_preview(&self) -> fn(&compat::Shared<dyn HudWidget>) {
        <dyn ElementBasedWidget>::update_config_preview_from::<Self>
    }

}

impl ElementCollector for ItemPickupWidget {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: public <T extends Element> T addElement(T c)
    fn add_element(&mut self, mut c: compat::Shared<dyn Element>) -> compat::Shared<dyn Element> {
        ({ let __f = compat::dispatch(&c, |__t| __t.set_parent()); __f(&c, self.hud_widget_base().as_parent()) });
        self.elements.push(c.clone());
        return c;
    }

}

impl ItemPickupWidget {
    /// java: @Override public boolean shouldUpdateBeforeRendering()
    pub fn should_update_before_rendering_impl(__this: &compat::Shared<Self>) -> bool {
        return true;
    }

    /// ItemPickupWidget.shouldUpdateBeforeRendering as the dispatch of ElementBasedWidget.shouldUpdateBeforeRendering
    pub fn should_update_before_rendering_dyn(__this: &compat::Shared<dyn ElementBasedWidget>) -> bool {
        Self::should_update_before_rendering_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public void updateContent()
    pub fn update_content_impl(__this: &compat::Shared<Self>) {
        for item in __this.lock().borrow().added_count.keys().cloned() {
            let mut entry: ChangeData = __this.lock().borrow().added_count.get(&item).cloned().unwrap();
            let Some(mut item_name) = ItemPickupWidget::check_next_item(__this, &entry) else {
                __this.lock().borrow_mut().added_count.remove(&item);
                continue;
            };
            if entry.item.is_empty() {
                continue;
            }
            ({ let __f = compat::dispatch(__this, |__t| __t.add_simple_ico_text_flexible_item_stack_string_chat_formatting_string()); __f(&compat::view(__this, |a| a as compat::Shared<dyn ElementCollector>), Some(FlexibleItemStack::new4(entry.item.clone())), &item_name, &ChatFormatting::Green, &formatters::DIFF_NUMBERS.get_mut().format((entry.amount) as f64)) });
        }
        for item in __this.lock().borrow().removed_count.keys().cloned() {
            let mut entry: ChangeData = __this.lock().borrow().removed_count.get(&item).cloned().unwrap();
            let Some(mut item_name) = ItemPickupWidget::check_next_item(__this, &entry) else {
                __this.lock().borrow_mut().removed_count.remove(&item);
                continue;
            };
            if entry.item.is_empty() {
                continue;
            }
            ({ let __f = compat::dispatch(__this, |__t| __t.add_simple_ico_text_flexible_item_stack_string_chat_formatting_string()); __f(&compat::view(__this, |a| a as compat::Shared<dyn ElementCollector>), Some(FlexibleItemStack::new4(entry.item.clone())), &item_name, &ChatFormatting::Red, &formatters::DIFF_NUMBERS.get_mut().format((entry.amount) as f64)) });
        }
        if compat::read(__this, |__t| __t.split_notifications) && !(__this.lock().borrow().added_sack_count.is_empty() && __this.lock().borrow().removed_sack_count.is_empty()) {
            { let (__a0,) = ((SeparatorElement::new(Some(component::null_to_empty(Some("Sacks")))) as compat::Shared<dyn Element>),); __this.lock().borrow_mut().add_element(__a0) };
            for item in __this.lock().borrow().added_sack_count.keys().cloned() {
                let mut entry: ChangeData = __this.lock().borrow().added_sack_count.get(&item).cloned().unwrap();
                let Some(mut item_name) = ItemPickupWidget::check_next_item(__this, &entry) else {
                    __this.lock().borrow_mut().added_sack_count.remove(&item);
                    continue;
                };
                ({ let __f = compat::dispatch(__this, |__t| __t.add_simple_ico_text_flexible_item_stack_string_chat_formatting_string()); __f(&compat::view(__this, |a| a as compat::Shared<dyn ElementCollector>), Some(FlexibleItemStack::new4(entry.item.clone())), &item_name, &ChatFormatting::Green, &formatters::DIFF_NUMBERS.get_mut().format((entry.amount) as f64)) });
            }
            for item in __this.lock().borrow().removed_sack_count.keys().cloned() {
                let mut entry: ChangeData = __this.lock().borrow().removed_sack_count.get(&item).cloned().unwrap();
                let Some(mut item_name) = ItemPickupWidget::check_next_item(__this, &entry) else {
                    __this.lock().borrow_mut().removed_sack_count.remove(&item);
                    continue;
                };
                ({ let __f = compat::dispatch(__this, |__t| __t.add_simple_ico_text_flexible_item_stack_string_chat_formatting_string()); __f(&compat::view(__this, |a| a as compat::Shared<dyn ElementCollector>), Some(FlexibleItemStack::new4(entry.item.clone())), &item_name, &ChatFormatting::Red, &formatters::DIFF_NUMBERS.get_mut().format((entry.amount) as f64)) });
            }
        }
    }

    /// ItemPickupWidget.updateContent as the dispatch of ElementBasedWidget.updateContent
    pub fn update_content_dyn(__this: &compat::Shared<dyn ElementBasedWidget>) {
        Self::update_content_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override protected void updateConfigContent(ElementCollector collector)
    pub fn update_config_content_impl(__this: &compat::Shared<Self>, collector: &compat::Shared<dyn ElementCollector>) {
        ({ let __f = compat::dispatch(collector, |__t| __t.add_simple_ico_text_flexible_item_stack_string_chat_formatting_string()); __f(collector, Some(ico::BONE.get_mut().clone()), "Bone ", &ChatFormatting::Green, "+64") });
        if compat::read(__this, |__t| __t.sack_notifications) {
            if compat::read(__this, |__t| __t.split_notifications) {
                { let (__a0,) = ((SeparatorElement::new(Some(component::null_to_empty(Some("Sacks")))) as compat::Shared<dyn Element>),); collector.lock().borrow_mut().add_element(__a0) };
            }
            ({ let __f = compat::dispatch(collector, |__t| __t.add_simple_ico_text_flexible_item_stack_string_chat_formatting_string()); __f(collector, Some(ico::BONE.get_mut().clone()), "Enchanted Bone ", &ChatFormatting::Green, "+1") });
        }
    }

    /// ItemPickupWidget.updateConfigContent as the dispatch of ElementBasedWidget.updateConfigContent
    pub fn update_config_content_dyn(__this: &compat::Shared<dyn ElementBasedWidget>, __p0: &compat::Shared<dyn ElementCollector>) {
        Self::update_config_content_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public void getOptionWidgets(OptionWidgetCollector collector)
    pub fn get_option_widgets_impl(__this: &compat::Shared<Self>, collector: &mut OptionWidgetCollector) {
        <dyn ElementBasedWidget>::get_option_widgets_body(&compat::view(__this, |a| a as compat::Shared<dyn ElementBasedWidget>), collector);
        let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        collector.yes_no_button(&component::translatable("skyblocker.config.uiAndVisuals.itemPickup.sackNotifications"), Arc::new(move |b| { let Some(__this_1) = __this_w1.get().upgrade() else { return Default::default(); }; { { let __v = b; compat::write((&__this_1), |__t| __t.sack_notifications = __v); } } }), compat::read(__this, |__t| __t.sack_notifications), Some(component::translatable("skyblocker.config.uiAndVisuals.itemPickup.sackNotifications.@Tooltip")));
        let __this_w2 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        collector.yes_no_button(&component::translatable("skyblocker.config.uiAndVisuals.itemPickup.splitSack"), Arc::new(move |b| { let Some(__this_2) = __this_w2.get().upgrade() else { return Default::default(); }; { { let __v = b; compat::write((&__this_2), |__t| __t.split_notifications = __v); } } }), compat::read(__this, |__t| __t.split_notifications), Some(component::translatable("skyblocker.config.uiAndVisuals.itemPickup.splitSack.@Tooltip")));
        let __this_w3 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        collector.yes_no_button(&component::translatable("skyblocker.config.uiAndVisuals.itemPickup.showItemName"), Arc::new(move |b| { let Some(__this_3) = __this_w3.get().upgrade() else { return Default::default(); }; { { let __v = b; compat::write((&__this_3), |__t| __t.show_item_name = __v); } } }), compat::read(__this, |__t| __t.show_item_name), Some(component::translatable("skyblocker.config.uiAndVisuals.itemPickup.showItemName.@Tooltip")));
        // TODO(fabric2seer): an assignment used as a value
        //   java: collector.slider(Component.translatable("skyblocker.config.uiAndVisuals.itemPickup.lifeTime"), d -> lifetime = (float) d, lifetime, 0.3, 10).tooltip(Component.translatable("skyblocker.config.uiAndVisuals.itemPickup.lifeTime.@Tooltip"));
    }

    /// ItemPickupWidget.getOptionWidgets as the dispatch of HudWidget.getOptionWidgets
    pub fn get_option_widgets_dyn(__this: &compat::Shared<dyn HudWidget>, __p0: &mut OptionWidgetCollector) {
        Self::get_option_widgets_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public void load(JsonValueInput input)
    pub fn load_impl(__this: &compat::Shared<Self>, input: &JsonValueInput) {
        <dyn ElementBasedWidget>::load_body(&compat::view(__this, |a| a as compat::Shared<dyn ElementBasedWidget>), input);
        { let __v = input.read_boolean_or("sack_notifications", false); compat::write(__this, |__t| __t.sack_notifications = __v); }
        { let __v = input.read_boolean_or("split_sack", false); compat::write(__this, |__t| __t.split_notifications = __v); }
        { let __v = input.read_boolean_or("show_item_name", true); compat::write(__this, |__t| __t.show_item_name = __v); }
        { let __v = input.read_float_or("lifetime", ((3) as f32) as f32); compat::write(__this, |__t| __t.lifetime = __v); }
    }

    /// ItemPickupWidget.load as the dispatch of HudWidget.load
    pub fn load_dyn(__this: &compat::Shared<dyn HudWidget>, __p0: &JsonValueInput) {
        Self::load_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public void save(JsonValueOutput output)
    pub fn save_impl(__this: &compat::Shared<Self>, output: &mut JsonValueOutput) {
        <dyn ElementBasedWidget>::save_body(&compat::view(__this, |a| a as compat::Shared<dyn ElementBasedWidget>), output);
        output.write_bool("sack_notifications", compat::read(__this, |__t| __t.sack_notifications));
        output.write_bool("split_sack", compat::read(__this, |__t| __t.split_notifications));
        output.write_bool("show_item_name", compat::read(__this, |__t| __t.show_item_name));
        output.write_number("lifetime", &mcrust::java::lang::number::Number::Float(compat::read(__this, |__t| __t.lifetime)));
    }

    /// ItemPickupWidget.save as the dispatch of HudWidget.save
    pub fn save_dyn(__this: &compat::Shared<dyn HudWidget>, __p0: &mut JsonValueOutput) {
        Self::save_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public boolean shouldRender()
    pub fn should_render_impl(__this: &compat::Shared<Self>) -> bool {
        return (!(__this.lock().borrow().added_count.is_empty()) || !(__this.lock().borrow().removed_count.is_empty()) || !(__this.lock().borrow().added_sack_count.is_empty()) || !(__this.lock().borrow().removed_sack_count.is_empty()));
    }

    /// ItemPickupWidget.shouldRender as the dispatch of HudWidget.shouldRender
    pub fn should_render_dyn(__this: &compat::Shared<dyn HudWidget>) -> bool {
        Self::should_render_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}

/// java: private record ChangeData(ItemStack item, int amount, long lastChange)
#[derive(Clone, Debug, PartialEq)]
pub struct ChangeData {
    /// java: ItemStack item
    pub item: mcrust::net::minecraft::world::item::item_stack::ItemStack,
    /// java: int amount
    pub amount: i32,
    /// java: long lastChange
    pub last_change: i64,
}

impl ChangeData {
    /// java: the canonical record constructor
    pub fn new(mut item: mcrust::net::minecraft::world::item::item_stack::ItemStack, mut amount: i32, mut last_change: i64) -> Self {
        Self { item, amount, last_change }
    }

    /// java: the record accessor `item()`
    pub fn item(&self) -> mcrust::net::minecraft::world::item::item_stack::ItemStack {
        self.item.clone()
    }

    /// java: the record accessor `amount()`
    pub fn amount(&self) -> i32 {
        self.amount
    }

    /// java: the record accessor `lastChange()`
    pub fn last_change(&self) -> i64 {
        self.last_change
    }

}

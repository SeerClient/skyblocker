//! Ported from de.hysky.skyblocker.skyblock.itemlist.recipebook.UpcomingEventsTab by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::events::event_notifications;
use crate::de::hysky::skyblocker::skyblock::events::event_notifications::EventNotifications;
use crate::de::hysky::skyblocker::skyblock::events::event_notifications::JerryPerks;
use crate::de::hysky::skyblocker::skyblock::events::event_notifications::SkyblockEvent;
use crate::de::hysky::skyblocker::skyblock::itemlist::recipebook::filter_option::FilterOption;
use crate::de::hysky::skyblocker::skyblock::itemlist::recipebook::recipe_area_display::RecipeAreaDisplay;
use crate::de::hysky::skyblocker::skyblock::itemlist::recipebook::recipe_tab::RecipeTab;
use crate::de::hysky::skyblocker::skyblock::tabhud::util::ico;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::jacobs_contest_widget;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use crate::de::hysky::skyblocker::utils::formatters;
use crate::de::hysky::skyblocker::utils::render::gui_helper::GuiHelper;
use crate::de::hysky::skyblocker::utils::scheduler::message_scheduler;
use crate::de::hysky::skyblocker::utils::scheduler::message_scheduler::MessageScheduler;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::font::Font;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::screens::inventory::tooltip::client_tooltip_component;
use mcrust::net::minecraft::client::gui::screens::inventory::tooltip::client_tooltip_component::ClientTooltipComponent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::world::item::items;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final ItemStack CLOCK = new ItemStack(Items.CLOCK);
pub static CLOCK: compat::MainThread<mcrust::net::minecraft::world::item::item_stack::ItemStack> = compat::MainThread::new(|| mcrust::net::minecraft::world::item::item_stack::ItemStack::of_one(items::items().get("clock")));
/// java: private static final int HEIGHT = 20;
pub const EVENT_RENDERER_HEIGHT: i32 = 20;

/// java: public class UpcomingEventsTab implements RecipeTab
#[derive(Clone)]
pub struct UpcomingEventsTab {
    /// java: private final List<EventRenderer> events = new ArrayList<>();
    pub events: Vec<EventRenderer>,
    /// java: private @Nullable EventRenderer hovered = null;
    pub hovered: Option<EventRenderer>,
}

impl UpcomingEventsTab {
    /// java: protected UpcomingEventsTab()
    pub fn new() -> compat::Shared<Self> {
        // TODO(fabric2seer): List.peekFirst(..)
        //   java: List<EventRenderer> renderers = EventNotifications.getEvents().entrySet().stream()
        //   java: 				.sorted(Comparator.comparingLong(a -> a.getValue().isEmpty() ? Long.MAX_VALUE : a.getValue().peekFirst().start()))
        //   java: 				.map(stringLinkedListEntry -> new EventRenderer(stringLinkedListEntry.getKey(), stringLinkedListEntry.getValue()))
        //   java: 				.toList();
        let mut renderers: Vec<EventRenderer> = todo!("fabric2seer: the initialiser of `renderers` (TODO above)");
        let __this = compat::shared(Self {
            events: Vec::new(),
            hovered: None,
        });
        let __h0 = renderers.iter().cloned();
        __this.lock().borrow_mut().events.extend(__h0);
        __this
    }

}

impl RecipeTab for UpcomingEventsTab {
    /// java: @Override public ItemStack icon() (dispatch: the body for this class)
    fn icon(&self) -> fn(&compat::Shared<dyn RecipeTab>) -> mcrust::net::minecraft::world::item::item_stack::ItemStack {
        UpcomingEventsTab::icon_dyn
    }

}

impl RecipeAreaDisplay for UpcomingEventsTab {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void initialize(Minecraft client, int parentLeft, int parentTop) (dispatch: the body for this class)
    fn initialize(&self) -> fn(&compat::Shared<dyn RecipeAreaDisplay>, &mut mcrust::net::minecraft::client::minecraft::Minecraft, i32, i32) {
        UpcomingEventsTab::initialize_dyn
    }

    /// java: @Override public void extractRenderState(GuiGraphicsExtractor graphics, int x, int y, int mouseX, int mouseY, float delta) (dispatch: the body for this class)
    fn extract_render_state(&self) -> fn(&compat::Shared<dyn RecipeAreaDisplay>, &GuiGraphicsExtractor, i32, i32, i32, i32, f32) {
        UpcomingEventsTab::extract_render_state_dyn
    }

    /// java: @Override public void extractTooltip(GuiGraphicsExtractor graphics, int x, int y) (dispatch: the body for this class)
    fn extract_tooltip(&self) -> fn(&compat::Shared<dyn RecipeAreaDisplay>, &mut GuiGraphicsExtractor, i32, i32) {
        UpcomingEventsTab::extract_tooltip_dyn
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled) (dispatch: the body for this class)
    fn mouse_clicked(&self) -> fn(&compat::Shared<dyn RecipeAreaDisplay>, &MouseButtonEvent, bool) -> bool {
        UpcomingEventsTab::mouse_clicked_dyn
    }

    /// java: @Override public void updateSearchResults(String query, FilterOption filterOption, boolean refresh) (dispatch: the body for this class)
    fn update_search_results_3(&self) -> fn(&compat::Shared<dyn RecipeAreaDisplay>, String, FilterOption, bool) {
        UpcomingEventsTab::update_search_results_3_dyn
    }

}

impl UpcomingEventsTab {
    /// java: @Override public ItemStack icon()
    pub fn icon_impl(__this: &compat::Shared<Self>) -> mcrust::net::minecraft::world::item::item_stack::ItemStack {
        return CLOCK.get_mut().clone();
    }

    /// UpcomingEventsTab.icon as the dispatch of RecipeTab.icon
    pub fn icon_dyn(__this: &compat::Shared<dyn RecipeTab>) -> mcrust::net::minecraft::world::item::item_stack::ItemStack {
        Self::icon_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public void initialize(Minecraft client, int parentLeft, int parentTop)
    pub fn initialize_impl(__this: &compat::Shared<Self>, client: &mut mcrust::net::minecraft::client::minecraft::Minecraft, mut parent_left: i32, mut parent_top: i32) {
    }

    /// UpcomingEventsTab.initialize as the dispatch of RecipeAreaDisplay.initialize
    pub fn initialize_dyn(__this: &compat::Shared<dyn RecipeAreaDisplay>, __p0: &mut mcrust::net::minecraft::client::minecraft::Minecraft, __p1: i32, __p2: i32) {
        Self::initialize_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2)
    }

    /// java: @Override public void extractRenderState(GuiGraphicsExtractor graphics, int x, int y, int mouseX, int mouseY, float delta)
    pub fn extract_render_state_impl(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, mut x: i32, mut y: i32, mut mouse_x: i32, mut mouse_y: i32, mut delta: f32) {
        x = x.wrapping_add(9);
        y = y.wrapping_add(9);
        graphics.enable_scissor(x, y, x.wrapping_add(131), y.wrapping_add(150));
        graphics.fake_item(CLOCK.get_mut(), x, y.wrapping_add(4));
        gui_graphics_extractor::text_string(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), Some("Upcoming Events"), x.wrapping_add(17), y.wrapping_add(7), mcrust::net::minecraft::util::common_colors::WHITE);
        let mut events_y: i32 = (y.wrapping_add(7)).wrapping_add(24);
        { let __v = None; compat::write(__this, |__t| __t.hovered = __v); }
        for event_renderer in compat::read(__this, |__t| __t.events.clone()) {
            event_renderer.extract_render_state(graphics, x.wrapping_add(1), events_y, mouse_x, mouse_y);
            if GuiHelper::point_is_in_area((mouse_x) as f64, (mouse_y) as f64, (x) as f64, (y) as f64, ((x.wrapping_add(131)) as f64) as f64, ((y.wrapping_add(150)) as f64) as f64) && EventRenderer::is_mouse_over(mouse_x, mouse_y, x.wrapping_add(1), events_y) {
                { let __v = Some(event_renderer.clone()); compat::write(__this, |__t| __t.hovered = __v); }
            }
            events_y = events_y.wrapping_add(EVENT_RENDERER_HEIGHT);
        }
        graphics.disable_scissor();
    }

    /// UpcomingEventsTab.extractRenderState as the dispatch of RecipeAreaDisplay.extractRenderState
    pub fn extract_render_state_dyn(__this: &compat::Shared<dyn RecipeAreaDisplay>, __p0: &GuiGraphicsExtractor, __p1: i32, __p2: i32, __p3: i32, __p4: i32, __p5: f32) {
        Self::extract_render_state_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2, __p3, __p4, __p5)
    }

    /// java: @Override public void extractTooltip(GuiGraphicsExtractor graphics, int x, int y)
    pub fn extract_tooltip_impl(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, mut x: i32, mut y: i32) {
        if compat::read(__this, |__t| __t.hovered.clone()).is_some() {
            // TODO(fabric2seer): `tooltip` takes 7 argument(s) in the port (`graphics: &mut crate::net::minecraft::client::gui::screens::inventory::tooltip::client_tooltip_component::TooltipGraphics<'_>, font: &dyn crate::net::minecraft::client::gui::screens::inventory::tooltip::client_tooltip_component::TooltipFont ..`), Java passes 6
            // rust: gui_graphics_extractor::tooltip(&mut compat::font(), &__this.lock().borrow().hovered.as_ref().unwrap().get_tooltip(), &x, y, mcrust::net::minecraft::client::gui::screens::inventory::tooltip::default_tooltip_positioner::DefaultTooltipPositioner::INSTANCE, None /* java: null */);
        }
    }

    /// UpcomingEventsTab.extractTooltip as the dispatch of RecipeAreaDisplay.extractTooltip
    pub fn extract_tooltip_dyn(__this: &compat::Shared<dyn RecipeAreaDisplay>, __p0: &mut GuiGraphicsExtractor, __p1: i32, __p2: i32) {
        Self::extract_tooltip_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2)
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    pub fn mouse_clicked_impl(__this: &compat::Shared<Self>, click: &MouseButtonEvent, mut doubled: bool) -> bool {
        if compat::read(__this, |__t| __t.hovered.clone()).is_some() && __this.lock().borrow().hovered.as_ref().unwrap().get_warp_command().is_some() {
            MessageScheduler::send_message_after_cooldown(message_scheduler::INSTANCE.get_mut(), __this.lock().borrow().hovered.as_ref().unwrap().get_warp_command().as_deref().unwrap(), true);
            return true;
        }
        return false;
    }

    /// UpcomingEventsTab.mouseClicked as the dispatch of RecipeAreaDisplay.mouseClicked
    pub fn mouse_clicked_dyn(__this: &compat::Shared<dyn RecipeAreaDisplay>, __p0: &MouseButtonEvent, __p1: bool) -> bool {
        Self::mouse_clicked_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1)
    }

    /// java: @Override public void updateSearchResults(String query, FilterOption filterOption, boolean refresh)
    pub fn update_search_results_3_impl(__this: &compat::Shared<Self>, mut query: String, mut filter_option: FilterOption, mut refresh: bool) {
    }

    /// UpcomingEventsTab.updateSearchResults as the dispatch of RecipeAreaDisplay.updateSearchResults
    pub fn update_search_results_3_dyn(__this: &compat::Shared<dyn RecipeAreaDisplay>, __p0: String, __p1: FilterOption, __p2: bool) {
        Self::update_search_results_3_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2)
    }

}

/// java: private record EventRenderer(String eventName, LinkedList<EventNotifications.SkyblockEvent> events)
#[derive(Clone, Debug, PartialEq)]
pub struct EventRenderer {
    /// java: String eventName
    pub event_name: String,
    /// java: LinkedList<EventNotifications.SkyblockEvent> events
    pub events: Vec<SkyblockEvent>,
}

impl EventRenderer {
    /// java: the canonical record constructor
    pub fn new(mut event_name: String, mut events: Vec<SkyblockEvent>) -> Self {
        Self { event_name, events }
    }

    /// java: the record accessor `eventName()`
    pub fn event_name(&self) -> String {
        self.event_name.clone()
    }

    /// java: the record accessor `events()`
    pub fn events(&self) -> Vec<SkyblockEvent> {
        self.events.clone()
    }

    /// java: private static boolean isMouseOver(int mouseX, int mouseY, int x, int y)
    pub fn is_mouse_over(mut mouse_x: i32, mut mouse_y: i32, mut x: i32, mut y: i32) -> bool {
        return GuiHelper::point_is_in_area((mouse_x) as f64, (mouse_y) as f64, (x) as f64, (y) as f64, ((x.wrapping_add(131)) as f64) as f64, ((y.wrapping_add(EVENT_RENDERER_HEIGHT)) as f64) as f64);
    }

    /// java: private void extractRenderState(GuiGraphicsExtractor graphics, int x, int y, int mouseX, int mouseY)
    pub fn extract_render_state(&self, graphics: &GuiGraphicsExtractor, mut x: i32, mut y: i32, mut mouse_x: i32, mut mouse_y: i32) {
        let mut time: i64 = (compat::millis()).wrapping_div(1000 as i64);
        let mut text_renderer: Font = compat::font();
        gui_graphics_extractor::draw_formatted_char_sequence(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::literal(&self.event_name).with_style(&mcrust::net::minecraft::network::chat::style::Style::EMPTY.with_underlined(Some(EventRenderer::is_mouse_over(mouse_x, mouse_y, x, y))))), x, y, mcrust::net::minecraft::util::common_colors::WHITE);
        // TODO(fabric2seer): List.peekFirst(..)
        //   java: if (events.isEmpty()) {
        //   java: 				graphics.text(textRenderer, Component.literal(" ").append(Component.translatable("skyblocker.events.tab.noMore")), x, y + textRenderer.lineHeight, CommonColors.GRAY, false);
        //   java: 			} else if (events.peekFirst().start() > time) {
        //   java: 				Component formatted = Component.literal(" ").append(Component.translatable("skyblocker.events.tab.startsIn", SkyblockTime.formatTime((int) (events.peekFirst().start() - time)))).withStyle(ChatFormatting.YELLOW);
        //   java:
        //   java: 				graphics.text(textRenderer, formatted, x, y + textRenderer.lineHeight, CommonColors.WHITE);
        //   java: 			} else {
        //   java: 				Component formatted = Component.literal(" ").append(Component.translatable("skyblocker.events.tab.endsIn", SkyblockTime.formatTime((int) (events.peekFirst().start() + events.peekFirst().duration() - time)))).withStyle(ChatFormatting.GREEN);
        //   java:
        //   java: 				graphics.text(textRenderer, formatted, x, y + textRenderer.lineHeight, CommonColors.WHITE);
        //   java: 			}
    }

    /// java: private List<ClientTooltipComponent> getTooltip()
    pub fn get_tooltip(&self) -> Vec<Box<dyn ClientTooltipComponent>> {
        let mut components: Vec<Box<dyn ClientTooltipComponent>> = Vec::new();
        // TODO(fabric2seer): List.peekFirst(..)
        //   java: EventNotifications.SkyblockEvent event = events.peekFirst();
        let mut event: SkyblockEvent = todo!("fabric2seer: the initialiser of `event` (TODO above)");
        if false /* java: event == null: never null here */ {
            return components;
        }
        if self.event_name == event_notifications::JACOBS && event.extras().left_value().is_some() {
            components.push(JacobsTooltip::new(event.extras().left_value().unwrap()));
        } else if self.event_name == event_notifications::MAYOR_JERRY && event.extras().right_value().is_some() {
            let mut jerry_info: JerryPerks = event.extras().right_value().unwrap();
            let mut mayor_text: MutableComponent = (component::translatable_args("skyblocker.events.tab.currentJerryMayor", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of(jerry_info.mayor_name())])).with_style_format(ChatFormatting::DarkGreen);
            // TODO(fabric2seer): the port's `create_text` ports this Java method with other parameters ( for Java's char_sequence)
            // TODO(fabric2seer): `create_text` takes 0 argument(s) in the port (``), Java passes 1
            // rust: components.push(client_tooltip_component::create_text(mayor_text.get_visual_order_text()));
            for perk in jerry_info.perks() {
                let mut perk_text: MutableComponent = (component::literal("- ")).with_style_format(ChatFormatting::Gray).append(component::literal(&perk).with_style_format(ChatFormatting::Green));
                // TODO(fabric2seer): the port's `create_text` ports this Java method with other parameters ( for Java's char_sequence)
                // TODO(fabric2seer): `create_text` takes 0 argument(s) in the port (``), Java passes 1
                // rust: components.push(client_tooltip_component::create_text(perk_text.get_visual_order_text()));
            }
            return components;
        }
        if !(event.warp_command().is_empty()) {
            // TODO(fabric2seer): the port's `create_text` ports this Java method with other parameters ( for Java's char_sequence)
            // TODO(fabric2seer): `create_text` takes 0 argument(s) in the port (``), Java passes 1
            // rust: components.push(client_tooltip_component::create_text(component::translatable("skyblocker.events.tab.clickToWarp").with_style_format(ChatFormatting::Italic).get_visual_order_text()));
        }
        // TODO(fabric2seer): the port's `create_text` ports this Java method with other parameters ( for Java's char_sequence)
        // TODO(fabric2seer): `create_text` takes 0 argument(s) in the port (``), Java passes 1
        // rust: components.push(client_tooltip_component::create_text((component::literal(&formatters::DATE_FORMATTER.get_mut().format(mcrust::java::time::instant::Instant::of_epoch_second(event.start()))).with_style_formats(&[ChatFormatting::Italic, ChatFormatting::DarkGray])).get_visual_order_text()));
        return components;
    }

    /// java: private @Nullable String getWarpCommand()
    pub fn get_warp_command(&self) -> Option<String> {
        // TODO(fabric2seer): List.peek(..)
        //   java: return !events.isEmpty() ? events.peek().warpCommand() : null;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}

/// java: private record JacobsTooltip(List<String> crops) implements ClientTooltipComponent
#[derive(Clone, Debug, PartialEq)]
pub struct JacobsTooltip {
    /// java: List<String> crops
    pub crops: Vec<String>,
}

impl JacobsTooltip {
    /// java: the canonical record constructor
    pub fn new(mut crops: Vec<String>) -> Self {
        Self { crops }
    }

    /// java: the record accessor `crops()`
    pub fn crops(&self) -> Vec<String> {
        self.crops.clone()
    }

    /// java: @Override public int getHeight(Font textRenderer)
    pub fn get_height(&self, text_renderer: &Font) -> i32 {
        return 20;
    }

    /// java: @Override public int getWidth(Font textRenderer)
    pub fn get_width(&self, text_renderer: &Font) -> i32 {
        return (16i32.wrapping_mul(3)).wrapping_add(4);
    }

    /// java: @Override public void extractImage(Font textRenderer, int x, int y, int width, int height, GuiGraphicsExtractor graphics)
    pub fn extract_image(&self, text_renderer: &Font, mut x: i32, mut y: i32, mut width: i32, mut height: i32, graphics: &GuiGraphicsExtractor) {
        for i in 0..(self.crops.len() as i32) {
            let mut crop: String = self.crops[(i) as usize].clone();
            graphics.fake_item(&FlexibleItemStack::get_stack_or_throw(&jacobs_contest_widget::FARM_DATA.get_mut().get(&crop).cloned().unwrap_or_else(|| ico::BARRIER.get_mut().clone())), x.wrapping_add(18i32.wrapping_mul(i)), y.wrapping_add(2));
        }
    }

}

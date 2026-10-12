//! Ported from de.hysky.skyblocker.skyblock.chat.ChatRuleConfigScreen by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::chat::chat_rule::AnnouncementMessage;
use crate::de::hysky::skyblocker::skyblock::chat::chat_rule::ChatRule;
use crate::de::hysky::skyblocker::skyblock::chat::chat_rule::ToastMessage;
use crate::de::hysky::skyblocker::skyblock::chat::chat_rule_location_config_screen::ChatRuleLocationConfigScreen;
use crate::de::hysky::skyblocker::skyblock::chat::chat_rules_handler;
use crate::de::hysky::skyblocker::skyblock::tabhud::util::ico;
use crate::de::hysky::skyblocker::utils::datafixer::item_stack_componentization_fixer::ItemStackComponentizationFixer;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use crate::de::hysky::skyblocker::utils::formatters;
use crate::de::hysky::skyblocker::utils::render::gui::ranged_slider_widget::RangedSliderWidget;
use crate::de::hysky::skyblocker::utils::render::gui::toggleable_layout_widget::ToggleableLayoutWidget;
use mcrust::java::net::uri::Uri;
use mcrust::mcrust::modding::screens::ModScreen;
use mcrust::mcrust::modding::screens::ScreenRenderContext;
use mcrust::mcrust::modding::widgets::ModWidget;
use mcrust::mcrust::modding::widgets::WidgetRenderContext;
use mcrust::net::minecraft::chat_formatting;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::active_text_collector::ClickableStyleFinder;
use mcrust::net::minecraft::client::gui::components::abstract_container_widget::AbstractContainerWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::PressLambda;
use mcrust::net::minecraft::client::gui::components::abstract_widget::WidgetKind;
use mcrust::net::minecraft::client::gui::components::button;
use mcrust::net::minecraft::client::gui::components::checkbox::Checkbox;
use mcrust::net::minecraft::client::gui::components::cycle_button::CycleButton;
use mcrust::net::minecraft::client::gui::components::edit_box::EditBox;
use mcrust::net::minecraft::client::gui::components::edit_box::Responder;
use mcrust::net::minecraft::client::gui::components::edit_box::ResponderLambda;
use mcrust::net::minecraft::client::gui::components::edit_box::TextFormatter;
use mcrust::net::minecraft::client::gui::components::events::gui_event_listener::GuiEventListener;
use mcrust::net::minecraft::client::gui::components::multi_line_text_widget::MultiLineTextWidget;
use mcrust::net::minecraft::client::gui::components::tooltip::Tooltip;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::layouts::frame_layout::FrameLayout;
use mcrust::net::minecraft::client::gui::layouts::grid_layout::GridLayout;
use mcrust::net::minecraft::client::gui::layouts::grid_layout::RowHelper;
use mcrust::net::minecraft::client::gui::layouts::header_and_footer_layout::HeaderAndFooterLayout;
use mcrust::net::minecraft::client::gui::layouts::layout_element::LayoutElement;
use mcrust::net::minecraft::client::gui::layouts::layout_settings::LayoutSettings;
use mcrust::net::minecraft::client::gui::layouts::linear_layout::LinearLayout;
use mcrust::net::minecraft::client::gui::layouts::spacer_element::SpacerElement;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::gui::text_alignment::TextAlignment;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::client::input::mouse_button_info::MouseButtonInfo;
use mcrust::net::minecraft::network::chat::click_event::ClickEvent;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::chat::style::Style;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::sounds::sound_event::SoundEvent;
use mcrust::net::minecraft::util::util;
use crate::de::hysky::skyblocker::utils::data::json_data::JsonData as _;
use mcrust::net::minecraft::client::gui::components::renderable::Renderable as _;
use mcrust::net::minecraft::core::registry::Registry as _;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final int COLUMN_WIDTH = 105;
pub const COLUMN_WIDTH: i32 = 105;
/// java: private static final int GRID_SPACING = 2;
pub const GRID_SPACING: i32 = 2;
/// java: protected static final Identifier SEARCH_ICON_TEXTURE = Identifier.withDefaultNamespace("icon/search");
pub static SEARCH_ICON_TEXTURE: compat::MainThread<Identifier> = compat::MainThread::new(|| Identifier::with_default_namespace("icon/search"));
/// java: private static final FlexibleItemStack INVALID_ITEM = Ico.BARRIER;
pub static INVALID_ITEM: compat::MainThread<compat::Shared<FlexibleItemStack>> = compat::MainThread::new(|| ico::BARRIER.get_mut().clone());
/// java: private static final Component REGEX_LINK = Component.translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.regexLink").withStyle( style -> style.withUnderlined(true).withClickEvent(new Cli
pub static REGEX_LINK: compat::MainThread<MutableComponent> = compat::MainThread::new(|| component::translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.regexLink").with_style_fn(|style| style.with_underlined(Some(true)).with_click_event(Some(ClickEvent::new_open_url(Uri::create("https://regex101.com/").unwrap())))));
/// java: private static final int SIDE_PADDING = 10;
pub const CONTENT_CONTAINER_SIDE_PADDING: i32 = 10;

/// java: public class ChatRuleConfigScreen extends Screen
pub struct ChatRuleConfigScreen {
    /// java: extends net.minecraft.client.gui.screens.Screen
    pub base: mcrust::net::minecraft::client::gui::screens::screen::Screen,
    /// java: private final Map<@Nullable SoundEvent, Component> soundNames = Util.make(new Object2ObjectOpenHashMap<>(), map -> { map.put(SoundEvents.NOTE_BLOCK_PLING.value(
    pub sound_names: std::collections::HashMap<Option<SoundEvent>, MutableComponent>,
    /// java: private final int chatRuleIndex;
    pub chat_rule_index: i32,
    /// java: private final ChatRule chatRule;
    pub chat_rule: compat::Shared<ChatRule>,
    /// java: private ChatRule.@Nullable ToastMessage previousToastMessage = null;
    pub previous_toast_message: Option<compat::Shared<ToastMessage>>,
    /// java: private ChatRule.@Nullable AnnouncementMessage previousAnnouncementMessage = null;
    pub previous_announcement_message: Option<compat::Shared<AnnouncementMessage>>,
    /// java: private final Screen parent;
    pub parent: mcrust::net::minecraft::client::gui::screens::screen::Screen,
    /// java: private final HeaderAndFooterLayout layout = new HeaderAndFooterLayout(this);
    pub layout: Option<HeaderAndFooterLayout>,
    /// java: private final GridLayout content = new GridLayout().columnSpacing(GRID_SPACING);
    pub content: GridLayout,
    /// java: @SuppressWarnings("rawtypes") private CycleButton soundButton;
    pub sound_button: Option<CycleButton>,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
    /// the screen's widgets as this object last saw them (see `compat::WidgetCache`)
    pub __widgets: compat::WidgetCache,
}

impl std::ops::Deref for ChatRuleConfigScreen {
    type Target = mcrust::net::minecraft::client::gui::screens::screen::Screen;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for ChatRuleConfigScreen {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl ChatRuleConfigScreen {
    /// java: public ChatRuleConfigScreen(Screen parent, int chatRuleIndex)
    pub fn new(mut parent: mcrust::net::minecraft::client::gui::screens::screen::Screen, mut chat_rule_index: i32) -> compat::Shared<Self> {
        let __init_chat_rule_index = chat_rule_index;
        // TODO(fabric2seer): `.get(..)` on a T
        //   java: this.chatRule = ChatRulesHandler.CHAT_RULE_LIST.getData().get(chatRuleIndex);
        // TODO(fabric2seer): the mod keeps the game's `Screen` (Java holds a reference to the live object); the port's `Screen` is not a value to copy and the port lends the mod no handle to it
        //   java: this.parent = parent;
        let __this = compat::shared(Self {
            base: mcrust::net::minecraft::client::gui::screens::screen::Screen::new(component::translatable("skyblocker.config.chat.chatRules.screen.ruleScreen").get_string()),
            self_ref: std::sync::Weak::new(),
            __widgets: Default::default(),
            sound_names: util::make_with(std::collections::HashMap::new(), |map| {
            // TODO(fabric2seer): `.put(..)` on a T
            //   java: map.put(SoundEvents.NOTE_BLOCK_PLING.value(), Component.translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.sounds.pling").withStyle(ChatFormatting.YELLOW));
            // TODO(fabric2seer): `.put(..)` on a T
            //   java: map.put(SoundEvents.AMBIENT_CAVE.value(), Component.translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.sounds.cave").withStyle(ChatFormatting.YELLOW));
            // TODO(fabric2seer): `.put(..)` on a T
            //   java: map.put(SoundEvents.ZOMBIE_AMBIENT, Component.translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.sounds.zombie").withStyle(ChatFormatting.YELLOW));
            // TODO(fabric2seer): `.put(..)` on a T
            //   java: map.put(SoundEvents.PLAYER_ATTACK_CRIT, Component.translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.sounds.crit").withStyle(ChatFormatting.YELLOW));
            // TODO(fabric2seer): `.put(..)` on a T
            //   java: map.put(SoundEvents.ARROW_HIT_PLAYER, Component.translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.sounds.arrowHit").withStyle(ChatFormatting.YELLOW));
            // TODO(fabric2seer): `.put(..)` on a T
            //   java: map.put(SoundEvents.AMETHYST_BLOCK_HIT, Component.translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.sounds.amethyst").withStyle(ChatFormatting.YELLOW));
            // TODO(fabric2seer): `.put(..)` on a T
            //   java: map.put(SoundEvents.ANVIL_LAND, Component.translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.sounds.anvil").withStyle(ChatFormatting.YELLOW));
            // TODO(fabric2seer): `.put(..)` on a T
            //   java: map.put(null, Component.translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.sounds.none").withStyle(ChatFormatting.RED));
            // TODO(fabric2seer): `.defaultReturnValue(..)` on a T
            //   java: map.defaultReturnValue(Component.translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.sounds.custom").withStyle(ChatFormatting.YELLOW));
        }),
            chat_rule_index: __init_chat_rule_index,
            chat_rule: todo!("fabric2seer: the field value (TODO above)"),
            previous_toast_message: None,
            previous_announcement_message: None,
            parent: todo!("fabric2seer: the field value (TODO above)"),
            layout: None,
            content: GridLayout::new_default().column_spacing(GRID_SPACING),
            sound_button: None,
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        { let __v = Some(HeaderAndFooterLayout::new(0, 0)); compat::write(&__this, |__t| __t.layout = __v); }
        __this
    }

    /// java: private static String getItemString(ItemStack stack)
    pub fn get_item_string(stack: &mut mcrust::net::minecraft::world::item::item_stack::ItemStack) -> String {
        return format!("{}{}", compat::opt_str(&mcrust::net::minecraft::core::registries::built_in_registries::item().get_key(&stack.get_item())), ItemStackComponentizationFixer::components_as_string(stack));
    }

    /// java: private static int getWidth(float occupiedColumns)
    pub fn get_width(mut occupied_columns: f32) -> i32 {
        return (((((COLUMN_WIDTH as f32) * occupied_columns) + ((GRID_SPACING as f32) * (occupied_columns - (1 as f32))))) as i32);
    }

    /// java: @Override protected void init()
    pub fn init(__this: &compat::Shared<Self>) {
        compat::minecraft();
        let __r0 = compat::read(__this, |__t| __t.title.clone());
        { let (__q0,) = ({ let __w = compat::string_widget(0, 0, -1, 9, __r0); let (__ww, __wh) = (__w.get_width(), __w.get_height()); LayoutElement::widget(compat::write(__this, |__t| __t.add_widget(__w)), __ww, __wh) },); __this.lock().borrow_mut().layout.as_mut().unwrap().add_to_header(__q0) };
        let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        { let (__q0,) = ({ let __w = (button::builder(mcrust::net::minecraft::network::chat::common_components::gui_done().get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { let Some(__this_1) = __this_w1.get().upgrade() else { return Default::default(); }; compat::lend_screen(&__this_1, __screen, || { let mut __pidx = compat::WidgetRef::Screen(__index); { ChatRuleConfigScreen::on_close((&__this_1)); } }) })))).build(); let (__ww, __wh) = (__w.get_width(), __w.get_height()); LayoutElement::widget(compat::write(__this, |__t| __t.add_widget(__w)), __ww, __wh) },); __this.lock().borrow_mut().layout.as_mut().unwrap().add_to_footer(__q0) };
        { let (__q0,) = (ContentContainer::new(),); __this.lock().borrow_mut().layout.as_mut().unwrap().add_to_contents(__q0) };
        __this.lock().borrow_mut().content.default_cell_setting().align_vertically_middle().align_horizontally_center().padding_top(GRID_SPACING);
        let mut aligned_left: LayoutSettings = __this.lock().borrow().content.new_cell_settings().align_horizontally_left();
        let mut content_adder: RowHelper = __this.lock().borrow().content.create_row_helper(3);
        // TODO(fabric2seer): the port's `add_child_width` ports this Java method with other parameters (grid, child, column_width for Java's widget, column_width)
        // TODO(fabric2seer): `add_child_width` takes 3 argument(s) in the port (`grid: &mut GridLayout, child: LayoutElement, column_width: i32`), Java passes 2
        // rust: content_adder.add_child_width(&mut compat::string_widget(0, 0, -1, 9, component::translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.name").with_style_formats(&[ChatFormatting::Bold, ChatFormatting::Underline])), 3).unwrap();
        // TODO(fabric2seer): the port's `new` ports this Java method with other parameters (splitter, x, y, width, height for Java's width, height, narration)
        // TODO(fabric2seer): `new` takes 5 argument(s) in the port (`splitter: &StringSplitter<W>, x: i32, y: i32, width: i32, height: i32`), Java passes 4
        // TODO(fabric2seer): the port's `add_child_width` ports this Java method with other parameters (grid, child, column_width for Java's widget, column_width)
        // TODO(fabric2seer): `add_child_width` takes 3 argument(s) in the port (`grid: &mut GridLayout, child: LayoutElement, column_width: i32`), Java passes 2
        // rust: let mut name_widget: EditBox = content_adder.add_child_width(&mut EditBox::new(&compat::font(), ChatRuleConfigScreen::get_width(((3) as f32) as f32), 20, component::empty()), 3).unwrap();
        let mut name_widget: EditBox = todo!("fabric2seer: the initialiser of `nameWidget` (TODO above)");
        // TODO(fabric2seer): `set_value` takes 2 argument(s) in the port (`splitter: &StringSplitter<W>, value: &str`), Java passes 1
        // rust: name_widget.set_value(&ChatRule::get_name(&compat::read(__this, |__t| __t.chat_rule.clone())));
        let __this_2 = compat::RenderThread::new(__this.clone());
        name_widget.set_responder(Responder::Lambda(ResponderLambda::new(move |__a0| { ChatRule::set_name(&compat::read(__this_2.get(), |__t| __t.chat_rule.clone()), __a0.to_string()); })));
        // TODO(fabric2seer): `AbstractWidget::set_tooltip` ports `setTooltip`, but the port's `EditBox` holds no `AbstractWidget` (no Deref, no `base` field) to call it on
        // rust: name_widget.set_tooltip(Tooltip::create((component::translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.name.@Tooltip")).get_string()));
        // TODO(fabric2seer): the port's `add_child_width` ports this Java method with other parameters (grid, child, column_width for Java's widget, column_width)
        // TODO(fabric2seer): `add_child_width` takes 3 argument(s) in the port (`grid: &mut GridLayout, child: LayoutElement, column_width: i32`), Java passes 2
        // rust: content_adder.add_child_width(&mut compat::string_widget(0, 0, -1, 9, component::translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.filter").with_style_formats(&[ChatFormatting::Bold, ChatFormatting::Underline])), 3).unwrap();
        // TODO(fabric2seer): the port's `new` ports this Java method with other parameters (splitter, x, y, width, height for Java's width, height, narration)
        // TODO(fabric2seer): `new` takes 5 argument(s) in the port (`splitter: &StringSplitter<W>, x: i32, y: i32, width: i32, height: i32`), Java passes 4
        // TODO(fabric2seer): the port's `add_child_width` ports this Java method with other parameters (grid, child, column_width for Java's widget, column_width)
        // TODO(fabric2seer): `add_child_width` takes 3 argument(s) in the port (`grid: &mut GridLayout, child: LayoutElement, column_width: i32`), Java passes 2
        // rust: let mut filter_input: EditBox = content_adder.add_child_width(&mut EditBox::new(&compat::font(), ChatRuleConfigScreen::get_width(((3) as f32) as f32), 20, component::empty()), 3).unwrap();
        let mut filter_input: EditBox = todo!("fabric2seer: the initialiser of `filterInput` (TODO above)");
        filter_input.set_max_length(1024);
        filter_input.add_formatter(ChatRuleConfigScreen::create_render_text_provider_2(__this, Arc::new(move || filter_input.get_value()), true));
        // TODO(fabric2seer): `set_value` takes 2 argument(s) in the port (`splitter: &StringSplitter<W>, value: &str`), Java passes 1
        // rust: filter_input.set_value(&ChatRule::get_filter(&compat::read(__this, |__t| __t.chat_rule.clone())));
        let __this_3 = compat::RenderThread::new(__this.clone());
        filter_input.set_responder(Responder::Lambda(ResponderLambda::new(move |__a0| { ChatRule::set_filter(&compat::read(__this_3.get(), |__t| __t.chat_rule.clone()), __a0.to_string()); })));
        filter_input.set_hint_component(component::translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.filter").with_style_formats(&[ChatFormatting::Gray, ChatFormatting::Italic]));
        // TODO(fabric2seer): `AbstractWidget::set_tooltip` ports `setTooltip`, but the port's `EditBox` holds no `AbstractWidget` (no Deref, no `base` field) to call it on
        // rust: filter_input.set_tooltip(Tooltip::create((component::translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.filter.@Tooltip")).get_string()));
        // TODO(fabric2seer): the port's `add_child_width` ports this Java method with other parameters (grid, child, column_width for Java's widget, column_width)
        // TODO(fabric2seer): `add_child_width` takes 3 argument(s) in the port (`grid: &mut GridLayout, child: LayoutElement, column_width: i32`), Java passes 2
        // rust: let mut filters_row1: LinearLayout = content_adder.add_child_width(&mut LinearLayout::horizontal().spacing(GRID_SPACING), 3).unwrap();
        let mut filters_row1: LinearLayout = todo!("fabric2seer: the initialiser of `filtersRow1` (TODO above)");
        let __this_w4 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        filters_row1.add_child(LayoutElement::Frame(ChatRuleConfigScreen::build_checkbox(__this, &component::translatable_args("skyblocker.config.chat.chatRules.screen.ruleScreen.regex", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(REGEX_LINK.get_mut().clone())]), &component::translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.regex.@Tooltip"), ChatRuleConfigScreen::get_width(1.5f32), &TextAlignment::Center, Arc::new(move |__a0| { let Some(__this_4) = __this_w4.get().upgrade() else { return Default::default(); }; { ChatRule::set_regex(&compat::read((&__this_4), |__t| __t.chat_rule.clone()), __a0); } }), ChatRule::get_regex(&compat::read(__this, |__t| __t.chat_rule.clone())))));
        let __this_5 = compat::RenderThread::new(__this.clone());
        filters_row1.add_child({ let __w = (button::builder(component::translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.locations").get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { compat::lend_screen(__this_5.get(), __screen, || { let mut __pidx = compat::WidgetRef::Screen(__index); { mcrust::mcrust::modding::game::set_screen_keeping_parent(Box::new(compat::ScreenAdapter::new(ChatRuleLocationConfigScreen::new(None, compat::read(__this_5.get(), |__t| __t.chat_rule.clone()))))); } }) })))).tooltip_text(Some((Tooltip::create((component::translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.locations.@Tooltip")).get_string())).message.clone())).width(ChatRuleConfigScreen::get_width(1.5f32)).build(); let (__ww, __wh) = (__w.get_width(), __w.get_height()); LayoutElement::widget(compat::write(__this, |__t| __t.add_widget(__w)), __ww, __wh) });
        // TODO(fabric2seer): the port's `add_child_width` ports this Java method with other parameters (grid, child, column_width for Java's widget, column_width)
        // TODO(fabric2seer): `add_child_width` takes 3 argument(s) in the port (`grid: &mut GridLayout, child: LayoutElement, column_width: i32`), Java passes 2
        // rust: let mut filters_row2: LinearLayout = content_adder.add_child_width(&mut LinearLayout::horizontal().spacing(GRID_SPACING), 3).unwrap();
        let mut filters_row2: LinearLayout = todo!("fabric2seer: the initialiser of `filtersRow2` (TODO above)");
        let __this_w6 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        filters_row2.add_child(LayoutElement::Frame(ChatRuleConfigScreen::build_checkbox(__this, &component::translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.includeFormatting"), &component::translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.includeFormatting.@Tooltip"), ChatRuleConfigScreen::get_width(1.25f32), &TextAlignment::Left, Arc::new(move |__a0| { let Some(__this_6) = __this_w6.get().upgrade() else { return Default::default(); }; { ChatRule::set_include_formatting(&compat::read((&__this_6), |__t| __t.chat_rule.clone()), __a0); } }), ChatRule::get_include_formatting(&compat::read(__this, |__t| __t.chat_rule.clone())))));
        let __this_w7 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        filters_row2.add_child(LayoutElement::Frame(ChatRuleConfigScreen::build_checkbox(__this, &component::translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.partialMatch"), &component::translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.partialMatch.@Tooltip"), ChatRuleConfigScreen::get_width(0.75f32), &TextAlignment::Center, Arc::new(move |__a0| { let Some(__this_7) = __this_w7.get().upgrade() else { return Default::default(); }; { ChatRule::set_partial_match(&compat::read((&__this_7), |__t| __t.chat_rule.clone()), __a0); } }), ChatRule::get_partial_match(&compat::read(__this, |__t| __t.chat_rule.clone())))));
        let __this_w8 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        filters_row2.add_child(LayoutElement::Frame(ChatRuleConfigScreen::build_checkbox(__this, &component::translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.ignoreCase"), &component::translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.ignoreCase.@Tooltip"), ChatRuleConfigScreen::get_width(1.0f32), &TextAlignment::Right, Arc::new(move |__a0| { let Some(__this_8) = __this_w8.get().upgrade() else { return Default::default(); }; { ChatRule::set_ignore_case(&compat::read((&__this_8), |__t| __t.chat_rule.clone()), __a0); } }), ChatRule::get_ignore_case(&compat::read(__this, |__t| __t.chat_rule.clone())))));
        // TODO(fabric2seer): the port's `add_child_spanning` ports this Java method with other parameters (grid, child, column_width, settings for Java's widget, column_width, layout_settings)
        // TODO(fabric2seer): `add_child_spanning` takes 4 argument(s) in the port (`grid: &mut GridLayout, child: LayoutElement, column_width: i32, settings: LayoutSettings`), Java passes 3
        // rust: content_adder.add_child_spanning(&mut compat::string_widget(0, 0, -1, 9, component::translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.outputs").with_style_formats(&[ChatFormatting::Bold, ChatFormatting::Underline])), 3, { let (__q0,) = (4i32.wrapping_add(GRID_SPACING),); __this.lock().borrow().content.new_cell_settings().padding_top(__q0) }).unwrap();
        // TODO(fabric2seer): `.setCentered(..)` on a value of unknown type
        //   java: contentAdder.addChild(new MultiLineTextWidget(Component.translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.formatting"), font).setMaxWidth(getWidth(2)), 3).setCentered(true);
        // TODO(fabric2seer): the port's `add_child_width` ports this Java method with other parameters (grid, child, column_width for Java's widget, column_width)
        // TODO(fabric2seer): `add_child_width` takes 3 argument(s) in the port (`grid: &mut GridLayout, child: LayoutElement, column_width: i32`), Java passes 2
        // rust: let mut buttons: LinearLayout = content_adder.add_child_width(&mut LinearLayout::horizontal().spacing(GRID_SPACING), 3).unwrap();
        let mut buttons: LinearLayout = todo!("fabric2seer: the initialiser of `buttons` (TODO above)");
        let __this_w9 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        buttons.add_child(LayoutElement::Frame(ChatRuleConfigScreen::build_checkbox(__this, &component::translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.hideMessage"), &component::translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.hideMessage.@Tooltip"), ChatRuleConfigScreen::get_width(1.5f32), &TextAlignment::Left, Arc::new(move |value| { let Some(__this_9) = __this_w9.get().upgrade() else { return Default::default(); }; {
            ChatRule::set_hide_message(&compat::read((&__this_9), |__t| __t.chat_rule.clone()), value);
            ChatRuleConfigScreen::recreate_layout((&__this_9));
        } }), ChatRule::get_hide_message(&compat::read(__this, |__t| __t.chat_rule.clone())))));
        let mut available_values: Vec<Option<SoundEvent>> = __this.lock().borrow().sound_names.keys().cloned().map(|__a0| __a0.clone()).collect::<Vec<_>>().into_iter().collect::<Vec<_>>();
        let mut displayed_values: Vec<Option<SoundEvent>>;
        if __this.lock().borrow().sound_names.contains_key(ChatRule::get_custom_sound(&compat::read(__this, |__t| __t.chat_rule.clone())).as_ref().unwrap()) {
            displayed_values = available_values.clone();
        } else {
            displayed_values = available_values.iter().cloned().collect::<Vec<_>>();
            displayed_values.push(ChatRule::get_custom_sound(&compat::read(__this, |__t| __t.chat_rule.clone())));
        }
        // TODO(fabric2seer): `.orElse(..)` on a T
        //   java: soundButton = CycleButton.builder(opt -> soundNames.get(opt.orElse(null)), Optional.ofNullable(chatRule.getCustomSound()))
        //   java: 				.withValues(() -> true, displayedValues, availableValues)
        //   java: 				.create(0, 0, getWidth(1.3f), 20, Component.translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.sounds"), (_, value) -> {
        //   java: 					chatRule.setCustomSound(value.orElse(null));
        //   java: 					value.ifPresent(soundEvent -> minecraft.getSoundManager().play(SimpleSoundInstance.forUI(soundEvent, 1.0f)));
        //   java: 				});
        // TODO(fabric2seer): `AbstractWidget::set_tooltip` ports `setTooltip`, but the port's `CycleButton` holds no `AbstractWidget` (no Deref, no `base` field) to call it on
        // rust: { let (__q0,) = (Tooltip::create((component::translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.sounds.@Tooltip")).get_string()),); __this.lock().borrow_mut().sound_button.as_mut().unwrap().set_tooltip(__q0) };
        buttons.add_child(compat::read(__this, |__t| __t.sound_button.clone()).unwrap());
        buttons.add_child_adjusted({ let __w = compat::WidgetAdapter::widget(SoundSearchMenu::new()); let (__ww, __wh) = (__w.get_width(), __w.get_height()); LayoutElement::widget(compat::write(__this, |__t| __t.add_widget(__w)), __ww, __wh) }, &mut |__a0| { __a0.align_horizontally_right(); });
        // TODO(fabric2seer): the port's `new` ports this Java method with other parameters (splitter, x, y, width, height for Java's width, height, narration)
        // TODO(fabric2seer): `new` takes 5 argument(s) in the port (`splitter: &StringSplitter<W>, x: i32, y: i32, width: i32, height: i32`), Java passes 4
        // rust: let mut chat_message_input: EditBox = EditBox::new(&compat::font(), ChatRuleConfigScreen::get_width(((2) as f32) as f32), 20, component::empty());
        let mut chat_message_input: EditBox = todo!("fabric2seer: the initialiser of `chatMessageInput` (TODO above)");
        chat_message_input.set_max_length(1024);
        // TODO(fabric2seer): the port's `new` ports this Java method with other parameters (x, y, message for Java's message)
        // TODO(fabric2seer): `new` takes 3 argument(s) in the port (`x: i32, y: i32, message: FormattedText`), Java passes 2
        // TODO(fabric2seer): the port's `add_child_with` ports this Java method with other parameters (grid, child, settings for Java's widget, layout_settings)
        // TODO(fabric2seer): `add_child_with` takes 3 argument(s) in the port (`grid: &mut GridLayout, child: LayoutElement, settings: LayoutSettings`), Java passes 2
        // rust: let __this_10 = compat::RenderThread::new(__this.clone());
        // rust: content_adder.add_child_with(&mut ToggleableLayoutWidget::new(((MultiLineTextWidget::new(component::translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.chatMessage"), compat::font())).set_max_width(ChatRuleConfigScreen::get_width(((1) as f32) as f32))).set_centered(false), Arc::new(move || !(ChatRule::get_hide_message(&compat::read(__this_10.get(), |__t| __t.chat_rule.clone()))))), aligned_left.clone()).unwrap();
        chat_message_input.add_formatter(ChatRuleConfigScreen::create_render_text_provider(__this, Arc::new(move || chat_message_input.get_value())));
        // TODO(fabric2seer): `set_value` takes 2 argument(s) in the port (`splitter: &StringSplitter<W>, value: &str`), Java passes 1
        // rust: chat_message_input.set_value(&(if ChatRule::get_chat_message(&compat::read(__this, |__t| __t.chat_rule.clone())).is_some() { ChatRule::get_chat_message(&compat::read(__this, |__t| __t.chat_rule.clone())) } else { Some("".to_string()) }).unwrap());
        let __this_11 = compat::RenderThread::new(__this.clone());
        chat_message_input.set_responder(Responder::Lambda(ResponderLambda::new(move |__a0| { ChatRule::set_chat_message(&compat::read(__this_11.get(), |__t| __t.chat_rule.clone()), Some(__a0.to_string())); })));
        chat_message_input.set_hint_component((component::translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.chatMessage.@Placeholder")).with_style_formats(&[ChatFormatting::Gray, ChatFormatting::Italic]));
        // TODO(fabric2seer): the port's `add_child_width` ports this Java method with other parameters (grid, child, column_width for Java's widget, column_width)
        // TODO(fabric2seer): `add_child_width` takes 3 argument(s) in the port (`grid: &mut GridLayout, child: LayoutElement, column_width: i32`), Java passes 2
        // rust: let __this_12 = compat::RenderThread::new(__this.clone());
        // rust: content_adder.add_child_width(&mut ToggleableLayoutWidget::new(chat_message_input.clone(), Arc::new(move || !(ChatRule::get_hide_message(&compat::read(__this_12.get(), |__t| __t.chat_rule.clone()))))), 2).unwrap();
        // TODO(fabric2seer): `.setMaxWidth(..)` on a value of unknown type
        //   java: contentAdder.addChild(new MultiLineTextWidget(Component.translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.actionBar"), font), alignedLeft).setMaxWidth(getWidth(1)).setCentered(false);
        // TODO(fabric2seer): the port's `new` ports this Java method with other parameters (splitter, x, y, width, height for Java's width, height, narration)
        // TODO(fabric2seer): `new` takes 5 argument(s) in the port (`splitter: &StringSplitter<W>, x: i32, y: i32, width: i32, height: i32`), Java passes 4
        // rust: let mut action_bar_message_input: EditBox = EditBox::new(&compat::font(), ChatRuleConfigScreen::get_width(((2) as f32) as f32), 20, component::empty());
        let mut action_bar_message_input: EditBox = todo!("fabric2seer: the initialiser of `actionBarMessageInput` (TODO above)");
        action_bar_message_input.set_max_length(1024);
        action_bar_message_input.add_formatter(ChatRuleConfigScreen::create_render_text_provider(__this, Arc::new(move || action_bar_message_input.get_value())));
        // TODO(fabric2seer): `set_value` takes 2 argument(s) in the port (`splitter: &StringSplitter<W>, value: &str`), Java passes 1
        // rust: action_bar_message_input.set_value(&(if ChatRule::get_action_bar_message(&compat::read(__this, |__t| __t.chat_rule.clone())).is_some() { ChatRule::get_action_bar_message(&compat::read(__this, |__t| __t.chat_rule.clone())) } else { Some("".to_string()) }).unwrap());
        let __this_13 = compat::RenderThread::new(__this.clone());
        action_bar_message_input.set_responder(Responder::Lambda(ResponderLambda::new(move |__a0| { ChatRule::set_action_bar_message(&compat::read(__this_13.get(), |__t| __t.chat_rule.clone()), Some(__a0.to_string())); })));
        action_bar_message_input.set_hint_component((component::translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.outputField.@Placeholder")).with_style_formats(&[ChatFormatting::Gray, ChatFormatting::Italic]));
        // TODO(fabric2seer): the port's `add_child_width` ports this Java method with other parameters (grid, child, column_width for Java's widget, column_width)
        // TODO(fabric2seer): `add_child_width` takes 3 argument(s) in the port (`grid: &mut GridLayout, child: LayoutElement, column_width: i32`), Java passes 2
        // rust: content_adder.add_child_width(&mut action_bar_message_input, 2).unwrap();
        // TODO(fabric2seer): `.setMaxWidth(..)` on a value of unknown type
        //   java: contentAdder.addChild(new MultiLineTextWidget(Component.translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.announcement"), font), alignedLeft).setMaxWidth(getWidth(1)).setCentered(false);
        // TODO(fabric2seer): the port's `new` ports this Java method with other parameters (splitter, x, y, width, height for Java's width, height, narration)
        // TODO(fabric2seer): `new` takes 5 argument(s) in the port (`splitter: &StringSplitter<W>, x: i32, y: i32, width: i32, height: i32`), Java passes 4
        // rust: let mut announcement_message_input: EditBox = EditBox::new(&compat::font(), ChatRuleConfigScreen::get_width(((2) as f32) as f32), 20, component::empty());
        let mut announcement_message_input: EditBox = todo!("fabric2seer: the initialiser of `announcementMessageInput` (TODO above)");
        announcement_message_input.set_max_length(1024);
        announcement_message_input.add_formatter(ChatRuleConfigScreen::create_render_text_provider(__this, Arc::new(move || announcement_message_input.get_value())));
        // TODO(fabric2seer): `set_value` takes 2 argument(s) in the port (`splitter: &StringSplitter<W>, value: &str`), Java passes 1
        // rust: announcement_message_input.set_value(&(if ChatRule::get_announcement_message(&compat::read(__this, |__t| __t.chat_rule.clone())).is_some() { compat::read(&ChatRule::get_announcement_message(&compat::read(__this, |__t| __t.chat_rule.clone())).unwrap(), |__t| __t.message.clone()) } else { "".to_string() }));
        let __this_14 = compat::RenderThread::new(__this.clone());
        announcement_message_input.set_responder(Responder::Lambda(ResponderLambda::new(move |s| {
            if s.is_empty() {
                { let __v = ChatRule::get_announcement_message(&compat::read(__this_14.get(), |__t| __t.chat_rule.clone())); compat::write(__this_14.get(), |__t| __t.previous_announcement_message = __v); }
                ChatRule::set_announcement_message(&compat::read(__this_14.get(), |__t| __t.chat_rule.clone()), None);
            } else {
                if ChatRule::get_announcement_message(&compat::read(__this_14.get(), |__t| __t.chat_rule.clone())).is_none() {
                    ChatRule::set_announcement_message(&compat::read(__this_14.get(), |__t| __t.chat_rule.clone()), (if compat::read(__this_14.get(), |__t| __t.previous_announcement_message.clone()).is_some() { compat::read(__this_14.get(), |__t| __t.previous_announcement_message.clone()) } else { Some(AnnouncementMessage::new_0()) }));
                }
                { let __v = s.to_string(); compat::write(&ChatRule::get_announcement_message(&compat::read(__this_14.get(), |__t| __t.chat_rule.clone())).unwrap(), |__t| __t.message = __v); }
            }
            ChatRuleConfigScreen::recreate_layout(__this_14.get());
        })));
        // TODO(fabric2seer): `AbstractWidget::set_tooltip` ports `setTooltip`, but the port's `EditBox` holds no `AbstractWidget` (no Deref, no `base` field) to call it on
        // rust: announcement_message_input.set_tooltip(Tooltip::create((component::translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.announcement.@Tooltip")).get_string()));
        announcement_message_input.set_hint_component((component::translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.outputField.@Placeholder")).with_style_formats(&[ChatFormatting::Gray, ChatFormatting::Italic]));
        // TODO(fabric2seer): the port's `add_child_width` ports this Java method with other parameters (grid, child, column_width for Java's widget, column_width)
        // TODO(fabric2seer): `add_child_width` takes 3 argument(s) in the port (`grid: &mut GridLayout, child: LayoutElement, column_width: i32`), Java passes 2
        // rust: content_adder.add_child_width(&mut announcement_message_input, 2).unwrap();
        // TODO(fabric2seer): the port's `add_child_with` ports this Java method with other parameters (grid, child, settings for Java's widget, layout_settings)
        // TODO(fabric2seer): `add_child_with` takes 3 argument(s) in the port (`grid: &mut GridLayout, child: LayoutElement, settings: LayoutSettings`), Java passes 2
        // rust: content_adder.add_child_with(&mut SpacerElement::height(0), __this.lock().borrow().content.new_cell_settings().padding_top(0)).unwrap();
        let __this_15 = compat::RenderThread::new(__this.clone());
        let mut announcement_duration_slider: RangedSliderWidget = ((((((RangedSliderWidget::builder().min_max(((1) as f64) as f64, ((10) as f64) as f64)).step(0.1)).width(ChatRuleConfigScreen::get_width(((2) as f32) as f32))).default_value((if ChatRule::get_announcement_message(&compat::read(__this, |__t| __t.chat_rule.clone())).is_some() { ((compat::read(&ChatRule::get_announcement_message(&compat::read(__this, |__t| __t.chat_rule.clone())).unwrap(), |__t| __t.display_duration) as f64) / 1000.0) } else { 5000.0 }))).option_formatter(&component::translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.announcement.duration"), &Arc::new(move |d| component::literal(&format!("{}s", formatters::FLOAT_NUMBERS.get_mut().format(d.clone())))))).callback(Arc::new(move |d| {
            if ChatRule::get_announcement_message(&compat::read(__this_15.get(), |__t| __t.chat_rule.clone())).is_none() {
                return;
            }
            { let __v = ((d.wrapping_mul(1000)) as i64); compat::write(&ChatRule::get_announcement_message(&compat::read(__this_15.get(), |__t| __t.chat_rule.clone())).unwrap(), |__t| __t.display_duration = __v); }
        }))).build();
        // TODO(fabric2seer): `.addChild(..)` on a T
        //   java: contentAdder.addChild(new ToggleableLayoutWidget(
        //   java: 				Util.make(new FrameLayout(), w -> w.addChild(announcementDurationSlider, p -> p.paddingTop(GRID_SPACING))),
        //   java: 				() -> chatRule.getAnnouncementMessage() != null
        //   java: 		), 2, content.newCellSettings().paddingTop(0));
        // TODO(fabric2seer): `.setMaxWidth(..)` on a value of unknown type
        //   java: contentAdder.addChild(new MultiLineTextWidget(Component.translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.toast"), font), alignedLeft).setMaxWidth(getWidth(1)).setCentered(false);
        // TODO(fabric2seer): the port's `new` ports this Java method with other parameters (splitter, x, y, width, height for Java's width, height, narration)
        // TODO(fabric2seer): `new` takes 5 argument(s) in the port (`splitter: &StringSplitter<W>, x: i32, y: i32, width: i32, height: i32`), Java passes 4
        // rust: let mut toast_message_input: EditBox = EditBox::new(&compat::font(), ChatRuleConfigScreen::get_width(((2) as f32) as f32), 20, component::empty());
        let mut toast_message_input: EditBox = todo!("fabric2seer: the initialiser of `toastMessageInput` (TODO above)");
        toast_message_input.set_max_length(1024);
        toast_message_input.add_formatter(ChatRuleConfigScreen::create_render_text_provider(__this, Arc::new(move || toast_message_input.get_value())));
        // TODO(fabric2seer): `set_value` takes 2 argument(s) in the port (`splitter: &StringSplitter<W>, value: &str`), Java passes 1
        // rust: toast_message_input.set_value(&(if ChatRule::get_toast_message(&compat::read(__this, |__t| __t.chat_rule.clone())).is_some() { compat::read(&ChatRule::get_toast_message(&compat::read(__this, |__t| __t.chat_rule.clone())).unwrap(), |__t| __t.message.clone()) } else { "".to_string() }));
        let __this_16 = compat::RenderThread::new(__this.clone());
        toast_message_input.set_responder(Responder::Lambda(ResponderLambda::new(move |s| {
            if s.is_empty() {
                { let __v = ChatRule::get_toast_message(&compat::read(__this_16.get(), |__t| __t.chat_rule.clone())); compat::write(__this_16.get(), |__t| __t.previous_toast_message = __v); }
                ChatRule::set_toast_message(&compat::read(__this_16.get(), |__t| __t.chat_rule.clone()), None);
            } else {
                if ChatRule::get_toast_message(&compat::read(__this_16.get(), |__t| __t.chat_rule.clone())).is_none() {
                    ChatRule::set_toast_message(&compat::read(__this_16.get(), |__t| __t.chat_rule.clone()), (if compat::read(__this_16.get(), |__t| __t.previous_toast_message.clone()).is_some() { compat::read(__this_16.get(), |__t| __t.previous_toast_message.clone()) } else { Some(ToastMessage::new_0()) }));
                }
                { let __v = s.to_string(); compat::write(&ChatRule::get_toast_message(&compat::read(__this_16.get(), |__t| __t.chat_rule.clone())).unwrap(), |__t| __t.message = __v); }
            }
            ChatRuleConfigScreen::recreate_layout(__this_16.get());
        })));
        toast_message_input.set_hint_component((component::translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.outputField.@Placeholder")).with_style_formats(&[ChatFormatting::Gray, ChatFormatting::Italic]));
        // TODO(fabric2seer): the port's `add_child_width` ports this Java method with other parameters (grid, child, column_width for Java's widget, column_width)
        // TODO(fabric2seer): `add_child_width` takes 3 argument(s) in the port (`grid: &mut GridLayout, child: LayoutElement, column_width: i32`), Java passes 2
        // rust: content_adder.add_child_width(&mut toast_message_input, 2).unwrap();
        let __this_17 = compat::RenderThread::new(__this.clone());
        let mut toast_options_predicate: Arc<dyn Fn() -> bool> = Arc::new(move || ChatRule::get_toast_message(&compat::read(__this_17.get(), |__t| __t.chat_rule.clone())).is_some());
        let mut text_and_icon: FrameLayout = FrameLayout::of(ChatRuleConfigScreen::get_width(((1) as f32) as f32), 0);
        // TODO(fabric2seer): the port's `add_child` ports this Java method with other parameters (grid, child for Java's widget)
        // TODO(fabric2seer): `add_child` takes 2 argument(s) in the port (`grid: &mut GridLayout, child: LayoutElement`), Java passes 1
        // rust: content_adder.add_child(&mut ToggleableLayoutWidget::new(LayoutElement::Frame(text_and_icon.clone()), toast_options_predicate.clone())).unwrap();
        // TODO(fabric2seer): the port's `new` ports this Java method with other parameters (splitter, x, y, width, height for Java's width, height, narration)
        // TODO(fabric2seer): `new` takes 5 argument(s) in the port (`splitter: &StringSplitter<W>, x: i32, y: i32, width: i32, height: i32`), Java passes 4
        // rust: let mut item_input: EditBox = EditBox::new(&compat::font(), ChatRuleConfigScreen::get_width(((1) as f32) as f32), 20, component::empty());
        let mut item_input: EditBox = todo!("fabric2seer: the initialiser of `itemInput` (TODO above)");
        item_input.set_max_length(300);
        let mut preview: compat::Shared<ToastIconPreview> = text_and_icon.add_child_adjusted({ let __w = compat::WidgetAdapter::widget(ToastIconPreview::new(item_input.clone())); let (__ww, __wh) = (__w.get_width(), __w.get_height()); LayoutElement::widget(compat::write(__this, |__t| __t.add_widget(__w)), __ww, __wh) }, &mut |__a0| { __a0.align_horizontally_right(); });
        // TODO(fabric2seer): `.setMaxWidth(..)` on a value of unknown type
        //   java: textAndIcon.addChild(new MultiLineTextWidget(Component.translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.toast.icon"), font), LayoutSettings::alignHorizontallyLeft).setMaxWidth(getWidth(1) - preview.getWidth()).setCentered(false);
        // TODO(fabric2seer): the port's `add_child` ports this Java method with other parameters (grid, child for Java's widget)
        // TODO(fabric2seer): `add_child` takes 2 argument(s) in the port (`grid: &mut GridLayout, child: LayoutElement`), Java passes 1
        // rust: content_adder.add_child(&mut ToggleableLayoutWidget::new(item_input.clone(), toast_options_predicate.clone())).unwrap();
        let __this_18 = compat::RenderThread::new(__this.clone());
        item_input.set_responder(Responder::Lambda(ResponderLambda::new(move |item_data| {
            let mut parsed_stack: Option<mcrust::net::minecraft::world::item::item_stack::ItemStack> = Some(ItemStackComponentizationFixer::from_item_string(item_data, 1));
            if parsed_stack.as_ref().unwrap().is_empty() {
                parsed_stack = FlexibleItemStack::get_stack(INVALID_ITEM.get_mut());
            }
            if let Some(parsed_stack) = &parsed_stack {
            } else {
                return;
            }
            let mut stack: compat::Shared<FlexibleItemStack> = FlexibleItemStack::new4(parsed_stack.clone().unwrap());
            { let __v = FlexibleItemStack::get_stack_or_empty(&stack); compat::write(&preview, |__t| __t.stack = __v); }
            if preview.lock().borrow().stack.is_empty() {
                return;
            }
            let Some(mut message) = ChatRule::get_toast_message(&compat::read(__this_18.get(), |__t| __t.chat_rule.clone())) else {
                return;
            };
            { let __v = Some(stack.clone()); compat::write(&message, |__t| __t.icon = __v); }
        })));
        // TODO(fabric2seer): `AbstractWidget::set_tooltip` ports `setTooltip`, but the port's `EditBox` holds no `AbstractWidget` (no Deref, no `base` field) to call it on
        // rust: item_input.set_tooltip(Tooltip::create((component::translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.toast.icon.@Tooltip")).get_string()));
        // TODO(fabric2seer): `set_value` takes 2 argument(s) in the port (`splitter: &StringSplitter<W>, value: &str`), Java passes 1
        // rust: item_input.set_value(&(if ChatRule::get_toast_message(&compat::read(__this, |__t| __t.chat_rule.clone())).is_some() { ChatRuleConfigScreen::get_item_string(&mut compat::read(&ChatRule::get_toast_message(&compat::read(__this, |__t| __t.chat_rule.clone())).unwrap(), |__t| __t.icon.clone()).and_then(|__a0| FlexibleItemStack::get_stack(&__a0)).unwrap_or(mcrust::net::minecraft::world::item::item_stack::empty())) } else { "minecraft:painting".to_string() }));
        if mcrust::mcrust::modding::game::level().is_none() {
            // TODO(fabric2seer): `set_editable` takes 2 argument(s) in the port (`is_editable: bool, is_focused: bool`), Java passes 1
            // rust: item_input.set_editable(false);
            { let __v = false; compat::write(&preview, |__t| __t.active = __v); }
            let mut tooltip: Tooltip = Tooltip::create(component::translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.toast.icon.unableToEdit").get_string());
            // TODO(fabric2seer): `AbstractWidget::set_tooltip` ports `setTooltip`, but the port's `EditBox` holds no `AbstractWidget` (no Deref, no `base` field) to call it on
            // rust: item_input.set_tooltip(tooltip.clone());
            let __wa1 = tooltip.clone();
            let __o19 = preview.clone();
            compat::write(&__o19, |__t| __t.set_tooltip(__wa1.clone()));
        }
        let __this_20 = compat::RenderThread::new(__this.clone());
        let mut slider_widget: RangedSliderWidget = ((((((RangedSliderWidget::builder().min_max(((1) as f64) as f64, ((10) as f64) as f64)).step(0.1)).width(ChatRuleConfigScreen::get_width(((1) as f32) as f32))).default_value((if ChatRule::get_toast_message(&compat::read(__this, |__t| __t.chat_rule.clone())).is_some() { ((compat::read(&ChatRule::get_toast_message(&compat::read(__this, |__t| __t.chat_rule.clone())).unwrap(), |__t| __t.display_duration) as f64) / 1000.0) } else { 5000.0 }))).option_formatter(&component::translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.toast.duration"), &Arc::new(move |d| component::literal(&format!("{}s", formatters::FLOAT_NUMBERS.get_mut().format(d.clone())))))).callback(Arc::new(move |d| {
            if ChatRule::get_toast_message(&compat::read(__this_20.get(), |__t| __t.chat_rule.clone())).is_none() {
                return;
            }
            { let __v = ((d.wrapping_mul(1000)) as i64); compat::write(&ChatRule::get_toast_message(&compat::read(__this_20.get(), |__t| __t.chat_rule.clone())).unwrap(), |__t| __t.display_duration = __v); }
        }))).build();
        // TODO(fabric2seer): the port's `add_child` ports this Java method with other parameters (grid, child for Java's widget)
        // TODO(fabric2seer): `add_child` takes 2 argument(s) in the port (`grid: &mut GridLayout, child: LayoutElement`), Java passes 1
        // rust: content_adder.add_child(&mut ToggleableLayoutWidget::new(slider_widget.clone(), toast_options_predicate.clone())).unwrap();
        slider_widget.set_tooltip(Tooltip::create((component::translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.toast.duration.@Tooltip")).get_string()));
        ChatRuleConfigScreen::recreate_layout(__this);
    }

    /// java: private FrameLayout buildCheckbox(Component text, Component tooltip, int width, TextAlignment align, Consumer<Boolean> setter, boolean selected)
    pub fn build_checkbox(__this: &compat::Shared<Self>, text: &MutableComponent, tooltip: &MutableComponent, mut width: i32, align: &TextAlignment, setter: Arc<dyn Fn(bool)>, mut selected: bool) -> FrameLayout {
        let mut frame: FrameLayout = FrameLayout::new().set_min_width(width);
        // TODO(fabric2seer): static `Checkbox.builder` has no Rust counterpart in the port
        // TODO(fabric2seer): `Builder.selected` has no Rust method in the port (looked for `selected` in Checkbox and its supertypes)
        // TODO(fabric2seer): `Builder.onValueChange` has no Rust method in the port (looked for `on_value_change` in Checkbox and its supertypes)
        // TODO(fabric2seer): `Builder.maxWidth` has no Rust method in the port (looked for `max_width` in Checkbox and its supertypes)
        // TODO(fabric2seer): `Builder.build` has no Rust method in the port (looked for `build` in Checkbox and its supertypes)
        // rust: let mut r#box: Checkbox = (Checkbox::builder(text.clone(), compat::font())).selected(selected).on_value_change(&Arc::new(move |_, value| (setter)(value.clone()))).max_width(width).build();
        let mut r#box: Checkbox = todo!("fabric2seer: the initialiser of `box` (TODO above)");
        match *align {
            TextAlignment::Left => {
                frame.default_child_layout_setting().align_horizontally_left();
            }
            TextAlignment::Center => {
                frame.default_child_layout_setting().align_horizontally_center();
            }
            TextAlignment::Right => {
                frame.default_child_layout_setting().align_horizontally_right();
            }
            _ => {}
        }
        // TODO(fabric2seer): `AbstractWidget::set_tooltip` ports `setTooltip`, but the port's `Checkbox` holds no `AbstractWidget` (no Deref, no `base` field) to call it on
        // rust: (r#box).set_tooltip(Tooltip::create(tooltip.get_string()));
        frame.add_child(r#box.clone());
        return frame;
    }

    /// java: private EditBox.TextFormatter createRenderTextProvider(Supplier<String> fullTextSupplier)
    pub fn create_render_text_provider(__this: &compat::Shared<Self>, full_text_supplier: Arc<dyn Fn() -> String>) -> TextFormatter {
        return ChatRuleConfigScreen::create_render_text_provider_2(__this, full_text_supplier.clone(), false);
    }

    /// java: private EditBox.TextFormatter createRenderTextProvider(Supplier<String> fullTextSupplier, boolean onlyIfFormatted)
    pub fn create_render_text_provider_2(__this: &compat::Shared<Self>, full_text_supplier: Arc<dyn Fn() -> String>, mut only_if_formatted: bool) -> TextFormatter {
        let __this_1 = compat::RenderThread::new(__this.clone());
        return Arc::new(move |s, start| {
            if only_if_formatted && !(ChatRule::get_include_formatting(&compat::read(__this_1.get(), |__t| __t.chat_rule.clone()))) || ChatRule::get_regex(&compat::read(__this_1.get(), |__t| __t.chat_rule.clone())) {
                return Default::default(); // java: return null
            }
            return Arc::new(move |visitor| {
                let mut full_text: String = (full_text_supplier)();
                let mut prefix: char = (if full_text.contains("§") { '§' } else { '&' });
                let mut style: Style = mcrust::net::minecraft::network::chat::style::Style::EMPTY;
                for i in 0..compat::str_len(&full_text) {
                    if compat::char_at(&full_text, i) == prefix {
                        if i.wrapping_add(1) < compat::str_len(&full_text) {
                            let mut formatting: ChatFormatting = chat_formatting::get_by_code(compat::char_at(&full_text, i.wrapping_add(1))).unwrap();
                            if true /* java: formatting != null: never null here */ {
                                style = (if formatting == ChatFormatting::Reset { mcrust::net::minecraft::network::chat::style::Style::EMPTY } else { style.apply_legacy_format(formatting) });
                            }
                        }
                    }
                    let mut code_point: i32 = (compat::char_at(&full_text, i) as i32);
                    // TODO(fabric2seer): the condition: `.length(..)` on a value of unknown type
                    //   java: i >= start && i < start + s.length()
                    if todo!("fabric2seer: the condition (TODO above)") {
                        // TODO(fabric2seer): `.accept(..)` on a value of unknown type
                        //   java: visitor.accept(i, style, codePoint);
                    }
                }
                return true;
            });
        });
    }

    /// java: private void recreateLayout()
    pub fn recreate_layout(__this: &compat::Shared<Self>) {
        __this.lock().borrow_mut().clear_widgets();
        ChatRuleConfigScreen::reposition_elements(__this);
        { let __vw = __this.lock().borrow().layout.as_ref().unwrap().visit_widgets(); for (__i, _, _) in __vw { compat::write(__this, |__t| __t.mark_renderable(__i)); } };
    }

    /// java: @Override protected void repositionElements()
    pub fn reposition_elements(__this: &compat::Shared<Self>) {
        let __r0 = compat::read(__this, |__t| (__t.width, __t.height));
        { let (__sw, __sh) = __r0; __this.lock().borrow_mut().layout.as_mut().unwrap().screen_width = __sw; __this.lock().borrow_mut().layout.as_mut().unwrap().screen_height = __sh; __this.lock().borrow_mut().layout.as_mut().unwrap().arrange_elements(); let __vw = __this.lock().borrow().layout.as_ref().unwrap().visit_widgets(); for (__i, __x, __y) in __vw { compat::write(__this, |__t| __t.widget_mut(__i).set_position(__x, __y)); } };
    }

    /// java: @Override public boolean mouseClicked(final MouseButtonEvent event, final boolean doubleClick)
    pub fn mouse_clicked(__this: &compat::Shared<Self>, event: &MouseButtonEvent, mut double_click: bool) -> bool {
        let mut child: Option<Box<dyn GuiEventListener>> = { let (__q0, __q1,) = (event.x(), event.y(),); __this.lock().borrow().get_child_at(__q0, __q1) }.unwrap();
        // TODO(fabric2seer): instanceof with a pattern binding outside an `if` condition
        //   java: while (child.isPresent() && child.get() instanceof ContainerEventHandler container) {
        //   java: 			child = container.getChildAt(event.x(), event.y());
        //   java: 		}
        if child.is_some() && let Some(check) = child.clone().unwrap().as_any().downcast_ref::<Checkbox>() {
            // TODO(fabric2seer): `new` takes 2 argument(s) in the port (`test_x: f64, test_y: f64`), Java passes 3
            // rust: let mut finder: ClickableStyleFinder = (ClickableStyleFinder::new(compat::font(), (((event.x()) as i32)) as f64, ((event.y()) as i32))).include_insertions(false);
            let mut finder: ClickableStyleFinder = todo!("fabric2seer: the initialiser of `finder` (TODO above)");
            // TODO(fabric2seer): `.visitLines(..)` on a MultiLineTextWidget
            //   java: ((CheckboxAccessor) check).getTextWidget().visitLines(finder);
            let mut style: Option<Style> = finder.result();
            if style.is_some() && style.as_ref().unwrap().get_click_event().is_some() {
                // TODO(fabric2seer): `Screen.defaultHandleClickEvent` has no Rust method in the port (looked for `default_handle_click_event` in Screen and its supertypes)
                // rust: let __h0 = style.as_ref().unwrap().get_click_event();
                // rust: let __h1 = compat::minecraft();
                // rust: let __h2 = __this;
                // rust: __this.lock().borrow().default_handle_click_event(__h0, __h1, __h2);
                return true;
            }
        }
        return false;
    }

    /// java: @Override public void onClose()
    pub fn on_close(__this: &compat::Shared<Self>) {
        ChatRuleConfigScreen::save(__this);
        mcrust::mcrust::modding::game::set_screen_to_parent();
    }

    /// java: private void save()
    pub fn save(__this: &compat::Shared<Self>) {
        // TODO(fabric2seer): `.set(..)` on a T
        //   java: ChatRulesHandler.CHAT_RULE_LIST.getData().set(chatRuleIndex, chatRule);
    }

}

/// ChatRuleConfigScreen's `Screen` half (`super`'s fields and widget tree), lent by the game for each call
impl compat::ScreenHalf for ChatRuleConfigScreen {
    fn screen_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::screens::screen::Screen {
        &mut self.base
    }
    fn screen_parts(&mut self) -> (&mut mcrust::net::minecraft::client::gui::screens::screen::Screen, Option<&mut compat::WidgetCache>) {
        let __s = self;
        (&mut __s.base, Some(&mut __s.__widgets))
    }
}

/// ChatRuleConfigScreen as the game's screen: `game::set_screen(Box::new(compat::ScreenAdapter::new(..)))`
impl ModScreen for compat::ScreenAdapter<ChatRuleConfigScreen> {
    fn get_title(&self) -> String {
        self.title()
    }

    /// java: @Override protected void init()
    fn init(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        self.lend(screen, |__h| ChatRuleConfigScreen::init(__h))
    }

    /// java: @Override protected void repositionElements()
    fn reposition_elements(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        self.lend(screen, |__h| ChatRuleConfigScreen::reposition_elements(__h))
    }

    /// java: Screen.tick (the base body)
    fn tick(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        self.lend(screen, |_| ())
    }

    /// java: Screen.keyPressed (the base body)
    fn key_pressed(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, event: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        self.lend(screen, |_| ());
        if event.is_escape() && self.should_close_on_esc() {
            self.on_close();
            return true;
        }
        false
    }

    /// java: @Override public boolean mouseClicked(final MouseButtonEvent event, final boolean doubleClick)
    fn mouse_clicked(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, x: f64, y: f64, button: i32) -> bool {
        self.lend(screen, |__h| ChatRuleConfigScreen::mouse_clicked(__h, &MouseButtonEvent::new(x, y, MouseButtonInfo::new(button, 0)), false))
    }

    /// java: @Override public void onClose()
    fn on_close(&mut self) {
        self.call(|__h| ChatRuleConfigScreen::on_close(__h))
    }

}

// NOTE(fabric2seer): init, mouseClicked, onClose, repositionElements override(s) of Screen - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them

/// java: private class ToastIconPreview extends AbstractWidget
#[derive(Clone)]
pub struct ToastIconPreview {
    /// java: extends net.minecraft.client.gui.components.AbstractWidget
    pub base: AbstractWidget,
    /// java: private ItemStack stack = ItemStack.EMPTY;
    pub stack: mcrust::net::minecraft::world::item::item_stack::ItemStack,
    /// java: private final EditBox input;
    pub input: EditBox,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl std::ops::Deref for ToastIconPreview {
    type Target = AbstractWidget;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for ToastIconPreview {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl ToastIconPreview {
    /// java: private ToastIconPreview(EditBox input)
    pub fn new(mut input: EditBox) -> compat::Shared<Self> {
        let __init_input = input.clone();
        let __this = compat::shared(Self {
            base: AbstractWidget::new(0, 0, 16, 16, (&component::empty().get_string()).to_string(), WidgetKind::PlainText, OnPress::Inert),
            self_ref: std::sync::Weak::new(),
            stack: mcrust::net::minecraft::world::item::item_stack::empty(),
            input: __init_input,
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        { let (__q0,) = (Tooltip::create((component::translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.toast.iconPreview.@Tooltip")).get_string()),); __this.lock().borrow_mut().set_tooltip(__q0) };
        __this
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_widget_render_state(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        graphics.fake_item(&compat::read(__this, |__t| __t.stack.clone()), __this.lock().borrow().get_x(), __this.lock().borrow().get_y());
    }

    /// java: @Override public void setX(int x)
    pub fn set_x(__this: &compat::Shared<Self>, mut x: i32) {
        __this.lock().borrow_mut().base.set_x(x);
    }

    /// java: @Override public void onClick(MouseButtonEvent click, boolean doubled)
    pub fn on_click(__this: &compat::Shared<Self>, click: &MouseButtonEvent, mut doubled: bool) {
        // TODO(fabric2seer): `AbstractWidget.onClick` has no Rust method in the port (looked for `on_click` in AbstractWidget and its supertypes)
        // rust: __this.lock().borrow_mut().base.on_click(click, doubled);
        // TODO(fabric2seer): unknown name `minecraft`
        //   java: Objects.requireNonNull(minecraft);
        // TODO(fabric2seer): unknown name `minecraft`
        //   java: minecraft.gui.setScreen(new ItemSelectionPopup(ChatRuleConfigScreen.this, item -> {
        //   java: 				if (item == null) return;
        //   java: 				input.setValue(getItemString(item));
        //   java: 			}));
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    pub fn update_widget_narration(__this: &compat::Shared<Self>, builder: &dyn NarrationElementOutput) {
    }

}

/// ToastIconPreview's widget half (`super`'s fields: x, y, size, message, ...), lent by the game for each call
impl compat::WidgetHalf for ToastIconPreview {
    fn widget_half(&mut self) -> &mut AbstractWidget {
        &mut self.base
    }
}

/// ToastIconPreview as the game's widget: `compat::WidgetAdapter::widget(handle)` where Java hands it on
impl ModWidget for compat::WidgetAdapter<ToastIconPreview> {
    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_widget_render_state(&mut self, widget: &AbstractWidget, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.lend_ref(widget, |__h| ToastIconPreview::extract_widget_render_state(__h, &context.graphics, mouse_x, mouse_y, a))
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    fn update_widget_narration(&mut self, widget: &AbstractWidget, output: &mut dyn NarrationElementOutput) {
        self.lend_ref(widget, |__h| ToastIconPreview::update_widget_narration(__h, output))
    }

    /// java: @Override public void onClick(MouseButtonEvent click, boolean doubled)
    fn on_click(&mut self, widget: &mut AbstractWidget, event: &MouseButtonEvent, double_click: bool) {
        self.lend(widget, |__h| ToastIconPreview::on_click(__h, event, double_click))
    }

}

// NOTE(fabric2seer): extractWidgetRenderState, onClick, setX, updateWidgetNarration override(s) of AbstractWidget - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them

/// java: private class SoundSearchMenu extends AbstractWidget
#[derive(Clone)]
pub struct SoundSearchMenu {
    /// java: extends net.minecraft.client.gui.components.AbstractWidget
    pub base: AbstractWidget,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl std::ops::Deref for SoundSearchMenu {
    type Target = AbstractWidget;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for SoundSearchMenu {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl SoundSearchMenu {
    /// java: private SoundSearchMenu()
    pub fn new() -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            base: AbstractWidget::new(0, 0, 16, 16, (&component::empty().get_string()).to_string(), WidgetKind::PlainText, OnPress::Inert),
            self_ref: std::sync::Weak::new(),
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        { let (__q0,) = (Tooltip::create((component::translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.sounds.search.@Tooltip")).get_string()),); __this.lock().borrow_mut().set_tooltip(__q0) };
        __this
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_widget_render_state(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        // TODO(fabric2seer): no overload of `getWidth` takes 0 argument(s)
        //   java: graphics.blitSprite(RenderPipelines.GUI_TEXTURED, SEARCH_ICON_TEXTURE, this.getX(), this.getY(), this.getWidth(), this.getHeight());
    }

    /// java: @Override public void setX(int x)
    pub fn set_x(__this: &compat::Shared<Self>, mut x: i32) {
        __this.lock().borrow_mut().base.set_x(x);
    }

    /// java: @Override public void onClick(MouseButtonEvent click, boolean doubled)
    pub fn on_click(__this: &compat::Shared<Self>, click: &MouseButtonEvent, mut doubled: bool) {
        // TODO(fabric2seer): `AbstractWidget.onClick` has no Rust method in the port (looked for `on_click` in AbstractWidget and its supertypes)
        // rust: __this.lock().borrow_mut().base.on_click(click, doubled);
        // TODO(fabric2seer): unknown name `minecraft`
        //   java: Objects.requireNonNull(minecraft);
        // TODO(fabric2seer): unknown name `minecraft`
        //   java: minecraft.gui.setScreen(new SoundSelectionPopup(ChatRuleConfigScreen.this, sound -> {
        //   java: 				if (sound != null) {
        //   java: 					chatRule.setCustomSound(sound);
        //   java: 					soundButton.setMessage(Component.translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.sounds.custom").withStyle(ChatFormatting.YELLOW));
        //   java: 				}
        //   java: 			}));
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    pub fn update_widget_narration(__this: &compat::Shared<Self>, builder: &dyn NarrationElementOutput) {
    }

}

/// SoundSearchMenu's widget half (`super`'s fields: x, y, size, message, ...), lent by the game for each call
impl compat::WidgetHalf for SoundSearchMenu {
    fn widget_half(&mut self) -> &mut AbstractWidget {
        &mut self.base
    }
}

/// SoundSearchMenu as the game's widget: `compat::WidgetAdapter::widget(handle)` where Java hands it on
impl ModWidget for compat::WidgetAdapter<SoundSearchMenu> {
    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_widget_render_state(&mut self, widget: &AbstractWidget, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.lend_ref(widget, |__h| SoundSearchMenu::extract_widget_render_state(__h, &context.graphics, mouse_x, mouse_y, a))
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    fn update_widget_narration(&mut self, widget: &AbstractWidget, output: &mut dyn NarrationElementOutput) {
        self.lend_ref(widget, |__h| SoundSearchMenu::update_widget_narration(__h, output))
    }

    /// java: @Override public void onClick(MouseButtonEvent click, boolean doubled)
    fn on_click(&mut self, widget: &mut AbstractWidget, event: &MouseButtonEvent, double_click: bool) {
        self.lend(widget, |__h| SoundSearchMenu::on_click(__h, event, double_click))
    }

}

// NOTE(fabric2seer): extractWidgetRenderState, onClick, setX, updateWidgetNarration override(s) of AbstractWidget - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them

/// java: private class ContentContainer extends AbstractContainerWidget implements Layout
#[derive(Clone)]
pub struct ContentContainer {
    /// java: extends net.minecraft.client.gui.components.AbstractContainerWidget
    pub base: AbstractContainerWidget,
    /// java: private final List<AbstractWidget> children = new ArrayList<>();
    pub children: Vec<AbstractWidget>,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl std::ops::Deref for ContentContainer {
    type Target = AbstractContainerWidget;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for ContentContainer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl ContentContainer {
    /// java: private ContentContainer()
    pub fn new() -> compat::Shared<Self> {
        // TODO(fabric2seer): static `AbstractScrollArea.defaultSettings` has no Rust counterpart in the port
        //   java: super(0, 0, 0, 0, Component.empty(), AbstractScrollArea.defaultSettings(8));
        let __this: compat::Shared<Self> = todo!("constructor");
        __this
    }

    /// java: @Override public List<? extends GuiEventListener> children()
    pub fn children(__this: &compat::Shared<Self>) -> Vec<Box<dyn GuiEventListener>> {
        return compat::read(__this, |__t| __t.children.clone());
    }

    /// java: @Override protected int contentHeight()
    pub fn content_height(__this: &compat::Shared<Self>) -> i32 {
        // TODO(fabric2seer): `content` is a field of the enclosing instance (ChatRuleConfigScreen.this) - an inner class has no outer reference in Rust
        //   java: return content.getHeight();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override protected double scrollRate()
    pub fn scroll_rate(__this: &compat::Shared<Self>) -> f64 {
        return ((10) as f64) as f64;
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_widget_render_state(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        graphics.enable_scissor(__this.lock().borrow().get_x(), __this.lock().borrow().get_y(), (__this.lock().borrow().get_x()).wrapping_add(compat::read(__this, |__t| __t.width)), (__this.lock().borrow().get_y()).wrapping_add(compat::read(__this, |__t| __t.height)));
        for clickable_widget in compat::read(__this, |__t| __t.children.clone()) {
            // TODO(fabric2seer): `extract_render_state` is a method of the port's trait `Renderable`, which the port's `AbstractWidget` does not implement
            // rust: clickable_widget.extract_render_state(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mouse_x, mouse_y, a);
        }
        graphics.disable_scissor();
        // TODO(fabric2seer): `AbstractContainerWidget.extractScrollbar` has no Rust method in the port (looked for `extract_scrollbar` in AbstractContainerWidget and its supertypes)
        // rust: __this.lock().borrow().extract_scrollbar(graphics, mouse_x, mouse_y);
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    pub fn update_widget_narration(__this: &compat::Shared<Self>, builder: &dyn NarrationElementOutput) {
    }

    /// java: @Override public void visitChildren(Consumer<LayoutElement> consumer)
    pub fn visit_children(__this: &compat::Shared<Self>, consumer: Arc<dyn Fn(LayoutElement)>) {
        (consumer)(__this.clone());
    }

    /// java: @Override public void setX(int x)
    pub fn set_x(__this: &compat::Shared<Self>, mut x: i32) {
        __this.lock().borrow_mut().base.set_x(x);
        // TODO(fabric2seer): `content` is a field of the enclosing instance (ChatRuleConfigScreen.this) - an inner class has no outer reference in Rust
        //   java: content.setX(x + SIDE_PADDING);
    }

    /// java: @Override public void setY(int y)
    pub fn set_y(__this: &compat::Shared<Self>, mut y: i32) {
        __this.lock().borrow_mut().base.set_y(y);
        // TODO(fabric2seer): `refresh_scroll_amount` takes 1 argument(s) in the port (`content_height: i32`), Java passes 0
        // TODO(fabric2seer): `ScrollArea::refresh_scroll_amount` ports `refreshScrollAmount`, but the port's `AbstractContainerWidget` holds no `ScrollArea` (no Deref, no `base` field) to call it on
        // rust: __this.lock().borrow_mut().refresh_scroll_amount();
    }

    /// java: @Override public void setScrollAmount(double scrollY)
    pub fn set_scroll_amount(__this: &compat::Shared<Self>, mut scroll_y: f64) {
        // TODO(fabric2seer): `set_scroll_amount` takes 2 argument(s) in the port (`content_height: i32, scroll_amount: f64`), Java passes 1
        // TODO(fabric2seer): `ScrollArea::set_scroll_amount` ports `setScrollAmount`, but the port's `AbstractContainerWidget` holds no `ScrollArea` (no Deref, no `base` field) to call it on
        // rust: __this.lock().borrow_mut().base.set_scroll_amount(scroll_y);
        // TODO(fabric2seer): `content` is a field of the enclosing instance (ChatRuleConfigScreen.this) - an inner class has no outer reference in Rust
        //   java: content.setY(getY() - (int) scrollAmount());
    }

    /// java: @Override public void arrangeElements()
    pub fn arrange_elements(__this: &compat::Shared<Self>) {
        // TODO(fabric2seer): `content` is a field of the enclosing instance (ChatRuleConfigScreen.this) - an inner class has no outer reference in Rust
        //   java: content.arrangeElements();
        __this.lock().borrow_mut().children.clear();
        // TODO(fabric2seer): `content` is a field of the enclosing instance (ChatRuleConfigScreen.this) - an inner class has no outer reference in Rust
        //   java: content.visitWidgets(children::add);
        // TODO(fabric2seer): `content` is a field of the enclosing instance (ChatRuleConfigScreen.this) - an inner class has no outer reference in Rust
        //   java: setWidth(content.getWidth() + SIDE_PADDING * 2);
        // TODO(fabric2seer): `layout` is a field of the enclosing instance (ChatRuleConfigScreen.this) - an inner class has no outer reference in Rust
        //   java: setHeight(layout.getContentHeight());
    }

    /// java: @Override public void visitWidgets(Consumer<AbstractWidget> consumer)
    pub fn visit_widgets(__this: &compat::Shared<Self>, consumer: Arc<dyn Fn(AbstractWidget)>) {
        // TODO(fabric2seer): `AbstractWidget::visit_widgets` ports `visitWidgets`, but the port's `AbstractContainerWidget` holds no `AbstractWidget` (no Deref, no `base` field) to call it on
        // rust: __this.lock().borrow_mut().base.visit_widgets(&mut consumer);
    }

    /// java: @Override public void removeChildren()
    pub fn remove_children(__this: &compat::Shared<Self>) {
        __this.lock().borrow_mut().children.clear();
    }

}

// NOTE(fabric2seer): arrangeElements, children, contentHeight, extractWidgetRenderState, removeChildren, scrollRate, setScrollAmount, setX, setY, updateWidgetNarration, visitChildren, visitWidgets override(s) of AbstractContainerWidget - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them

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
use crate::de::hysky::skyblocker::utils::render::gui::item_selection_popup::ItemSelectionPopup;
use crate::de::hysky::skyblocker::utils::render::gui::ranged_slider_widget::RangedSliderWidget;
use crate::de::hysky::skyblocker::utils::render::gui::sound_selection_popup::SoundSelectionPopup;
use crate::de::hysky::skyblocker::skyblock::dungeon::partyfinder::finder_settings_container::ContainerWidgetAdapter;
use crate::de::hysky::skyblocker::skyblock::dungeon::partyfinder::finder_settings_container::ContainerWidgetHalf;
use crate::de::hysky::skyblocker::utils::data::json_data::JsonData as _;
use mcrust::net::minecraft::core::defaulted_registry::DefaultedRegistry as _;

use mcrust::java::net::uri::Uri;
use mcrust::mcrust::modding::screens::ModScreen;
use mcrust::mcrust::modding::widgets::ModWidget;
use mcrust::mcrust::modding::widgets::WidgetRenderContext;
use mcrust::net::minecraft::chat_formatting;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::active_text_collector::ClickableStyleFinder;
use mcrust::net::minecraft::client::gui::components::abstract_container_widget::AbstractContainerWidget;
use mcrust::net::minecraft::client::gui::components::abstract_scroll_area::ScrollArea;
use mcrust::net::minecraft::client::gui::components::abstract_scroll_area::ScrollbarSettings;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::PressLambda;
use mcrust::net::minecraft::client::gui::components::abstract_widget::WidgetKind;
use mcrust::net::minecraft::client::gui::components::button;
use mcrust::net::minecraft::client::gui::components::checkbox::Checkbox;
use mcrust::net::minecraft::client::gui::components::cycle_button;
use mcrust::net::minecraft::client::gui::components::cycle_button::ValueListSupplier;
use mcrust::net::minecraft::client::gui::components::edit_box::Responder;
use mcrust::net::minecraft::client::gui::components::edit_box::ResponderLambda;
use mcrust::net::minecraft::client::gui::components::multi_line_text_widget::MultiLineTextWidget;
use mcrust::net::minecraft::client::gui::components::tooltip::Tooltip;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::layouts::frame_layout::FrameLayout;
use mcrust::net::minecraft::client::gui::layouts::grid_layout::GridLayout;
use mcrust::net::minecraft::client::gui::layouts::header_and_footer_layout::HeaderAndFooterLayout;
use mcrust::net::minecraft::client::gui::layouts::layout_element::LayoutElement;
use mcrust::net::minecraft::client::gui::layouts::layout_settings::LayoutSettings;
use mcrust::net::minecraft::client::gui::layouts::linear_layout::LinearLayout;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::gui::text_alignment::TextAlignment;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::client::input::mouse_button_info::MouseButtonInfo;
use mcrust::net::minecraft::network::chat::click_event::ClickEvent;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::formatted_text::FormattedText;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::chat::style::Style;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::sounds::sound_event::SoundEvent;
use mcrust::net::minecraft::util::util;
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

/// One `contentAdder.addChild(..)` of `init`: the child, its column span and cell settings, and - for a
/// `new ToggleableLayoutWidget(child, isEnabled)` - the predicate. The content grid is laid out from these on every
/// `arrangeElements` (see `ChatRuleConfigScreen::build_content`).
#[derive(Clone)]
pub struct ContentEntry {
    pub element: LayoutElement,
    pub column_width: i32,
    pub settings: LayoutSettings,
    /// java: `ToggleableLayoutWidget.isEnabled`
    pub is_enabled: Option<Arc<dyn Fn() -> bool>>,
}

/// java: public class ChatRuleConfigScreen extends Screen
pub struct ChatRuleConfigScreen {
    /// java: extends net.minecraft.client.gui.screens.Screen
    pub base: mcrust::net::minecraft::client::gui::screens::screen::Screen,
    /// java: private final Map<@Nullable SoundEvent, Component> soundNames = Util.make(new Object2ObjectOpenHashMap<>(), map -> { map.put(SoundEvents.NOTE_BLOCK_PLING.value(
    /// (`SoundEvent` is a record over a `float`: no `Hash`/`Eq` in Rust, so the map is its entries, looked up by
    /// `equals`; the iteration order is insertion order where fastutil's is its hash order)
    pub sound_names: Vec<(Option<SoundEvent>, MutableComponent)>,
    /// java: map.defaultReturnValue(..) - what `soundNames.get(sound)` answers for a sound not in the map
    pub sound_names_default: MutableComponent,
    /// java: private final int chatRuleIndex;
    pub chat_rule_index: i32,
    /// java: private final ChatRule chatRule;
    pub chat_rule: compat::Shared<ChatRule>,
    /// java: private ChatRule.@Nullable ToastMessage previousToastMessage = null;
    pub previous_toast_message: Option<compat::Shared<ToastMessage>>,
    /// java: private ChatRule.@Nullable AnnouncementMessage previousAnnouncementMessage = null;
    pub previous_announcement_message: Option<compat::Shared<AnnouncementMessage>>,
    // java: private final Screen parent; - the screen this one was opened over, which
    // `game::set_screen_keeping_parent` holds; `onClose`'s `setScreen(parent)` is `game::set_screen_to_parent()`.
    /// java: private final HeaderAndFooterLayout layout = new HeaderAndFooterLayout(this);
    pub layout: Option<HeaderAndFooterLayout>,
    /// java: private final GridLayout content = new GridLayout().columnSpacing(GRID_SPACING);
    pub content: GridLayout,
    /// java: @SuppressWarnings("rawtypes") private CycleButton soundButton;
    pub sound_button: Option<compat::WidgetRef>,
    /// `init`'s `contentAdder.addChild(..)` calls, in order (the grid is laid out from them)
    pub content_entries: Vec<ContentEntry>,
    /// the `ContentContainer` of `layout.addToContents(new ContentContainer())`, and its widget's index
    pub content_container: Option<(compat::Shared<ContentContainer>, usize)>,
    /// `recreateLayout()` asked for by a responder or a checkbox, which run inside the game's widget pass (where
    /// the screen's widgets are not lent to this object): run at the start of the next lent call
    pub recreate_requested: bool,
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
    ///
    /// (the parent is the screen up when this one is opened: `game::set_screen_keeping_parent` holds it)
    pub fn new(chat_rule_index: i32) -> compat::Shared<Self> {
        // java: this.chatRule = ChatRulesHandler.CHAT_RULE_LIST.getData().get(chatRuleIndex);
        let chat_rule: compat::Shared<ChatRule> = chat_rules_handler::CHAT_RULE_LIST.get().lock().borrow().get_data()[chat_rule_index as usize].clone();
        let __this = compat::shared(Self {
            base: mcrust::net::minecraft::client::gui::screens::screen::Screen::new(component::translatable("skyblocker.config.chat.chatRules.screen.ruleScreen").get_string()),
            self_ref: std::sync::Weak::new(),
            __widgets: Default::default(),
            sound_names: util::make_with(Vec::new(), |map: &mut Vec<(Option<SoundEvent>, MutableComponent)>| {
                use mcrust::net::minecraft::sounds::sound_events;
                map.push((Some(sound_events::note_block_pling().clone()), component::translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.sounds.pling").with_style_format(ChatFormatting::Yellow)));
                map.push((Some(sound_events::ambient_cave().clone()), component::translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.sounds.cave").with_style_format(ChatFormatting::Yellow)));
                map.push((Some(sound_events::zombie_ambient().clone()), component::translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.sounds.zombie").with_style_format(ChatFormatting::Yellow)));
                map.push((Some(sound_events::player_attack_crit().clone()), component::translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.sounds.crit").with_style_format(ChatFormatting::Yellow)));
                map.push((Some(sound_events::arrow_hit_player().clone()), component::translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.sounds.arrowHit").with_style_format(ChatFormatting::Yellow)));
                map.push((Some(sound_events::amethyst_block_hit().clone()), component::translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.sounds.amethyst").with_style_format(ChatFormatting::Yellow)));
                map.push((Some(sound_events::anvil_land().clone()), component::translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.sounds.anvil").with_style_format(ChatFormatting::Yellow)));
                map.push((None, component::translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.sounds.none").with_style_format(ChatFormatting::Red)));
            }),
            // java: map.defaultReturnValue(Component.translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.sounds.custom").withStyle(ChatFormatting.YELLOW));
            sound_names_default: component::translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.sounds.custom").with_style_format(ChatFormatting::Yellow),
            chat_rule_index,
            chat_rule,
            previous_toast_message: None,
            previous_announcement_message: None,
            layout: None,
            // java: new GridLayout().columnSpacing(GRID_SPACING) - `columnSpacing` answers `this`
            content: { let mut __g = GridLayout::new_default(); __g.column_spacing(GRID_SPACING); __g },
            sound_button: None,
            content_entries: Vec::new(),
            content_container: None,
            recreate_requested: false,
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        { let __v = Some(HeaderAndFooterLayout::new(0, 0)); compat::write(&__this, |__t| __t.layout = __v); }
        __this
    }

    /// java: soundNames.get(sound) - the map's value, or its default return value
    fn sound_name(__this: &compat::Shared<Self>, sound: &Option<SoundEvent>) -> MutableComponent {
        compat::read(__this, |__t| __t.sound_names.iter().find(|(__k, _)| __k == sound).map(|(_, __v)| __v.clone()).unwrap_or_else(|| __t.sound_names_default.clone()))
    }

    /// java: private static String getItemString(ItemStack stack)
    pub fn get_item_string(stack: &mut mcrust::net::minecraft::world::item::item_stack::ItemStack) -> String {
        // java: BuiltInRegistries.ITEM.getKey(stack.getItem()) + ItemStackComponentizationFixer.componentsAsString(stack)
        return format!("{}{}", mcrust::net::minecraft::core::registries::built_in_registries::item().get_key_defaulted(&stack.get_item().erased_obj()), ItemStackComponentizationFixer::components_as_string(stack));
    }

    /// java: private static int getWidth(float occupiedColumns)
    pub fn get_width(occupied_columns: f32) -> i32 {
        return (((COLUMN_WIDTH as f32) * occupied_columns) + ((GRID_SPACING as f32) * (occupied_columns - (1 as f32)))) as i32;
    }

    /// `new StringWidget(message, font)` added to the screen: its layout leaf.
    fn add_string_widget(__this: &compat::Shared<Self>, message: MutableComponent) -> LayoutElement {
        let __w = compat::string_widget(0, 0, -1, 9, message);
        let (__ww, __wh) = (__w.get_width(), __w.get_height());
        LayoutElement::widget(compat::write(__this, |__t| __t.add_widget(__w)), __ww, __wh)
    }

    /// `new MultiLineTextWidget(message, font).setMaxWidth(maxWidth).setCentered(centered)` added to the screen: its
    /// layout leaf (the widget is sized as the text lays out in the font).
    fn add_multi_line_text(__this: &compat::Shared<Self>, message: MutableComponent, max_width: i32, centered: bool) -> LayoutElement {
        let text = FormattedText::from(message);
        let widget = MultiLineTextWidget::new(0, 0, text.clone()).set_max_width(max_width).set_centered(centered);
        let __font = compat::font();
        let splitter = __font.splitter();
        let (__ww, __wh) = (widget.get_width(&splitter), widget.get_height(&splitter));
        let mut __w = AbstractWidget::new(0, 0, __ww, __wh, text.get_string(), WidgetKind::PlainMultiLineText(Box::new(widget)), OnPress::Inert);
        __w.set_message_text(text);
        __w.active = false;
        LayoutElement::widget(compat::write(__this, |__t| __t.add_widget(__w)), __ww, __wh)
    }

    /// `new EditBox(font, width, 20, Component.empty())` with `setMaxLength`, `setValue`, `setResponder` (in Java's
    /// order: the initial value does not reach the responder), `setHint` and `setTooltip`, joined to the screen's
    /// widgets: the reference (which reaches the live box) and its layout leaf.
    fn add_edit_box(__this: &compat::Shared<Self>, width: i32, max_length: i32, value: &str, responder: Option<Responder>, hint: Option<MutableComponent>, tooltip: Option<MutableComponent>) -> (compat::WidgetRef, LayoutElement) {
        let mut __w = compat::edit_box(0, 0, width, 20, component::empty());
        let __font = compat::font();
        compat::with_edit_box(&mut __w, |__e| {
            __e.set_max_length(max_length);
            __e.set_value(&__font.splitter(), value);
            if let Some(responder) = responder {
                __e.set_responder(responder);
            }
            if let Some(hint) = hint {
                __e.set_hint_component(FormattedText::from(hint));
            }
        });
        if let Some(tooltip) = tooltip {
            __w.set_tooltip(Tooltip::create_text(FormattedText::from(tooltip)));
        }
        let mut __r = compat::WidgetRef::value(__w);
        let (__i, __ww, __wh) = compat::write(__this, |__t| { let __s = &mut *__t; __s.__widgets.add_to_layout(&mut __s.base, &mut __r) });
        (__r, LayoutElement::widget(__i, __ww, __wh))
    }

    /// `contentAdder.addChild(child, columnWidth, settings)` - recorded; see `ContentEntry`.
    fn add_content(__this: &compat::Shared<Self>, element: LayoutElement, column_width: i32, settings: Option<LayoutSettings>, is_enabled: Option<Arc<dyn Fn() -> bool>>) {
        compat::write(__this, |__t| {
            // `addChild(widget[, columnWidth])` takes `defaultCellSetting()` - the default itself
            let settings = settings.unwrap_or(__t.content.default_cell_settings);
            __t.content_entries.push(ContentEntry { element, column_width, settings, is_enabled });
        });
    }

    /// The live chat rule (the `chatRule` field).
    fn rule(__this: &compat::Shared<Self>) -> compat::Shared<ChatRule> {
        compat::read(__this, |__t| __t.chat_rule.clone())
    }

    /// A responder's or checkbox's `recreateLayout()`: see `recreate_requested`.
    fn request_recreate(__this: &compat::Shared<Self>) {
        compat::write(__this, |__t| __t.recreate_requested = true);
    }

    /// The start of a lent call: a `recreateLayout()` asked for since the last one runs now.
    fn run_requested_recreate(__this: &compat::Shared<Self>) {
        if compat::write(__this, |__t| std::mem::take(&mut __t.recreate_requested)) {
            ChatRuleConfigScreen::recreate_layout(__this);
        }
    }

    /// java: @Override protected void init()
    pub fn init(__this: &compat::Shared<Self>) {
        compat::write(__this, |__t| { __t.content_entries.clear(); __t.content_container = None; __t.recreate_requested = false; });
        { let __v = Some(HeaderAndFooterLayout::new(0, 0)); compat::write(__this, |__t| __t.layout = __v); }
        compat::write(__this, |__t| { let mut __g = GridLayout::new_default(); __g.column_spacing(GRID_SPACING); __t.content = __g; });
        // java: layout.addToHeader(new StringWidget(title, font));
        let __title = compat::read(__this, |__t| __t.title.clone());
        let __header = ChatRuleConfigScreen::add_string_widget(__this, component::literal(&__title));
        compat::write(__this, |__t| __t.layout.as_mut().unwrap().add_to_header(__header));
        // java: layout.addToFooter(Button.builder(CommonComponents.GUI_DONE, _ -> onClose()).build());
        let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        let __footer = { let __w = (button::builder(mcrust::net::minecraft::network::chat::common_components::gui_done().get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, _: usize| { let Some(__this_1) = __this_w1.get().upgrade() else { return; }; compat::lend_screen(&__this_1, __screen, || ChatRuleConfigScreen::on_close(&__this_1)) })))).build(); let (__ww, __wh) = (__w.get_width(), __w.get_height()); LayoutElement::widget(compat::write(__this, |__t| __t.add_widget(__w)), __ww, __wh) };
        compat::write(__this, |__t| __t.layout.as_mut().unwrap().add_to_footer(__footer));
        // java: layout.addToContents(new ContentContainer()); - the container joins the screen's widgets after the
        // content's (the content's widgets are the screen's own children here, so they take the clicks first)
        let __content_container: compat::Shared<ContentContainer> = ContentContainer::new();

        // java: content.defaultCellSetting().alignVerticallyMiddle().alignHorizontallyCenter().paddingTop(GRID_SPACING);
        compat::write(__this, |__t| { let __s = __t.content.default_cell_setting(); *__s = (*__s).align_vertically_middle().align_horizontally_center().padding_top(GRID_SPACING); });
        let aligned_left: LayoutSettings = compat::read(__this, |__t| __t.content.new_cell_settings().align_horizontally_left());
        let rule = ChatRuleConfigScreen::rule(__this);

        // Name
        let __l = ChatRuleConfigScreen::add_string_widget(__this, component::translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.name").with_style_formats(&[ChatFormatting::Bold, ChatFormatting::Underline]));
        ChatRuleConfigScreen::add_content(__this, __l, 3, None, None);
        let __rule_2 = compat::RenderThread::new(rule.clone());
        let (_name_widget, __l) = ChatRuleConfigScreen::add_edit_box(__this, ChatRuleConfigScreen::get_width(3.0), 32, &ChatRule::get_name(&rule), Some(Responder::Lambda(ResponderLambda::new(move |__a0| ChatRule::set_name(__rule_2.get(), __a0.to_string())))), None, Some(component::translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.name.@Tooltip")));
        ChatRuleConfigScreen::add_content(__this, __l, 3, None, None);

        // Filter
        let __l = ChatRuleConfigScreen::add_string_widget(__this, component::translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.filter").with_style_formats(&[ChatFormatting::Bold, ChatFormatting::Underline]));
        ChatRuleConfigScreen::add_content(__this, __l, 3, None, None);
        // java: filterInput.addFormatter(createRenderTextProvider(filterInput::getValue, true));
        // rust: port gap - `EditBox.TextFormatter` is a closed enum of the game's two formatters, so the mod's formatter
        // (`create_render_text_provider_2`) cannot be attached to the box (GAPS_batchF.md)
        let __rule_3 = compat::RenderThread::new(rule.clone());
        let (_filter_input, __l) = ChatRuleConfigScreen::add_edit_box(__this, ChatRuleConfigScreen::get_width(3.0), 1024, &ChatRule::get_filter(&rule), Some(Responder::Lambda(ResponderLambda::new(move |__a0| ChatRule::set_filter(__rule_3.get(), __a0.to_string())))), Some(component::translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.filter").with_style_formats(&[ChatFormatting::Gray, ChatFormatting::Italic])), Some(component::translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.filter.@Tooltip")));
        ChatRuleConfigScreen::add_content(__this, __l, 3, None, None);

        // Filter settings
        let mut filters_row1: LinearLayout = LinearLayout::horizontal().spacing(GRID_SPACING);
        let __rule_4 = compat::RenderThread::new(rule.clone());
        filters_row1.add_child(LayoutElement::Frame(ChatRuleConfigScreen::build_checkbox(__this, &component::translatable_args("skyblocker.config.chat.chatRules.screen.ruleScreen.regex", vec![FormattedText::from(REGEX_LINK.get_mut().clone())]), &component::translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.regex.@Tooltip"), ChatRuleConfigScreen::get_width(1.5f32), &TextAlignment::Center, Arc::new(move |__a0| ChatRule::set_regex(__rule_4.get(), __a0)), ChatRule::get_regex(&rule))));
        let __this_5 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        filters_row1.add_child({
            let mut __w = button::builder(component::translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.locations").get_string(), OnPress::Lambda(PressLambda::new(move |_: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, _: usize| {
                let Some(__this_5) = __this_5.get().upgrade() else { return; };
                // java: minecraft.gui.setScreen(new ChatRuleLocationConfigScreen(this, chatRule))
                mcrust::mcrust::modding::game::set_screen_keeping_parent(Box::new(compat::ScreenAdapter::new(ChatRuleLocationConfigScreen::new(ChatRuleConfigScreen::rule(&__this_5)))));
            }))).width(ChatRuleConfigScreen::get_width(1.5f32)).build();
            __w.set_tooltip(Tooltip::create_text(FormattedText::from(component::translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.locations.@Tooltip"))));
            let (__ww, __wh) = (__w.get_width(), __w.get_height());
            LayoutElement::widget(compat::write(__this, |__t| __t.add_widget(__w)), __ww, __wh)
        });
        ChatRuleConfigScreen::add_content(__this, LayoutElement::Grid(filters_row1.wrapped), 3, None, None);
        let mut filters_row2: LinearLayout = LinearLayout::horizontal().spacing(GRID_SPACING);
        let __rule_6 = compat::RenderThread::new(rule.clone());
        filters_row2.add_child(LayoutElement::Frame(ChatRuleConfigScreen::build_checkbox(__this, &component::translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.includeFormatting"), &component::translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.includeFormatting.@Tooltip"), ChatRuleConfigScreen::get_width(1.25f32), &TextAlignment::Left, Arc::new(move |__a0| ChatRule::set_include_formatting(__rule_6.get(), __a0)), ChatRule::get_include_formatting(&rule))));
        let __rule_7 = compat::RenderThread::new(rule.clone());
        filters_row2.add_child(LayoutElement::Frame(ChatRuleConfigScreen::build_checkbox(__this, &component::translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.partialMatch"), &component::translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.partialMatch.@Tooltip"), ChatRuleConfigScreen::get_width(0.75f32), &TextAlignment::Center, Arc::new(move |__a0| ChatRule::set_partial_match(__rule_7.get(), __a0)), ChatRule::get_partial_match(&rule))));
        let __rule_8 = compat::RenderThread::new(rule.clone());
        filters_row2.add_child(LayoutElement::Frame(ChatRuleConfigScreen::build_checkbox(__this, &component::translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.ignoreCase"), &component::translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.ignoreCase.@Tooltip"), ChatRuleConfigScreen::get_width(1.0f32), &TextAlignment::Right, Arc::new(move |__a0| ChatRule::set_ignore_case(__rule_8.get(), __a0)), ChatRule::get_ignore_case(&rule))));
        ChatRuleConfigScreen::add_content(__this, LayoutElement::Grid(filters_row2.wrapped), 3, None, None);

        // ==== Outputs
        let __l = ChatRuleConfigScreen::add_string_widget(__this, component::translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.outputs").with_style_formats(&[ChatFormatting::Bold, ChatFormatting::Underline]));
        let __s = compat::read(__this, |__t| __t.content.new_cell_settings().padding_top(4 + GRID_SPACING));
        ChatRuleConfigScreen::add_content(__this, __l, 3, Some(__s), None);
        // java: contentAdder.addChild(new MultiLineTextWidget(..formatting.., font).setMaxWidth(getWidth(2)), 3).setCentered(true);
        let __l = ChatRuleConfigScreen::add_multi_line_text(__this, component::translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.formatting"), ChatRuleConfigScreen::get_width(2.0), true);
        ChatRuleConfigScreen::add_content(__this, __l, 3, None, None);

        let mut buttons: LinearLayout = LinearLayout::horizontal().spacing(GRID_SPACING);
        let __this_9 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        buttons.add_child(LayoutElement::Frame(ChatRuleConfigScreen::build_checkbox(__this, &component::translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.hideMessage"), &component::translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.hideMessage.@Tooltip"), ChatRuleConfigScreen::get_width(1.5f32), &TextAlignment::Left, Arc::new(move |value| {
            let Some(__this_9) = __this_9.get().upgrade() else { return; };
            ChatRule::set_hide_message(&ChatRuleConfigScreen::rule(&__this_9), value);
            ChatRuleConfigScreen::request_recreate(&__this_9);
        }), ChatRule::get_hide_message(&rule))));

        // Sound
        // java: List<Optional<SoundEvent>> availableValues = new ArrayList<>(soundNames.keySet().stream().map(Optional::ofNullable).toList());
        let available_values: Vec<Option<SoundEvent>> = compat::read(__this, |__t| __t.sound_names.iter().map(|(__k, _)| __k.clone()).collect::<Vec<_>>());
        let displayed_values: Vec<Option<SoundEvent>>;
        let __custom_sound: Option<SoundEvent> = ChatRule::get_custom_sound(&rule);
        if compat::read(__this, |__t| __t.sound_names.iter().any(|(__k, _)| *__k == __custom_sound)) {
            displayed_values = available_values.clone();
        } else {
            displayed_values = available_values.iter().cloned().chain(std::iter::once(__custom_sound.clone())).collect::<Vec<_>>();
        }
        // java: soundButton = CycleButton.builder(opt -> soundNames.get(opt.orElse(null)), Optional.ofNullable(chatRule.getCustomSound()))
        //   .withValues(() -> true, displayedValues, availableValues) - the alt list (`availableValues`) is always the selected one
        //   .create(0, 0, getWidth(1.3f), 20, ..sounds.., (_, value) -> {..});
        // (the port's cycle button cycles the values' labels; the listener maps the index back to the value)
        let __labels = |__values: &Vec<Option<SoundEvent>>| -> Vec<String> { __values.iter().map(|__v| ChatRuleConfigScreen::sound_name(__this, __v).get_string()).collect() };
        let mut __supplier = ValueListSupplier::create_with_alt(__labels(&displayed_values), __labels(&available_values));
        __supplier.alt_selected = true;
        let __builder = cycle_button::builder(ChatRuleConfigScreen::sound_name(__this, &__custom_sound).get_string()).with_value_list_supplier(__supplier);
        let __rule_10 = compat::RenderThread::new(rule.clone());
        let __available = compat::RenderThread::new(available_values.clone());
        let mut __sound_button = compat::cycle_button_widget(0, 0, ChatRuleConfigScreen::get_width(1.3f32), 20, &component::translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.sounds"), __builder, move |__index: usize| {
            let value: Option<SoundEvent> = __available.get().get(__index).cloned().flatten();
            // java: chatRule.setCustomSound(value.orElse(null));
            ChatRule::set_custom_sound(__rule_10.get(), value.clone());
            // java: value.ifPresent(soundEvent -> minecraft.getSoundManager().play(SimpleSoundInstance.forUI(soundEvent, 1.0f)));
            // rust: port gap (GAPS_batchH #1) - `SoundManager.play` is not reachable from a mod
        });
        // java: soundButton.setTooltip(Tooltip.create(..sounds.@Tooltip));
        __sound_button.set_tooltip(Tooltip::create_text(FormattedText::from(component::translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.sounds.@Tooltip"))));
        let mut __sound_ref = compat::WidgetRef::value(__sound_button);
        let (__i, __ww, __wh) = compat::write(__this, |__t| { let __s = &mut *__t; __s.__widgets.add_to_layout(&mut __s.base, &mut __sound_ref) });
        compat::write(__this, |__t| __t.sound_button = Some(__sound_ref));
        buttons.add_child(LayoutElement::widget(__i, __ww, __wh));
        // search button
        let __search = SoundSearchMenu::new(std::sync::Arc::downgrade(__this));
        buttons.add_child_adjusted({ let __w = compat::WidgetAdapter::widget(__search.clone()); let (__ww, __wh) = (__w.get_width(), __w.get_height()); LayoutElement::widget(compat::write(__this, |__t| __t.add_widget(__w)), __ww, __wh) }, &mut |__a0| { *__a0 = __a0.align_horizontally_right(); });
        ChatRuleConfigScreen::add_content(__this, LayoutElement::Grid(buttons.wrapped), 3, None, None);

        // Chat message
        let __rule_11 = compat::RenderThread::new(rule.clone());
        let __not_hidden: Arc<dyn Fn() -> bool> = Arc::new(move || !ChatRule::get_hide_message(__rule_11.get()));
        let __l = ChatRuleConfigScreen::add_multi_line_text(__this, component::translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.chatMessage"), ChatRuleConfigScreen::get_width(1.0), false);
        ChatRuleConfigScreen::add_content(__this, __l, 1, Some(aligned_left), Some(__not_hidden.clone()));
        // java: chatMessageInput.addFormatter(createRenderTextProvider(chatMessageInput::getValue));
        // rust: port gap - `EditBox.TextFormatter` is a closed enum; the mod's formatter cannot be attached (GAPS_batchF.md)
        let __rule_12 = compat::RenderThread::new(rule.clone());
        let (_chat_message_input, __l) = ChatRuleConfigScreen::add_edit_box(__this, ChatRuleConfigScreen::get_width(2.0), 1024, &ChatRule::get_chat_message(&rule).unwrap_or_default(), Some(Responder::Lambda(ResponderLambda::new(move |__a0| ChatRule::set_chat_message(__rule_12.get(), Some(__a0.to_string()))))), Some(component::translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.chatMessage.@Placeholder").with_style_formats(&[ChatFormatting::Gray, ChatFormatting::Italic])), None);
        ChatRuleConfigScreen::add_content(__this, __l, 2, None, Some(__not_hidden));

        // Action Bar
        let __l = ChatRuleConfigScreen::add_multi_line_text(__this, component::translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.actionBar"), ChatRuleConfigScreen::get_width(1.0), false);
        ChatRuleConfigScreen::add_content(__this, __l, 1, Some(aligned_left), None);
        let __rule_13 = compat::RenderThread::new(rule.clone());
        let (_action_bar_message_input, __l) = ChatRuleConfigScreen::add_edit_box(__this, ChatRuleConfigScreen::get_width(2.0), 1024, &ChatRule::get_action_bar_message(&rule).unwrap_or_default(), Some(Responder::Lambda(ResponderLambda::new(move |__a0| ChatRule::set_action_bar_message(__rule_13.get(), Some(__a0.to_string()))))), Some(component::translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.outputField.@Placeholder").with_style_formats(&[ChatFormatting::Gray, ChatFormatting::Italic])), None);
        ChatRuleConfigScreen::add_content(__this, __l, 2, None, None);

        // Announcement
        let __l = ChatRuleConfigScreen::add_multi_line_text(__this, component::translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.announcement"), ChatRuleConfigScreen::get_width(1.0), false);
        ChatRuleConfigScreen::add_content(__this, __l, 1, Some(aligned_left), None);
        let __announcement_value: String = ChatRule::get_announcement_message(&rule).map(|__m| compat::read(&__m, |__t| __t.message.clone())).unwrap_or_default();
        let __this_14 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        let (_announcement_message_input, __l) = ChatRuleConfigScreen::add_edit_box(__this, ChatRuleConfigScreen::get_width(2.0), 1024, &__announcement_value, Some(Responder::Lambda(ResponderLambda::new(move |s| {
            let Some(__this_14) = __this_14.get().upgrade() else { return; };
            let rule = ChatRuleConfigScreen::rule(&__this_14);
            if s.is_empty() {
                { let __v = ChatRule::get_announcement_message(&rule); compat::write(&__this_14, |__t| __t.previous_announcement_message = __v); }
                ChatRule::set_announcement_message(&rule, None);
            } else {
                if ChatRule::get_announcement_message(&rule).is_none() {
                    let __previous = compat::read(&__this_14, |__t| __t.previous_announcement_message.clone());
                    ChatRule::set_announcement_message(&rule, Some(__previous.unwrap_or_else(AnnouncementMessage::new_0)));
                }
                { let __v = s.to_string(); compat::write(&ChatRule::get_announcement_message(&rule).unwrap(), |__t| __t.message = __v); }
            }
            ChatRuleConfigScreen::request_recreate(&__this_14);
        }))), Some(component::translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.outputField.@Placeholder").with_style_formats(&[ChatFormatting::Gray, ChatFormatting::Italic])), Some(component::translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.announcement.@Tooltip")));
        ChatRuleConfigScreen::add_content(__this, __l, 2, None, None);

        // java: contentAdder.addChild(SpacerElement.height(0), content.newCellSettings().paddingTop(0));
        let __s = compat::read(__this, |__t| __t.content.new_cell_settings().padding_top(0));
        ChatRuleConfigScreen::add_content(__this, LayoutElement::Spacer { x: 0, y: 0, width: 0, height: 0 }, 1, Some(__s), None);
        let __rule_15 = compat::RenderThread::new(rule.clone());
        let announcement_duration_slider: compat::Shared<RangedSliderWidget> = RangedSliderWidget::builder().min_max(1.0, 10.0).step(0.1).width(ChatRuleConfigScreen::get_width(2.0)).default_value(match ChatRule::get_announcement_message(&rule) { Some(__m) => compat::read(&__m, |__t| __t.display_duration) as f64 / 1000.0, None => 5000.0 }).option_formatter_component_double2_object_function(&component::translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.announcement.duration"), &(Arc::new(move |d: f64| component::literal(&format!("{}s", formatters::FLOAT_NUMBERS.get_mut().format(d)))) as Arc<dyn Fn(f64) -> MutableComponent>)).callback(Arc::new(move |d: f64| {
            let Some(__m) = ChatRule::get_announcement_message(__rule_15.get()) else { return; };
            compat::write(&__m, |__t| __t.display_duration = (d * 1000.0) as i64);
        })).build();
        // java: Util.make(new FrameLayout(), w -> w.addChild(announcementDurationSlider, p -> p.paddingTop(GRID_SPACING)))
        let mut __frame = FrameLayout::new();
        { let __w = RangedSliderWidget::widget(announcement_duration_slider); let (__ww, __wh) = (__w.get_width(), __w.get_height()); let __i = compat::write(__this, |__t| __t.add_widget(__w)); __frame.add_child_adjusted(LayoutElement::widget(__i, __ww, __wh), &mut |__p| { *__p = __p.padding_top(GRID_SPACING); }); }
        let __rule_16 = compat::RenderThread::new(rule.clone());
        let __s = compat::read(__this, |__t| __t.content.new_cell_settings().padding_top(0));
        ChatRuleConfigScreen::add_content(__this, LayoutElement::Frame(__frame), 2, Some(__s), Some(Arc::new(move || ChatRule::get_announcement_message(__rule_16.get()).is_some())));

        // Toast
        let __l = ChatRuleConfigScreen::add_multi_line_text(__this, component::translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.toast"), ChatRuleConfigScreen::get_width(1.0), false);
        ChatRuleConfigScreen::add_content(__this, __l, 1, Some(aligned_left), None);
        let __toast_value: String = ChatRule::get_toast_message(&rule).map(|__m| compat::read(&__m, |__t| __t.message.clone())).unwrap_or_default();
        let __this_17 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        let (_toast_message_input, __l) = ChatRuleConfigScreen::add_edit_box(__this, ChatRuleConfigScreen::get_width(2.0), 1024, &__toast_value, Some(Responder::Lambda(ResponderLambda::new(move |s| {
            let Some(__this_17) = __this_17.get().upgrade() else { return; };
            let rule = ChatRuleConfigScreen::rule(&__this_17);
            if s.is_empty() {
                { let __v = ChatRule::get_toast_message(&rule); compat::write(&__this_17, |__t| __t.previous_toast_message = __v); }
                ChatRule::set_toast_message(&rule, None);
            } else {
                if ChatRule::get_toast_message(&rule).is_none() {
                    let __previous = compat::read(&__this_17, |__t| __t.previous_toast_message.clone());
                    ChatRule::set_toast_message(&rule, Some(__previous.unwrap_or_else(ToastMessage::new_0)));
                }
                { let __v = s.to_string(); compat::write(&ChatRule::get_toast_message(&rule).unwrap(), |__t| __t.message = __v); }
            }
            ChatRuleConfigScreen::request_recreate(&__this_17);
        }))), Some(component::translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.outputField.@Placeholder").with_style_formats(&[ChatFormatting::Gray, ChatFormatting::Italic])), None);
        ChatRuleConfigScreen::add_content(__this, __l, 2, None, None);

        let __rule_18 = compat::RenderThread::new(rule.clone());
        let toast_options_predicate: Arc<dyn Fn() -> bool> = Arc::new(move || ChatRule::get_toast_message(__rule_18.get()).is_some());
        // Label + preview
        let mut text_and_icon: FrameLayout = FrameLayout::of(ChatRuleConfigScreen::get_width(1.0), 0);
        // java: EditBox itemInput = new EditBox(font, getWidth(1), 20, Component.empty()); itemInput.setMaxLength(300);
        // java: ToastIconPreview preview = textAndIcon.addChild(new ToastIconPreview(itemInput), LayoutSettings::alignHorizontallyRight);
        let preview: compat::Shared<ToastIconPreview> = ToastIconPreview::new(std::sync::Arc::downgrade(__this));
        let __preview_widget = compat::WidgetAdapter::widget(preview.clone());
        let (__pw, __ph) = (__preview_widget.get_width(), __preview_widget.get_height());
        let __preview_index = compat::write(__this, |__t| __t.add_widget(__preview_widget));
        text_and_icon.add_child_adjusted(LayoutElement::widget(__preview_index, __pw, __ph), &mut |__a0| { *__a0 = __a0.align_horizontally_right(); });
        // java: textAndIcon.addChild(new MultiLineTextWidget(..toast.icon.., font), LayoutSettings::alignHorizontallyLeft).setMaxWidth(getWidth(1) - preview.getWidth()).setCentered(false);
        let __l = ChatRuleConfigScreen::add_multi_line_text(__this, component::translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.toast.icon"), ChatRuleConfigScreen::get_width(1.0) - __pw, false);
        text_and_icon.add_child_adjusted(__l, &mut |__a0| { *__a0 = __a0.align_horizontally_left(); });
        ChatRuleConfigScreen::add_content(__this, LayoutElement::Frame(text_and_icon), 1, None, Some(toast_options_predicate.clone()));

        // Item input
        let __preview = compat::RenderThread::new(preview.clone());
        let __rule_19 = compat::RenderThread::new(rule.clone());
        let __item_responder = Responder::Lambda(ResponderLambda::new(move |item_data| {
            let mut parsed_stack: mcrust::net::minecraft::world::item::item_stack::ItemStack = ItemStackComponentizationFixer::from_item_string(item_data, 1);
            if parsed_stack.is_empty() {
                // java: parsedStack = INVALID_ITEM.getStack(); if (parsedStack == null) return;
                let Some(__invalid) = FlexibleItemStack::get_stack(INVALID_ITEM.get_mut()) else { return; };
                parsed_stack = __invalid;
            }
            let stack: compat::Shared<FlexibleItemStack> = FlexibleItemStack::new4(parsed_stack);
            { let __v = FlexibleItemStack::get_stack_or_empty(&stack); compat::write(__preview.get(), |__t| __t.stack = __v); }
            if compat::read(__preview.get(), |__t| __t.stack.is_empty()) {
                return;
            }
            let Some(message) = ChatRule::get_toast_message(__rule_19.get()) else { return; };
            compat::write(&message, |__t| __t.icon = Some(stack.clone()));
        }));
        // java: itemInput.setValue(chatRule.getToastMessage() != null ? getItemString(chatRule.getToastMessage().icon.map(FlexibleItemStack::getStack).orElse(ItemStack.EMPTY)) : "minecraft:painting");
        // (Java sets the value AFTER the responder, so the preview shows the icon from the start)
        let __item_value: String = match ChatRule::get_toast_message(&rule) {
            Some(__m) => ChatRuleConfigScreen::get_item_string(&mut compat::read(&__m, |__t| __t.icon.clone()).and_then(|__a0| FlexibleItemStack::get_stack(&__a0)).unwrap_or(mcrust::net::minecraft::world::item::item_stack::empty())),
            None => "minecraft:painting".to_string(),
        };
        let (mut item_input, __l) = ChatRuleConfigScreen::add_edit_box(__this, ChatRuleConfigScreen::get_width(1.0), 300, "", Some(__item_responder), None, Some(component::translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.toast.icon.@Tooltip")));
        let __font = compat::font();
        compat::write(__this, |__t| { let __s = &mut *__t; __s.__widgets.with(&mut __s.base, &mut item_input, |__w| compat::with_edit_box(__w, |__e| __e.set_value(&__font.splitter(), &__item_value))) });
        compat::write(&preview, |__t| __t.input = Some(item_input.clone()));
        ChatRuleConfigScreen::add_content(__this, __l, 1, None, Some(toast_options_predicate.clone()));
        // java: if (minecraft.level == null)
        if mcrust::mcrust::modding::game::level().is_none() {
            let tooltip = Tooltip::create_text(FormattedText::from(component::translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.toast.icon.unableToEdit")));
            compat::write(__this, |__t| { let __s = &mut *__t; __s.__widgets.with(&mut __s.base, &mut item_input, |__w| {
                // java: itemInput.setEditable(false);
                let __focused = __w.is_focused();
                compat::with_edit_box(__w, |__e| __e.set_editable(false, __focused));
                // java: itemInput.setTooltip(tooltip);
                __w.set_tooltip(tooltip.clone());
            }) });
            // java: preview.active = false; preview.setTooltip(tooltip);
            compat::write(__this, |__t| { let __w = __t.widget_mut(__preview_index); __w.active = false; __w.set_tooltip(tooltip.clone()); });
        }

        // Duration slider
        let __rule_20 = compat::RenderThread::new(rule.clone());
        let slider_widget: compat::Shared<RangedSliderWidget> = RangedSliderWidget::builder().min_max(1.0, 10.0).step(0.1).width(ChatRuleConfigScreen::get_width(1.0)).default_value(match ChatRule::get_toast_message(&rule) { Some(__m) => compat::read(&__m, |__t| __t.display_duration) as f64 / 1000.0, None => 5000.0 }).option_formatter_component_double2_object_function(&component::translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.toast.duration"), &(Arc::new(move |d: f64| component::literal(&format!("{}s", formatters::FLOAT_NUMBERS.get_mut().format(d)))) as Arc<dyn Fn(f64) -> MutableComponent>)).callback(Arc::new(move |d: f64| {
            let Some(__m) = ChatRule::get_toast_message(__rule_20.get()) else { return; };
            compat::write(&__m, |__t| __t.display_duration = (d * 1000.0) as i64);
        })).build();
        {
            let mut __w = RangedSliderWidget::widget(slider_widget);
            // java: sliderWidget.setTooltip(Tooltip.create(..toast.duration.@Tooltip));
            __w.set_tooltip(Tooltip::create_text(FormattedText::from(component::translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.toast.duration.@Tooltip"))));
            let (__ww, __wh) = (__w.get_width(), __w.get_height());
            let __i = compat::write(__this, |__t| __t.add_widget(__w));
            ChatRuleConfigScreen::add_content(__this, LayoutElement::widget(__i, __ww, __wh), 1, None, Some(toast_options_predicate));
        }

        // the container joins the layout (`layout.addToContents(new ContentContainer())`) and the screen's widgets
        let __container_widget = ContainerWidgetAdapter::widget(__content_container.clone());
        let __ci = compat::write(__this, |__t| __t.add_widget(__container_widget));
        compat::write(__this, |__t| __t.content_container = Some((__content_container, __ci)));
        ChatRuleConfigScreen::recreate_layout(__this);
    }

    /// java: private FrameLayout buildCheckbox(Component text, Component tooltip, int width, TextAlignment align, Consumer<Boolean> setter, boolean selected)
    pub fn build_checkbox(__this: &compat::Shared<Self>, text: &MutableComponent, tooltip: &MutableComponent, width: i32, align: &TextAlignment, setter: Arc<dyn Fn(bool)>, selected: bool) -> FrameLayout {
        // java: FrameLayout frame = new FrameLayout().setMinWidth(width); - `setMinWidth` answers `this`
        let mut frame: FrameLayout = FrameLayout::new();
        frame.set_min_width(width);
        // java: Checkbox box = Checkbox.builder(text, font).selected(selected).onValueChange((_, value) -> setter.accept(value)).maxWidth(width).build();
        // (`Builder.build` keeps the tooltip only when the text overflows its rows; no tooltip was given to the builder)
        let __font = compat::font();
        let r#box: Checkbox = Checkbox::build(&__font.splitter(), 0, 0, Some(width), FormattedText::from(text.clone()), selected, false);
        // (the port's `LayoutSettings` setters answer a new value: written back to the frame's default)
        match *align {
            TextAlignment::Left => {
                let __s = frame.default_child_layout_setting();
                *__s = __s.align_horizontally_left();
            }
            TextAlignment::Center => {
                let __s = frame.default_child_layout_setting();
                *__s = __s.align_horizontally_center();
            }
            TextAlignment::Right => {
                let __s = frame.default_child_layout_setting();
                *__s = __s.align_horizontally_right();
            }
        }
        // java: box.setTooltip(Tooltip.create(tooltip)); frame.addChild(box);
        // The box joins the screen's widgets (a layout holds a widget by index); its press is `Checkbox.onPress`:
        // `selected = !selected; onValueChange.onValueChange(this, selected)` - here the toggle, then the setter.
        let __setter = compat::RenderThread::new(setter);
        let (__bw, __bh) = (r#box.width, r#box.height);
        let mut __box_widget = AbstractWidget::new(0, 0, __bw, __bh, text.get_string(), WidgetKind::Checkbox(r#box), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| {
            let __value = if let WidgetKind::Checkbox(__c) = &mut __screen.widget_mut(__index).kind { __c.on_press() } else { return; };
            (__setter.get())(__value);
        })));
        __box_widget.set_tooltip(Tooltip::create_text(FormattedText::from(tooltip.clone())));
        frame.add_child(LayoutElement::widget(compat::write(__this, |__t| __t.add_widget(__box_widget)), __bw, __bh));
        return frame;
    }

    /// java: private EditBox.TextFormatter createRenderTextProvider(Supplier<String> fullTextSupplier)
    pub fn create_render_text_provider(__this: &compat::Shared<Self>, full_text_supplier: Arc<dyn Fn() -> String>) -> ModTextFormatter {
        return ChatRuleConfigScreen::create_render_text_provider_2(__this, full_text_supplier.clone(), false);
    }

    /// java: private EditBox.TextFormatter createRenderTextProvider(Supplier<String> fullTextSupplier, boolean onlyIfFormatted)
    pub fn create_render_text_provider_2(__this: &compat::Shared<Self>, full_text_supplier: Arc<dyn Fn() -> String>, only_if_formatted: bool) -> ModTextFormatter {
        let __this_1 = compat::RenderThread::new(__this.clone());
        let __supplier = compat::RenderThread::new(full_text_supplier);
        return Arc::new(move |s: &str, start: i32| -> Option<mcrust::net::minecraft::util::formatted_char_sequence::Seq> {
            if only_if_formatted && (!(ChatRule::get_include_formatting(&compat::read(__this_1.get(), |__t| __t.chat_rule.clone()))) || ChatRule::get_regex(&compat::read(__this_1.get(), |__t| __t.chat_rule.clone()))) {
                return None; // java: return null
            }
            // java: return visitor -> {..};
            return Some(Arc::new(RenderTextSequence { full_text_supplier: __supplier.clone(), length: compat::str_len(s), start }));
        });
    }

    /// The content grid as `arrangeElements` sees it: each recorded `contentAdder.addChild` in order, a
    /// `ToggleableLayoutWidget` whose `isEnabled` answers false standing as an empty cell (Java's disabled toggle is
    /// 0 x 0 and visits no child, which is a `SpacerElement` of no size).
    fn build_content(__this: &compat::Shared<Self>) {
        let (entries, default_settings) = compat::read(__this, |__t| (__t.content_entries.clone(), __t.content.default_cell_settings));
        let mut grid = GridLayout::new_default();
        grid.column_spacing(GRID_SPACING);
        grid.default_cell_settings = default_settings;
        let mut adder = grid.create_row_helper(3);
        for entry in entries {
            let enabled = entry.is_enabled.as_ref().map_or(true, |__f| __f());
            let element = if enabled { entry.element } else { LayoutElement::Spacer { x: 0, y: 0, width: 0, height: 0 } };
            let _ = adder.add_child_spanning(&mut grid, element, entry.column_width, entry.settings);
        }
        compat::write(__this, |__t| __t.content = grid);
    }

    /// java: private void recreateLayout() - `clearWidgets(); repositionElements(); layout.visitWidgets(this::addRenderableWidget);`
    ///
    /// The screen keeps every widget `init` made (a layout holds them by index): the ones the layout visits now are
    /// rendered and reachable, the rest (a disabled `ToggleableLayoutWidget`'s) leave the screen's lists.
    pub fn recreate_layout(__this: &compat::Shared<Self>) {
        ChatRuleConfigScreen::reposition_elements(__this);
        let mut __visited: Vec<usize> = compat::read(__this, |__t| __t.layout.as_ref().unwrap().visit_widgets()).into_iter().map(|(__i, _, _)| __i).collect();
        let mut __content: Vec<(usize, i32, i32)> = Vec::new();
        compat::read(__this, |__t| LayoutElement::Grid(__t.content.clone()).visit_widgets(&mut __content));
        __visited.extend(__content.into_iter().map(|(__i, _, _)| __i));
        compat::write(__this, |__t| {
            let __count = __t.children().len();
            for __i in 0..__count {
                if __visited.contains(&__i) {
                    __t.widget_mut(__i).visible = true;
                    __t.mark_renderable(__i);
                } else {
                    __t.remove_widget(__i);
                }
            }
        });
    }

    /// java: @Override protected void repositionElements() - `layout.arrangeElements()`, which arranges the
    /// `ContentContainer` (`content.arrangeElements()`, its size) and places it (`setX`/`setY`: the content follows,
    /// scrolled)
    pub fn reposition_elements(__this: &compat::Shared<Self>) {
        let (__sw, __sh) = compat::read(__this, |__t| (__t.width, __t.height));
        // ContentContainer.arrangeElements: content.arrangeElements(); setWidth(content.getWidth() + SIDE_PADDING * 2);
        // setHeight(layout.getContentHeight());
        ChatRuleConfigScreen::build_content(__this);
        let (content_width, content_height) = compat::write(__this, |__t| { __t.content.arrange_elements(); (__t.content.width, __t.content.height) });
        let Some((container, container_index)) = compat::read(__this, |__t| __t.content_container.clone()) else { return; };
        let (container_width, container_height) = compat::write(__this, |__t| {
            let layout = __t.layout.as_mut().unwrap();
            layout.screen_width = __sw;
            layout.screen_height = __sh;
            (content_width + CONTENT_CONTAINER_SIDE_PADDING * 2, layout.get_content_height())
        });
        compat::write(__this, |__t| {
            __t.widget_mut(container_index).set_size(container_width, container_height);
            let layout = __t.layout.as_mut().unwrap();
            layout.contents_frame.remove_children();
            layout.add_to_contents(LayoutElement::widget(container_index, container_width, container_height));
            layout.arrange_elements();
            for (__i, __x, __y) in layout.visit_widgets() {
                __t.widget_mut(__i).set_position(__x, __y);
            }
        });
        let (container_x, container_y) = compat::read(__this, |__t| (__t.widget(container_index).get_x(), __t.widget(container_index).get_y()));
        // ContentContainer.setY: super.setY(y); refreshScrollAmount();
        compat::write(&container, |__c| {
            __c.base.set_position(container_x, container_y);
            __c.base.set_size(container_width, container_height);
            __c.content_height = content_height;
            __c.area.y = container_y;
            __c.area.height = container_height;
            __c.area.refresh_scroll_amount(content_height);
        });
        ChatRuleConfigScreen::place_content(__this);
    }

    /// ContentContainer.setX / setScrollAmount: `content.setX(x + SIDE_PADDING)`, `content.setY(getY() - (int)
    /// scrollAmount())` - the content's widgets moved with it.
    fn place_content(__this: &compat::Shared<Self>) {
        let Some((container, _)) = compat::read(__this, |__t| __t.content_container.clone()) else { return; };
        let (x, y, bottom, scroll) = compat::read(&container, |__c| (__c.base.get_x(), __c.base.get_y(), __c.base.get_bottom(), __c.area.scroll_amount()));
        let mut __placed: Vec<(usize, i32, i32)> = Vec::new();
        compat::write(__this, |__t| {
            __t.content.x = x + CONTENT_CONTAINER_SIDE_PADDING;
            __t.content.y = y - scroll as i32;
            __t.content.arrange_elements();
            LayoutElement::Grid(__t.content.clone()).visit_widgets(&mut __placed);
        });
        compat::write(&container, |__c| __c.children = __placed.iter().map(|(__i, _, _)| *__i).collect());
        compat::write(__this, |__t| {
            for (__i, __x, __y) in &__placed {
                let __w = __t.widget_mut(*__i);
                __w.set_position(*__x, *__y);
                // ContentContainer.extractWidgetRenderState draws its children inside `enableScissor(getX(), getY(),
                // getRight(), getBottom())`: a child wholly outside the box draws nothing.
                // rust: port gap - the game draws the content's widgets itself, without the container's scissor, so a
                // child partly outside the box is drawn whole (GAPS_N #3)
                __w.visible = *__y + __w.get_height() > y && *__y < bottom;
            }
        });
    }

    /// java: @Override public boolean mouseClicked(final MouseButtonEvent event, final boolean doubleClick)
    pub fn mouse_clicked(__this: &compat::Shared<Self>, event: &MouseButtonEvent, _double_click: bool) -> bool {
        // java: Optional<GuiEventListener> child = getChildAt(event.x(), event.y());
        //   while (child.isPresent() && child.get() instanceof ContainerEventHandler container) child = container.getChildAt(..);
        // The port's screen holds the boxes themselves (a layout holds its widgets by index in the screen's list), so
        // the non-container child under the mouse is a screen child: the first one that is not a container widget.
        let __check: Option<Checkbox> = compat::read(__this, |__t| {
            __t.children().iter().find(|__w| __w.visible && __w.is_mouse_over(event.x(), event.y()) && !matches!(&__w.kind, WidgetKind::Mod(_))).and_then(|__w| if let WidgetKind::Checkbox(__c) = &__w.kind { Some(__c.clone()) } else { None })
        });
        if let Some(check) = &__check {
            let mut finder: ClickableStyleFinder = (ClickableStyleFinder::new(((event.x()) as i32) as f64, ((event.y()) as i32) as f64)).include_insertions(false);
            // java: ((CheckboxAccessor) check).getTextWidget().visitLines(finder);
            {
                use mcrust::net::minecraft::client::string_splitter::WidthProvider as _;
                let __font = compat::font();
                let __splitter = __font.splitter();
                let __advance = |__ch: char, __style: &Style| __splitter.width_provider.get_width(__ch as u32, __style);
                check.visit_lines(&__splitter, &mut |__x: i32, __y: i32, __line: &FormattedText| finder.accept(__x, __y, __line, &__advance));
            }
            let style: Option<Style> = finder.result().cloned();
            if let Some(__click) = style.as_ref().and_then(|__s| __s.get_click_event()) {
                // java: defaultHandleClickEvent(style.getClickEvent(), minecraft, this);
                use mcrust::net::minecraft::client::gui::screens::screen::{default_handle_click_event, ClickEventOutcome};
                let (__links, __prompt) = { let __o = compat::minecraft().options.borrow(); (mcrust::net::minecraft::client::options::chat_links(&__o), mcrust::net::minecraft::client::options::chat_links_prompt(&__o)) };
                match default_handle_click_event(__click, __links, __prompt) {
                    // `clickUrlAction`: the confirm screen over this one, or the link opened outright
                    ClickEventOutcome::OpenUrl { uri, prompt } => {
                        if prompt {
                            compat::confirm_link_now(&uri);
                        } else {
                            mcrust::net::minecraft::util::util::open_uri(&uri);
                        }
                    }
                    // rust: port gap - the other outcomes (`CopyToClipboard`, `OpenFile`, `SuggestCommand`) are run by the
                    // game's screen pass, which a mod screen's override cannot hand them to (GAPS_N #4)
                    _ => {}
                }
                return true;
            }
        }
        return false;
    }

    /// AbstractScrollArea.mouseScrolled (the `ContentContainer` under the mouse): `setScrollAmount(scrollAmount() -
    /// scrollY * scrollRate())`, which moves the content.
    pub fn mouse_scrolled(__this: &compat::Shared<Self>, x: f64, y: f64, scroll_y: f64) -> bool {
        let Some((container, _)) = compat::read(__this, |__t| __t.content_container.clone()) else { return false; };
        let over = compat::read(&container, |__c| __c.base.visible && __c.base.is_mouse_over(x, y));
        if !over {
            return false;
        }
        compat::write(&container, |__c| { let __h = __c.content_height; __c.area.mouse_scrolled(__h, scroll_y); });
        ChatRuleConfigScreen::place_content(__this);
        true
    }

    /// java: @Override public void onClose()
    pub fn on_close(__this: &compat::Shared<Self>) {
        ChatRuleConfigScreen::save(__this);
        // java: minecraft.gui.setScreen(parent);
        mcrust::mcrust::modding::game::set_screen_to_parent();
    }

    /// java: private void save()
    pub fn save(__this: &compat::Shared<Self>) {
        // java: ChatRulesHandler.CHAT_RULE_LIST.getData().set(chatRuleIndex, chatRule);
        let (index, rule) = compat::read(__this, |__t| (__t.chat_rule_index as usize, __t.chat_rule.clone()));
        let list = chat_rules_handler::CHAT_RULE_LIST.get().clone();
        let mut data = list.lock().borrow().get_data();
        if index < data.len() {
            data[index] = rule;
        } else {
            panic!("IndexOutOfBoundsException: Index {} out of bounds for length {}", index, data.len());
        }
        list.lock().borrow_mut().set_data(data);
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

    /// java: Screen.tick (the base body) - and a `recreateLayout()` asked for since the last call
    fn tick(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        self.lend(screen, |__h| ChatRuleConfigScreen::run_requested_recreate(__h))
    }

    /// java: Screen.keyPressed (the base body)
    fn key_pressed(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, event: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        self.lend(screen, |__h| ChatRuleConfigScreen::run_requested_recreate(__h));
        if event.is_escape() && self.should_close_on_esc() {
            self.on_close();
            return true;
        }
        false
    }

    /// java: @Override public boolean mouseClicked(final MouseButtonEvent event, final boolean doubleClick)
    fn mouse_clicked(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, x: f64, y: f64, button: i32) -> bool {
        self.lend(screen, |__h| { ChatRuleConfigScreen::run_requested_recreate(__h); ChatRuleConfigScreen::mouse_clicked(__h, &MouseButtonEvent::new(x, y, MouseButtonInfo::new(button, 0)), false) })
    }

    /// ContainerEventHandler.mouseScrolled - the `ContentContainer` under the mouse scrolls
    fn mouse_scrolled(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, x: f64, y: f64, _scroll_x: f64, scroll_y: f64) -> bool {
        self.lend(screen, |__h| { ChatRuleConfigScreen::run_requested_recreate(__h); ChatRuleConfigScreen::mouse_scrolled(__h, x, y, scroll_y) })
    }

    /// java: @Override public void onClose()
    fn on_close(&mut self) {
        self.call(|__h| ChatRuleConfigScreen::on_close(__h))
    }

}

/// java: private class ToastIconPreview extends AbstractWidget
pub struct ToastIconPreview {
    /// java: extends net.minecraft.client.gui.components.AbstractWidget
    pub base: AbstractWidget,
    /// java: private ItemStack stack = ItemStack.EMPTY;
    pub stack: mcrust::net::minecraft::world::item::item_stack::ItemStack,
    /// java: private final EditBox input; - the screen's item input (set once it joined the screen)
    pub input: Option<compat::WidgetRef>,
    /// java: ChatRuleConfigScreen.this
    pub outer: compat::WeakShared<ChatRuleConfigScreen>,
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
    pub fn new(outer: compat::WeakShared<ChatRuleConfigScreen>) -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            base: AbstractWidget::new(0, 0, 16, 16, (&component::empty().get_string()).to_string(), WidgetKind::PlainText, OnPress::Inert),
            self_ref: std::sync::Weak::new(),
            stack: mcrust::net::minecraft::world::item::item_stack::empty(),
            input: None,
            outer,
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        { let (__q0,) = (Tooltip::create_text(FormattedText::from(component::translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.toast.iconPreview.@Tooltip"))),); __this.lock().borrow_mut().set_tooltip(__q0) };
        __this
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_widget_render_state(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, _mouse_x: i32, _mouse_y: i32, _a: f32) {
        let (__stack, __x, __y) = compat::read(__this, |__t| (__t.stack.clone(), __t.get_x(), __t.get_y()));
        graphics.fake_item(&__stack, __x, __y);
    }

    /// java: @Override public void setX(int x)
    pub fn set_x(__this: &compat::Shared<Self>, x: i32) {
        __this.lock().borrow_mut().base.set_x(x);
    }

    /// java: @Override public void onClick(MouseButtonEvent click, boolean doubled)
    pub fn on_click(__this: &compat::Shared<Self>, _click: &MouseButtonEvent, _doubled: bool) {
        // java: super.onClick(click, doubled); - `AbstractWidget.onClick` is empty
        let Some(outer) = compat::read(__this, |__t| __t.outer.upgrade()) else { return; };
        let Some(input) = compat::read(__this, |__t| __t.input.clone()) else { return; };
        let __outer = compat::RenderThread::new(std::sync::Arc::downgrade(&outer));
        let __input = compat::RenderThread::new(input);
        // java: minecraft.gui.setScreen(new ItemSelectionPopup(ChatRuleConfigScreen.this, item -> {..}));
        let popup = ItemSelectionPopup::new(mcrust::net::minecraft::client::gui::screens::screen::Screen::new(String::new()), Arc::new(move |item: Option<mcrust::net::minecraft::world::item::item_stack::ItemStack>| {
            // java: if (item == null) return;
            let Some(mut item) = item else { return; };
            let Some(outer) = __outer.get().upgrade() else { return; };
            // java: input.setValue(getItemString(item));
            let value = ChatRuleConfigScreen::get_item_string(&mut item);
            let mut input = __input.get().clone();
            compat::write(&outer, |__t| { let __s = &mut *__t; __s.__widgets.with(&mut __s.base, &mut input, |__w| compat::set_edit_box_value(__w, &value)) });
        }));
        mcrust::mcrust::modding::game::set_screen_keeping_parent(Box::new(compat::ScreenAdapter::new(popup)));
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    pub fn update_widget_narration(_this: &compat::Shared<Self>, _builder: &dyn NarrationElementOutput) {
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

/// java: private class SoundSearchMenu extends AbstractWidget
pub struct SoundSearchMenu {
    /// java: extends net.minecraft.client.gui.components.AbstractWidget
    pub base: AbstractWidget,
    /// java: ChatRuleConfigScreen.this
    pub outer: compat::WeakShared<ChatRuleConfigScreen>,
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
    pub fn new(outer: compat::WeakShared<ChatRuleConfigScreen>) -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            base: AbstractWidget::new(0, 0, 16, 16, (&component::empty().get_string()).to_string(), WidgetKind::PlainText, OnPress::Inert),
            outer,
            self_ref: std::sync::Weak::new(),
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        { let (__q0,) = (Tooltip::create_text(FormattedText::from(component::translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.sounds.search.@Tooltip"))),); __this.lock().borrow_mut().set_tooltip(__q0) };
        __this
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_widget_render_state(__this: &compat::Shared<Self>, context: &WidgetRenderContext<'_>, _mouse_x: i32, _mouse_y: i32, _a: f32) {
        // java: graphics.blitSprite(RenderPipelines.GUI_TEXTURED, SEARCH_ICON_TEXTURE, this.getX(), this.getY(), this.getWidth(), this.getHeight());
        let (__x, __y, __w, __h) = compat::read(__this, |__t| (__t.get_x(), __t.get_y(), __t.get_width(), __t.get_height()));
        gui_graphics_extractor::blit_sprite_identifier(context.gl, context.minecraft, mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), SEARCH_ICON_TEXTURE.get(), __x, __y, __w, __h);
    }

    /// java: @Override public void setX(int x)
    pub fn set_x(__this: &compat::Shared<Self>, x: i32) {
        __this.lock().borrow_mut().base.set_x(x);
    }

    /// java: @Override public void onClick(MouseButtonEvent click, boolean doubled)
    pub fn on_click(__this: &compat::Shared<Self>, _click: &MouseButtonEvent, _doubled: bool) {
        // java: super.onClick(click, doubled); - `AbstractWidget.onClick` is empty
        let Some(outer) = compat::read(__this, |__t| __t.outer.upgrade()) else { return; };
        let __outer = compat::RenderThread::new(std::sync::Arc::downgrade(&outer));
        // java: minecraft.gui.setScreen(new SoundSelectionPopup(ChatRuleConfigScreen.this, sound -> {..}));
        let popup = SoundSelectionPopup::new(mcrust::net::minecraft::client::gui::screens::screen::Screen::new(String::new()), Arc::new(move |sound: Option<SoundEvent>| {
            let Some(outer) = __outer.get().upgrade() else { return; };
            if let Some(sound) = sound {
                // java: chatRule.setCustomSound(sound);
                ChatRule::set_custom_sound(&ChatRuleConfigScreen::rule(&outer), Some(sound));
                // java: soundButton.setMessage(Component.translatable("..sounds.custom").withStyle(ChatFormatting.YELLOW));
                let Some(mut sound_button) = compat::read(&outer, |__t| __t.sound_button.clone()) else { return; };
                let message = FormattedText::from(component::translatable("skyblocker.config.chat.chatRules.screen.ruleScreen.sounds.custom").with_style_format(ChatFormatting::Yellow));
                compat::write(&outer, |__t| { let __s = &mut *__t; __s.__widgets.with(&mut __s.base, &mut sound_button, |__w| __w.set_message_text(message.clone())) });
            }
        }));
        mcrust::mcrust::modding::game::set_screen_keeping_parent(Box::new(compat::ScreenAdapter::new(popup)));
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    pub fn update_widget_narration(_this: &compat::Shared<Self>, _builder: &dyn NarrationElementOutput) {
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
        self.lend_ref(widget, |__h| SoundSearchMenu::extract_widget_render_state(__h, context, mouse_x, mouse_y, a))
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

/// java: private class ContentContainer extends AbstractContainerWidget implements Layout
///
/// The content grid and its widgets belong to the screen here (a layout holds its widgets by index in the screen's
/// list, and the game dispatches input to them as screen children); the container keeps `AbstractScrollArea`'s
/// scroll state, its box and the content's height, which the screen brings up to date in `repositionElements`.
pub struct ContentContainer {
    /// java: extends net.minecraft.client.gui.components.AbstractContainerWidget
    pub base: AbstractContainerWidget,
    /// java: private final List<AbstractWidget> children = new ArrayList<>(); - the screen's indices of the content's
    /// widgets (`content.visitWidgets(children::add)`)
    pub children: Vec<usize>,
    /// AbstractScrollArea's scroll state (`AbstractScrollArea.defaultSettings(8)`, `scrollRate()` 10)
    pub area: ScrollArea,
    /// java: @Override protected int contentHeight() - `content.getHeight()`, as of the last arrangement
    pub content_height: i32,
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
        // java: super(0, 0, 0, 0, Component.empty(), AbstractScrollArea.defaultSettings(8));
        let mut settings = ScrollbarSettings::default_settings(8.0);
        // java: @Override protected double scrollRate() { return 10; }
        settings.scroll_rate = ContentContainer::scroll_rate();
        let __this: compat::Shared<Self> = compat::shared(Self {
            base: AbstractContainerWidget::new_with_scrollbar_settings(0, 0, 0, 0, FormattedText::from(component::empty()), ScrollbarSettings::default_settings(8.0)),
            children: Vec::new(),
            area: ScrollArea::new(0, 0, settings),
            content_height: 0,
            self_ref: std::sync::Weak::new(),
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        __this
    }

    /// java: @Override public List<? extends GuiEventListener> children() - the `children` list itself
    pub fn children(__this: &compat::Shared<Self>) -> Vec<usize> {
        return compat::read(__this, |__t| __t.children.clone());
    }

    /// java: @Override protected int contentHeight()
    pub fn content_height(__this: &compat::Shared<Self>) -> i32 {
        // java: return content.getHeight();
        compat::read(__this, |__t| __t.content_height)
    }

    /// java: @Override protected double scrollRate()
    pub fn scroll_rate() -> f64 {
        return 10.0;
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    ///
    /// (the children are the screen's widgets, which the game draws; see `ChatRuleConfigScreen::place_content`)
    pub fn extract_widget_render_state(__this: &compat::Shared<Self>, context: &WidgetRenderContext<'_>, _mouse_x: i32, _mouse_y: i32, _a: f32) {
        // java: this.extractScrollbar(graphics, mouseX, mouseY);
        let (area, content_height, scroll_bar_x) = compat::read(__this, |__t| (__t.area, __t.content_height, __t.base.get_right() - __t.area.scrollbar_width()));
        compat::extract_scrollbar(context, &area, content_height, scroll_bar_x);
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    pub fn update_widget_narration(_this: &compat::Shared<Self>, _builder: &dyn NarrationElementOutput) {
    }

    /// java: @Override public void removeChildren()
    pub fn remove_children(__this: &compat::Shared<Self>) {
        __this.lock().borrow_mut().children.clear();
    }

}

/// ContentContainer's container half (`super`'s fields)
impl ContainerWidgetHalf for ContentContainer {
    fn container_half(&mut self) -> &mut AbstractContainerWidget {
        &mut self.base
    }
}

/// ContentContainer as the game's widget (`this.layout.addToContents(new ContentContainer())`)
impl ModWidget for ContainerWidgetAdapter<ContentContainer> {
    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_widget_render_state(&mut self, _widget: &AbstractWidget, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.lend(|__h| ContentContainer::extract_widget_render_state(__h, context, mouse_x, mouse_y, a))
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    fn update_widget_narration(&mut self, _widget: &AbstractWidget, output: &mut dyn NarrationElementOutput) {
        self.lend(|__h| ContentContainer::update_widget_narration(__h, &*output))
    }

    fn container_widget_mut(&mut self) -> Option<&mut AbstractContainerWidget> {
        Some(self.part_mut())
    }
}

/// java: `EditBox.TextFormatter` - `(String text, int start) -> FormattedCharSequence` (null: no formatting). The
/// port's `TextFormatter` is a closed enum of the game's own two, so the mod's formatter is this function.
pub type ModTextFormatter = Arc<dyn Fn(&str, i32) -> Option<mcrust::net::minecraft::util::formatted_char_sequence::Seq>>;

/// java: the `visitor -> {..}` sequence `createRenderTextProvider` answers: the box's whole text, re-styled by its
/// `§`/`&` codes, of which the characters in `[start, start + s.length())` are handed to the visitor.
struct RenderTextSequence {
    full_text_supplier: compat::RenderThread<Arc<dyn Fn() -> String>>,
    length: i32,
    start: i32,
}

impl mcrust::net::minecraft::util::formatted_char_sequence::FormattedCharSequence for RenderTextSequence {
    fn accept(&self, visitor: &mut dyn mcrust::net::minecraft::util::formatted_char_sink::FormattedCharSink) -> bool {
        let mut full_text: String = (self.full_text_supplier.get())();
        let mut prefix: char = (if full_text.contains("§") { '§' } else { '&' });
        let mut style: Style = mcrust::net::minecraft::network::chat::style::Style::EMPTY;
        for i in 0..compat::str_len(&full_text) {
            if compat::char_at(&full_text, i) == prefix {
                if i.wrapping_add(1) < compat::str_len(&full_text) {
                    // java: ChatFormatting formatting = ChatFormatting.getByCode(..); if (formatting != null) {..}
                    if let Some(formatting) = chat_formatting::get_by_code(compat::char_at(&full_text, i.wrapping_add(1))) {
                        style = (if formatting == ChatFormatting::Reset { mcrust::net::minecraft::network::chat::style::Style::EMPTY } else { style.apply_legacy_format(formatting) });
                    }
                }
            }
            let mut code_point: i32 = (compat::char_at(&full_text, i) as i32);
            if i >= self.start && i < self.start.wrapping_add(self.length) {
                visitor.accept(i, &style, code_point as u32);
            }
        }
        return true;
    }
}

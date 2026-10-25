//! Ported from de.hysky.skyblocker.skyblock.tabhud.config.list.WidgetsListScreen by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::tabhud::config::list::entries::slot::boolean_slot_entry::BooleanSlotEntry;
use crate::de::hysky::skyblocker::skyblock::tabhud::config::list::entries::slot::default_slot_entry::DefaultSlotEntry;
use crate::de::hysky::skyblocker::skyblock::tabhud::config::list::entries::slot::editable_slot_entry::EditableSlotEntry;
use crate::de::hysky::skyblocker::skyblock::tabhud::config::list::entries::slot::widget_slot_entry::WidgetSlotEntry;
use crate::de::hysky::skyblocker::skyblock::tabhud::config::list::entries::slot::widgets_list_slot_entry::WidgetsListSlotEntry;
use crate::de::hysky::skyblocker::skyblock::tabhud::config::list::entries::widgets_list_entry;
use crate::de::hysky::skyblocker::skyblock::tabhud::config::list::widgets_element_list::WidgetsElementList;
use crate::de::hysky::skyblocker::utils::container_utils::ContainerUtils;
use crate::de::hysky::skyblocker::utils::item_utils::ItemUtils;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler::Scheduler;
use mcrust::java::text::decimal_format_symbols::Locale;
use mcrust::mcrust::modding::screens::ModScreen;
use mcrust::mcrust::modding::screens::ScreenRenderContext;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::PressLambda;
use mcrust::net::minecraft::client::gui::components::button;
use mcrust::net::minecraft::client::gui::components::component_render_utils;
use mcrust::net::minecraft::client::gui::components::multi_line_text_widget::MultiLineTextWidget;
use mcrust::net::minecraft::client::gui::components::string_widget;
use mcrust::net::minecraft::client::gui::components::toasts::system_toast::SystemToastId;
use mcrust::net::minecraft::client::gui::components::tooltip::Tooltip;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::layouts::frame_layout::FrameLayout;
use mcrust::net::minecraft::client::gui::layouts::header_and_footer_layout::HeaderAndFooterLayout;
use mcrust::net::minecraft::client::gui::layouts::layout_element::LayoutElement;
use mcrust::net::minecraft::client::gui::layouts::linear_layout::LinearLayout;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::client::input::mouse_button_info::MouseButtonInfo;
use mcrust::net::minecraft::client::minecraft;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::util::formatted_char_sequence::FormattedCharSequence;
use mcrust::net::minecraft::util::formatted_char_sequence::Seq;
use mcrust::net::minecraft::world::inventory::abstract_container_menu::AbstractContainerMenu;
use mcrust::net::minecraft::world::inventory::container_input::ContainerInput;
use mcrust::net::minecraft::world::item::items;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;
use mcrust::net::minecraft::client::gui::layouts::abstract_layout::AbstractLayout as _;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: public static boolean overrideWidgetsScreen = false;
pub static OVERRIDE_WIDGETS_SCREEN: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));
/// java: public static final SystemToast.SystemToastId SYSTEM_TOAST_ID = new SystemToast.SystemToastId(1_000);
pub static SYSTEM_TOAST_ID: compat::MainThread<SystemToastId> = compat::MainThread::new(|| SystemToastId::with_display_time(1_000));
/// java: private static final int PADDING = 8;
pub const PADDING: i32 = 8;

/// java: public class WidgetsListScreen extends Screen implements ContainerListener
pub struct WidgetsListScreen {
    /// java: extends net.minecraft.client.gui.screens.Screen
    pub base: mcrust::net::minecraft::client::gui::screens::screen::Screen,
    /// java: private WidgetsElementList widgetsElementList;
    pub widgets_element_list: Option<compat::WidgetRef>,
    /// java: private Button back;
    pub back: Option<compat::WidgetRef>,
    /// java: private Button previousPage;
    pub previous_page: Option<compat::WidgetRef>,
    /// java: private Button nextPage;
    pub next_page: Option<compat::WidgetRef>,
    /// java: private Button thirdColumnButton;
    pub third_column_button: Option<compat::WidgetRef>,
    /// java: private Button resetButton;
    pub reset_button: Option<compat::WidgetRef>,
    /// java: private StringWidget waitingForServerText;
    pub waiting_for_server_text: Option<compat::WidgetRef>,
    /// java: private MultiLineTextWidget infoText;
    /// (a widget of this screen: the header layout holds it by its index)
    pub info_text: Option<compat::WidgetRef>,
    /// java: private final HeaderAndFooterLayout layout = new HeaderAndFooterLayout(this, 25);
    pub layout: Option<HeaderAndFooterLayout>,
    /// java: private final FrameLayout headerLayout = new FrameLayout(0, 25);
    pub header_layout: FrameLayout,
    /// java: @SuppressWarnings("unchecked") private final List<Component>[] previewColumns = new List[3];
    pub preview_columns: Vec<Vec<MutableComponent>>,
    /// java: private final Int2ObjectMap<WidgetsListSlotEntry> entries = new Int2ObjectOpenHashMap<>();
    pub entries: std::collections::HashMap<i32, compat::Shared<dyn WidgetsListSlotEntry>>,
    /// java: private ChestMenu handler;
    pub handler: Option<compat::LiveChestMenu>,
    /// java: private boolean waitingForServer = false;
    pub waiting_for_server: bool,
    /// java: private int resetSlotId = -1;
    pub reset_slot_id: i32,
    /// java: private boolean shouldResetScroll = false;
    pub should_reset_scroll: bool,
    /// java: private String titleLowercase;
    pub title_lowercase: String,
    /// java: private boolean overflowing = false;
    pub overflowing: bool,
    /// java: private boolean previewVisible = false;
    pub preview_visible: bool,
    /// java: private boolean thirdColumnEnabled = false;
    pub third_column_enabled: bool,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
    /// the screen's widgets as this object last saw them (see `compat::WidgetCache`)
    pub __widgets: compat::WidgetCache,
    /// `handler.addSlotListener(this)`: the menu's `slotChanged` calls, queued for this screen's next own call
    /// (`compat::SlotChangeQueue` - the menu calls its listeners inside the game's hold of the player); `None` until
    /// the menu is the player's open one, where the listener is added
    pub slot_changes: Option<compat::SlotChangeQueue>,
}

impl std::ops::Deref for WidgetsListScreen {
    type Target = mcrust::net::minecraft::client::gui::screens::screen::Screen;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for WidgetsListScreen {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl WidgetsListScreen {
    /// java: public WidgetsListScreen(ChestMenu handler, Component name)
    pub fn new(mut handler: compat::LiveChestMenu, mut name: MutableComponent) -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            base: mcrust::net::minecraft::client::gui::screens::screen::Screen::new(name.get_string()),
            self_ref: std::sync::Weak::new(),
            __widgets: Default::default(),
            widgets_element_list: None,
            back: None,
            previous_page: None,
            next_page: None,
            third_column_button: None,
            reset_button: None,
            waiting_for_server_text: None,
            info_text: None,
            layout: None,
            header_layout: FrameLayout::of(0, 25),
            preview_columns: (0..(3) as usize).map(|_| Default::default()).collect::<Vec<_>>(),
            entries: std::collections::HashMap::new(),
            handler: None,
            waiting_for_server: false,
            reset_slot_id: -1,
            should_reset_scroll: false,
            title_lowercase: Default::default(),
            overflowing: false,
            preview_visible: false,
            third_column_enabled: false,
            slot_changes: None,
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        { let __v = Some(HeaderAndFooterLayout::with_height(0, 0, 25)); compat::write(&__this, |__t| __t.layout = __v); }
        { let __v = (Some(WidgetsElementList::new(__this.clone(), compat::minecraft(), 0, 0, 0))).map(|__o| compat::WidgetRef::value(compat::attach_list(__o))); compat::write(&__this, |__t| { let __s = &mut *__t; __s.widgets_element_list = __v; }); }
        { let __v = Some(handler); compat::write(&__this, |__t| __t.handler = __v); }
        { let __v = name.get_string().to_lowercase(); compat::write(&__this, |__t| __t.title_lowercase = __v); }
        // java: handler.addSlotListener(this);
        WidgetsListScreen::listen_to_handler(&__this);
        __this
    }

    /// java: @Init public static void initCommands()
    pub fn init_commands() {
        client_command_registration_callback::EVENT.register(Arc::new(
            |dispatcher: &mut client_command_registration_callback::ClientCommandDispatcher| {
                //   java: dispatcher.register(ClientCommands.literal(SkyblockerMod.NAMESPACE).then(ClientCommands.literal("hypixelWidgets").executes(_ -> {
                //   java:     if (Utils.isOnSkyblock()) { overrideWidgetsScreen = true; MessageScheduler.INSTANCE.sendMessageAfterCooldown("/widgets", true); } return Command.SINGLE_SUCCESS; })));
                use mcrust::com::mojang::brigadier::builder::argument_builder::ArgumentBuilder as _;
                let mut hypixel_widgets = client_command_registration_callback::literal("hypixelWidgets");
                hypixel_widgets.executes(Arc::new(|_: &mcrust::com::mojang::brigadier::context::command_context::CommandContext<client_command_registration_callback::FabricClientCommandSource>| {
                    if crate::de::hysky::skyblocker::utils::utils::Utils::is_on_skyblock() {
                        compat::set_state(&OVERRIDE_WIDGETS_SCREEN, true);
                        crate::de::hysky::skyblocker::utils::scheduler::message_scheduler::MessageScheduler::send_message_after_cooldown(crate::de::hysky::skyblocker::utils::scheduler::message_scheduler::INSTANCE.get_mut(), "/widgets", true);
                    }
                    Ok(1)
                }));
                let mut root = client_command_registration_callback::literal(crate::entry::skyblocker_mod::NAMESPACE);
                root.then(&hypixel_widgets);
                dispatcher.dispatcher().register(&root);
            },
        ));
    }

    /// java: public ObjectSet<Int2ObjectMap.Entry<WidgetsListSlotEntry>> getEntries()
    /// (the map's entry set, in its iteration order: a row object has no hash of its own)
    pub fn get_entries(__this: &compat::Shared<Self>) -> Vec<(i32, compat::Shared<dyn WidgetsListSlotEntry>)> {
        return __this.lock().borrow().entries.iter().map(|(k, v)| (k.clone(), v.clone())).collect::<Vec<_>>();
    }

    /// java: @Override protected void init()
    pub fn init(__this: &compat::Shared<Self>) {
        ();
        let __r0 = compat::read(__this, |__t| __t.header_layout.clone());
        { let (__q0,) = (LayoutElement::Frame(__r0),); __this.lock().borrow_mut().layout.as_mut().unwrap().add_to_header(__q0) };
        { let __v = (Some(WidgetsElementList::new(__this.clone(), compat::minecraft(), 0, 0, 0))).map(|__o| compat::WidgetRef::value(compat::attach_list(__o))); compat::write(__this, |__t| { let __s = &mut *__t; __s.widgets_element_list = __v; }); }
        { let (__q0,) = ({ let (__i, __ww, __wh) = compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.widgets_element_list.as_mut().unwrap(); __s.__widgets.add_to_layout(&mut __s.base, __r) }); LayoutElement::widget(__i, __ww, __wh) },); __this.lock().borrow_mut().layout.as_mut().unwrap().add_to_contents(__q0) };
        let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        { let __v = (Some((button::builder(component::translatable("gui.back").get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { let Some(__this_1) = __this_w1.get().upgrade() else { return Default::default(); }; compat::lend_screen(&__this_1, __screen, || { let mut __pidx = compat::WidgetRef::Screen(__index); {
            WidgetsListScreen::click_and_wait_for_server((&__this_1), 48, 0);
            WidgetsListScreen::reset_scroll_on_load((&__this_1));
        } }) })))).size(60, 15).build())).map(compat::WidgetRef::value); compat::write(__this, |__t| { let __s = &mut *__t; __s.back = __v; }); }
        { let (__q0, __q1,) = ({ let (__i, __ww, __wh) = compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.back.as_mut().unwrap(); __s.__widgets.add_to_layout(&mut __s.base, __r) }); LayoutElement::widget(__i, __ww, __wh) }, &mut |l: &mut mcrust::net::minecraft::client::gui::layouts::layout_settings::LayoutSettings| { *l = l.align_horizontally_left().padding_left(PADDING); },); __this.lock().borrow_mut().header_layout.add_child_adjusted(__q0, __q1) };
        let __this_w2 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        { let __v = (Some((button::builder((component::literal("if you are seeing this for more than a second, something went very wrong! (no i will not translate this aaron)")).get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { let Some(__this_2) = __this_w2.get().upgrade() else { return Default::default(); }; compat::lend_screen(&__this_2, __screen, || { let mut __pidx = compat::WidgetRef::Screen(__index); { WidgetsListScreen::click_and_wait_for_server((&__this_2), 50, 0); } }) })))).size(120, 15).build())).map(compat::WidgetRef::value); compat::write(__this, |__t| { let __s = &mut *__t; __s.third_column_button = __v; }); }
        { let (__q0, __q1,) = ({ let (__i, __ww, __wh) = compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.third_column_button.as_mut().unwrap(); __s.__widgets.add_to_layout(&mut __s.base, __r) }); LayoutElement::widget(__i, __ww, __wh) }, &mut |l: &mut mcrust::net::minecraft::client::gui::layouts::layout_settings::LayoutSettings| { *l = l.align_horizontally_right().padding_right(PADDING); },); __this.lock().borrow_mut().header_layout.add_child_adjusted(__q0, __q1) };
        // java: infoText = headerLayout.addChild(new MultiLineTextWidget(Component.empty(), font).setCentered(true), l -> l.paddingVertical(4));
        {
            let __info = MultiLineTextWidget::new(0, 0, mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::empty())).set_centered(true);
            let (__iw, __ih) = { let __font = compat::font(); let __splitter = __font.splitter(); (__info.get_width(&__splitter), __info.get_height(&__splitter)) };
            let __info_widget = AbstractWidget::new(0, 0, __iw, __ih, String::new(), mcrust::net::minecraft::client::gui::components::abstract_widget::WidgetKind::PlainMultiLineText(Box::new(__info)), OnPress::Inert);
            let __v = Some(compat::WidgetRef::value(__info_widget));
            compat::write(__this, |__t| __t.info_text = __v);
        }
        { let (__q0, __q1,) = ({ let (__i, __ww, __wh) = compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.info_text.as_mut().unwrap(); __s.__widgets.add_to_layout(&mut __s.base, __r) }); LayoutElement::widget(__i, __ww, __wh) }, &mut |l: &mut mcrust::net::minecraft::client::gui::layouts::layout_settings::LayoutSettings| { *l = l.padding_vertical(4); },); __this.lock().borrow_mut().header_layout.add_child_adjusted(__q0, __q1) };
        let __wa1 = Tooltip::create((component::translatable("skyblocker.config.hud.widgetsList.thirdColumn.@Tooltip")).get_string());
        compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.third_column_button.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.set_tooltip(__wa1.clone())) });
        let mut footer: LinearLayout = LinearLayout::horizontal().spacing(10);
        let __this_w3 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        { let __v = (Some((button::builder(component::translatable("book.page_button.previous").get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { let Some(__this_3) = __this_w3.get().upgrade() else { return Default::default(); }; compat::lend_screen(&__this_3, __screen, || { let mut __pidx = compat::WidgetRef::Screen(__index); { WidgetsListScreen::click_and_wait_for_server((&__this_3), 45, 0); } }) })))).size(100, 15).build())).map(compat::WidgetRef::value); compat::write(__this, |__t| { let __s = &mut *__t; __s.previous_page = __v; }); }
        footer.add_child({ let (__i, __ww, __wh) = compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.previous_page.as_mut().unwrap(); __s.__widgets.add_to_layout(&mut __s.base, __r) }); LayoutElement::widget(__i, __ww, __wh) });
        let __this_w4 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        { let __v = (Some((button::builder(component::translatable("book.page_button.next").get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { let Some(__this_4) = __this_w4.get().upgrade() else { return Default::default(); }; compat::lend_screen(&__this_4, __screen, || { let mut __pidx = compat::WidgetRef::Screen(__index); { WidgetsListScreen::click_and_wait_for_server((&__this_4), 53, 0); } }) })))).size(100, 15).build())).map(compat::WidgetRef::value); compat::write(__this, |__t| { let __s = &mut *__t; __s.next_page = __v; }); }
        footer.add_child({ let (__i, __ww, __wh) = compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.next_page.as_mut().unwrap(); __s.__widgets.add_to_layout(&mut __s.base, __r) }); LayoutElement::widget(__i, __ww, __wh) });
        { let (__q0,) = (LayoutElement::Grid(footer.clone().wrapped),); __this.lock().borrow_mut().layout.as_mut().unwrap().add_to_footer(__q0) };
        { let __v = (Some(compat::string_widget(0, 0, -1, 9, component::translatable("skyblocker.config.hud.widgetsList.waitingForServer")))).map(compat::WidgetRef::value); compat::write(__this, |__t| { let __s = &mut *__t; __s.waiting_for_server_text = __v; }); }
        let __wa2 = compat::font().width_of_cached(&mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.waiting_for_server_text.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| compat::component_of(__w.get_message_component())) })));
        compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.waiting_for_server_text.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.set_width(__wa2)) });
        // java: resetButton = layout.addToFooter(Button.builder(Component.translatable("skyblocker.config.hud.widgetsList.reset"), _ -> {
        //           if (resetSlotId == -1) return; clickAndWaitForServer(resetSlotId, 0); }).size(60, 15).build(), l -> l.alignHorizontallyRight().paddingRight(PADDING));
        let __this_w5 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        { let __v = (Some((button::builder(component::translatable("skyblocker.config.hud.widgetsList.reset").get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { let Some(__this_5) = __this_w5.get().upgrade() else { return Default::default(); }; compat::lend_screen(&__this_5, __screen, || { let mut __pidx = compat::WidgetRef::Screen(__index); {
            if compat::read((&__this_5), |__t| __t.reset_slot_id) == -1 {
                return;
            }
            WidgetsListScreen::click_and_wait_for_server((&__this_5), compat::read((&__this_5), |__t| __t.reset_slot_id), 0);
        } }) })))).size(60, 15).build())).map(compat::WidgetRef::value); compat::write(__this, |__t| { let __s = &mut *__t; __s.reset_button = __v; }); }
        {
            // (`l -> ..` applies to the footer frame's own child settings, `newChildLayoutSettings()`)
            let __settings: mcrust::net::minecraft::client::gui::layouts::layout_settings::LayoutSettings = __this.lock().borrow().layout.as_ref().unwrap().footer_frame.new_child_layout_settings().align_horizontally_right().padding_right(PADDING);
            let __q0 = { let (__i, __ww, __wh) = compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.reset_button.as_mut().unwrap(); __s.__widgets.add_to_layout(&mut __s.base, __r) }); LayoutElement::widget(__i, __ww, __wh) };
            __this.lock().borrow_mut().layout.as_mut().unwrap().add_to_footer_with(__q0, __settings);
        }
        { let __vw = __this.lock().borrow().layout.as_ref().unwrap().visit_widgets(); for (__i, _, _) in __vw { compat::write(__this, |__t| __t.mark_renderable(__i)); } };
        compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.waiting_for_server_text.as_mut().unwrap(); __s.__widgets.add(&mut __s.base, __r, true) });
        WidgetsListScreen::reposition_elements(__this);
    }

    /// java: @Override protected void repositionElements()
    pub fn reposition_elements(__this: &compat::Shared<Self>) {
        // java: infoText.setMaxWidth(width - thirdColumnButton.getWidth() * 2 - PADDING * 3);
        let __max_width: i32 = ((compat::read(__this, |__t| __t.width)).wrapping_sub((compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.third_column_button.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.get_width()) })).wrapping_mul(2))).wrapping_sub(PADDING.wrapping_mul(3));
        WidgetsListScreen::with_info_text(__this, |__m| __m.max_width = Some(__max_width));
        let __r0 = compat::read(__this, |__t| __t.width);
        { let (__q0,) = (__r0,); __this.lock().borrow_mut().header_layout.set_min_width(__q0) };
        { __this.lock().borrow_mut().header_layout.arrange_elements(); let mut __vw = Vec::new(); __this.lock().borrow().header_layout.visit_widgets(&mut __vw); for (__i, __x, __y) in __vw { compat::write(__this, |__t| __t.widget_mut(__i).set_position(__x, __y)); } };
        { let (__q0,) = (__this.lock().borrow().header_layout.get_height(),); __this.lock().borrow_mut().layout.as_mut().unwrap().set_header_height(__q0) };
        let __wa1 = compat::read(__this, |__t| __t.width);
        let __wa2 = compat::read(__this, |__t| __t.layout.clone());
        compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.widgets_element_list.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.mod_widget().and_then(|__ms| __ms.with::<mcrust::mcrust::modding::widgets::ModSelectionList, _>(|__l| __l.list.update_size(__wa1, __wa2.as_ref().unwrap()))).expect("a mod list reached from inside its own method")) });
        let __r0 = compat::read(__this, |__t| (__t.width, __t.height));
        { let (__sw, __sh) = __r0; __this.lock().borrow_mut().layout.as_mut().unwrap().screen_width = __sw; __this.lock().borrow_mut().layout.as_mut().unwrap().screen_height = __sh; __this.lock().borrow_mut().layout.as_mut().unwrap().arrange_elements(); let __vw = __this.lock().borrow().layout.as_ref().unwrap().visit_widgets(); for (__i, __x, __y) in __vw { compat::write(__this, |__t| __t.widget_mut(__i).set_position(__x, __y)); } };
        // java: waitingForServerText.setPosition(width - waitingForServerText.getWidth() - 5, height - font.lineHeight - 2);
        let __wa3 = ((compat::read(__this, |__t| __t.width)).wrapping_sub(compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.waiting_for_server_text.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.get_width()) }))).wrapping_sub(5);
        let __wa4 = ((compat::read(__this, |__t| __t.height)).wrapping_sub(mcrust::net::minecraft::client::gui::font::LINE_HEIGHT)).wrapping_sub(2);
        compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.waiting_for_server_text.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.set_position(__wa3, __wa4)) });
    }

    /// java: public void resetScrollOnLoad()
    pub fn reset_scroll_on_load(__this: &compat::Shared<Self>) {
        { let __v = true; compat::write(__this, |__t| __t.should_reset_scroll = __v); }
    }

    /// java: public boolean isWaitingForServer()
    pub fn is_waiting_for_server(__this: &compat::Shared<Self>) -> bool {
        return compat::read(__this, |__t| __t.waiting_for_server);
    }

    /// java: public void clickAndWaitForServer(int slot, int button)
    pub fn click_and_wait_for_server(__this: &compat::Shared<Self>, mut slot: i32, mut button: i32) {
        if compat::read(__this, |__t| __t.waiting_for_server) {
            return;
        }
        if false /* java: compat::minecraft().game_mode == null: never null here */ || mcrust::mcrust::modding::game::with_player(|_| ()).is_none() {
            return;
        }
        // java: client.gameMode.handleContainerInput(handler.containerId, slot, ContainerUtils.getContainerClickButton(button), ContainerInput.PICKUP, client.player);
        let __container_id: i32 = compat::read(__this, |__t| __t.handler.as_ref().unwrap().container_id);
        compat::handle_container_input(__container_id, slot, ContainerUtils::get_container_click_button(button), ContainerInput::Pickup);
        { let __v = true; compat::write(__this, |__t| __t.waiting_for_server = __v); }
        { let __v = true; compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.waiting_for_server_text.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.visible = __v) }); }
    }

    /// java: public void shiftClickAndWaitForServer(int slot, int button)
    pub fn shift_click_and_wait_for_server(__this: &compat::Shared<Self>, mut slot: i32, mut button: i32) {
        if compat::read(__this, |__t| __t.waiting_for_server) {
            return;
        }
        if false /* java: compat::minecraft().game_mode == null: never null here */ || mcrust::mcrust::modding::game::with_player(|_| ()).is_none() {
            return;
        }
        // java: client.gameMode.handleContainerInput(handler.containerId, slot, ContainerUtils.getContainerClickButton(button), ContainerInput.QUICK_MOVE, client.player);
        let __container_id: i32 = compat::read(__this, |__t| __t.handler.as_ref().unwrap().container_id);
        compat::handle_container_input(__container_id, slot, ContainerUtils::get_container_click_button(button), ContainerInput::QuickMove);
        let __this_1 = compat::RenderThread::new(__this.clone());
        ({ let __r = scheduler::INSTANCE.get_mut(); let __f = compat::dispatch(__r, |__t| __t.schedule()); __f(__r, Arc::new(move || {
            { let __v = false; compat::write(__this_1.get(), |__t| __t.waiting_for_server = __v); }
            { let __v = false; compat::write(__this_1.get(), |__t| { let __s = &mut *__t; let __r = __s.waiting_for_server_text.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.visible = __v) }); }
        }), 4) });
        { let __v = true; compat::write(__this, |__t| __t.waiting_for_server = __v); }
        { let __v = true; compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.waiting_for_server_text.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.visible = __v) }); }
    }

    /// java: public void updateHandler(ChestMenu newHandler, Component name)
    pub fn update_handler(__this: &compat::Shared<Self>, mut new_handler: compat::LiveChestMenu, name: &MutableComponent) {
        { let __v = name.get_string().to_lowercase(); compat::write(__this, |__t| __t.title_lowercase = __v); }
        // java: handler.removeSlotListener(this); - the port's `removeSlotListener` clears the menu's listeners
        { let __old = compat::read(__this, |__t| __t.handler.clone()); let _ = __old.as_ref().and_then(|__h| __h.with(|__chest| __chest.menu.remove_slot_listener())); }
        compat::write(__this, |__t| __t.slot_changes = None);
        { let __v = Some(new_handler); compat::write(__this, |__t| __t.handler = __v); }
        // java: newHandler.addSlotListener(this);
        WidgetsListScreen::listen_to_handler(__this);
        { let __v = true; compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.back.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.visible = __v) }); }
        __this.lock().borrow_mut().entries.clear();
        compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.widgets_element_list.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.mod_widget().and_then(|__ms| __ms.with::<mcrust::mcrust::modding::widgets::ModSelectionList, _>(|__l| { let __h = &__l.class::<compat::ListClass<WidgetsElementList>>().map(|__c| __c.object().clone()).expect("a mod list without its class"); compat::lend_list(__h, __l, || WidgetsElementList::update_list(__h)) })).expect("a mod list reached from inside its own method")) });
        { let __v = false; compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.reset_button.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.visible = __v) }); }
        if compat::read(__this, |__t| __t.should_reset_scroll) {
            { let __v = false; compat::write(__this, |__t| __t.should_reset_scroll = __v); }
            compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.widgets_element_list.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.mod_widget().and_then(|__ms| __ms.with::<mcrust::mcrust::modding::widgets::ModSelectionList, _>(|__l| __l.list.set_scroll_amount((0) as f64))).expect("a mod list reached from inside its own method")) });
        }
    }

    /// java: @Override public void extractRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_render_state(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        ();
        if minecraft::has_control_down() {
            let mut max: i32 = __this.lock().borrow().preview_columns.iter().cloned().flat_map(|__a0| __a0.into_iter()).map(|__a0| compat::font().width_of_cached(&mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(__a0.clone()))).max().unwrap_or(0);
            if max <= 0 {
                return;
            }
            let mut col_width: i32 = (((__this.lock().borrow().preview_columns.len() as i32).wrapping_mul(max)).min((compat::read(__this, |__t| __t.width)).wrapping_sub(20))).wrapping_div(__this.lock().borrow().preview_columns.len() as i32);
            let mut line_count: i32 = 0;
            for column in compat::read(__this, |__t| __t.preview_columns.clone()) {
                line_count = line_count.max((column.len() as i32));
            }
            let mut column_spacing: i32 = 4;
            let mut total_width: i32 = (col_width.wrapping_mul(__this.lock().borrow().preview_columns.len() as i32)).wrapping_add(column_spacing.wrapping_mul((__this.lock().borrow().preview_columns.len() as i32).wrapping_sub(1)));
            let mut total_height: i32 = line_count.wrapping_mul(mcrust::net::minecraft::client::gui::font::LINE_HEIGHT);
            let mut start_x: i32 = ((compat::read(__this, |__t| __t.width)).wrapping_sub(total_width)).wrapping_div(2);
            let mut start_y: i32 = ((compat::read(__this, |__t| __t.height)).wrapping_sub(total_height)).wrapping_div(2);
            graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), start_x.wrapping_sub(5), start_y.wrapping_sub(5), (start_x.wrapping_add(total_width)).wrapping_add(5), (start_y.wrapping_add(total_height)).wrapping_add(5), 0xA0000000u32 as i32);
            for i in { 0..(__this.lock().borrow().preview_columns.len() as i32) } {
                let mut col_x: i32 = start_x.wrapping_add(i.wrapping_mul(col_width.wrapping_add(column_spacing)));
                graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), col_x, start_y, col_x.wrapping_add(col_width), start_y.wrapping_add(total_height), 0x20FFFFFF);
                // (the screen's own list, read in place: nothing below reaches this screen)
                let __this_guard = __this.lock();
                let __this_ref = __this_guard.borrow();
                let column = &__this_ref.preview_columns[(i) as usize];
                if column.is_empty() {
                    // java: List<FormattedCharSequence> split = font.split(..) - the port's lines are FormattedText
                    let mut split: Vec<mcrust::net::minecraft::network::chat::formatted_text::FormattedText> = compat::font().split(&mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::translatable("skyblocker.config.hud.widgetsList.playerColumn")), col_width);
                    for j in 0..(split.len() as i32) {
                        gui_graphics_extractor::draw_formatted_char_sequence(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &split[(j) as usize], col_x, start_y.wrapping_add(j.wrapping_mul(mcrust::net::minecraft::client::gui::font::LINE_HEIGHT)), -1);
                    }
                } else {
                    for j in 0..(column.len() as i32) {
                        let component = &column[(j) as usize];
                        // java: FormattedCharSequence trimmed = .. ? ComponentRenderUtils.clipText(component, font, colWidth) : component.getVisualOrderText();
                        let mut trimmed: mcrust::net::minecraft::network::chat::formatted_text::FormattedText = (if compat::font().width_of_cached(&mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component.clone())) >= col_width { component_render_utils::clip_text_formatted(&mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component.clone()), &compat::font(), col_width) } else { mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component.clone()) });
                        gui_graphics_extractor::draw_formatted_char_sequence(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &trimmed, col_x, start_y.wrapping_add(j.wrapping_mul(mcrust::net::minecraft::client::gui::font::LINE_HEIGHT)), -1);
                    }
                }
            }
        }
    }

    /// java: public void hopper(@Nullable List<String> hopperTooltip)
    pub fn hopper(__this: &compat::Shared<Self>, mut hopper_tooltip: Option<Vec<String>>) {
        let Some(mut hopper_tooltip) = hopper_tooltip.clone() else {
            compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.widgets_element_list.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.mod_widget().and_then(|__ms| __ms.with::<mcrust::mcrust::modding::widgets::ModSelectionList, _>(|__l| { let __h = &__l.class::<compat::ListClass<WidgetsElementList>>().map(|__c| __c.object().clone()).expect("a mod list without its class"); compat::lend_list(__h, __l, || WidgetsElementList::set_editing_position(__h, false, -1, -1)) })).expect("a mod list reached from inside its own method")) });
            return;
        };
        let mut start: i32 = -1;
        let mut editing: i32 = 1;
        let mut end: i32 = -1;
        for i in 0..(hopper_tooltip.len() as i32) {
            let mut string: String = hopper_tooltip[(i) as usize].clone();
            if string.contains("▶") {
                if start == -1 {
                    start = i;
                }
                if i > end {
                    end = i;
                }
            }
            if string.contains("(EDITING)") {
                editing = i;
            }
        }
        let __wa1 = editing.wrapping_sub(start);
        let __wa2 = end.wrapping_sub(start);
        compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.widgets_element_list.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.mod_widget().and_then(|__ms| __ms.with::<mcrust::mcrust::modding::widgets::ModSelectionList, _>(|__l| { let __h = &__l.class::<compat::ListClass<WidgetsElementList>>().map(|__c| __c.object().clone()).expect("a mod list without its class"); compat::lend_list(__h, __l, || WidgetsElementList::set_editing_position(__h, true, __wa1, __wa2)) })).expect("a mod list reached from inside its own method")) });
    }

    /// java: public void onSlotChange(int slot, ItemStack stack)
    pub fn on_slot_change(__this: &compat::Shared<Self>, mut slot: i32, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack) {
        { let __v = false; compat::write(__this, |__t| __t.waiting_for_server = __v); }
        { let __v = false; compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.waiting_for_server_text.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.visible = __v) }); }
        compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.widgets_element_list.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.mod_widget().and_then(|__ms| __ms.with::<mcrust::mcrust::modding::widgets::ModSelectionList, _>(|__l| { let __h = &__l.class::<compat::ListClass<WidgetsElementList>>().map(|__c| __c.object().clone()).expect("a mod list without its class"); compat::lend_list(__h, __l, || WidgetsElementList::update_list(__h)) })).expect("a mod list reached from inside its own method")) });
        match slot {
            45 => {
                // java: widgetsElementList.setIsOnSecondPage(previousPage.visible = stack.is(Items.ARROW));
                let __second_page: bool = stack.is_item(&items::items().get("arrow"));
                compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.previous_page.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.visible = __second_page) });
                compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.widgets_element_list.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.mod_widget().and_then(|__ms| __ms.with::<mcrust::mcrust::modding::widgets::ModSelectionList, _>(|__l| { let __h = &__l.class::<compat::ListClass<WidgetsElementList>>().map(|__c| __c.object().clone()).expect("a mod list without its class"); compat::lend_list(__h, __l, || WidgetsElementList::set_is_on_second_page(__h, __second_page)) })).expect("a mod list reached from inside its own method")) });
                return;
            }
            51 | 53 => {
                if slot == 53 {
                    { let __v = stack.is_item(&items::items().get("arrow")); compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.next_page.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.visible = __v) }); }
                }
                if stack.is_item(&items::items().get("player_head")) {
                    let mut stack_name: String = compat::component_of(stack.get_hover_name()).get_string().to_lowercase();
                    if !(stack_name.starts_with("reset")) {
                        return;
                    }
                    let mut button_text: MutableComponent = component::translatable("skyblocker.config.hud.widgetsList.resetAll").with_style_fn(|style| style.with_color_format(Some(ChatFormatting::Red)).with_underlined(Some(true)));
                    if slot == 51 {
                        button_text = component::translatable("skyblocker.config.hud.widgetsList.reset").with_style_format(ChatFormatting::Red);
                    }
                    { let __v = true; compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.reset_button.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.visible = __v) }); }
                    let __wa1 = button_text.clone();
                    compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.reset_button.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.set_message_text(mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(__wa1.clone()))) });
                    { let __v = slot; compat::write(__this, |__t| __t.reset_slot_id = __v); }
                }
                return;
            }
            50 => {
                { let __v = (stack.is_item(&items::items().get("bookshelf")) || stack.is_item(&items::items().get("stone_button"))); compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.third_column_button.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.visible = __v) }); }
                if compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.third_column_button.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.visible) }) {
                    if stack.is_item(&items::items().get("stone_button")) {
                        let __wa2 = component::translatable("skyblocker.config.hud.widgetsList.applyToAllLocations");
                        compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.third_column_button.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.set_message_text(mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(__wa2.clone()))) });
                    } else if ItemUtils::get_lore_line_if(stack, Arc::new(move |s| s.contains("DISABLED"))).is_none() {
                        { let __v = true; compat::write(__this, |__t| __t.third_column_enabled = __v); }
                        let __wa3 = component::translatable_args("skyblocker.config.hud.widgetsList.thirdColumn", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(widgets_list_entry::ENABLED_TEXT.get_mut().clone())]);
                        compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.third_column_button.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.set_message_text(mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(__wa3.clone()))) });
                        WidgetsListScreen::update_info_text(__this);
                    } else {
                        { let __v = false; compat::write(__this, |__t| __t.third_column_enabled = __v); }
                        let __wa4 = component::translatable_args("skyblocker.config.hud.widgetsList.thirdColumn", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(widgets_list_entry::DISABLED_TEXT.get_mut().clone())]);
                        compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.third_column_button.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.set_message_text(mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(__wa4.clone()))) });
                        WidgetsListScreen::update_info_text(__this);
                    }
                }
                return;
            }
            _ => {}
        }
        if stack.is_empty() || stack.is_item(mcrust::net::minecraft::world::item::items::stained_glass_pane().black()) {
            __this.lock().borrow_mut().entries.remove(&slot);
            return;
        }
        let mut lower_case: String = compat::trim(&compat::component_of(stack.get_hover_name()).get_string()).to_lowercase();
        let mut lore: Vec<String> = stack.skyblocker_get_lore_strings();
        let mut last_lower_case: String = (lore[lore.len() - 1]).to_lowercase();
        let mut entry: compat::Shared<dyn WidgetsListSlotEntry>;
        if lower_case.starts_with("widgets on") || lower_case.starts_with("widgets in") || last_lower_case.contains("click to edit") || stack.is_item(mcrust::net::minecraft::world::item::items::stained_glass_pane().red()) {
            entry = (EditableSlotEntry::new(__this.clone(), slot, stack.clone()) as compat::Shared<dyn WidgetsListSlotEntry>);
        } else if lower_case.ends_with("widget") {
            entry = (WidgetSlotEntry::new(__this.clone(), slot, stack.clone()) as compat::Shared<dyn WidgetsListSlotEntry>);
        } else if last_lower_case.contains("enable") || last_lower_case.contains("disable") {
            entry = (BooleanSlotEntry::new(__this.clone(), slot, stack.clone()) as compat::Shared<dyn WidgetsListSlotEntry>);
        } else {
            entry = (DefaultSlotEntry::new(__this.clone(), slot, stack.clone()) as compat::Shared<dyn WidgetsListSlotEntry>);
        }
        let __h0 = entry.clone();
        __this.lock().borrow_mut().entries.insert(slot, __h0);
    }

    /// java: @SuppressWarnings("deprecation") @Override public void slotChanged(AbstractContainerMenu handler, int slotId, ItemStack stack)
    pub fn slot_changed(__this: &compat::Shared<Self>, handler: &AbstractContainerMenu, mut slot_id: i32, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack) {
        if slot_id >= 3 && slot_id <= 5 {
            if !(compat::component_of(stack.get_hover_name()).get_string().ends_with("Widgets Preview")) {
                { let __v = false; compat::write(__this, |__t| __t.preview_visible = __v); }
                WidgetsListScreen::update_info_text(__this);
                return;
            } else {
                { let __v = true; compat::write(__this, |__t| __t.preview_visible = __v); }
            }
            if slot_id == 5 {
                let mut list: Vec<bool> = stack.skyblocker_get_lore_strings().into_iter().map(|s| compat::is_blank(&(compat::substring(&s, (compat::index_of(&s, '⬛')).wrapping_add(1))))).collect::<Vec<_>>();
                { let __v = ((list.len() as i32) >= 2 && !(list[list.len() - 1] && list[((list.len() as i32).wrapping_sub(2)) as usize])); compat::write(__this, |__t| __t.overflowing = __v); }
                WidgetsListScreen::update_info_text(__this);
            }
            __this.lock().borrow_mut().preview_columns[(slot_id.wrapping_sub(3)) as usize] = ItemUtils::get_lore(stack).into_iter().filter(|c| c.get_string().starts_with("⬛")).collect::<Vec<_>>();
        }
        if slot_id == 13 {
            if stack.is_item(&items::items().get("hopper")) {
                WidgetsListScreen::hopper(__this, Some(stack.skyblocker_get_lore_strings()));
            } else {
                WidgetsListScreen::hopper(__this, None);
            }
        }
        if slot_id > (if __this.lock().borrow().title_lowercase.starts_with("tablist widgets") { 9 } else { 18 }) && slot_id < ((__this.lock().borrow().handler.as_ref().unwrap().with(|__chest| __chest.get_row_count()).unwrap()).wrapping_mul(9)).wrapping_sub(9) || slot_id == 45 || slot_id == 53 || slot_id == 50 || slot_id == 51 {
            WidgetsListScreen::on_slot_change(__this, slot_id, stack);
        }
    }

    /// java: private void updateInfoText()
    pub fn update_info_text(__this: &compat::Shared<Self>) {
        if !(compat::read(__this, |__t| __t.preview_visible)) {
            // java: infoText.setMessage(CommonComponents.EMPTY);
            WidgetsListScreen::set_info_text_message(__this, mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::empty()));
            WidgetsListScreen::reposition_elements(__this);
            return;
        }
        let mut text: MutableComponent = component::translatable("skyblocker.config.hud.widgetsList.info.preview");
        if compat::read(__this, |__t| __t.overflowing) {
            let mut overflow_warning: MutableComponent = component::translatable("skyblocker.config.hud.widgetsList.info.overflowWarning").with_style_format(ChatFormatting::Red).append_str("\n");
            if !(compat::read(__this, |__t| __t.third_column_enabled)) {
                overflow_warning = overflow_warning.append(component::translatable("skyblocker.config.hud.widgetsList.info.overflowWarning.columnTip")).append_str("\n");
            }
            // (Java's `append` mutates the component; the port's answers the appended one)
            overflow_warning = overflow_warning.append(component::translatable("skyblocker.config.hud.widgetsList.info.overflowWarning.wrappingSpacingTip"));
            text = text.append_str("\n");
            text = text.append(overflow_warning.clone());
        }
        // java: infoText.setMessage(text);
        WidgetsListScreen::set_info_text_message(__this, mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(text.clone()));
        WidgetsListScreen::reposition_elements(__this);
    }

    /// java: @Override public void removed()
    pub fn removed(__this: &compat::Shared<Self>) {
        if mcrust::mcrust::modding::game::with_player(|_| ()).is_some() {
            // java: handler.removed(minecraft.player);
            { let __h = compat::read(__this, |__t| __t.handler.clone()); let _ = __h.as_ref().and_then(|__h| __h.with(|__chest| __chest.removed())); }
        }
        // java: handler.removeSlotListener(this); - the port's `removeSlotListener` clears the menu's listeners
        { let __h = compat::read(__this, |__t| __t.handler.clone()); let _ = __h.as_ref().and_then(|__h| __h.with(|__chest| __chest.menu.remove_slot_listener())); }
        compat::write(__this, |__t| __t.slot_changes = None);
    }

    /// `handler.addSlotListener(this)` once the handler is the player's open menu (the constructor runs inside
    /// `MenuScreens.fromPacket`, before it is): the listener queues `slotChanged` for `drain_slot_changes`.
    pub fn listen_to_handler(__this: &compat::Shared<Self>) {
        if compat::read(__this, |__t| __t.slot_changes.is_some()) {
            return;
        }
        let __handler = compat::read(__this, |__t| __t.handler.clone());
        let __queue: Option<compat::SlotChangeQueue> = __handler.as_ref().and_then(|__h| __h.with(|__chest| compat::listen_slot_changes(__chest, "WidgetsListScreen")));
        compat::write(__this, |__t| __t.slot_changes = __queue);
    }

    /// The `slotChanged(handler, slotId, stack)` calls the menu made since this screen's last own call, in order.
    pub fn drain_slot_changes(__this: &compat::Shared<Self>) {
        WidgetsListScreen::listen_to_handler(__this);
        let Some(__queue) = compat::read(__this, |__t| __t.slot_changes.clone()) else { return; };
        let __pending = compat::take_slot_changes(&__queue);
        if __pending.is_empty() {
            return;
        }
        // `slotChanged`'s `handler` argument is this screen's `handler` (its body does not read the argument; it reads the
        // field, through `LiveChestMenu`): the call gets the menu's id only, so no hold of the player is kept across it
        let Some(__container_id) = compat::read(__this, |__t| __t.handler.map(|__h| __h.container_id)) else { return; };
        let __menu = mcrust::net::minecraft::world::inventory::abstract_container_menu::AbstractContainerMenu::new(__container_id);
        for (__slot_id, __stack) in __pending {
            WidgetsListScreen::slot_changed(__this, &__menu, __slot_id, &__stack);
        }
    }

    /// The header's `MultiLineTextWidget` (`infoText`), with its box kept at the size the widget answers
    /// (`MultiLineTextWidget.getWidth()` / `getHeight()` follow its message and max width).
    pub fn with_info_text(__this: &compat::Shared<Self>, f: impl FnOnce(&mut MultiLineTextWidget)) {
        let __font = compat::font();
        let __splitter = __font.splitter();
        compat::write(__this, |__t| {
            let __s = &mut *__t;
            if let Some(__r) = __s.info_text.as_mut() {
                __s.__widgets.with(&mut __s.base, __r, |__w| {
                    if let mcrust::net::minecraft::client::gui::components::abstract_widget::WidgetKind::PlainMultiLineText(__m) = &mut __w.kind {
                        f(__m);
                        let (__mw, __mh) = (__m.get_width(&__splitter), __m.get_height(&__splitter));
                        __w.width = __mw;
                        __w.height = __mh;
                    }
                });
            }
        });
    }

    /// `infoText.setMessage(text)`.
    pub fn set_info_text_message(__this: &compat::Shared<Self>, text: mcrust::net::minecraft::network::chat::formatted_text::FormattedText) {
        let __text = text.clone();
        WidgetsListScreen::with_info_text(__this, |__m| __m.message = __text);
        compat::write(__this, |__t| {
            let __s = &mut *__t;
            if let Some(__r) = __s.info_text.as_mut() {
                __s.__widgets.with(&mut __s.base, __r, |__w| __w.set_message_text(text));
            }
        });
    }

    /// java: @Override public void onClose()
    pub fn on_close(__this: &compat::Shared<Self>) {
        if mcrust::mcrust::modding::game::with_player(|_| ()).is_some() {
            mcrust::mcrust::modding::game::with_player(|player| { player.close_container(); });
        }
        compat::set_state(&OVERRIDE_WIDGETS_SCREEN, false);
    }

    /// java: @Override public void dataChanged(AbstractContainerMenu container, int id, int value)
    pub fn data_changed(__this: &compat::Shared<Self>, container: &AbstractContainerMenu, mut id: i32, mut value: i32) {
    }

}

/// WidgetsListScreen's `Screen` half (`super`'s fields and widget tree), lent by the game for each call
impl compat::ScreenHalf for WidgetsListScreen {
    fn screen_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::screens::screen::Screen {
        &mut self.base
    }
    fn screen_parts(&mut self) -> (&mut mcrust::net::minecraft::client::gui::screens::screen::Screen, Option<&mut compat::WidgetCache>) {
        let __s = self;
        (&mut __s.base, Some(&mut __s.__widgets))
    }
}

/// WidgetsListScreen as the game's screen: `game::set_screen(Box::new(compat::ScreenAdapter::new(..)))`
impl ModScreen for compat::ScreenAdapter<WidgetsListScreen> {
    fn get_title(&self) -> String {
        self.title()
    }

    /// java: @Override protected void init()
    fn init(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        self.lend(screen, |__h| WidgetsListScreen::init(__h))
    }

    /// java: @Override protected void repositionElements()
    fn reposition_elements(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        self.lend(screen, |__h| WidgetsListScreen::reposition_elements(__h))
    }

    /// java: Screen.tick (the base body) - and the `ContainerListener` calls queued since the last call
    fn tick(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        self.lend(screen, |__h| WidgetsListScreen::drain_slot_changes(__h))
    }

    /// java: @Override public void removed()
    fn removed(&mut self) {
        self.call(|__h| WidgetsListScreen::removed(__h))
    }

    /// java: @Override public void extractRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_render_state(&mut self, screen: &mcrust::net::minecraft::client::gui::screens::screen::Screen, context: &mut ScreenRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.lend_ref(screen, |__h| WidgetsListScreen::extract_render_state(__h, &mut context.graphics, mouse_x, mouse_y, a))
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

    /// java: Screen.mouseClicked (the base body)
    fn mouse_clicked(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, x: f64, y: f64, button: i32) -> bool {
        self.lend(screen, |_| false)
    }

    /// java: @Override public void onClose()
    fn on_close(&mut self) {
        self.call(|__h| WidgetsListScreen::on_close(__h))
    }

}

// NOTE(fabric2seer): dataChanged, extractRenderState, init, onClose, removed, repositionElements, slotChanged override(s) of Screen - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them

//! Ported from de.hysky.skyblocker.config.screens.eventnotifications.EventConfigTimesEditScreen by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::network::chat::component::Component as _;
use crate::de::hysky::skyblocker::config::configs::event_notifications_config::EventConfig;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::utils::config::duration_controller::DurationController;
use crate::de::hysky::skyblocker::utils::time::skyblock_time::SkyblockTime;
use mcrust::mcrust::modding::screens::ModScreen;
use mcrust::mcrust::modding::widgets::ListKind;
use mcrust::mcrust::modding::widgets::ModList;
use mcrust::mcrust::modding::widgets::ModListEntry;
use mcrust::mcrust::modding::widgets::ModListEntrySlot;
use mcrust::mcrust::modding::widgets::ModSelectionList;
use mcrust::mcrust::modding::widgets::ModWidgetSlot;
use mcrust::mcrust::modding::widgets::WidgetRenderContext;
use mcrust::net::minecraft::client::gui::components::abstract_selection_list::AbstractSelectionList;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::PressLambda;
use mcrust::net::minecraft::client::gui::components::button;
use mcrust::net::minecraft::client::gui::components::edit_box::Responder;
use mcrust::net::minecraft::client::gui::components::edit_box::ResponderLambda;
use mcrust::net::minecraft::client::gui::components::sprite_icon_button;
use mcrust::net::minecraft::client::gui::layouts::header_and_footer_layout::HeaderAndFooterLayout;
use mcrust::net::minecraft::client::gui::layouts::layout_element::LayoutElement;
use mcrust::net::minecraft::client::gui::layouts::linear_layout::LinearLayout;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::key_event::KeyEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::formatted_text::FormattedText;
use mcrust::net::minecraft::resources::identifier::Identifier;

/// java: private static final Identifier DELETE_ICON = SkyblockerMod.id("trash_can");
pub static ENTRY_DELETE_ICON: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("trash_can"));
/// java: private static final int ICON_WIDTH = 12, ICON_HEIGHT = 15;
pub const ENTRY_ICON_WIDTH: i32 = 12;
/// java: private static final int ICON_WIDTH = 12, ICON_HEIGHT = 15;
pub const ENTRY_ICON_HEIGHT: i32 = 15;

/// java: public class EventConfigTimesEditScreen extends Screen
pub struct EventConfigTimesEditScreen {
    /// java: extends net.minecraft.client.gui.screens.Screen
    pub base: mcrust::net::minecraft::client::gui::screens::screen::Screen,
    /// java: private final EventNotificationsConfig.EventConfig eventConfig; (the config's own object)
    pub event_config: compat::Shared<EventConfig>,
    /// java: private final HeaderAndFooterLayout layout = new HeaderAndFooterLayout(this);
    pub layout: Option<HeaderAndFooterLayout>,
    /// java: private final Screen parent; (the game holds the parent: `set_screen_keeping_parent`)
    pub parent: Option<mcrust::net::minecraft::client::gui::screens::screen::Screen>,
    /// java: private EntryList list;
    pub list: Option<compat::WidgetRef>,
    /// the list OBJECT (Java's `list` field): the game's list widget's own slot, reached without the widget tree
    pub list_slot: Option<ModWidgetSlot>,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
    /// the screen's widgets as this object last saw them (see `compat::WidgetCache`)
    pub __widgets: compat::WidgetCache,
}

impl std::ops::Deref for EventConfigTimesEditScreen {
    type Target = mcrust::net::minecraft::client::gui::screens::screen::Screen;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for EventConfigTimesEditScreen {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl EventConfigTimesEditScreen {
    /// java: public EventConfigTimesEditScreen(Screen parent, String name, EventNotificationsConfig.EventConfig eventConfig)
    pub fn new(parent: Option<mcrust::net::minecraft::client::gui::screens::screen::Screen>, name: String, event_config: compat::Shared<EventConfig>) -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            base: mcrust::net::minecraft::client::gui::screens::screen::Screen::new((component::translatable_args("skyblocker.config.eventNotifications.screen.title", vec![FormattedText::of(name.clone())])).get_string()),
            self_ref: std::sync::Weak::new(),
            __widgets: Default::default(),
            event_config,
            // java: new HeaderAndFooterLayout(this) - sized by `repositionElements`
            layout: Some(HeaderAndFooterLayout::new(0, 0)),
            parent,
            list: None,
            list_slot: None,
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        __this
    }

    /// `list.x(..)`: the list object, lent for `f` (no borrow of this screen held).
    fn with_list<R>(__this: &compat::Shared<Self>, f: impl FnOnce(&mut AbstractSelectionList) -> R) -> R {
        let slot = compat::read(__this, |__t| __t.list_slot.clone()).expect("EventConfigTimesEditScreen.list before init()");
        slot.with::<ModSelectionList, _>(|l| f(&mut l.list)).expect("the list reached from inside its own method")
    }

    /// `screen.addWidget(widget)` as a layout element (the layout holds the screen's widget by index).
    fn add_layout_widget(__this: &compat::Shared<Self>, widget: AbstractWidget) -> LayoutElement {
        let (w, h) = (widget.get_width(), widget.get_height());
        let index = compat::write(__this, |__t| __t.add_widget(widget));
        LayoutElement::widget(index, w, h)
    }

    /// java: @Override protected void init()
    pub fn init(__this: &compat::Shared<Self>) {
        // java: layout.addTitleHeader(getTitle(), font); - `addToHeader(new StringWidget(title, font))`
        let title = compat::read(__this, |__t| __t.title.clone());
        let header = EventConfigTimesEditScreen::add_layout_widget(__this, compat::string_widget(0, 0, -1, 9, title));
        compat::write(__this, |__t| __t.layout.as_mut().unwrap().add_to_header(header));
        // java: list = layout.addToContents(new EntryList(width, layout.getContentHeight(), 0, 24));
        let (width, content_height) = compat::read(__this, |__t| (__t.width, __t.layout.as_ref().unwrap().get_content_height()));
        let list_widget = EntryList::new(width, content_height, 0, 24);
        let slot = list_widget.mod_widget();
        compat::write(__this, |__t| {
            __t.list_slot = slot;
            __t.list = Some(compat::WidgetRef::value(list_widget));
        });
        let (__i, __ww, __wh) = compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.list.as_mut().unwrap(); __s.__widgets.add_to_layout(&mut __s.base, __r) });
        compat::write(__this, |__t| __t.layout.as_mut().unwrap().add_to_contents(LayoutElement::widget(__i, __ww, __wh)));
        // java: list.replaceEntries(eventConfig.reminderTimes.intStream().sorted().mapToObj(Entry::new).toList());
        let mut reminder_times: Vec<i32> = compat::read(&compat::read(__this, |__t| __t.event_config.clone()), |__e| __e.reminder_times.clone());
        reminder_times.sort();
        EventConfigTimesEditScreen::with_list(__this, |list| {
            list.clear_entries();
            for seconds in reminder_times {
                add_entry(list, Entry::new(seconds));
            }
        });

        let mut footer_layout: LinearLayout = LinearLayout::horizontal().spacing(2);
        let __this_w = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        let add_reminder = button::builder(component::translatable("skyblocker.config.eventNotifications.screen.addReminder").get_string(), OnPress::Lambda(PressLambda::new(move |_screen, _index| {
            let Some(__this) = __this_w.get().upgrade() else { return; };
            // java: list.addEntry(new Entry(5 * 60))
            EventConfigTimesEditScreen::with_list(&__this, |list| add_entry(list, Entry::new(5 * 60)));
        })))
        .build();
        footer_layout.add_child(EventConfigTimesEditScreen::add_layout_widget(__this, add_reminder));
        let __this_w = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        let done = button::builder(mcrust::net::minecraft::network::chat::common_components::gui_done().get_string(), OnPress::Lambda(PressLambda::new(move |_screen, _index| {
            let Some(__this) = __this_w.get().upgrade() else { return; };
            EventConfigTimesEditScreen::on_close(&__this);
        })))
        .build();
        footer_layout.add_child(EventConfigTimesEditScreen::add_layout_widget(__this, done));
        compat::write(__this, |__t| __t.layout.as_mut().unwrap().add_to_footer(LayoutElement::Grid(footer_layout.wrapped)));
        // java: layout.visitWidgets(this::addRenderableWidget);
        let __vw = compat::read(__this, |__t| __t.layout.as_ref().unwrap().visit_widgets());
        compat::write(__this, |__t| {
            for (__i, _, _) in __vw {
                __t.mark_renderable(__i);
            }
        });
        EventConfigTimesEditScreen::reposition_elements(__this);
    }

    /// java: @Override protected void repositionElements()
    pub fn reposition_elements(__this: &compat::Shared<Self>) {
        let (width, height) = compat::read(__this, |__t| (__t.width, __t.height));
        compat::write(__this, |__t| {
            let layout = __t.layout.as_mut().unwrap();
            layout.screen_width = width;
            layout.screen_height = height;
        });
        // java: list.updateSize(this.width, this.layout);
        let layout = compat::read(__this, |__t| __t.layout.clone()).unwrap();
        EventConfigTimesEditScreen::with_list(__this, |list| list.update_size(width, &layout));
        // java: layout.arrangeElements();
        compat::write(__this, |__t| {
            __t.layout.as_mut().unwrap().arrange_elements();
            let __vw = __t.layout.as_ref().unwrap().visit_widgets();
            for (__i, __x, __y) in __vw {
                __t.widget_mut(__i).set_position(__x, __y);
            }
        });
    }

    /// java: @Override public void onClose()
    pub fn on_close(__this: &compat::Shared<Self>) {
        // java: list.children().stream().map(e -> e.box).filter(b -> b.valid).mapToInt(b -> b.seconds).sorted()
        let mut reminder_times: Vec<i32> = EventConfigTimesEditScreen::with_list(__this, |list| {
            (0..list.entry_count())
                .filter_map(|i| list.entry_object(i).and_then(|slot| slot.with::<Entry, _>(|e| e.r#box.state())).flatten())
                .collect()
        });
        reminder_times.sort();
        // java: SkyblockerConfigManager.update(_ -> eventConfig.reminderTimes = ..);
        let event_config = compat::RenderThread::new(compat::read(__this, |__t| __t.event_config.clone()));
        SkyblockerConfigManager::update(Arc::new(move |_: &mut crate::de::hysky::skyblocker::config::skyblocker_config::SkyblockerConfig| {
            let times = reminder_times.clone();
            compat::write(event_config.get(), |__e| __e.reminder_times = times);
        }));
        // java: minecraft.gui.setScreen(parent);
        mcrust::mcrust::modding::game::set_screen_to_parent();
    }
}

/// EventConfigTimesEditScreen's `Screen` half (`super`'s fields and widget tree), lent by the game for each call
impl compat::ScreenHalf for EventConfigTimesEditScreen {
    fn screen_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::screens::screen::Screen {
        &mut self.base
    }
    fn screen_parts(&mut self) -> (&mut mcrust::net::minecraft::client::gui::screens::screen::Screen, Option<&mut compat::WidgetCache>) {
        let __s = self;
        (&mut __s.base, Some(&mut __s.__widgets))
    }
}

/// EventConfigTimesEditScreen as the game's screen: `game::set_screen_keeping_parent(Box::new(compat::ScreenAdapter::new(..)))`
impl ModScreen for compat::ScreenAdapter<EventConfigTimesEditScreen> {
    fn get_title(&self) -> String {
        self.title()
    }

    /// java: @Override protected void init()
    fn init(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        self.lend(screen, |__h| EventConfigTimesEditScreen::init(__h))
    }

    /// java: @Override protected void repositionElements()
    fn reposition_elements(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        self.lend(screen, |__h| EventConfigTimesEditScreen::reposition_elements(__h))
    }

    /// java: Screen.tick (the base body)
    fn tick(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        self.lend(screen, |_| ())
    }

    /// java: @Override public void onClose()
    fn on_close(&mut self) {
        self.call(|__h| EventConfigTimesEditScreen::on_close(__h))
    }
}

/// `addEntry(entry)` of a row object.
fn add_entry(list: &mut AbstractSelectionList, entry: Entry) -> usize {
    let mut row = mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry::new(0);
    row.object = Some(ModListEntrySlot::new(Box::new(entry)));
    list.add_entry(row)
}

/// java: private class EntryList extends ContainerObjectSelectionList<EventConfigTimesEditScreen.Entry>
/// - the list's class (its overrides); the list itself is the game's `ModSelectionList`.
pub struct EntryList {}

impl EntryList {
    /// java: private EntryList(int width, int height, int y, int itemHeight) - the game's list widget with this class
    pub fn new(width: i32, height: i32, y: i32, item_height: i32) -> AbstractWidget {
        let list = ModSelectionList::with_class(ListKind::Container, width, height, y, item_height, Arc::new(EntryList {}));
        let (x, y, w, h) = (list.list.get_x(), list.list.get_y(), list.list.get_width(), list.list.get_height());
        ModWidgetSlot::widget(x, y, w, h, String::new(), Box::new(list))
    }

    /// java: @Override protected void removeEntry(EventConfigTimesEditScreen.Entry entry)
    pub fn remove_entry(list: &mut AbstractSelectionList, index: usize) {
        list.remove_entry(index);
    }

    /// java: @Override protected int addEntry(EventConfigTimesEditScreen.Entry entry)
    pub fn add_entry(list: &mut AbstractSelectionList, entry: Entry) -> i32 {
        add_entry(list, entry) as i32
    }
}

impl ModList for EntryList {
    /// The delete button's `_ -> list.removeEntry(this)`: the button runs inside its row's click (the row is lent
    /// to the game for it), so the row marks itself and the list removes it as the click returns.
    fn mouse_clicked(&self, list: &mut ModSelectionList, event: &MouseButtonEvent, double_click: bool, super_mouse_clicked: &mut dyn FnMut(&mut ModSelectionList) -> bool) -> bool {
        let answer = super_mouse_clicked(list);
        let mut i = 0;
        while i < list.list.entry_count() {
            let remove = list.list.entry_object(i).and_then(|slot| slot.with::<Entry, _>(|e| e.remove_requested.swap(false, std::sync::atomic::Ordering::Relaxed))).unwrap_or(false);
            if remove {
                EntryList::remove_entry(&mut list.list, i);
            } else {
                i += 1;
            }
        }
        answer
    }
}

/// java: private class Entry extends ContainerObjectSelectionList.Entry<Entry>
pub struct Entry {
    /// java: private final TimeEditBox box; (its state, and its position in `widgets`)
    pub r#box: TimeEditBox,
    /// java: private final List<AbstractWidget> widgets; - `List.of(box, buttonDelete)`, the row's children
    pub widgets: mcrust::net::minecraft::client::gui::screens::screen::Screen,
    /// java: private final LinearLayout entryLayout = LinearLayout.horizontal().spacing(4);
    pub entry_layout: LayoutElement,
    /// the delete button's `list.removeEntry(this)` (see `EntryList::mouse_clicked`)
    pub remove_requested: Arc<std::sync::atomic::AtomicBool>,
    thread: std::thread::ThreadId,
}

// SAFETY: a row is made and reached on the render thread only - every method checks it (`check`).
unsafe impl Send for Entry {}

impl Entry {
    /// java: private Entry(int seconds)
    pub fn new(seconds: i32) -> Entry {
        let mut widgets = mcrust::net::minecraft::client::gui::screens::screen::Screen::new(String::new());
        // java: box = new TimeEditBox(seconds);
        let (r#box, box_widget) = TimeEditBox::new(seconds);
        let box_index = widgets.add_renderable_widget(box_widget);
        let remove_requested = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let request = remove_requested.clone();
        // java: Button buttonDelete = SpriteIconButton.builder(Component.translatable("selectServer.deleteButton"), _ -> list.removeEntry(this), true)
        //         .size(20, 20).sprite(DELETE_ICON, ICON_WIDTH, ICON_HEIGHT).build();
        let button_delete = sprite_icon_button::builder(component::translatable("selectServer.deleteButton").get_string(), OnPress::Lambda(PressLambda::new(move |_screen, _index| {
            request.store(true, std::sync::atomic::Ordering::Relaxed);
        })), true)
        .size(20, 20)
        .sprite(ENTRY_DELETE_ICON.get().clone(), ENTRY_ICON_WIDTH, ENTRY_ICON_HEIGHT)
        .build()
        .expect("SpriteIconButton.Builder.build");
        let (delete_w, delete_h) = (button_delete.get_width(), button_delete.get_height());
        let delete_index = widgets.add_renderable_widget(button_delete);
        // java: entryLayout.addChild(box); entryLayout.addChild(buttonDelete); entryLayout.arrangeElements();
        let mut entry_layout = LinearLayout::horizontal().spacing(4);
        entry_layout.add_child(LayoutElement::widget(box_index, 150, 20));
        entry_layout.add_child(LayoutElement::widget(delete_index, delete_w, delete_h));
        let mut entry_layout = LayoutElement::Grid(entry_layout.wrapped);
        entry_layout.arrange_elements();
        Entry { r#box, widgets, entry_layout, remove_requested, thread: std::thread::current().id() }
    }

    fn check(&self) {
        assert!(std::thread::current().id() == self.thread, "a mod list row was reached off the render thread");
    }
}

impl ModListEntry for Entry {
    /// java: @Override public void extractContent(GuiGraphicsExtractor graphics, int mouseX, int mouseY, boolean hovered, float a)
    fn extract_content(&mut self, list: &mut ModSelectionList, index: usize, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, _hovered: bool, _a: f32) {
        self.check();
        let row = list.list.entry(index);
        // java: entryLayout.setPosition(getContentRight() - entryLayout.getWidth(), getContentY());
        let x = row.get_content_right() - self.entry_layout.get_width();
        self.entry_layout.set_position(x, row.get_content_y());
        let mut placed = Vec::new();
        self.entry_layout.visit_widgets(&mut placed);
        for (i, x, y) in placed {
            self.widgets.widget_mut(i).set_position(x, y);
        }
        // java: for (AbstractWidget widget : widgets) widget.extractRenderState(graphics, mouseX, mouseY, a);
        context.extract_widgets(&mut self.widgets, mouse_x, mouse_y);
    }

    /// java: @Override public List<? extends GuiEventListener> children() - `widgets`
    fn children(&mut self) -> Option<&mut mcrust::net::minecraft::client::gui::screens::screen::Screen> {
        Some(&mut self.widgets)
    }

    /// `ContainerEventHandler.keyPressed` - the focused child
    fn key_pressed(&mut self, _list: &mut ModSelectionList, _index: usize, event: &KeyEvent) -> bool {
        self.check();
        compat::children_key_pressed(&mut self.widgets, event)
    }

    /// `ContainerEventHandler.charTyped` - the focused child
    fn char_typed(&mut self, _list: &mut ModSelectionList, _index: usize, event: &CharacterEvent) -> bool {
        self.check();
        compat::children_char_typed(&mut self.widgets, event)
    }
}

/// java: private static class TimeEditBox extends EditBox
///
/// The box itself is the row's `EditBox` widget; this is its own state, which its responder (`this::onUpdate`)
/// writes as the value changes.
pub struct TimeEditBox {
    /// java: private int seconds; / private boolean valid = false;
    pub state: Arc<Mutex<(i32, bool)>>,
}

impl TimeEditBox {
    /// java: TimeEditBox(int seconds) - the object and its `EditBox` widget
    pub fn new(seconds: i32) -> (TimeEditBox, AbstractWidget) {
        // java: super(Minecraft.getInstance().font, 150, 20, Component.empty());
        let mut widget = compat::edit_box(0, 0, 150, 20, component::empty());
        // java: this.seconds = seconds;
        let state = Arc::new(Mutex::new((seconds, false)));
        // java: setResponder(this::onUpdate);
        let responder_state = state.clone();
        compat::with_edit_box(&mut widget, |e| e.set_responder(Responder::Lambda(ResponderLambda::new(move |s| TimeEditBox::on_update(&responder_state, s)))));
        // java: setValue(SkyblockTime.formatTime(seconds).getString());
        compat::set_edit_box_value(&mut widget, &SkyblockTime::format_time(seconds as f64).get_string());
        // java: addFormatter((string, _) -> FormattedCharSequence.forward(string, valid ? Style.EMPTY : Style.EMPTY.applyFormat(ChatFormatting.RED)));
        // rust: port gap - `EditBox.addFormatter` takes the port's closed `TextFormatter` enum, so a mod's formatter (the
        //   red text of an invalid time) cannot be added (GAPS_batchE.md)
        (TimeEditBox { state }, widget)
    }

    /// java: private void onUpdate(String s)
    pub fn on_update(state: &Mutex<(i32, bool)>, s: &str) {
        let mut state = compat::lock(state);
        state.1 = DurationController::is_valid(s.to_string());
        if state.1 {
            state.0 = DurationController::from_string(s);
        }
    }

    /// `b.valid ? b.seconds : (filtered out)`
    pub fn state(&self) -> Option<i32> {
        let state = compat::lock(&self.state);
        state.1.then_some(state.0)
    }
}

//! Ported from de.hysky.skyblocker.config.screens.eventnotifications.EventConfigTimesEditScreen by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::configs::event_notifications_config::EventConfig;
use crate::de::hysky::skyblocker::utils::config::duration_controller::DurationController;
use crate::de::hysky::skyblocker::utils::time::skyblock_time::SkyblockTime;
use mcrust::mcrust::modding::screens::ModScreen;
use mcrust::mcrust::modding::screens::ScreenRenderContext;
use mcrust::mcrust::modding::widgets::ModList;
use mcrust::mcrust::modding::widgets::ModListEntry;
use mcrust::mcrust::modding::widgets::ModSelectionList;
use mcrust::mcrust::modding::widgets::WidgetRenderContext;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::components::abstract_selection_list::AbstractSelectionList;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::PressLambda;
use mcrust::net::minecraft::client::gui::components::button;
use mcrust::net::minecraft::client::gui::components::edit_box::EditBox;
use mcrust::net::minecraft::client::gui::components::edit_box::Responder;
use mcrust::net::minecraft::client::gui::components::edit_box::ResponderLambda;
use mcrust::net::minecraft::client::gui::components::events::gui_event_listener::GuiEventListener;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::layouts::header_and_footer_layout::HeaderAndFooterLayout;
use mcrust::net::minecraft::client::gui::layouts::layout_element::LayoutElement;
use mcrust::net::minecraft::client::gui::layouts::linear_layout::LinearLayout;
use mcrust::net::minecraft::client::gui::narration::narratable_entry::NarratableEntry;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::client::input::mouse_button_info::MouseButtonInfo;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::util::formatted_char_sequence;
use mcrust::net::minecraft::client::gui::components::renderable::Renderable as _;
use mcrust::net::minecraft::network::chat::component::Component as _;

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
    /// java: private final EventNotificationsConfig.EventConfig eventConfig;
    pub event_config: compat::Shared<EventConfig>,
    /// java: private final HeaderAndFooterLayout layout = new HeaderAndFooterLayout(this);
    pub layout: Option<HeaderAndFooterLayout>,
    /// java: private final Screen parent;
    pub parent: mcrust::net::minecraft::client::gui::screens::screen::Screen,
    /// java: private EntryList list;
    pub list: Option<compat::WidgetRef>,
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
    pub fn new(mut parent: mcrust::net::minecraft::client::gui::screens::screen::Screen, mut name: String, mut event_config: compat::Shared<EventConfig>) -> compat::Shared<Self> {
        let __init_event_config = event_config.clone();
        // TODO(fabric2seer): the mod keeps the game's `Screen` (Java holds a reference to the live object); the port's `Screen` is not a value to copy and the port lends the mod no handle to it
        //   java: this.parent = parent;
        let __this = compat::shared(Self {
            base: mcrust::net::minecraft::client::gui::screens::screen::Screen::new((component::translatable_args("skyblocker.config.eventNotifications.screen.title", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of(name.clone())])).get_string()),
            self_ref: std::sync::Weak::new(),
            __widgets: Default::default(),
            event_config: __init_event_config,
            layout: None,
            parent: todo!("fabric2seer: the field value (TODO above)"),
            list: None,
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        { let __v = Some(HeaderAndFooterLayout::new(0, 0)); compat::write(&__this, |__t| __t.layout = __v); }
        __this
    }

    /// java: @Override protected void init()
    pub fn init(__this: &compat::Shared<Self>) {
        ();
        // TODO(fabric2seer): `HeaderAndFooterLayout.addTitleHeader` has no Rust method in the port (looked for `add_title_header` in HeaderAndFooterLayout and its supertypes)
        // rust: let __h0 = __this.lock().borrow().get_title();
        // rust: let __h1 = &compat::font();
        // rust: __this.lock().borrow().layout.as_ref().unwrap().add_title_header(__h0, __h1);
        { let __v = (Some(EntryList::new(compat::read(__this, |__t| __t.width), __this.lock().borrow().layout.as_ref().unwrap().get_content_height(), 0, 24))).map(|__o| compat::WidgetRef::value(compat::attach_list(__o))); compat::write(__this, |__t| { let __s = &mut *__t; __s.list = __v; }); }
        { let (__q0,) = ({ let (__i, __ww, __wh) = compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.list.as_mut().unwrap(); __s.__widgets.add_to_layout(&mut __s.base, __r) }); LayoutElement::widget(__i, __ww, __wh) },); __this.lock().borrow_mut().layout.as_mut().unwrap().add_to_contents(__q0) };
        // TODO(fabric2seer): List.intStream(..)
        //   java: list.replaceEntries(eventConfig.reminderTimes.intStream().sorted().mapToObj(Entry::new).toList());
        let mut footer_layout: LinearLayout = LinearLayout::horizontal().spacing(2);
        let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        footer_layout.add_child({ let __w = (button::builder(component::translatable("skyblocker.config.eventNotifications.screen.addReminder").get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { let Some(__this_1) = __this_w1.get().upgrade() else { return Default::default(); }; compat::lend_screen(&__this_1, __screen, || { let mut __pidx = compat::WidgetRef::Screen(__index); { let __wa2 = Entry::new(5i32.wrapping_mul(60)); compat::write((&__this_1), |__t| { let __s = &mut *__t; let __r = __s.list.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.mod_widget().and_then(|__ms| __ms.with::<mcrust::mcrust::modding::widgets::ModSelectionList, _>(|__l| { let __h = &__l.class::<compat::ListClass<EntryList>>().map(|__c| __c.object().clone()).expect("a mod list without its class"); compat::lend_list(__h, __l, || EntryList::add_entry(__h, &__wa2)) })).expect("a mod list reached from inside its own method")) }); } }) })))).build(); let (__ww, __wh) = (__w.get_width(), __w.get_height()); LayoutElement::widget(compat::write(__this, |__t| __t.add_widget(__w)), __ww, __wh) });
        let __this_w2 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        footer_layout.add_child({ let __w = (button::builder(mcrust::net::minecraft::network::chat::common_components::gui_done().get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { let Some(__this_2) = __this_w2.get().upgrade() else { return Default::default(); }; compat::lend_screen(&__this_2, __screen, || { let mut __pidx = compat::WidgetRef::Screen(__index); { EventConfigTimesEditScreen::on_close((&__this_2)); } }) })))).build(); let (__ww, __wh) = (__w.get_width(), __w.get_height()); LayoutElement::widget(compat::write(__this, |__t| __t.add_widget(__w)), __ww, __wh) });
        { let (__q0,) = (LayoutElement::Grid(footer_layout.clone().wrapped),); __this.lock().borrow_mut().layout.as_mut().unwrap().add_to_footer(__q0) };
        { let __vw = __this.lock().borrow().layout.as_ref().unwrap().visit_widgets(); for (__i, _, _) in __vw { compat::write(__this, |__t| __t.mark_renderable(__i)); } };
        EventConfigTimesEditScreen::reposition_elements(__this);
    }

    /// java: @Override protected void repositionElements()
    pub fn reposition_elements(__this: &compat::Shared<Self>) {
        let __wa1 = compat::read(__this, |__t| __t.width);
        let __wa2 = compat::read(__this, |__t| __t.layout.clone());
        compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.list.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.mod_widget().and_then(|__ms| __ms.with::<mcrust::mcrust::modding::widgets::ModSelectionList, _>(|__l| __l.list.update_size(__wa1, __wa2.as_ref().unwrap()))).expect("a mod list reached from inside its own method")) });
        let __r0 = compat::read(__this, |__t| (__t.width, __t.height));
        { let (__sw, __sh) = __r0; __this.lock().borrow_mut().layout.as_mut().unwrap().screen_width = __sw; __this.lock().borrow_mut().layout.as_mut().unwrap().screen_height = __sh; __this.lock().borrow_mut().layout.as_mut().unwrap().arrange_elements(); let __vw = __this.lock().borrow().layout.as_ref().unwrap().visit_widgets(); for (__i, __x, __y) in __vw { compat::write(__this, |__t| __t.widget_mut(__i).set_position(__x, __y)); } };
    }

    /// java: @Override public void onClose()
    pub fn on_close(__this: &compat::Shared<Self>) {
        // TODO(fabric2seer): Stream.collect
        //   java: SkyblockerConfigManager.update(_ -> eventConfig.reminderTimes = list.children().stream()
        //   java: 				.map(e -> e.box)
        //   java: 				.filter(b -> b.valid)
        //   java: 				.mapToInt(b -> b.seconds)
        //   java: 				.sorted()
        //   java: 				.collect(IntArrayList::new, IntArrayList::add, IntArrayList::addAll));
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

/// EventConfigTimesEditScreen as the game's screen: `game::set_screen(Box::new(compat::ScreenAdapter::new(..)))`
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
        self.call(|__h| EventConfigTimesEditScreen::on_close(__h))
    }

}

// NOTE(fabric2seer): init, onClose, repositionElements override(s) of Screen - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them

/// java: private class EntryList extends ContainerObjectSelectionList<EventConfigTimesEditScreen.Entry>
pub struct EntryList {
    /// java: extends net.minecraft.client.gui.components.ContainerObjectSelectionList
    pub base: ModSelectionList,
}

impl std::ops::Deref for EntryList {
    type Target = AbstractSelectionList;
    fn deref(&self) -> &Self::Target {
        &self.base.list
    }
}

impl std::ops::DerefMut for EntryList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base.list
    }
}

impl EntryList {
    /// java: private EntryList(int width, int height, int y, int itemHeight)
    pub fn new(mut width: i32, mut height: i32, mut y: i32, mut item_height: i32) -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            base: mcrust::mcrust::modding::widgets::ModSelectionList::new(mcrust::mcrust::modding::widgets::ListKind::Container, width, height, y, item_height),
        });
        __this
    }

    /// java: @Override protected void removeEntry(EventConfigTimesEditScreen.Entry entry)
    pub fn remove_entry(__this: &compat::Shared<Self>, entry: &(_, _)) {
        let __entry = entry.clone();
        compat::write(__this, |__t| { let __l = &mut __t.base; { if let Some(__i) = compat::entry_index::<Entry>(__l, &__entry) { __l.remove_entry(__i); } } });
    }

    /// java: @Override protected int addEntry(EventConfigTimesEditScreen.Entry entry)
    pub fn add_entry(__this: &compat::Shared<Self>, entry: &(_, _)) -> i32 {
        let __entry = entry.clone();
        return compat::write(__this, |__t| { let __l = &mut __t.base; (__l.add_entry(Box::new(compat::EntryAdapter::new(__entry))) as i32) });
    }

}

/// EntryList's list (the port's `ModSelectionList`) until the game takes it (`compat::attach_list`)
impl compat::ListHalf for EntryList {
    fn list_half(&mut self) -> &mut ModSelectionList {
        &mut self.base
    }
}

/// EntryList's overrides of the list's own methods, asked by the port's list bodies
impl ModList for compat::ListClass<EntryList> {
}

// NOTE(fabric2seer): addEntry, removeEntry override(s) of ContainerObjectSelectionList - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them

/// java: private class Entry extends ContainerObjectSelectionList.Entry<Entry>
pub struct Entry {
    /// java: extends net.minecraft.client.gui.components.ContainerObjectSelectionList$Entry
    pub base: mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry,
    /// java: private final TimeEditBox box;
    pub r#box: TimeEditBox,
    /// java: private final List<AbstractWidget> widgets;
    pub widgets: Vec<AbstractWidget>,
    /// java: private final LinearLayout entryLayout = LinearLayout.horizontal().spacing(4);
    pub entry_layout: LinearLayout,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
    /// the row's widgets (`children()`): lent to the object by its `compat::EntryAdapter`
    pub __row: mcrust::net::minecraft::client::gui::screens::screen::Screen,
    /// the screen's widgets as this object last saw them (see `compat::WidgetCache`)
    pub __widgets: compat::WidgetCache,
}

impl std::ops::Deref for Entry {
    type Target = mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for Entry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl Entry {
    /// java: private Entry(int seconds)
    pub fn new(mut seconds: i32) -> compat::Shared<Self> {
        let __init_box = TimeEditBox::new(seconds);
        let __this = compat::shared(Self {
            base: mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry::new(0),
            self_ref: std::sync::Weak::new(),
            __widgets: Default::default(),
            __row: mcrust::net::minecraft::client::gui::screens::screen::Screen::new(String::new()),
            r#box: __init_box,
            widgets: Default::default(),
            entry_layout: LinearLayout::horizontal().spacing(4),
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        // TODO(fabric2seer): `list` is a field of the enclosing instance (EventConfigTimesEditScreen.this) - an inner class has no outer reference in Rust
        //   java: Button buttonDelete = SpriteIconButton.builder(Component.translatable("selectServer.deleteButton"), _ -> list.removeEntry(this), true).size(20, 20).sprite(DELETE_ICON, ICON_WIDTH, ICON_HEIGHT).build();
        let mut button_delete: AbstractWidget = todo!("fabric2seer: the initialiser of `buttonDelete` (TODO above)");
        let __r0 = compat::read(&__this, |__t| __t.r#box.clone());
        { let (__q0,) = (__r0,); __this.lock().borrow_mut().entry_layout.add_child(__q0) };
        { let (__q0,) = ({ let __w = button_delete.clone(); let (__ww, __wh) = (__w.get_width(), __w.get_height()); LayoutElement::widget(compat::write(&__this, |__t| __t.__row.add_widget(__w)), __ww, __wh) },); __this.lock().borrow_mut().entry_layout.add_child(__q0) };
        { __this.lock().borrow_mut().entry_layout.arrange_elements(); let mut __vw = Vec::new(); LayoutElement::Grid(__this.lock().borrow().entry_layout.wrapped.clone()).visit_widgets(&mut __vw); for (__i, __x, __y) in __vw { compat::write(&__this, |__t| __t.__row.widget_mut(__i).set_position(__x, __y)); } };
        { let __v = vec![compat::read(&__this, |__t| __t.r#box.clone()), button_delete.clone()]; compat::write(&__this, |__t| __t.widgets = __v); }
        __this
    }

    /// java: @Override public List<? extends NarratableEntry> narratables()
    pub fn narratables(__this: &compat::Shared<Self>) -> Vec<Box<dyn NarratableEntry>> {
        return compat::read(__this, |__t| __t.widgets.clone());
    }

    /// java: @Override public void extractContent(GuiGraphicsExtractor graphics, int mouseX, int mouseY, boolean hovered, float a)
    pub fn extract_content(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut hovered: bool, mut a: f32) {
        // TODO(fabric2seer): `LayoutElement::set_position` ports `setPosition`, but the port's `LinearLayout` holds no `LayoutElement` (no Deref, no `base` field) to call it on
        // rust: { let (__q0, __q1,) = ((__this.lock().borrow().get_content_right()).wrapping_sub(__this.lock().borrow().entry_layout.get_width()), __this.lock().borrow().get_content_y(),); __this.lock().borrow_mut().entry_layout.set_position(__q0, __q1) };
        for widget in compat::read(__this, |__t| __t.widgets.clone()) {
            // TODO(fabric2seer): `extract_render_state` is a method of the port's trait `Renderable`, which the port's `AbstractWidget` does not implement
            // rust: widget.extract_render_state(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mouse_x, mouse_y, a);
        }
    }

    /// java: @Override public List<? extends GuiEventListener> children()
    pub fn children(__this: &compat::Shared<Self>) -> Vec<Box<dyn GuiEventListener>> {
        return compat::read(__this, |__t| __t.widgets.clone());
    }

}

/// Entry's row box and its widgets, lent by its `compat::EntryAdapter` for each call
impl compat::RowHalf for Entry {
    fn entry_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry {
        &mut self.base
    }
    fn row_parts(&mut self) -> (&mut mcrust::net::minecraft::client::gui::screens::screen::Screen, &mut compat::WidgetCache) {
        let __s = self;
        (&mut __s.__row, &mut __s.__widgets)
    }
}

/// Entry as a row of the game's list (`addEntry` attaches it)
impl ModListEntry for compat::EntryAdapter<Entry> {
    /// java: @Override public void extractContent(GuiGraphicsExtractor graphics, int mouseX, int mouseY, boolean hovered, float a)
    fn extract_content(&mut self, list: &mut ModSelectionList, index: usize, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, hovered: bool, a: f32) {
        self.lend(Some(list.list.entry(index)), |__h| Entry::extract_content(__h, &context.graphics, mouse_x, mouse_y, hovered, a));
        // the row's widgets (Java's `child.extractRenderState(..)` over `children()`)
        context.extract_widgets(&mut self.row, mouse_x, mouse_y);
    }

    fn children(&mut self) -> Option<&mut mcrust::net::minecraft::client::gui::screens::screen::Screen> {
        Some(&mut self.row)
    }

}

// NOTE(fabric2seer): children, extractContent, narratables override(s) of Entry - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them

/// java: private static class TimeEditBox extends EditBox
#[derive(Clone)]
pub struct TimeEditBox {
    /// java: extends net.minecraft.client.gui.components.EditBox
    pub base: EditBox,
    /// java: private int seconds;
    pub seconds: i32,
    /// java: private boolean valid = false;
    pub valid: bool,
}

impl std::ops::Deref for TimeEditBox {
    type Target = EditBox;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for TimeEditBox {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl TimeEditBox {
    /// java: TimeEditBox(int seconds)
    pub fn new(mut seconds: i32) -> Self {
        let __init_seconds = seconds;
        // TODO(fabric2seer): the port's `new` ports this Java method with other parameters (splitter, x, y, width, height for Java's width, height, narration)
        //   java: super(Minecraft.getInstance().font, 150, 20, Component.empty());
        let mut this: Self = todo!("constructor");
        this.set_responder(Responder::Lambda(ResponderLambda::new(move |__a0| { this.on_update(__a0); })));
        // TODO(fabric2seer): `set_value` takes 2 argument(s) in the port (`splitter: &StringSplitter<W>, value: &str`), Java passes 1
        // rust: this.set_value(&(SkyblockTime::format_time((seconds) as f64)).get_string());
        // TODO(fabric2seer): a lambda for a parameter the port does not type as a closure (`net.minecraft.client.gui.components.EditBox.TextFormatter`)
        // rust: this.add_formatter(|string, _| formatted_char_sequence::forward(&string, (if this.valid { mcrust::net::minecraft::network::chat::style::Style::EMPTY } else { mcrust::net::minecraft::network::chat::style::Style::EMPTY.apply_format(ChatFormatting::Red) })));
        this
    }

    /// java: private void onUpdate(String s)
    pub fn on_update(&mut self, s: &str) {
        self.valid = DurationController::is_valid(s.to_string());
        if self.valid {
            self.seconds = DurationController::from_string(s);
        }
    }

}

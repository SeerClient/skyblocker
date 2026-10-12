//! Ported from de.hysky.skyblocker.config.screens.greenhouse.GreenhousePresetsScreen by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::network::chat::component::Component as _;
use crate::de::hysky::skyblocker::skyblock::garden::greenhouse_paste;
use crate::de::hysky::skyblocker::utils::data::json_data::JsonData as _;
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
use mcrust::net::minecraft::client::gui::layouts::header_and_footer_layout::HeaderAndFooterLayout;
use mcrust::net::minecraft::client::gui::layouts::layout_element::LayoutElement;
use mcrust::net::minecraft::client::gui::layouts::linear_layout::LinearLayout;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::key_event::KeyEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::formatted_text::FormattedText;

/// java: public class GreenhousePresetsScreen extends Screen
pub struct GreenhousePresetsScreen {
    /// java: extends net.minecraft.client.gui.screens.Screen
    pub base: mcrust::net::minecraft::client::gui::screens::screen::Screen,
    /// java: private final HeaderAndFooterLayout layout = new HeaderAndFooterLayout(this);
    pub layout: Option<HeaderAndFooterLayout>,
    /// java: private final @Nullable Screen parent; (the game holds the parent: `set_screen_keeping_parent`)
    pub parent: Option<mcrust::net::minecraft::client::gui::screens::screen::Screen>,
    /// java: private PresetsList presetsList;
    pub presets_list: Option<compat::WidgetRef>,
    /// the list OBJECT (Java's `presetsList` field): the game's list widget's own slot
    pub list_slot: Option<ModWidgetSlot>,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
    /// the screen's widgets as this object last saw them (see `compat::WidgetCache`)
    pub __widgets: compat::WidgetCache,
}

impl std::ops::Deref for GreenhousePresetsScreen {
    type Target = mcrust::net::minecraft::client::gui::screens::screen::Screen;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for GreenhousePresetsScreen {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

/// `GreenhousePaste.PRESETS_DATA.getData()` - the live map (Java's `getData()` hands out the object itself).
fn with_presets_data<R>(f: impl FnOnce(&mut std::collections::HashMap<String, String>) -> R) -> R {
    let handle = greenhouse_paste::PRESETS_DATA.get().clone();
    let guard = handle.lock();
    let mut data = guard.borrow_mut();
    f(data.json_data_base_mut().data.as_mut().expect("GreenhousePaste.PRESETS_DATA"))
}

impl GreenhousePresetsScreen {
    /// java: public GreenhousePresetsScreen(@Nullable Screen parent)
    pub fn new(parent: Option<mcrust::net::minecraft::client::gui::screens::screen::Screen>) -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            base: mcrust::net::minecraft::client::gui::screens::screen::Screen::new(component::translatable("skyblocker.config.farming.greenhouse.greenhousePresets.title").get_string()),
            self_ref: std::sync::Weak::new(),
            __widgets: Default::default(),
            // java: new HeaderAndFooterLayout(this) - sized by `repositionElements`
            layout: Some(HeaderAndFooterLayout::new(0, 0)),
            parent,
            presets_list: None,
            list_slot: None,
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        __this
    }

    /// java: public GreenhousePresetsScreen()
    pub fn new_0() -> compat::Shared<Self> {
        Self::new(None)
    }

    /// `presetsList.x(..)`: the list object, lent for `f` (no borrow of this screen held).
    fn with_list<R>(__this: &compat::Shared<Self>, f: impl FnOnce(&mut AbstractSelectionList) -> R) -> R {
        let slot = compat::read(__this, |__t| __t.list_slot.clone()).expect("GreenhousePresetsScreen.presetsList before init()");
        slot.with::<ModSelectionList, _>(|l| f(&mut l.list)).expect("the presets list reached from inside its own method")
    }

    /// `addRenderableWidget`-bound widget as a layout element (the layout holds the screen's widget by index).
    fn add_layout_widget(__this: &compat::Shared<Self>, widget: AbstractWidget) -> LayoutElement {
        let (w, h) = (widget.get_width(), widget.get_height());
        let index = compat::write(__this, |__t| __t.add_widget(widget));
        LayoutElement::widget(index, w, h)
    }

    /// java: @Override protected void init()
    pub fn init(__this: &compat::Shared<Self>) {
        // java: layout.addTitleHeader(getTitle(), font); - `addToHeader(new StringWidget(title, font))`
        let title = compat::read(__this, |__t| __t.title.clone());
        let header = GreenhousePresetsScreen::add_layout_widget(__this, compat::string_widget(0, 0, -1, 9, title));
        compat::write(__this, |__t| __t.layout.as_mut().unwrap().add_to_header(header));
        // java: presetsList = layout.addToContents(new PresetsList(minecraft, width, layout.getContentHeight()));
        let (width, content_height) = compat::read(__this, |__t| (__t.width, __t.layout.as_ref().unwrap().get_content_height()));
        let list_widget = PresetsList::new(width, content_height);
        let slot = list_widget.mod_widget();
        compat::write(__this, |__t| {
            __t.list_slot = slot;
            __t.presets_list = Some(compat::WidgetRef::value(list_widget));
        });
        let (__i, __ww, __wh) = compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.presets_list.as_mut().unwrap(); __s.__widgets.add_to_layout(&mut __s.base, __r) });
        compat::write(__this, |__t| __t.layout.as_mut().unwrap().add_to_contents(LayoutElement::widget(__i, __ww, __wh)));
        // java: GreenhousePaste.PRESETS_DATA.getData().forEach((name, link) -> presetsList.addEntry(new PresetEntry(name, link)));
        let presets: Vec<(String, String)> = greenhouse_paste::PRESETS_DATA.get().lock().borrow().get_data().into_iter().collect();
        GreenhousePresetsScreen::with_list(__this, |list| {
            for (name, link) in presets {
                PresetsList::add_entry(list, PresetEntry::new(&name, &link));
            }
        });
        let mut footer: LinearLayout = LinearLayout::horizontal().spacing(8);
        let __this_w = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        let add_preset = button::builder(component::translatable("skyblocker.config.farming.greenhouse.greenhousePresets.addPreset").get_string(), OnPress::Lambda(PressLambda::new(move |_screen, _index| {
            let Some(__this) = __this_w.get().upgrade() else { return; };
            // java: presetsList.addEntry(new PresetEntry())
            GreenhousePresetsScreen::with_list(&__this, |list| PresetsList::add_entry(list, PresetEntry::new_0()));
        })))
        .build();
        footer.add_child(GreenhousePresetsScreen::add_layout_widget(__this, add_preset));
        let __this_w = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        let done = button::builder(mcrust::net::minecraft::network::chat::common_components::gui_done().get_string(), OnPress::Lambda(PressLambda::new(move |_screen, _index| {
            let Some(__this) = __this_w.get().upgrade() else { return; };
            GreenhousePresetsScreen::on_close(&__this);
        })))
        .build();
        footer.add_child(GreenhousePresetsScreen::add_layout_widget(__this, done));
        compat::write(__this, |__t| __t.layout.as_mut().unwrap().add_to_footer(LayoutElement::Grid(footer.wrapped)));
        GreenhousePresetsScreen::reposition_elements(__this);
        // java: layout.visitWidgets(this::addRenderableWidget);
        let __vw = compat::read(__this, |__t| __t.layout.as_ref().unwrap().visit_widgets());
        compat::write(__this, |__t| {
            for (__i, _, _) in __vw {
                __t.mark_renderable(__i);
            }
        });
    }

    /// java: @Override protected void repositionElements()
    pub fn reposition_elements(__this: &compat::Shared<Self>) {
        let (width, height) = compat::read(__this, |__t| (__t.width, __t.height));
        compat::write(__this, |__t| {
            let layout = __t.layout.as_mut().unwrap();
            layout.screen_width = width;
            layout.screen_height = height;
        });
        // java: presetsList.updateSize(width, layout);
        let layout = compat::read(__this, |__t| __t.layout.clone()).unwrap();
        GreenhousePresetsScreen::with_list(__this, |list| list.update_size(width, &layout));
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
        // java: GreenhousePaste.PRESETS_DATA.getData().clear();
        with_presets_data(|data| data.clear());
        // java: presetsList.children().forEach(entry -> { .. put(name.trim(), link.trim()); });
        let entries: Vec<(String, String)> = GreenhousePresetsScreen::with_list(__this, |list| {
            (0..list.entry_count()).filter_map(|i| list.entry_object(i).and_then(|slot| slot.with::<PresetEntry, _>(|e| (e.name_value(), e.link_value())))).collect()
        });
        for (name, link) in entries {
            if name.trim().is_empty() || link.trim().is_empty() {
                continue;
            }
            with_presets_data(|data| data.insert(name.trim().to_string(), link.trim().to_string()));
        }
        greenhouse_paste::PRESETS_DATA.get().lock().borrow().save();
        // java: minecraft.gui.setScreen(parent);
        mcrust::mcrust::modding::game::set_screen_to_parent();
    }
}

/// GreenhousePresetsScreen's `Screen` half (`super`'s fields and widget tree), lent by the game for each call
impl compat::ScreenHalf for GreenhousePresetsScreen {
    fn screen_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::screens::screen::Screen {
        &mut self.base
    }
    fn screen_parts(&mut self) -> (&mut mcrust::net::minecraft::client::gui::screens::screen::Screen, Option<&mut compat::WidgetCache>) {
        let __s = self;
        (&mut __s.base, Some(&mut __s.__widgets))
    }
}

/// GreenhousePresetsScreen as the game's screen: `game::set_screen_keeping_parent(Box::new(compat::ScreenAdapter::new(..)))`
impl ModScreen for compat::ScreenAdapter<GreenhousePresetsScreen> {
    fn get_title(&self) -> String {
        self.title()
    }

    /// java: @Override protected void init()
    fn init(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        self.lend(screen, |__h| GreenhousePresetsScreen::init(__h))
    }

    /// java: @Override protected void repositionElements()
    fn reposition_elements(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        self.lend(screen, |__h| GreenhousePresetsScreen::reposition_elements(__h))
    }

    /// java: Screen.tick (the base body)
    fn tick(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        self.lend(screen, |_| ())
    }

    /// java: @Override public void onClose()
    fn on_close(&mut self) {
        self.call(|__h| GreenhousePresetsScreen::on_close(__h))
    }
}

/// java: private static class PresetsList extends ContainerObjectSelectionList<PresetEntry> - the list's class
pub struct PresetsList {}

impl PresetsList {
    /// java: private PresetsList(Minecraft minecraft, int width, int height) - `super(minecraft, width, height, 0, 24)`
    pub fn new(width: i32, height: i32) -> AbstractWidget {
        let list = ModSelectionList::with_class(ListKind::Container, width, height, 0, 24, Arc::new(PresetsList {}));
        let (x, y, w, h) = (list.list.get_x(), list.list.get_y(), list.list.get_width(), list.list.get_height());
        ModWidgetSlot::widget(x, y, w, h, String::new(), Box::new(list))
    }

    /// java: @Override protected void removeEntry(PresetEntry entry)
    pub fn remove_entry(list: &mut AbstractSelectionList, index: usize) {
        list.remove_entry(index);
    }

    /// java: @Override protected int addEntry(PresetEntry entry)
    pub fn add_entry(list: &mut AbstractSelectionList, entry: PresetEntry) -> i32 {
        let mut row = mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry::new(0);
        row.object = Some(ModListEntrySlot::new(Box::new(entry)));
        list.add_entry(row) as i32
    }
}

impl ModList for PresetsList {
    /// java: @Override public int getRowWidth()
    fn get_row_width(&self, _list: &AbstractSelectionList, _super_answer: i32) -> i32 {
        return 360;
    }

    /// The remove button's `_ -> presetsList.removeEntry(this)`: the button runs inside its row's click, so the row
    /// marks itself and the list removes it as the click returns.
    fn mouse_clicked(&self, list: &mut ModSelectionList, event: &MouseButtonEvent, double_click: bool, super_mouse_clicked: &mut dyn FnMut(&mut ModSelectionList) -> bool) -> bool {
        let answer = super_mouse_clicked(list);
        let mut i = 0;
        while i < list.list.entry_count() {
            let remove = list.list.entry_object(i).and_then(|slot| slot.with::<PresetEntry, _>(|e| e.remove_requested.swap(false, std::sync::atomic::Ordering::Relaxed))).unwrap_or(false);
            if remove {
                PresetsList::remove_entry(&mut list.list, i);
            } else {
                i += 1;
            }
        }
        answer
    }
}

/// java: private class PresetEntry extends ContainerObjectSelectionList.Entry<PresetEntry>
pub struct PresetEntry {
    /// java: private final EditBox nameBox; (its position in `widgets`)
    pub name_box: usize,
    /// java: private final EditBox linkBox; (its position in `widgets`)
    pub link_box: usize,
    /// java: private final List<AbstractWidget> widgets; - the row's children
    pub widgets: mcrust::net::minecraft::client::gui::screens::screen::Screen,
    /// java: private final LinearLayout layout = LinearLayout.horizontal().spacing(8);
    pub layout: LayoutElement,
    /// the remove button's `presetsList.removeEntry(this)` (see `PresetsList::mouse_clicked`)
    pub remove_requested: Arc<std::sync::atomic::AtomicBool>,
    thread: std::thread::ThreadId,
}

// SAFETY: a row is made and reached on the render thread only - every method checks it (`check`).
unsafe impl Send for PresetEntry {}

impl PresetEntry {
    /// java: private PresetEntry(String name, String link)
    pub fn new(name: &str, link: &str) -> PresetEntry {
        let mut widgets = mcrust::net::minecraft::client::gui::screens::screen::Screen::new(String::new());
        let mut layout = LinearLayout::horizontal().spacing(8);
        // java: nameBox = layout.addChild(new EditBox(font, Component.empty())); - 150 x 20
        let mut name_box = compat::edit_box(0, 0, 150, 20, component::empty());
        name_box.set_width(100);
        compat::set_edit_box_value(&mut name_box, name);
        compat::with_edit_box(&mut name_box, |e| {
            e.set_max_length(64);
            e.set_hint_component(FormattedText::from(component::translatable("skyblocker.config.farming.greenhouse.greenhousePresets.name")));
        });
        let name_index = widgets.add_widget(name_box);
        layout.add_child(LayoutElement::widget(name_index, 100, 20));
        // java: linkBox = layout.addChild(new EditBox(font, Component.empty()));
        let mut link_box = compat::edit_box(0, 0, 150, 20, component::empty());
        link_box.set_width(180);
        compat::with_edit_box(&mut link_box, |e| e.set_max_length(2048));
        compat::set_edit_box_value(&mut link_box, link);
        compat::with_edit_box(&mut link_box, |e| e.set_hint_component(FormattedText::from(component::translatable("skyblocker.config.farming.greenhouse.greenhousePresets.link"))));
        let link_index = widgets.add_widget(link_box);
        layout.add_child(LayoutElement::widget(link_index, 180, 20));
        // java: layout.addChild(Button.builder(Component.translatable("gui.remove"), _ -> presetsList.removeEntry(this)).width(80).build());
        let remove_requested = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let request = remove_requested.clone();
        let remove = button::builder(component::translatable("gui.remove").get_string(), OnPress::Lambda(PressLambda::new(move |_screen, _index| {
            request.store(true, std::sync::atomic::Ordering::Relaxed);
        })))
        .width(80)
        .build();
        let (remove_w, remove_h) = (remove.get_width(), remove.get_height());
        let remove_index = widgets.add_widget(remove);
        layout.add_child(LayoutElement::widget(remove_index, remove_w, remove_h));
        // java: widgets = new ArrayList<>(); layout.arrangeElements(); layout.visitWidgets(widgets::add);
        let mut layout = LayoutElement::Grid(layout.wrapped);
        layout.arrange_elements();
        let mut placed = Vec::new();
        layout.visit_widgets(&mut placed);
        for (i, x, y) in placed {
            widgets.widget_mut(i).set_position(x, y);
            widgets.mark_renderable(i);
        }
        PresetEntry { name_box: name_index, link_box: link_index, widgets, layout, remove_requested, thread: std::thread::current().id() }
    }

    /// java: private PresetEntry()
    pub fn new_0() -> PresetEntry {
        Self::new("", "")
    }

    fn check(&self) {
        assert!(std::thread::current().id() == self.thread, "a mod list row was reached off the render thread");
    }

    /// `nameBox.getValue()`
    pub fn name_value(&self) -> String {
        compat::edit_box_value(self.widgets.widget(self.name_box))
    }

    /// `linkBox.getValue()`
    pub fn link_value(&self) -> String {
        compat::edit_box_value(self.widgets.widget(self.link_box))
    }
}

impl ModListEntry for PresetEntry {
    /// java: @Override public void extractContent(GuiGraphicsExtractor graphics, int mouseX, int mouseY, boolean hovered, float a)
    fn extract_content(&mut self, list: &mut ModSelectionList, index: usize, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, _hovered: bool, _a: f32) {
        self.check();
        let row = list.list.entry(index);
        // java: layout.setPosition(getContentXMiddle() - layout.getWidth() / 2, getContentYMiddle() - layout.getHeight() / 2);
        let (x, y) = (row.get_content_x_middle() - self.layout.get_width() / 2, row.get_content_y_middle() - self.layout.get_height() / 2);
        self.layout.set_position(x, y);
        let mut placed = Vec::new();
        self.layout.visit_widgets(&mut placed);
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

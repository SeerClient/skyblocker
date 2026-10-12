//! Ported from de.hysky.skyblocker.config.screens.powdertracker.ItemTickList by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::mcrust::modding::widgets::ListKind;
use mcrust::mcrust::modding::widgets::ModList;
use mcrust::mcrust::modding::widgets::ModListEntry;
use mcrust::mcrust::modding::widgets::ModListEntrySlot;
use mcrust::mcrust::modding::widgets::ModSelectionList;
use mcrust::mcrust::modding::widgets::ModWidgetSlot;
use mcrust::mcrust::modding::widgets::WidgetRenderContext;
use mcrust::net::minecraft::client::gui::components::abstract_selection_list::AbstractSelectionList;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::WidgetKind;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::key_event::KeyEvent;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::formatted_text::FormattedText;

/// java: public class ItemTickList<T> extends ContainerObjectSelectionList<ItemTickList.ItemTickEntry>
///
/// The list's class: the game's list widget (`ModSelectionList`) holds this object, and so does whoever made it
/// (the same `Arc`), so the owner can call `init`/`clearAndInit` on the widget's list (`ModWidgetSlot::with`).
pub struct ItemTickList<T> {
    /// java: private final Collection<T> filters; - the CALLER's collection (the checkboxes write into it)
    pub filters: compat::RenderThread<compat::Shared<Vec<T>>>,
    /// java: private final Collection<T> allItems;
    pub all_items: Vec<T>,
    /// java: private final boolean whitelist;
    pub whitelist: bool,
}

impl<T: Clone + PartialEq + std::fmt::Display + Send + Sync + 'static> ItemTickList<T> {
    /// java: public ItemTickList(Minecraft minecraftClient, int width, int height, int y, int entryHeight, Collection<T> filters, Collection<T> allItems)
    /// - the object and its widget
    pub fn new(width: i32, height: i32, y: i32, entry_height: i32, filters: compat::Shared<Vec<T>>, all_items: Vec<T>) -> (Arc<Self>, AbstractWidget) {
        // java: this.whitelist = false;
        Self::new_8(width, height, y, entry_height, filters, all_items, false)
    }

    /// java: public ItemTickList(Minecraft minecraftClient, int width, int height, int y, int entryHeight, Collection<T> filters, Collection<T> allItems, boolean whitelist)
    pub fn new_8(width: i32, height: i32, y: i32, entry_height: i32, filters: compat::Shared<Vec<T>>, all_items: Vec<T>, whitelist: bool) -> (Arc<Self>, AbstractWidget) {
        let this = Arc::new(ItemTickList { filters: compat::RenderThread::new(filters), all_items, whitelist });
        // java: super(minecraftClient, width, height, y, entryHeight);
        let list = ModSelectionList::with_class(ListKind::Container, width, height, y, entry_height, this.clone());
        let (x, ly, w, h) = (list.list.get_x(), list.list.get_y(), list.list.get_width(), list.list.get_height());
        (this, ModWidgetSlot::widget(x, ly, w, h, String::new(), Box::new(list)))
    }

    /// java: public void clearAndInit()
    pub fn clear_and_init(&self, list: &mut AbstractSelectionList) {
        list.clear_entries();
        self.init(list);
    }

    /// java: public ItemTickList<T> init()
    pub fn init(&self, list: &mut AbstractSelectionList) {
        for item in self.all_items.iter() {
            // java: .selected(whitelist == filters.contains(item))
            let contains = compat::read(self.filters.get(), |filters| filters.contains(item));
            let filters = compat::RenderThread::new(self.filters.get().clone());
            let whitelist = self.whitelist;
            let on_item = item.clone();
            // java: .onValueChange((_, checked) -> { .. })
            let on_value_change = move |checked: bool| {
                let filters = filters.get();
                if whitelist {
                    if checked {
                        compat::write(filters, |f| f.push(on_item.clone()));
                    } else {
                        compat::write(filters, |f| { compat::remove_item(f, &on_item); });
                    }
                } else {
                    if checked {
                        compat::write(filters, |f| { compat::remove_item(f, &on_item); });
                    } else {
                        compat::write(filters, |f| f.push(on_item.clone()));
                    }
                }
            };
            // java: Checkbox.builder(Component.nullToEmpty(item.toString()), minecraft.font)....build()
            let entry = ItemTickEntry::new(compat::checkbox(FormattedText::from(component::null_to_empty(Some(&compat::jstring(item)))), 0, 0, None, whitelist == contains, on_value_change));
            // java: addEntry(entry);
            let mut row = mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry::new(0);
            row.object = Some(ModListEntrySlot::new(Box::new(entry)));
            list.add_entry(row);
        }
    }
}

impl<T: Clone + PartialEq + std::fmt::Display + Send + Sync + 'static> ModList for ItemTickList<T> {}

/// java: public static class ItemTickEntry extends ContainerObjectSelectionList.Entry<ItemTickEntry>
pub struct ItemTickEntry {
    /// java: private final List<Checkbox> children; - `List.of(checkboxWidget)`
    pub children: mcrust::net::minecraft::client::gui::screens::screen::Screen,
    thread: std::thread::ThreadId,
}

// SAFETY: a row is made and reached on the render thread only - every method checks it (`check`).
unsafe impl Send for ItemTickEntry {}

impl ItemTickEntry {
    /// java: ItemTickEntry(Checkbox checkboxWidget)
    pub fn new(checkbox_widget: AbstractWidget) -> ItemTickEntry {
        let mut children = mcrust::net::minecraft::client::gui::screens::screen::Screen::new(String::new());
        children.add_renderable_widget(checkbox_widget);
        ItemTickEntry { children, thread: std::thread::current().id() }
    }

    fn check(&self) {
        assert!(std::thread::current().id() == self.thread, "a mod list row was reached off the render thread");
    }

    /// java: public void setChecked(boolean checked)
    pub fn set_checked(&mut self, checked: bool) {
        self.check();
        for i in 0..self.children.children().len() {
            // java: ((CheckboxAccessor) child).setSelected(checked);
            if let WidgetKind::Checkbox(c) = &mut self.children.widget_mut(i).kind {
                c.selected = checked;
            }
        }
    }
}

impl ModListEntry for ItemTickEntry {
    /// java: @Override public void extractContent(GuiGraphicsExtractor graphics, int mouseX, int mouseY, boolean hovered, float deltaTicks)
    fn extract_content(&mut self, list: &mut ModSelectionList, index: usize, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, _hovered: bool, _delta_ticks: f32) {
        self.check();
        let row = list.list.entry(index);
        let (x, y, width, height) = (row.get_x(), row.get_y(), row.get_width(), row.get_height());
        for i in 0..self.children.children().len() {
            let child = self.children.widget_mut(i);
            child.set_x(x);
            child.set_y(y);
            child.set_width(width);
            child.set_height(height);
        }
        // java: child.extractRenderState(graphics, mouseX, mouseY, deltaTicks);
        context.extract_widgets(&mut self.children, mouse_x, mouse_y);
    }

    /// java: @Override public List<? extends GuiEventListener> children() (and `narratables()`, the same list)
    fn children(&mut self) -> Option<&mut mcrust::net::minecraft::client::gui::screens::screen::Screen> {
        Some(&mut self.children)
    }

    /// `ContainerEventHandler.keyPressed` - the focused child
    fn key_pressed(&mut self, _list: &mut ModSelectionList, _index: usize, event: &KeyEvent) -> bool {
        self.check();
        compat::children_key_pressed(&mut self.children, event)
    }

    /// `ContainerEventHandler.charTyped` - the focused child
    fn char_typed(&mut self, _list: &mut ModSelectionList, _index: usize, event: &CharacterEvent) -> bool {
        self.check();
        compat::children_char_typed(&mut self.children, event)
    }
}

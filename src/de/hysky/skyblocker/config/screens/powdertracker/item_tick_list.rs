//! Ported from de.hysky.skyblocker.config.screens.powdertracker.ItemTickList by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::mcrust::modding::widgets::ModListEntry;
use mcrust::mcrust::modding::widgets::ModSelectionList;
use mcrust::mcrust::modding::widgets::WidgetRenderContext;
use mcrust::net::minecraft::client::gui::components::abstract_selection_list::AbstractSelectionList;
use mcrust::net::minecraft::client::gui::components::checkbox::Checkbox;
use mcrust::net::minecraft::client::gui::components::events::gui_event_listener::GuiEventListener;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::narration::narratable_entry::NarratableEntry;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::client::gui::components::renderable::Renderable as _;

/// java: public class ItemTickList<T> extends ContainerObjectSelectionList<ItemTickList.ItemTickEntry>
pub struct ItemTickList<T> {
    /// java: extends net.minecraft.client.gui.components.ContainerObjectSelectionList
    pub base: ModSelectionList,
    /// java: private final Collection<T> filters;
    pub filters: Vec<T>,
    /// java: private final Collection<T> allItems;
    pub all_items: Vec<T>,
    /// java: private final boolean whitelist;
    pub whitelist: bool,
}

impl<T: 'static> std::ops::Deref for ItemTickList<T> {
    type Target = AbstractSelectionList;
    fn deref(&self) -> &Self::Target {
        &self.base.list
    }
}

impl<T: 'static> std::ops::DerefMut for ItemTickList<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base.list
    }
}

impl<T: Clone + 'static> ItemTickList<T> {
    /// java: public ItemTickList(Minecraft minecraftClient, int width, int height, int y, int entryHeight, Collection<T> filters, Collection<T> allItems)
    pub fn new(minecraft_client: &mut mcrust::net::minecraft::client::minecraft::Minecraft, mut width: i32, mut height: i32, mut y: i32, mut entry_height: i32, mut filters: Vec<T>, mut all_items: Vec<T>) -> compat::Shared<Self> {
        let __init_filters = filters.clone();
        let __init_all_items = all_items.clone();
        let __init_whitelist = false;
        let __this = compat::shared(Self {
            base: mcrust::mcrust::modding::widgets::ModSelectionList::new(mcrust::mcrust::modding::widgets::ListKind::Container, width, height, y, entry_height),
            filters: __init_filters,
            all_items: __init_all_items,
            whitelist: __init_whitelist,
        });
        __this
    }

    /// java: public ItemTickList(Minecraft minecraftClient, int width, int height, int y, int entryHeight, Collection<T> filters, Collection<T> allItems, boolean whitelist)
    pub fn new_8(minecraft_client: &mut mcrust::net::minecraft::client::minecraft::Minecraft, mut width: i32, mut height: i32, mut y: i32, mut entry_height: i32, mut filters: Vec<T>, mut all_items: Vec<T>, mut whitelist: bool) -> compat::Shared<Self> {
        let __init_filters = filters.clone();
        let __init_all_items = all_items.clone();
        let __init_whitelist = whitelist;
        let __this = compat::shared(Self {
            base: mcrust::mcrust::modding::widgets::ModSelectionList::new(mcrust::mcrust::modding::widgets::ListKind::Container, width, height, y, entry_height),
            filters: __init_filters,
            all_items: __init_all_items,
            whitelist: __init_whitelist,
        });
        __this
    }

    /// java: public void clearAndInit()
    pub fn clear_and_init(__this: &compat::Shared<Self>) {
        compat::write(__this, |__t| { let __l = &mut __t.base; __l.clear_entries() });
        ItemTickList::init(__this);
    }

    /// java: public ItemTickList<T> init()
    pub fn init(__this: &compat::Shared<Self>) -> compat::Shared<ItemTickList<T>> {
        for item in compat::read(__this, |__t| __t.all_items.clone()) {
            // TODO(fabric2seer): static `Checkbox.builder` has no Rust counterpart in the port
            // TODO(fabric2seer): `Builder.selected` has no Rust method in the port (looked for `selected` in Checkbox and its supertypes)
            // TODO(fabric2seer): `Builder.onValueChange` has no Rust method in the port (looked for `on_value_change` in Checkbox and its supertypes)
            // TODO(fabric2seer): `Builder.build` has no Rust method in the port (looked for `build` in Checkbox and its supertypes)
            // rust: let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
            // rust: let mut entry: compat::Shared<ItemTickEntry> = ItemTickEntry::new((Checkbox::builder(component::null_to_empty(Some(&compat::jstring(&item))), compat::font())).selected((compat::read(__this, |__t| __t.whitelist) == __this.lock().borrow().filters.iter().any(|e| *e == item.clone()))).on_value_change(&Arc::new(move |_, checked| { let Some(__this_1) = __this_w1.get().upgrade() else { return Default::default(); }; {
            // rust: if compat::read((&__this_1), |__t| __t.whitelist) {
            // rust: if checked {
            // rust: let __h0 = item.clone();
            // rust: (&__this_1).lock().borrow_mut().filters.push(__h0);
            // rust: } else {
            // rust: compat::remove_item(&mut compat::read((&__this_1), |__t| __t.filters.clone()), &item.clone());
            // rust: }
            // rust: } else {
            // rust: if checked {
            // rust: compat::remove_item(&mut compat::read((&__this_1), |__t| __t.filters.clone()), &item.clone());
            // rust: } else {
            // rust: let __h0 = item.clone();
            // rust: (&__this_1).lock().borrow_mut().filters.push(__h0);
            // rust: }
            // rust: }
            // rust: } })).build());
            let mut entry: compat::Shared<ItemTickEntry> = todo!("fabric2seer: the initialiser of `entry` (TODO above)");
            let __entry = entry.clone();
            compat::write(__this, |__t| { let __l = &mut __t.base; (__l.add_entry(Box::new(compat::EntryAdapter::new(__entry))) as i32) });
        }
        return __this.clone();
    }

}

/// java: public static class ItemTickEntry extends ContainerObjectSelectionList.Entry<ItemTickEntry>
pub struct ItemTickEntry {
    /// java: extends net.minecraft.client.gui.components.ContainerObjectSelectionList$Entry
    pub base: mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry,
    /// java: private final List<Checkbox> children;
    pub children: Vec<Checkbox>,
    /// the row's widgets (`children()`): lent to the object by its `compat::EntryAdapter`
    pub __row: mcrust::net::minecraft::client::gui::screens::screen::Screen,
    /// the screen's widgets as this object last saw them (see `compat::WidgetCache`)
    pub __widgets: compat::WidgetCache,
}

impl std::ops::Deref for ItemTickEntry {
    type Target = mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for ItemTickEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl ItemTickEntry {
    /// java: ItemTickEntry(Checkbox checkboxWidget)
    pub fn new(mut checkbox_widget: Checkbox) -> compat::Shared<Self> {
        let __init_children = vec![checkbox_widget.clone()];
        let __this = compat::shared(Self {
            base: mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry::new(0),
            __widgets: Default::default(),
            __row: mcrust::net::minecraft::client::gui::screens::screen::Screen::new(String::new()),
            children: __init_children,
        });
        __this
    }

    /// java: public void setChecked(boolean checked)
    pub fn set_checked(__this: &compat::Shared<Self>, mut checked: bool) {
        for child in compat::read(__this, |__t| __t.children.clone()) {
            child.selected = checked;
        }
    }

    /// java: @Override public void extractContent(GuiGraphicsExtractor graphics, int mouseX, int mouseY, boolean hovered, float deltaTicks)
    pub fn extract_content(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut hovered: bool, mut delta_ticks: f32) {
        for child in compat::read(__this, |__t| __t.children.clone()) {
            // TODO(fabric2seer): `AbstractWidget::set_x` ports `setX`, but the port's `Checkbox` holds no `AbstractWidget` (no Deref, no `base` field) to call it on
            // rust: child.set_x(__this.lock().borrow().get_x());
            // TODO(fabric2seer): `AbstractWidget::set_y` ports `setY`, but the port's `Checkbox` holds no `AbstractWidget` (no Deref, no `base` field) to call it on
            // rust: child.set_y(__this.lock().borrow().get_y());
            // TODO(fabric2seer): `AbstractWidget::set_width` ports `setWidth`, but the port's `Checkbox` holds no `AbstractWidget` (no Deref, no `base` field) to call it on
            // rust: child.set_width(__this.lock().borrow().get_width());
            // TODO(fabric2seer): `AbstractWidget::set_height` ports `setHeight`, but the port's `Checkbox` holds no `AbstractWidget` (no Deref, no `base` field) to call it on
            // rust: child.set_height(__this.lock().borrow().get_height());
            // TODO(fabric2seer): `extract_render_state` is a method of the port's trait `Renderable`, which the port's `Checkbox` does not implement
            // rust: child.extract_render_state(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mouse_x, mouse_y, delta_ticks);
        }
    }

    /// java: @Override public List<? extends NarratableEntry> narratables()
    pub fn narratables(__this: &compat::Shared<Self>) -> Vec<Box<dyn NarratableEntry>> {
        return compat::read(__this, |__t| __t.children.clone());
    }

    /// java: @Override public List<? extends GuiEventListener> children()
    pub fn children(__this: &compat::Shared<Self>) -> Vec<Box<dyn GuiEventListener>> {
        return compat::read(__this, |__t| __t.children.clone());
    }

}

/// ItemTickEntry's row box and its widgets, lent by its `compat::EntryAdapter` for each call
impl compat::RowHalf for ItemTickEntry {
    fn entry_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry {
        &mut self.base
    }
    fn row_parts(&mut self) -> (&mut mcrust::net::minecraft::client::gui::screens::screen::Screen, &mut compat::WidgetCache) {
        let __s = self;
        (&mut __s.__row, &mut __s.__widgets)
    }
}

/// ItemTickEntry as a row of the game's list (`addEntry` attaches it)
impl ModListEntry for compat::EntryAdapter<ItemTickEntry> {
    /// java: @Override public void extractContent(GuiGraphicsExtractor graphics, int mouseX, int mouseY, boolean hovered, float deltaTicks)
    fn extract_content(&mut self, list: &mut ModSelectionList, index: usize, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, hovered: bool, a: f32) {
        self.lend(Some(list.list.entry(index)), |__h| ItemTickEntry::extract_content(__h, &context.graphics, mouse_x, mouse_y, hovered, a));
        // the row's widgets (Java's `child.extractRenderState(..)` over `children()`)
        context.extract_widgets(&mut self.row, mouse_x, mouse_y);
    }

    fn children(&mut self) -> Option<&mut mcrust::net::minecraft::client::gui::screens::screen::Screen> {
        Some(&mut self.row)
    }

}

// NOTE(fabric2seer): children, extractContent, narratables override(s) of Entry - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them

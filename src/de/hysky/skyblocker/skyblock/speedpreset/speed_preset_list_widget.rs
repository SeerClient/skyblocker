//! Ported from de.hysky.skyblocker.skyblock.speedpreset.SpeedPresetListWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::speedpreset::speed_presets::SpeedPresets;
use crate::de::hysky::skyblocker::utils::render::gui::filtered_edit_box::FilteredEditBox;
use crate::de::hysky::skyblocker::utils::render::gui::filtered_edit_box::FilteredEditBoxBase;
use mcrust::mcrust::modding::widgets::ModList;
use mcrust::mcrust::modding::widgets::ModListEntry;
use mcrust::mcrust::modding::widgets::ModSelectionList;
use mcrust::mcrust::modding::widgets::WidgetRenderContext;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::PressLambda;
use mcrust::net::minecraft::client::gui::components::button;
use mcrust::net::minecraft::client::gui::components::abstract_selection_list::AbstractSelectionList;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::container_object_selection_list;
use mcrust::net::minecraft::client::gui::components::events::gui_event_listener::GuiEventListener;
use mcrust::net::minecraft::client::gui::components::renderable::Renderable;
use mcrust::net::minecraft::client::gui::font::Font;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::layouts::layout_element::LayoutElement;
use mcrust::net::minecraft::client::gui::layouts::linear_layout::LinearLayout;
use mcrust::net::minecraft::client::gui::narration::narratable_entry::NarratableEntry;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final Pattern NUMBER = Pattern.compile("^-?\\d+(\\.\\d+)?$");
pub static NUMBER: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("^-?\\d+(\\.\\d+)?$"));
/// java: private static final Pattern TITLE = Pattern.compile("^[a-zA-Z]\\w*$");
pub static TITLE: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("^[a-zA-Z]\\w*$"));

/// java: public class SpeedPresetListWidget extends ContainerObjectSelectionList<SpeedPresetListWidget.AbstractEntry>
pub struct SpeedPresetListWidget {
    /// java: extends net.minecraft.client.gui.components.ContainerObjectSelectionList
    pub base: ModSelectionList,
}

impl std::ops::Deref for SpeedPresetListWidget {
    type Target = AbstractSelectionList;
    fn deref(&self) -> &Self::Target {
        &self.base.list
    }
}

impl std::ops::DerefMut for SpeedPresetListWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base.list
    }
}

impl SpeedPresetListWidget {
    /// java: public SpeedPresetListWidget(int width, int height, int y)
    pub fn new(mut width: i32, mut height: i32, mut y: i32) -> compat::Shared<Self> {
        let mut instance: SpeedPresets = SpeedPresets::get_instance();
        let __this = compat::shared(Self {
            base: mcrust::mcrust::modding::widgets::ModSelectionList::new(mcrust::mcrust::modding::widgets::ListKind::Container, width, height, y, 25),
        });
        let __entry = (TitleEntry::new() as compat::Shared<dyn AbstractEntry>);
        compat::write(&__this, |__t| { let __l = &mut __t.base; (__l.add_entry(Box::new(compat::EntryAdapter::new(__entry))) as i32) });
        if !(instance.get_presets().is_empty()) {
            for (title, speed) in instance.get_presets().iter() {
                let __entry = (SpeedPresetEntry::new(title.to_string(), speed.to_string()) as compat::Shared<dyn AbstractEntry>);
                compat::write(&__this, |__t| { let __l = &mut __t.base; (__l.add_entry(Box::new(compat::EntryAdapter::new(__entry))) as i32) });
            }
        } else {
            let __entry = (SpeedPresetEntry::new("".to_string(), "".to_string()) as compat::Shared<dyn AbstractEntry>);
            compat::write(&__this, |__t| { let __l = &mut __t.base; (__l.add_entry(Box::new(compat::EntryAdapter::new(__entry))) as i32) });
        }
        __this
    }

    /// java: @Override public int getRowWidth()
    pub fn get_row_width(__this: &compat::Shared<Self>) -> i32 {
        return 190;
    }

    /// java: public boolean hasBeenChanged()
    pub fn has_been_changed(__this: &compat::Shared<Self>) -> bool {
        let mut instance: SpeedPresets = SpeedPresets::get_instance();
        if ((compat::write(__this, |__t| { let __l = &mut __t.base; compat::list_entries::<dyn AbstractEntry>(__l) })).len() as i32) < (instance.get_presets().len() as i32) {
            return true;
        }
        // java: var childrenMap = this.children().stream().filter(SpeedPresetEntry.class::isInstance).map(SpeedPresetEntry.class::cast)
        //   .map(SpeedPresetEntry::getMapping).filter(Objects::nonNull).collect(Collectors.toMap(ObjectIntPair::key, ObjectIntPair::valueInt));
        let mut mappings: Vec<(String, i32)> = Vec::new();
        compat::write(__this, |__t| with_speed_preset_entries(&mut __t.base, &mut |entry, row| {
            if let Some(mapping) = SpeedPresetEntry::get_mapping(entry, row) {
                mappings.push(mapping);
            }
        }));
        let mut children_map: std::collections::HashMap<String, i32> = std::collections::HashMap::new();
        for (key, value) in mappings {
            // Collectors.toMap refuses a duplicate key
            if children_map.insert(key.clone(), value).is_some() {
                panic!("java.lang.IllegalStateException: Duplicate key {key}");
            }
        }
        return !(instance.are_presets_equal(&children_map));
    }

    /// java: public void updatePosition()
    pub fn update_position(__this: &compat::Shared<Self>) {
        // java: children().forEach(AbstractEntry::updatePosition);
        compat::write(__this, |__t| {
            let count: usize = __t.base.list.entry_count();
            for i in 0..count {
                update_entry_position(&mut __t.base, i);
            }
        });
    }

    /// java: public void newEntry()
    pub fn new_entry(__this: &compat::Shared<Self>) {
        let mut entry: compat::Shared<SpeedPresetEntry> = SpeedPresetEntry::new("".to_string(), "".to_string());
        let __entry = (entry.clone() as compat::Shared<dyn AbstractEntry>);
        compat::write(__this, |__t| { let __l = &mut __t.base; (__l.add_entry(Box::new(compat::EntryAdapter::new(__entry))) as i32) });
        // java: entry.updatePosition(); - through its adapter, which holds the row's widgets
        { let __e = (entry.clone() as compat::Shared<dyn AbstractEntry>); compat::write(__this, |__t| { if let Some(__i) = compat::entry_index::<dyn AbstractEntry>(&__t.base, &__e) { update_entry_position(&mut __t.base, __i); } }); }
        let __entry = Some((entry.clone() as compat::Shared<dyn AbstractEntry>));
        // java: this.centerScrollOn(entry);
        compat::write(__this, |__t| { let __l = &mut __t.base; if let Some(__i) = __entry.as_ref().and_then(|__e| compat::entry_index::<dyn AbstractEntry>(__l, __e)) { __l.list.center_scroll_on(__i); } });
        compat::write(__this, |__t| { let __l = &mut __t.base; { let __i = __entry.as_ref().and_then(|__e| compat::entry_index::<dyn AbstractEntry>(__l, __e)); __l.set_selected(__i); } });
        // java: this.setFocused(entry); - ContainerObjectSelectionList.setFocused:76-83: `super.setFocused(entry)`, then the
        // selection is cleared only when the focus changed to null
        compat::write(__this, |__t| {
            let __l = &mut __t.base;
            let __i = __entry.as_ref().and_then(|__e| compat::entry_index::<dyn AbstractEntry>(__l, __e));
            let __changed = __l.list.get_focused() != __i;
            __l.list.set_focused(__i, false);
            if mcrust::net::minecraft::client::gui::components::container_object_selection_list::set_focused_clears_selection(__changed, __i) {
                __l.set_selected(None);
            }
        });
    }

    /// java: public void save()
    pub fn save(__this: &compat::Shared<Self>) {
        let mut instance: SpeedPresets = SpeedPresets::get_instance();
        // java: instance.getPresets().clear(); - the JsonData's own map (`get_presets` answers a copy: the cleared map is put back)
        let mut presets: std::collections::HashMap<String, i32> = instance.get_presets();
        presets.clear();
        instance.presets.lock().borrow_mut().set_data(presets);
        // java: children().stream().filter(SpeedPresetEntry.class::isInstance).map(SpeedPresetEntry.class::cast).forEach(SpeedPresetEntry::save);
        compat::write(__this, |__t| with_speed_preset_entries(&mut __t.base, &mut |entry, row| SpeedPresetEntry::save(entry, row)));
        instance.save_presets(); // Write down the changes.
    }

}

/// SpeedPresetListWidget's list (the port's `ModSelectionList`) until the game takes it (`compat::attach_list`)
impl compat::ListHalf for SpeedPresetListWidget {
    fn list_half(&mut self) -> &mut ModSelectionList {
        &mut self.base
    }
}

/// SpeedPresetListWidget's overrides of the list's own methods, asked by the port's list bodies
impl ModList for compat::ListClass<SpeedPresetListWidget> {
    /// java: @Override public int getRowWidth()
    fn get_row_width(&self, list: &AbstractSelectionList, super_answer: i32) -> i32 {
        self.ask(list, super_answer, |__h| SpeedPresetListWidget::get_row_width(__h))
    }

    /// The remove button's `_ -> SpeedPresetListWidget.this.removeEntry(this)`: the button runs inside its row's click,
    /// so the row marks itself and the list removes it as the click returns.
    fn mouse_clicked(&self, list: &mut ModSelectionList, event: &MouseButtonEvent, double_click: bool, super_mouse_clicked: &mut dyn FnMut(&mut ModSelectionList) -> bool) -> bool {
        let _ = (event, double_click);
        let answer = super_mouse_clicked(list);
        let mut i = 0;
        while i < list.list.entry_count() {
            let remove: bool = list
                .list
                .entry_object(i)
                .and_then(|slot| slot.with::<compat::EntryAdapter<dyn AbstractEntry>, _>(|adapter| adapter.object().clone()))
                .and_then(|entry| compat::try_view::<SpeedPresetEntry, _>(&entry).map(|e| compat::read(&*e, |__t| __t.remove_requested.swap(false, std::sync::atomic::Ordering::Relaxed))))
                .unwrap_or(false);
            if remove {
                // java: removeEntry(entry)
                list.list.remove_entry(i);
            } else {
                i += 1;
            }
        }
        answer
    }

}

// NOTE(fabric2seer): getRowWidth override(s) of ContainerObjectSelectionList - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them

/// java: public abstract static class AbstractEntry extends ContainerObjectSelectionList.Entry<AbstractEntry>
pub struct AbstractEntryBase {
    /// java: extends net.minecraft.client.gui.components.ContainerObjectSelectionList$Entry
    pub base: mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry,
    /// the row's widgets (`children()`): lent to the object by its `compat::EntryAdapter`
    pub __row: mcrust::net::minecraft::client::gui::screens::screen::Screen,
    /// the screen's widgets as this object last saw them (see `compat::WidgetCache`)
    pub __widgets: compat::WidgetCache,
}

impl std::ops::Deref for AbstractEntryBase {
    type Target = mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for AbstractEntryBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl AbstractEntryBase {
    /// java: the implicit no-argument constructor (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_in<__R>(__wrap: impl FnOnce(Self) -> (compat::Shared<dyn AbstractEntry>, __R)) -> (compat::Shared<dyn AbstractEntry>, __R) {
        let this = Self {
            base: mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry::new(0),
            __widgets: Default::default(),
            __row: mcrust::net::minecraft::client::gui::screens::screen::Screen::new(String::new()),
        };
        let (__h, __r) = __wrap(this);
        let __this = __h;
        (__this, __r)
    }

}

/// java: the virtual methods of AbstractEntry (implemented by AbstractEntryBase and every subclass)
pub trait AbstractEntry: compat::DynAny {
    fn abstract_entry_base(&self) -> &AbstractEntryBase;
    fn abstract_entry_base_mut(&mut self) -> &mut AbstractEntryBase;
    fn as_any(&self) -> &dyn std::any::Any;

    /// java: protected void updatePosition() (dispatch: the body for this class)
    fn update_position(&self) -> fn(&compat::Shared<dyn AbstractEntry>) {
        <dyn AbstractEntry>::update_position_body
    }

    /// java: @Override public void extractContent(GuiGraphicsExtractor graphics, int mouseX, int mouseY, boolean hovered, float deltaTicks) (dispatch: the body for this class)
    fn extract_content(&self) -> fn(&compat::Shared<dyn AbstractEntry>, &GuiGraphicsExtractor, i32, i32, bool, f32) {
        <dyn AbstractEntry>::extract_content_body
    }

}

impl dyn AbstractEntry {
    /// java: protected void updatePosition()
    pub fn update_position_body(__this: &compat::Shared<dyn AbstractEntry>) {
    }

    /// java: @Override public void extractContent(GuiGraphicsExtractor graphics, int mouseX, int mouseY, boolean hovered, float deltaTicks)
    pub fn extract_content_body(__this: &compat::Shared<dyn AbstractEntry>, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut hovered: bool, mut delta_ticks: f32) {
        // java: for (var child : children()) { if (child instanceof LayoutElement widget) widget.setY(getY());
        //         if (child instanceof Renderable drawable) drawable.extractRenderState(graphics, mouseX, mouseY, deltaTicks); }
        // The row's widgets are the row screen the adapter lends the game (`children()`), which draws each of them after
        // this call: the loop's draw half is the game's; its `setY` half is here.
        compat::write(__this, |__t| {
            let __base = __t.abstract_entry_base_mut();
            let __y = __base.base.get_y();
            for __widget in __base.__row.children_mut() {
                __widget.set_y(__y);
            }
        });
    }

}

/// AbstractEntry's row box and its widgets, lent by its `compat::EntryAdapter` for each call
impl compat::RowHalf for dyn AbstractEntry {
    fn entry_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry {
        &mut self.abstract_entry_base_mut().base
    }
    fn row_parts(&mut self) -> (&mut mcrust::net::minecraft::client::gui::screens::screen::Screen, &mut compat::WidgetCache) {
        let __s = self.abstract_entry_base_mut();
        (&mut __s.__row, &mut __s.__widgets)
    }
}

/// AbstractEntry as a row of the game's list (`addEntry` attaches it)
impl ModListEntry for compat::EntryAdapter<dyn AbstractEntry> {
    /// java: @Override public void extractContent(GuiGraphicsExtractor graphics, int mouseX, int mouseY, boolean hovered, float deltaTicks)
    fn extract_content(&mut self, list: &mut ModSelectionList, index: usize, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, hovered: bool, a: f32) {
        // `TitleEntry` (an inner class) reads the enclosing list's `width`
        let list_width: i32 = list.list.get_width();
        if let Some(__c) = compat::try_view::<TitleEntry, _>(self.object()) {
            compat::write(&*__c, |__t| __t.list_width = list_width);
        }
        self.lend(Some(list.list.entry(index)), |__h| { if let Some(__c) = compat::try_view::<SpeedPresetEntry, _>(__h) { return ({ let __f = compat::dispatch(&*__c, |__t| __t.extract_content()); __f(&compat::view(&*__c, |a| a as compat::Shared<dyn AbstractEntry>), &context.graphics, mouse_x, mouse_y, hovered, a) }); } if let Some(__c) = compat::try_view::<TitleEntry, _>(__h) { return ({ let __f = compat::dispatch(&*__c, |__t| __t.extract_content()); __f(&compat::view(&*__c, |a| a as compat::Shared<dyn AbstractEntry>), &context.graphics, mouse_x, mouse_y, hovered, a) }); }  });
        // the row's widgets (Java's `child.extractRenderState(..)` over `children()`)
        context.extract_widgets(&mut self.row, mouse_x, mouse_y);
    }

    fn children(&mut self) -> Option<&mut mcrust::net::minecraft::client::gui::screens::screen::Screen> {
        Some(&mut self.row)
    }

    /// java: `ContainerEventHandler.keyPressed` - the row's focused widget (a `FilteredEditBox` takes only the edits its
    /// filter passes, `filtered_edit`)
    fn key_pressed(&mut self, list: &mut ModSelectionList, index: usize, event: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        let _ = (list, index);
        let filter = compat::try_view::<SpeedPresetEntry, _>(self.object()).and_then(|__c| compat::read(&*__c, |__t| __t.focused_filter(&self.row)));
        match filter {
            Some(filter) => filtered_edit(&mut self.row, filter, |__row| compat::children_key_pressed(__row, event)),
            None => compat::children_key_pressed(&mut self.row, event),
        }
    }

    /// java: `ContainerEventHandler.charTyped` - the row's focused widget (see `key_pressed`)
    fn char_typed(&mut self, list: &mut ModSelectionList, index: usize, event: &CharacterEvent) -> bool {
        let _ = (list, index);
        let filter = compat::try_view::<SpeedPresetEntry, _>(self.object()).and_then(|__c| compat::read(&*__c, |__t| __t.focused_filter(&self.row)));
        match filter {
            Some(filter) => filtered_edit(&mut self.row, filter, |__row| compat::children_char_typed(__row, event)),
            None => compat::children_char_typed(&mut self.row, event),
        }
    }
}

/// java: public class TitleEntry extends AbstractEntry
pub struct TitleEntry {
    /// java: extends AbstractEntry
    pub base: AbstractEntryBase,
    /// java: `SpeedPresetListWidget.this.width` (the enclosing list's, read by the inner class) as the list last drew it
    pub list_width: i32,
}

impl std::ops::Deref for TitleEntry {
    type Target = AbstractEntryBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for TitleEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl TitleEntry {
    /// java: the implicit no-argument constructor
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = AbstractEntryBase::new_in(move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                list_width: 0,
            });
            ((__h.clone() as compat::Shared<dyn AbstractEntry>), __h)
        });
        __this
    }

    /// java: @Override public List<? extends NarratableEntry> narratables()
    pub fn narratables(__this: &compat::Shared<Self>) -> Vec<Box<dyn NarratableEntry>> {
        return vec![];
    }

    /// java: @Override public List<? extends GuiEventListener> children()
    pub fn children(__this: &compat::Shared<Self>) -> Vec<Box<dyn GuiEventListener>> {
        return vec![];
    }

}

impl AbstractEntry for TitleEntry {
    fn abstract_entry_base(&self) -> &AbstractEntryBase {
        &self.base
    }
    fn abstract_entry_base_mut(&mut self) -> &mut AbstractEntryBase {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void extractContent(GuiGraphicsExtractor graphics, int mouseX, int mouseY, boolean hovered, float deltaTicks) (dispatch: the body for this class)
    fn extract_content(&self) -> fn(&compat::Shared<dyn AbstractEntry>, &GuiGraphicsExtractor, i32, i32, bool, f32) {
        TitleEntry::extract_content_dyn
    }

}

impl TitleEntry {
    /// java: @Override public void extractContent(GuiGraphicsExtractor graphics, int mouseX, int mouseY, boolean hovered, float deltaTicks)
    pub fn extract_content_impl(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut hovered: bool, mut delta_ticks: f32) {
        // The line height is 25, the height of a single character is always 9.
        // 25 - 9 = 16, 16 / 2 = 8, therefore the Y-offset should be 8.
        let (width, y) = compat::read(__this, |__t| (__t.list_width, __t.base.base.get_y()));
        let gl = mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl();
        // java: graphics.centeredText(minecraft.font, Component.translatable("skyblocker.config.general.speedPresets.config.title"), width / 2 - 50, this.getY() + 8, CommonColors.WHITE);
        mcrust::net::minecraft::client::gui::gui_graphics_extractor::centered_text_string(gl, &*compat::minecraft(), &component::translatable("skyblocker.config.general.speedPresets.config.title").get_string(), (width.wrapping_div(2)).wrapping_sub(50), y.wrapping_add(8), mcrust::net::minecraft::util::common_colors::WHITE);
        // java: graphics.centeredText(minecraft.font, Component.translatable("skyblocker.config.general.speedPresets.config.speed"), width / 2 + 50, this.getY() + 8, CommonColors.WHITE);
        mcrust::net::minecraft::client::gui::gui_graphics_extractor::centered_text_string(gl, &*compat::minecraft(), &component::translatable("skyblocker.config.general.speedPresets.config.speed").get_string(), (width.wrapping_div(2)).wrapping_add(50), y.wrapping_add(8), mcrust::net::minecraft::util::common_colors::WHITE);
    }

    /// TitleEntry.extractContent as the dispatch of AbstractEntry.extractContent
    pub fn extract_content_dyn(__this: &compat::Shared<dyn AbstractEntry>, __p0: &GuiGraphicsExtractor, __p1: i32, __p2: i32, __p3: bool, __p4: f32) {
        Self::extract_content_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2, __p3, __p4)
    }

}

/// TitleEntry's row box and its widgets, lent by its `compat::EntryAdapter` for each call
impl compat::RowHalf for TitleEntry {
    fn entry_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry {
        &mut self.base.base
    }
    fn row_parts(&mut self) -> (&mut mcrust::net::minecraft::client::gui::screens::screen::Screen, &mut compat::WidgetCache) {
        let __s = &mut self.base;
        (&mut __s.__row, &mut __s.__widgets)
    }
}

/// TitleEntry as a row of the game's list (`addEntry` attaches it)
impl ModListEntry for compat::EntryAdapter<TitleEntry> {
    /// java: @Override public void extractContent(GuiGraphicsExtractor graphics, int mouseX, int mouseY, boolean hovered, float deltaTicks)
    fn extract_content(&mut self, list: &mut ModSelectionList, index: usize, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, hovered: bool, a: f32) {
        self.lend(Some(list.list.entry(index)), |__h| ({ let __f = compat::dispatch(__h, |__t| __t.extract_content()); __f(&compat::view(__h, |a| a as compat::Shared<dyn AbstractEntry>), &context.graphics, mouse_x, mouse_y, hovered, a) }));
        // the row's widgets (Java's `child.extractRenderState(..)` over `children()`)
        context.extract_widgets(&mut self.row, mouse_x, mouse_y);
    }

    fn children(&mut self) -> Option<&mut mcrust::net::minecraft::client::gui::screens::screen::Screen> {
        Some(&mut self.row)
    }

}

/// java: public class SpeedPresetEntry extends AbstractEntry
///
/// The row's three widgets (`children()`: `titleInput`, `speedInput`, `removeButton`) are the game's widgets in the
/// row screen the entry's adapter lends the game (`__row`), at the indices below. The two `FilteredEditBox`es are the
/// game's edit boxes with Java's `FilteredEditBox` filter in front of their edits (`filtered_edit`): the
/// `FilteredEditBox` class (utils/render/gui, another batch's file) has no game-widget adapter.
pub struct SpeedPresetEntry {
    /// java: extends AbstractEntry
    pub base: AbstractEntryBase,
    /// java: protected final FilteredEditBox titleInput; - its index in the row's widgets
    pub title_input: usize,
    /// java: protected final FilteredEditBox speedInput; - its index in the row's widgets
    pub speed_input: usize,
    /// java: protected final Button removeButton; - its index in the row's widgets
    pub remove_button: usize,
    /// java: protected final LinearLayout layout;
    pub layout: LinearLayout,
    /// the remove button's `SpeedPresetListWidget.this.removeEntry(this)`: the button runs inside its row's click, so
    /// the row marks itself and the list removes it as the click returns (`ModList::mouse_clicked`)
    pub remove_requested: Arc<std::sync::atomic::AtomicBool>,
}

impl std::ops::Deref for SpeedPresetEntry {
    type Target = AbstractEntryBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for SpeedPresetEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

/// java: `FilteredEditBox.setFilter(..)`'s filters of the two boxes
fn title_filter(str: &str) -> bool {
    // java: str -> str.isEmpty() || TITLE.matcher(str).matches()
    str.is_empty() || TITLE.matcher(str).matches()
}

fn speed_filter(str: &str) -> bool {
    // java: str -> str.isEmpty() || NUMBER.matcher(str).matches()
    str.is_empty() || NUMBER.matcher(str).matches()
}

impl SpeedPresetEntry {
    /// java: public SpeedPresetEntry(String title, String speed)
    pub fn new(mut title: String, mut speed: String) -> compat::Shared<Self> {
        let mut font: Font = compat::font();
        // java: layout = LinearLayout.horizontal(); layout.spacing(2);
        let mut layout: LinearLayout = LinearLayout::horizontal().spacing(2);
        let mut row = mcrust::net::minecraft::client::gui::screens::screen::Screen::new(String::new());
        // java: this.titleInput = layout.addChild(new FilteredEditBox(font, 120, 20, Component.empty()));
        let mut title_input: AbstractWidget = compat::edit_box(0, 0, 120, 20, component::empty());
        // java: this.titleInput.setFilter(str -> str.isEmpty() || TITLE.matcher(str).matches());
        // java: this.titleInput.setValue(title); - `FilteredEditBox.setValue`: only a value the filter passes
        if title_filter(&title) {
            compat::set_edit_box_value(&mut title_input, &title);
        }
        compat::with_edit_box(&mut title_input, |e| {
            // java: this.titleInput.setMaxLength(16);
            e.set_max_length(16);
            // java: this.titleInput.setHint(Component.literal("newPreset").withStyle(ChatFormatting.DARK_GRAY));
            e.set_hint_component(component::literal("newPreset").with_style_format(ChatFormatting::DarkGray));
        });
        let title_index = row.add_renderable_widget(title_input);
        layout.add_child(LayoutElement::widget(title_index, 120, 20));
        // java: this.speedInput = layout.addChild(new FilteredEditBox(font, 50, 20, Component.empty()));
        let mut speed_input: AbstractWidget = compat::edit_box(0, 0, 50, 20, component::empty());
        // java: this.speedInput.setFilter(str -> str.isEmpty() || NUMBER.matcher(str).matches());
        // java: this.speedInput.setValue(speed);
        if speed_filter(&speed) {
            compat::set_edit_box_value(&mut speed_input, &speed);
        }
        compat::with_edit_box(&mut speed_input, |e| {
            // java: this.speedInput.setMaxLength(3);
            e.set_max_length(3);
            // java: this.speedInput.setHint(Component.literal("0").withStyle(ChatFormatting.DARK_GRAY));
            e.set_hint_component(component::literal("0").with_style_format(ChatFormatting::DarkGray));
        });
        let speed_index = row.add_renderable_widget(speed_input);
        layout.add_child(LayoutElement::widget(speed_index, 50, 20));
        // java: this.removeButton = layout.addChild(Button.builder(Component.literal("-"), _ -> SpeedPresetListWidget.this.removeEntry(this))
        // java: 		.size(20, 20)
        // java: 		.build());
        let remove_requested = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let request = remove_requested.clone();
        let remove_button: AbstractWidget = button::builder(component::literal("-").get_string(), OnPress::Lambda(PressLambda::new(move |_screen, _index| {
            request.store(true, std::sync::atomic::Ordering::Relaxed);
        })))
        .size(20, 20)
        .build();
        let remove_index = row.add_renderable_widget(remove_button);
        layout.add_child(LayoutElement::widget(remove_index, 20, 20));
        let _ = font;
        let (_, __this) = AbstractEntryBase::new_in(move |mut __base| {
            __base.__row = row;
            let __h = compat::shared(Self {
                base: __base,
                title_input: title_index,
                speed_input: speed_index,
                remove_button: remove_index,
                layout,
                remove_requested,
            });
            ((__h.clone() as compat::Shared<dyn AbstractEntry>), __h)
        });
        __this
    }

    /// java: @Override public List<? extends NarratableEntry> narratables()
    /// java: return List.of(titleInput, speedInput, removeButton); - the row's widgets, which the game narrates from the
    /// row screen (`children()`), in this order
    pub fn narratables(__this: &compat::Shared<Self>) -> Vec<usize> {
        compat::read(__this, |__t| vec![__t.title_input, __t.speed_input, __t.remove_button])
    }

    /// java: @Override public List<? extends GuiEventListener> children()
    /// java: return List.of(titleInput, speedInput, removeButton); - their indices in the row screen
    pub fn children(__this: &compat::Shared<Self>) -> Vec<usize> {
        compat::read(__this, |__t| vec![__t.title_input, __t.speed_input, __t.remove_button])
    }

    /// The value of one of the row's edit boxes (`titleInput.getValue()` / `speedInput.getValue()`). `row` is the entry's
    /// widgets (`children()`), which its adapter holds once the list has the entry (`addEntry`).
    fn input_value(__this: &compat::Shared<Self>, row: &mcrust::net::minecraft::client::gui::screens::screen::Screen, index: usize) -> String {
        let _ = __this;
        compat::edit_box_value(row.widget(index))
    }

    /// java: public void save()
    pub fn save(__this: &compat::Shared<Self>, row: &mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        let Some(mut mapping) = SpeedPresetEntry::get_mapping(__this, row) else {
            return;
        };
        SpeedPresets::get_instance().set_preset(mapping.0.clone(), mapping.1);
    }

    /// java: protected boolean isEmpty()
    pub fn is_empty(__this: &compat::Shared<Self>, row: &mcrust::net::minecraft::client::gui::screens::screen::Screen) -> bool {
        let (title, speed) = compat::read(__this, |__t| (__t.title_input, __t.speed_input));
        return SpeedPresetEntry::input_value(__this, row, title).is_empty() && SpeedPresetEntry::input_value(__this, row, speed).is_empty();
    }

    /// java: protected @Nullable ObjectIntPair<String> getMapping()
    pub fn get_mapping(__this: &compat::Shared<Self>, row: &mcrust::net::minecraft::client::gui::screens::screen::Screen) -> Option<(String, i32)> {
        if SpeedPresetEntry::is_empty(__this, row) {
            return None;
        }
        let (title, speed) = compat::read(__this, |__t| (__t.title_input, __t.speed_input));
        // java: try { return ObjectIntPair.of(titleInput.getValue(), Integer.parseInt(speedInput.getValue())); }
        // java: catch (NumberFormatException _) { return null; } - `Integer.parseInt`: no trim
        match SpeedPresetEntry::input_value(__this, row, speed).parse::<i32>() {
            Ok(value) => Some((SpeedPresetEntry::input_value(__this, row, title), value)),
            Err(_) => None,
        }
    }
}

/// The list's `SpeedPresetEntry` rows, in order, each with its widgets (its adapter's row screen) lent to `f` - Java's
/// `children().stream().filter(SpeedPresetEntry.class::isInstance).map(SpeedPresetEntry.class::cast)` with the entries'
/// own widgets reachable.
fn with_speed_preset_entries(list: &mut ModSelectionList, f: &mut dyn FnMut(&compat::Shared<SpeedPresetEntry>, &mcrust::net::minecraft::client::gui::screens::screen::Screen)) {
    let count: usize = list.list.entry_count();
    for i in 0..count {
        let Some(slot) = list.list.entry_object(i) else {
            continue;
        };
        slot.with::<compat::EntryAdapter<dyn AbstractEntry>, _>(|adapter| {
            let object = adapter.object().clone();
            if let Some(entry) = compat::try_view::<SpeedPresetEntry, _>(&object) {
                f(&*entry, &adapter.row);
            }
        });
    }
}

/// `entry.updatePosition()` for the list's row `index`: the row object's class body, with its widgets and its row box
/// lent by its adapter (the list holds both once the entry is added)
fn update_entry_position(list: &mut ModSelectionList, index: usize) {
    let entry_box = list.list.entry(index).clone();
    let Some(slot) = list.list.entry_object(index) else {
        return;
    };
    slot.with::<compat::EntryAdapter<dyn AbstractEntry>, _>(|adapter| {
        adapter.lend(Some(&entry_box), |__h| {
            let __f = compat::dispatch(__h, |__t| __t.update_position());
            __f(__h)
        })
    });
}

impl AbstractEntry for SpeedPresetEntry {
    fn abstract_entry_base(&self) -> &AbstractEntryBase {
        &self.base
    }
    fn abstract_entry_base_mut(&mut self) -> &mut AbstractEntryBase {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override protected void updatePosition() (dispatch: the body for this class)
    fn update_position(&self) -> fn(&compat::Shared<dyn AbstractEntry>) {
        SpeedPresetEntry::update_position_dyn
    }
}

impl SpeedPresetEntry {
    /// java: @Override protected void updatePosition()
    pub fn update_position_impl(__this: &compat::Shared<Self>) {
        // java: layout.setPosition(getRowLeft(), getY()); - `getRowLeft()` is the enclosing list's (an inner class), which
        // is the x the list gives each of its rows (`Entry.getX()`); `setPosition` is `setX` then `setY`
        { let (__q0, __q1) = compat::read(__this, |__t| (__t.base.base.get_x(), __t.base.base.get_y())); compat::write(__this, |__t| { __t.layout.set_x(__q0); __t.layout.set_y(__q1); }) };
        // java: layout.arrangeElements(); - the positions written onto the row's widgets
        { __this.lock().borrow_mut().layout.arrange_elements(); let mut __vw = Vec::new(); LayoutElement::Grid(__this.lock().borrow().layout.wrapped.clone()).visit_widgets(&mut __vw); for (__i, __x, __y) in __vw { compat::write(__this, |__t| __t.base.__row.widget_mut(__i).set_position(__x, __y)); } };
    }

    /// SpeedPresetEntry.updatePosition as the dispatch of AbstractEntry.updatePosition
    pub fn update_position_dyn(__this: &compat::Shared<dyn AbstractEntry>) {
        Self::update_position_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// The filter of the row's focused widget when it is one of the two `FilteredEditBox`es
    fn focused_filter(&self, row: &mcrust::net::minecraft::client::gui::screens::screen::Screen) -> Option<fn(&str) -> bool> {
        let focused = row.get_focused()?;
        if focused == self.title_input {
            Some(title_filter)
        } else if focused == self.speed_input {
            Some(speed_filter)
        } else {
            None
        }
    }
}

/// java: `FilteredEditBox`'s overrides of `EditBox` in front of an edit of the row's focused box: the edit is made on a
/// copy of the box first (with no responder), and the box takes it only when Java's override would have:
/// `insertText(input)` (a typed character, a paste, or `insertText("")` over a selection) tests the INSERTED text
/// (`filter.test(input)`), `deleteCharsToPos(pos)` (a deletion with no selection) tests the new value. A refused edit
/// answers what the box would have (the key or character was taken).
fn filtered_edit(row: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, filter: fn(&str) -> bool, edit: impl Fn(&mut mcrust::net::minecraft::client::gui::screens::screen::Screen) -> bool) -> bool {
    let Some(focused) = row.get_focused() else {
        return edit(row);
    };
    let mut trial_widget = row.widget(focused).clone();
    compat::with_edit_box(&mut trial_widget, |__e| __e.set_responder(mcrust::net::minecraft::client::gui::components::edit_box::Responder::Lambda(mcrust::net::minecraft::client::gui::components::edit_box::ResponderLambda::new(|_| {}))));
    let (cursor, highlight) = compat::with_edit_box(row.widget_mut(focused), |__e| (__e.get_cursor_position(), __e.get_highlight_pos())).unwrap_or((0, 0));
    let mut trial = mcrust::net::minecraft::client::gui::screens::screen::Screen::new(String::new());
    let trial_index = trial.add_widget(trial_widget);
    trial.set_focused(Some(trial_index));
    let answer = edit(&mut trial);
    let before: Vec<char> = compat::edit_box_value(row.widget(focused)).chars().collect();
    let after: Vec<char> = compat::edit_box_value(trial.widget(trial_index)).chars().collect();
    if after == before {
        return edit(row);
    }
    let passes: bool = if highlight != cursor || after.len() > before.len() {
        // insertText(input): the text that replaced [start, end)
        let start = (cursor.min(highlight).max(0) as usize).min(before.len());
        let end = (cursor.max(highlight).max(0) as usize).min(before.len());
        let inserted_end = after.len().saturating_sub(before.len() - end).max(start);
        let inserted: String = after[start.min(after.len())..inserted_end.min(after.len())].iter().collect();
        filter(&inserted)
    } else {
        // deleteCharsToPos(pos): the new value
        filter(&after.iter().collect::<String>())
    };
    if !passes {
        return answer;
    }
    edit(row)
}

/// SpeedPresetEntry's row box and its widgets, lent by its `compat::EntryAdapter` for each call
impl compat::RowHalf for SpeedPresetEntry {
    fn entry_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry {
        &mut self.base.base
    }
    fn row_parts(&mut self) -> (&mut mcrust::net::minecraft::client::gui::screens::screen::Screen, &mut compat::WidgetCache) {
        let __s = &mut self.base;
        (&mut __s.__row, &mut __s.__widgets)
    }
}

/// SpeedPresetEntry as a row of the game's list (`addEntry` attaches it)
impl ModListEntry for compat::EntryAdapter<SpeedPresetEntry> {
    /// java: @Override public void extractContent(GuiGraphicsExtractor graphics, int mouseX, int mouseY, boolean hovered, float deltaTicks)
    fn extract_content(&mut self, list: &mut ModSelectionList, index: usize, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, hovered: bool, a: f32) {
        self.lend(Some(list.list.entry(index)), |__h| ({ let __f = compat::dispatch(__h, |__t| __t.extract_content()); __f(&compat::view(__h, |a| a as compat::Shared<dyn AbstractEntry>), &context.graphics, mouse_x, mouse_y, hovered, a) }));
        // the row's widgets (Java's `child.extractRenderState(..)` over `children()`)
        context.extract_widgets(&mut self.row, mouse_x, mouse_y);
    }

    fn children(&mut self) -> Option<&mut mcrust::net::minecraft::client::gui::screens::screen::Screen> {
        Some(&mut self.row)
    }

}

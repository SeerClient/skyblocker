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
        let entries: Vec<compat::Shared<dyn AbstractEntry>> = compat::write(__this, |__t| compat::list_entries::<dyn AbstractEntry>(&mut __t.base));
        let mut children_map: std::collections::HashMap<String, i32> = std::collections::HashMap::new();
        for entry in entries.iter() {
            if !compat::read(entry, |__t| __t.as_any().is::<SpeedPresetEntry>()) {
                continue;
            }
            let Some((key, value)) = SpeedPresetEntry::get_mapping(&compat::handle_as::<SpeedPresetEntry, _>(entry)) else {
                continue;
            };
            // Collectors.toMap refuses a duplicate key
            if children_map.insert(key.clone(), value).is_some() {
                panic!("java.lang.IllegalStateException: Duplicate key {key}");
            }
        }
        return !(instance.are_presets_equal(&children_map));
    }

    /// java: public void updatePosition()
    pub fn update_position(__this: &compat::Shared<Self>) {
        for __a0 in compat::write(__this, |__t| { let __l = &mut __t.base; compat::list_entries::<dyn AbstractEntry>(__l) }) {
            ({ let __f = compat::dispatch(&__a0, |__t| __t.update_position()); __f(&__a0) });
        }
    }

    /// java: public void newEntry()
    pub fn new_entry(__this: &compat::Shared<Self>) {
        let mut entry: compat::Shared<SpeedPresetEntry> = SpeedPresetEntry::new("".to_string(), "".to_string());
        let __entry = (entry.clone() as compat::Shared<dyn AbstractEntry>);
        compat::write(__this, |__t| { let __l = &mut __t.base; (__l.add_entry(Box::new(compat::EntryAdapter::new(__entry))) as i32) });
        ({ let __f = compat::dispatch(&entry, |__t| __t.update_position()); __f(&compat::view(&entry, |a| a as compat::Shared<dyn AbstractEntry>)) });
        // TODO(fabric2seer): `ContainerObjectSelectionList.centerScrollOn` has no Rust method in the port (looked for `center_scroll_on` in ContainerObjectSelectionList and its supertypes)
        // rust: __this.lock().borrow().center_scroll_on(&entry);
        let __entry = Some((entry.clone() as compat::Shared<dyn AbstractEntry>));
        compat::write(__this, |__t| { let __l = &mut __t.base; { let __i = __entry.as_ref().and_then(|__e| compat::entry_index::<dyn AbstractEntry>(__l, __e)); __l.set_selected(__i); } });
        // TODO(fabric2seer): `set_focused_clears_selection` takes 2 argument(s) in the port (`changed: bool, focused: Option<usize>`), Java passes 1
        // rust: container_object_selection_list::set_focused_clears_selection(entry.clone());
    }

    /// java: public void save()
    pub fn save(__this: &compat::Shared<Self>) {
        let mut instance: SpeedPresets = SpeedPresets::get_instance();
        instance.get_presets().clear();
        // TODO(fabric2seer): `.isInstance(..)` on a Class
        //   java: children().stream()
        //   java: 				.filter(SpeedPresetEntry.class::isInstance)
        //   java: 				.map(SpeedPresetEntry.class::cast)
        //   java: 				.forEach(SpeedPresetEntry::save);
        instance.save_presets();
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
        // TODO(fabric2seer): `Entry.children` has no Rust method in the port (looked for `children` in ContainerObjectSelectionList and its supertypes)
        // rust: for child in __this.lock().borrow().abstract_entry_base().children() {
        // rust: if let Some(widget) = child.as_any().downcast_ref::<LayoutElement>() {
        // rust: widget.set_y(__this.lock().borrow().abstract_entry_base().get_y());
        // rust: }
        // rust: // TODO(fabric2seer): instanceof an interface (`Renderable`): Rust cannot test for a trait at run time
        // rust: //   java: if (child instanceof Renderable drawable)
        // rust: //   java: 					drawable.extractRenderState(graphics, mouseX, mouseY, deltaTicks);
        // rust: }
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
        self.lend(Some(list.list.entry(index)), |__h| { if let Some(__c) = compat::try_view::<SpeedPresetEntry, _>(__h) { return ({ let __f = compat::dispatch(&*__c, |__t| __t.extract_content()); __f(&compat::view(&*__c, |a| a as compat::Shared<dyn AbstractEntry>), &context.graphics, mouse_x, mouse_y, hovered, a) }); } if let Some(__c) = compat::try_view::<TitleEntry, _>(__h) { return ({ let __f = compat::dispatch(&*__c, |__t| __t.extract_content()); __f(&compat::view(&*__c, |a| a as compat::Shared<dyn AbstractEntry>), &context.graphics, mouse_x, mouse_y, hovered, a) }); }  });
        // the row's widgets (Java's `child.extractRenderState(..)` over `children()`)
        context.extract_widgets(&mut self.row, mouse_x, mouse_y);
    }

    fn children(&mut self) -> Option<&mut mcrust::net::minecraft::client::gui::screens::screen::Screen> {
        Some(&mut self.row)
    }

}

/// java: public class TitleEntry extends AbstractEntry
pub struct TitleEntry {
    /// java: extends AbstractEntry
    pub base: AbstractEntryBase,
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
        // TODO(fabric2seer): unknown name `minecraft`
        //   java: graphics.centeredText(minecraft.font, Component.translatable("skyblocker.config.general.speedPresets.config.title"), width / 2 - 50, this.getY() + 8, CommonColors.WHITE);
        // TODO(fabric2seer): unknown name `minecraft`
        //   java: graphics.centeredText(minecraft.font, Component.translatable("skyblocker.config.general.speedPresets.config.speed"), width / 2 + 50, this.getY() + 8, CommonColors.WHITE);
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
pub struct SpeedPresetEntry {
    /// java: extends AbstractEntry
    pub base: AbstractEntryBase,
    /// java: protected final FilteredEditBox titleInput;
    pub title_input: Option<compat::Shared<dyn FilteredEditBox>>,
    /// java: protected final FilteredEditBox speedInput;
    pub speed_input: Option<compat::Shared<dyn FilteredEditBox>>,
    /// java: protected final Button removeButton;
    pub remove_button: Option<compat::WidgetRef>,
    /// java: protected final LinearLayout layout;
    pub layout: LinearLayout,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
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

impl SpeedPresetEntry {
    /// java: public SpeedPresetEntry(String title, String speed)
    pub fn new(mut title: String, mut speed: String) -> compat::Shared<Self> {
        let mut font: Font = compat::font();
        let __init_layout = LinearLayout::horizontal();
        let (_, __this) = AbstractEntryBase::new_in(move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                self_ref: std::sync::Weak::new(),
                title_input: None,
                speed_input: None,
                remove_button: None,
                layout: __init_layout,
            });
            compat::write(&__h, |__t| __t.self_ref = std::sync::Arc::downgrade(&__h));
            ((__h.clone() as compat::Shared<dyn AbstractEntry>), __h)
        });
        // java: layout.spacing(2); - the port's `spacing` takes the layout by value and answers it
        compat::write(&__this, |__t| { let __l = std::mem::replace(&mut __t.layout, LinearLayout::horizontal()); __t.layout = __l.spacing(2); });
        // java: titleInput = layout.addChild(new FilteredEditBox(font, 120, 20, Component.empty()));
        // rust: port gap (GAPS_batchA #3) - a FilteredEditBox is a mod subclass of EditBox with no widget adapter: it is kept
        // as the field but cannot join the row's layout / widgets
        { let __v = Some((FilteredEditBoxBase::new(compat::font(), 120, 20, component::empty()) as compat::Shared<dyn FilteredEditBox>)); compat::write(&__this, |__t| __t.title_input = __v); }
        ({ let __r = &compat::read(&__this, |__t| __t.title_input.clone()).unwrap(); let __f = compat::dispatch(__r, |__t| __t.set_filter()); __f(__r, Arc::new(move |str| (str.is_empty() || TITLE.matcher(&str).matches()))) });
        ({ let __r = &compat::read(&__this, |__t| __t.title_input.clone()).unwrap(); let __f = compat::dispatch(__r, |__t| __t.set_value()); __f(__r, &title) });
        let __o1 = compat::read(&__this, |__t| __t.title_input.clone()).unwrap();
        compat::write(&__o1, |__t| __t.filtered_edit_box_base_mut().set_max_length(16));
        let __wa1 = component::literal("newPreset").with_style_format(ChatFormatting::DarkGray);
        let __o2 = compat::read(&__this, |__t| __t.title_input.clone()).unwrap();
        compat::write(&__o2, |__t| __t.filtered_edit_box_base_mut().set_hint_component(__wa1.clone()));
        // java: speedInput = layout.addChild(new FilteredEditBox(font, 50, 20, Component.empty())); (same port gap as titleInput)
        { let __v = Some((FilteredEditBoxBase::new(compat::font(), 50, 20, component::empty()) as compat::Shared<dyn FilteredEditBox>)); compat::write(&__this, |__t| __t.speed_input = __v); }
        ({ let __r = &compat::read(&__this, |__t| __t.speed_input.clone()).unwrap(); let __f = compat::dispatch(__r, |__t| __t.set_filter()); __f(__r, Arc::new(move |str| (str.is_empty() || NUMBER.matcher(&str).matches()))) });
        ({ let __r = &compat::read(&__this, |__t| __t.speed_input.clone()).unwrap(); let __f = compat::dispatch(__r, |__t| __t.set_value()); __f(__r, &speed) });
        let __o3 = compat::read(&__this, |__t| __t.speed_input.clone()).unwrap();
        compat::write(&__o3, |__t| __t.filtered_edit_box_base_mut().set_max_length(3));
        let __wa2 = component::literal("0").with_style_format(ChatFormatting::DarkGray);
        let __o4 = compat::read(&__this, |__t| __t.speed_input.clone()).unwrap();
        compat::write(&__o4, |__t| __t.filtered_edit_box_base_mut().set_hint_component(__wa2.clone()));
        // TODO(fabric2seer): `SpeedPresetListWidget.this` (the enclosing instance of an inner class)
        //   java: this.removeButton = layout.addChild(Button.builder(Component.literal("-"), _ -> SpeedPresetListWidget.this.removeEntry(this))
        //   java: 					.size(20, 20)
        //   java: 					.build());
        __this
    }

    /// java: @Override public List<? extends NarratableEntry> narratables()
    pub fn narratables(__this: &compat::Shared<Self>) -> Vec<Box<dyn NarratableEntry>> {
        // java: return List.of(titleInput, speedInput, removeButton);
        // rust: port gap (GAPS_batchA #2 / #9) - the entry's widgets live in its row screen (`children()` below lends it); the
        // edit boxes are mod objects with no adapter, so no owned narratable list can be built here
        let _ = __this;
        return vec![];
    }

    /// java: @Override public List<? extends GuiEventListener> children()
    pub fn children(__this: &compat::Shared<Self>) -> Vec<Box<dyn GuiEventListener>> {
        // java: return List.of(titleInput, speedInput, removeButton); (see narratables - GAPS_batchA #9)
        let _ = __this;
        return vec![];
    }

    /// java: public void save()
    pub fn save(__this: &compat::Shared<Self>) {
        let Some(mut mapping) = SpeedPresetEntry::get_mapping(__this) else {
            return;
        };
        SpeedPresets::get_instance().set_preset(mapping.0.clone(), mapping.1);
    }

    /// java: protected boolean isEmpty()
    pub fn is_empty(__this: &compat::Shared<Self>) -> bool {
        let __o1 = compat::read(__this, |__t| __t.title_input.clone()).unwrap();
        let __o2 = compat::read(__this, |__t| __t.speed_input.clone()).unwrap();
        return ((compat::write(&__o1, |__t| __t.filtered_edit_box_base_mut().get_value())).is_empty() && (compat::write(&__o2, |__t| __t.filtered_edit_box_base_mut().get_value())).is_empty());
    }

    /// java: protected @Nullable ObjectIntPair<String> getMapping()
    pub fn get_mapping(__this: &compat::Shared<Self>) -> Option<(String, i32)> {
        if SpeedPresetEntry::is_empty(__this) {
            return None;
        }
        // java: try {..} catch (NumberFormatException _) - an exception here is a panic, caught by `catch_unwind`; the body's `return` is the closure's value
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let __o1 = compat::read(__this, |__t| __t.title_input.clone()).unwrap();
            let __o2 = compat::read(__this, |__t| __t.speed_input.clone()).unwrap();
            return Some((compat::write(&__o1, |__t| __t.filtered_edit_box_base_mut().get_value()), (compat::write(&__o2, |__t| __t.filtered_edit_box_base_mut().get_value())).parse::<i32>().expect("java.lang.NumberFormatException") /* Integer.parseInt: no trim */));
        })) {
            Ok(__v) => return __v,
            Err(_) => {
                return None;
            }
        }
    }

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
        // TODO(fabric2seer): `Entry.getRowLeft` has no Rust method in the port (looked for `get_row_left` in ContainerObjectSelectionList and JavaSrc)
        // TODO(fabric2seer): `LayoutElement::set_position` ports `setPosition`, but the port's `LinearLayout` holds no `LayoutElement` (no Deref, no `base` field) to call it on
        // rust: { let (__q0, __q1,) = (__this.lock().borrow().get_row_left(), __this.lock().borrow().get_y(),); __this.lock().borrow_mut().layout.set_position(__q0, __q1) };
        { __this.lock().borrow_mut().layout.arrange_elements(); let mut __vw = Vec::new(); LayoutElement::Grid(__this.lock().borrow().layout.wrapped.clone()).visit_widgets(&mut __vw); for (__i, __x, __y) in __vw { compat::write(__this, |__t| __t.base.__row.widget_mut(__i).set_position(__x, __y)); } };
    }

    /// SpeedPresetEntry.updatePosition as the dispatch of AbstractEntry.updatePosition
    pub fn update_position_dyn(__this: &compat::Shared<dyn AbstractEntry>) {
        Self::update_position_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

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

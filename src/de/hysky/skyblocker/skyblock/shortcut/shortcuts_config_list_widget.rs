//! Ported from de.hysky.skyblocker.skyblock.shortcut.ShortcutsConfigListWidget by fabric2seer.
//!
//! The list is the game's `ModSelectionList` (a `ContainerObjectSelectionList`); this class's overrides are its
//! `ModList` class (`ShortcutsConfigListWidget`), and each row class is a `ModListEntry` the list holds. The inner
//! classes reach their list (`ShortcutsConfigListWidget.this`) through the `list` the game lends each row method, and
//! the list's own methods below take that list (`&mut AbstractSelectionList`) as Java's `this`.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::network::chat::component::Component as _;
use crate::de::hysky::skyblocker::skyblock::shortcut::keybind_widget::KeybindWidget;
use crate::de::hysky::skyblocker::skyblock::shortcut::shortcut_key_binding::ShortcutKeyBinding;
use crate::de::hysky::skyblocker::skyblock::shortcut::shortcuts;
use crate::de::hysky::skyblocker::skyblock::shortcut::shortcuts::Shortcuts;
use crate::de::hysky::skyblocker::skyblock::shortcut::shortcuts::ShortcutsRecord;
use crate::de::hysky::skyblocker::skyblock::shortcut::shortcuts_config_screen::ShortcutsConfigScreen;
use crate::de::hysky::skyblocker::utils::data::json_data::JsonData as _;
use mcrust::com::mojang::blaze3d::platform::input_constants;
use mcrust::mcrust::modding::widgets::ListKind;
use mcrust::mcrust::modding::widgets::ModList;
use mcrust::mcrust::modding::widgets::ModListEntry;
use mcrust::mcrust::modding::widgets::ModListEntrySlot;
use mcrust::mcrust::modding::widgets::ModSelectionList;
use mcrust::mcrust::modding::widgets::ModWidgetSlot;
use mcrust::mcrust::modding::widgets::WidgetRenderContext;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::components::abstract_selection_list::AbstractSelectionList;
use mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::tooltip::Tooltip;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::key_event::KeyEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::formatted_text::FormattedText;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::chat::text_color::TextColor;
use mcrust::net::minecraft::util::common_colors;

/// java: private static final int TEXT_Y_OFFSET = 5 + 2;
pub const TEXT_Y_OFFSET: i32 = 5 + 2;
/// java: private static final int TEXT_FIELD_PADDING = 2;
pub const TEXT_FIELD_PADDING: i32 = 2;

/// java: public class ShortcutsConfigListWidget extends ContainerObjectSelectionList<ShortcutsConfigListWidget.AbstractShortcutEntry>
/// - the list's class: its overrides, asked by the game's list bodies.
pub struct ShortcutsConfigListWidget {
    /// java: private final ShortcutsConfigScreen screen; (the screen holds the list: a weak reference back)
    pub screen: compat::RenderThread<compat::WeakShared<ShortcutsConfigScreen>>,
}

impl ShortcutsConfigListWidget {
    /// java: public ShortcutsConfigListWidget(Minecraft minecraftClient, ShortcutsConfigScreen screen, int width, int height, int y)
    /// - the game's list widget, built with this class and its rows (`addRenderableWidget` takes it)
    pub fn new(screen: &compat::Shared<ShortcutsConfigScreen>, width: i32, height: i32, y: i32) -> AbstractWidget {
        // java: super(minecraftClient, width, height, y, 24); this.screen = screen;
        let class = ShortcutsConfigListWidget { screen: compat::RenderThread::new(std::sync::Arc::downgrade(screen)) };
        let mut list = ModSelectionList::with_class(ListKind::Container, width, height, y, 24, Arc::new(class));
        let loaded = Shortcuts::is_shortcuts_loaded();
        let data: Option<ShortcutsRecord> = loaded.then(|| shortcuts::SHORTCUTS.get().lock().borrow().get_data());

        let command_category = ShortcutCategoryEntry::new(&mut list.list, CategoryKind::Commands, "skyblocker.shortcuts.command.target", "skyblocker.shortcuts.command.replacement", None);
        if let Some(data) = &data {
            for command_target in compat::sorted(data.commands.keys().cloned()) {
                let entry = CommandShortcutEntry::new_2(&list.list, command_category.clone(), &command_target);
                add_entry(&mut list.list, Box::new(entry));
            }
        } else {
            add_entry(&mut list.list, Box::new(ShortcutLoadingEntry::new()));
        }
        let command_arg_category = ShortcutCategoryEntry::new(&mut list.list, CategoryKind::CommandArgs, "skyblocker.shortcuts.commandArg.target", "skyblocker.shortcuts.commandArg.replacement", Some("skyblocker.shortcuts.commandArg.tooltip"));
        if let Some(data) = &data {
            for command_arg_target in compat::sorted(data.command_args.keys().cloned()) {
                let entry = CommandArgShortcutEntry::new_2(&list.list, command_arg_category.clone(), &command_arg_target);
                add_entry(&mut list.list, Box::new(entry));
            }
        } else {
            add_entry(&mut list.list, Box::new(ShortcutLoadingEntry::new()));
        }
        let keybind_category = ShortcutCategoryEntry::new(&mut list.list, CategoryKind::KeyBindings, "skyblocker.shortcuts.keyBinding.target", "skyblocker.shortcuts.keyBinding.replacement", Some("skyblocker.shortcuts.keyBinding.tooltip"));
        if let Some(data) = &data {
            for key_binding in compat::sorted(data.key_bindings.keys().cloned()) {
                let mut entry = KeybindShortcutEntry::new_2(&list.list, keybind_category.clone(), key_binding.copy());
                let index = list.list.entry_count();
                entry.update(&list.list, index);
                add_entry(&mut list.list, Box::new(entry));
            }
        } else {
            add_entry(&mut list.list, Box::new(ShortcutLoadingEntry::new()));
        }
        let (x, y, w, h) = (list.list.get_x(), list.list.get_y(), list.list.get_width(), list.list.get_height());
        ModWidgetSlot::widget(x, y, w, h, String::new(), Box::new(list))
    }

    /// The screen this list belongs to, while it is alive.
    fn screen(&self) -> Option<compat::Shared<ShortcutsConfigScreen>> {
        self.screen.get().upgrade()
    }

    /// java: protected Optional<ShortcutCategoryEntry<?>> getCategory()
    pub fn get_category(list: &AbstractSelectionList) -> Option<std::sync::Arc<CategoryInfo>> {
        // java: switch (getSelected()) { case ShortcutCategoryEntry<?> c -> c; case ShortcutEntry<?> e -> e.category; default -> empty }
        let selected = list.get_selected()?;
        let slot = list.entry_object(selected)?;
        if let Some(category) = slot.with::<ShortcutCategoryEntry, _>(|c| c.category.clone()) {
            return Some(category);
        }
        with_shortcut_entry(slot, |e| e.category())
    }

    /// java: protected void addShortcutAfterSelected()
    pub fn add_shortcut_after_selected(list: &mut AbstractSelectionList) {
        let Some(category) = ShortcutsConfigListWidget::get_category(list) else {
            return;
        };
        // java: ArrayList<AbstractShortcutEntry> newEntries = new ArrayList<>(children());
        let mut new_entries: Vec<Entry> = list.children().to_vec();
        // java: ShortcutEntry<?> newEntry = category.entrySupplier.get();
        let new_entry: Box<dyn ModListEntry> = category.new_entry(list);
        // java: newEntries.add(children().indexOf(getSelected()) + 1, newEntry); (indexOf of null is -1)
        let index = list.get_selected().map_or(0, |selected| selected + 1);
        let height = list.default_entry_height;
        let mut row = Entry::new(height);
        row.object = Some(ModListEntrySlot::new(new_entry));
        new_entries.insert(index, row);
        list.replace_entries(new_entries);
        // java: setSelected(newEntry); - through the class (this override)
        list.set_selected(Some(index), false);
    }

    /// java: protected void updatePositions()
    pub fn update_positions(list: &AbstractSelectionList) {
        let width = list.get_width();
        for i in 0..list.entry_count() {
            if let Some(slot) = list.entry_object(i) {
                with_any_entry(slot, |e| e.update_positions(width));
            }
        }
    }

    /// java: protected void updateKeybinds()
    /// (`current` is a row that is inside one of its own methods - the one whose widget asked for this - updated
    /// directly, since the list cannot lend it again)
    pub fn update_keybinds(list: &AbstractSelectionList, current: Option<(usize, &mut KeybindShortcutEntry)>) {
        let current_index = current.as_ref().map(|(i, _)| *i);
        for i in 0..list.entry_count() {
            if Some(i) == current_index {
                continue;
            }
            if let Some(slot) = list.entry_object(i) {
                slot.with::<KeybindShortcutEntry, _>(|e| e.update(list, i));
            }
        }
        if let Some((i, e)) = current {
            e.update(list, i);
        }
    }

    /// java: protected boolean stopEditing()
    pub fn stop_editing(list: &AbstractSelectionList) -> bool {
        // java: children().stream().filter(KeybindShortcutEntry.class::isInstance)...anyMatch(KeybindShortcutEntry::stopEditing)
        let mut bl = false;
        for i in 0..list.entry_count() {
            if let Some(slot) = list.entry_object(i)
                && slot.with::<KeybindShortcutEntry, _>(|e| e.stop_editing()).unwrap_or(false)
            {
                bl = true;
                break;
            }
        }
        bl
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent event, boolean doubleClick)
    fn mouse_clicked_body(&self, list: &mut ModSelectionList, event: &MouseButtonEvent, double_click: bool, super_mouse_clicked: &mut dyn FnMut(&mut ModSelectionList) -> bool) -> bool {
        if super_mouse_clicked(list) {
            return true;
        }
        if ShortcutsConfigListWidget::stop_editing(&list.list) {
            ShortcutsConfigListWidget::update_keybinds(&list.list, None);
        }
        if let Some(screen) = self.screen() {
            ShortcutsConfigScreen::check_for_duplicates_with(&screen, &list.list);
        }
        return false;
    }

    /// java: @Override protected void removeEntry(AbstractShortcutEntry entry)
    pub fn remove_entry(list: &mut AbstractSelectionList, index: usize) {
        list.remove_entry(index);
    }

    /// java: protected boolean hasChanges()
    pub fn has_changes(list: &AbstractSelectionList) -> bool {
        // java: notEmptyShortcuts.length != Shortcuts.shortcuts.getData().size() || anyMatch(ShortcutEntry::isChanged)
        let not_empty = ShortcutsConfigListWidget::get_not_empty_shortcuts(list);
        let size = shortcuts::SHORTCUTS.get().lock().borrow().get_data().size();
        return (not_empty.len() as i32) != size
            || not_empty.iter().any(|&i| list.entry_object(i).and_then(|slot| with_shortcut_entry(slot, |e| e.is_changed())).unwrap_or(false));
    }

    /// java: protected boolean hasDuplicates()
    pub fn has_duplicates(list: &AbstractSelectionList) -> bool {
        let mut keys: std::collections::HashSet<String> = std::collections::HashSet::new();
        for i in 0..list.entry_count() {
            let Some(key) = list.entry_object(i).and_then(|slot| with_shortcut_entry(slot, |e| e.key())) else {
                continue;
            };
            if keys.contains(&key) {
                return true;
            }
            keys.insert(key);
        }
        return false;
    }

    /// java: protected void saveShortcuts()
    pub fn save_shortcuts(list: &AbstractSelectionList) {
        // java: Shortcuts.shortcuts.getData().clear();
        with_shortcuts_data(|data| data.clear());
        for i in ShortcutsConfigListWidget::get_not_empty_shortcuts(list) {
            if let Some(slot) = list.entry_object(i) {
                with_shortcut_entry(slot, |e| e.save());
            }
        }
        // Save shortcuts to disk
        shortcuts::SHORTCUTS.get().lock().borrow().save();
    }

    /// java: private Stream<? extends ShortcutEntry<?>> getNotEmptyShortcuts() (the rows' positions)
    pub fn get_not_empty_shortcuts(list: &AbstractSelectionList) -> Vec<usize> {
        (0..list.entry_count())
            .filter(|&i| list.entry_object(i).and_then(|slot| with_shortcut_entry(slot, |e| e.is_not_empty())).unwrap_or(false))
            .collect()
    }
}

/// ShortcutsConfigListWidget's overrides of the list's own methods, asked by the port's list bodies
impl ModList for ShortcutsConfigListWidget {
    /// java: @Override public int getRowWidth()
    fn get_row_width(&self, _list: &AbstractSelectionList, super_answer: i32) -> i32 {
        return super_answer + 100 + 2 * TEXT_FIELD_PADDING;
    }

    /// java: @Override protected int scrollBarX()
    fn scroll_bar_x(&self, _list: &AbstractSelectionList, super_answer: i32) -> i32 {
        return super_answer;
    }

    /// java: @Override protected boolean entriesCanBeSelected()
    fn entries_can_be_selected(&self, _list: &AbstractSelectionList, _super_answer: bool) -> bool {
        return true;
    }

    /// java: @Override public void setSelected(@Nullable AbstractShortcutEntry entry)
    fn set_selected(&self, list: &mut AbstractSelectionList, selected: Option<usize>, super_set_selected: &mut dyn FnMut(&mut AbstractSelectionList, Option<usize>)) {
        super_set_selected(list, selected);
        if let Some(screen) = self.screen() {
            ShortcutsConfigScreen::update_buttons_with(&screen, list);
        }
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent event, boolean doubleClick)
    fn mouse_clicked(&self, list: &mut ModSelectionList, event: &MouseButtonEvent, double_click: bool, super_mouse_clicked: &mut dyn FnMut(&mut ModSelectionList) -> bool) -> bool {
        self.mouse_clicked_body(list, event, double_click, super_mouse_clicked)
    }
}

/// `addEntry(entry)` of a row object.
fn add_entry(list: &mut AbstractSelectionList, entry: Box<dyn ModListEntry>) -> usize {
    let mut row = Entry::new(0);
    row.object = Some(ModListEntrySlot::new(entry));
    list.add_entry(row)
}

/// `Shortcuts.shortcuts.getData()` - the live record (Java's `getData()` hands out the object itself).
pub fn with_shortcuts_data<R>(f: impl FnOnce(&mut ShortcutsRecord) -> R) -> R {
    let handle = shortcuts::SHORTCUTS.get().clone();
    let guard = handle.lock();
    let mut data = guard.borrow_mut();
    f(data.json_data_base_mut().data.as_mut().expect("Shortcuts data"))
}

/// `entry instanceof ShortcutEntry<?> e` on a row object, `e` handed to `f`.
pub fn with_shortcut_entry<R>(slot: &ModListEntrySlot, f: impl FnOnce(&mut dyn ShortcutEntry) -> R) -> Option<R> {
    if slot.is::<CommandArgShortcutEntry>() {
        return slot.with::<CommandArgShortcutEntry, _>(|e| f(e));
    }
    if slot.is::<CommandShortcutEntry>() {
        return slot.with::<CommandShortcutEntry, _>(|e| f(e));
    }
    if slot.is::<KeybindShortcutEntry>() {
        return slot.with::<KeybindShortcutEntry, _>(|e| f(e));
    }
    None
}

/// Any row of this list as an `AbstractShortcutEntry`.
fn with_any_entry<R>(slot: &ModListEntrySlot, f: impl FnOnce(&mut dyn AbstractShortcutEntry) -> R) -> Option<R> {
    if slot.is::<ShortcutCategoryEntry>() {
        return slot.with::<ShortcutCategoryEntry, _>(|e| f(e));
    }
    if slot.is::<ShortcutLoadingEntry>() {
        return slot.with::<ShortcutLoadingEntry, _>(|e| f(e));
    }
    if slot.is::<CommandArgShortcutEntry>() {
        return slot.with::<CommandArgShortcutEntry, _>(|e| f(e));
    }
    if slot.is::<CommandShortcutEntry>() {
        return slot.with::<CommandShortcutEntry, _>(|e| f(e));
    }
    if slot.is::<KeybindShortcutEntry>() {
        return slot.with::<KeybindShortcutEntry, _>(|e| f(e));
    }
    None
}

/// java: public abstract static class AbstractShortcutEntry extends ContainerObjectSelectionList.Entry<AbstractShortcutEntry>
pub trait AbstractShortcutEntry {
    /// java: protected void updatePositions() (`width` is the list's)
    fn update_positions(&mut self, width: i32) {
        let _ = width;
    }
}

/// java: AbstractShortcutEntry.mouseClicked - `super.mouseClicked(click, doubled); return true;` (`super` is the
/// container row's: the children)
fn abstract_entry_mouse_clicked(children: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, event: &MouseButtonEvent) -> bool {
    children.mouse_clicked(event.button(), event.x(), event.y());
    true
}

/// The category kinds: which map of `Shortcuts.shortcuts.getData()` a category edits, and which row class its
/// `entrySupplier` makes (Java's `ShortcutCategoryEntry<T>` with its `entryFactory`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CategoryKind {
    /// `ShortcutCategoryEntry<String>` over `commands()`, `CommandShortcutEntry::new`
    Commands,
    /// `ShortcutCategoryEntry<String>` over `commandArgs()`, `CommandArgShortcutEntry::new`
    CommandArgs,
    /// `ShortcutCategoryEntry<ShortcutKeyBinding>` over `keyBindings()`, `KeybindShortcutEntry::new`
    KeyBindings,
}

/// java: protected class ShortcutCategoryEntry<T> - its fields, which the category's entries share (`category`)
pub struct CategoryInfo {
    /// java: private final Map<T, String> shortcutsMap; (the map of the live record this kind names)
    pub kind: CategoryKind,
    /// java: private final Component targetName;
    pub target_name: MutableComponent,
    /// java: private final Component replacementName;
    pub replacement_name: MutableComponent,
    /// java: private final @Nullable Component tooltip;
    pub tooltip: Option<MutableComponent>,
}

// SAFETY: a category is made and read on the render thread only (the list's rows are; see `RowThread`).
unsafe impl Send for CategoryInfo {}
unsafe impl Sync for CategoryInfo {}

impl CategoryInfo {
    /// java: private final Supplier<ShortcutEntry<T>> entrySupplier; - `() -> entryFactory.apply(this)`
    pub fn new_entry(self: &std::sync::Arc<CategoryInfo>, list: &AbstractSelectionList) -> Box<dyn ModListEntry> {
        match self.kind {
            CategoryKind::Commands => Box::new(CommandShortcutEntry::new(list, self.clone())),
            CategoryKind::CommandArgs => Box::new(CommandArgShortcutEntry::new(list, self.clone())),
            CategoryKind::KeyBindings => {
                let mut entry = KeybindShortcutEntry::new(list, self.clone());
                entry.update(list, usize::MAX);
                Box::new(entry)
            }
        }
    }
}

/// Pins a row object to the render thread: the game holds rows as `Send` objects, and a row's widgets are not -
/// every method a row implements checks the thread first.
pub struct RowThread(std::thread::ThreadId);

impl RowThread {
    fn new() -> RowThread {
        RowThread(std::thread::current().id())
    }

    fn check(&self) {
        assert!(std::thread::current().id() == self.0, "a mod list row was reached off the render thread");
    }
}

/// java: protected class ShortcutCategoryEntry<T> extends AbstractShortcutEntry
pub struct ShortcutCategoryEntry {
    /// the category's fields (`shortcutsMap`, `entrySupplier`, `targetName`, `replacementName`, `tooltip`)
    pub category: std::sync::Arc<CategoryInfo>,
    pub thread: RowThread,
}

// SAFETY: see `RowThread`.
unsafe impl Send for ShortcutCategoryEntry {}

impl ShortcutCategoryEntry {
    /// java: private ShortcutCategoryEntry(Map<T, String> shortcutsMap, Function<..> entryFactory, String targetName, String replacementName[, String tooltip])
    /// - ends in `addEntry(this)`
    pub fn new(list: &mut AbstractSelectionList, kind: CategoryKind, target_name: &str, replacement_name: &str, tooltip: Option<&str>) -> std::sync::Arc<CategoryInfo> {
        let category = std::sync::Arc::new(CategoryInfo {
            kind,
            target_name: component::translatable(target_name),
            replacement_name: component::translatable(replacement_name),
            tooltip: tooltip.map(component::translatable),
        });
        add_entry(list, Box::new(ShortcutCategoryEntry { category: category.clone(), thread: RowThread::new() }));
        category
    }
}

impl AbstractShortcutEntry for ShortcutCategoryEntry {}

impl ModListEntry for ShortcutCategoryEntry {
    /// java: @Override public void extractContent(GuiGraphicsExtractor graphics, int mouseX, int mouseY, boolean hovered, float a)
    fn extract_content(&mut self, list: &mut ModSelectionList, index: usize, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, _hovered: bool, _a: f32) {
        self.thread.check();
        let entry = list.list.entry(index);
        gui_graphics_extractor::centered_text(context.gl, context.minecraft, &FormattedText::from(self.category.target_name.clone()), entry.get_content_x_middle() - 85, entry.get_y() + TEXT_Y_OFFSET, common_colors::WHITE);
        gui_graphics_extractor::centered_text(context.gl, context.minecraft, &FormattedText::from(self.category.replacement_name.clone()), entry.get_content_x_middle() + 85, entry.get_y() + TEXT_Y_OFFSET, common_colors::WHITE);
        if self.category.tooltip.is_some() && entry.is_mouse_over(mouse_x as f64, mouse_y as f64) {
            // java: graphics.setTooltipForNextFrame(tooltip, mouseX, mouseY); - `setTooltipForNextFrame(font, Component, x, y)`,
            // the component's one line
            compat::batch_s_set_tooltip_for_next_frame(vec![FormattedText::from(self.category.tooltip.clone().unwrap())]);
        }
    }

    /// java: @Override public List<? extends NarratableEntry> narratables() - one entry narrating `TITLE: targetName, replacementName`
    fn get_narration(&self) -> FormattedText {
        FormattedText::from(self.category.target_name.clone().append_str(", ").append(self.category.replacement_name.clone()))
    }

    /// java: @Override public List<? extends GuiEventListener> children() - `List.of()`
    fn children(&mut self) -> Option<&mut mcrust::net::minecraft::client::gui::screens::screen::Screen> {
        None
    }

    /// java: AbstractShortcutEntry.mouseClicked - `return true` (no children)
    fn mouse_clicked(&mut self, _list: &mut ModSelectionList, _index: usize, _event: &MouseButtonEvent, _double_click: bool) -> bool {
        true
    }
}

/// java: private class ShortcutLoadingEntry extends AbstractShortcutEntry
pub struct ShortcutLoadingEntry {
    /// java: private final Component text;
    pub text: MutableComponent,
    pub thread: RowThread,
}

// SAFETY: see `RowThread`.
unsafe impl Send for ShortcutLoadingEntry {}

impl ShortcutLoadingEntry {
    /// java: private ShortcutLoadingEntry()
    pub fn new() -> ShortcutLoadingEntry {
        ShortcutLoadingEntry { text: component::translatable("skyblocker.shortcuts.notLoaded"), thread: RowThread::new() }
    }
}

impl AbstractShortcutEntry for ShortcutLoadingEntry {}

impl ModListEntry for ShortcutLoadingEntry {
    /// java: @Override public void extractContent(GuiGraphicsExtractor graphics, int mouseX, int mouseY, boolean hovered, float a)
    fn extract_content(&mut self, list: &mut ModSelectionList, index: usize, context: &mut WidgetRenderContext<'_>, _mouse_x: i32, _mouse_y: i32, _hovered: bool, _a: f32) {
        self.thread.check();
        let entry = list.list.entry(index);
        gui_graphics_extractor::centered_text(context.gl, context.minecraft, &FormattedText::from(self.text.clone()), entry.get_width() / 2, entry.get_y() + TEXT_Y_OFFSET, common_colors::WHITE);
    }

    /// java: @Override public List<? extends NarratableEntry> narratables() - `TITLE: text`
    fn get_narration(&self) -> FormattedText {
        FormattedText::from(self.text.clone())
    }

    /// java: AbstractShortcutEntry.mouseClicked - `return true` (no children)
    fn mouse_clicked(&mut self, _list: &mut ModSelectionList, _index: usize, _event: &MouseButtonEvent, _double_click: bool) -> bool {
        true
    }
}

/// java: protected abstract class ShortcutEntry<T> extends AbstractShortcutEntry
pub trait ShortcutEntry: AbstractShortcutEntry {
    /// java: protected final ShortcutCategoryEntry<T> category;
    fn category(&self) -> std::sync::Arc<CategoryInfo>;

    /// java: protected abstract boolean isNotEmpty();
    fn is_not_empty(&self) -> bool;

    /// java: protected abstract boolean isChanged();
    fn is_changed(&self) -> bool;

    /// java: protected abstract void save();
    fn save(&self);

    /// java: protected abstract String key();
    fn key(&self) -> String;

    /// java: toString() - the delete warning names the shortcut by it
    fn to_display_string(&self) -> String;
}

/// java: ShortcutEntry's fields: `category` and `replacement`, in the row's widgets (`children()`)
pub struct ShortcutEntryBase {
    /// java: protected final ShortcutCategoryEntry<T> category;
    pub category: std::sync::Arc<CategoryInfo>,
    /// the row's widgets (`children()`), the `EditBox`es and the key button
    pub children: mcrust::net::minecraft::client::gui::screens::screen::Screen,
    /// java: protected final EditBox replacement; (its position in `children`)
    pub replacement: usize,
    pub thread: RowThread,
}

impl ShortcutEntryBase {
    /// java: private ShortcutEntry(ShortcutCategoryEntry<T> category, T targetKey) - `replacement`, valued from the map
    /// (the subclass puts it in `children` at Java's `List.of(.., replacement)` position)
    fn new(list: &AbstractSelectionList, category: std::sync::Arc<CategoryInfo>, replacement_value: String) -> (ShortcutEntryBase, AbstractWidget) {
        // java: replacement = new EditBox(font, width / 2 + 10, TEXT_Y_OFFSET, 150, 20, category.replacementName);
        let mut replacement = compat::edit_box(list.get_width() / 2 + 10, TEXT_Y_OFFSET, 150, 20, category.replacement_name.clone());
        compat::with_edit_box(&mut replacement, |e| e.set_max_length(256)); // same as chat
        compat::set_edit_box_value(&mut replacement, &replacement_value);
        let base = ShortcutEntryBase {
            category,
            children: mcrust::net::minecraft::client::gui::screens::screen::Screen::new(String::new()),
            replacement: usize::MAX,
            thread: RowThread::new(),
        };
        (base, replacement)
    }

    /// `replacement.getValue()`
    pub fn replacement_value(&self) -> String {
        compat::edit_box_value(self.children.widget(self.replacement))
    }

    /// java: ShortcutEntry.extractContent - `replacement` at the row's top, then the arrow
    fn extract_content(&mut self, list: &ModSelectionList, index: usize, context: &WidgetRenderContext<'_>) {
        let entry = list.list.entry(index);
        self.children.widget_mut(self.replacement).set_y(entry.get_y() + TEXT_FIELD_PADDING);
        gui_graphics_extractor::centered_text(context.gl, context.minecraft, &FormattedText::of("→"), entry.get_x() + entry.get_width() / 2, entry.get_y() + TEXT_Y_OFFSET, common_colors::WHITE);
    }

    /// java: ShortcutEntry.updatePositions
    fn update_positions(&mut self, width: i32) {
        self.children.widget_mut(self.replacement).set_x(width / 2 + 10);
    }
}

/// java: protected class CommandShortcutEntry extends ShortcutEntry<String>
pub struct CommandShortcutEntry {
    /// java: extends ShortcutEntry<String>
    pub base: ShortcutEntryBase,
    /// java: private final EditBox target; (its position in `children` - `List.of(target, replacement)`)
    pub target: usize,
}

// SAFETY: see `RowThread`.
unsafe impl Send for CommandShortcutEntry {}

impl CommandShortcutEntry {
    /// java: private CommandShortcutEntry(ShortcutCategoryEntry<String> category)
    pub fn new(list: &AbstractSelectionList, category: std::sync::Arc<CategoryInfo>) -> CommandShortcutEntry {
        Self::new_2(list, category, "")
    }

    /// java: private CommandShortcutEntry(ShortcutCategoryEntry<String> category, String targetString)
    pub fn new_2(list: &AbstractSelectionList, category: std::sync::Arc<CategoryInfo>, target_string: &str) -> CommandShortcutEntry {
        // java: super(category, targetString); - replacement.setValue(category.shortcutsMap.getOrDefault(targetKey, ""))
        let replacement_value = with_shortcuts_data(|data| match category.kind {
            CategoryKind::CommandArgs => data.command_args.get(target_string).cloned(),
            _ => data.commands.get(target_string).cloned(),
        })
        .unwrap_or_default();
        let (mut base, replacement) = ShortcutEntryBase::new(list, category.clone(), replacement_value);
        // java: target = new EditBox(font, width / 2 - 160, TEXT_Y_OFFSET, 150, 20, category.targetName);
        let mut target = compat::edit_box(list.get_width() / 2 - 160, TEXT_Y_OFFSET, 150, 20, category.target_name.clone());
        compat::with_edit_box(&mut target, |e| e.set_max_length(256)); // same as chat
        compat::set_edit_box_value(&mut target, target_string);
        // java: children = List.of(target, replacement);
        let target = base.children.add_renderable_widget(target);
        base.replacement = base.children.add_renderable_widget(replacement);
        CommandShortcutEntry { base, target }
    }

    /// `target.getValue()`
    pub fn target_value(&self) -> String {
        compat::edit_box_value(self.base.children.widget(self.target))
    }

    /// The map this row's category edits.
    fn with_map<R>(&self, f: impl FnOnce(&mut std::collections::HashMap<String, String>) -> R) -> R {
        let kind = self.base.category.kind;
        with_shortcuts_data(|data| match kind {
            CategoryKind::CommandArgs => f(&mut data.command_args),
            _ => f(&mut data.commands),
        })
    }

    fn extract(&mut self, list: &mut ModSelectionList, index: usize, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32) {
        self.base.thread.check();
        // java: super.extractContent(..); target.setY(this.getY() + TEXT_FIELD_PADDING); target.extractRenderState(..);
        self.base.extract_content(list, index, context);
        let y = list.list.entry(index).get_y();
        self.base.children.widget_mut(self.target).set_y(y + TEXT_FIELD_PADDING);
        context.extract_widgets(&mut self.base.children, mouse_x, mouse_y);
    }
}

impl AbstractShortcutEntry for CommandShortcutEntry {
    /// java: @Override protected void updatePositions()
    fn update_positions(&mut self, width: i32) {
        self.base.update_positions(width);
        self.base.children.widget_mut(self.target).set_x(width / 2 - 160);
    }
}

impl ShortcutEntry for CommandShortcutEntry {
    fn category(&self) -> std::sync::Arc<CategoryInfo> {
        self.base.category.clone()
    }

    /// java: @Override protected boolean isNotEmpty()
    fn is_not_empty(&self) -> bool {
        return !self.target_value().is_empty() && !self.base.replacement_value().is_empty();
    }

    /// java: @Override protected boolean isChanged()
    fn is_changed(&self) -> bool {
        let (target, replacement) = (self.target_value(), self.base.replacement_value());
        self.with_map(|map| !map.contains_key(&target) || map.get(&target).map(|v| *v != replacement).unwrap_or(true))
    }

    /// java: @Override protected void save()
    fn save(&self) {
        let (target, replacement) = (self.target_value(), self.base.replacement_value());
        self.with_map(|map| map.insert(target, replacement));
    }

    /// java: @Override protected String key()
    fn key(&self) -> String {
        return self.target_value();
    }

    /// java: @Override public String toString()
    fn to_display_string(&self) -> String {
        return format!("{} → {}", self.target_value(), self.base.replacement_value());
    }
}

impl ModListEntry for CommandShortcutEntry {
    /// java: @Override public void extractContent(GuiGraphicsExtractor graphics, int mouseX, int mouseY, boolean hovered, float a)
    fn extract_content(&mut self, list: &mut ModSelectionList, index: usize, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, _hovered: bool, _a: f32) {
        self.extract(list, index, context, mouse_x, mouse_y);
    }

    /// java: @Override public List<? extends GuiEventListener> children() - `List.of(target, replacement)`
    fn children(&mut self) -> Option<&mut mcrust::net::minecraft::client::gui::screens::screen::Screen> {
        Some(&mut self.base.children)
    }

    /// java: AbstractShortcutEntry.mouseClicked
    fn mouse_clicked(&mut self, _list: &mut ModSelectionList, _index: usize, event: &MouseButtonEvent, _double_click: bool) -> bool {
        self.base.thread.check();
        abstract_entry_mouse_clicked(&mut self.base.children, event)
    }

    /// `ContainerEventHandler.keyPressed` - the focused child
    fn key_pressed(&mut self, _list: &mut ModSelectionList, _index: usize, event: &KeyEvent) -> bool {
        self.base.thread.check();
        compat::children_key_pressed(&mut self.base.children, event)
    }

    /// `ContainerEventHandler.charTyped` - the focused child
    fn char_typed(&mut self, _list: &mut ModSelectionList, _index: usize, event: &CharacterEvent) -> bool {
        self.base.thread.check();
        compat::children_char_typed(&mut self.base.children, event)
    }
}

/// java: protected class CommandArgShortcutEntry extends CommandShortcutEntry
pub struct CommandArgShortcutEntry {
    /// java: extends CommandShortcutEntry
    pub base: CommandShortcutEntry,
}

// SAFETY: see `RowThread`.
unsafe impl Send for CommandArgShortcutEntry {}

impl CommandArgShortcutEntry {
    /// java: private CommandArgShortcutEntry(ShortcutCategoryEntry<String> category)
    pub fn new(list: &AbstractSelectionList, category: std::sync::Arc<CategoryInfo>) -> CommandArgShortcutEntry {
        CommandArgShortcutEntry { base: CommandShortcutEntry::new(list, category) }
    }

    /// java: private CommandArgShortcutEntry(ShortcutCategoryEntry<String> category, String targetString)
    pub fn new_2(list: &AbstractSelectionList, category: std::sync::Arc<CategoryInfo>, target_string: &str) -> CommandArgShortcutEntry {
        CommandArgShortcutEntry { base: CommandShortcutEntry::new_2(list, category, target_string) }
    }
}

impl AbstractShortcutEntry for CommandArgShortcutEntry {
    fn update_positions(&mut self, width: i32) {
        self.base.update_positions(width);
    }
}

impl ShortcutEntry for CommandArgShortcutEntry {
    fn category(&self) -> std::sync::Arc<CategoryInfo> {
        self.base.category()
    }

    fn is_not_empty(&self) -> bool {
        self.base.is_not_empty()
    }

    fn is_changed(&self) -> bool {
        self.base.is_changed()
    }

    fn save(&self) {
        self.base.save()
    }

    /// java: @Override protected String key()
    fn key(&self) -> String {
        return format!("arg{}", self.base.key());
    }

    fn to_display_string(&self) -> String {
        self.base.to_display_string()
    }
}

impl ModListEntry for CommandArgShortcutEntry {
    fn extract_content(&mut self, list: &mut ModSelectionList, index: usize, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, hovered: bool, a: f32) {
        self.base.extract_content(list, index, context, mouse_x, mouse_y, hovered, a);
    }

    fn children(&mut self) -> Option<&mut mcrust::net::minecraft::client::gui::screens::screen::Screen> {
        self.base.children()
    }

    fn mouse_clicked(&mut self, list: &mut ModSelectionList, index: usize, event: &MouseButtonEvent, double_click: bool) -> bool {
        self.base.mouse_clicked(list, index, event, double_click)
    }

    fn key_pressed(&mut self, list: &mut ModSelectionList, index: usize, event: &KeyEvent) -> bool {
        self.base.key_pressed(list, index, event)
    }

    fn char_typed(&mut self, list: &mut ModSelectionList, index: usize, event: &CharacterEvent) -> bool {
        self.base.char_typed(list, index, event)
    }
}

/// java: protected class KeybindShortcutEntry extends ShortcutEntry<ShortcutKeyBinding>
pub struct KeybindShortcutEntry {
    /// java: extends ShortcutEntry<ShortcutKeyBinding>
    pub base: ShortcutEntryBase,
    /// java: private final ShortcutKeyBinding keyBinding; - shared with `keybindButton`, which edits it
    pub key_binding: compat::Shared<ShortcutKeyBinding>,
    /// java: private final KeybindWidget keybindButton; (the object, and its position in `children`)
    pub keybind_button: compat::Shared<KeybindWidget>,
    pub keybind_button_index: usize,
    /// java: private boolean conflicting = false;
    pub conflicting: bool,
    /// java: private boolean duplicate = false;
    pub duplicate: bool,
    /// `ShortcutsConfigListWidget.this::updateKeybinds`, which the button's update listener runs: the button is
    /// reached through this row's own call, so the listener marks it and the row runs it as the call returns
    pub update_requested: std::rc::Rc<std::cell::Cell<bool>>,
    /// `replacement.getValue()` as the key button's narration lambda reads it: the box is a widget of this row's
    /// `children`, which the lambda cannot reach, so the row keeps this copy current (`run_requested_update`, the
    /// one place an edit of the box returns through)
    pub replacement_for_narration: std::rc::Rc<std::cell::RefCell<String>>,
}

// SAFETY: see `RowThread`.
unsafe impl Send for KeybindShortcutEntry {}

impl KeybindShortcutEntry {
    /// java: private KeybindShortcutEntry(ShortcutCategoryEntry<ShortcutKeyBinding> category)
    pub fn new(list: &AbstractSelectionList, category: std::sync::Arc<CategoryInfo>) -> KeybindShortcutEntry {
        Self::new_2(list, category, ShortcutKeyBinding::new(vec![input_constants::unknown()]))
    }

    /// java: private KeybindShortcutEntry(ShortcutCategoryEntry<ShortcutKeyBinding> category, ShortcutKeyBinding keyBinding)
    /// (its closing `update()` needs the list's other rows: the caller runs it once the row has a position)
    pub fn new_2(list: &AbstractSelectionList, category: std::sync::Arc<CategoryInfo>, key_binding: ShortcutKeyBinding) -> KeybindShortcutEntry {
        // java: super(category, keyBinding);
        let replacement_value = with_shortcuts_data(|data| data.key_bindings.get(&key_binding).cloned()).unwrap_or_default();
        let replacement_for_narration = std::rc::Rc::new(std::cell::RefCell::new(replacement_value.clone()));
        let (mut base, replacement) = ShortcutEntryBase::new(list, category, replacement_value);
        let bound_keys_text = key_binding.get_bound_keys_text();
        let key_binding = compat::shared(key_binding);
        let update_requested = std::rc::Rc::new(std::cell::Cell::new(false));
        let request = update_requested.clone();
        // java: keybindButton = new KeybindWidget(keyBinding, width / 2 - 160, TEXT_Y_OFFSET, 150, 20, keyBinding.getBoundKeysText(),
        //   textSupplier -> keyBinding.isUnbound() ? translatable("narrator.controls.unbound", replacement.getValue())
        //                                          : translatable("narrator.controls.bound", replacement.getValue(), textSupplier.get()),
        //   ShortcutsConfigListWidget.this::updateKeybinds);
        let narration_key_binding = key_binding.clone();
        let narration_replacement = replacement_for_narration.clone();
        let narration: Arc<dyn Fn(&dyn Fn() -> FormattedText) -> FormattedText> = Arc::new(move |text_supplier: &dyn Fn() -> FormattedText| {
            let replacement_text = FormattedText::of(narration_replacement.borrow().clone());
            if compat::read(&narration_key_binding, |k| k.is_unbound()) {
                FormattedText::from(component::translatable_args("narrator.controls.unbound", vec![replacement_text]))
            } else {
                FormattedText::from(component::translatable_args("narrator.controls.bound", vec![replacement_text, text_supplier()]))
            }
        });
        let keybind_button = KeybindWidget::new_with_narration(key_binding.clone(), list.get_width() / 2 - 160, TEXT_Y_OFFSET, 150, 20, bound_keys_text, narration, Arc::new(move || request.set(true)));
        // java: replacement.setResponder(_ -> ShortcutsConfigListWidget.this.updateKeybinds()); - see `run_requested_update`
        // java: children = List.of(keybindButton, replacement);
        let keybind_button_index = base.children.add_renderable_widget(KeybindWidget::widget(keybind_button.clone()));
        base.replacement = base.children.add_renderable_widget(replacement);
        KeybindShortcutEntry { base, key_binding, keybind_button, keybind_button_index, conflicting: false, duplicate: false, update_requested, replacement_for_narration }
    }

    /// The key button's message (`getMessage()`): the widget half the game holds.
    fn button_message(&self) -> MutableComponent {
        let widget = self.base.children.widget(self.keybind_button_index);
        match &widget.message_text {
            Some(text) => compat::component_of(text.clone()),
            None => component::literal(&widget.message),
        }
    }

    /// `keybindButton.setMessage(message)`
    fn set_button_message(&mut self, message: MutableComponent) {
        self.base.children.widget_mut(self.keybind_button_index).set_message_text(FormattedText::from(message));
    }

    /// java: protected void update() (`index` is this row's position: Java's `keyBinding != other.keyBinding` skips it)
    pub fn update(&mut self, list: &AbstractSelectionList, index: usize) {
        let key_binding: ShortcutKeyBinding = compat::read(&self.key_binding, |k| k.clone());
        self.set_button_message(key_binding.get_bound_keys_text());
        self.conflicting = false;
        self.duplicate = false;
        let mut conflict_text: MutableComponent = component::empty();
        let mut duplicate_text: MutableComponent = component::empty();
        if !key_binding.is_unbound() {
            // Check for conflicts with regular keybinds
            let names = key_binding.get_bound_keys_translation_key();
            for other_key_binding in mcrust::net::minecraft::client::key_mapping::key_mappings() {
                // java: otherKeyBinding.saveString() - the mapping's live key
                let save_string = mcrust::net::minecraft::client::key_mapping::key_of(other_key_binding.get_name()).map(|k| k.get_name().to_string()).unwrap_or_else(|| other_key_binding.save_string().to_string());
                if names.contains(&save_string) {
                    if self.conflicting {
                        conflict_text = conflict_text.append_str(", ");
                    }
                    self.conflicting = true;
                    conflict_text = conflict_text.append(component::translatable(other_key_binding.get_name()));
                }
            }
            // Check for conflicts with other keybind shortcuts
            for i in 0..list.entry_count() {
                if i == index {
                    continue;
                }
                let Some(other) = list.entry_object(i).and_then(|slot| slot.with::<KeybindShortcutEntry, _>(|e| (compat::read(&e.key_binding, |k| k.clone()), e.base.replacement_value()))) else {
                    continue;
                };
                if key_binding == other.0 {
                    if self.duplicate {
                        duplicate_text = duplicate_text.append_str(", ");
                    }
                    self.duplicate = true;
                    // We display the replacement command to help users identify which shortcuts have conflicting keybinds.
                    duplicate_text = duplicate_text.append_str(&other.1);
                }
            }
        }
        let tooltip: Option<Tooltip>;
        if self.duplicate {
            let message = component::literal("[ ").append(self.button_message().with_style_format(ChatFormatting::White)).append_str(" ]").with_color_text(TextColor::RED);
            self.set_button_message(message);
            tooltip = Some(Tooltip::create_text(FormattedText::from(component::translatable_args("skyblocker.shortcuts.keyBinding.duplicate", vec![FormattedText::from(duplicate_text)]))));
        } else if self.conflicting {
            let message = component::literal("[ ").append(self.button_message().with_style_format(ChatFormatting::White)).append_str(" ]").with_color_text(TextColor::YELLOW);
            self.set_button_message(message);
            tooltip = Some(Tooltip::create_text(FormattedText::from(component::translatable_args("controls.keybinds.duplicateKeybinds", vec![FormattedText::from(conflict_text)]))));
        } else {
            tooltip = None;
        }
        let button = self.base.children.widget_mut(self.keybind_button_index);
        match tooltip {
            Some(tooltip) => button.set_tooltip(tooltip),
            // java: keybindButton.setTooltip(null);
            None => button.tooltip = None,
        }
        if compat::read(&self.keybind_button, |b| b.is_editing()) {
            let message = component::literal("> ").append(self.button_message().with_style_formats(&[ChatFormatting::White, ChatFormatting::Underline])).append_str(" <").with_style_format(ChatFormatting::Yellow);
            self.set_button_message(message);
        }
    }

    /// java: protected boolean stopEditing()
    pub fn stop_editing(&mut self) -> bool {
        return compat::write(&self.keybind_button, |b| b.stop_editing());
    }

    /// After a call into the row's widgets: the key button's update listener and the replacement's responder
    /// (`ShortcutsConfigListWidget.this::updateKeybinds`), which ran inside it.
    fn run_requested_update(&mut self, list: &ModSelectionList, index: usize, replacement_before: &str) {
        let replacement_now = self.base.replacement_value();
        if replacement_now != replacement_before {
            *self.replacement_for_narration.borrow_mut() = replacement_now.clone();
        }
        if self.update_requested.replace(false) || replacement_now != replacement_before {
            ShortcutsConfigListWidget::update_keybinds(&list.list, Some((index, self)));
        }
    }
}

impl AbstractShortcutEntry for KeybindShortcutEntry {
    /// java: @Override protected void updatePositions()
    fn update_positions(&mut self, width: i32) {
        self.base.update_positions(width);
        self.base.children.widget_mut(self.keybind_button_index).set_x(width / 2 - 160);
    }
}

impl ShortcutEntry for KeybindShortcutEntry {
    fn category(&self) -> std::sync::Arc<CategoryInfo> {
        self.base.category.clone()
    }

    /// java: @Override protected boolean isNotEmpty()
    fn is_not_empty(&self) -> bool {
        return !compat::read(&self.key_binding, |k| k.is_unbound()) && !self.base.replacement_value().is_empty();
    }

    /// java: @Override protected boolean isChanged()
    fn is_changed(&self) -> bool {
        let (key_binding, replacement) = (compat::read(&self.key_binding, |k| k.clone()), self.base.replacement_value());
        with_shortcuts_data(|data| !data.key_bindings.contains_key(&key_binding) || data.key_bindings.get(&key_binding).map(|v| *v != replacement).unwrap_or(true))
    }

    /// java: @Override protected void save()
    fn save(&self) {
        let (key_binding, replacement) = (compat::read(&self.key_binding, |k| k.clone()), self.base.replacement_value());
        with_shortcuts_data(|data| data.key_bindings.insert(key_binding, replacement));
    }

    /// java: @Override protected String key() - `keyBinding.getBoundKeysTranslationKey().toString()` (a `List`'s `[a, b]`)
    fn key(&self) -> String {
        return format!("[{}]", compat::read(&self.key_binding, |k| k.get_bound_keys_translation_key()).join(", "));
    }

    /// java: @Override public String toString() - used in the delete warning screen, so the localized text
    fn to_display_string(&self) -> String {
        return format!("{} → {}", compat::read(&self.key_binding, |k| k.get_bound_keys_text()).get_string(), self.base.replacement_value());
    }
}

impl ModListEntry for KeybindShortcutEntry {
    /// java: @Override public void extractContent(GuiGraphicsExtractor graphics, int mouseX, int mouseY, boolean hovered, float a)
    fn extract_content(&mut self, list: &mut ModSelectionList, index: usize, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, _hovered: bool, _a: f32) {
        self.base.thread.check();
        // java: super.extractContent(..); keybindButton.setY(this.getY() + TEXT_FIELD_PADDING); keybindButton.extractRenderState(..);
        self.base.extract_content(list, index, context);
        let entry = list.list.entry(index);
        let (y, height) = (entry.get_y(), entry.get_height());
        self.base.children.widget_mut(self.keybind_button_index).set_y(y + TEXT_FIELD_PADDING);
        context.extract_widgets(&mut self.base.children, mouse_x, mouse_y);
        if self.conflicting || self.duplicate {
            let color: i32 = if self.duplicate { common_colors::RED } else { common_colors::YELLOW };
            let x = self.base.children.widget(self.keybind_button_index).get_x();
            context.graphics.fill(context.gl, x - 6, y, x - 3, y + height, color);
        }
    }

    /// java: @Override public List<? extends GuiEventListener> children() - `List.of(keybindButton, replacement)`
    fn children(&mut self) -> Option<&mut mcrust::net::minecraft::client::gui::screens::screen::Screen> {
        Some(&mut self.base.children)
    }

    /// java: AbstractShortcutEntry.mouseClicked
    fn mouse_clicked(&mut self, list: &mut ModSelectionList, index: usize, event: &MouseButtonEvent, _double_click: bool) -> bool {
        self.base.thread.check();
        let before = self.base.replacement_value();
        let answer = abstract_entry_mouse_clicked(&mut self.base.children, event);
        self.run_requested_update(list, index, &before);
        answer
    }

    /// `ContainerEventHandler.keyPressed` - the focused child
    fn key_pressed(&mut self, list: &mut ModSelectionList, index: usize, event: &KeyEvent) -> bool {
        self.base.thread.check();
        let before = self.base.replacement_value();
        let answer = compat::children_key_pressed(&mut self.base.children, event);
        self.run_requested_update(list, index, &before);
        answer
    }

    /// `ContainerEventHandler.charTyped` - the focused child
    fn char_typed(&mut self, list: &mut ModSelectionList, index: usize, event: &CharacterEvent) -> bool {
        self.base.thread.check();
        let before = self.base.replacement_value();
        let answer = compat::children_char_typed(&mut self.base.children, event);
        self.run_requested_update(list, index, &before);
        answer
    }
}

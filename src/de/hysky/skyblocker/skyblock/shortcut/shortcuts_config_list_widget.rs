//! Ported from de.hysky.skyblocker.skyblock.shortcut.ShortcutsConfigListWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::shortcut::keybind_widget::KeybindWidget;
use crate::de::hysky::skyblocker::skyblock::shortcut::shortcut_key_binding::ShortcutKeyBinding;
use crate::de::hysky::skyblocker::skyblock::shortcut::shortcuts;
use crate::de::hysky::skyblocker::skyblock::shortcut::shortcuts::Shortcuts;
use crate::de::hysky::skyblocker::skyblock::shortcut::shortcuts_config_screen::ShortcutsConfigScreen;
use mcrust::mcrust::modding::widgets::ModList;
use mcrust::mcrust::modding::widgets::ModListEntry;
use mcrust::mcrust::modding::widgets::ModSelectionList;
use mcrust::mcrust::modding::widgets::WidgetRenderContext;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::components::abstract_selection_list::AbstractSelectionList;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::edit_box::EditBox;
use mcrust::net::minecraft::client::gui::components::events::gui_event_listener::GuiEventListener;
use mcrust::net::minecraft::client::gui::components::tooltip::Tooltip;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::narration::narratable_entry::NarratableEntry;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use crate::de::hysky::skyblocker::utils::data::json_data::JsonData as _;
use mcrust::net::minecraft::client::gui::components::renderable::Renderable as _;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final int TEXT_Y_OFFSET = 5 + 2;
pub const TEXT_Y_OFFSET: i32 = 5i32.wrapping_add(2);
/// java: private static final int TEXT_FIELD_PADDING = 2;
pub const TEXT_FIELD_PADDING: i32 = 2;

/// java: public class ShortcutsConfigListWidget extends ContainerObjectSelectionList<ShortcutsConfigListWidget.AbstractShortcutEntry>
pub struct ShortcutsConfigListWidget {
    /// java: extends net.minecraft.client.gui.components.ContainerObjectSelectionList
    pub base: ModSelectionList,
    /// java: private final ShortcutsConfigScreen screen;
    pub screen: compat::Shared<ShortcutsConfigScreen>,
}

impl std::ops::Deref for ShortcutsConfigListWidget {
    type Target = AbstractSelectionList;
    fn deref(&self) -> &Self::Target {
        &self.base.list
    }
}

impl std::ops::DerefMut for ShortcutsConfigListWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base.list
    }
}

impl ShortcutsConfigListWidget {
    /// java: public ShortcutsConfigListWidget(Minecraft minecraftClient, ShortcutsConfigScreen screen, int width, int height, int y)
    pub fn new(minecraft_client: &mut mcrust::net::minecraft::client::minecraft::Minecraft, mut screen: compat::Shared<ShortcutsConfigScreen>, mut width: i32, mut height: i32, mut y: i32) -> compat::Shared<Self> {
        let __init_screen = screen.clone();
        // TODO(fabric2seer): `.commands(..)` on a T
        //   java: ShortcutCategoryEntry<String> commandCategory = new ShortcutCategoryEntry<>(Shortcuts.shortcuts.getData().commands(), CommandShortcutEntry::new, "skyblocker.shortcuts.command.target", "skyblocker.shortcuts.command.replacement");
        let mut command_category: compat::Shared<ShortcutCategoryEntry<String>> = todo!("fabric2seer: the initialiser of `commandCategory` (TODO above)");
        let __this = compat::shared(Self {
            base: mcrust::mcrust::modding::widgets::ModSelectionList::new(mcrust::mcrust::modding::widgets::ListKind::Container, width, height, y, 24),
            screen: __init_screen,
        });
        if Shortcuts::is_shortcuts_loaded() {
            for command_target in compat::sorted(command_category.lock().borrow().shortcuts_map.keys().cloned()) {
                let __entry = ((CommandShortcutEntryBase::new_2(command_category.clone(), command_target.clone()) as compat::Shared<dyn CommandShortcutEntry>) as compat::Shared<dyn AbstractShortcutEntry>);
                compat::write(&__this, |__t| { let __l = &mut __t.base; (__l.add_entry(Box::new(compat::EntryAdapter::new(__entry))) as i32) });
            }
        } else {
            let __entry = (ShortcutLoadingEntry::new() as compat::Shared<dyn AbstractShortcutEntry>);
            compat::write(&__this, |__t| { let __l = &mut __t.base; (__l.add_entry(Box::new(compat::EntryAdapter::new(__entry))) as i32) });
        }
        // TODO(fabric2seer): `.commandArgs(..)` on a T
        //   java: ShortcutCategoryEntry<String> commandArgCategory = new ShortcutCategoryEntry<>(Shortcuts.shortcuts.getData().commandArgs(), CommandArgShortcutEntry::new, "skyblocker.shortcuts.commandArg.target", "skyblocker.shortcuts.commandArg.replacement", "skyblocker.shortcuts.commandArg.tooltip");
        let mut command_arg_category: compat::Shared<ShortcutCategoryEntry<String>> = todo!("fabric2seer: the initialiser of `commandArgCategory` (TODO above)");
        if Shortcuts::is_shortcuts_loaded() {
            for command_arg_target in compat::sorted(command_arg_category.lock().borrow().shortcuts_map.keys().cloned()) {
                let __entry = (CommandArgShortcutEntry::new_2(command_arg_category.clone(), command_arg_target.clone()) as compat::Shared<dyn AbstractShortcutEntry>);
                compat::write(&__this, |__t| { let __l = &mut __t.base; (__l.add_entry(Box::new(compat::EntryAdapter::new(__entry))) as i32) });
            }
        } else {
            let __entry = (ShortcutLoadingEntry::new() as compat::Shared<dyn AbstractShortcutEntry>);
            compat::write(&__this, |__t| { let __l = &mut __t.base; (__l.add_entry(Box::new(compat::EntryAdapter::new(__entry))) as i32) });
        }
        // TODO(fabric2seer): `.keyBindings(..)` on a T
        //   java: ShortcutCategoryEntry<ShortcutKeyBinding> keybindCategory = new ShortcutCategoryEntry<>(Shortcuts.shortcuts.getData().keyBindings(), KeybindShortcutEntry::new, "skyblocker.shortcuts.keyBinding.target", "skyblocker.shortcuts.keyBinding.replacement", "skyblocker.shortcuts.keyBinding.tooltip");
        let mut keybind_category: compat::Shared<ShortcutCategoryEntry<compat::Shared<ShortcutKeyBinding>>> = todo!("fabric2seer: the initialiser of `keybindCategory` (TODO above)");
        if Shortcuts::is_shortcuts_loaded() {
            for key_binding in compat::sorted(keybind_category.lock().borrow().shortcuts_map.keys().cloned()) {
                // TODO(fabric2seer): `.copy(..)` on a T
                //   java: addEntry(new KeybindShortcutEntry(keybindCategory, keyBinding.copy()));
            }
        } else {
            let __entry = (ShortcutLoadingEntry::new() as compat::Shared<dyn AbstractShortcutEntry>);
            compat::write(&__this, |__t| { let __l = &mut __t.base; (__l.add_entry(Box::new(compat::EntryAdapter::new(__entry))) as i32) });
        }
        __this
    }

    /// java: @Override public int getRowWidth()
    pub fn get_row_width(__this: &compat::Shared<Self>) -> i32 {
        return ((compat::super_answer::<i32>()).wrapping_add(100)).wrapping_add(2i32.wrapping_mul(TEXT_FIELD_PADDING));
    }

    /// java: @Override protected int scrollBarX()
    pub fn scroll_bar_x(__this: &compat::Shared<Self>) -> i32 {
        return compat::super_answer::<i32>();
    }

    /// java: protected Optional<ShortcutCategoryEntry<?>> getCategory()
    pub fn get_category(__this: &compat::Shared<Self>) -> Option<compat::Shared<ShortcutCategoryEntry<compat::Object>>> {
        // TODO(fabric2seer): a pattern switch expression
        //   java: return switch (getSelected()) {
        //   java: 			case ShortcutCategoryEntry<?> category -> Optional.of(category);
        //   java: 			case ShortcutEntry<?> shortcutEntry -> Optional.of(shortcutEntry.category);
        //   java:
        //   java: 			case null, default -> Optional.empty();
        //   java: 		};
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public void setSelected(ShortcutsConfigListWidget.@Nullable AbstractShortcutEntry entry)
    pub fn set_selected(__this: &compat::Shared<Self>, mut entry: Option<compat::Shared<dyn AbstractShortcutEntry>>) {
        compat::call_super();
        ShortcutsConfigScreen::update_buttons(&compat::read(__this, |__t| __t.screen.clone()));
    }

    /// java: protected void addShortcutAfterSelected()
    pub fn add_shortcut_after_selected(__this: &compat::Shared<Self>) {
        if let Some(category) = ShortcutsConfigListWidget::get_category(__this) {
            let mut new_entries: Vec<compat::Shared<dyn AbstractShortcutEntry>> = compat::write(__this, |__t| { let __l = &mut __t.base; compat::list_entries::<dyn AbstractShortcutEntry>(__l) }).into_iter().collect::<Vec<_>>();
            let mut new_entry: compat::Shared<dyn ShortcutEntry<compat::Object>> = ((compat::read(&category, |__t| __t.entry_supplier.clone()).unwrap()))();
            new_entries.insert((((compat::write(__this, |__t| { let __l = &mut __t.base; compat::list_entries::<dyn AbstractShortcutEntry>(__l) })).iter().position(|e| *e == compat::write(__this, |__t| { let __l = &mut __t.base; __l.get_selected().and_then(|__i| compat::entry_at::<dyn AbstractShortcutEntry>(__l, __i)) }).unwrap()).map_or(-1, |i| i as i32)).wrapping_add(1)) as usize, (new_entry.clone() as compat::Shared<dyn AbstractShortcutEntry>));
            { let (__q0,) = (new_entries.clone(),); __this.lock().borrow_mut().replace_entries(__q0) };
            ShortcutsConfigListWidget::set_selected(__this, Some((new_entry.clone() as compat::Shared<dyn AbstractShortcutEntry>)));
        }
    }

    /// java: protected void updatePositions()
    pub fn update_positions(__this: &compat::Shared<Self>) {
        for child in compat::write(__this, |__t| { let __l = &mut __t.base; compat::list_entries::<dyn AbstractShortcutEntry>(__l) }) {
            ({ let __f = compat::dispatch(&child, |__t| __t.update_positions()); __f(&child) });
        }
    }

    /// java: protected void updateKeybinds()
    pub fn update_keybinds(__this: &compat::Shared<Self>) {
        // TODO(fabric2seer): `.isInstance(..)` on a Class
        //   java: children().stream().filter(KeybindShortcutEntry.class::isInstance).map(KeybindShortcutEntry.class::cast).forEach(KeybindShortcutEntry::update);
    }

    /// java: protected boolean stopEditing()
    pub fn stop_editing(__this: &compat::Shared<Self>) -> bool {
        // TODO(fabric2seer): `.isInstance(..)` on a Class
        //   java: boolean bl = children().stream().filter(KeybindShortcutEntry.class::isInstance).map(KeybindShortcutEntry.class::cast).anyMatch(KeybindShortcutEntry::stopEditing);
        let mut bl: bool = todo!("fabric2seer: the initialiser of `bl` (TODO above)");
        ShortcutsConfigScreen::check_for_duplicates(&compat::read(__this, |__t| __t.screen.clone()));
        return bl;
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent event, boolean doubleClick)
    pub fn mouse_clicked(__this: &compat::Shared<Self>, event: &MouseButtonEvent, mut double_click: bool) -> bool {
        // TODO(fabric2seer): `AbstractContainerWidget::mouse_clicked` ports `mouseClicked`, but the port's `AbstractSelectionList` holds no `AbstractContainerWidget` (no Deref, no `base` field) to call it on
        if { let (__q0, __q1,) = (event.clone(), double_click,); __this.lock().borrow_mut().base.list.mouse_clicked(__q0, __q1) } {
            return true;
        }
        if ShortcutsConfigListWidget::stop_editing(__this) {
            ShortcutsConfigListWidget::update_keybinds(__this);
        }
        ShortcutsConfigScreen::check_for_duplicates(&compat::read(__this, |__t| __t.screen.clone()));
        return false;
    }

    /// java: @Override protected boolean entriesCanBeSelected()
    pub fn entries_can_be_selected(__this: &compat::Shared<Self>) -> bool {
        return true;
    }

    /// java: @Override protected void removeEntry(AbstractShortcutEntry entry)
    pub fn remove_entry(__this: &compat::Shared<Self>, entry: &compat::Shared<dyn AbstractShortcutEntry>) {
        let __entry = entry.clone();
        compat::write(__this, |__t| { let __l = &mut __t.base; { if let Some(__i) = compat::entry_index::<dyn AbstractShortcutEntry>(__l, &__entry) { __l.remove_entry(__i); } } });
    }

    /// java: protected boolean hasChanges()
    pub fn has_changes(__this: &compat::Shared<Self>) -> bool {
        let mut not_empty_shortcuts: Vec<compat::Shared<dyn ShortcutEntry<compat::Object>>> = ShortcutsConfigListWidget::get_not_empty_shortcuts(__this).collect::<Vec<_>>();
        // TODO(fabric2seer): `.size(..)` on a T
        //   java: return notEmptyShortcuts.length != Shortcuts.shortcuts.getData().size() || Arrays.stream(notEmptyShortcuts).anyMatch(ShortcutEntry::isChanged);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: protected boolean hasDuplicates()
    pub fn has_duplicates(__this: &compat::Shared<Self>) -> bool {
        let mut keys: std::collections::HashSet<String> = std::collections::HashSet::new();
        for entry in compat::write(__this, |__t| { let __l = &mut __t.base; compat::list_entries::<dyn AbstractShortcutEntry>(__l) }) {
            if let Some(shortcut_entry) = entry.lock().borrow().as_any().downcast_ref::<compat::Shared<dyn ShortcutEntry<compat::Object>>>() {
                let mut key: String = shortcut_entry.lock().borrow().key();
                if keys.contains(&key) {
                    return true;
                }
                keys.insert(key.clone());
            }
        }
        return false;
    }

    /// java: protected void saveShortcuts()
    pub fn save_shortcuts(__this: &compat::Shared<Self>) {
        // TODO(fabric2seer): `.clear(..)` on a T
        //   java: Shortcuts.shortcuts.getData().clear();
        for __a0 in ShortcutsConfigListWidget::get_not_empty_shortcuts(__this) {
            __a0.lock().borrow().save();
        }
        shortcuts::SHORTCUTS.get_mut().lock().borrow().save();
    }

    /// java: private Stream<? extends ShortcutEntry<?>> getNotEmptyShortcuts()
    pub fn get_not_empty_shortcuts(__this: &compat::Shared<Self>) -> Box<dyn Iterator<Item = compat::Shared<dyn ShortcutEntry<compat::Object>>> + '_> {
        // TODO(fabric2seer): `.isInstance(..)` on a Class
        //   java: return children().stream().filter(ShortcutEntry.class::isInstance).map(e -> (ShortcutEntry<?>) e).filter(ShortcutEntry::isNotEmpty);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}

/// ShortcutsConfigListWidget's list (the port's `ModSelectionList`) until the game takes it (`compat::attach_list`)
impl compat::ListHalf for ShortcutsConfigListWidget {
    fn list_half(&mut self) -> &mut ModSelectionList {
        &mut self.base
    }
}

/// ShortcutsConfigListWidget's overrides of the list's own methods, asked by the port's list bodies
impl ModList for compat::ListClass<ShortcutsConfigListWidget> {
    /// java: @Override public int getRowWidth()
    fn get_row_width(&self, list: &AbstractSelectionList, super_answer: i32) -> i32 {
        self.ask(list, super_answer, |__h| ShortcutsConfigListWidget::get_row_width(__h))
    }

    /// java: @Override protected int scrollBarX()
    fn scroll_bar_x(&self, list: &AbstractSelectionList, super_answer: i32) -> i32 {
        self.ask(list, super_answer, |__h| ShortcutsConfigListWidget::scroll_bar_x(__h))
    }

    /// java: @Override protected boolean entriesCanBeSelected()
    fn entries_can_be_selected(&self, list: &AbstractSelectionList, super_answer: bool) -> bool {
        self.ask(list, super_answer, |__h| ShortcutsConfigListWidget::entries_can_be_selected(__h))
    }

    /// java: @Override public void setSelected(ShortcutsConfigListWidget.@Nullable AbstractShortcutEntry entry)
    fn set_selected(&self, list: &mut AbstractSelectionList, selected: Option<usize>, super_set_selected: &mut dyn FnMut(&mut AbstractSelectionList, Option<usize>)) {
        super_set_selected(list, selected);
        let __entry = selected.and_then(|__i| compat::row_object::<dyn AbstractShortcutEntry>(list, __i));
        compat::with_super(&mut || {}, || self.ask(list, (), |__h| ShortcutsConfigListWidget::set_selected(__h, __entry.clone())))
    }

}

// NOTE(fabric2seer): entriesCanBeSelected, getRowWidth, mouseClicked, removeEntry, scrollBarX, setSelected override(s) of ContainerObjectSelectionList - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them

/// java: public abstract static class AbstractShortcutEntry extends ContainerObjectSelectionList.Entry<AbstractShortcutEntry>
pub struct AbstractShortcutEntryBase {
    /// java: extends net.minecraft.client.gui.components.ContainerObjectSelectionList$Entry
    pub base: mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry,
    /// the row's widgets (`children()`): lent to the object by its `compat::EntryAdapter`
    pub __row: mcrust::net::minecraft::client::gui::screens::screen::Screen,
    /// the screen's widgets as this object last saw them (see `compat::WidgetCache`)
    pub __widgets: compat::WidgetCache,
}

impl std::ops::Deref for AbstractShortcutEntryBase {
    type Target = mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for AbstractShortcutEntryBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl AbstractShortcutEntryBase {
    /// java: the implicit no-argument constructor (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_in<__R>(__wrap: impl FnOnce(Self) -> (compat::Shared<dyn AbstractShortcutEntry>, __R)) -> (compat::Shared<dyn AbstractShortcutEntry>, __R) {
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

/// java: the virtual methods of AbstractShortcutEntry (implemented by AbstractShortcutEntryBase and every subclass)
pub trait AbstractShortcutEntry: compat::DynAny {
    fn abstract_shortcut_entry_base(&self) -> &AbstractShortcutEntryBase;
    fn abstract_shortcut_entry_base_mut(&mut self) -> &mut AbstractShortcutEntryBase;
    fn as_any(&self) -> &dyn std::any::Any;

    /// java: protected void updatePositions() (dispatch: the body for this class)
    fn update_positions(&self) -> fn(&compat::Shared<dyn AbstractShortcutEntry>) {
        <dyn AbstractShortcutEntry>::update_positions_body
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled) (dispatch: the body for this class)
    fn mouse_clicked(&self) -> fn(&compat::Shared<dyn AbstractShortcutEntry>, &MouseButtonEvent, bool) -> bool {
        <dyn AbstractShortcutEntry>::mouse_clicked_body
    }

}

impl dyn AbstractShortcutEntry {
    /// java: protected void updatePositions()
    pub fn update_positions_body(__this: &compat::Shared<dyn AbstractShortcutEntry>) {
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    pub fn mouse_clicked_body(__this: &compat::Shared<dyn AbstractShortcutEntry>, click: &MouseButtonEvent, mut doubled: bool) -> bool {
        // TODO(fabric2seer): `mouse_clicked` takes 4 argument(s) in the port (`_button: i32, _x: f64, _y: f64, _double_click: bool`), Java passes 2
        // TODO(fabric2seer): `mouse_clicked` is a method of the port's trait `GuiEventListener`, which the port's `Entry` does not implement
        // rust: { let (__q0, __q1,) = (click.clone(), doubled,); __this.lock().borrow_mut().abstract_shortcut_entry_base_mut().base.mouse_clicked(__q0, __q1) };
        return true;
    }

}

/// AbstractShortcutEntry's row box and its widgets, lent by its `compat::EntryAdapter` for each call
impl compat::RowHalf for dyn AbstractShortcutEntry {
    fn entry_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry {
        &mut self.abstract_shortcut_entry_base_mut().base
    }
    fn row_parts(&mut self) -> (&mut mcrust::net::minecraft::client::gui::screens::screen::Screen, &mut compat::WidgetCache) {
        let __s = &mut self.abstract_shortcut_entry_base_mut();
        (&mut __s.__row, &mut __s.__widgets)
    }
}

/// AbstractShortcutEntry as a row of the game's list (`addEntry` attaches it)
impl ModListEntry for compat::EntryAdapter<dyn AbstractShortcutEntry> {
    /// java: @Override public void extractContent(GuiGraphicsExtractor graphics, int mouseX, int mouseY, boolean hovered, float a)
    fn extract_content(&mut self, list: &mut ModSelectionList, index: usize, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, hovered: bool, a: f32) {
        self.lend(Some(list.list.entry(index)), |__h| { if let Some(__c) = compat::try_view::<ShortcutLoadingEntry, _>(__h) { return ShortcutLoadingEntry::extract_content(&*__c, &context.graphics, mouse_x, mouse_y, hovered, a); }  });
        // the row's widgets (Java's `child.extractRenderState(..)` over `children()`)
        context.extract_widgets(&mut self.row, mouse_x, mouse_y);
    }

    fn children(&mut self) -> Option<&mut mcrust::net::minecraft::client::gui::screens::screen::Screen> {
        Some(&mut self.row)
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    fn mouse_clicked(&mut self, list: &mut ModSelectionList, index: usize, event: &MouseButtonEvent, double_click: bool) -> bool {
        self.lend(Some(list.list.entry(index)), |__h| { if let Some(__c) = compat::try_view::<CommandArgShortcutEntry, _>(__h) { return ({ let __f = compat::dispatch(&*__c, |__t| __t.mouse_clicked()); __f(&compat::view(&*__c, |a| a as compat::Shared<dyn AbstractShortcutEntry>), event, double_click) }); } if let Some(__c) = compat::try_view::<KeybindShortcutEntry, _>(__h) { return ({ let __f = compat::dispatch(&*__c, |__t| __t.mouse_clicked()); __f(&compat::view(&*__c, |a| a as compat::Shared<dyn AbstractShortcutEntry>), event, double_click) }); } if let Some(__c) = compat::try_view::<ShortcutLoadingEntry, _>(__h) { return ({ let __f = compat::dispatch(&*__c, |__t| __t.mouse_clicked()); __f(&compat::view(&*__c, |a| a as compat::Shared<dyn AbstractShortcutEntry>), event, double_click) }); } false })
    }

}

/// java: protected class ShortcutCategoryEntry<T> extends AbstractShortcutEntry
pub struct ShortcutCategoryEntry<T> {
    /// java: extends AbstractShortcutEntry
    pub base: AbstractShortcutEntryBase,
    /// java: private final Map<T, String> shortcutsMap;
    pub shortcuts_map: std::collections::HashMap<T, String>,
    /// java: private final Supplier<ShortcutEntry<T>> entrySupplier;
    pub entry_supplier: Option<Arc<dyn Fn() -> compat::Shared<dyn ShortcutEntry<T>>>>,
    /// java: private final Component targetName;
    pub target_name: Option<MutableComponent>,
    /// java: private final Component replacementName;
    pub replacement_name: Option<MutableComponent>,
    /// java: private final @Nullable Component tooltip;
    pub tooltip: Option<MutableComponent>,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl<T: 'static> std::ops::Deref for ShortcutCategoryEntry<T> {
    type Target = AbstractShortcutEntryBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<T: 'static> std::ops::DerefMut for ShortcutCategoryEntry<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl<T: Clone + 'static> ShortcutCategoryEntry<T> {
    /// java: private ShortcutCategoryEntry(Map<T, String> shortcutsMap, Function<ShortcutCategoryEntry<T>, ShortcutEntry<T>> entryFactory, @Translatable String targetName, @Translatable String replacementName)
    pub fn new(mut shortcuts_map: std::collections::HashMap<T, String>, entry_factory: Arc<dyn Fn(compat::Shared<ShortcutCategoryEntry<T>>) -> compat::Shared<dyn ShortcutEntry<T>>>, mut target_name: String, mut replacement_name: String) -> compat::Shared<Self> {
        let __this = Self::new3(shortcuts_map.clone(), entry_factory.clone(), target_name.clone(), replacement_name.clone(), Some(None));
        __this
    }

    /// java: private ShortcutCategoryEntry(Map<T, String> shortcutsMap, Function<ShortcutCategoryEntry<T>, ShortcutEntry<T>> entryFactory, @Translatable String targetName, @Translatable String replacementName, @Tr
    pub fn new2(mut shortcuts_map: std::collections::HashMap<T, String>, entry_factory: Arc<dyn Fn(compat::Shared<ShortcutCategoryEntry<T>>) -> compat::Shared<dyn ShortcutEntry<T>>>, mut target_name: String, mut replacement_name: String, mut tooltip: String) -> compat::Shared<Self> {
        let __this = Self::new3(shortcuts_map.clone(), entry_factory.clone(), target_name.clone(), replacement_name.clone(), Some(component::translatable(&tooltip)));
        __this
    }

    /// java: private ShortcutCategoryEntry(Map<T, String> shortcutsMap, Function<ShortcutCategoryEntry<T>, ShortcutEntry<T>> entryFactory, @Translatable String targetName, @Translatable String replacementName, @Nu
    pub fn new3(mut shortcuts_map: std::collections::HashMap<T, String>, entry_factory: Arc<dyn Fn(compat::Shared<ShortcutCategoryEntry<T>>) -> compat::Shared<dyn ShortcutEntry<T>>>, mut target_name: String, mut replacement_name: String, mut tooltip: Option<MutableComponent>) -> compat::Shared<Self> {
        let __init_shortcuts_map = shortcuts_map.clone();
        let (_, __this) = AbstractShortcutEntryBase::new_in(move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                self_ref: std::sync::Weak::new(),
                shortcuts_map: __init_shortcuts_map,
                entry_supplier: None,
                target_name: None,
                replacement_name: None,
                tooltip: None,
            });
            compat::write(&__h, |__t| __t.self_ref = std::sync::Arc::downgrade(&__h));
            ((__h.clone() as compat::Shared<dyn AbstractShortcutEntry>), __h)
        });
        let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(&__this));
        { let __v = Some(Arc::new(move || { let Some(__this_1) = __this_w1.get().upgrade() else { return Default::default(); }; (entry_factory)(__this_1) })); compat::write(&__this, |__t| __t.entry_supplier = __v); }
        { let __v = Some(component::translatable(&target_name)); compat::write(&__this, |__t| __t.target_name = __v); }
        { let __v = Some(component::translatable(&replacement_name)); compat::write(&__this, |__t| __t.replacement_name = __v); }
        { let __v = tooltip.clone(); compat::write(&__this, |__t| __t.tooltip = __v); }
        // TODO(fabric2seer): `Entry.addEntry` has no Rust method in the port (looked for `add_entry` in ContainerObjectSelectionList and JavaSrc)
        // rust: let __h0 = &__this;
        // rust: __this.lock().borrow().add_entry(__h0);
        __this
    }

    /// java: @Override public List<? extends GuiEventListener> children()
    pub fn children(__this: &compat::Shared<Self>) -> Vec<Box<dyn GuiEventListener>> {
        return vec![];
    }

    /// java: @Override public List<? extends NarratableEntry> narratables()
    pub fn narratables(__this: &compat::Shared<Self>) -> Vec<Box<dyn NarratableEntry>> {
        // TODO(fabric2seer): `new NarratableEntry(..) {..}` (an anonymous class with state or several methods)
        //   java: return List.of(new NarratableEntry() {
        //   java: 				@Override
        //   java: 				public NarrationPriority narrationPriority() {
        //   java: 					return NarrationPriority.HOVERED;
        //   java: 				}
        //   java:
        //   java: 				@Override
        //   java: 				public void updateNarration(NarrationElementOutput builder) {
        //   java: 					builder.add(NarratedElementType.TITLE, targetName, replacementName);
        //   java: 				}
        //   java: 			});
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public void extractContent(GuiGraphicsExtractor graphics, int mouseX, int mouseY, boolean hovered, float a)
    pub fn extract_content(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut hovered: bool, mut a: f32) {
        // TODO(fabric2seer): unknown name `minecraft`
        //   java: graphics.centeredText(minecraft.font, targetName, getContentXMiddle() - 85, getY() + TEXT_Y_OFFSET, CommonColors.WHITE);
        // TODO(fabric2seer): unknown name `minecraft`
        //   java: graphics.centeredText(minecraft.font, replacementName, getContentXMiddle() + 85, getY() + TEXT_Y_OFFSET, CommonColors.WHITE);
        if compat::read(__this, |__t| __t.tooltip.clone()).is_some() && { let (__q0, __q1,) = ((mouse_x) as f64, (mouse_y) as f64,); __this.lock().borrow().is_mouse_over(__q0, __q1) } {
            // TODO(fabric2seer): the port's `set_tooltip_for_next_frame` ports this Java method with other parameters (tooltip_for_next_frame, component for Java's component, x, y)
            // TODO(fabric2seer): `set_tooltip_for_next_frame` takes 2 argument(s) in the port (`tooltip_for_next_frame: &mut Option<DeferredTooltip>, component: crate::net::minecraft::network::chat::formatted_text::FormattedText`), Java passes 3
            // rust: gui_graphics_extractor::set_tooltip_for_next_frame(compat::read(__this, |__t| __t.tooltip.clone()).as_mut().unwrap(), mouse_x, mouse_y);
        }
    }

}

impl<T: Clone + 'static> AbstractShortcutEntry for ShortcutCategoryEntry<T> {
    fn abstract_shortcut_entry_base(&self) -> &AbstractShortcutEntryBase {
        &self.base
    }
    fn abstract_shortcut_entry_base_mut(&mut self) -> &mut AbstractShortcutEntryBase {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}

/// java: private class ShortcutLoadingEntry extends AbstractShortcutEntry
pub struct ShortcutLoadingEntry {
    /// java: extends AbstractShortcutEntry
    pub base: AbstractShortcutEntryBase,
    /// java: private final Component text;
    pub text: MutableComponent,
}

impl std::ops::Deref for ShortcutLoadingEntry {
    type Target = AbstractShortcutEntryBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for ShortcutLoadingEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl ShortcutLoadingEntry {
    /// java: private ShortcutLoadingEntry()
    pub fn new() -> compat::Shared<Self> {
        let __init_text = component::translatable("skyblocker.shortcuts.notLoaded");
        let (_, __this) = AbstractShortcutEntryBase::new_in(move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                text: __init_text,
            });
            ((__h.clone() as compat::Shared<dyn AbstractShortcutEntry>), __h)
        });
        __this
    }

    /// java: @Override public List<? extends GuiEventListener> children()
    pub fn children(__this: &compat::Shared<Self>) -> Vec<Box<dyn GuiEventListener>> {
        return vec![];
    }

    /// java: @Override public List<? extends NarratableEntry> narratables()
    pub fn narratables(__this: &compat::Shared<Self>) -> Vec<Box<dyn NarratableEntry>> {
        // TODO(fabric2seer): `new NarratableEntry(..) {..}` (an anonymous class with state or several methods)
        //   java: return List.of(new NarratableEntry() {
        //   java: 				@Override
        //   java: 				public NarrationPriority narrationPriority() {
        //   java: 					return NarrationPriority.HOVERED;
        //   java: 				}
        //   java:
        //   java: 				@Override
        //   java: 				public void updateNarration(NarrationElementOutput builder) {
        //   java: 					builder.add(NarratedElementType.TITLE, text);
        //   java: 				}
        //   java: 			});
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public void extractContent(GuiGraphicsExtractor graphics, int mouseX, int mouseY, boolean hovered, float a)
    pub fn extract_content(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut hovered: bool, mut a: f32) {
        // TODO(fabric2seer): unknown name `minecraft`
        //   java: graphics.centeredText(minecraft.font, text, this.getWidth() / 2, this.getY() + TEXT_Y_OFFSET, CommonColors.WHITE);
    }

}

impl AbstractShortcutEntry for ShortcutLoadingEntry {
    fn abstract_shortcut_entry_base(&self) -> &AbstractShortcutEntryBase {
        &self.base
    }
    fn abstract_shortcut_entry_base_mut(&mut self) -> &mut AbstractShortcutEntryBase {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}

/// ShortcutLoadingEntry's row box and its widgets, lent by its `compat::EntryAdapter` for each call
impl compat::RowHalf for ShortcutLoadingEntry {
    fn entry_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry {
        &mut self.base.base
    }
    fn row_parts(&mut self) -> (&mut mcrust::net::minecraft::client::gui::screens::screen::Screen, &mut compat::WidgetCache) {
        let __s = &mut self.base;
        (&mut __s.__row, &mut __s.__widgets)
    }
}

/// ShortcutLoadingEntry as a row of the game's list (`addEntry` attaches it)
impl ModListEntry for compat::EntryAdapter<ShortcutLoadingEntry> {
    /// java: @Override public void extractContent(GuiGraphicsExtractor graphics, int mouseX, int mouseY, boolean hovered, float a)
    fn extract_content(&mut self, list: &mut ModSelectionList, index: usize, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, hovered: bool, a: f32) {
        self.lend(Some(list.list.entry(index)), |__h| ShortcutLoadingEntry::extract_content(__h, &context.graphics, mouse_x, mouse_y, hovered, a));
        // the row's widgets (Java's `child.extractRenderState(..)` over `children()`)
        context.extract_widgets(&mut self.row, mouse_x, mouse_y);
    }

    fn children(&mut self) -> Option<&mut mcrust::net::minecraft::client::gui::screens::screen::Screen> {
        Some(&mut self.row)
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    fn mouse_clicked(&mut self, list: &mut ModSelectionList, index: usize, event: &MouseButtonEvent, double_click: bool) -> bool {
        self.lend(Some(list.list.entry(index)), |__h| ({ let __f = compat::dispatch(__h, |__t| __t.mouse_clicked()); __f(&compat::view(__h, |a| a as compat::Shared<dyn AbstractShortcutEntry>), event, double_click) }))
    }

}

/// java: protected abstract class ShortcutEntry<T> extends AbstractShortcutEntry
pub struct ShortcutEntryBase<T> {
    /// java: extends AbstractShortcutEntry
    pub base: AbstractShortcutEntryBase,
    /// java: protected final ShortcutCategoryEntry<T> category;
    pub category: compat::Shared<ShortcutCategoryEntry<T>>,
    /// java: protected final EditBox replacement;
    pub replacement: EditBox,
}

impl<T: 'static> std::ops::Deref for ShortcutEntryBase<T> {
    type Target = AbstractShortcutEntryBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<T: 'static> std::ops::DerefMut for ShortcutEntryBase<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl<T: Clone + 'static> ShortcutEntryBase<T> {
    /// java: private ShortcutEntry(ShortcutCategoryEntry<T> category, T targetKey) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_in<__R>(mut category: compat::Shared<ShortcutCategoryEntry<T>>, mut target_key: T, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn ShortcutEntry>, __R)) -> (compat::Shared<dyn ShortcutEntry>, __R) {
        let __init_category = category.clone();
        // TODO(fabric2seer): `new` takes 5 argument(s) in the port (`splitter: &StringSplitter<W>, x: i32, y: i32, width: i32, height: i32`), Java passes 6
        //   java: replacement = new EditBox(Minecraft.getInstance().font, width / 2 + 10, TEXT_Y_OFFSET, 150, 20, category.replacementName);
        let (_, (__this, __r)) = AbstractShortcutEntryBase::new_in(move |__base| {
            let this = Self {
                base: __base,
                category: __init_category,
                replacement: todo!("fabric2seer: the field value (TODO above)"),
            };
            let (__h, __r) = __wrap(this);
            ((__h.clone() as compat::Shared<dyn AbstractShortcutEntry>), (__h, __r))
        });
        __this.lock().borrow_mut().shortcut_entry_base_mut().replacement.set_max_length(256);
        // TODO(fabric2seer): `set_value` takes 2 argument(s) in the port (`splitter: &StringSplitter<W>, value: &str`), Java passes 1
        // rust: { let (__q0,) = (&category.lock().borrow().shortcuts_map.get(&target_key).cloned().unwrap_or_else(|| "".to_string()),); __this.lock().borrow_mut().shortcut_entry_base_mut().replacement.set_value(__q0) };
        (__this, __r)
    }

}

/// java: the virtual methods of ShortcutEntry (implemented by ShortcutEntryBase and every subclass)
pub trait ShortcutEntry<T>: AbstractShortcutEntry {
    fn shortcut_entry_base(&self) -> &ShortcutEntryBase<T>;
    fn shortcut_entry_base_mut(&mut self) -> &mut ShortcutEntryBase<T>;

    /// java: protected abstract boolean isNotEmpty();
    fn is_not_empty(&self) -> bool;

    /// java: protected abstract boolean isChanged();
    fn is_changed(&self) -> bool;

    /// java: protected abstract void save();
    fn save(&self);

    /// java: protected abstract String key();
    fn key(&self) -> String;

    /// java: @Override public void extractContent(GuiGraphicsExtractor graphics, int mouseX, int mouseY, boolean hovered, float a)
    fn extract_content(&mut self, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut hovered: bool, mut a: f32) {
        // TODO(fabric2seer): `set_y` takes 2 argument(s) in the port (`y: i32, splitter: &StringSplitter<W>`), Java passes 1
        // rust: { let __s0 = (self.shortcut_entry_base().get_y()).wrapping_add(TEXT_FIELD_PADDING); self.shortcut_entry_base_mut().replacement.set_y(__s0) };
        // TODO(fabric2seer): `extract_render_state` is a method of the port's trait `Renderable`, which the port's `EditBox` does not implement
        // rust: self.shortcut_entry_base_mut().replacement.extract_render_state(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mouse_x, mouse_y, a);
        // TODO(fabric2seer): unknown name `minecraft`
        //   java: graphics.centeredText(minecraft.font, "→", this.getX() + this.getWidth() / 2, this.getY() + TEXT_Y_OFFSET, CommonColors.WHITE);
    }

}

// ShortcutEntry.updatePositions implements AbstractShortcutEntry.updatePositions: it is emitted in each concrete subclass's `impl AbstractShortcutEntry` (Rust has no inherited impls)
/// java: protected class CommandShortcutEntry extends ShortcutEntry<String>
pub struct CommandShortcutEntryBase {
    /// java: extends ShortcutEntry
    pub base: ShortcutEntryBase<String>,
    /// java: private final List<EditBox> children;
    pub children: Vec<EditBox>,
    /// java: private final EditBox target;
    pub target: Option<EditBox>,
}

impl std::ops::Deref for CommandShortcutEntryBase {
    type Target = ShortcutEntryBase<String>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for CommandShortcutEntryBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl CommandShortcutEntryBase {
    /// java: private CommandShortcutEntry(ShortcutCategoryEntry<String> category) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_in<__R>(mut category: compat::Shared<ShortcutCategoryEntry<String>>, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn CommandShortcutEntry>, __R)) -> (compat::Shared<dyn CommandShortcutEntry>, __R) {
        let (__this, __r) = Self::new_2_in(category.clone(), "".to_string(), __wrap);
        (__this, __r)
    }

    /// java: private CommandShortcutEntry(ShortcutCategoryEntry<String> category)
    pub fn new(mut category: compat::Shared<ShortcutCategoryEntry<String>>) -> compat::Shared<Self> {
        Self::new_in(category, |this| {
            let __h = compat::shared(this);
            ((__h.clone() as compat::Shared<dyn CommandShortcutEntry>), __h)
        }).1
    }

    /// java: private CommandShortcutEntry(ShortcutCategoryEntry<String> category, String targetString) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_2_in<__R>(mut category: compat::Shared<ShortcutCategoryEntry<String>>, mut target_string: String, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn CommandShortcutEntry>, __R)) -> (compat::Shared<dyn CommandShortcutEntry>, __R) {
        // TODO(fabric2seer): `new` takes 5 argument(s) in the port (`splitter: &StringSplitter<W>, x: i32, y: i32, width: i32, height: i32`), Java passes 6
        //   java: target = new EditBox(Minecraft.getInstance().font, width / 2 - 160, TEXT_Y_OFFSET, 150, 20, category.targetName);
        let this = Self {
            base: ShortcutEntryBase::<String>::new(category.clone(), target_string.clone()),
            children: Default::default(),
            target: todo!("fabric2seer: the field value (TODO above)"),
        };
        let (__h, __r) = __wrap(this);
        let __this = __h;
        __this.lock().borrow_mut().command_shortcut_entry_base_mut().target.as_mut().unwrap().set_max_length(256);
        // TODO(fabric2seer): `set_value` takes 2 argument(s) in the port (`splitter: &StringSplitter<W>, value: &str`), Java passes 1
        // rust: __this.lock().borrow_mut().command_shortcut_entry_base_mut().target.as_mut().unwrap().set_value(&target_string);
        { let __v = vec![compat::read(&__this, |__t| __t.command_shortcut_entry_base().target.clone()).unwrap(), compat::read(&__this, |__t| __t.command_shortcut_entry_base().replacement.clone())]; compat::write(&__this, |__t| __t.command_shortcut_entry_base_mut().children = __v); }
        (__this, __r)
    }

    /// java: private CommandShortcutEntry(ShortcutCategoryEntry<String> category, String targetString)
    pub fn new_2(mut category: compat::Shared<ShortcutCategoryEntry<String>>, mut target_string: String) -> compat::Shared<Self> {
        Self::new_2_in(category, target_string, |this| {
            let __h = compat::shared(this);
            ((__h.clone() as compat::Shared<dyn CommandShortcutEntry>), __h)
        }).1
    }

}

impl std::fmt::Display for CommandShortcutEntryBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

/// java: the virtual methods of CommandShortcutEntry (implemented by CommandShortcutEntryBase and every subclass)
pub trait CommandShortcutEntry: ShortcutEntry {
    fn command_shortcut_entry_base(&self) -> &CommandShortcutEntryBase;
    fn command_shortcut_entry_base_mut(&mut self) -> &mut CommandShortcutEntryBase;

    /// java: @Override public String toString() (dispatch: the body for this class)
    fn to_string(&self) -> fn(&compat::Shared<dyn CommandShortcutEntry>) -> String {
        <dyn CommandShortcutEntry>::to_string_body
    }

    /// java: @Override public List<? extends GuiEventListener> children() (dispatch: the body for this class)
    fn children(&self) -> fn(&compat::Shared<dyn CommandShortcutEntry>) -> Vec<Box<dyn GuiEventListener>> {
        <dyn CommandShortcutEntry>::children_body
    }

    /// java: @Override public List<? extends NarratableEntry> narratables() (dispatch: the body for this class)
    fn narratables(&self) -> fn(&compat::Shared<dyn CommandShortcutEntry>) -> Vec<Box<dyn NarratableEntry>> {
        <dyn CommandShortcutEntry>::narratables_body
    }

}

impl dyn CommandShortcutEntry {
    /// java: @Override public String toString()
    pub fn to_string_body(__this: &compat::Shared<dyn CommandShortcutEntry>) -> String {
        return format!("{} → {}", __this.lock().borrow().command_shortcut_entry_base().target.as_ref().unwrap().get_value(), __this.lock().borrow().command_shortcut_entry_base().replacement.get_value());
    }

    /// java: @Override public List<? extends GuiEventListener> children()
    pub fn children_body(__this: &compat::Shared<dyn CommandShortcutEntry>) -> Vec<Box<dyn GuiEventListener>> {
        return compat::read(__this, |__t| __t.command_shortcut_entry_base().children.clone());
    }

    /// java: @Override public List<? extends NarratableEntry> narratables()
    pub fn narratables_body(__this: &compat::Shared<dyn CommandShortcutEntry>) -> Vec<Box<dyn NarratableEntry>> {
        return compat::read(__this, |__t| __t.command_shortcut_entry_base().children.clone());
    }

    /// java: @Override protected boolean isNotEmpty()
    pub fn is_not_empty_body(__this: &compat::Shared<dyn CommandShortcutEntry>) -> bool {
        return (!(__this.lock().borrow().command_shortcut_entry_base().target.as_ref().unwrap().get_value().is_empty()) && !(__this.lock().borrow().command_shortcut_entry_base().replacement.get_value().is_empty()));
    }

    /// java: @Override protected boolean isChanged()
    pub fn is_changed_body(__this: &compat::Shared<dyn CommandShortcutEntry>) -> bool {
        return (!(compat::read(__this, |__t| __t.command_shortcut_entry_base().category.clone()).lock().borrow().shortcuts_map.contains_key(&__this.lock().borrow().command_shortcut_entry_base().target.as_ref().unwrap().get_value())) || !(compat::read(__this, |__t| __t.command_shortcut_entry_base().category.clone()).lock().borrow().shortcuts_map.get(&__this.lock().borrow().command_shortcut_entry_base().target.as_ref().unwrap().get_value()).cloned().as_deref() == Some(&*(__this.lock().borrow().command_shortcut_entry_base().replacement.get_value()))));
    }

    /// java: @Override protected void save()
    pub fn save_body(__this: &compat::Shared<dyn CommandShortcutEntry>) {
        let __r0 = compat::read(__this, |__t| __t.command_shortcut_entry_base().category.clone());
        __r0.lock().borrow_mut().shortcuts_map.insert(__this.lock().borrow().command_shortcut_entry_base().target.as_ref().unwrap().get_value(), __this.lock().borrow().command_shortcut_entry_base().replacement.get_value());
    }

    /// java: @Override protected String key()
    pub fn key_body(__this: &compat::Shared<dyn CommandShortcutEntry>) -> String {
        return __this.lock().borrow().command_shortcut_entry_base().target.as_ref().unwrap().get_value();
    }

    /// java: @Override public void extractContent(GuiGraphicsExtractor graphics, int mouseX, int mouseY, boolean hovered, float a)
    pub fn extract_content_body(__this: &compat::Shared<dyn CommandShortcutEntry>, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut hovered: bool, mut a: f32) {
        __this.base.extract_content(graphics, mouse_x, mouse_y, hovered, a);
        // TODO(fabric2seer): `set_y` takes 2 argument(s) in the port (`y: i32, splitter: &StringSplitter<W>`), Java passes 1
        // rust: { let (__q0,) = ((__this.lock().borrow().command_shortcut_entry_base().get_y()).wrapping_add(TEXT_FIELD_PADDING),); __this.lock().borrow_mut().command_shortcut_entry_base_mut().target.as_mut().unwrap().set_y(__q0) };
        // TODO(fabric2seer): `extract_render_state` is a method of the port's trait `Renderable`, which the port's `EditBox` does not implement
        // rust: { let (__q0, __q1, __q2, __q3, __q4,) = (mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mouse_x, mouse_y, a,); __this.lock().borrow_mut().command_shortcut_entry_base_mut().target.as_mut().unwrap().extract_render_state(__q0, __q1, __q2, __q3, __q4) };
    }

    /// java: @Override protected void updatePositions()
    pub fn update_positions_body(__this: &compat::Shared<dyn CommandShortcutEntry>) {
        __this.base.update_positions();
        // TODO(fabric2seer): `set_x` takes 2 argument(s) in the port (`x: i32, splitter: &StringSplitter<W>`), Java passes 1
        // rust: let __r0 = compat::read(__this, |__t| __t.command_shortcut_entry_base().width);
        // rust: { let (__q0,) = (((__r0).wrapping_div(2)).wrapping_sub(160),); __this.lock().borrow_mut().command_shortcut_entry_base_mut().target.as_mut().unwrap().set_x(__q0) };
    }

    /// CommandShortcutEntry.updatePositions as the dispatch of AbstractShortcutEntry.updatePositions for the concrete class `__C`
    pub fn update_positions_from<__C: CommandShortcutEntry + 'static>(__this: &compat::Shared<dyn AbstractShortcutEntry>) {
        <dyn CommandShortcutEntry>::update_positions_body(&compat::view_as::<__C, _, _>(__this, |a| a as compat::Shared<dyn CommandShortcutEntry>))
    }

}

impl CommandShortcutEntry for CommandShortcutEntryBase {
    fn command_shortcut_entry_base(&self) -> &CommandShortcutEntryBase {
        &self
    }
    fn command_shortcut_entry_base_mut(&mut self) -> &mut CommandShortcutEntryBase {
        &mut self
    }

}

impl ShortcutEntry<String> for CommandShortcutEntryBase {
    fn shortcut_entry_base(&self) -> &ShortcutEntryBase<String> {
        &self.base
    }
    fn shortcut_entry_base_mut(&mut self) -> &mut ShortcutEntryBase<String> {
        &mut self.base
    }

    /// java: @Override protected boolean isNotEmpty()
    fn is_not_empty(&self) -> bool {
        return (!(self.target.as_ref().unwrap().get_value().is_empty()) && !(self.replacement.get_value().is_empty()));
    }

    /// java: @Override protected boolean isChanged()
    fn is_changed(&self) -> bool {
        return (!(self.category.lock().borrow().shortcuts_map.contains_key(&self.target.as_ref().unwrap().get_value())) || !(self.category.lock().borrow().shortcuts_map.get(&self.target.as_ref().unwrap().get_value()).cloned().as_deref() == Some(&*(self.replacement.get_value()))));
    }

    /// java: @Override protected void save()
    fn save(&self) {
        let __h0 = self.target.as_ref().unwrap().get_value();
        let __h1 = self.replacement.get_value();
        self.category.lock().borrow_mut().shortcuts_map.insert(__h0, __h1);
    }

    /// java: @Override protected String key()
    fn key(&self) -> String {
        return self.target.as_ref().unwrap().get_value();
    }

    /// java: @Override public void extractContent(GuiGraphicsExtractor graphics, int mouseX, int mouseY, boolean hovered, float a)
    fn extract_content(&mut self, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut hovered: bool, mut a: f32) {
        self.base.extract_content(graphics, mouse_x, mouse_y, hovered, a);
        // TODO(fabric2seer): `set_y` takes 2 argument(s) in the port (`y: i32, splitter: &StringSplitter<W>`), Java passes 1
        // rust: { let __s0 = self.get_y().wrapping_add(TEXT_FIELD_PADDING); self.target.as_mut().unwrap().set_y(__s0) };
        // TODO(fabric2seer): `extract_render_state` is a method of the port's trait `Renderable`, which the port's `EditBox` does not implement
        // rust: self.target.as_mut().unwrap().extract_render_state(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mouse_x, mouse_y, a);
    }

}

impl AbstractShortcutEntry for CommandShortcutEntryBase {
    fn abstract_shortcut_entry_base(&self) -> &AbstractShortcutEntryBase {
        &self.base.base
    }
    fn abstract_shortcut_entry_base_mut(&mut self) -> &mut AbstractShortcutEntryBase {
        &mut self.base.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override protected void updatePositions() (dispatch: the body for this class)
    fn update_positions(&self) -> fn(&compat::Shared<dyn AbstractShortcutEntry>) {
        <dyn CommandShortcutEntry>::update_positions_from::<Self>
    }

}

/// java: protected class CommandArgShortcutEntry extends CommandShortcutEntry
pub struct CommandArgShortcutEntry {
    /// java: extends CommandShortcutEntry
    pub base: CommandShortcutEntryBase,
}

impl std::ops::Deref for CommandArgShortcutEntry {
    type Target = CommandShortcutEntryBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for CommandArgShortcutEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl CommandArgShortcutEntry {
    /// java: private CommandArgShortcutEntry(ShortcutCategoryEntry<String> category)
    pub fn new(mut category: compat::Shared<ShortcutCategoryEntry<String>>) -> compat::Shared<Self> {
        let (_, __this) = CommandShortcutEntryBase::new_in(category.clone(), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
            });
            ((__h.clone() as compat::Shared<dyn CommandShortcutEntry>), __h)
        });
        __this
    }

    /// java: private CommandArgShortcutEntry(ShortcutCategoryEntry<String> category, String targetString)
    pub fn new_2(mut category: compat::Shared<ShortcutCategoryEntry<String>>, mut target_string: String) -> compat::Shared<Self> {
        let (_, __this) = CommandShortcutEntryBase::new_2_in(category.clone(), target_string.clone(), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
            });
            ((__h.clone() as compat::Shared<dyn CommandShortcutEntry>), __h)
        });
        __this
    }

}

impl CommandShortcutEntry for CommandArgShortcutEntry {
    fn command_shortcut_entry_base(&self) -> &CommandShortcutEntryBase {
        &self.base
    }
    fn command_shortcut_entry_base_mut(&mut self) -> &mut CommandShortcutEntryBase {
        &mut self.base
    }

}

impl ShortcutEntry<compat::Object> for CommandArgShortcutEntry {
    fn shortcut_entry_base(&self) -> &ShortcutEntryBase<compat::Object> {
        &self.base.base
    }
    fn shortcut_entry_base_mut(&mut self) -> &mut ShortcutEntryBase<compat::Object> {
        &mut self.base.base
    }

    /// java: @Override protected String key()
    fn key(&self) -> String {
        return format!("arg{}", self.base.key());
    }

    /// java: @Override protected boolean isNotEmpty()
    fn is_not_empty(&self) -> bool {
        return (!(self.target.as_ref().unwrap().get_value().is_empty()) && !(self.replacement.get_value().is_empty()));
    }

    /// java: @Override protected boolean isChanged()
    fn is_changed(&self) -> bool {
        return (!(self.category.lock().borrow().shortcuts_map.contains_key(&self.target.as_ref().unwrap().get_value())) || !(self.category.lock().borrow().shortcuts_map.get(&self.target.as_ref().unwrap().get_value()).cloned().as_deref() == Some(&*(self.replacement.get_value()))));
    }

    /// java: @Override protected void save()
    fn save(&self) {
        let __h0 = self.target.as_ref().unwrap().get_value();
        let __h1 = self.replacement.get_value();
        self.category.lock().borrow_mut().shortcuts_map.insert(__h0, __h1);
    }

}

impl AbstractShortcutEntry for CommandArgShortcutEntry {
    fn abstract_shortcut_entry_base(&self) -> &AbstractShortcutEntryBase {
        &self.base.base.base
    }
    fn abstract_shortcut_entry_base_mut(&mut self) -> &mut AbstractShortcutEntryBase {
        &mut self.base.base.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override protected void updatePositions() (dispatch: the body for this class)
    fn update_positions(&self) -> fn(&compat::Shared<dyn AbstractShortcutEntry>) {
        <dyn CommandShortcutEntry>::update_positions_from::<Self>
    }

}

/// CommandArgShortcutEntry's row box and its widgets, lent by its `compat::EntryAdapter` for each call
impl compat::RowHalf for CommandArgShortcutEntry {
    fn entry_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry {
        &mut self.base.base.base.base
    }
    fn row_parts(&mut self) -> (&mut mcrust::net::minecraft::client::gui::screens::screen::Screen, &mut compat::WidgetCache) {
        let __s = &mut self.base.base.base;
        (&mut __s.__row, &mut __s.__widgets)
    }
}

/// CommandArgShortcutEntry as a row of the game's list (`addEntry` attaches it)
impl ModListEntry for compat::EntryAdapter<CommandArgShortcutEntry> {
    /// java: extractContent (not overridden here)
    fn extract_content(&mut self, list: &mut ModSelectionList, index: usize, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, hovered: bool, a: f32) {
        let _ = (list, index, hovered, a);
        // the row's widgets (Java's `child.extractRenderState(..)` over `children()`)
        context.extract_widgets(&mut self.row, mouse_x, mouse_y);
    }

    fn children(&mut self) -> Option<&mut mcrust::net::minecraft::client::gui::screens::screen::Screen> {
        Some(&mut self.row)
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    fn mouse_clicked(&mut self, list: &mut ModSelectionList, index: usize, event: &MouseButtonEvent, double_click: bool) -> bool {
        self.lend(Some(list.list.entry(index)), |__h| ({ let __f = compat::dispatch(__h, |__t| __t.mouse_clicked()); __f(&compat::view(__h, |a| a as compat::Shared<dyn AbstractShortcutEntry>), event, double_click) }))
    }

}

/// java: protected class KeybindShortcutEntry extends ShortcutEntry<ShortcutKeyBinding>
pub struct KeybindShortcutEntry {
    /// java: extends ShortcutEntry
    pub base: ShortcutEntryBase<compat::Shared<ShortcutKeyBinding>>,
    /// java: private final List<AbstractWidget> children;
    pub children: Vec<AbstractWidget>,
    /// java: private final ShortcutKeyBinding keyBinding;
    pub key_binding: Option<compat::Shared<ShortcutKeyBinding>>,
    /// java: private final KeybindWidget keybindButton;
    pub keybind_button: Option<KeybindWidget>,
    /// java: private boolean conflicting = false;
    pub conflicting: bool,
    /// java: private boolean duplicate = false;
    pub duplicate: bool,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl std::ops::Deref for KeybindShortcutEntry {
    type Target = ShortcutEntryBase<compat::Shared<ShortcutKeyBinding>>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for KeybindShortcutEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl KeybindShortcutEntry {
    /// java: private KeybindShortcutEntry(ShortcutCategoryEntry<ShortcutKeyBinding> category)
    pub fn new(mut category: compat::Shared<ShortcutCategoryEntry<compat::Shared<ShortcutKeyBinding>>>) -> compat::Shared<Self> {
        let __this = Self::new_2(category.clone(), ShortcutKeyBinding::new(vec![mcrust::com::mojang::blaze3d::platform::input_constants::unknown()]));
        __this
    }

    /// java: @SuppressWarnings("JavadocReference") private KeybindShortcutEntry(ShortcutCategoryEntry<ShortcutKeyBinding> category, ShortcutKeyBinding keyBinding)
    pub fn new_2(mut category: compat::Shared<ShortcutCategoryEntry<compat::Shared<ShortcutKeyBinding>>>, mut key_binding: compat::Shared<ShortcutKeyBinding>) -> compat::Shared<Self> {
        let __init_key_binding = Some(key_binding.clone());
        let __this = compat::shared(Self {
            base: ShortcutEntryBase::<compat::Shared<ShortcutKeyBinding>>::new(category.clone(), key_binding.clone()),
            self_ref: std::sync::Weak::new(),
            children: Default::default(),
            key_binding: __init_key_binding,
            keybind_button: None,
            conflicting: false,
            duplicate: false,
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        // TODO(fabric2seer): `.get(..)` on a value of unknown type
        //   java: keybindButton = new KeybindWidget(keyBinding, width / 2 - 160, TEXT_Y_OFFSET, 150, 20, keyBinding.getBoundKeysText(),
        //   java: 					textSupplier -> keyBinding.isUnbound()
        //   java: 							? Component.translatable("narrator.controls.unbound", replacement.getValue())
        //   java: 							: Component.translatable("narrator.controls.bound", replacement.getValue(), textSupplier.get()),
        //   java: 					ShortcutsConfigListWidget.this::updateKeybinds);
        // TODO(fabric2seer): `ShortcutsConfigListWidget.this` (the enclosing instance of an inner class)
        //   java: replacement.setResponder(_ -> ShortcutsConfigListWidget.this.updateKeybinds());
        { let __v = vec![compat::read(&__this, |__t| __t.keybind_button.clone()).unwrap(), compat::read(&__this, |__t| __t.replacement.clone())]; compat::write(&__this, |__t| __t.children = __v); }
        KeybindShortcutEntry::update(&__this);
        __this
    }

    /// java: @Override public String toString()
    pub fn to_string(&self) -> String {
        return format!("{} → {}", ShortcutKeyBinding::get_bound_keys_text(self.key_binding.as_ref().unwrap()).get_string(), self.replacement.get_value());
    }

    /// java: @Override public List<? extends GuiEventListener> children()
    pub fn children(__this: &compat::Shared<Self>) -> Vec<Box<dyn GuiEventListener>> {
        return compat::read(__this, |__t| __t.children.clone());
    }

    /// java: @Override public List<? extends NarratableEntry> narratables()
    pub fn narratables(__this: &compat::Shared<Self>) -> Vec<Box<dyn NarratableEntry>> {
        return compat::read(__this, |__t| __t.children.clone());
    }

    /// java: protected void update()
    pub fn update(__this: &compat::Shared<Self>) {
        let __r0 = compat::read(__this, |__t| __t.key_binding.clone());
        { let (__q0,) = (mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(ShortcutKeyBinding::get_bound_keys_text(&__r0.unwrap())),); __this.lock().borrow_mut().keybind_button.as_mut().unwrap().set_message_text(__q0) };
        { let __v = false; compat::write(__this, |__t| __t.conflicting = __v); }
        { let __v = false; compat::write(__this, |__t| __t.duplicate = __v); }
        let mut conflict_text: MutableComponent = component::empty();
        let mut duplicate_text: MutableComponent = component::empty();
        if !(ShortcutKeyBinding::is_unbound(&compat::read(__this, |__t| __t.key_binding.clone()).unwrap())) {
            // TODO(fabric2seer): unknown name `minecraft`
            //   java: for (KeyMapping otherKeyBinding : minecraft.options.keyMappings) {
            //   java: 					if (keyBinding.getBoundKeysTranslationKey().contains(otherKeyBinding.saveString())) {
            //   java: 						if (conflicting) {
            //   java: 							conflictText.append(", ");
            //   java: 						}
            //   java: 						conflicting = true;
            //   java: 						conflictText.append(Component.translatable(otherKeyBinding.getName()));
            //   java: 					}
            //   java: 				}
            // TODO(fabric2seer): `ShortcutsConfigListWidget.this` (the enclosing instance of an inner class)
            //   java: for (AbstractShortcutEntry shortcut : ShortcutsConfigListWidget.this.children()) {
            //   java: 					if (shortcut instanceof KeybindShortcutEntry keyBindingShortcut && keyBinding != keyBindingShortcut.keyBinding && keyBinding.equals(keyBindingShortcut.keyBinding)) {
            //   java: 						if (duplicate) {
            //   java: 							duplicateText.append(", ");
            //   java: 						}
            //   java: 						duplicate = true;
            //   java: 						// We display the replacement command to help users identify which shortcuts have conflicting keybinds.
            //   java: 						duplicateText.append(keyBindingShortcut.replacement.getValue());
            //   java: 					}
            //   java: 				}
        }
        if compat::read(__this, |__t| __t.duplicate) {
            { let (__q0,) = (mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::literal("[ ").append(compat::component_of(__this.lock().borrow().keybind_button.as_ref().unwrap().get_message_component()).copy().with_style_format(ChatFormatting::White)).append_str(" ]").with_color_text(mcrust::net::minecraft::network::chat::text_color::TextColor::RED)),); __this.lock().borrow_mut().keybind_button.as_mut().unwrap().set_message_text(__q0) };
            { let (__q0,) = (Tooltip::create((component::translatable_args("skyblocker.shortcuts.keyBinding.duplicate", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(duplicate_text.clone())])).get_string()),); __this.lock().borrow_mut().keybind_button.as_mut().unwrap().set_tooltip(__q0) };
        } else if compat::read(__this, |__t| __t.conflicting) {
            { let (__q0,) = (mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::literal("[ ").append(compat::component_of(__this.lock().borrow().keybind_button.as_ref().unwrap().get_message_component()).copy().with_style_format(ChatFormatting::White)).append_str(" ]").with_color_text(mcrust::net::minecraft::network::chat::text_color::TextColor::YELLOW)),); __this.lock().borrow_mut().keybind_button.as_mut().unwrap().set_message_text(__q0) };
            { let (__q0,) = (Tooltip::create((component::translatable_args("controls.keybinds.duplicateKeybinds", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(conflict_text.clone())])).get_string()),); __this.lock().borrow_mut().keybind_button.as_mut().unwrap().set_tooltip(__q0) };
        } else {
            __this.lock().borrow_mut().keybind_button.as_mut().unwrap().set_tooltip(None);
        }
        if __this.lock().borrow().keybind_button.as_ref().unwrap().is_editing() {
            { let (__q0,) = (mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from((component::literal("> ")).append(compat::component_of(__this.lock().borrow().keybind_button.as_ref().unwrap().get_message_component()).copy().with_style_formats(&[ChatFormatting::White, ChatFormatting::Underline])).append_str(" <").with_style_format(ChatFormatting::Yellow)),); __this.lock().borrow_mut().keybind_button.as_mut().unwrap().set_message_text(__q0) };
        }
    }

    /// java: protected boolean stopEditing()
    pub fn stop_editing(__this: &compat::Shared<Self>) -> bool {
        return __this.lock().borrow_mut().keybind_button.as_mut().unwrap().stop_editing();
    }

}

impl std::fmt::Display for KeybindShortcutEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", KeybindShortcutEntry::to_string(self))
    }
}

impl ShortcutEntry<compat::Shared<crate::de::hysky::skyblocker::skyblock::shortcut::shortcut_key_binding::ShortcutKeyBinding>> for KeybindShortcutEntry {
    fn shortcut_entry_base(&self) -> &ShortcutEntryBase<compat::Shared<crate::de::hysky::skyblocker::skyblock::shortcut::shortcut_key_binding::ShortcutKeyBinding>> {
        &self.base
    }
    fn shortcut_entry_base_mut(&mut self) -> &mut ShortcutEntryBase<compat::Shared<crate::de::hysky::skyblocker::skyblock::shortcut::shortcut_key_binding::ShortcutKeyBinding>> {
        &mut self.base
    }

    /// java: @Override protected boolean isNotEmpty()
    fn is_not_empty(&self) -> bool {
        return (!(ShortcutKeyBinding::is_unbound(self.key_binding.as_ref().unwrap())) && !(self.replacement.get_value().is_empty()));
    }

    /// java: @Override protected boolean isChanged()
    fn is_changed(&self) -> bool {
        return (!(self.category.lock().borrow().shortcuts_map.contains_key(self.key_binding.as_ref().unwrap())) || !(self.category.lock().borrow().shortcuts_map.get(self.key_binding.as_ref().unwrap()).cloned().as_deref() == Some(&*(self.replacement.get_value()))));
    }

    /// java: @Override protected void save()
    fn save(&self) {
        let __h0 = self.key_binding.clone().unwrap();
        let __h1 = self.replacement.get_value();
        self.category.lock().borrow_mut().shortcuts_map.insert(__h0, __h1);
    }

    /// java: @Override protected String key()
    fn key(&self) -> String {
        return format!("{:?}", ShortcutKeyBinding::get_bound_keys_translation_key(self.key_binding.as_ref().unwrap()));
    }

    /// java: @Override public void extractContent(GuiGraphicsExtractor graphics, int mouseX, int mouseY, boolean hovered, float a)
    fn extract_content(&mut self, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut hovered: bool, mut a: f32) {
        self.base.extract_content(graphics, mouse_x, mouse_y, hovered, a);
        { let __s0 = self.get_y().wrapping_add(TEXT_FIELD_PADDING); self.keybind_button.as_mut().unwrap().set_y(__s0) };
        // TODO(fabric2seer): `extract_render_state` is a method of the port's trait `Renderable`, which the port's `AbstractWidget` does not implement
        // rust: self.keybind_button.as_mut().unwrap().extract_render_state(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mouse_x, mouse_y, a);
        if self.conflicting || self.duplicate {
            let mut color: i32 = (if self.duplicate { mcrust::net::minecraft::util::common_colors::RED } else { mcrust::net::minecraft::util::common_colors::YELLOW });
            graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), (self.keybind_button.as_ref().unwrap().get_x()).wrapping_sub(6), self.get_y(), (self.keybind_button.as_ref().unwrap().get_x()).wrapping_sub(3), self.get_y().wrapping_add(self.get_height()), color);
        }
    }

}

impl AbstractShortcutEntry for KeybindShortcutEntry {
    fn abstract_shortcut_entry_base(&self) -> &AbstractShortcutEntryBase {
        &self.base.base
    }
    fn abstract_shortcut_entry_base_mut(&mut self) -> &mut AbstractShortcutEntryBase {
        &mut self.base.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override protected void updatePositions() (dispatch: the body for this class)
    fn update_positions(&self) -> fn(&compat::Shared<dyn AbstractShortcutEntry>) {
        KeybindShortcutEntry::update_positions_dyn
    }

}

impl KeybindShortcutEntry {
    /// java: @Override protected void updatePositions()
    pub fn update_positions_impl(__this: &compat::Shared<Self>) {
        __this.base.update_positions();
        let __r0 = compat::read(__this, |__t| __t.width);
        { let (__q0,) = (((__r0).wrapping_div(2)).wrapping_sub(160),); __this.lock().borrow_mut().keybind_button.as_mut().unwrap().set_x(__q0) };
    }

    /// KeybindShortcutEntry.updatePositions as the dispatch of AbstractShortcutEntry.updatePositions
    pub fn update_positions_dyn(__this: &compat::Shared<dyn AbstractShortcutEntry>) {
        Self::update_positions_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}

/// KeybindShortcutEntry's row box and its widgets, lent by its `compat::EntryAdapter` for each call
impl compat::RowHalf for KeybindShortcutEntry {
    fn entry_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry {
        &mut self.base.base.base
    }
    fn row_parts(&mut self) -> (&mut mcrust::net::minecraft::client::gui::screens::screen::Screen, &mut compat::WidgetCache) {
        let __s = &mut self.base.base;
        (&mut __s.__row, &mut __s.__widgets)
    }
}

/// KeybindShortcutEntry as a row of the game's list (`addEntry` attaches it)
impl ModListEntry for compat::EntryAdapter<KeybindShortcutEntry> {
    /// java: extractContent (not overridden here)
    fn extract_content(&mut self, list: &mut ModSelectionList, index: usize, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, hovered: bool, a: f32) {
        let _ = (list, index, hovered, a);
        // the row's widgets (Java's `child.extractRenderState(..)` over `children()`)
        context.extract_widgets(&mut self.row, mouse_x, mouse_y);
    }

    fn children(&mut self) -> Option<&mut mcrust::net::minecraft::client::gui::screens::screen::Screen> {
        Some(&mut self.row)
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    fn mouse_clicked(&mut self, list: &mut ModSelectionList, index: usize, event: &MouseButtonEvent, double_click: bool) -> bool {
        self.lend(Some(list.list.entry(index)), |__h| ({ let __f = compat::dispatch(__h, |__t| __t.mouse_clicked()); __f(&compat::view(__h, |a| a as compat::Shared<dyn AbstractShortcutEntry>), event, double_click) }))
    }

}

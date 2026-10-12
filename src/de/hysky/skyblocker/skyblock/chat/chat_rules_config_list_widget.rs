//! Ported from de.hysky.skyblocker.skyblock.chat.ChatRulesConfigListWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::chat::chat_rule::ChatRule;
use crate::de::hysky::skyblocker::skyblock::chat::chat_rules_config_screen::ChatRulesConfigScreen;
use crate::de::hysky::skyblocker::skyblock::chat::chat_rules_handler;
use crate::de::hysky::skyblocker::skyblock::chat::chat_rules_handler::ChatRulesHandler;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler::Scheduler;
use mcrust::mcrust::modding::widgets::ModList;
use mcrust::mcrust::modding::widgets::ModListEntry;
use mcrust::mcrust::modding::widgets::ModSelectionList;
use mcrust::mcrust::modding::widgets::WidgetRenderContext;
use mcrust::net::minecraft::client::gui::components::abstract_selection_list::AbstractSelectionList;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::checkbox::Checkbox;
use mcrust::net::minecraft::client::gui::components::events::gui_event_listener::GuiEventListener;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::layouts::layout_element::LayoutElement;
use mcrust::net::minecraft::client::gui::layouts::linear_layout::LinearLayout;
use mcrust::net::minecraft::client::gui::narration::narratable_entry::NarratableEntry;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::network::chat::component;
use crate::de::hysky::skyblocker::utils::data::json_data::JsonData as _;
use mcrust::net::minecraft::client::gui::components::renderable::Renderable as _;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final int ROW_HEIGHT = 26;
pub const ROW_HEIGHT: i32 = 26;

/// java: public class ChatRulesConfigListWidget extends ContainerObjectSelectionList<ChatRulesConfigListWidget.AbstractChatRuleEntry>
pub struct ChatRulesConfigListWidget {
    /// java: extends net.minecraft.client.gui.components.ContainerObjectSelectionList
    pub base: ModSelectionList,
    /// java: private final ChatRulesConfigScreen screen;
    pub screen: compat::Shared<ChatRulesConfigScreen>,
    /// java: private boolean hasChanged;
    pub has_changed: bool,
}

impl std::ops::Deref for ChatRulesConfigListWidget {
    type Target = AbstractSelectionList;
    fn deref(&self) -> &Self::Target {
        &self.base.list
    }
}

impl std::ops::DerefMut for ChatRulesConfigListWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base.list
    }
}

impl ChatRulesConfigListWidget {
    /// java: public ChatRulesConfigListWidget(Minecraft client, ChatRulesConfigScreen screen, int width, int height, int y)
    pub fn new(client: &mut mcrust::net::minecraft::client::minecraft::Minecraft, mut screen: compat::Shared<ChatRulesConfigScreen>, mut width: i32, mut height: i32, mut y: i32) -> compat::Shared<Self> {
        let __init_screen = screen.clone();
        let __init_has_changed = false;
        let __this = compat::shared(Self {
            base: mcrust::mcrust::modding::widgets::ModSelectionList::new(mcrust::mcrust::modding::widgets::ListKind::Container, width, height, y, ROW_HEIGHT),
            screen: __init_screen,
            has_changed: __init_has_changed,
        });
        ChatRulesConfigListWidget::update_entries(&__this);
        __this
    }

    /// java: public void updateEntries()
    pub fn update_entries(__this: &compat::Shared<Self>) {
        compat::write(__this, |__t| { let __l = &mut __t.base; __l.clear_entries() });
        let __entry = (LabelsEntry::new() as compat::Shared<dyn AbstractChatRuleEntry>);
        compat::write(__this, |__t| { let __l = &mut __t.base; (__l.add_entry(Box::new(compat::EntryAdapter::new(__entry))) as i32) });
        // TODO(fabric2seer): `.size(..)` on a T
        //   java: for (int i = 0; i < ChatRulesHandler.CHAT_RULE_LIST.getData().size(); i++) {
        //   java: 			addEntry(new ChatRuleEntry(i));
        //   java: 		}
    }

    /// java: @Override public int getRowWidth()
    pub fn get_row_width(__this: &compat::Shared<Self>) -> i32 {
        return 320;
    }

    /// java: @Override protected boolean entriesCanBeSelected()
    pub fn entries_can_be_selected(__this: &compat::Shared<Self>) -> bool {
        return true;
    }

    /// java: protected void addRuleAfterSelected()
    pub fn add_rule_after_selected(__this: &compat::Shared<Self>) {
        { let __v = true; compat::write(__this, |__t| __t.has_changed = __v); }
        let mut new_index: i32 = ((compat::write(__this, |__t| { let __l = &mut __t.base; compat::list_entries::<dyn AbstractChatRuleEntry>(__l) })).iter().position(|e| *e == compat::write(__this, |__t| { let __l = &mut __t.base; __l.get_selected().and_then(|__i| compat::entry_at::<dyn AbstractChatRuleEntry>(__l, __i)) }).unwrap()).map_or(-1, |i| i as i32)).max(0);
        // TODO(fabric2seer): `.add(..)` on a T
        //   java: ChatRulesHandler.CHAT_RULE_LIST.getData().add(newIndex, new ChatRule());
        ChatRulesConfigListWidget::update_entries(__this);
        if new_index.wrapping_add(1) >= ((compat::write(__this, |__t| { let __l = &mut __t.base; compat::list_entries::<dyn AbstractChatRuleEntry>(__l) })).len() as i32) {
            return;
        }
        let mut entry: compat::Shared<dyn AbstractChatRuleEntry> = (compat::write(__this, |__t| { let __l = &mut __t.base; compat::list_entries::<dyn AbstractChatRuleEntry>(__l) }))[(new_index.wrapping_add(1)) as usize].clone();
        let __this_1 = compat::RenderThread::new(__this.clone());
        ({ let __r = scheduler::INSTANCE.get_mut(); let __f = compat::dispatch(__r, |__t| __t.schedule()); __f(__r, Arc::new(move || { let __entry = Some(entry.clone()); compat::write(__this_1.get(), |__t| { let __l = &mut __t.base; { let __i = __entry.as_ref().and_then(|__e| compat::entry_index::<dyn AbstractChatRuleEntry>(__l, __e)); __l.set_selected(__i); } }); }), 0) });
    }

    /// java: @Override protected void removeEntry(AbstractChatRuleEntry entry)
    pub fn remove_entry(__this: &compat::Shared<Self>, entry: &compat::Shared<dyn AbstractChatRuleEntry>) {
        let __entry = entry.clone();
        compat::write(__this, |__t| { let __l = &mut __t.base; { if let Some(__i) = compat::entry_index::<dyn AbstractChatRuleEntry>(__l, &__entry) { __l.remove_entry(__i); } } });
        { let __v = true; compat::write(__this, |__t| __t.has_changed = __v); }
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    pub fn mouse_clicked(__this: &compat::Shared<Self>, click: &MouseButtonEvent, mut doubled: bool) -> bool {
        // TODO(fabric2seer): `AbstractContainerWidget::mouse_clicked` ports `mouseClicked`, but the port's `AbstractSelectionList` holds no `AbstractContainerWidget` (no Deref, no `base` field) to call it on
        if { let (__q0, __q1,) = (click.clone(), doubled,); __this.lock().borrow_mut().base.list.mouse_clicked(__q0, __q1) } {
            return true;
        }
        let __entry = None;
        compat::write(__this, |__t| { let __l = &mut __t.base; { let __i = __entry.as_ref().and_then(|__e| compat::entry_index::<dyn AbstractChatRuleEntry>(__l, __e)); __l.set_selected(__i); } });
        return false;
    }

    /// java: protected void saveRules()
    pub fn save_rules(__this: &compat::Shared<Self>) {
        { let __v = false; compat::write(__this, |__t| __t.has_changed = __v); }
        ChatRulesHandler::save_chat_rules();
    }

    /// java: protected boolean hasChanges()
    pub fn has_changes(__this: &compat::Shared<Self>) -> bool {
        return (compat::read(__this, |__t| __t.has_changed) || compat::write(__this, |__t| { let __l = &mut __t.base; compat::list_entries::<dyn AbstractChatRuleEntry>(__l) }).into_iter().any(|__a0| ({ let __f = compat::dispatch(&__a0, |__t| __t.has_changed()); __f(&__a0) })));
    }

}

/// ChatRulesConfigListWidget's list (the port's `ModSelectionList`) until the game takes it (`compat::attach_list`)
impl compat::ListHalf for ChatRulesConfigListWidget {
    fn list_half(&mut self) -> &mut ModSelectionList {
        &mut self.base
    }
}

/// ChatRulesConfigListWidget's overrides of the list's own methods, asked by the port's list bodies
impl ModList for compat::ListClass<ChatRulesConfigListWidget> {
    /// java: @Override public int getRowWidth()
    fn get_row_width(&self, list: &AbstractSelectionList, super_answer: i32) -> i32 {
        self.ask(list, super_answer, |__h| ChatRulesConfigListWidget::get_row_width(__h))
    }

    /// java: @Override protected boolean entriesCanBeSelected()
    fn entries_can_be_selected(&self, list: &AbstractSelectionList, super_answer: bool) -> bool {
        self.ask(list, super_answer, |__h| ChatRulesConfigListWidget::entries_can_be_selected(__h))
    }

}

// NOTE(fabric2seer): entriesCanBeSelected, getRowWidth, mouseClicked, removeEntry override(s) of ContainerObjectSelectionList - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them

/// java: protected abstract static class AbstractChatRuleEntry extends ContainerObjectSelectionList.Entry<ChatRulesConfigListWidget.AbstractChatRuleEntry>
pub struct AbstractChatRuleEntryBase {
    /// java: extends net.minecraft.client.gui.components.ContainerObjectSelectionList$Entry
    pub base: mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry,
    /// the row's widgets (`children()`): lent to the object by its `compat::EntryAdapter`
    pub __row: mcrust::net::minecraft::client::gui::screens::screen::Screen,
    /// the screen's widgets as this object last saw them (see `compat::WidgetCache`)
    pub __widgets: compat::WidgetCache,
}

impl std::ops::Deref for AbstractChatRuleEntryBase {
    type Target = mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for AbstractChatRuleEntryBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl AbstractChatRuleEntryBase {
    /// java: the implicit no-argument constructor (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_in<__R>(__wrap: impl FnOnce(Self) -> (compat::Shared<dyn AbstractChatRuleEntry>, __R)) -> (compat::Shared<dyn AbstractChatRuleEntry>, __R) {
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

/// java: the virtual methods of AbstractChatRuleEntry (implemented by AbstractChatRuleEntryBase and every subclass)
pub trait AbstractChatRuleEntry: compat::DynAny {
    fn abstract_chat_rule_entry_base(&self) -> &AbstractChatRuleEntryBase;
    fn abstract_chat_rule_entry_base_mut(&mut self) -> &mut AbstractChatRuleEntryBase;
    fn as_any(&self) -> &dyn std::any::Any;

    /// java: public boolean hasChanged() (dispatch: the body for this class)
    fn has_changed(&self) -> fn(&compat::Shared<dyn AbstractChatRuleEntry>) -> bool {
        <dyn AbstractChatRuleEntry>::has_changed_body
    }

}

impl dyn AbstractChatRuleEntry {
    /// java: public boolean hasChanged()
    pub fn has_changed_body(__this: &compat::Shared<dyn AbstractChatRuleEntry>) -> bool {
        return false;
    }

}

/// AbstractChatRuleEntry's row box and its widgets, lent by its `compat::EntryAdapter` for each call
impl compat::RowHalf for dyn AbstractChatRuleEntry {
    fn entry_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry {
        &mut self.abstract_chat_rule_entry_base_mut().base
    }
    fn row_parts(&mut self) -> (&mut mcrust::net::minecraft::client::gui::screens::screen::Screen, &mut compat::WidgetCache) {
        let __s = &mut self.abstract_chat_rule_entry_base_mut();
        (&mut __s.__row, &mut __s.__widgets)
    }
}

/// AbstractChatRuleEntry as a row of the game's list (`addEntry` attaches it)
impl ModListEntry for compat::EntryAdapter<dyn AbstractChatRuleEntry> {
    /// java: @Override public void extractContent(GuiGraphicsExtractor graphics, int mouseX, int mouseY, boolean hovered, float a)
    fn extract_content(&mut self, list: &mut ModSelectionList, index: usize, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, hovered: bool, a: f32) {
        self.lend(Some(list.list.entry(index)), |__h| { if let Some(__c) = compat::try_view::<ChatRuleEntry, _>(__h) { return ChatRuleEntry::extract_content(&*__c, &context.graphics, mouse_x, mouse_y, hovered, a); } if let Some(__c) = compat::try_view::<LabelsEntry, _>(__h) { return LabelsEntry::extract_content(&*__c, &mut context.graphics, mouse_x, mouse_y, hovered, a); }  });
        // the row's widgets (Java's `child.extractRenderState(..)` over `children()`)
        context.extract_widgets(&mut self.row, mouse_x, mouse_y);
    }

    fn children(&mut self) -> Option<&mut mcrust::net::minecraft::client::gui::screens::screen::Screen> {
        Some(&mut self.row)
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    fn mouse_clicked(&mut self, list: &mut ModSelectionList, index: usize, event: &MouseButtonEvent, double_click: bool) -> bool {
        self.lend(Some(list.list.entry(index)), |__h| { if let Some(__c) = compat::try_view::<ChatRuleEntry, _>(__h) { return ChatRuleEntry::mouse_clicked(&*__c, event, double_click); } false })
    }

}

/// java: private class LabelsEntry extends AbstractChatRuleEntry
pub struct LabelsEntry {
    /// java: extends AbstractChatRuleEntry
    pub base: AbstractChatRuleEntryBase,
}

impl std::ops::Deref for LabelsEntry {
    type Target = AbstractChatRuleEntryBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for LabelsEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl LabelsEntry {
    /// java: the implicit no-argument constructor
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = AbstractChatRuleEntryBase::new_in(move |__base| {
            let __h = compat::shared(Self {
                base: __base,
            });
            ((__h.clone() as compat::Shared<dyn AbstractChatRuleEntry>), __h)
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

    /// java: @Override public void extractContent(GuiGraphicsExtractor graphics, int mouseX, int mouseY, boolean hovered, float a)
    pub fn extract_content(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut hovered: bool, mut a: f32) {
        // TODO(fabric2seer): unknown name `minecraft`
        //   java: graphics.centeredText(minecraft.font, Component.translatable("skyblocker.config.chat.chatRules.screen.ruleName"), ChatRulesConfigListWidget.this.getWidth() / 2 - 100, this.getY() + 5, CommonColors.WHITE);
        // TODO(fabric2seer): unknown name `minecraft`
        //   java: graphics.centeredText(minecraft.font, Component.translatable("skyblocker.config.chat.chatRules.screen.ruleEnabled"), ChatRulesConfigListWidget.this.getWidth() / 2 - 10, this.getY() + 5, CommonColors.WHITE);
        // TODO(fabric2seer): unknown name `minecraft`
        //   java: graphics.centeredText(minecraft.font, Component.translatable("skyblocker.config.chat.chatRules.screen.modify"), ChatRulesConfigListWidget.this.getWidth() / 2 + 77, this.getY() + 5, CommonColors.WHITE);
        // TODO(fabric2seer): `Entry.getRowLeft` has no Rust method in the port (looked for `get_row_left` in ContainerObjectSelectionList and JavaSrc)
        // TODO(fabric2seer): `Entry.getRowRight` has no Rust method in the port (looked for `get_row_right` in ContainerObjectSelectionList and JavaSrc)
        // rust: graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), __this.lock().borrow().get_row_left(), (__this.lock().borrow().get_y()).wrapping_add(15), __this.lock().borrow().get_row_right(), (__this.lock().borrow().get_y()).wrapping_add(16), mcrust::net::minecraft::util::common_colors::LIGHT_GRAY);
    }

}

impl AbstractChatRuleEntry for LabelsEntry {
    fn abstract_chat_rule_entry_base(&self) -> &AbstractChatRuleEntryBase {
        &self.base
    }
    fn abstract_chat_rule_entry_base_mut(&mut self) -> &mut AbstractChatRuleEntryBase {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}

/// LabelsEntry's row box and its widgets, lent by its `compat::EntryAdapter` for each call
impl compat::RowHalf for LabelsEntry {
    fn entry_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry {
        &mut self.base.base
    }
    fn row_parts(&mut self) -> (&mut mcrust::net::minecraft::client::gui::screens::screen::Screen, &mut compat::WidgetCache) {
        let __s = &mut self.base;
        (&mut __s.__row, &mut __s.__widgets)
    }
}

/// LabelsEntry as a row of the game's list (`addEntry` attaches it)
impl ModListEntry for compat::EntryAdapter<LabelsEntry> {
    /// java: @Override public void extractContent(GuiGraphicsExtractor graphics, int mouseX, int mouseY, boolean hovered, float a)
    fn extract_content(&mut self, list: &mut ModSelectionList, index: usize, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, hovered: bool, a: f32) {
        self.lend(Some(list.list.entry(index)), |__h| LabelsEntry::extract_content(__h, &mut context.graphics, mouse_x, mouse_y, hovered, a));
        // the row's widgets (Java's `child.extractRenderState(..)` over `children()`)
        context.extract_widgets(&mut self.row, mouse_x, mouse_y);
    }

    fn children(&mut self) -> Option<&mut mcrust::net::minecraft::client::gui::screens::screen::Screen> {
        Some(&mut self.row)
    }

}

/// java: private class ChatRuleEntry extends AbstractChatRuleEntry
pub struct ChatRuleEntry {
    /// java: extends AbstractChatRuleEntry
    pub base: AbstractChatRuleEntryBase,
    /// java: private final int chatRuleIndex;
    pub chat_rule_index: i32,
    /// java: private final ChatRule chatRule;
    pub chat_rule: compat::Shared<ChatRule>,
    /// java: private final StringWidget nameWidget;
    pub name_widget: Option<compat::WidgetRef>,
    /// java: private final List<? extends GuiEventListener> children;
    pub children: Vec<Box<dyn GuiEventListener>>,
    /// java: private final LinearLayout layout;
    pub layout: Option<LinearLayout>,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl std::ops::Deref for ChatRuleEntry {
    type Target = AbstractChatRuleEntryBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for ChatRuleEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl ChatRuleEntry {
    /// java: private ChatRuleEntry(int chatRuleIndex)
    pub fn new(mut chat_rule_index: i32) -> compat::Shared<Self> {
        let __init_chat_rule_index = chat_rule_index;
        // TODO(fabric2seer): `.get(..)` on a T
        //   java: this.chatRule = ChatRulesHandler.CHAT_RULE_LIST.getData().get(chatRuleIndex);
        let (_, __this) = AbstractChatRuleEntryBase::new_in(move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                self_ref: std::sync::Weak::new(),
                chat_rule_index: __init_chat_rule_index,
                chat_rule: todo!("fabric2seer: the field value (TODO above)"),
                name_widget: None,
                children: Default::default(),
                layout: None,
            });
            compat::write(&__h, |__t| __t.self_ref = std::sync::Arc::downgrade(&__h));
            ((__h.clone() as compat::Shared<dyn AbstractChatRuleEntry>), __h)
        });
        // TODO(fabric2seer): unknown name `minecraft`
        //   java: nameWidget = new StringWidget(Component.literal(chatRule.getName()), minecraft.font);
        // TODO(fabric2seer): field `SCROLLING` of a value of unknown type
        //   java: nameWidget.setMaxWidth(110, StringWidget.TextOverflow.SCROLLING);
        // TODO(fabric2seer): field `HORIZONTAL` of a value of unknown type
        //   java: layout = new LinearLayout(0, 0, LinearLayout.Orientation.HORIZONTAL);
        __this.lock().borrow_mut().layout.as_mut().unwrap().default_cell_setting().padding_right(10).padding_top(3);
        // TODO(fabric2seer): unknown name `minecraft`
        //   java: Checkbox enabledCheck = layout.addChild(Checkbox.builder(Component.empty(), minecraft.font)
        //   java: 					.selected(chatRule.getEnabled())
        //   java: 					.onValueChange((_, value) -> {
        //   java: 						hasChanged = true;
        //   java: 						chatRule.setEnabled(value);
        //   java: 					})
        //   java: 					.build(), s -> s.padding(16, 4, 23, 0));
        let mut enabled_check: Checkbox = todo!("fabric2seer: the initialiser of `enabledCheck` (TODO above)");
        // TODO(fabric2seer): unknown name `minecraft`
        //   java: Button openConfigButton = layout.addChild(Button.builder(Component.translatable("skyblocker.config.chat.chatRules.screen.editRule"), _ -> minecraft.gui.setScreen(new ChatRuleConfigScreen(screen, chatRuleIndex))).size(50, 20).tooltip(Tooltip.create(Component.translatable("skyblocker.config.chat.chatRules.screen.editRule.@Tooltip"))).build());
        let mut open_config_button: AbstractWidget = todo!("fabric2seer: the initialiser of `openConfigButton` (TODO above)");
        // TODO(fabric2seer): unknown name `minecraft`
        //   java: Button deleteButton = layout.addChild(Button.builder(Component.translatable("selectServer.delete"), _ ->
        //   java: 				minecraft.gui.setScreen(new ConfirmScreen(this::deleteEntry, Component.translatable("skyblocker.config.chat.chatRules.screen.deleteQuestion"), Component.translatable("skyblocker.config.chat.chatRules.screen.deleteWarning", chatRule.getName()), Component.translatable("selectServer.deleteButton"), CommonComponents.GUI_CANCEL))
        //   java: 			).size(50, 20).build());
        let mut delete_button: AbstractWidget = todo!("fabric2seer: the initialiser of `deleteButton` (TODO above)");
        { let __v = vec![enabled_check.clone(), open_config_button.clone(), delete_button.clone()]; compat::write(&__this, |__t| __t.children = __v); }
        __this
    }

    /// java: @Override public void setX(int x)
    pub fn set_x(__this: &compat::Shared<Self>, mut x: i32) {
        __this.lock().borrow_mut().base.set_x(x);
        let __wa1 = x.wrapping_add(10);
        compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.name_widget.as_mut().unwrap(); __s.base.__widgets.with(&mut __s.base.__row, __r, |__w| __w.set_x(__wa1)) });
        { let (__q0,) = (x.wrapping_add(125),); __this.lock().borrow_mut().layout.as_mut().unwrap().set_x(__q0) };
        { __this.lock().borrow_mut().layout.as_mut().unwrap().arrange_elements(); let mut __vw = Vec::new(); LayoutElement::Grid(__this.lock().borrow().layout.as_ref().unwrap().wrapped.clone()).visit_widgets(&mut __vw); for (__i, __x, __y) in __vw { compat::write(__this, |__t| __t.base.__row.widget_mut(__i).set_position(__x, __y)); } };
    }

    /// java: @Override public void setY(int y)
    pub fn set_y(__this: &compat::Shared<Self>, mut y: i32) {
        __this.lock().borrow_mut().base.set_y(y);
        let __wa1 = y.wrapping_add(8);
        compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.name_widget.as_mut().unwrap(); __s.base.__widgets.with(&mut __s.base.__row, __r, |__w| __w.set_y(__wa1)) });
        __this.lock().borrow_mut().layout.as_mut().unwrap().set_y(y);
        { __this.lock().borrow_mut().layout.as_mut().unwrap().arrange_elements(); let mut __vw = Vec::new(); LayoutElement::Grid(__this.lock().borrow().layout.as_ref().unwrap().wrapped.clone()).visit_widgets(&mut __vw); for (__i, __x, __y) in __vw { compat::write(__this, |__t| __t.base.__row.widget_mut(__i).set_position(__x, __y)); } };
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    pub fn mouse_clicked(__this: &compat::Shared<Self>, click: &MouseButtonEvent, mut doubled: bool) -> bool {
        // TODO(fabric2seer): `mouse_clicked` takes 4 argument(s) in the port (`_button: i32, _x: f64, _y: f64, _double_click: bool`), Java passes 2
        // TODO(fabric2seer): `mouse_clicked` is a method of the port's trait `GuiEventListener`, which the port's `Entry` does not implement
        // rust: { let (__q0, __q1,) = (click.clone(), doubled,); __this.lock().borrow_mut().base.mouse_clicked(__q0, __q1) };
        // TODO(fabric2seer): `Entry.setSelected` has no Rust method in the port (looked for `set_selected` in ContainerObjectSelectionList and JavaSrc)
        // rust: let __h0 = __this;
        // rust: __this.lock().borrow().set_selected(__h0);
        return true;
    }

    /// java: private void deleteEntry(boolean confirmedAction)
    pub fn delete_entry(__this: &compat::Shared<Self>, mut confirmed_action: bool) {
        if confirmed_action {
            // TODO(fabric2seer): `.remove(..)` on a T
            //   java: ChatRulesHandler.CHAT_RULE_LIST.getData().remove(chatRuleIndex);
            // TODO(fabric2seer): `removeEntry(..)` is a method of the enclosing instance (ChatRulesConfigListWidget.this) - an inner class has no outer reference in Rust
            //   java: removeEntry(this);
        }
        // TODO(fabric2seer): unknown name `minecraft`
        //   java: minecraft.gui.setScreen(screen);
        // TODO(fabric2seer): `updateEntries(..)` is a method of the enclosing instance (ChatRulesConfigListWidget.this) - an inner class has no outer reference in Rust
        //   java: updateEntries();
    }

    /// java: @Override public void extractContent(GuiGraphicsExtractor graphics, int mouseX, int mouseY, boolean hovered, float a)
    pub fn extract_content(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut hovered: bool, mut a: f32) {
        // TODO(fabric2seer): `extract_render_state` is a method of the port's trait `Renderable`, which the port's `AbstractWidget` does not implement
        // rust: let __wa1 = graphics.clone();
        // rust: let __wa2 = mouse_x;
        // rust: let __wa3 = mouse_y;
        // rust: let __wa4 = a;
        // rust: compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.name_widget.as_mut().unwrap(); __s.base.__widgets.with(&mut __s.base.__row, __r, |__w| __w.extract_render_state(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), __wa2, __wa3, __wa4)) });
        { let mut __vw = Vec::new(); LayoutElement::Grid(__this.lock().borrow().layout.as_ref().unwrap().wrapped.clone()).visit_widgets(&mut __vw); for (__i, _, _) in __vw { compat::write(__this, |__t| __t.base.__row.mark_renderable(__i)); } };
    }

    /// java: @Override public List<? extends GuiEventListener> children()
    pub fn children(__this: &compat::Shared<Self>) -> Vec<Box<dyn GuiEventListener>> {
        return compat::read(__this, |__t| __t.children.clone());
    }

    /// java: @Override public List<? extends NarratableEntry> narratables()
    pub fn narratables(__this: &compat::Shared<Self>) -> Vec<Box<dyn NarratableEntry>> {
        return vec![];
    }

}

impl AbstractChatRuleEntry for ChatRuleEntry {
    fn abstract_chat_rule_entry_base(&self) -> &AbstractChatRuleEntryBase {
        &self.base
    }
    fn abstract_chat_rule_entry_base_mut(&mut self) -> &mut AbstractChatRuleEntryBase {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public boolean hasChanged() (dispatch: the body for this class)
    fn has_changed(&self) -> fn(&compat::Shared<dyn AbstractChatRuleEntry>) -> bool {
        ChatRuleEntry::has_changed_dyn
    }

}

impl ChatRuleEntry {
    /// java: @Override public boolean hasChanged()
    pub fn has_changed_impl(__this: &compat::Shared<Self>) -> bool {
        // TODO(fabric2seer): `.get(..)` on a T
        //   java: return chatRule.getEnabled() != ChatRulesHandler.CHAT_RULE_LIST.getData().get(chatRuleIndex).getEnabled();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// ChatRuleEntry.hasChanged as the dispatch of AbstractChatRuleEntry.hasChanged
    pub fn has_changed_dyn(__this: &compat::Shared<dyn AbstractChatRuleEntry>) -> bool {
        Self::has_changed_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}

/// ChatRuleEntry's row box and its widgets, lent by its `compat::EntryAdapter` for each call
impl compat::RowHalf for ChatRuleEntry {
    fn entry_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry {
        &mut self.base.base
    }
    fn row_parts(&mut self) -> (&mut mcrust::net::minecraft::client::gui::screens::screen::Screen, &mut compat::WidgetCache) {
        let __s = &mut self.base;
        (&mut __s.__row, &mut __s.__widgets)
    }
}

/// ChatRuleEntry as a row of the game's list (`addEntry` attaches it)
impl ModListEntry for compat::EntryAdapter<ChatRuleEntry> {
    /// java: @Override public void extractContent(GuiGraphicsExtractor graphics, int mouseX, int mouseY, boolean hovered, float a)
    fn extract_content(&mut self, list: &mut ModSelectionList, index: usize, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, hovered: bool, a: f32) {
        self.lend(Some(list.list.entry(index)), |__h| ChatRuleEntry::extract_content(__h, &context.graphics, mouse_x, mouse_y, hovered, a));
        // the row's widgets (Java's `child.extractRenderState(..)` over `children()`)
        context.extract_widgets(&mut self.row, mouse_x, mouse_y);
    }

    fn children(&mut self) -> Option<&mut mcrust::net::minecraft::client::gui::screens::screen::Screen> {
        Some(&mut self.row)
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    fn mouse_clicked(&mut self, list: &mut ModSelectionList, index: usize, event: &MouseButtonEvent, double_click: bool) -> bool {
        self.lend(Some(list.list.entry(index)), |__h| ChatRuleEntry::mouse_clicked(__h, event, double_click))
    }

}

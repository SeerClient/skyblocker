//! Ported from de.hysky.skyblocker.skyblock.tabhud.config.list.WidgetsElementList by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::tabhud::config::list::entries::slot::widgets_list_slot_entry::WidgetsListSlotEntry;
use crate::de::hysky::skyblocker::skyblock::tabhud::config::list::entries::widgets_list_entry::WidgetsListEntry;
use crate::de::hysky::skyblocker::skyblock::tabhud::config::list::widgets_list_screen::WidgetsListScreen;
use mcrust::mcrust::modding::widgets::ModList;
use mcrust::mcrust::modding::widgets::ModSelectionList;
use mcrust::mcrust::modding::widgets::WidgetRenderContext;
use mcrust::net::minecraft::client::gui::components::abstract_selection_list::AbstractSelectionList;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::client::gui::components::renderable::Renderable as _;

/// java: static final Identifier MOVE_UP_HIGHLIGHTED_TEXTURE = Identifier.withDefaultNamespace("transferable_list/move_up_highlighted");
pub static MOVE_UP_HIGHLIGHTED_TEXTURE: compat::MainThread<Identifier> = compat::MainThread::new(|| Identifier::with_default_namespace("transferable_list/move_up_highlighted"));
/// java: static final Identifier MOVE_UP_TEXTURE = Identifier.withDefaultNamespace("transferable_list/move_up");
pub static MOVE_UP_TEXTURE: compat::MainThread<Identifier> = compat::MainThread::new(|| Identifier::with_default_namespace("transferable_list/move_up"));
/// java: static final Identifier MOVE_DOWN_HIGHLIGHTED_TEXTURE = Identifier.withDefaultNamespace("transferable_list/move_down_highlighted");
pub static MOVE_DOWN_HIGHLIGHTED_TEXTURE: compat::MainThread<Identifier> = compat::MainThread::new(|| Identifier::with_default_namespace("transferable_list/move_down_highlighted"));
/// java: static final Identifier MOVE_DOWN_TEXTURE = Identifier.withDefaultNamespace("transferable_list/move_down");
pub static MOVE_DOWN_TEXTURE: compat::MainThread<Identifier> = compat::MainThread::new(|| Identifier::with_default_namespace("transferable_list/move_down"));
/// java: public static int editingPosition = -1;
pub static EDITING_POSITION: LazyLock<Mutex<i32>> = LazyLock::new(|| Mutex::new(-1));
/// java: public static int maxPosition = -1;
pub static MAX_POSITION: LazyLock<Mutex<i32>> = LazyLock::new(|| Mutex::new(-1));
/// java: public static boolean isOnSecondPage = false;
pub static IS_ON_SECOND_PAGE: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));

/// java: public class WidgetsElementList extends ContainerObjectSelectionList<WidgetsListEntry>
pub struct WidgetsElementList {
    /// java: extends net.minecraft.client.gui.components.ContainerObjectSelectionList
    pub base: ModSelectionList,
    /// java: final int SELECT_COLOR = 0x761111FF;
    pub select_color: i32,
    /// java: final int MOVE_COLOR = 0x76FF3333;
    pub move_color: i32,
    /// java: private final WidgetsListScreen parent;
    pub parent: compat::Shared<WidgetsListScreen>,
    /// java: private @Nullable Button backButton;
    /// (the button object, held in its own `Screen` list - the list this port's widget pass draws and asks; empty is
    /// Java's `null`)
    pub back_button: mcrust::net::minecraft::client::gui::screens::screen::Screen,
    /// java: private boolean rightUpArrowHovered = false;
    pub right_up_arrow_hovered: bool,
    /// java: private boolean rightDownArrowHovered = false;
    pub right_down_arrow_hovered: bool,
    /// java: private boolean leftUpArrowHovered = false;
    pub left_up_arrow_hovered: bool,
    /// java: private boolean leftDownArrowHovered = false;
    pub left_down_arrow_hovered: bool,
    /// java: private boolean enableEditing = false;
    pub enable_editing: bool,
}

impl std::ops::Deref for WidgetsElementList {
    type Target = AbstractSelectionList;
    fn deref(&self) -> &Self::Target {
        &self.base.list
    }
}

impl std::ops::DerefMut for WidgetsElementList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base.list
    }
}

impl WidgetsElementList {
    /// java: public WidgetsElementList(WidgetsListScreen parent, Minecraft minecraftClient, int width, int height, int y)
    pub fn new(mut parent: compat::Shared<WidgetsListScreen>, minecraft_client: &mut mcrust::net::minecraft::client::minecraft::Minecraft, mut width: i32, mut height: i32, mut y: i32) -> compat::Shared<Self> {
        let __init_parent = parent.clone();
        let __this = compat::shared(Self {
            base: mcrust::mcrust::modding::widgets::ModSelectionList::new(mcrust::mcrust::modding::widgets::ListKind::Container, width, height, y, 32),
            select_color: 0x761111FF,
            move_color: 0x76FF3333,
            parent: __init_parent,
            back_button: mcrust::net::minecraft::client::gui::screens::screen::Screen::new(String::new()),
            right_up_arrow_hovered: false,
            right_down_arrow_hovered: false,
            left_up_arrow_hovered: false,
            left_down_arrow_hovered: false,
            enable_editing: false,
        });
        __this
    }

    /// java: @Override public @Nullable WidgetsListEntry getSelected()
    pub fn get_selected(__this: &compat::Shared<Self>) -> Option<compat::Shared<dyn WidgetsListEntry>> {
        if !(compat::read(__this, |__t| __t.enable_editing)) {
            return compat::super_answer::<Option<compat::Shared<dyn WidgetsListEntry>>>();
        }
        if compat::get_state(&EDITING_POSITION) < 0 || compat::get_state(&EDITING_POSITION) >= ((compat::write(__this, |__t| { let __l = &mut __t.base; compat::list_entries::<dyn WidgetsListEntry>(__l) })).len() as i32) {
            return None;
        }
        let mut entry: compat::Shared<dyn WidgetsListEntry> = (compat::write(__this, |__t| { let __l = &mut __t.base; compat::list_entries::<dyn WidgetsListEntry>(__l) }))[(compat::get_state(&EDITING_POSITION)) as usize].clone();
        // java: if (!(entry instanceof WidgetSlotEntry widgetSlotEntry) || widgetSlotEntry.getState() != WidgetSlotEntry.State.ENABLED) return null;
        use crate::de::hysky::skyblocker::skyblock::tabhud::config::list::entries::slot::widget_slot_entry::{State, WidgetSlotEntry};
        let __enabled: bool = compat::read(&entry, |__t| __t.as_any().downcast_ref::<WidgetSlotEntry>().is_some_and(|__w| __w.state == Some(State::Enabled)));
        if !__enabled {
            return None;
        }
        return Some(entry);
    }

    /// java: @Override protected boolean entriesCanBeSelected()
    pub fn entries_can_be_selected(__this: &compat::Shared<Self>) -> bool {
        return true;
    }

    /// java: @Override public void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float delta)
    /// (the part after `super.extractWidgetRenderState(..)`, which the game's list body has drawn when this runs - see
    /// `extract_list_items` below; `context` is the frame's widget render context)
    pub fn extract_widget_render_state(__this: &compat::Shared<Self>, context: &mut WidgetRenderContext<'_>, mut mouse_x: i32, mut mouse_y: i32, mut delta: f32) {
        let graphics: &mut GuiGraphicsExtractor = &mut context.graphics;
        // java: WidgetsListEntry hoveredEntry = getHovered(); - the list answers the row's index: its object is the entry
        let __hovered_index: Option<usize> = __this.lock().borrow().base.list.get_hovered();
        let mut hovered_entry: Option<compat::Shared<dyn WidgetsListEntry>> = __hovered_index.and_then(|__i| compat::read(__this, |__t| compat::entry_at::<dyn WidgetsListEntry>(&__t.base, __i)));
        if let Some(hovered_entry) = &hovered_entry {
            let __o1 = hovered_entry.clone();
            let __o2 = hovered_entry.clone();
            let __o3 = hovered_entry.clone();
            let __o4 = hovered_entry.clone();
            ({ let __f = compat::dispatch(hovered_entry, |__t| __t.extract_tooltip()); __f(hovered_entry, graphics, compat::write(&__o1, |__t| __t.widgets_list_entry_base_mut().get_x()), compat::write(&__o2, |__t| __t.widgets_list_entry_base_mut().get_y()), compat::write(&__o3, |__t| __t.widgets_list_entry_base_mut().get_width()), compat::write(&__o4, |__t| __t.widgets_list_entry_base_mut().get_height()), mouse_x, mouse_y) });
        }
        // java: if (backButton != null) backButton.extractRenderState(graphics, mouseX, mouseY, delta);
        if compat::read(__this, |__t| !__t.back_button.children().is_empty()) {
            let mut __button = compat::write(__this, |__t| std::mem::replace(&mut __t.back_button, mcrust::net::minecraft::client::gui::screens::screen::Screen::new(String::new())));
            context.extract_widgets(&mut __button, mouse_x, mouse_y);
            compat::write(__this, |__t| __t.back_button = __button);
        }
        if !(compat::read(__this, |__t| __t.enable_editing)) || WidgetsElementList::get_selected(__this).is_none() {
            return;
        }
        if compat::read(__this, |__t| __t.right_up_arrow_hovered) || compat::read(__this, |__t| __t.right_down_arrow_hovered) {
            // java: graphics.setTooltipForNextFrame(minecraft.font, Component.literal("Move widget"), mouseX, mouseY);
            compat::batch_s_set_tooltip_for_next_frame(vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::literal("Move widget"))]);
        }
        if compat::read(__this, |__t| __t.left_up_arrow_hovered) || compat::read(__this, |__t| __t.left_down_arrow_hovered) {
            // java: graphics.setTooltipForNextFrame(minecraft.font, Component.literal("Change selection"), mouseX, mouseY);
            compat::batch_s_set_tooltip_for_next_frame(vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::literal("Change selection"))]);
        }
    }

    /// java: @Override protected void extractItem(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float delta, WidgetsListEntry entry)
    pub fn extract_item(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut delta: f32, entry: &compat::Shared<dyn WidgetsListEntry>) {
        compat::call_super();
        if !(compat::read(__this, |__t| __t.enable_editing)) || !std::ptr::addr_eq(std::sync::Arc::as_ptr(&WidgetsElementList::get_selected(__this).unwrap()), std::sync::Arc::as_ptr(entry)) {
            return;
        }
        let __o1 = entry.clone();
        let mut x: i32 = compat::write(&__o1, |__t| __t.widgets_list_entry_base_mut().get_x());
        let __o2 = entry.clone();
        let mut y: i32 = compat::write(&__o2, |__t| __t.widgets_list_entry_base_mut().get_y());
        let __o3 = entry.clone();
        let mut entry_height: i32 = compat::write(&__o3, |__t| __t.widgets_list_entry_base_mut().get_height());
        let mut half_height: i32 = entry_height.wrapping_div(2);
        let mut right_x_good: bool = (mouse_x >= (__this.lock().borrow().get_row_right()).wrapping_add(2) && mouse_x < (__this.lock().borrow().get_row_right()).wrapping_add(15));
        let mut left_x_good: bool = (mouse_x >= x.wrapping_sub(15) && mouse_x < x.wrapping_sub(2));
        let mut is_on_up: bool = (mouse_y >= y.wrapping_add(5) && mouse_y < y.wrapping_add(half_height));
        let mut is_on_down: bool = (mouse_y >= y.wrapping_add(half_height) && mouse_y < (y.wrapping_add(entry_height)).wrapping_sub(5));
        { let __v = (right_x_good && is_on_up); compat::write(__this, |__t| __t.right_up_arrow_hovered = __v); }
        { let __v = (right_x_good && is_on_down); compat::write(__this, |__t| __t.right_down_arrow_hovered = __v); }
        { let __v = (left_x_good && is_on_up); compat::write(__this, |__t| __t.left_up_arrow_hovered = __v); }
        { let __v = (left_x_good && is_on_down); compat::write(__this, |__t| __t.left_down_arrow_hovered = __v); }
        graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), (__this.lock().borrow().get_row_right()).wrapping_add(2), y.wrapping_add(5), (__this.lock().borrow().get_row_right()).wrapping_add(15), y.wrapping_add(27), compat::read(__this, |__t| __t.move_color));
        gui_graphics_extractor::blit_sprite_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), &(if compat::read(__this, |__t| __t.right_up_arrow_hovered) { MOVE_UP_HIGHLIGHTED_TEXTURE.get_mut().clone() } else { MOVE_UP_TEXTURE.get_mut().clone() }), (__this.lock().borrow().get_row_right()).wrapping_sub(15), y, 32, 32);
        gui_graphics_extractor::blit_sprite_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), &(if compat::read(__this, |__t| __t.right_down_arrow_hovered) { MOVE_DOWN_HIGHLIGHTED_TEXTURE.get_mut().clone() } else { MOVE_DOWN_TEXTURE.get_mut().clone() }), (__this.lock().borrow().get_row_right()).wrapping_sub(15), y, 32, 32);
        graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), x.wrapping_sub(15), y.wrapping_add(5), x.wrapping_sub(2), y.wrapping_add(27), compat::read(__this, |__t| __t.select_color));
        gui_graphics_extractor::blit_sprite_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), &(if compat::read(__this, |__t| __t.left_up_arrow_hovered) { MOVE_UP_HIGHLIGHTED_TEXTURE.get_mut().clone() } else { MOVE_UP_TEXTURE.get_mut().clone() }), x.wrapping_sub(32), y, 32, 32);
        gui_graphics_extractor::blit_sprite_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), &(if compat::read(__this, |__t| __t.left_down_arrow_hovered) { MOVE_DOWN_HIGHLIGHTED_TEXTURE.get_mut().clone() } else { MOVE_DOWN_TEXTURE.get_mut().clone() }), x.wrapping_sub(32), y, 32, 32);
    }

    /// java: @Override protected int scrollBarX()
    pub fn scroll_bar_x(__this: &compat::Shared<Self>) -> i32 {
        return (compat::super_answer::<i32>()).wrapping_add(if compat::get_state(&EDITING_POSITION) != -1 { 15 } else { 0 });
    }

    /// java: @Override public int getRowWidth()
    pub fn get_row_width(__this: &compat::Shared<Self>) -> i32 {
        return 280;
    }

    /// java: public void setEditingPosition(boolean enable, int editing, int max)
    pub fn set_editing_position(__this: &compat::Shared<Self>, mut enable: bool, mut editing: i32, mut max: i32) {
        { let __v = enable; compat::write(__this, |__t| __t.enable_editing = __v); }
        compat::set_state(&EDITING_POSITION, editing);
        compat::set_state(&MAX_POSITION, max);
    }

    /// java: public void setIsOnSecondPage(boolean secondPage)
    pub fn set_is_on_second_page(__this: &compat::Shared<Self>, mut second_page: bool) {
        compat::set_state(&IS_ON_SECOND_PAGE, second_page);
        if second_page {
            compat::set_state(&EDITING_POSITION, (compat::get_state(&EDITING_POSITION)).wrapping_sub(21));
            __this.lock().borrow_mut().refresh_scroll_amount();
        }
    }

    /// java: public void setBackButton(Button backButton)
    pub fn set_back_button(__this: &compat::Shared<Self>, mut back_button: AbstractWidget) {
        compat::write(__this, |__t| {
            __t.back_button.clear_widgets();
            __t.back_button.add_renderable_widget(back_button);
        });
    }

    /// java: public void updateList()
    pub fn update_list(__this: &compat::Shared<Self>) {
        let mut entries: Vec<(i32, compat::Shared<dyn WidgetsListSlotEntry>)> = WidgetsListScreen::get_entries(&compat::read(__this, |__t| __t.parent.clone())).into_iter().collect::<Vec<_>>();
        compat::write(__this, |__t| { let __l = &mut __t.base; __l.clear_entries() });
        // java: entries.stream().sorted(Comparator.comparingInt(Int2ObjectMap.Entry::getIntKey)).map(Map.Entry::getValue).forEach(this::addEntry);
        entries.sort_by_key(|(key, _)| *key);
        for (_, entry) in entries {
            let __entry: compat::Shared<dyn WidgetsListEntry> = compat::view(&entry, |a| a as compat::Shared<dyn WidgetsListEntry>).clone();
            compat::write(__this, |__t| __t.base.add_entry(Box::new(compat::EntryAdapter::new(__entry))));
        }
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    pub fn mouse_clicked(__this: &compat::Shared<Self>, click: &MouseButtonEvent, mut doubled: bool) -> bool {
        match WidgetsElementList::mouse_clicked_before_super(__this, click, doubled) {
            Some(answer) => answer,
            // java: return super.mouseClicked(click, doubled);
            None => WidgetsElementList::super_mouse_clicked(__this, click, doubled),
        }
    }

    /// `mouseClicked`'s body up to its `return super.mouseClicked(click, doubled)` statements: the answer, or `None`
    /// where the body returns `super`'s (which the caller runs over the live list)
    pub fn mouse_clicked_before_super(__this: &compat::Shared<Self>, click: &MouseButtonEvent, mut doubled: bool) -> Option<bool> {
        // java: if (backButton != null && backButton.mouseClicked(click, doubled)) return true;
        // (the button's own list runs its press; the list is taken out for it, as the press may reach this object)
        if compat::read(__this, |__t| !__t.back_button.children().is_empty()) {
            let mut __button = compat::write(__this, |__t| std::mem::replace(&mut __t.back_button, mcrust::net::minecraft::client::gui::screens::screen::Screen::new(String::new())));
            let __pressed = __button.mouse_clicked(click.button(), click.x(), click.y()).is_some();
            compat::write(__this, |__t| if __t.back_button.children().is_empty() { __t.back_button = __button; });
            if __pressed {
                return Some(true);
            }
        }
        if !(compat::read(__this, |__t| __t.enable_editing)) || WidgetsElementList::get_selected(__this).is_none() {
            return None;
        }
        let __parent = compat::read(__this, |__t| __t.parent.clone());
        if compat::read(__this, |__t| __t.right_up_arrow_hovered) {
            WidgetsListScreen::shift_click_and_wait_for_server(&__parent, 13, 1);
            return Some(true);
        }
        if compat::read(__this, |__t| __t.right_down_arrow_hovered) {
            WidgetsListScreen::shift_click_and_wait_for_server(&__parent, 13, 0);
            return Some(true);
        }
        if compat::read(__this, |__t| __t.left_up_arrow_hovered) {
            WidgetsListScreen::click_and_wait_for_server(&__parent, 13, 1);
            return Some(true);
        }
        if compat::read(__this, |__t| __t.left_down_arrow_hovered) {
            WidgetsListScreen::click_and_wait_for_server(&__parent, 13, 0);
            return Some(true);
        }
        None
    }

    /// `super.mouseClicked(click, doubled)` - `ContainerObjectSelectionList`'s body (`ModSelectionList::mouse_clicked_base`)
    /// over this object's list. The list is taken out of the object for the call, so a row that reaches back into this
    /// object meets no held borrow (it sees an empty stand-in list until the call returns).
    pub fn super_mouse_clicked(__this: &compat::Shared<Self>, click: &MouseButtonEvent, doubled: bool) -> bool {
        let mut __list: ModSelectionList = compat::write(__this, |__t| {
            let __kind = __t.base.kind;
            std::mem::replace(&mut __t.base, ModSelectionList::new(__kind, 0, 0, 0, 0))
        });
        let __answer: bool = __list.mouse_clicked_base(click, doubled);
        compat::write(__this, |__t| __t.base = __list);
        __answer
    }

}

/// WidgetsElementList's list (the port's `ModSelectionList`) until the game takes it (`compat::attach_list`)
impl compat::ListHalf for WidgetsElementList {
    fn list_half(&mut self) -> &mut ModSelectionList {
        &mut self.base
    }
}

/// WidgetsElementList's overrides of the list's own methods, asked by the port's list bodies
impl ModList for compat::ListClass<WidgetsElementList> {
    /// java: @Override public int getRowWidth()
    fn get_row_width(&self, list: &AbstractSelectionList, super_answer: i32) -> i32 {
        self.ask(list, super_answer, |__h| WidgetsElementList::get_row_width(__h))
    }

    /// java: @Override protected int scrollBarX()
    fn scroll_bar_x(&self, list: &AbstractSelectionList, super_answer: i32) -> i32 {
        self.ask(list, super_answer, |__h| WidgetsElementList::scroll_bar_x(__h))
    }

    /// java: @Override protected boolean entriesCanBeSelected()
    fn entries_can_be_selected(&self, list: &AbstractSelectionList, super_answer: bool) -> bool {
        self.ask(list, super_answer, |__h| WidgetsElementList::entries_can_be_selected(__h))
    }

    /// java: @Override public @Nullable WidgetsListEntry getSelected()
    fn get_selected(&self, list: &AbstractSelectionList, super_answer: Option<usize>) -> Option<usize> {
        let __super = super_answer.and_then(|__i| compat::row_object::<dyn WidgetsListEntry>(list, __i));
        let __entry = self.ask(list, __super, |__h| WidgetsElementList::get_selected(__h));
        __entry.and_then(|__e| compat::row_index::<dyn WidgetsListEntry>(list, &__e))
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    fn mouse_clicked(&self, list: &mut ModSelectionList, event: &MouseButtonEvent, double_click: bool, super_mouse_clicked: &mut dyn FnMut(&mut ModSelectionList) -> bool) -> bool {
        match self.ask(&list.list, false, |__h| WidgetsElementList::mouse_clicked_before_super(__h, event, double_click)) {
            Some(answer) => answer,
            None => super_mouse_clicked(list),
        }
    }

    /// java: @Override public void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float delta) -
    /// `super.extractWidgetRenderState(..)` and then this class's part. The game's list body asks this slot inside that
    /// super body (after the background, before the scrollbar); the part after `super` sets deferred tooltips and
    /// draws the back button, so it runs once the rows are drawn.
    fn extract_list_items(&self, list: &mut ModSelectionList, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32, super_extract_list_items: &mut dyn FnMut(&mut ModSelectionList, &mut WidgetRenderContext<'_>)) {
        super_extract_list_items(list, context);
        self.ask(&list.list, (), |__h| WidgetsElementList::extract_widget_render_state(__h, context, mouse_x, mouse_y, a))
    }

    /// java: @Override protected void extractItem(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float delta, WidgetsListEntry entry)
    fn extract_item(&self, list: &mut ModSelectionList, row: usize, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32, super_extract_item: &mut dyn FnMut(&mut ModSelectionList, &mut WidgetRenderContext<'_>)) {
        super_extract_item(list, context);
        let Some(__entry) = compat::row_object::<dyn WidgetsListEntry>(&list.list, row) else { return; };
        compat::with_super(&mut || {}, || self.ask(&list.list, (), |__h| WidgetsElementList::extract_item(__h, &mut context.graphics, mouse_x, mouse_y, a, &__entry)))
    }

}


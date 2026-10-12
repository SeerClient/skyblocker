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
    pub back_button: Option<AbstractWidget>,
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
            back_button: None,
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
        // TODO(fabric2seer): an `instanceof` pattern under `||` / `!`
        //   java: if (!(entry instanceof WidgetSlotEntry widgetSlotEntry) || widgetSlotEntry.getState() != WidgetSlotEntry.State.ENABLED) return null;
        return Some(entry);
    }

    /// java: @Override protected boolean entriesCanBeSelected()
    pub fn entries_can_be_selected(__this: &compat::Shared<Self>) -> bool {
        return true;
    }

    /// java: @Override public void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float delta)
    pub fn extract_widget_render_state(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut delta: f32) {
        // TODO(fabric2seer): `ContainerObjectSelectionList.extractWidgetRenderState` has no Rust method in the port (looked for `extract_widget_render_state` in ContainerObjectSelectionList and its supertypes)
        // rust: __this.lock().borrow_mut().base.list.extract_widget_render_state(graphics, mouse_x, mouse_y, delta);
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
        if compat::read(__this, |__t| __t.back_button.clone()).is_some() {
            // TODO(fabric2seer): `extract_render_state` is a method of the port's trait `Renderable`, which the port's `AbstractWidget` does not implement
            // rust: { let (__q0, __q1, __q2, __q3, __q4,) = (mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mouse_x, mouse_y, delta,); __this.lock().borrow_mut().back_button.as_mut().unwrap().extract_render_state(__q0, __q1, __q2, __q3, __q4) };
        }
        if !(compat::read(__this, |__t| __t.enable_editing)) || WidgetsElementList::get_selected(__this).is_none() {
            return;
        }
        if compat::read(__this, |__t| __t.right_up_arrow_hovered) || compat::read(__this, |__t| __t.right_down_arrow_hovered) {
            // TODO(fabric2seer): the port's `set_tooltip_for_next_frame_text` ports this Java method with other parameters (tooltip_for_next_frame, text for Java's text, xo, yo)
            // TODO(fabric2seer): `set_tooltip_for_next_frame_text` takes 2 argument(s) in the port (`tooltip_for_next_frame: &mut Option<DeferredTooltip>, text: crate::net::minecraft::network::chat::formatted_text::FormattedText`), Java passes 4
            // rust: gui_graphics_extractor::set_tooltip_for_next_frame_text(&mut compat::font(), mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::literal("Move widget")), mouse_x, mouse_y);
        }
        if compat::read(__this, |__t| __t.left_up_arrow_hovered) || compat::read(__this, |__t| __t.left_down_arrow_hovered) {
            // TODO(fabric2seer): the port's `set_tooltip_for_next_frame_text` ports this Java method with other parameters (tooltip_for_next_frame, text for Java's text, xo, yo)
            // TODO(fabric2seer): `set_tooltip_for_next_frame_text` takes 2 argument(s) in the port (`tooltip_for_next_frame: &mut Option<DeferredTooltip>, text: crate::net::minecraft::network::chat::formatted_text::FormattedText`), Java passes 4
            // rust: gui_graphics_extractor::set_tooltip_for_next_frame_text(&mut compat::font(), mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::literal("Change selection")), mouse_x, mouse_y);
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
        { let __v = Some(back_button.clone()); compat::write(__this, |__t| __t.back_button = __v); }
    }

    /// java: public void updateList()
    pub fn update_list(__this: &compat::Shared<Self>) {
        let mut entries: Vec<(i32, compat::Shared<dyn WidgetsListSlotEntry>)> = WidgetsListScreen::get_entries(&compat::read(__this, |__t| __t.parent.clone())).into_iter().collect::<Vec<_>>();
        compat::write(__this, |__t| { let __l = &mut __t.base; __l.clear_entries() });
        // TODO(fabric2seer): static field Int2ObjectMap.Entry
        //   java: entries.stream()
        //   java: 				.sorted(Comparator.comparingInt(Int2ObjectMap.Entry::getIntKey))
        //   java: 				.map(Map.Entry::getValue)
        //   java: 				.forEach(this::addEntry);
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    pub fn mouse_clicked(__this: &compat::Shared<Self>, click: &MouseButtonEvent, mut doubled: bool) -> bool {
        // java: if (backButton != null && backButton.mouseClicked(click, doubled)) return true;
        // rust: port gap (GAPS_batchC #L3) - the back button is this list's own widget value, not a screen's: the port's
        // `mouse_clicked` answers the press it would run (its `OnPress`), which only a screen can run
        if compat::read(__this, |__t| __t.back_button.is_some()) && __this.lock().borrow().back_button.as_ref().unwrap().mouse_clicked(click.button(), click.x(), click.y()).is_some() {
            return true;
        }
        if !(compat::read(__this, |__t| __t.enable_editing)) || WidgetsElementList::get_selected(__this).is_none() {
            // TODO(fabric2seer): `AbstractContainerWidget::mouse_clicked` ports `mouseClicked`, but the port's `AbstractSelectionList` holds no `AbstractContainerWidget` (no Deref, no `base` field) to call it on
            // rust: return { let (__q0, __q1,) = (click.clone(), doubled,); __this.lock().borrow_mut().base.list.mouse_clicked(__q0, __q1) };
            return todo!("fabric2seer: see the TODO above");
        }
        if compat::read(__this, |__t| __t.right_up_arrow_hovered) {
            WidgetsListScreen::shift_click_and_wait_for_server(&compat::read(__this, |__t| __t.parent.clone()), 13, 1);
            return true;
        }
        if compat::read(__this, |__t| __t.right_down_arrow_hovered) {
            WidgetsListScreen::shift_click_and_wait_for_server(&compat::read(__this, |__t| __t.parent.clone()), 13, 0);
            return true;
        }
        if compat::read(__this, |__t| __t.left_up_arrow_hovered) {
            WidgetsListScreen::click_and_wait_for_server(&compat::read(__this, |__t| __t.parent.clone()), 13, 1);
            return true;
        }
        if compat::read(__this, |__t| __t.left_down_arrow_hovered) {
            WidgetsListScreen::click_and_wait_for_server(&compat::read(__this, |__t| __t.parent.clone()), 13, 0);
            return true;
        }
        // TODO(fabric2seer): `AbstractContainerWidget::mouse_clicked` ports `mouseClicked`, but the port's `AbstractSelectionList` holds no `AbstractContainerWidget` (no Deref, no `base` field) to call it on
        // rust: return { let (__q0, __q1,) = (click.clone(), doubled,); __this.lock().borrow_mut().base.list.mouse_clicked(__q0, __q1) };
        return todo!("fabric2seer: see the TODO above");
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

    /// java: @Override protected void extractItem(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float delta, WidgetsListEntry entry)
    fn extract_item(&self, list: &mut ModSelectionList, row: usize, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32, super_extract_item: &mut dyn FnMut(&mut ModSelectionList, &mut WidgetRenderContext<'_>)) {
        super_extract_item(list, context);
        let Some(__entry) = compat::row_object::<dyn WidgetsListEntry>(&list.list, row) else { return; };
        compat::with_super(&mut || {}, || self.ask(&list.list, (), |__h| WidgetsElementList::extract_item(__h, &mut context.graphics, mouse_x, mouse_y, a, &__entry)))
    }

}

// NOTE(fabric2seer): entriesCanBeSelected, extractItem, extractWidgetRenderState, getRowWidth, getSelected, mouseClicked, scrollBarX override(s) of ContainerObjectSelectionList - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them

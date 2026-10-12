//! Ported from de.hysky.skyblocker.utils.render.gui.DropdownWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::tabhud::config::custom_dropdown_widget::CustomEntry;
use crate::de::hysky::skyblocker::utils::render::gui_helper::GuiHelper;
use mcrust::com::mojang::blaze3d::platform::cursor::cursor_types::CursorTypes;
use mcrust::mcrust::modding::widgets::ModListEntry;
use mcrust::mcrust::modding::widgets::ModSelectionList;
use mcrust::mcrust::modding::widgets::WidgetRenderContext;
use mcrust::net::minecraft::client::gui::components::abstract_container_widget::AbstractContainerWidget;
use mcrust::net::minecraft::client::gui::components::abstract_selection_list::AbstractSelectionList;
use mcrust::net::minecraft::client::gui::components::events::container_event_handler;
use mcrust::net::minecraft::client::gui::components::events::gui_event_listener::GuiEventListener;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::narration::narratable_entry::NarratableEntry;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::client::gui::components::renderable::Renderable as _;

/// java: public class DropdownWidget<T> extends AbstractContainerWidget
#[derive(Clone)]
pub struct DropdownWidgetBase<T> {
    /// java: extends net.minecraft.client.gui.components.AbstractContainerWidget
    pub base: AbstractContainerWidget,
    /// java: public final int entryHeight;
    pub entry_height: i32,
    /// java: protected int headerHeight;
    pub header_height: i32,
    /// java: protected final List<T> entries;
    pub entries: Vec<T>,
    /// java: protected final Consumer<T> selectCallback;
    pub select_callback: Option<Arc<dyn Fn(T)>>,
    /// java: protected final Consumer<Boolean> openedCallback;
    pub opened_callback: Option<Arc<dyn Fn(bool)>>,
    /// java: private final DropdownList dropdownList;
    pub dropdown_list: Option<compat::Shared<dyn DropdownList>>,
    /// java: protected T prevSelected;
    pub prev_selected: Option<T>,
    /// java: protected T selected;
    pub selected: Option<T>,
    /// java: protected boolean open;
    pub open: bool,
    /// java: private int maxHeight;
    pub max_height: i32,
    /// java: protected Function<T, Component> formatter = t -> Component.literal(t.toString());
    pub formatter: Arc<dyn Fn(T) -> MutableComponent>,
}

impl<T: 'static> std::ops::Deref for DropdownWidgetBase<T> {
    type Target = AbstractContainerWidget;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<T: 'static> std::ops::DerefMut for DropdownWidgetBase<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl<T: Clone + 'static> DropdownWidgetBase<T> {
    /// java: public DropdownWidget(Minecraft minecraftClient, int x, int y, int width, int maxHeight, int entryHeight, List<T> entries, Consumer<T> selectCallback, T selected, Consumer<Boolean> openedCallback)
    pub fn new(minecraft_client: &mut mcrust::net::minecraft::client::minecraft::Minecraft, mut x: i32, mut y: i32, mut width: i32, mut max_height: i32, mut entry_height: i32, mut entries: Vec<T>, select_callback: Arc<dyn Fn(T)>, mut selected: T, opened_callback: Arc<dyn Fn(bool)>) -> Self {
        let __init_entry_height = entry_height;
        let __init_header_height = entry_height.wrapping_add(4);
        let __init_max_height = max_height;
        let __init_entries = entries.clone();
        let __init_select_callback = Some(select_callback.clone());
        let __init_opened_callback = Some(opened_callback.clone());
        let __init_selected = Some(selected.clone());
        // TODO(fabric2seer): call to `defaultSettings(..)` (not a method of this class)
        //   java: super(x, y, width, 0, Component.empty(), defaultSettings(4));
        let mut this: Self = todo!("constructor");
        this.dropdown_list = Some(this.create_dropdown());
        let __wa1 = width.wrapping_sub(2);
        let __wa2 = max_height.wrapping_sub(this.header_height);
        let __wa3 = x.wrapping_add(1);
        let __wa4 = y.wrapping_add(this.header_height);
        let __o1 = this.dropdown_list.clone().unwrap();
        compat::write(&__o1, |__t| __t.dropdown_list_base_mut().set_rectangle(__wa1, __wa2, __wa3, __wa4));
        for element in entries.iter() {
            ({ let __r = this.dropdown_list.as_ref().unwrap(); let __f = compat::dispatch(__r, |__t| __t.add_entry()); __f(__r, &this.create_entry(element)) });
        }
        this.set_height(this.header_height);
        this
    }

    /// java: public DropdownWidget(Minecraft minecraftClient, int x, int y, int width, int maxHeight, List<T> entries, Consumer<T> selectCallback, T selected, Consumer<Boolean> openedCallback)
    pub fn new_9(minecraft_client: &mut mcrust::net::minecraft::client::minecraft::Minecraft, mut x: i32, mut y: i32, mut width: i32, mut max_height: i32, mut entries: Vec<T>, select_callback: Arc<dyn Fn(T)>, mut selected: T, opened_callback: Arc<dyn Fn(bool)>) -> Self {
        let mut this = Self::new(minecraft_client, x, y, width, max_height, 15, entries.clone(), select_callback.clone(), selected.clone(), opened_callback.clone());
        this
    }

}

/// java: the virtual methods of DropdownWidget (implemented by DropdownWidgetBase and every subclass)
pub trait DropdownWidget<T> {
    fn dropdown_widget_base(&self) -> &DropdownWidgetBase<T>;
    fn dropdown_widget_base_mut(&mut self) -> &mut DropdownWidgetBase<T>;
    fn as_any(&self) -> &dyn std::any::Any;

    /// java: public int getHeaderHeight()
    fn get_header_height(&self) -> i32 {
        return self.dropdown_widget_base().header_height;
    }

    /// java: protected DropdownList createDropdown()
    fn create_dropdown(&self) -> compat::Shared<dyn DropdownList> {
        return (DropdownListBase::new(compat::minecraft()) as compat::Shared<dyn DropdownList>);
    }

    /// java: protected Entry createEntry(T element)
    fn create_entry(&self, element: &T) -> (_, _) {
        return (EntryBase::new(element.clone()) as (_, _));
    }

    /// java: public void setFormatter(Function<T, Component> formatter)
    fn set_formatter(&mut self, formatter: Arc<dyn Fn(T) -> MutableComponent>) {
        self.dropdown_widget_base_mut().formatter = formatter.clone();
    }

    /// java: public void setMaxHeight(int maxHeight)
    fn set_max_height(&mut self, mut max_height: i32) {
        self.dropdown_widget_base_mut().max_height = max_height;
        { let __s0 = self.dropdown_widget_base().open; self.set_open(__s0) };
    }

    /// java: @Override public List<? extends GuiEventListener> children()
    fn children(&self) -> Vec<Box<dyn GuiEventListener>> {
        return vec![self.dropdown_widget_base().dropdown_list.clone().unwrap()];
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float delta)
    fn extract_widget_render_state(&mut self, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut delta: f32) {
        // TODO(fabric2seer): `AbstractContainerWidget.visible` is a field of the port's `AbstractContainerWidget`, which the port's `AbstractSelectionList` does not hold (no Deref, no `base` field)
        // rust: { let __v = self.dropdown_widget_base().open; compat::write(self.dropdown_widget_base().dropdown_list.as_ref().unwrap(), |__t| __t.dropdown_list_base_mut().visible = __v); }
        // TODO(fabric2seer): `extract_render_state` is a method of the port's trait `Renderable`, which the port's `AbstractSelectionList` does not implement
        // rust: let __wa1 = graphics.clone();
        // rust: let __wa2 = mouse_x;
        // rust: let __wa3 = mouse_y;
        // rust: let __wa4 = delta;
        // rust: let __o1 = self.dropdown_widget_base().dropdown_list.clone().unwrap();
        // rust: compat::write(&__o1, |__t| __t.dropdown_list_base_mut().extract_render_state(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), __wa2, __wa3, __wa4));
        self.extract_header(graphics, mouse_x, mouse_y, delta);
        if self.dropdown_widget_base().is_mouse_over((mouse_x) as f64, (mouse_y) as f64) {
            // TODO(fabric2seer): static `CursorTypes.POINTING_HAND` has no Rust counterpart in the port's CursorTypes file
            // TODO(fabric2seer): `GuiGraphicsExtractor.requestCursor` has no Rust method in the port (looked for `request_cursor` in GuiGraphicsExtractor and its supertypes)
            // rust: graphics.request_cursor(&CursorTypes::POINTING_HAND);
        }
    }

    /// java: protected void extractHeader(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float delta)
    fn extract_header(&self, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut delta: f32) {
        graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), self.dropdown_widget_base().get_x(), self.dropdown_widget_base().get_y(), self.dropdown_widget_base().get_right(), ((self.dropdown_widget_base().get_y()).wrapping_add(self.dropdown_widget_base().header_height)).wrapping_add(1), mcrust::net::minecraft::util::common_colors::BLACK);
        GuiHelper::border(graphics, self.dropdown_widget_base().get_x(), self.dropdown_widget_base().get_y(), self.dropdown_widget_base().get_width(), (self.dropdown_widget_base().header_height).wrapping_add(1), mcrust::net::minecraft::util::common_colors::WHITE);
        gui_graphics_extractor::text_string_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), Some(">"), (self.dropdown_widget_base().get_x()).wrapping_add(4), (self.dropdown_widget_base().get_y()).wrapping_add(6), mcrust::net::minecraft::util::common_colors::LIGHTER_GRAY, true);
        gui_graphics_extractor::text_string_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), Some(&compat::opt_str(&self.dropdown_widget_base().selected)), (self.dropdown_widget_base().get_x()).wrapping_add(12), (self.dropdown_widget_base().get_y()).wrapping_add(6), mcrust::net::minecraft::util::common_colors::WHITE, true);
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    fn update_widget_narration(&self, builder: &dyn NarrationElementOutput) {
    }

    /// java: private void setOpen(boolean open)
    fn set_open(&mut self, mut open: bool) {
        self.dropdown_widget_base_mut().open = open;
        if self.dropdown_widget_base().open {
            self.set_height(self.dropdown_widget_base().max_height);
            let __wa1 = (((self.dropdown_widget_base().entries.len() as i32).wrapping_mul(self.dropdown_widget_base().entry_height)).wrapping_add(4)).min((self.dropdown_widget_base().max_height).wrapping_sub(self.dropdown_widget_base().header_height));
            let __o1 = self.dropdown_widget_base().dropdown_list.clone().unwrap();
            compat::write(&__o1, |__t| __t.dropdown_list_base_mut().set_height(__wa1));
        } else {
            self.set_height(self.dropdown_widget_base().header_height);
        }
        (self.dropdown_widget_base().opened_callback.as_ref().unwrap())(open);
    }

    /// java: public boolean isOpen()
    fn is_open(&self) -> bool {
        return self.dropdown_widget_base().open;
    }

    /// java: protected void select(T entry)
    fn select(&mut self, mut entry: T) {
        self.dropdown_widget_base_mut().selected = Some(entry.clone());
        self.set_open(false);
        if self.dropdown_widget_base().selected.clone().unwrap() != self.dropdown_widget_base().prev_selected.clone().unwrap() {
            (self.dropdown_widget_base().select_callback.as_ref().unwrap())(entry.clone());
            self.dropdown_widget_base_mut().prev_selected = self.dropdown_widget_base().selected.clone();
        }
    }

    /// java: @Override public void setX(int x)
    fn set_x(&mut self, mut x: i32) {
        self.dropdown_widget_base().set_x(x);
        let __wa1 = (self.dropdown_widget_base().get_x()).wrapping_add(1);
        let __o1 = self.dropdown_widget_base().dropdown_list.clone().unwrap();
        compat::write(&__o1, |__t| __t.dropdown_list_base_mut().set_x(__wa1));
        let __o2 = self.dropdown_widget_base().dropdown_list.clone().unwrap();
        compat::write(&__o2, |__t| __t.dropdown_list_base_mut().refresh_scroll_amount());
    }

    /// java: @Override public void setY(int y)
    fn set_y(&mut self, mut y: i32) {
        self.dropdown_widget_base().set_y(y);
        let __wa1 = (self.dropdown_widget_base().get_y()).wrapping_add(self.dropdown_widget_base().header_height);
        let __o1 = self.dropdown_widget_base().dropdown_list.clone().unwrap();
        compat::write(&__o1, |__t| __t.dropdown_list_base_mut().set_y(__wa1));
    }

    /// java: @Override public void setWidth(int width)
    fn set_width(&mut self, mut width: i32) {
        self.dropdown_widget_base().set_width(width);
        let __wa1 = (self.dropdown_widget_base().get_width()).wrapping_sub(2);
        let __o1 = self.dropdown_widget_base().dropdown_list.clone().unwrap();
        compat::write(&__o1, |__t| __t.dropdown_list_base_mut().set_width(__wa1));
    }

    /// java: @Override public void setHeight(int height)
    fn set_height(&self, mut height: i32) {
        self.dropdown_widget_base().set_height(height);
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    fn mouse_clicked(&mut self, click: &MouseButtonEvent, mut doubled: bool) -> bool {
        if !(self.dropdown_widget_base().visible) {
            return false;
        }
        if (self.dropdown_widget_base().get_x() as f64) <= click.x() && click.x() < ((self.dropdown_widget_base().get_x()).wrapping_add(self.dropdown_widget_base().get_width()) as f64) && (self.dropdown_widget_base().get_y() as f64) <= click.y() && click.y() < ((self.dropdown_widget_base().get_y()).wrapping_add(self.dropdown_widget_base().header_height) as f64) {
            { let __s0 = !(self.dropdown_widget_base().open); self.set_open(__s0) };
            // TODO(fabric2seer): `AbstractContainerWidget.playDownSound` has no Rust method in the port (looked for `play_down_sound` in AbstractContainerWidget and its supertypes)
            // rust: self.dropdown_widget_base().play_down_sound(compat::minecraft().get_sound_manager());
            return true;
        }
        return self.dropdown_widget_base().mouse_clicked(click.clone(), doubled);
    }

    /// java: @Override protected int contentHeight()
    fn content_height(&self) -> i32 {
        return self.dropdown_widget_base().get_height();
    }

    /// java: @Override protected double scrollRate()
    fn scroll_rate(&self) -> f64 {
        return ((0) as f64) as f64;
    }

    /// java: @Override public boolean mouseScrolled(double mouseX, double mouseY, double horizontalAmount, double verticalAmount)
    fn mouse_scrolled(&self, mut mouse_x: f64, mut mouse_y: f64, mut horizontal_amount: f64, mut vertical_amount: f64) -> bool {
        if !(self.dropdown_widget_base().visible) {
            return false;
        }
        // TODO(fabric2seer): `get_child_at` takes 3 argument(s) in the port (`children: &[(usize, ScreenRectangle)], x: f64, y: f64`), Java passes 2
        // TODO(fabric2seer): the condition: Optional.mouseScrolled
        //   java: this.getChildAt(..).filter(..).isPresent(..)
        if todo!("fabric2seer: the condition (TODO above)") {
            return true;
        }
        // TODO(fabric2seer): `mouse_scrolled` takes 2 argument(s) in the port (`content_height: i32, scroll_y: f64`), Java passes 4
        // TODO(fabric2seer): `ScrollArea::mouse_scrolled` ports `mouseScrolled`, but the port's `AbstractContainerWidget` holds no `ScrollArea` (no Deref, no `base` field) to call it on
        // rust: return self.dropdown_widget_base().mouse_scrolled(mouse_x, mouse_y, horizontal_amount, vertical_amount);
        return todo!("fabric2seer: see the TODO above");
    }

}

impl<T: Clone + 'static> DropdownWidget<T> for DropdownWidgetBase<T> {
    fn dropdown_widget_base(&self) -> &DropdownWidgetBase<T> {
        &self
    }
    fn dropdown_widget_base_mut(&mut self) -> &mut DropdownWidgetBase<T> {
        &mut self
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}

/// java: protected class DropdownList extends ContainerObjectSelectionList<Entry>
pub struct DropdownListBase {
    /// java: extends net.minecraft.client.gui.components.ContainerObjectSelectionList
    pub base: ModSelectionList,
}

impl std::ops::Deref for DropdownListBase {
    type Target = AbstractSelectionList;
    fn deref(&self) -> &Self::Target {
        &self.base.list
    }
}

impl std::ops::DerefMut for DropdownListBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base.list
    }
}

impl DropdownListBase {
    /// java: protected DropdownList(Minecraft minecraftClient) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_in<__R>(minecraft_client: &mut mcrust::net::minecraft::client::minecraft::Minecraft, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn DropdownList>, __R)) -> (compat::Shared<dyn DropdownList>, __R) {
        // TODO(fabric2seer): `entryHeight` is a field of the enclosing instance (DropdownWidget.this) - an inner class has no outer reference in Rust
        //   java: super(minecraftClient, 0, 0, 0, entryHeight);
        let (__this, __r): (compat::Shared<dyn DropdownList>, __R) = todo!("constructor");
        (__this, __r)
    }

    /// java: protected DropdownList(Minecraft minecraftClient)
    pub fn new(minecraft_client: &mut mcrust::net::minecraft::client::minecraft::Minecraft) -> compat::Shared<Self> {
        Self::new_in(minecraft_client, |this| {
            let __h = compat::shared(this);
            ((__h.clone() as compat::Shared<dyn DropdownList>), __h)
        }).1
    }

}

/// java: the virtual methods of DropdownList (implemented by DropdownListBase and every subclass)
pub trait DropdownList: compat::DynAny {
    fn dropdown_list_base(&self) -> &DropdownListBase;
    fn dropdown_list_base_mut(&mut self) -> &mut DropdownListBase;
    fn as_any(&self) -> &dyn std::any::Any;

    /// java: @Override protected int addEntry(DropdownWidget<T>.Entry entry) (dispatch: the body for this class)
    fn add_entry(&self) -> fn(&compat::Shared<dyn DropdownList>, &(_, _)) -> i32 {
        <dyn DropdownList>::add_entry_body
    }

    /// java: @Override public int getRowLeft() (dispatch: the body for this class)
    fn get_row_left(&self) -> fn(&compat::Shared<dyn DropdownList>) -> i32 {
        <dyn DropdownList>::get_row_left_body
    }

    /// java: @Override public int getRowWidth() (dispatch: the body for this class)
    fn get_row_width(&self) -> fn(&compat::Shared<dyn DropdownList>) -> i32 {
        <dyn DropdownList>::get_row_width_body
    }

    /// java: @Override protected void extractScrollbar(GuiGraphicsExtractor graphics, int mouseX, int mouseY) (dispatch: the body for this class)
    fn extract_scrollbar(&self) -> fn(&compat::Shared<dyn DropdownList>, &mut GuiGraphicsExtractor, i32, i32) {
        <dyn DropdownList>::extract_scrollbar_body
    }

    /// java: @Override protected int scrollBarX() (dispatch: the body for this class)
    fn scroll_bar_x(&self) -> fn(&compat::Shared<dyn DropdownList>) -> i32 {
        <dyn DropdownList>::scroll_bar_x_body
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled) (dispatch: the body for this class)
    fn mouse_clicked(&self) -> fn(&compat::Shared<dyn DropdownList>, &MouseButtonEvent, bool) -> bool {
        <dyn DropdownList>::mouse_clicked_body
    }

    /// java: @Override public boolean mouseReleased(MouseButtonEvent click) (dispatch: the body for this class)
    fn mouse_released(&self) -> fn(&compat::Shared<dyn DropdownList>, &MouseButtonEvent) -> bool {
        <dyn DropdownList>::mouse_released_body
    }

    /// java: @Override public boolean mouseDragged(MouseButtonEvent click, double offsetX, double offsetY) (dispatch: the body for this class)
    fn mouse_dragged(&self) -> fn(&compat::Shared<dyn DropdownList>, &MouseButtonEvent, f64, f64) -> bool {
        <dyn DropdownList>::mouse_dragged_body
    }

    /// java: @Override public boolean mouseScrolled(double mouseX, double mouseY, double horizontalAmount, double verticalAmount) (dispatch: the body for this class)
    fn mouse_scrolled(&self) -> fn(&compat::Shared<dyn DropdownList>, f64, f64, f64, f64) -> bool {
        <dyn DropdownList>::mouse_scrolled_body
    }

    /// java: @Override protected void extractListSeparators(GuiGraphicsExtractor graphics) (dispatch: the body for this class)
    fn extract_list_separators(&self) -> fn(&compat::Shared<dyn DropdownList>, &GuiGraphicsExtractor) {
        <dyn DropdownList>::extract_list_separators_body
    }

    /// java: @Override protected void extractListBackground(GuiGraphicsExtractor graphics) (dispatch: the body for this class)
    fn extract_list_background(&self) -> fn(&compat::Shared<dyn DropdownList>, &mut GuiGraphicsExtractor) {
        <dyn DropdownList>::extract_list_background_body
    }

    /// java: @Override protected void enableScissor(GuiGraphicsExtractor graphics) (dispatch: the body for this class)
    fn enable_scissor(&self) -> fn(&compat::Shared<dyn DropdownList>, &GuiGraphicsExtractor) {
        <dyn DropdownList>::enable_scissor_body
    }

}

impl dyn DropdownList {
    /// java: @Override protected int addEntry(DropdownWidget<T>.Entry entry)
    pub fn add_entry_body(__this: &compat::Shared<dyn DropdownList>, entry: &(_, _)) -> i32 {
        let __entry = entry.clone();
        return compat::write(__this, |__t| { let __l = &mut __t.dropdown_list_base_mut().base; (__l.add_entry(Box::new(compat::EntryAdapter::new(__entry))) as i32) });
    }

    /// java: @Override public int getRowLeft()
    pub fn get_row_left_body(__this: &compat::Shared<dyn DropdownList>) -> i32 {
        return (__this.lock().borrow().dropdown_list_base().get_x()).wrapping_add(2);
    }

    /// java: @Override public int getRowWidth()
    pub fn get_row_width_body(__this: &compat::Shared<dyn DropdownList>) -> i32 {
        return (__this.lock().borrow().dropdown_list_base().get_width()).wrapping_sub(5);
    }

    /// java: @Override protected void extractScrollbar(GuiGraphicsExtractor graphics, int mouseX, int mouseY)
    pub fn extract_scrollbar_body(__this: &compat::Shared<dyn DropdownList>, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32) {
        if __this.lock().borrow().dropdown_list_base().scrollable() {
            let mut i: i32 = ({ let __f = compat::dispatch(__this, |__t| __t.scroll_bar_x()); __f(__this) });
            let mut j: i32 = __this.lock().borrow().dropdown_list_base().scroller_height();
            let mut k: i32 = __this.lock().borrow().dropdown_list_base().scroll_bar_y();
            graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), i, k.wrapping_add(1), i.wrapping_add(2), k.wrapping_add(j), -1);
        }
    }

    /// java: @Override protected int scrollBarX()
    pub fn scroll_bar_x_body(__this: &compat::Shared<dyn DropdownList>) -> i32 {
        return ({ let __f = compat::dispatch(__this, |__t| __t.get_row_left()); __f(__this) }).wrapping_add({ let __f = compat::dispatch(__this, |__t| __t.get_row_width()); __f(__this) });
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    pub fn mouse_clicked_body(__this: &compat::Shared<dyn DropdownList>, click: &MouseButtonEvent, mut doubled: bool) -> bool {
        // TODO(fabric2seer): `AbstractContainerWidget.visible` is a field of the port's `AbstractContainerWidget`, which the port's `AbstractSelectionList` does not hold (no Deref, no `base` field)
        if !(compat::read(__this, |__t| __t.dropdown_list_base().visible)) {
            return false;
        }
        // TODO(fabric2seer): `AbstractContainerWidget::mouse_clicked` ports `mouseClicked`, but the port's `AbstractSelectionList` holds no `AbstractContainerWidget` (no Deref, no `base` field) to call it on
        // rust: return { let (__q0, __q1,) = (click.clone(), doubled,); __this.lock().borrow_mut().dropdown_list_base_mut().base.list.mouse_clicked(__q0, __q1) };
        return todo!("fabric2seer: see the TODO above");
    }

    /// java: @Override public boolean mouseReleased(MouseButtonEvent click)
    pub fn mouse_released_body(__this: &compat::Shared<dyn DropdownList>, click: &MouseButtonEvent) -> bool {
        // TODO(fabric2seer): `AbstractContainerWidget.visible` is a field of the port's `AbstractContainerWidget`, which the port's `AbstractSelectionList` does not hold (no Deref, no `base` field)
        if !(compat::read(__this, |__t| __t.dropdown_list_base().visible)) {
            return false;
        }
        // TODO(fabric2seer): `mouse_released` takes 3 argument(s) in the port (`_button: i32, _x: f64, _y: f64`), Java passes 1
        // TODO(fabric2seer): `mouse_released` is a method of the port's trait `GuiEventListener`, which the port's `AbstractSelectionList` does not implement
        // rust: return { let (__q0,) = (click.clone(),); __this.lock().borrow_mut().dropdown_list_base_mut().base.list.mouse_released(__q0) };
        return todo!("fabric2seer: see the TODO above");
    }

    /// java: @Override public boolean mouseDragged(MouseButtonEvent click, double offsetX, double offsetY)
    pub fn mouse_dragged_body(__this: &compat::Shared<dyn DropdownList>, click: &MouseButtonEvent, mut offset_x: f64, mut offset_y: f64) -> bool {
        // TODO(fabric2seer): `AbstractContainerWidget.visible` is a field of the port's `AbstractContainerWidget`, which the port's `AbstractSelectionList` does not hold (no Deref, no `base` field)
        if !(compat::read(__this, |__t| __t.dropdown_list_base().visible)) {
            return false;
        }
        // TODO(fabric2seer): `mouse_dragged` takes 2 argument(s) in the port (`event_y: f64, dy: f64`), Java passes 3
        // rust: return { let (__q0, __q1, __q2,) = (click.clone(), offset_x, offset_y,); __this.lock().borrow_mut().dropdown_list_base_mut().base.list.mouse_dragged(__q0, __q1, __q2) };
        return todo!("fabric2seer: see the TODO above");
    }

    /// java: @Override public boolean mouseScrolled(double mouseX, double mouseY, double horizontalAmount, double verticalAmount)
    pub fn mouse_scrolled_body(__this: &compat::Shared<dyn DropdownList>, mut mouse_x: f64, mut mouse_y: f64, mut horizontal_amount: f64, mut vertical_amount: f64) -> bool {
        // TODO(fabric2seer): `AbstractContainerWidget.visible` is a field of the port's `AbstractContainerWidget`, which the port's `AbstractSelectionList` does not hold (no Deref, no `base` field)
        if !(compat::read(__this, |__t| __t.dropdown_list_base().visible)) {
            return false;
        }
        // TODO(fabric2seer): `mouse_scrolled` takes 1 argument(s) in the port (`scroll_y: f64`), Java passes 4
        // rust: return __this.lock().borrow_mut().dropdown_list_base_mut().base.list.mouse_scrolled(mouse_x, mouse_y, horizontal_amount, vertical_amount);
        return todo!("fabric2seer: see the TODO above");
    }

    /// java: @Override protected void extractListSeparators(GuiGraphicsExtractor graphics)
    pub fn extract_list_separators_body(__this: &compat::Shared<dyn DropdownList>, graphics: &GuiGraphicsExtractor) {
    }

    /// java: @Override protected void extractListBackground(GuiGraphicsExtractor graphics)
    pub fn extract_list_background_body(__this: &compat::Shared<dyn DropdownList>, graphics: &mut GuiGraphicsExtractor) {
        graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), __this.lock().borrow().dropdown_list_base().get_x(), __this.lock().borrow().dropdown_list_base().get_y(), __this.lock().borrow().dropdown_list_base().get_right(), __this.lock().borrow().dropdown_list_base().get_bottom(), (0xFF).wrapping_shl((24) as u32));
        GuiHelper::border(graphics, __this.lock().borrow().dropdown_list_base().get_x(), __this.lock().borrow().dropdown_list_base().get_y(), __this.lock().borrow().dropdown_list_base().get_width(), __this.lock().borrow().dropdown_list_base().get_height(), -1);
    }

    /// java: @Override protected void enableScissor(GuiGraphicsExtractor graphics)
    pub fn enable_scissor_body(__this: &compat::Shared<dyn DropdownList>, graphics: &GuiGraphicsExtractor) {
        graphics.enable_scissor(__this.lock().borrow().dropdown_list_base().get_x(), (__this.lock().borrow().dropdown_list_base().get_y()).wrapping_add(1), __this.lock().borrow().dropdown_list_base().get_right(), (__this.lock().borrow().dropdown_list_base().get_bottom()).wrapping_sub(1));
    }

}

impl DropdownList for DropdownListBase {
    fn dropdown_list_base(&self) -> &DropdownListBase {
        &self
    }
    fn dropdown_list_base_mut(&mut self) -> &mut DropdownListBase {
        &mut self
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}

/// java: protected class Entry extends ContainerObjectSelectionList.Entry<Entry>
pub struct EntryBase {
    /// java: extends net.minecraft.client.gui.components.ContainerObjectSelectionList$Entry
    pub base: mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry,
    /// java: protected final T entry;
    pub entry: T,
    /// the row's widgets (`children()`): lent to the object by its `compat::EntryAdapter`
    pub __row: mcrust::net::minecraft::client::gui::screens::screen::Screen,
    /// the screen's widgets as this object last saw them (see `compat::WidgetCache`)
    pub __widgets: compat::WidgetCache,
}

impl std::ops::Deref for EntryBase {
    type Target = mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for EntryBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl EntryBase {
    /// java: protected Entry(T element) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_in<__R>(mut element: T, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn Entry>, __R)) -> (compat::Shared<dyn Entry>, __R) {
        let __init_entry = element.clone();
        let this = Self {
            base: mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry::new(0),
            __widgets: Default::default(),
            __row: mcrust::net::minecraft::client::gui::screens::screen::Screen::new(String::new()),
            entry: __init_entry,
        };
        let (__h, __r) = __wrap(this);
        let __this = __h;
        (__this, __r)
    }

    /// java: protected Entry(T element)
    pub fn new(mut element: T) -> compat::Shared<Self> {
        Self::new_in(element, |this| {
            let __h = compat::shared(this);
            ((__h.clone() as compat::Shared<dyn Entry>), __h)
        }).1
    }

}

/// java: the virtual methods of Entry (implemented by EntryBase and every subclass)
pub trait Entry: compat::DynAny {
    fn entry_base(&self) -> &EntryBase;
    fn entry_base_mut(&mut self) -> &mut EntryBase;
    fn as_any(&self) -> &dyn std::any::Any;

    /// java: @Override public List<? extends NarratableEntry> narratables() (dispatch: the body for this class)
    fn narratables(&self) -> fn(&compat::Shared<dyn Entry>) -> Vec<Box<dyn NarratableEntry>> {
        <dyn Entry>::narratables_body
    }

    /// java: @Override public List<? extends GuiEventListener> children() (dispatch: the body for this class)
    fn children(&self) -> fn(&compat::Shared<dyn Entry>) -> Vec<Box<dyn GuiEventListener>> {
        <dyn Entry>::children_body
    }

    /// java: @Override public void extractContent(GuiGraphicsExtractor graphics, int mouseX, int mouseY, boolean hovered, float deltaTicks) (dispatch: the body for this class)
    fn extract_content(&self) -> fn(&compat::Shared<dyn Entry>, &GuiGraphicsExtractor, i32, i32, bool, f32) {
        <dyn Entry>::extract_content_body
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled) (dispatch: the body for this class)
    fn mouse_clicked(&self) -> fn(&compat::Shared<dyn Entry>, &MouseButtonEvent, bool) -> bool {
        <dyn Entry>::mouse_clicked_body
    }

}

impl dyn Entry {
    /// java: @Override public List<? extends NarratableEntry> narratables()
    pub fn narratables_body(__this: &compat::Shared<dyn Entry>) -> Vec<Box<dyn NarratableEntry>> {
        return vec![];
    }

    /// java: @Override public List<? extends GuiEventListener> children()
    pub fn children_body(__this: &compat::Shared<dyn Entry>) -> Vec<Box<dyn GuiEventListener>> {
        return vec![];
    }

    /// java: @Override public void extractContent(GuiGraphicsExtractor graphics, int mouseX, int mouseY, boolean hovered, float deltaTicks)
    pub fn extract_content_body(__this: &compat::Shared<dyn Entry>, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut hovered: bool, mut delta_ticks: f32) {
        // TODO(fabric2seer): field `NONE` of a value of unknown type
        //   java: graphics.textRenderer(GuiGraphicsExtractor.HoveredTextEffects.NONE).acceptScrollingWithDefaultCenter(Component.literal(entry.toString()).withStyle(Style.EMPTY.withUnderlined(hovered)), this.getX() + 10, this.getX() + this.getWidth(), this.getY(), this.getY() + 11);
        // TODO(fabric2seer): the condition: `selected` is a field of the enclosing instance (DropdownWidget.this) - an inner class has no outer reference in Rust
        //   java: selected == this.entry
        if todo!("fabric2seer: the condition (TODO above)") {
            // TODO(fabric2seer): `client` is a field of the enclosing instance (DropdownWidget.this) - an inner class has no outer reference in Rust
            //   java: graphics.text(client.font, "✔", this.getX() + 1, this.getY() + 2, 0xFFFFFFFF);
        }
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    pub fn mouse_clicked_body(__this: &compat::Shared<dyn Entry>, click: &MouseButtonEvent, mut doubled: bool) -> bool {
        // TODO(fabric2seer): `select(..)` is a method of the enclosing instance (DropdownWidget.this) - an inner class has no outer reference in Rust
        //   java: select(entry);
        return true;
    }

}

impl Entry for EntryBase {
    fn entry_base(&self) -> &EntryBase {
        &self
    }
    fn entry_base_mut(&mut self) -> &mut EntryBase {
        &mut self
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}

/// Entry's row box and its widgets, lent by its `compat::EntryAdapter` for each call
impl compat::RowHalf for dyn Entry {
    fn entry_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry {
        &mut self.entry_base_mut().base
    }
    fn row_parts(&mut self) -> (&mut mcrust::net::minecraft::client::gui::screens::screen::Screen, &mut compat::WidgetCache) {
        let __s = &mut self.entry_base_mut();
        (&mut __s.__row, &mut __s.__widgets)
    }
}

/// Entry as a row of the game's list (`addEntry` attaches it)
impl ModListEntry for compat::EntryAdapter<dyn Entry> {
    /// java: @Override public void extractContent(GuiGraphicsExtractor graphics, int mouseX, int mouseY, boolean hovered, float deltaTicks)
    fn extract_content(&mut self, list: &mut ModSelectionList, index: usize, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, hovered: bool, a: f32) {
        self.lend(Some(list.list.entry(index)), |__h| { if let Some(__c) = compat::try_view::<CustomEntry, _>(__h) { return ({ let __f = compat::dispatch(&*__c, |__t| __t.extract_content()); __f(&compat::view(&*__c, |a| a as compat::Shared<dyn Entry>), &context.graphics, mouse_x, mouse_y, hovered, a) }); }  });
        // the row's widgets (Java's `child.extractRenderState(..)` over `children()`)
        context.extract_widgets(&mut self.row, mouse_x, mouse_y);
    }

    fn children(&mut self) -> Option<&mut mcrust::net::minecraft::client::gui::screens::screen::Screen> {
        Some(&mut self.row)
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    fn mouse_clicked(&mut self, list: &mut ModSelectionList, index: usize, event: &MouseButtonEvent, double_click: bool) -> bool {
        self.lend(Some(list.list.entry(index)), |__h| { if let Some(__c) = compat::try_view::<CustomEntry, _>(__h) { return ({ let __f = compat::dispatch(&*__c, |__t| __t.mouse_clicked()); __f(&compat::view(&*__c, |a| a as compat::Shared<dyn Entry>), event, double_click) }); } false })
    }

}

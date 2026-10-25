//! Ported from de.hysky.skyblocker.utils.render.gui.DropdownWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::render::gui_helper::GuiHelper;
use mcrust::com::mojang::blaze3d::platform::cursor::cursor_types::CursorTypes;
use mcrust::mcrust::modding::widgets::ModListEntry;
use mcrust::mcrust::modding::widgets::ModList;
use mcrust::mcrust::modding::widgets::ModWidget as _;
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

/// `DropdownWidget.this` as its inner class `Entry` reads it (`selected == this.entry`, `select(entry)`,
/// `formatter.apply(entry)`, `entry.toString()`): the widget's share of its own state that a row reaches through
/// its `Weak` handle (`EntryBase::outer`). A row's `select(entry)` runs inside the widget's own `mouseClicked`
/// (the widget is borrowed for that call), so the row parks the entry here and `mouseClicked` applies it as soon
/// as the child pass returns - nothing runs between the two in Java's nested call either.
#[derive(Default)]
pub struct DropdownLink {
    /// `selected.toString()` (the element type is erased in the rows; Java's `==` on the shared element reference is
    /// the element's identity, which for the enum constants every dropdown of this mod holds is its name)
    pub selected_text: std::cell::RefCell<Option<String>>,
    /// `select(entry)` asked by a row, waiting for the widget's `mouseClicked` to return to it
    pub pending_select: std::cell::RefCell<Option<std::rc::Rc<dyn std::any::Any>>>,
    /// `entry.toString()` for the erased `T`
    pub to_text: std::cell::RefCell<Option<std::rc::Rc<dyn Fn(&dyn std::any::Any) -> String>>>,
    /// `formatter.apply(entry)` for the erased `T` (`None`: not a `T`)
    pub format: std::cell::RefCell<Option<std::rc::Rc<dyn Fn(&dyn std::any::Any) -> Option<MutableComponent>>>>,
}

impl DropdownLink {
    /// `entry.toString()`
    pub fn text_of(&self, entry: &dyn std::any::Any) -> String {
        let f = self.to_text.borrow().clone();
        f.map(|f| f(entry)).unwrap_or_default()
    }

    /// `selected == entry`
    pub fn is_selected(&self, entry: &dyn std::any::Any) -> bool {
        let text = self.text_of(entry);
        self.selected_text.borrow().as_deref() == Some(text.as_str())
    }

    /// `formatter.apply(entry)`
    pub fn format_entry(&self, entry: &dyn std::any::Any) -> Option<MutableComponent> {
        let f = self.format.borrow().clone();
        f.and_then(|f| f(entry))
    }
}

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
    /// what the rows read of `DropdownWidget.this` (see `DropdownLink`)
    pub link: std::rc::Rc<DropdownLink>,
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
    pub fn new(minecraft_client: &mut mcrust::net::minecraft::client::minecraft::Minecraft, mut x: i32, mut y: i32, mut width: i32, mut max_height: i32, mut entry_height: i32, mut entries: Vec<T>, select_callback: Arc<dyn Fn(T)>, mut selected: T, opened_callback: Arc<dyn Fn(bool)>) -> Self where T: std::fmt::Display {
        let _ = minecraft_client;
        let mut this: Self = Self::new_base(x, y, width, max_height, entry_height, entries, select_callback, selected, opened_callback);
        this.init_dropdown();
        this
    }

    /// The constructor's field half: `super(x, y, width, 0, Component.empty(), defaultSettings(4))` and the field
    /// writes, before `dropdownList = createDropdown();`. A subclass builds its value around this base and then runs
    /// `init_dropdown` on ITSELF, so that `createDropdown()` / `createEntry(..)` dispatch to its overrides as Java's
    /// constructor does.
    pub fn new_base(mut x: i32, mut y: i32, mut width: i32, mut max_height: i32, mut entry_height: i32, mut entries: Vec<T>, select_callback: Arc<dyn Fn(T)>, mut selected: T, opened_callback: Arc<dyn Fn(bool)>) -> Self where T: std::fmt::Display {
        // java: protected Function<T, Component> formatter = t -> Component.literal(t.toString());
        let formatter: Arc<dyn Fn(T) -> MutableComponent> = Arc::new(|t: T| component::literal(&compat::jstring(&t)));
        let link = std::rc::Rc::new(DropdownLink::default());
        *link.to_text.borrow_mut() = Some(std::rc::Rc::new(|a: &dyn std::any::Any| a.downcast_ref::<T>().map(|t| compat::jstring(t)).unwrap_or_default()));
        *link.selected_text.borrow_mut() = Some(compat::jstring(&selected));
        {
            let f = formatter.clone();
            *link.format.borrow_mut() = Some(std::rc::Rc::new(move |a: &dyn std::any::Any| a.downcast_ref::<T>().map(|t| f(t.clone()))));
        }
        Self {
            base: AbstractContainerWidget::new_with_scrollbar_settings(x, y, width, 0, mcrust::net::minecraft::network::chat::formatted_text::FormattedText::Empty, mcrust::net::minecraft::client::gui::components::abstract_scroll_area::ScrollbarSettings::default_settings(4.0)),
            entry_height,
            header_height: entry_height.wrapping_add(4),
            entries,
            select_callback: Some(select_callback),
            opened_callback: Some(opened_callback),
            dropdown_list: None,
            prev_selected: None,
            selected: Some(selected),
            open: false,
            max_height,
            formatter,
            link,
        }
    }

    /// java: public DropdownWidget(Minecraft minecraftClient, int x, int y, int width, int maxHeight, List<T> entries, Consumer<T> selectCallback, T selected, Consumer<Boolean> openedCallback)
    pub fn new_9(minecraft_client: &mut mcrust::net::minecraft::client::minecraft::Minecraft, mut x: i32, mut y: i32, mut width: i32, mut max_height: i32, mut entries: Vec<T>, select_callback: Arc<dyn Fn(T)>, mut selected: T, opened_callback: Arc<dyn Fn(bool)>) -> Self where T: std::fmt::Display {
        let mut this = Self::new(minecraft_client, x, y, width, max_height, 15, entries.clone(), select_callback.clone(), selected.clone(), opened_callback.clone());
        this
    }

}

/// java: the virtual methods of DropdownWidget (implemented by DropdownWidgetBase and every subclass)
pub trait DropdownWidget<T: Clone + 'static> {
    fn dropdown_widget_base(&self) -> &DropdownWidgetBase<T>;
    fn dropdown_widget_base_mut(&mut self) -> &mut DropdownWidgetBase<T>;
    fn as_any(&self) -> &dyn std::any::Any;

    /// The constructor's second half (`DropdownWidget(..)`, after the field writes):
    /// `dropdownList = createDropdown(); dropdownList.setRectangle(width - 2, maxHeight - headerHeight, x + 1, y + headerHeight);
    /// for (T element : entries) dropdownList.addEntry(createEntry(element)); setHeight(headerHeight);`
    fn init_dropdown(&mut self) {
        let list = self.create_dropdown();
        self.dropdown_widget_base_mut().dropdown_list = Some(list.clone());
        let (width, max_height, header_height, x, y) = { let b = self.dropdown_widget_base(); (b.get_width(), b.max_height, b.header_height, b.get_x(), b.get_y()) };
        compat::write(&list, |__t| __t.dropdown_list_base_mut().set_rectangle(width.wrapping_sub(2), max_height.wrapping_sub(header_height), x.wrapping_add(1), y.wrapping_add(header_height)));
        for element in self.dropdown_widget_base().entries.clone().iter() {
            let entry = self.create_entry(element);
            // the row's `DropdownWidget.this`
            let outer = std::rc::Rc::downgrade(&self.dropdown_widget_base().link);
            compat::write(&entry, |__t| __t.entry_base_mut().outer = outer);
            ({ let __f = compat::dispatch(&list, |__t| __t.add_entry()); __f(&list, &entry) });
        }
        self.set_height(header_height);
    }

    /// java: public int getHeaderHeight()
    fn get_header_height(&self) -> i32 {
        return self.dropdown_widget_base().header_height;
    }

    /// java: protected DropdownList createDropdown()
    fn create_dropdown(&self) -> compat::Shared<dyn DropdownList> {
        return (DropdownListBase::new(compat::minecraft(), self.dropdown_widget_base().entry_height) as compat::Shared<dyn DropdownList>);
    }

    /// java: protected Entry createEntry(T element)
    fn create_entry(&self, element: &T) -> compat::Shared<dyn Entry> {
        return (EntryBase::new(element.clone()) as compat::Shared<dyn Entry>);
    }

    /// java: public void setFormatter(Function<T, Component> formatter)
    fn set_formatter(&mut self, formatter: Arc<dyn Fn(T) -> MutableComponent>) {
        self.dropdown_widget_base_mut().formatter = formatter.clone();
        // (the rows' view of the field)
        let f = formatter.clone();
        *self.dropdown_widget_base().link.format.borrow_mut() = Some(std::rc::Rc::new(move |a: &dyn std::any::Any| a.downcast_ref::<T>().map(|t| f(t.clone()))));
    }

    /// java: public void setMaxHeight(int maxHeight)
    fn set_max_height(&mut self, mut max_height: i32) {
        self.dropdown_widget_base_mut().max_height = max_height;
        { let __s0 = self.dropdown_widget_base().open; self.set_open(__s0) };
    }

    /// java: @Override public List<? extends GuiEventListener> children()
    ///
    /// (the one child is the list OBJECT, by reference - the handle every event of this class reaches it through)
    fn children(&self) -> Vec<compat::Shared<dyn DropdownList>> {
        // java: return List.of(dropdownList);
        return vec![self.dropdown_widget_base().dropdown_list.clone().unwrap()];
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float delta)
    fn extract_widget_render_state(&mut self, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut delta: f32) where T: std::fmt::Display {
        // java: dropdownList.visible = open; - `ModSelectionList.visible` is the list's own field
        { let __v = self.dropdown_widget_base().open; compat::write(self.dropdown_widget_base().dropdown_list.as_ref().unwrap(), |__t| __t.dropdown_list_base_mut().base.visible = __v); }
        // java: dropdownList.extractRenderState(graphics, mouseX, mouseY, delta);
        let __list = self.dropdown_widget_base().dropdown_list.clone().unwrap();
        <dyn DropdownList>::extract_render_state(&__list, graphics, mouse_x, mouse_y, delta);
        self.extract_header(graphics, mouse_x, mouse_y, delta);
        if self.dropdown_widget_base().is_mouse_over((mouse_x) as f64, (mouse_y) as f64) {
            // java: graphics.requestCursor(CursorTypes.POINTING_HAND);
            compat::batch_w_request_cursor(compat::BatchWCursor::PointingHand);
        }
    }

    /// java: protected void extractHeader(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float delta)
    ///
    /// (`selected.toString()` is `Display`)
    fn extract_header(&self, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut delta: f32) where T: std::fmt::Display {
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
    ///
    /// (`selected != prevSelected` compares the two references; the port's entries are values, compared by `==`)
    fn select(&mut self, mut entry: T) where T: PartialEq {
        self.dropdown_widget_base_mut().selected = Some(entry.clone());
        let __text = self.dropdown_widget_base().link.text_of(&entry);
        *self.dropdown_widget_base().link.selected_text.borrow_mut() = Some(__text);
        self.set_open(false);
        if self.dropdown_widget_base().selected != self.dropdown_widget_base().prev_selected {
            let __callback = self.dropdown_widget_base().select_callback.clone().unwrap();
            (__callback)(entry);
            let __selected = self.dropdown_widget_base().selected.clone();
            self.dropdown_widget_base_mut().prev_selected = __selected;
        }
    }

    /// java: protected void select(T entry), as a row asks it (`Entry.mouseClicked`: `select(entry)` with the row's own
    /// element). `selected != prevSelected` is Java's reference comparison of two elements of `entries`, read through
    /// the rows' erased view (`DropdownLink::text_of`), so no `T: PartialEq` is needed on the click path.
    fn select_entry(&mut self, entry: T) {
        let link = self.dropdown_widget_base().link.clone();
        self.dropdown_widget_base_mut().selected = Some(entry.clone());
        *link.selected_text.borrow_mut() = Some(link.text_of(&entry));
        self.set_open(false);
        let prev = self.dropdown_widget_base().prev_selected.as_ref().map(|p| link.text_of(p));
        if prev.as_deref() != Some(link.text_of(&entry).as_str()) {
            let __callback = self.dropdown_widget_base().select_callback.clone().unwrap();
            (__callback)(entry);
            let __selected = self.dropdown_widget_base().selected.clone();
            self.dropdown_widget_base_mut().prev_selected = __selected;
        }
    }

    /// java: @Override public void setX(int x)
    fn set_x(&mut self, mut x: i32) {
        // java: super.setX(x);
        self.dropdown_widget_base_mut().base.set_x(x);
        let __wa1 = (self.dropdown_widget_base().get_x()).wrapping_add(1);
        let __o1 = self.dropdown_widget_base().dropdown_list.clone().unwrap();
        compat::write(&__o1, |__t| __t.dropdown_list_base_mut().set_x(__wa1));
        let __o2 = self.dropdown_widget_base().dropdown_list.clone().unwrap();
        compat::write(&__o2, |__t| __t.dropdown_list_base_mut().refresh_scroll_amount());
    }

    /// java: @Override public void setY(int y)
    fn set_y(&mut self, mut y: i32) {
        // java: super.setY(y);
        self.dropdown_widget_base_mut().base.set_y(y);
        let __wa1 = (self.dropdown_widget_base().get_y()).wrapping_add(self.dropdown_widget_base().header_height);
        let __o1 = self.dropdown_widget_base().dropdown_list.clone().unwrap();
        compat::write(&__o1, |__t| __t.dropdown_list_base_mut().set_y(__wa1));
    }

    /// java: @Override public void setWidth(int width)
    fn set_width(&mut self, mut width: i32) {
        // java: super.setWidth(width);
        self.dropdown_widget_base_mut().base.set_width(width);
        let __wa1 = (self.dropdown_widget_base().get_width()).wrapping_sub(2);
        let __o1 = self.dropdown_widget_base().dropdown_list.clone().unwrap();
        compat::write(&__o1, |__t| __t.dropdown_list_base_mut().set_width(__wa1));
    }

    /// java: @Override public void setHeight(int height)
    fn set_height(&mut self, mut height: i32) {
        // java: super.setHeight(height);
        self.dropdown_widget_base_mut().base.set_height(height);
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    fn mouse_clicked(&mut self, click: &MouseButtonEvent, mut doubled: bool) -> bool {
        if !(self.dropdown_widget_base().visible) {
            return false;
        }
        if (self.dropdown_widget_base().get_x() as f64) <= click.x() && click.x() < ((self.dropdown_widget_base().get_x()).wrapping_add(self.dropdown_widget_base().get_width()) as f64) && (self.dropdown_widget_base().get_y() as f64) <= click.y() && click.y() < ((self.dropdown_widget_base().get_y()).wrapping_add(self.dropdown_widget_base().header_height) as f64) {
            { let __s0 = !(self.dropdown_widget_base().open); self.set_open(__s0) };
            // java: playDownSound(Minecraft.getInstance().getSoundManager());
            compat::batch_w_play_down_sound();
            return true;
        }
        // java: return super.mouseClicked(click, doubled); - AbstractContainerWidget:62-66: the scrollbar pass
        // (`updateScrolling`: contentHeight() == getHeight(), so no scrollbar is shown and none is armed) then
        // ContainerEventHandler's children pass over `children()` = `List.of(dropdownList)`
        let dropdown_list = self.dropdown_widget_base().dropdown_list.clone();
        let __answer = self.dropdown_widget_base_mut().base.mouse_clicked(
            || false,
            || match &dropdown_list {
                Some(list) => {
                    let __f = compat::dispatch(list, |__t| __t.mouse_clicked());
                    __f(list, click, doubled)
                }
                None => false,
            },
        ).0;
        // a row's `select(entry)` (Entry.mouseClicked), made inside the child pass above
        let __pending = self.dropdown_widget_base().link.pending_select.borrow_mut().take();
        if let Some(__entry) = __pending.and_then(|e| e.downcast_ref::<T>().cloned()) {
            self.select_entry(__entry);
        }
        return __answer;
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
        // java: if (this.getChildAt(mouseX, mouseY).filter(c -> c.mouseScrolled(mouseX, mouseY, horizontalAmount, verticalAmount)).isPresent()) return true;
        // (the one child: `dropdownList`, found when the mouse is over it)
        if let Some(list) = self.dropdown_widget_base().dropdown_list.clone() {
            let over = list.lock().borrow().dropdown_list_base().base.list.is_mouse_over(mouse_x, mouse_y);
            if over {
                let __f = compat::dispatch(&list, |__t| __t.mouse_scrolled());
                if __f(&list, mouse_x, mouse_y, horizontal_amount, vertical_amount) {
                    return true;
                }
            }
        }
        // java: return super.mouseScrolled(..); - AbstractScrollArea.mouseScrolled: visible (checked above), then
        // `setScrollAmount(scrollAmount() - verticalAmount * scrollRate())` - `scrollRate()` is 0 here, so the amount is
        // unchanged - and `true`
        return true;
    }

}

impl<T: Clone + 'static> DropdownWidget<T> for DropdownWidgetBase<T> {
    fn dropdown_widget_base(&self) -> &DropdownWidgetBase<T> {
        self
    }
    fn dropdown_widget_base_mut(&mut self) -> &mut DropdownWidgetBase<T> {
        self
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
    ///
    /// (`entry_height` is the enclosing `DropdownWidget.this.entryHeight`, which the inner class reads)
    pub fn new_in<__R>(minecraft_client: &mut mcrust::net::minecraft::client::minecraft::Minecraft, entry_height: i32, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn DropdownList>, __R)) -> (compat::Shared<dyn DropdownList>, __R) {
        //   java: super(minecraftClient, 0, 0, 0, entryHeight); - `ContainerObjectSelectionList(minecraft, width, height, y, itemHeight)`
        let _ = minecraft_client;
        let this = Self { base: ModSelectionList::new(mcrust::mcrust::modding::widgets::ListKind::Container, 0, 0, 0, entry_height) };
        let (__h, __r) = __wrap(this);
        // the class's overrides of the list's own methods, asked by the port's list bodies
        let __class = DropdownListClass { object: compat::RenderThread::new(std::sync::Arc::downgrade(&__h)) };
        compat::write(&__h, |__t| __t.dropdown_list_base_mut().base.list.set_class(Some(mcrust::mcrust::modding::widgets::ModListClass(Arc::new(__class)))));
        (__h, __r)
    }

    /// java: protected DropdownList(Minecraft minecraftClient)
    pub fn new(minecraft_client: &mut mcrust::net::minecraft::client::minecraft::Minecraft, entry_height: i32) -> compat::Shared<Self> {
        Self::new_in(minecraft_client, entry_height, |this| {
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
    fn add_entry(&self) -> fn(&compat::Shared<dyn DropdownList>, &compat::Shared<dyn Entry>) -> i32 {
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
    pub fn add_entry_body(__this: &compat::Shared<dyn DropdownList>, entry: &compat::Shared<dyn Entry>) -> i32 {
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
        // java: if (!visible) return false;
        // (`ModSelectionList.visible` is the list's own `visible`, synced from its widget)
        let __visible: bool = __this.lock().borrow().dropdown_list_base().base.visible;
        if !(__visible) {
            return false;
        }
        // java: return super.mouseClicked(click, doubled);
        // (`ModSelectionList::mouse_clicked_base` is the list's `AbstractContainerWidget.mouseClicked`)
        let __g = __this.lock();
        let mut __t = __g.borrow_mut();
        return __t.dropdown_list_base_mut().base.mouse_clicked_base(click, doubled);
    }

    /// java: @Override public boolean mouseReleased(MouseButtonEvent click)
    pub fn mouse_released_body(__this: &compat::Shared<dyn DropdownList>, click: &MouseButtonEvent) -> bool {
        // java: if (!visible) return false;
        // (`ModSelectionList.visible` is the list's own `visible`, synced from its widget)
        let __visible: bool = __this.lock().borrow().dropdown_list_base().base.visible;
        if !(__visible) {
            return false;
        }
        // java: return super.mouseReleased(click);
        // (`ModSelectionList::mouse_released_base`)
        let __g = __this.lock();
        let mut __t = __g.borrow_mut();
        return __t.dropdown_list_base_mut().base.mouse_released_base(click);
    }

    /// java: @Override public boolean mouseDragged(MouseButtonEvent click, double offsetX, double offsetY)
    pub fn mouse_dragged_body(__this: &compat::Shared<dyn DropdownList>, click: &MouseButtonEvent, mut offset_x: f64, mut offset_y: f64) -> bool {
        // java: if (!visible) return false;
        // (`ModSelectionList.visible` is the list's own `visible`, synced from its widget)
        let __visible: bool = __this.lock().borrow().dropdown_list_base().base.visible;
        if !(__visible) {
            return false;
        }
        // java: return super.mouseDragged(click, offsetX, offsetY);
        // (`ModSelectionList::mouse_dragged_base`)
        let __g = __this.lock();
        let mut __t = __g.borrow_mut();
        return __t.dropdown_list_base_mut().base.mouse_dragged_base(click, offset_x, offset_y);
    }

    /// java: @Override public boolean mouseScrolled(double mouseX, double mouseY, double horizontalAmount, double verticalAmount)
    pub fn mouse_scrolled_body(__this: &compat::Shared<dyn DropdownList>, mut mouse_x: f64, mut mouse_y: f64, mut horizontal_amount: f64, mut vertical_amount: f64) -> bool {
        // java: if (!visible) return false;
        // (`ModSelectionList.visible` is the list's own `visible`, synced from its widget)
        let __visible: bool = __this.lock().borrow().dropdown_list_base().base.visible;
        if !(__visible) {
            return false;
        }
        // java: return super.mouseScrolled(mouseX, mouseY, horizontalAmount, verticalAmount);
        // (`ModSelectionList::mouse_scrolled_base`)
        let _ = (mouse_x, mouse_y, horizontal_amount);
        let __g = __this.lock();
        let mut __t = __g.borrow_mut();
        return __t.dropdown_list_base_mut().base.mouse_scrolled_base(vertical_amount);
    }

    /// java: @Override protected void extractListSeparators(GuiGraphicsExtractor graphics)
    pub fn extract_list_separators_body(__this: &compat::Shared<dyn DropdownList>, graphics: &GuiGraphicsExtractor) {
    }

    /// java: @Override protected void extractListBackground(GuiGraphicsExtractor graphics)
    pub fn extract_list_background_body(__this: &compat::Shared<dyn DropdownList>, graphics: &mut GuiGraphicsExtractor) {
        graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), __this.lock().borrow().dropdown_list_base().get_x(), __this.lock().borrow().dropdown_list_base().get_y(), __this.lock().borrow().dropdown_list_base().get_right(), __this.lock().borrow().dropdown_list_base().get_bottom(), (0xFFi32).wrapping_shl((24) as u32));
        GuiHelper::border(graphics, __this.lock().borrow().dropdown_list_base().get_x(), __this.lock().borrow().dropdown_list_base().get_y(), __this.lock().borrow().dropdown_list_base().get_width(), __this.lock().borrow().dropdown_list_base().get_height(), -1);
    }

    /// java: @Override protected void enableScissor(GuiGraphicsExtractor graphics)
    pub fn enable_scissor_body(__this: &compat::Shared<dyn DropdownList>, graphics: &GuiGraphicsExtractor) {
        graphics.enable_scissor(__this.lock().borrow().dropdown_list_base().get_x(), (__this.lock().borrow().dropdown_list_base().get_y()).wrapping_add(1), __this.lock().borrow().dropdown_list_base().get_right(), (__this.lock().borrow().dropdown_list_base().get_bottom()).wrapping_sub(1));
    }

}

impl DropdownList for DropdownListBase {
    fn dropdown_list_base(&self) -> &DropdownListBase {
        self
    }
    fn dropdown_list_base_mut(&mut self) -> &mut DropdownListBase {
        self
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}

impl dyn DropdownList {
    /// `dropdownList.extractRenderState(graphics, mouseX, mouseY, delta)` - `AbstractWidget.extractRenderState:56-62`:
    /// `if (visible) extractWidgetRenderState(..)`, the list's `AbstractSelectionList.extractWidgetRenderState`. The
    /// live list is taken out of the object for the draw (the object keeps a copy of its box, so the class's draw
    /// overrides - asked by the draw through `DropdownListClass` - read `this.getX()` & co. off the object as Java's
    /// do) and put back after it.
    pub fn extract_render_state(__this: &compat::Shared<dyn DropdownList>, graphics: &mut GuiGraphicsExtractor, mouse_x: i32, mouse_y: i32, delta: f32) {
        if !__this.lock().borrow().dropdown_list_base().base.visible {
            return;
        }
        let mut __live = compat::write(__this, |__t| {
            let __b = &mut __t.dropdown_list_base_mut().base;
            let __kind = __b.kind;
            let __live = std::mem::replace(__b, ModSelectionList::new(__kind, 0, 0, 0, 0));
            let mut __view = __live.list.clone();
            __view.set_class(None);
            __b.list = __view;
            __b.visible = __live.visible;
            __b.active = __live.active;
            __live
        });
        let (x, y, w, h) = (__live.list.x, __live.list.y, __live.list.width, __live.list.height);
        // the list's `AbstractWidget` half (its box and `visible` / `active`, which the list body syncs in)
        let mut __widget = mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget::new(x, y, w, h, String::new(), mcrust::net::minecraft::client::gui::components::abstract_widget::WidgetKind::Button, mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress::Inert);
        __widget.visible = __live.visible;
        __widget.active = __live.active;
        let __minecraft = &*compat::minecraft();
        let mut __context = WidgetRenderContext::new(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), __minecraft);
        // the caller's extractor is the one drawn into
        std::mem::swap(&mut __context.graphics, graphics);
        __live.extract_widget_render_state(&__widget, &mut __context, mouse_x, mouse_y, delta);
        std::mem::swap(&mut __context.graphics, graphics);
        compat::write(__this, |__t| __t.dropdown_list_base_mut().base = __live);
    }
}

/// `DropdownList`'s overrides of the list's own methods, as the port's list bodies ask them (`widgets::ModList`).
/// The geometry overrides (`getRowLeft`, `getRowWidth`, `scrollBarX`) are asked from inside the list's own calls -
/// while the object is borrowed for a click - so they read the box the list hands them; the draw overrides are
/// asked only by `extract_render_state`, which has taken the live list out of the object, and dispatch to the
/// object's class (a subclass's `extractListBackground` runs, as Java's virtual call does).
struct DropdownListClass {
    object: compat::RenderThread<compat::WeakShared<dyn DropdownList>>,
}

impl ModList for DropdownListClass {
    /// java: @Override public int getRowLeft() - `getX() + 2`
    fn get_row_left(&self, list: &AbstractSelectionList, _super_answer: i32) -> i32 {
        list.x.wrapping_add(2)
    }

    /// java: @Override public int getRowWidth() - `getWidth() - 5`
    fn get_row_width(&self, list: &AbstractSelectionList, _super_answer: i32) -> i32 {
        list.width.wrapping_sub(5)
    }

    /// java: @Override protected int scrollBarX() - `getRowLeft() + getRowWidth()`
    fn scroll_bar_x(&self, list: &AbstractSelectionList, _super_answer: i32) -> i32 {
        list.get_row_left().wrapping_add(list.get_row_width())
    }

    /// java: @Override protected void extractScrollbar(GuiGraphicsExtractor graphics, int mouseX, int mouseY)
    fn extract_scrollbar(&self, list: &AbstractSelectionList, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, super_extract_scrollbar: &mut dyn FnMut(&mut WidgetRenderContext<'_>)) {
        let _ = (list, super_extract_scrollbar); // the override replaces the base's body
        if let Some(__h) = self.object.get().upgrade() {
            let __f = compat::dispatch(&__h, |__t| __t.extract_scrollbar());
            __f(&__h, &mut context.graphics, mouse_x, mouse_y);
        }
    }

    /// java: @Override protected void extractListSeparators(GuiGraphicsExtractor graphics)
    fn extract_list_separators(&self, list: &AbstractSelectionList, context: &mut WidgetRenderContext<'_>, super_extract_list_separators: &mut dyn FnMut(&mut WidgetRenderContext<'_>)) {
        let _ = (list, super_extract_list_separators); // the override replaces the base's body
        if let Some(__h) = self.object.get().upgrade() {
            let __f = compat::dispatch(&__h, |__t| __t.extract_list_separators());
            __f(&__h, &context.graphics);
        }
    }

    /// java: @Override protected void extractListBackground(GuiGraphicsExtractor graphics)
    fn extract_list_background(&self, list: &AbstractSelectionList, context: &mut WidgetRenderContext<'_>, super_extract_list_background: &mut dyn FnMut(&mut WidgetRenderContext<'_>)) {
        let _ = (list, super_extract_list_background); // the override replaces the base's body
        if let Some(__h) = self.object.get().upgrade() {
            let __f = compat::dispatch(&__h, |__t| __t.extract_list_background());
            __f(&__h, &mut context.graphics);
        }
    }

    /// java: @Override protected void enableScissor(GuiGraphicsExtractor graphics) -
    /// `graphics.enableScissor(getX(), getY() + 1, getRight(), getBottom() - 1)`
    fn enable_scissor(&self, list: &AbstractSelectionList, context: &mut WidgetRenderContext<'_>, super_enable_scissor: &mut dyn FnMut(&mut WidgetRenderContext<'_>)) {
        let _ = super_enable_scissor; // the override replaces the base's body
        context.graphics.enable_scissor(list.x, list.y.wrapping_add(1), list.get_right(), list.get_bottom().wrapping_sub(1));
    }
}

/// java: protected class Entry extends ContainerObjectSelectionList.Entry<Entry>
pub struct EntryBase {
    /// java: extends net.minecraft.client.gui.components.ContainerObjectSelectionList$Entry
    pub base: mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry,
    /// java: protected final T entry; (the inner class sees the enclosing `DropdownWidget<T>`'s `T`, erased as Java
    /// erases it: the element object, by reference)
    pub entry: std::rc::Rc<dyn std::any::Any>,
    /// the row's widgets (`children()`): lent to the object by its `compat::EntryAdapter`
    pub __row: mcrust::net::minecraft::client::gui::screens::screen::Screen,
    /// the screen's widgets as this object last saw them (see `compat::WidgetCache`)
    pub __widgets: compat::WidgetCache,
    /// `DropdownWidget.this` (the inner class's outer instance): its `selected`, `select(..)` and `formatter`, as
    /// the widget shares them with its rows (`DropdownLink`); set by the widget's constructor when it adds the row
    pub outer: std::rc::Weak<DropdownLink>,
}

impl EntryBase {
    /// `formatter.apply(entry)` of the enclosing widget (`None` before the row is in a widget)
    pub fn format_entry(&self) -> Option<MutableComponent> {
        self.outer.upgrade().and_then(|o| o.format_entry(&*self.entry))
    }
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
    pub fn new_in<__R, T: 'static>(mut element: T, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn Entry>, __R)) -> (compat::Shared<dyn Entry>, __R) {
        let __init_entry: std::rc::Rc<dyn std::any::Any> = std::rc::Rc::new(element);
        let this = Self {
            base: mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry::new(0),
            __widgets: Default::default(),
            __row: mcrust::net::minecraft::client::gui::screens::screen::Screen::new(String::new()),
            entry: __init_entry,
            outer: std::rc::Weak::new(),
        };
        let (__h, __r) = __wrap(this);
        let __this = __h;
        (__this, __r)
    }

    /// java: protected Entry(T element)
    pub fn new<T: 'static>(mut element: T) -> compat::Shared<Self> {
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
    ///
    /// (the port's `NarratableEntry` is not dyn-compatible - its `getNarratables` answers `Vec<&Self>` - so the list is
    /// typed by its `NarrationSupplier` half; GAPS_batchA.md)
    fn narratables(&self) -> fn(&compat::Shared<dyn Entry>) -> Vec<Box<dyn mcrust::net::minecraft::client::gui::narration::narration_supplier::NarrationSupplier>> {
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
    pub fn narratables_body(__this: &compat::Shared<dyn Entry>) -> Vec<Box<dyn mcrust::net::minecraft::client::gui::narration::narration_supplier::NarrationSupplier>> {
        return vec![];
    }

    /// java: @Override public List<? extends GuiEventListener> children()
    pub fn children_body(__this: &compat::Shared<dyn Entry>) -> Vec<Box<dyn GuiEventListener>> {
        return vec![];
    }

    /// java: @Override public void extractContent(GuiGraphicsExtractor graphics, int mouseX, int mouseY, boolean hovered, float deltaTicks)
    pub fn extract_content_body(__this: &compat::Shared<dyn Entry>, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut hovered: bool, mut delta_ticks: f32) {
        let _ = (graphics, mouse_x, mouse_y, delta_ticks);
        let (entry, x, y, width, outer) = compat::read(__this, |__t| {
            let __b = __t.entry_base();
            (__b.entry.clone(), __b.get_x(), __b.get_y(), __b.get_width(), __b.outer.upgrade())
        });
        let text = outer.as_ref().map(|o| o.text_of(&*entry)).unwrap_or_default();
        // java: graphics.textRenderer(GuiGraphicsExtractor.HoveredTextEffects.NONE).acceptScrollingWithDefaultCenter(Component.literal(entry.toString()).withStyle(Style.EMPTY.withUnderlined(hovered)), this.getX() + 10, this.getX() + this.getWidth(), this.getY(), this.getY() + 11);
        let message = mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::literal(&text).with_style(&mcrust::net::minecraft::network::chat::style::Style::EMPTY.with_underlined(Some(hovered))));
        accept_scrolling_with_default_center(&message, x.wrapping_add(10), x.wrapping_add(width), y, y.wrapping_add(11));
        // java: if (selected == this.entry)
        if outer.as_ref().is_some_and(|o| o.is_selected(&*entry)) {
            // java: graphics.text(client.font, "✔", this.getX() + 1, this.getY() + 2, 0xFFFFFFFF);
            gui_graphics_extractor::text_string(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), Some("✔"), x.wrapping_add(1), y.wrapping_add(2), 0xFFFFFFFFu32 as i32);
        }
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    pub fn mouse_clicked_body(__this: &compat::Shared<dyn Entry>, click: &MouseButtonEvent, mut doubled: bool) -> bool {
        let _ = (click, doubled);
        // java: select(entry); - the enclosing widget is mid-`mouseClicked`; it applies the selection when the child
        // pass returns to it (`DropdownLink::pending_select`)
        let (entry, outer) = compat::read(__this, |__t| (__t.entry_base().entry.clone(), __t.entry_base().outer.upgrade()));
        if let Some(outer) = outer {
            *outer.pending_select.borrow_mut() = Some(entry);
        }
        return true;
    }

}

impl Entry for EntryBase {
    fn entry_base(&self) -> &EntryBase {
        self
    }
    fn entry_base_mut(&mut self) -> &mut EntryBase {
        self
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
        let __s = self.entry_base_mut();
        (&mut __s.__row, &mut __s.__widgets)
    }
}

/// Entry as a row of the game's list (`addEntry` attaches it)
impl ModListEntry for compat::EntryAdapter<dyn Entry> {
    /// java: @Override public void extractContent(GuiGraphicsExtractor graphics, int mouseX, int mouseY, boolean hovered, float deltaTicks)
    fn extract_content(&mut self, list: &mut ModSelectionList, index: usize, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, hovered: bool, a: f32) {
        self.lend(Some(list.list.entry(index)), |__h| {
            let __f = compat::dispatch(__h, |__t| __t.extract_content());
            __f(__h, &context.graphics, mouse_x, mouse_y, hovered, a)
        });
        // the row's widgets (Java's `child.extractRenderState(..)` over `children()`)
        context.extract_widgets(&mut self.row, mouse_x, mouse_y);
    }

    fn children(&mut self) -> Option<&mut mcrust::net::minecraft::client::gui::screens::screen::Screen> {
        Some(&mut self.row)
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    fn mouse_clicked(&mut self, list: &mut ModSelectionList, index: usize, event: &MouseButtonEvent, double_click: bool) -> bool {
        self.lend(Some(list.list.entry(index)), |__h| {
            let __f = compat::dispatch(__h, |__t| __t.mouse_clicked());
            __f(__h, event, double_click)
        })
    }

}

/// `graphics.textRenderer(..).acceptScrollingWithDefaultCenter(message, left, right, top, bottom)` from a row draw that
/// holds only the extractor: `compat::accept_scrolling_with_default_center`'s placement and clipped draw, off the
/// frame's `Minecraft`.
fn accept_scrolling_with_default_center(message: &mcrust::net::minecraft::network::chat::formatted_text::FormattedText, left: i32, right: i32, top: i32, bottom: i32) {
    let minecraft = &*compat::minecraft();
    let width = mcrust::net::minecraft::client::gui::font::width(minecraft.font.splitter().string_width_formatted(message));
    let placement = mcrust::net::minecraft::client::gui::active_text_collector::accept_scrolling_with_default_center(left, right, top, bottom, width, mcrust::net::minecraft::util::util::get_millis());
    gui_graphics_extractor::draw_clipped_formatted_text(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), minecraft, placement.x, placement.y, message, -1, &mcrust::net::minecraft::network::chat::style::Style::EMPTY, placement.scissor, None, minecraft.window.get_gui_scale(), minecraft.window.get_height());
}

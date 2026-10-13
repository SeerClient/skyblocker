//! Ported from de.hysky.skyblocker.skyblock.tabhud.config.CustomDropdownWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::tabhud::config::top_bar_widget::TopBarWidget;
use crate::de::hysky::skyblocker::utils::render::gui::dropdown_widget::DropdownList;
use crate::de::hysky::skyblocker::utils::render::gui::dropdown_widget::DropdownListBase;
use crate::de::hysky::skyblocker::utils::render::gui::dropdown_widget::DropdownWidget;
use crate::de::hysky::skyblocker::utils::render::gui::dropdown_widget::DropdownWidgetBase;
use crate::de::hysky::skyblocker::utils::render::gui::dropdown_widget::EntryBase;
use mcrust::mcrust::modding::widgets::ModList;
use mcrust::mcrust::modding::widgets::ModListEntry;
use mcrust::mcrust::modding::widgets::ModSelectionList;
use mcrust::mcrust::modding::widgets::WidgetRenderContext;
use mcrust::net::minecraft::client::gui::active_text_collector;
use mcrust::net::minecraft::client::gui::components::abstract_selection_list::AbstractSelectionList;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::util::argb;
use crate::de::hysky::skyblocker::utils::render::gui::dropdown_widget::Entry as _;

/// java: private static final Identifier TEXTURE = SkyblockerMod.id("menu_outer_space");
pub static TEXTURE: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("menu_outer_space"));

/// java: class CustomDropdownWidget<T> extends DropdownWidget<T>
#[derive(Clone)]
pub struct CustomDropdownWidget<T> {
    /// java: extends DropdownWidget
    pub base: DropdownWidgetBase<T>,
    /// java: private final T firstEntry;
    pub first_entry: Option<T>,
}

impl<T: 'static> std::ops::Deref for CustomDropdownWidget<T> {
    type Target = DropdownWidgetBase<T>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<T: 'static> std::ops::DerefMut for CustomDropdownWidget<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl<T: Clone + 'static> CustomDropdownWidget<T> {
    /// java: CustomDropdownWidget(int x, int y, int width, int maxHeight, List<T> entries, Consumer<T> selectCallback, T selected)
    pub fn new(mut x: i32, mut y: i32, mut width: i32, mut max_height: i32, mut entries: Vec<T>, select_callback: Arc<dyn Fn(T)>, mut selected: T) -> compat::Shared<Self> where T: std::fmt::Display {
        // java: super(Minecraft.getInstance(), x, y, width, maxHeight, 12, entries, selectCallback, selected, _ -> {}); - the
        // base's constructor runs `createDropdown()` / `createEntry(..)`, which are this class's overrides: the value is
        // built first and the constructor's second half runs on it (`init_dropdown`)
        let mut __value = Self {
            base: DropdownWidgetBase::<T>::new_base(x, y, width, max_height, 12, entries.clone(), select_callback.clone(), selected.clone(), Arc::new(move |_| {

        })),
            first_entry: None,
        };
        __value.init_dropdown();
        let __this = compat::shared(__value);
        { let __v = 15; compat::write(&__this, |__t| __t.header_height = __v); }
        { let __v = Some(entries[0].clone()); compat::write(&__this, |__t| __t.first_entry = __v); }
        __this
    }

}

impl<T: Clone + 'static> DropdownWidget<T> for CustomDropdownWidget<T> {
    fn dropdown_widget_base(&self) -> &DropdownWidgetBase<T> {
        &self.base
    }
    fn dropdown_widget_base_mut(&mut self) -> &mut DropdownWidgetBase<T> {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override protected void extractHeader(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float partialTicks)
    fn extract_header(&self, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut partial_ticks: f32) {
        let mut y: i32 = self.get_y().wrapping_sub(1);
        let mut y2: i32 = y.wrapping_add(self.header_height);
        TopBarWidget::draw_button_border(graphics, self.get_x(), y, y2);
        TopBarWidget::draw_button_border(graphics, self.get_right(), y, y2);
        if self.is_hovered() && mouse_y < y2 {
            graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), self.get_x(), y, self.get_right().wrapping_add(1), y2, argb::color_alpha_rgb(100, 0));
        } else {
            graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), self.get_x(), y, self.get_right().wrapping_add(1), y2, argb::color_alpha_rgb(50, 0));
        }
        gui_graphics_extractor::text_string_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), Some(">"), self.get_x().wrapping_add(4), (self.get_y().wrapping_add((self.header_height.wrapping_sub(mcrust::net::minecraft::client::gui::font::LINE_HEIGHT)).wrapping_div(2))).wrapping_add(1), mcrust::net::minecraft::util::common_colors::LIGHTER_GRAY, true);
        // java: graphics.textRenderer().acceptScrolling(formatter.apply(selected), getX() + getWidth() / 2, getX() + 4 + 6, getRight() - 2, getY() + 2, getY() + headerHeight - 2);
        let __message = mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from((self.formatter)(self.selected.clone().expect("java.lang.NullPointerException: selected")));
        accept_scrolling(&__message, self.get_x().wrapping_add(self.get_width().wrapping_div(2)), (self.get_x().wrapping_add(4)).wrapping_add(6), self.get_right().wrapping_sub(2), self.get_y().wrapping_add(2), (self.get_y().wrapping_add(self.header_height)).wrapping_sub(2));
    }

    /// java: @Override protected DropdownWidget<T>.DropdownList createDropdown()
    fn create_dropdown(&self) -> compat::Shared<dyn DropdownList> {
        return (CustomDropdownList::new(compat::minecraft(), self.dropdown_widget_base().entry_height) as compat::Shared<dyn DropdownList>);
    }

    /// java: @Override protected DropdownWidget<T>.Entry createEntry(T element)
    fn create_entry(&self, element: &T) -> compat::Shared<dyn crate::de::hysky::skyblocker::utils::render::gui::dropdown_widget::Entry> {
        // `entry != firstEntry` (firstEntry = entries.get(0)): the entries are created in `entries` order, so the first
        // one is the one created while the list is still empty
        let is_first = self.dropdown_widget_base().dropdown_list.as_ref().is_some_and(|__l| compat::read(__l, |__t| __t.dropdown_list_base().base.entry_count() == 0));
        return (CustomEntry::new(element.clone(), is_first) as compat::Shared<dyn crate::de::hysky::skyblocker::utils::render::gui::dropdown_widget::Entry>);
    }

}

/// `graphics.textRenderer().acceptScrolling(message, centerX, left, right, top, bottom)` from a draw that holds only the
/// extractor: the scrolling placement (`ActiveTextCollector.acceptScrolling`) and the clipped draw, as
/// `compat::accept_scrolling_with_default_center` does with a widget render context
fn accept_scrolling(message: &mcrust::net::minecraft::network::chat::formatted_text::FormattedText, center_x: i32, left: i32, right: i32, top: i32, bottom: i32) {
    let minecraft = &*compat::minecraft();
    let width = mcrust::net::minecraft::client::gui::font::width(minecraft.font.splitter().string_width_formatted(message));
    let placement = active_text_collector::accept_scrolling(center_x, left, right, top, bottom, width, mcrust::net::minecraft::util::util::get_millis());
    gui_graphics_extractor::draw_clipped_formatted_text(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), minecraft, placement.x, placement.y, message, -1, &mcrust::net::minecraft::network::chat::style::Style::EMPTY, placement.scissor, None, minecraft.window.get_gui_scale(), minecraft.window.get_height());
}

/// java: class CustomDropdownList extends DropdownList
pub struct CustomDropdownList {
    /// java: extends DropdownList
    pub base: DropdownListBase,
}

impl std::ops::Deref for CustomDropdownList {
    type Target = DropdownListBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for CustomDropdownList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl CustomDropdownList {
    /// java: protected CustomDropdownList(Minecraft minecraftClient)
    ///
    /// (`entry_height` is the enclosing `DropdownWidget.this.entryHeight`, which `DropdownList(..)` reads)
    pub fn new(minecraft_client: &mut mcrust::net::minecraft::client::minecraft::Minecraft, entry_height: i32) -> compat::Shared<Self> {
        let (_, __this) = DropdownListBase::new_in(minecraft_client, entry_height, move |__base| {
            let __h = compat::shared(Self {
                base: __base,
            });
            ((__h.clone() as compat::Shared<dyn DropdownList>), __h)
        });
        __this
    }

}

impl DropdownList for CustomDropdownList {
    fn dropdown_list_base(&self) -> &DropdownListBase {
        &self.base
    }
    fn dropdown_list_base_mut(&mut self) -> &mut DropdownListBase {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override protected void extractListBackground(GuiGraphicsExtractor graphics) (dispatch: the body for this class)
    fn extract_list_background(&self) -> fn(&compat::Shared<dyn DropdownList>, &mut GuiGraphicsExtractor) {
        CustomDropdownList::extract_list_background_dyn
    }

}

impl CustomDropdownList {
    /// java: @Override protected void extractListBackground(GuiGraphicsExtractor graphics)
    pub fn extract_list_background_impl(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor) {
        graphics.enable_scissor(__this.lock().borrow().get_x(), (__this.lock().borrow().get_y()).wrapping_sub(1), __this.lock().borrow().get_right(), (__this.lock().borrow().get_bottom()).wrapping_add(2));
        graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), __this.lock().borrow().get_x(), (__this.lock().borrow().get_y()).wrapping_sub(3), __this.lock().borrow().get_right(), (__this.lock().borrow().get_bottom()).wrapping_add(2), argb::black_f32(0.4f32));
        gui_graphics_extractor::blit_sprite_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), TEXTURE.get_mut(), __this.lock().borrow().get_x(), (__this.lock().borrow().get_y()).wrapping_sub(3), __this.lock().borrow().get_width(), (__this.lock().borrow().get_height()).wrapping_add(5));
        graphics.disable_scissor();
    }

    /// CustomDropdownList.extractListBackground as the dispatch of DropdownList.extractListBackground
    pub fn extract_list_background_dyn(__this: &compat::Shared<dyn DropdownList>, __p0: &mut GuiGraphicsExtractor) {
        Self::extract_list_background_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

}

/// CustomDropdownList's list (the port's `ModSelectionList`) until the game takes it (`compat::attach_list`)
impl compat::ListHalf for CustomDropdownList {
    fn list_half(&mut self) -> &mut ModSelectionList {
        &mut self.base.base
    }
}

/// CustomDropdownList's overrides of the list's own methods, asked by the port's list bodies
impl ModList for compat::ListClass<CustomDropdownList> {
    /// java: @Override public int getRowWidth()
    fn get_row_width(&self, list: &AbstractSelectionList, super_answer: i32) -> i32 {
        self.ask(list, super_answer, |__h| ({ let __f = compat::dispatch(__h, |__t| __t.get_row_width()); __f(&compat::view(__h, |a| a as compat::Shared<dyn DropdownList>)) }))
    }

    /// java: @Override public int getRowLeft()
    fn get_row_left(&self, list: &AbstractSelectionList, super_answer: i32) -> i32 {
        self.ask(list, super_answer, |__h| ({ let __f = compat::dispatch(__h, |__t| __t.get_row_left()); __f(&compat::view(__h, |a| a as compat::Shared<dyn DropdownList>)) }))
    }

    /// java: @Override protected int scrollBarX()
    fn scroll_bar_x(&self, list: &AbstractSelectionList, super_answer: i32) -> i32 {
        self.ask(list, super_answer, |__h| ({ let __f = compat::dispatch(__h, |__t| __t.scroll_bar_x()); __f(&compat::view(__h, |a| a as compat::Shared<dyn DropdownList>)) }))
    }

    /// java: @Override protected void extractListBackground(GuiGraphicsExtractor graphics)
    fn extract_list_background(&self, list: &AbstractSelectionList, context: &mut WidgetRenderContext<'_>, super_extract_list_background: &mut dyn FnMut(&mut WidgetRenderContext<'_>)) {
        let _ = super_extract_list_background; // the override replaces the base's body (it never calls it)
        self.ask(list, (), |__h| ({ let __f = compat::dispatch(__h, |__t| __t.extract_list_background()); __f(&compat::view(__h, |a| a as compat::Shared<dyn DropdownList>), &mut context.graphics) }))
    }

    /// java: @Override protected void extractListSeparators(GuiGraphicsExtractor graphics)
    fn extract_list_separators(&self, list: &AbstractSelectionList, context: &mut WidgetRenderContext<'_>, super_extract_list_separators: &mut dyn FnMut(&mut WidgetRenderContext<'_>)) {
        let _ = super_extract_list_separators; // the override replaces the base's body (it never calls it)
        self.ask(list, (), |__h| ({ let __f = compat::dispatch(__h, |__t| __t.extract_list_separators()); __f(&compat::view(__h, |a| a as compat::Shared<dyn DropdownList>), &context.graphics) }))
    }

}

/// java: class CustomEntry extends Entry
pub struct CustomEntry {
    /// java: extends Entry
    pub base: EntryBase,
    /// `entry == CustomDropdownWidget.this.firstEntry` (the inner class's read of its enclosing widget, fixed when the
    /// entry is made)
    pub is_first: bool,
}

impl std::ops::Deref for CustomEntry {
    type Target = EntryBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for CustomEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl CustomEntry {
    /// java: protected CustomEntry(T element) (`T` is the enclosing CustomDropdownWidget's)
    pub fn new<T: Clone + 'static>(element: T, is_first: bool) -> compat::Shared<Self> {
        let (_, __this) = EntryBase::new_in(element.clone(), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                is_first,
            });
            ((__h.clone() as compat::Shared<dyn crate::de::hysky::skyblocker::utils::render::gui::dropdown_widget::Entry>), __h)
        });
        __this
    }

}

impl crate::de::hysky::skyblocker::utils::render::gui::dropdown_widget::Entry for CustomEntry {
    fn entry_base(&self) -> &EntryBase {
        &self.base
    }
    fn entry_base_mut(&mut self) -> &mut EntryBase {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void extractContent(GuiGraphicsExtractor graphics, int mouseX, int mouseY, boolean hovered, float deltaTicks) (dispatch: the body for this class)
    fn extract_content(&self) -> fn(&compat::Shared<dyn crate::de::hysky::skyblocker::utils::render::gui::dropdown_widget::Entry>, &GuiGraphicsExtractor, i32, i32, bool, f32) {
        CustomEntry::extract_content_dyn
    }

}

impl CustomEntry {
    /// java: @Override public void extractContent(GuiGraphicsExtractor graphics, int mouseX, int mouseY, boolean hovered, float deltaTicks)
    pub fn extract_content_impl(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut hovered: bool, mut delta_ticks: f32) {
        //   java: entry != firstEntry
        if !compat::read(__this, |__t| __t.is_first) {
            gui_graphics_extractor::horizontal_line(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), __this.lock().borrow().get_x(), (__this.lock().borrow().get_x()).wrapping_add(__this.lock().borrow().get_width()), __this.lock().borrow().get_y(), argb::color_alpha_rgb(15, 0));
        }
        // rust: port gap - `formatter` is a field of the enclosing CustomDropdownWidget.this, which the entry (made by the
        // widget's constructor before the widget is shared, and erased to `Rc<dyn Any>`) cannot reach; the label is not
        // drawn (GAPS_O)
        //   java: graphics.textRenderer().acceptScrollingWithDefaultCenter(
        //   java: 					formatter.apply(entry).copy().withStyle(Style.EMPTY.withUnderlined(hovered)),
        //   java: 					this.getX(),
        //   java: 					this.getX() + this.getWidth(),
        //   java: 					this.getY(),
        //   java: 					this.getY() + 11
        //   java: 			);
    }

    /// CustomEntry.extractContent as the dispatch of Entry.extractContent
    pub fn extract_content_dyn(__this: &compat::Shared<dyn crate::de::hysky::skyblocker::utils::render::gui::dropdown_widget::Entry>, __p0: &GuiGraphicsExtractor, __p1: i32, __p2: i32, __p3: bool, __p4: f32) {
        Self::extract_content_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2, __p3, __p4)
    }

}

/// CustomEntry's row box and its widgets, lent by its `compat::EntryAdapter` for each call
impl compat::RowHalf for CustomEntry {
    fn entry_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry {
        &mut self.base.base
    }
    fn row_parts(&mut self) -> (&mut mcrust::net::minecraft::client::gui::screens::screen::Screen, &mut compat::WidgetCache) {
        let __s = &mut self.base;
        (&mut __s.__row, &mut __s.__widgets)
    }
}

/// CustomEntry as a row of the game's list (`addEntry` attaches it)
impl ModListEntry for compat::EntryAdapter<CustomEntry> {
    /// java: @Override public void extractContent(GuiGraphicsExtractor graphics, int mouseX, int mouseY, boolean hovered, float deltaTicks)
    fn extract_content(&mut self, list: &mut ModSelectionList, index: usize, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, hovered: bool, a: f32) {
        self.lend(Some(list.list.entry(index)), |__h| ({ let __f = compat::dispatch(__h, |__t| __t.extract_content()); __f(&compat::view(__h, |a| a as compat::Shared<dyn crate::de::hysky::skyblocker::utils::render::gui::dropdown_widget::Entry>), &context.graphics, mouse_x, mouse_y, hovered, a) }));
        // the row's widgets (Java's `child.extractRenderState(..)` over `children()`)
        context.extract_widgets(&mut self.row, mouse_x, mouse_y);
    }

    fn children(&mut self) -> Option<&mut mcrust::net::minecraft::client::gui::screens::screen::Screen> {
        Some(&mut self.row)
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    fn mouse_clicked(&mut self, list: &mut ModSelectionList, index: usize, event: &MouseButtonEvent, double_click: bool) -> bool {
        self.lend(Some(list.list.entry(index)), |__h| ({ let __f = compat::dispatch(__h, |__t| __t.mouse_clicked()); __f(&compat::view(__h, |a| a as compat::Shared<dyn crate::de::hysky::skyblocker::utils::render::gui::dropdown_widget::Entry>), event, double_click) }))
    }

}

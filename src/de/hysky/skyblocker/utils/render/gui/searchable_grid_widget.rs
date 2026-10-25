//! Ported from de.hysky.skyblocker.utils.render.gui.SearchableGridWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::mcrust::modding::widgets::WidgetRenderContext;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::components::abstract_container_widget::AbstractContainerWidget;
use mcrust::net::minecraft::client::gui::components::abstract_scroll_area::ScrollArea;
use mcrust::net::minecraft::client::gui::components::abstract_scroll_area::ScrollbarSettings;
use mcrust::net::minecraft::client::gui::components::abstract_scroll_area::SCROLLBAR_WIDTH;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::edit_box::Responder;
use mcrust::net::minecraft::client::gui::components::edit_box::ResponderLambda;
use mcrust::net::minecraft::client::gui::components::events::gui_event_listener::GuiEventListener;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::layouts::grid_layout::GridLayout;
use mcrust::net::minecraft::client::gui::layouts::grid_layout::RowHelper;
use mcrust::net::minecraft::client::gui::layouts::layout_element::LayoutElement;
use mcrust::net::minecraft::client::gui::layouts::linear_layout::LinearLayout;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::gui::navigation::screen_rectangle::ScreenRectangle;
use mcrust::net::minecraft::client::gui::screens::screen::Screen;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::key_event::KeyEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::formatted_text::FormattedText;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: private static final int TEXT_FIELD_HEIGHT = 20;
pub const TEXT_FIELD_HEIGHT: i32 = 20;

/// The index of the search field in `SearchableGridWidgetBase::children`; the filtered widgets follow it.
pub const SEARCH_FIELD_INDEX: usize = 0;

/// java: public abstract class SearchableGridWidget extends AbstractContainerWidget
///
/// The widget's two children - the search field and the widgets container (whose children are the filtered widgets)
/// - are game widgets the object holds in a screen of its own (`children`: the search field first, then the filtered
/// widgets), drawn through the game's widget pass (`extract_children`) and handed their input by the object
/// (`mouse_clicked`, `key_pressed`, ..), as `ContainerEventHandler` hands its children theirs.
pub struct SearchableGridWidgetBase {
    /// java: extends net.minecraft.client.gui.components.AbstractContainerWidget
    pub base: AbstractContainerWidget,
    /// java: private final List<AbstractWidget> filteredWidgets = new ObjectArrayList<>(); - held in `children` from
    /// `SEARCH_FIELD_INDEX + 1` on
    pub children: Screen,
    /// java: private final LinearLayout layoutWidget = LinearLayout.vertical(); - the search field over the widgets
    /// container (both laid out from the widget's x and y: the field at the top, the container under it)
    pub layout_widget: LinearLayout,
    /// java: private final EditBox searchField; - the value the responder last reported, applied by the object once
    /// the field's call is over (the responder runs inside the field's own call)
    pub pending_search: std::rc::Rc<std::cell::RefCell<Option<String>>>,
    /// java: private final WidgetsContainer widgetsContainer;
    pub widgets_container: Option<WidgetsContainer>,
    /// java: private final int expectedWidgetWidth;
    pub expected_widget_width: i32,
    /// java: private final int maxPerRow;
    pub max_per_row: i32,
    /// java: private final boolean packed;
    pub packed: bool,
    /// java: private final boolean spaceElementsOut;
    pub space_elements_out: bool,
}

impl std::ops::Deref for SearchableGridWidgetBase {
    type Target = AbstractContainerWidget;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for SearchableGridWidgetBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl SearchableGridWidgetBase {
    /// java: public SearchableGridWidget(int x, int y, int width, int height, Component message, int expectedWidgetWidth, int maxPerRow, boolean packed, boolean spaceElementsOut) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_in<__R>(x: i32, y: i32, width: i32, height: i32, message: MutableComponent, expected_widget_width: i32, max_per_row: i32, packed: bool, space_elements_out: bool, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn SearchableGridWidget>, __R)) -> (compat::Shared<dyn SearchableGridWidget>, __R) {
        // java: super(x, y, width, height, message, AbstractScrollArea.defaultSettings(8));
        let base = AbstractContainerWidget::new_with_scrollbar_settings(x, y, width, height, FormattedText::from(message), ScrollbarSettings::default_settings(8.0));
        // java: searchField = new EditBox(Minecraft.getInstance().font, width, TEXT_FIELD_HEIGHT, Component.translatable("gui.recipebook.search_hint"));
        let mut search_field = compat::edit_box(0, 0, width, TEXT_FIELD_HEIGHT, component::translatable("gui.recipebook.search_hint"));
        // java: searchField.setHint(Component.translatable("gui.recipebook.search_hint").withStyle(ChatFormatting.ITALIC).withStyle(ChatFormatting.GRAY));
        // java: searchField.setResponder(this::filterInternal);
        let pending_search: std::rc::Rc<std::cell::RefCell<Option<String>>> = Default::default();
        let __pending = compat::RenderThread::new(pending_search.clone());
        compat::with_edit_box(&mut search_field, |__e| {
            __e.set_hint_component(FormattedText::from(component::translatable("gui.recipebook.search_hint").with_style_format(ChatFormatting::Italic).with_style_format(ChatFormatting::Gray)));
            __e.set_responder(Responder::Lambda(ResponderLambda::new(move |__a0: &str| { *__pending.get().borrow_mut() = Some(__a0.to_string()); })));
        });
        let mut children = Screen::new(String::new());
        children.add_renderable_widget(search_field);
        // java: widgetsContainer = new WidgetsContainer(); - `super(0, 0, SearchableGridWidget.this.getWidth(),
        // SearchableGridWidget.this.getHeight() - TEXT_FIELD_HEIGHT, Component.literal("Grid"), defaultSettings(8))`
        let widgets_container = WidgetsContainer::new(width, height - TEXT_FIELD_HEIGHT);
        let this = Self {
            base,
            children,
            layout_widget: LinearLayout::vertical(),
            pending_search,
            widgets_container: Some(widgets_container),
            expected_widget_width,
            max_per_row,
            packed,
            space_elements_out,
        };
        let (__this, __r) = __wrap(this);
        // java: layoutWidget.addChild(searchField); layoutWidget.addChild(widgetsContainer); layoutWidget.arrangeElements();
        // layoutWidget.setPosition(x, y);
        <dyn SearchableGridWidget>::arrange(&__this);
        // java: setWidth(getWidth()); - the class's own `setWidth`
        let __width = compat::read(&__this, |__t| __t.searchable_grid_widget_base().get_width());
        ({ let __f = compat::dispatch(&__this, |__t| __t.set_width()); __f(&__this, __width) });
        (__this, __r)
    }

    /// java: public SearchableGridWidget(int x, int y, int width, int height, Component message, int expectedWidgetWidth, int maxPerRow, boolean packed) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_8_in<__R>(x: i32, y: i32, width: i32, height: i32, message: MutableComponent, expected_widget_width: i32, max_per_row: i32, packed: bool, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn SearchableGridWidget>, __R)) -> (compat::Shared<dyn SearchableGridWidget>, __R) {
        Self::new_in(x, y, width, height, message, expected_widget_width, max_per_row, packed, false, __wrap)
    }

    /// java: public SearchableGridWidget(int x, int y, int width, int height, Component message, int expectedWidgetWidth, boolean packed) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_7_in<__R>(x: i32, y: i32, width: i32, height: i32, message: MutableComponent, expected_widget_width: i32, packed: bool, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn SearchableGridWidget>, __R)) -> (compat::Shared<dyn SearchableGridWidget>, __R) {
        Self::new_8_in(x, y, width, height, message, expected_widget_width, i32::MAX, packed, __wrap)
    }

    /// java: public SearchableGridWidget(int x, int y, int width, int height, Component message, int expectedWidgetWidth) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_6_in<__R>(x: i32, y: i32, width: i32, height: i32, message: MutableComponent, expected_widget_width: i32, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn SearchableGridWidget>, __R)) -> (compat::Shared<dyn SearchableGridWidget>, __R) {
        Self::new_7_in(x, y, width, height, message, expected_widget_width, false, __wrap)
    }

    /// The filtered widgets (`filteredWidgets`), in order.
    pub fn filtered_widgets(&self) -> &[AbstractWidget] {
        &self.children.children()[SEARCH_FIELD_INDEX + 1..]
    }

}

/// java: the virtual methods of SearchableGridWidget (implemented by SearchableGridWidgetBase and every subclass)
pub trait SearchableGridWidget: compat::DynAny {
    fn searchable_grid_widget_base(&self) -> &SearchableGridWidgetBase;
    fn searchable_grid_widget_base_mut(&mut self) -> &mut SearchableGridWidgetBase;
    fn as_any(&self) -> &dyn std::any::Any;

    /// java: @Override public void setX(int x) (dispatch: the body for this class)
    fn set_x(&self) -> fn(&compat::Shared<dyn SearchableGridWidget>, i32) {
        <dyn SearchableGridWidget>::set_x_body
    }

    /// java: @Override public void setY(int y) (dispatch: the body for this class)
    fn set_y(&self) -> fn(&compat::Shared<dyn SearchableGridWidget>, i32) {
        <dyn SearchableGridWidget>::set_y_body
    }

    /// java: @Override public void setWidth(int width) (dispatch: the body for this class)
    fn set_width(&self) -> fn(&compat::Shared<dyn SearchableGridWidget>, i32) {
        <dyn SearchableGridWidget>::set_width_body
    }

    /// java: @Override public void setHeight(int height) (dispatch: the body for this class)
    fn set_height(&self) -> fn(&compat::Shared<dyn SearchableGridWidget>, i32) {
        <dyn SearchableGridWidget>::set_height_body
    }

    /// java: public void setSearch(String search) (dispatch: the body for this class)
    fn set_search(&self) -> fn(&compat::Shared<dyn SearchableGridWidget>, &str) {
        <dyn SearchableGridWidget>::set_search_body
    }

    /// java: public void refreshSearch() (dispatch: the body for this class)
    fn refresh_search(&self) -> fn(&compat::Shared<dyn SearchableGridWidget>) {
        <dyn SearchableGridWidget>::refresh_search_body
    }

    /// java: public void setScrollAmount(double amount) (dispatch: the body for this class)
    fn set_scroll_amount(&self) -> fn(&compat::Shared<dyn SearchableGridWidget>, f64) {
        <dyn SearchableGridWidget>::set_scroll_amount_body
    }

    /// java: public double getScrollAmount() (dispatch: the body for this class)
    fn get_scroll_amount(&self) -> fn(&compat::Shared<dyn SearchableGridWidget>) -> f64 {
        <dyn SearchableGridWidget>::get_scroll_amount_body
    }

    /// java: public ScreenRectangle getGridRectangle() (dispatch: the body for this class)
    fn get_grid_rectangle(&self) -> fn(&compat::Shared<dyn SearchableGridWidget>) -> ScreenRectangle {
        <dyn SearchableGridWidget>::get_grid_rectangle_body
    }

    /// java: private void filterInternal(String input) (dispatch: the body for this class)
    fn filter_internal(&self) -> fn(&compat::Shared<dyn SearchableGridWidget>, &str) {
        <dyn SearchableGridWidget>::filter_internal_body
    }

    /// java: protected abstract Collection<? extends AbstractWidget> filterWidgets(String input); (dispatch: the body for this class)
    fn filter_widgets(&self) -> fn(&compat::Shared<dyn SearchableGridWidget>, String) -> Vec<AbstractWidget>;

    /// java: @Override public List<? extends GuiEventListener> children() (dispatch: the body for this class)
    fn children(&self) -> fn(&compat::Shared<dyn SearchableGridWidget>) -> Vec<Box<dyn GuiEventListener>> {
        <dyn SearchableGridWidget>::children_body
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor context, int mouseX, int mouseY, float deltaTicks) (dispatch: the body for this class)
    fn extract_widget_render_state(&self) -> fn(&compat::Shared<dyn SearchableGridWidget>, &GuiGraphicsExtractor, i32, i32, f32) {
        <dyn SearchableGridWidget>::extract_widget_render_state_body
    }

    /// java: @Override public boolean mouseScrolled(double mouseX, double mouseY, double horizontalAmount, double verticalAmount) (dispatch: the body for this class)
    fn mouse_scrolled(&self) -> fn(&compat::Shared<dyn SearchableGridWidget>, f64, f64, f64, f64) -> bool {
        <dyn SearchableGridWidget>::mouse_scrolled_body
    }

    /// java: protected double scrollRate() - `AbstractScrollArea`'s, which `WidgetsContainer.scrollRate()` asks of
    /// the enclosing grid (`SearchableGridWidget.this.scrollRate()`) (dispatch: the body for this class)
    fn scroll_rate(&self) -> fn(&compat::Shared<dyn SearchableGridWidget>) -> f64 {
        <dyn SearchableGridWidget>::scroll_rate_body
    }

    /// java: @Override protected int contentHeight() (dispatch: the body for this class)
    fn content_height(&self) -> fn(&compat::Shared<dyn SearchableGridWidget>) -> i32 {
        <dyn SearchableGridWidget>::content_height_body
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder) (dispatch: the body for this class)
    fn update_widget_narration(&self) -> fn(&compat::Shared<dyn SearchableGridWidget>, &dyn NarrationElementOutput) {
        <dyn SearchableGridWidget>::update_widget_narration_body
    }

}

impl dyn SearchableGridWidget {
    /// `layoutWidget.arrangeElements()` after `setPosition(x, y)`: the search field at the widget's x and y, the
    /// widgets container under it (a vertical `LinearLayout` with no spacing), the container's grid with it.
    pub fn arrange(__this: &compat::Shared<dyn SearchableGridWidget>) {
        compat::write(__this, |__t| {
            let __b = __t.searchable_grid_widget_base_mut();
            let (x, y) = (__b.base.get_x(), __b.base.get_y());
            __b.children.widget_mut(SEARCH_FIELD_INDEX).set_position(x, y);
            let container_y = y + __b.children.widget(SEARCH_FIELD_INDEX).get_height();
            if let Some(container) = __b.widgets_container.as_mut() {
                container.set_x(x);
                container.set_y(container_y);
            }
        });
        <dyn SearchableGridWidget>::recreate_grid(__this);
    }

    /// The search value the field's responder reported since the last call (`this::filterInternal`), run now.
    pub fn apply_pending_search(__this: &compat::Shared<dyn SearchableGridWidget>) {
        let pending = compat::read(__this, |__t| __t.searchable_grid_widget_base().pending_search.clone());
        let search = pending.borrow_mut().take();
        if let Some(search) = search {
            let __f = compat::dispatch(__this, |__t| __t.filter_internal());
            __f(__this, &search);
        }
    }

    /// java: @Override public void setX(int x)
    pub fn set_x_body(__this: &compat::Shared<dyn SearchableGridWidget>, x: i32) {
        // java: super.setX(x); layoutWidget.setX(x);
        __this.lock().borrow_mut().searchable_grid_widget_base_mut().base.set_x(x);
        <dyn SearchableGridWidget>::arrange(__this);
    }

    /// java: @Override public void setY(int y)
    pub fn set_y_body(__this: &compat::Shared<dyn SearchableGridWidget>, y: i32) {
        // java: super.setY(y); layoutWidget.setY(y);
        __this.lock().borrow_mut().searchable_grid_widget_base_mut().base.set_y(y);
        <dyn SearchableGridWidget>::arrange(__this);
    }

    /// java: @Override public void setWidth(int width)
    pub fn set_width_body(__this: &compat::Shared<dyn SearchableGridWidget>, mut width: i32) {
        if compat::read(__this, |__t| __t.searchable_grid_widget_base().packed) {
            let (expected, max_per_row, x) = compat::read(__this, |__t| { let __b = __t.searchable_grid_widget_base(); (__b.expected_widget_width, __b.max_per_row, __b.get_x()) });
            let per_row: i32 = ((width.wrapping_sub(SCROLLBAR_WIDTH)).wrapping_div(expected)).min(max_per_row);
            let new_width: i32 = (per_row.wrapping_mul(expected)).wrapping_add(SCROLLBAR_WIDTH);
            ({ let __f = compat::dispatch(__this, |__t| __t.set_x()); __f(__this, x.wrapping_add((width.wrapping_sub(new_width)).wrapping_div(2))) });
            width = new_width;
        }
        // java: super.setWidth(width); searchField.setWidth(width); widgetsContainer.setWidth(width); layoutWidget.arrangeElements();
        compat::write(__this, |__t| {
            let __b = __t.searchable_grid_widget_base_mut();
            __b.base.set_width(width);
            let __field = __b.children.widget_mut(SEARCH_FIELD_INDEX);
            __field.width = width;
            let __height = __field.get_height();
            compat::with_edit_box(__field, |__e| __e.set_geometry(__e.get_x(), __e.get_y(), width, __height));
        });
        <dyn SearchableGridWidget>::container_set_size(__this, Some(width), None);
        <dyn SearchableGridWidget>::arrange(__this);
    }

    /// java: @Override public void setHeight(int height)
    pub fn set_height_body(__this: &compat::Shared<dyn SearchableGridWidget>, height: i32) {
        // java: super.setHeight(height); widgetsContainer.setHeight(height - TEXT_FIELD_HEIGHT); layoutWidget.arrangeElements();
        __this.lock().borrow_mut().searchable_grid_widget_base_mut().base.set_height(height);
        <dyn SearchableGridWidget>::container_set_size(__this, None, Some(height.wrapping_sub(TEXT_FIELD_HEIGHT)));
        <dyn SearchableGridWidget>::arrange(__this);
    }

    /// `widgetsContainer.setWidth(w)` / `setHeight(h)` - each `recreateGrid()`s
    fn container_set_size(__this: &compat::Shared<dyn SearchableGridWidget>, width: Option<i32>, height: Option<i32>) {
        compat::write(__this, |__t| {
            let __b = __t.searchable_grid_widget_base_mut();
            if let Some(container) = __b.widgets_container.as_mut() {
                if let Some(width) = width {
                    container.base.set_width(width);
                }
                if let Some(height) = height {
                    container.base.set_height(height);
                    container.area.height = height;
                }
            }
        });
        <dyn SearchableGridWidget>::recreate_grid(__this);
    }

    /// java: public void setSearch(String search)
    pub fn set_search_body(__this: &compat::Shared<dyn SearchableGridWidget>, search: &str) {
        // java: searchField.setValue(search); - its responder (`filterInternal`) runs
        let __font = compat::font();
        compat::write(__this, |__t| compat::with_edit_box(__t.searchable_grid_widget_base_mut().children.widget_mut(SEARCH_FIELD_INDEX), |__e| __e.set_value(&__font.splitter(), search)));
        <dyn SearchableGridWidget>::apply_pending_search(__this);
    }

    /// java: public void refreshSearch()
    pub fn refresh_search_body(__this: &compat::Shared<dyn SearchableGridWidget>) {
        // java: searchField.setValue(searchField.getValue());
        let value: String = compat::read(__this, |__t| compat::edit_box_value(__t.searchable_grid_widget_base().children.widget(SEARCH_FIELD_INDEX)));
        <dyn SearchableGridWidget>::set_search_body(__this, &value);
    }

    /// java: public void setScrollAmount(double amount)
    pub fn set_scroll_amount_body(__this: &compat::Shared<dyn SearchableGridWidget>, amount: f64) {
        // java: widgetsContainer.setScrollAmount(amount); - `super.setScrollAmount(amount); grid.setY(getY() - (int) scrollAmount());`
        compat::write(__this, |__t| {
            if let Some(container) = __t.searchable_grid_widget_base_mut().widgets_container.as_mut() {
                container.set_scroll_amount(amount);
            }
        });
        <dyn SearchableGridWidget>::recreate_grid(__this);
    }

    /// java: public double getScrollAmount()
    pub fn get_scroll_amount_body(__this: &compat::Shared<dyn SearchableGridWidget>) -> f64 {
        // java: return widgetsContainer.scrollAmount();
        compat::read(__this, |__t| __t.searchable_grid_widget_base().widgets_container.as_ref().map_or(0.0, |__c| __c.area.scroll_amount()))
    }

    /// java: public ScreenRectangle getGridRectangle()
    pub fn get_grid_rectangle_body(__this: &compat::Shared<dyn SearchableGridWidget>) -> ScreenRectangle {
        return __this.lock().borrow().searchable_grid_widget_base().widgets_container.as_ref().unwrap().get_rectangle();
    }

    /// java: private void filterInternal(String input)
    pub fn filter_internal_body(__this: &compat::Shared<dyn SearchableGridWidget>, input: &str) {
        let widgets: Vec<AbstractWidget> = ({ let __f = compat::dispatch(__this, |__t| __t.filter_widgets()); __f(__this, input.to_string()) });
        // java: filteredWidgets.clear(); filteredWidgets.addAll(widgets);
        compat::write(__this, |__t| {
            let __children = &mut __t.searchable_grid_widget_base_mut().children;
            __children.truncate_widgets(SEARCH_FIELD_INDEX + 1);
            for widget in widgets {
                __children.add_renderable_widget(widget);
            }
        });
        // java: widgetsContainer.recreateGrid();
        <dyn SearchableGridWidget>::recreate_grid(__this);
    }

    /// WidgetsContainer.recreateGrid - the filtered widgets laid out in the container's columns
    pub fn recreate_grid(__this: &compat::Shared<dyn SearchableGridWidget>) {
        compat::write(__this, |__t| {
            let __b = __t.searchable_grid_widget_base_mut();
            let (expected, space_elements_out) = (__b.expected_widget_width, __b.space_elements_out);
            let sizes: Vec<(usize, i32, i32)> = (SEARCH_FIELD_INDEX + 1..__b.children.children().len()).map(|__i| (__i, __b.children.widget(__i).get_width(), __b.children.widget(__i).get_height())).collect();
            let Some(container) = __b.widgets_container.as_mut() else { return; };
            container.recreate_grid(expected, space_elements_out, &sizes);
            // the widgets take their places in the grid
            let mut placed: Vec<(usize, i32, i32)> = Vec::new();
            LayoutElement::Grid(container.grid.clone()).visit_widgets(&mut placed);
            for (__i, __x, __y) in placed {
                __b.children.widget_mut(__i).set_position(__x, __y);
            }
        });
    }

    /// java: @Override public List<? extends GuiEventListener> children()
    ///
    /// `List.of(searchField, widgetsContainer)`: the two children are game widgets in the object's own screen
    /// (`children`), handed their input by `mouse_clicked` / `key_pressed` / .. - a list of owned listeners cannot
    /// lend them, so it is empty.
    pub fn children_body(__this: &compat::Shared<dyn SearchableGridWidget>) -> Vec<Box<dyn GuiEventListener>> {
        let _ = __this;
        Vec::new()
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor context, int mouseX, int mouseY, float deltaTicks)
    ///
    /// `searchField.extractRenderState(..); widgetsContainer.extractRenderState(..);` - the children are game
    /// widgets, drawn through the game's widget pass, which needs the frame's render context: `extract_children`,
    /// which the class's widget slot calls after this body.
    pub fn extract_widget_render_state_body(__this: &compat::Shared<dyn SearchableGridWidget>, _context: &GuiGraphicsExtractor, _mouse_x: i32, _mouse_y: i32, _delta_ticks: f32) {
        let _ = __this;
    }

    /// `searchField.extractRenderState(..); widgetsContainer.extractRenderState(..);` - the container's
    /// `extractWidgetRenderState`: its visible widgets (`isVisible`) and its scroll bar.
    pub fn extract_children(__this: &compat::Shared<dyn SearchableGridWidget>, context: &WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32) {
        <dyn SearchableGridWidget>::apply_pending_search(__this);
        let (left, top, right, bottom, area, content_height, scroll_bar_x) = compat::read(__this, |__t| {
            let __c = __t.searchable_grid_widget_base().widgets_container.as_ref().unwrap();
            (__c.get_x(), __c.get_y(), __c.get_right(), __c.get_bottom(), __c.area, __c.content_height(), __c.get_right() - __c.area.scrollbar_width())
        });
        // (the children are drawn out of the object: a child's draw reaches mod objects that may reach this one)
        let mut children = compat::write(__this, |__t| std::mem::take(&mut __t.searchable_grid_widget_base_mut().children));
        // java: for (AbstractWidget widget : filteredWidgets) if (isVisible(widget)) widget.extractRenderState(..);
        // (the container's `enableScissor(getX(), getY(), getRight(), getBottom())` is each child's `clip`, which the
        // widget pass pushes around that child's draw)
        for __i in SEARCH_FIELD_INDEX + 1..children.children().len() {
            let __w = children.widget_mut(__i);
            __w.visible = __w.get_bottom() >= top && __w.get_y() < bottom;
            __w.clip = Some((left, top, right, bottom));
        }
        context.extract_widgets(&mut children, mouse_x, mouse_y);
        compat::write(__this, |__t| __t.searchable_grid_widget_base_mut().children = children);
        // java: this.extractScrollbar(context, mouseX, mouseY);
        compat::extract_scrollbar(context, &area, content_height, scroll_bar_x);
    }

    /// AbstractContainerWidget.mouseClicked - the container's scroll bar, then `ContainerEventHandler.mouseClicked`
    /// over the search field and the filtered widgets
    pub fn mouse_clicked(__this: &compat::Shared<dyn SearchableGridWidget>, event: &MouseButtonEvent) -> bool {
        let over_scrollbar = compat::write(__this, |__t| {
            let __c = __t.searchable_grid_widget_base_mut().widgets_container.as_mut().unwrap();
            let __x = __c.get_right() - __c.area.scrollbar_width();
            let __content = __c.content_height();
            __c.area.update_scrolling(__content, __x, event.button(), event.x(), event.y())
        });
        let mut children = compat::write(__this, |__t| std::mem::take(&mut __t.searchable_grid_widget_base_mut().children));
        let taken = compat::children_mouse_clicked(&mut children, event);
        compat::write(__this, |__t| __t.searchable_grid_widget_base_mut().children = children);
        <dyn SearchableGridWidget>::apply_pending_search(__this);
        taken || over_scrollbar
    }

    /// ContainerEventHandler.mouseReleased - the children, and the container's scroll bar released
    pub fn mouse_released(__this: &compat::Shared<dyn SearchableGridWidget>, event: &MouseButtonEvent) -> bool {
        compat::write(__this, |__t| { if let Some(__c) = __t.searchable_grid_widget_base_mut().widgets_container.as_mut() { __c.area.on_release(); } });
        let mut children = compat::write(__this, |__t| std::mem::take(&mut __t.searchable_grid_widget_base_mut().children));
        compat::children_mouse_released(&mut children, event);
        compat::write(__this, |__t| __t.searchable_grid_widget_base_mut().children = children);
        <dyn SearchableGridWidget>::apply_pending_search(__this);
        false
    }

    /// AbstractScrollArea.mouseDragged (the container's scroll bar) / ContainerEventHandler.mouseDragged
    pub fn mouse_dragged(__this: &compat::Shared<dyn SearchableGridWidget>, event: &MouseButtonEvent, dx: f64, dy: f64) -> bool {
        let scrolled = compat::write(__this, |__t| {
            let __c = __t.searchable_grid_widget_base_mut().widgets_container.as_mut().unwrap();
            let __content = __c.content_height();
            let __moved = __c.area.mouse_dragged(__content, event.y(), dy);
            if __moved {
                __c.place_grid();
            }
            __moved
        });
        if scrolled {
            <dyn SearchableGridWidget>::recreate_grid(__this);
            return true;
        }
        let mut children = compat::write(__this, |__t| std::mem::take(&mut __t.searchable_grid_widget_base_mut().children));
        compat::children_mouse_dragged(&mut children, event, dx, dy);
        compat::write(__this, |__t| __t.searchable_grid_widget_base_mut().children = children);
        <dyn SearchableGridWidget>::apply_pending_search(__this);
        false
    }

    /// ContainerEventHandler.keyPressed - the focused child (the search field)
    pub fn key_pressed(__this: &compat::Shared<dyn SearchableGridWidget>, event: &KeyEvent) -> bool {
        let mut children = compat::write(__this, |__t| std::mem::take(&mut __t.searchable_grid_widget_base_mut().children));
        let taken = compat::children_key_pressed(&mut children, event);
        compat::write(__this, |__t| __t.searchable_grid_widget_base_mut().children = children);
        <dyn SearchableGridWidget>::apply_pending_search(__this);
        taken
    }

    /// ContainerEventHandler.charTyped - the focused child (the search field)
    pub fn char_typed(__this: &compat::Shared<dyn SearchableGridWidget>, event: &CharacterEvent) -> bool {
        let mut children = compat::write(__this, |__t| std::mem::take(&mut __t.searchable_grid_widget_base_mut().children));
        let taken = compat::children_char_typed(&mut children, event);
        compat::write(__this, |__t| __t.searchable_grid_widget_base_mut().children = children);
        <dyn SearchableGridWidget>::apply_pending_search(__this);
        taken
    }

    /// java: @Override public boolean mouseScrolled(double mouseX, double mouseY, double horizontalAmount, double verticalAmount)
    pub fn mouse_scrolled_body(__this: &compat::Shared<dyn SearchableGridWidget>, mouse_x: f64, mouse_y: f64, horizontal_amount: f64, vertical_amount: f64) -> bool {
        // java: return getChildAt(mouseX, mouseY).filter(element -> element.mouseScrolled(mouseX, mouseY, horizontalAmount, verticalAmount)).isPresent();
        // - the search field (an `EditBox` does not scroll) or the widgets container (`AbstractScrollArea.mouseScrolled`)
        let _ = horizontal_amount;
        // (`WidgetsContainer.scrollRate()` is `SearchableGridWidget.this.scrollRate()` - the subclass's override)
        let __rate: f64 = { let __f = compat::dispatch(__this, |__t| __t.scroll_rate()); __f(__this) };
        let scrolled = compat::write(__this, |__t| {
            let __c = __t.searchable_grid_widget_base_mut().widgets_container.as_mut().unwrap();
            if !__c.visible || !__c.is_mouse_over(mouse_x, mouse_y) {
                return false;
            }
            __c.set_scroll_amount(__c.area.scroll_amount() - vertical_amount * __rate);
            true
        });
        if scrolled {
            <dyn SearchableGridWidget>::recreate_grid(__this);
        }
        scrolled
    }

    /// `AbstractScrollArea.scrollRate()` of the grid itself: its `defaultSettings(8)` rate
    pub fn scroll_rate_body(__this: &compat::Shared<dyn SearchableGridWidget>) -> f64 {
        compat::read(__this, |__t| __t.searchable_grid_widget_base().widgets_container.as_ref().map_or(8.0, |__c| __c.scroll_rate()))
    }

    /// java: @Override protected int contentHeight()
    pub fn content_height_body(__this: &compat::Shared<dyn SearchableGridWidget>) -> i32 {
        let _ = __this;
        return 0;
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    pub fn update_widget_narration_body(__this: &compat::Shared<dyn SearchableGridWidget>, builder: &dyn NarrationElementOutput) {
        let _ = (__this, builder);
    }

}

/// java: private class WidgetsContainer extends AbstractContainerWidget
///
/// (an inner class: what it reads of the enclosing `SearchableGridWidget` - `filteredWidgets`, `expectedWidgetWidth`,
/// `spaceElementsOut` - is handed in by the enclosing object's `recreate_grid`)
pub struct WidgetsContainer {
    /// java: extends net.minecraft.client.gui.components.AbstractContainerWidget
    pub base: AbstractContainerWidget,
    /// java: protected GridLayout grid = new GridLayout();
    pub grid: GridLayout,
    /// AbstractScrollArea's scroll state (`AbstractScrollArea.defaultSettings(8)`)
    pub area: ScrollArea,
}

impl std::ops::Deref for WidgetsContainer {
    type Target = AbstractContainerWidget;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for WidgetsContainer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl WidgetsContainer {
    /// java: private WidgetsContainer()
    pub fn new(width: i32, height: i32) -> Self {
        // java: super(0, 0, SearchableGridWidget.this.getWidth(), SearchableGridWidget.this.getHeight() - TEXT_FIELD_HEIGHT, Component.literal("Grid"), AbstractScrollArea.defaultSettings(8));
        WidgetsContainer {
            base: AbstractContainerWidget::new_with_scrollbar_settings(0, 0, width, height, FormattedText::from(component::literal("Grid")), ScrollbarSettings::default_settings(8.0)),
            grid: GridLayout::new_default(),
            area: ScrollArea::new(0, height, ScrollbarSettings::default_settings(8.0)),
        }
    }

    /// java: @Override public void setX(int x)
    pub fn set_x(&mut self, x: i32) {
        self.base.set_x(x);
        self.grid.x = x;
    }

    /// java: @Override public void setY(int y)
    pub fn set_y(&mut self, y: i32) {
        self.base.set_y(y);
        self.area.y = y;
        self.place_grid();
    }

    /// java: @Override protected int contentHeight()
    pub fn content_height(&self) -> i32 {
        return self.grid.height;
    }

    /// java: @Override protected double scrollRate() - `SearchableGridWidget.this.scrollRate()` (the outer's
    /// `defaultSettings(8)`)
    pub fn scroll_rate(&self) -> f64 {
        return self.area.scroll_rate();
    }

    /// java: @Override public void setScrollAmount(double scrollY)
    pub fn set_scroll_amount(&mut self, scroll_y: f64) {
        // java: super.setScrollAmount(scrollY); grid.setY(getY() - (int) scrollAmount());
        let content_height = self.content_height();
        self.area.set_scroll_amount(content_height, scroll_y);
        self.place_grid();
    }

    /// `grid.setY(getY() - (int) scrollAmount())`
    pub fn place_grid(&mut self) {
        self.grid.y = self.base.get_y() - self.area.scroll_amount() as i32;
    }

    /// java: private boolean isVisible(AbstractWidget widget)
    pub fn is_visible(&self, widget: &AbstractWidget) -> bool {
        return (widget.get_bottom() >= self.get_y() && widget.get_y() < self.get_bottom());
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    pub fn update_widget_narration(&self, _builder: &dyn NarrationElementOutput) {
    }

    /// java: protected void recreateGrid()
    ///
    /// `sizes`: the filtered widgets (their indices in the enclosing widget's `children`, and sizes)
    pub fn recreate_grid(&mut self, expected_widget_width: i32, space_elements_out: bool, sizes: &[(usize, i32, i32)]) {
        let mut new_grid: GridLayout = GridLayout::new_default();
        // java: int columns = (getWidth() - AbstractScrollArea.SCROLLBAR_WIDTH) / expectedWidgetWidth;
        let columns: i32 = (self.get_width() - SCROLLBAR_WIDTH) / expected_widget_width;
        let mut adder: RowHelper = new_grid.create_row_helper(columns);
        // java: filteredWidgets.forEach(adder::addChild);
        for (index, width, height) in sizes.iter().copied() {
            let _ = adder.add_child(&mut new_grid, LayoutElement::widget(index, width, height));
        }
        if space_elements_out {
            // java: newGrid.columnSpacing(((getWidth() - AbstractScrollArea.SCROLLBAR_WIDTH) - columns * expectedWidgetWidth) / columns);
            new_grid.column_spacing(((self.get_width() - SCROLLBAR_WIDTH) - columns * expected_widget_width) / columns);
        }
        // java: newGrid.arrangeElements(); newGrid.setPosition(grid.getX(), grid.getY()); grid = newGrid;
        new_grid.x = self.grid.x;
        new_grid.y = self.grid.y;
        new_grid.arrange_elements();
        self.grid = new_grid;
        // java: widgetsContainer.refreshScrollAmount();
        let content_height = self.content_height();
        self.area.refresh_scroll_amount(content_height);
        self.place_grid();
        self.grid.arrange_elements();
    }

}

/// A `SearchableGridWidget` subclass as the game's widget (`addRenderableWidget(grid)`, a layout's `addChild`): the
/// box the game's layouts move reaches the object through its `setX` / `setY` (the class's own), and the game's
/// calls reach the class's `extractWidgetRenderState` and the children (`extract_children`, `mouse_clicked`, ..).
pub struct SearchableGridSlot {
    object: compat::RenderThread<compat::Shared<dyn SearchableGridWidget>>,
    /// the game's x and y as of the last call
    snap: std::cell::Cell<(i32, i32)>,
}

impl SearchableGridSlot {
    /// The game's widget over `object`.
    pub fn widget(object: compat::Shared<dyn SearchableGridWidget>) -> AbstractWidget {
        let (x, y, w, h) = compat::read(&object, |__t| { let __b = __t.searchable_grid_widget_base(); (__b.get_x(), __b.get_y(), __b.get_width(), __b.get_height()) });
        let slot = SearchableGridSlot { object: compat::RenderThread::new(object), snap: std::cell::Cell::new((x, y)) };
        mcrust::mcrust::modding::widgets::ModWidgetSlot::widget(x, y, w, h, "SearchableGridWidget", Box::new(slot))
    }

    /// what the game changed (a layout moving the widget) through the object's `setX` / `setY`; the pass's hover
    fn sync(&self, widget: &AbstractWidget) {
        let object = self.object.get();
        let (sx, sy) = self.snap.get();
        if widget.get_x() != sx {
            let __f = compat::dispatch(object, |__t| __t.set_x());
            __f(object, widget.get_x());
        }
        if widget.get_y() != sy {
            let __f = compat::dispatch(object, |__t| __t.set_y());
            __f(object, widget.get_y());
        }
        self.snap.set((widget.get_x(), widget.get_y()));
        let hovered = widget.is_hovered();
        compat::write(object, |__t| __t.searchable_grid_widget_base_mut().base.is_hovered = hovered);
    }

    fn visible(&self) -> bool {
        compat::read(self.object.get(), |__t| __t.searchable_grid_widget_base().base.visible)
    }
}

impl mcrust::mcrust::modding::widgets::ModWidget for SearchableGridSlot {
    /// AbstractWidget.extractRenderState - `if (visible)`: the class's `extractWidgetRenderState`, then the children
    fn extract_widget_render_state(&mut self, widget: &AbstractWidget, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.sync(widget);
        if !self.visible() {
            return;
        }
        let object = self.object.get();
        let __f = compat::dispatch(object, |__t| __t.extract_widget_render_state());
        __f(object, &context.graphics, mouse_x, mouse_y, a);
        <dyn SearchableGridWidget>::extract_children(object, context, mouse_x, mouse_y);
    }

    /// the class's `updateWidgetNarration`
    fn update_widget_narration(&mut self, _widget: &AbstractWidget, output: &mut dyn NarrationElementOutput) {
        let object = self.object.get();
        let __f = compat::dispatch(object, |__t| __t.update_widget_narration());
        __f(object, &*output);
    }

    /// AbstractContainerWidget.mouseClicked
    fn mouse_clicked(&mut self, widget: &mut AbstractWidget, event: &MouseButtonEvent, _double_click: bool) -> bool {
        self.sync(widget);
        if !self.visible() {
            return false;
        }
        <dyn SearchableGridWidget>::mouse_clicked(self.object.get(), event)
    }

    /// ContainerEventHandler.mouseReleased
    fn mouse_released(&mut self, widget: &mut AbstractWidget, event: &MouseButtonEvent) -> bool {
        self.sync(widget);
        <dyn SearchableGridWidget>::mouse_released(self.object.get(), event)
    }

    /// ContainerEventHandler.mouseDragged
    fn mouse_dragged(&mut self, widget: &mut AbstractWidget, event: &MouseButtonEvent, dx: f64, dy: f64) -> bool {
        self.sync(widget);
        <dyn SearchableGridWidget>::mouse_dragged(self.object.get(), event, dx, dy)
    }

    /// the class's `mouseScrolled`
    fn mouse_scrolled(&mut self, widget: &mut AbstractWidget, x: f64, y: f64, scroll_x: f64, scroll_y: f64) -> bool {
        self.sync(widget);
        if !self.visible() {
            return false;
        }
        let object = self.object.get();
        let __f = compat::dispatch(object, |__t| __t.mouse_scrolled());
        __f(object, x, y, scroll_x, scroll_y)
    }

    /// ContainerEventHandler.keyPressed
    fn key_pressed(&mut self, widget: &mut AbstractWidget, event: &KeyEvent) -> bool {
        self.sync(widget);
        <dyn SearchableGridWidget>::key_pressed(self.object.get(), event)
    }

    /// ContainerEventHandler.charTyped
    fn char_typed(&mut self, widget: &mut AbstractWidget, event: &CharacterEvent) -> bool {
        self.sync(widget);
        <dyn SearchableGridWidget>::char_typed(self.object.get(), event)
    }

    /// AbstractWidget.isMouseOver over the object's box
    fn is_mouse_over(&self, _widget: &AbstractWidget, mouse_x: f64, mouse_y: f64) -> bool {
        compat::read(self.object.get(), |__t| __t.searchable_grid_widget_base().base.is_mouse_over(mouse_x, mouse_y))
    }
}

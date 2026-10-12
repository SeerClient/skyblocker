//! Ported from de.hysky.skyblocker.utils.render.gui.SearchableGridWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::components::abstract_container_widget::AbstractContainerWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::edit_box::EditBox;
use mcrust::net::minecraft::client::gui::components::edit_box::Responder;
use mcrust::net::minecraft::client::gui::components::edit_box::ResponderLambda;
use mcrust::net::minecraft::client::gui::components::events::container_event_handler;
use mcrust::net::minecraft::client::gui::components::events::gui_event_listener::GuiEventListener;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::layouts::grid_layout::GridLayout;
use mcrust::net::minecraft::client::gui::layouts::grid_layout::RowHelper;
use mcrust::net::minecraft::client::gui::layouts::layout_element::LayoutElement;
use mcrust::net::minecraft::client::gui::layouts::linear_layout::LinearLayout;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::gui::navigation::screen_rectangle::ScreenRectangle;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::client::gui::components::renderable::Renderable as _;
use mcrust::net::minecraft::client::gui::layouts::abstract_layout::AbstractLayout as _;

/// java: private static final int TEXT_FIELD_HEIGHT = 20;
pub const TEXT_FIELD_HEIGHT: i32 = 20;

/// java: public abstract class SearchableGridWidget extends AbstractContainerWidget
#[derive(Clone)]
pub struct SearchableGridWidgetBase {
    /// java: extends net.minecraft.client.gui.components.AbstractContainerWidget
    pub base: AbstractContainerWidget,
    /// java: private final List<AbstractWidget> filteredWidgets = new ObjectArrayList<>();
    pub filtered_widgets: Vec<AbstractWidget>,
    /// java: private final LinearLayout layoutWidget = LinearLayout.vertical();
    pub layout_widget: LinearLayout,
    /// java: private final EditBox searchField;
    pub search_field: Option<EditBox>,
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
    pub fn new_in<__R>(mut x: i32, mut y: i32, mut width: i32, mut height: i32, mut message: MutableComponent, mut expected_widget_width: i32, mut max_per_row: i32, mut packed: bool, mut space_elements_out: bool, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn SearchableGridWidget>, __R)) -> (compat::Shared<dyn SearchableGridWidget>, __R) {
        // TODO(fabric2seer): the port's `new` ports this Java method with other parameters (splitter, x, y, width, height for Java's width, height, narration)
        //   java: searchField = new EditBox(Minecraft.getInstance().font, width, TEXT_FIELD_HEIGHT, Component.translatable("gui.recipebook.search_hint"));
        // TODO(fabric2seer): static `AbstractScrollArea.defaultSettings` has no Rust counterpart in the port
        //   java: super(x, y, width, height, message, AbstractScrollArea.defaultSettings(8));
        let (__this, __r): (compat::Shared<dyn SearchableGridWidget>, __R) = todo!("constructor");
        { let (__q0,) = (component::translatable("gui.recipebook.search_hint").with_style_format(ChatFormatting::Italic).with_style_format(ChatFormatting::Gray),); __this.lock().borrow_mut().searchable_grid_widget_base_mut().search_field.as_mut().unwrap().set_hint_component(__q0) };
        let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(&__this));
        { let (__q0,) = (Responder::Lambda(ResponderLambda::new(move |__a0| { let Some(__this_1) = __this_w1.get().upgrade() else { return Default::default(); }; { ({ let __f = compat::dispatch(&__this_1, |__t| __t.filter_internal()); __f(&__this_1, __a0) }); } })),); __this.lock().borrow_mut().searchable_grid_widget_base_mut().search_field.as_mut().unwrap().set_responder(__q0) };
        { let __v = expected_widget_width; compat::write(&__this, |__t| __t.searchable_grid_widget_base_mut().expected_widget_width = __v); }
        { let __v = max_per_row; compat::write(&__this, |__t| __t.searchable_grid_widget_base_mut().max_per_row = __v); }
        { let __v = packed; compat::write(&__this, |__t| __t.searchable_grid_widget_base_mut().packed = __v); }
        { let __v = space_elements_out; compat::write(&__this, |__t| __t.searchable_grid_widget_base_mut().space_elements_out = __v); }
        { let __v = Some(WidgetsContainer::new()); compat::write(&__this, |__t| __t.searchable_grid_widget_base_mut().widgets_container = __v); }
        let __r0 = compat::read(&__this, |__t| __t.searchable_grid_widget_base().search_field.clone());
        { let (__q0,) = (__r0.unwrap(),); __this.lock().borrow_mut().searchable_grid_widget_base_mut().layout_widget.add_child(__q0) };
        let __r0 = compat::read(&__this, |__t| __t.searchable_grid_widget_base().widgets_container.clone());
        { let (__q0,) = (__r0.unwrap(),); __this.lock().borrow_mut().searchable_grid_widget_base_mut().layout_widget.add_child(__q0) };
        __this.lock().borrow_mut().searchable_grid_widget_base_mut().layout_widget.arrange_elements();
        // TODO(fabric2seer): `LayoutElement::set_position` ports `setPosition`, but the port's `LinearLayout` holds no `LayoutElement` (no Deref, no `base` field) to call it on
        // rust: __this.lock().borrow_mut().searchable_grid_widget_base_mut().layout_widget.set_position(x, y);
        ({ let __f = compat::dispatch(&__this, |__t| __t.set_width()); __f(&__this, __this.lock().borrow().searchable_grid_widget_base().get_width()) });
        (__this, __r)
    }

    /// java: public SearchableGridWidget(int x, int y, int width, int height, Component message, int expectedWidgetWidth, int maxPerRow, boolean packed) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_8_in<__R>(mut x: i32, mut y: i32, mut width: i32, mut height: i32, mut message: MutableComponent, mut expected_widget_width: i32, mut max_per_row: i32, mut packed: bool, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn SearchableGridWidget>, __R)) -> (compat::Shared<dyn SearchableGridWidget>, __R) {
        let (__this, __r) = Self::new_in(x, y, width, height, message.clone(), expected_widget_width, max_per_row, packed, false, __wrap);
        (__this, __r)
    }

    /// java: public SearchableGridWidget(int x, int y, int width, int height, Component message, int expectedWidgetWidth, boolean packed) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_7_in<__R>(mut x: i32, mut y: i32, mut width: i32, mut height: i32, mut message: MutableComponent, mut expected_widget_width: i32, mut packed: bool, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn SearchableGridWidget>, __R)) -> (compat::Shared<dyn SearchableGridWidget>, __R) {
        let (__this, __r) = Self::new_8_in(x, y, width, height, message.clone(), expected_widget_width, i32::MAX, packed, __wrap);
        (__this, __r)
    }

    /// java: public SearchableGridWidget(int x, int y, int width, int height, Component message, int expectedWidgetWidth) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_6_in<__R>(mut x: i32, mut y: i32, mut width: i32, mut height: i32, mut message: MutableComponent, mut expected_widget_width: i32, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn SearchableGridWidget>, __R)) -> (compat::Shared<dyn SearchableGridWidget>, __R) {
        let (__this, __r) = Self::new_7_in(x, y, width, height, message.clone(), expected_widget_width, false, __wrap);
        (__this, __r)
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
    /// java: @Override public void setX(int x)
    pub fn set_x_body(__this: &compat::Shared<dyn SearchableGridWidget>, mut x: i32) {
        __this.lock().borrow_mut().searchable_grid_widget_base_mut().base.set_x(x);
        __this.lock().borrow_mut().searchable_grid_widget_base_mut().layout_widget.set_x(x);
    }

    /// java: @Override public void setY(int y)
    pub fn set_y_body(__this: &compat::Shared<dyn SearchableGridWidget>, mut y: i32) {
        __this.lock().borrow_mut().searchable_grid_widget_base_mut().base.set_y(y);
        __this.lock().borrow_mut().searchable_grid_widget_base_mut().layout_widget.set_y(y);
    }

    /// java: @Override public void setWidth(int width)
    pub fn set_width_body(__this: &compat::Shared<dyn SearchableGridWidget>, mut width: i32) {
        if compat::read(__this, |__t| __t.searchable_grid_widget_base().packed) {
            let mut per_row: i32 = ((width.wrapping_sub(mcrust::net::minecraft::client::gui::components::abstract_scroll_area::SCROLLBAR_WIDTH)).wrapping_div(compat::read(__this, |__t| __t.searchable_grid_widget_base().expected_widget_width))).min(compat::read(__this, |__t| __t.searchable_grid_widget_base().max_per_row));
            let mut new_width: i32 = (per_row.wrapping_mul(compat::read(__this, |__t| __t.searchable_grid_widget_base().expected_widget_width))).wrapping_add(mcrust::net::minecraft::client::gui::components::abstract_scroll_area::SCROLLBAR_WIDTH);
            ({ let __f = compat::dispatch(__this, |__t| __t.set_x()); __f(__this, (__this.lock().borrow().searchable_grid_widget_base().get_x()).wrapping_add((width.wrapping_sub(new_width)).wrapping_div(2))) });
            width = new_width;
        }
        __this.lock().borrow_mut().searchable_grid_widget_base_mut().base.set_width(width);
        __this.lock().borrow_mut().searchable_grid_widget_base_mut().search_field.as_mut().unwrap().set_width(width);
        __this.lock().borrow_mut().searchable_grid_widget_base_mut().widgets_container.as_mut().unwrap().set_width(width);
        __this.lock().borrow_mut().searchable_grid_widget_base_mut().layout_widget.arrange_elements();
    }

    /// java: @Override public void setHeight(int height)
    pub fn set_height_body(__this: &compat::Shared<dyn SearchableGridWidget>, mut height: i32) {
        __this.lock().borrow_mut().searchable_grid_widget_base_mut().base.set_height(height);
        let __h0 = height.wrapping_sub(TEXT_FIELD_HEIGHT);
        __this.lock().borrow_mut().searchable_grid_widget_base_mut().widgets_container.as_mut().unwrap().set_height(__h0);
        __this.lock().borrow_mut().searchable_grid_widget_base_mut().layout_widget.arrange_elements();
    }

    /// java: public void setSearch(String search)
    pub fn set_search_body(__this: &compat::Shared<dyn SearchableGridWidget>, search: &str) {
        // TODO(fabric2seer): `set_value` takes 2 argument(s) in the port (`splitter: &StringSplitter<W>, value: &str`), Java passes 1
        // rust: __this.lock().borrow_mut().searchable_grid_widget_base_mut().search_field.as_mut().unwrap().set_value(search);
    }

    /// java: public void refreshSearch()
    pub fn refresh_search_body(__this: &compat::Shared<dyn SearchableGridWidget>) {
        // TODO(fabric2seer): `set_value` takes 2 argument(s) in the port (`splitter: &StringSplitter<W>, value: &str`), Java passes 1
        // rust: { let (__q0,) = (&__this.lock().borrow().searchable_grid_widget_base().search_field.as_ref().unwrap().get_value(),); __this.lock().borrow_mut().searchable_grid_widget_base_mut().search_field.as_mut().unwrap().set_value(__q0) };
    }

    /// java: public void setScrollAmount(double amount)
    pub fn set_scroll_amount_body(__this: &compat::Shared<dyn SearchableGridWidget>, mut amount: f64) {
        __this.lock().borrow_mut().searchable_grid_widget_base_mut().widgets_container.as_mut().unwrap().set_scroll_amount(amount);
    }

    /// java: public double getScrollAmount()
    pub fn get_scroll_amount_body(__this: &compat::Shared<dyn SearchableGridWidget>) -> f64 {
        // TODO(fabric2seer): `ScrollArea::scroll_amount` ports `scrollAmount`, but the port's `AbstractContainerWidget` holds no `ScrollArea` (no Deref, no `base` field) to call it on
        // rust: return __this.lock().borrow().searchable_grid_widget_base().widgets_container.as_ref().unwrap().scroll_amount();
        return todo!("fabric2seer: see the TODO above");
    }

    /// java: public ScreenRectangle getGridRectangle()
    pub fn get_grid_rectangle_body(__this: &compat::Shared<dyn SearchableGridWidget>) -> ScreenRectangle {
        return __this.lock().borrow().searchable_grid_widget_base().widgets_container.as_ref().unwrap().get_rectangle();
    }

    /// java: private void filterInternal(String input)
    pub fn filter_internal_body(__this: &compat::Shared<dyn SearchableGridWidget>, input: &str) {
        let mut widgets: Vec<AbstractWidget> = ({ let __f = compat::dispatch(__this, |__t| __t.filter_widgets()); __f(__this, input.to_string()) });
        __this.lock().borrow_mut().searchable_grid_widget_base_mut().filtered_widgets.clear();
        let __h0 = widgets.iter().cloned();
        __this.lock().borrow_mut().searchable_grid_widget_base_mut().filtered_widgets.extend(__h0);
        __this.lock().borrow_mut().searchable_grid_widget_base_mut().widgets_container.as_mut().unwrap().recreate_grid();
    }

    /// java: @Override public List<? extends GuiEventListener> children()
    pub fn children_body(__this: &compat::Shared<dyn SearchableGridWidget>) -> Vec<Box<dyn GuiEventListener>> {
        return vec![compat::read(__this, |__t| __t.searchable_grid_widget_base().search_field.clone()).unwrap(), compat::read(__this, |__t| __t.searchable_grid_widget_base().widgets_container.clone()).unwrap()];
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor context, int mouseX, int mouseY, float deltaTicks)
    pub fn extract_widget_render_state_body(__this: &compat::Shared<dyn SearchableGridWidget>, context: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut delta_ticks: f32) {
        // TODO(fabric2seer): `extract_render_state` is a method of the port's trait `Renderable`, which the port's `EditBox` does not implement
        // rust: { let (__q0, __q1, __q2, __q3, __q4,) = (mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mouse_x, mouse_y, delta_ticks,); __this.lock().borrow_mut().searchable_grid_widget_base_mut().search_field.as_mut().unwrap().extract_render_state(__q0, __q1, __q2, __q3, __q4) };
        // TODO(fabric2seer): `extract_render_state` is a method of the port's trait `Renderable`, which the port's `AbstractContainerWidget` does not implement
        // rust: { let (__q0, __q1, __q2, __q3, __q4,) = (mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mouse_x, mouse_y, delta_ticks,); __this.lock().borrow_mut().searchable_grid_widget_base_mut().widgets_container.as_mut().unwrap().extract_render_state(__q0, __q1, __q2, __q3, __q4) };
    }

    /// java: @Override public boolean mouseScrolled(double mouseX, double mouseY, double horizontalAmount, double verticalAmount)
    pub fn mouse_scrolled_body(__this: &compat::Shared<dyn SearchableGridWidget>, mut mouse_x: f64, mut mouse_y: f64, mut horizontal_amount: f64, mut vertical_amount: f64) -> bool {
        // TODO(fabric2seer): Optional.mouseScrolled
        //   java: return getChildAt(mouseX, mouseY).filter(element -> element.mouseScrolled(mouseX, mouseY, horizontalAmount, verticalAmount)).isPresent();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override protected int contentHeight()
    pub fn content_height_body(__this: &compat::Shared<dyn SearchableGridWidget>) -> i32 {
        return 0;
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    pub fn update_widget_narration_body(__this: &compat::Shared<dyn SearchableGridWidget>, builder: &dyn NarrationElementOutput) {
    }

}

/// java: private class WidgetsContainer extends AbstractContainerWidget
#[derive(Clone)]
pub struct WidgetsContainer {
    /// java: extends net.minecraft.client.gui.components.AbstractContainerWidget
    pub base: AbstractContainerWidget,
    /// java: protected GridLayout grid = new GridLayout();
    pub grid: GridLayout,
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
    pub fn new() -> Self {
        // TODO(fabric2seer): `SearchableGridWidget.this` (the enclosing instance of an inner class)
        //   java: super(0, 0, SearchableGridWidget.this.getWidth(), SearchableGridWidget.this.getHeight() - TEXT_FIELD_HEIGHT, Component.literal("Grid"), AbstractScrollArea.defaultSettings(8));
        let mut this: Self = todo!("constructor");
        this
    }

    /// java: @Override public void setX(int x)
    pub fn set_x(&mut self, mut x: i32) {
        self.base.set_x(x);
        self.grid.set_x(x);
    }

    /// java: @Override public void setY(int y)
    pub fn set_y(&mut self, mut y: i32) {
        self.base.set_y(y);
        self.grid.set_y(y);
    }

    /// java: @Override public void setWidth(int width)
    pub fn set_width(&mut self, mut width: i32) {
        self.base.set_width(width);
        self.recreate_grid();
    }

    /// java: @Override public void setHeight(int height)
    pub fn set_height(&mut self, mut height: i32) {
        self.base.set_height(height);
        self.recreate_grid();
    }

    /// java: @Override public List<? extends GuiEventListener> children()
    pub fn children(&self) -> Vec<Box<dyn GuiEventListener>> {
        // TODO(fabric2seer): `filteredWidgets` is a field of the enclosing instance (SearchableGridWidget.this) - an inner class has no outer reference in Rust
        //   java: return filteredWidgets;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override protected int contentHeight()
    pub fn content_height(&self) -> i32 {
        return self.grid.get_height();
    }

    /// java: @Override protected double scrollRate()
    pub fn scroll_rate(&self) -> f64 {
        // TODO(fabric2seer): `SearchableGridWidget.this` (the enclosing instance of an inner class)
        //   java: return SearchableGridWidget.this.scrollRate();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public void setScrollAmount(double scrollY)
    pub fn set_scroll_amount(&mut self, mut scroll_y: f64) {
        // TODO(fabric2seer): `set_scroll_amount` takes 2 argument(s) in the port (`content_height: i32, scroll_amount: f64`), Java passes 1
        // TODO(fabric2seer): `ScrollArea::set_scroll_amount` ports `setScrollAmount`, but the port's `AbstractContainerWidget` holds no `ScrollArea` (no Deref, no `base` field) to call it on
        // rust: self.base.set_scroll_amount(scroll_y);
        // TODO(fabric2seer): `ScrollArea::scroll_amount` ports `scrollAmount`, but the port's `AbstractContainerWidget` holds no `ScrollArea` (no Deref, no `base` field) to call it on
        // rust: { let __s0 = self.get_y().wrapping_sub((self.scroll_amount()) as i32); self.grid.set_y(__s0) };
    }

    /// java: private boolean isVisible(AbstractWidget widget)
    pub fn is_visible(&self, widget: &AbstractWidget) -> bool {
        return (widget.get_bottom() >= self.get_y() && widget.get_y() < self.get_bottom());
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor context, int mouseX, int mouseY, float deltaTicks)
    pub fn extract_widget_render_state(&self, context: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut delta_ticks: f32) {
        context.enable_scissor(self.get_x(), self.get_y(), self.get_right(), self.get_bottom());
        // TODO(fabric2seer): `filteredWidgets` is a field of the enclosing instance (SearchableGridWidget.this) - an inner class has no outer reference in Rust
        //   java: for (AbstractWidget widget : filteredWidgets) {
        //   java: 				if (isVisible(widget)) widget.extractRenderState(context, mouseX, mouseY, deltaTicks);
        //   java: 			}
        // TODO(fabric2seer): `AbstractContainerWidget.extractScrollbar` has no Rust method in the port (looked for `extract_scrollbar` in AbstractContainerWidget and its supertypes)
        // rust: self.extract_scrollbar(context, mouse_x, mouse_y);
        context.disable_scissor();
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    pub fn update_widget_narration(&self, builder: &dyn NarrationElementOutput) {
    }

    /// java: protected void recreateGrid()
    pub fn recreate_grid(&mut self) {
        let mut new_grid: GridLayout = GridLayout::new_default();
        // TODO(fabric2seer): `expectedWidgetWidth` is a field of the enclosing instance (SearchableGridWidget.this) - an inner class has no outer reference in Rust
        //   java: int columns = (getWidth() - AbstractScrollArea.SCROLLBAR_WIDTH) / expectedWidgetWidth;
        let mut columns: i32 = todo!("fabric2seer: the initialiser of `columns` (TODO above)");
        let mut adder: RowHelper = new_grid.create_row_helper(columns);
        // TODO(fabric2seer): `filteredWidgets` is a field of the enclosing instance (SearchableGridWidget.this) - an inner class has no outer reference in Rust
        //   java: filteredWidgets.forEach(adder::addChild);
        // TODO(fabric2seer): the condition: `spaceElementsOut` is a field of the enclosing instance (SearchableGridWidget.this) - an inner class has no outer reference in Rust
        //   java: spaceElementsOut
        if todo!("fabric2seer: the condition (TODO above)") {
            // TODO(fabric2seer): `expectedWidgetWidth` is a field of the enclosing instance (SearchableGridWidget.this) - an inner class has no outer reference in Rust
            //   java: newGrid.columnSpacing(((getWidth() - AbstractScrollArea.SCROLLBAR_WIDTH) - columns * expectedWidgetWidth) / columns);
        }
        new_grid.arrange_elements();
        // TODO(fabric2seer): `LayoutElement::set_position` ports `setPosition`, but the port's `GridLayout` holds no `LayoutElement` (no Deref, no `base` field) to call it on
        // rust: new_grid.set_position(self.grid.get_x(), self.grid.get_y());
        self.grid = new_grid.clone();
        // TODO(fabric2seer): `widgetsContainer` is a field of the enclosing instance (SearchableGridWidget.this) - an inner class has no outer reference in Rust
        //   java: widgetsContainer.refreshScrollAmount();
    }

}

// NOTE(fabric2seer): children, contentHeight, extractWidgetRenderState, scrollRate, setHeight, setScrollAmount, setWidth, setX, setY, updateWidgetNarration override(s) of AbstractContainerWidget - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them

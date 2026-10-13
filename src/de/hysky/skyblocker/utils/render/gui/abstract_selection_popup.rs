//! Ported from de.hysky.skyblocker.utils.render.gui.AbstractSelectionPopup by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::render::gui::abstract_popup_screen::AbstractPopupScreen;
use crate::de::hysky::skyblocker::utils::render::gui::abstract_popup_screen::AbstractPopupScreenBase;
use crate::de::hysky::skyblocker::utils::render::gui::searchable_grid_widget::SearchableGridSlot;
use crate::de::hysky::skyblocker::utils::render::gui::searchable_grid_widget::SearchableGridWidget;
use crate::de::hysky::skyblocker::utils::render::gui::searchable_grid_widget::SearchableGridWidgetBase;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::PressLambda;
use mcrust::net::minecraft::client::gui::components::button;
use mcrust::net::minecraft::client::gui::layouts::grid_layout::GridLayout;
use mcrust::net::minecraft::client::gui::layouts::layout_element::LayoutElement;
use mcrust::net::minecraft::client::gui::layouts::abstract_layout::AbstractLayout as _;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// `W extends AbstractWidget`: a selection popup's widget as the game's widget (the grid holds the game's widget
/// over the same object).
pub trait SelectionWidget: Clone + 'static {
    fn game_widget(&self) -> AbstractWidget;
}

/// java: public abstract class AbstractSelectionPopup<W extends AbstractWidget> extends AbstractPopupScreen
pub struct AbstractSelectionPopupBase<W> {
    /// java: extends AbstractPopupScreen
    pub base: AbstractPopupScreenBase,
    /// java: private final Consumer<Optional<W>> onDone;
    pub on_done: Arc<dyn Fn(Option<W>)>,
    /// java: private final int expectedWidgetWidth;
    pub expected_widget_width: i32,
    /// java: protected @Nullable W selectedItem = null;
    pub selected_item: Option<W>,
    /// java: private final GridLayout gridWidget = new GridLayout();
    pub grid_widget: GridLayout,
    /// java: private @Nullable Button doneButton;
    pub done_button: Option<compat::WidgetRef>,
}

impl<W: 'static> std::ops::Deref for AbstractSelectionPopupBase<W> {
    type Target = AbstractPopupScreenBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<W: 'static> std::ops::DerefMut for AbstractSelectionPopupBase<W> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl<W: Clone + 'static> AbstractSelectionPopupBase<W> {
    /// java: protected AbstractSelectionPopup(Component title, Screen backgroundScreen, Consumer<Optional<W>> onDone, int expectedWidgetWidth) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_in<__R>(title: MutableComponent, background_screen: mcrust::net::minecraft::client::gui::screens::screen::Screen, on_done: Arc<dyn Fn(Option<W>)>, expected_widget_width: i32, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn AbstractSelectionPopup<W>>, __R)) -> (compat::Shared<dyn AbstractSelectionPopup<W>>, __R) {
        // java: super(title, backgroundScreen);
        let this = Self {
            base: AbstractPopupScreenBase::new_base(title, background_screen),
            on_done,
            expected_widget_width,
            selected_item: None,
            grid_widget: GridLayout::new_default(),
            done_button: None,
        };
        __wrap(this)
    }

}

/// java: the virtual methods of AbstractSelectionPopup (implemented by AbstractSelectionPopupBase and every subclass)
pub trait AbstractSelectionPopup<W>: AbstractPopupScreen {
    fn abstract_selection_popup_base(&self) -> &AbstractSelectionPopupBase<W>;
    fn abstract_selection_popup_base_mut(&mut self) -> &mut AbstractSelectionPopupBase<W>;

    /// java: protected void setSelectedItem(W selectedItem)
    fn set_selected_item(&mut self, selected_item: W) {
        self.abstract_selection_popup_base_mut().selected_item = Some(selected_item);
        // java: if (doneButton != null) doneButton.active = true; - the button is the screen's widget once added
        let __s = self.abstract_selection_popup_base_mut();
        if let Some(__r) = __s.done_button.as_mut() {
            __s.base.__widgets.with(&mut __s.base.base, __r, |__w| __w.active = true);
        }
    }

    /// java: protected abstract Collection<W> filterWidgets(String input);
    fn filter_widgets(&self, input: &str) -> Vec<W>;

}

impl<W: SelectionWidget> dyn AbstractSelectionPopup<W> {
    /// java: @Override public void init() - AbstractSelectionPopup's body, as the dispatch of AbstractPopupScreen.init
    /// for the concrete class `__C`
    pub fn init_from<__C: AbstractSelectionPopup<W> + 'static>(__this: &compat::Shared<dyn AbstractPopupScreen>) {
        let __concrete = compat::view_as::<__C, _, __C>(__this, |a| a);
        let __outer: compat::WeakShared<__C> = std::sync::Arc::downgrade(&*__concrete);
        let (height, expected_widget_width) = compat::read(&__concrete, |__t| (__t.abstract_popup_screen_base().height, __t.abstract_selection_popup_base().expected_widget_width));
        compat::write(&__concrete, |__t| __t.abstract_selection_popup_base_mut().grid_widget = GridLayout::new_default());
        // java: GridLayout.RowHelper adder = gridWidget.createRowHelper(2);
        let mut adder = compat::read(&__concrete, |__t| __t.abstract_selection_popup_base().grid_widget.create_row_helper(2));
        // java: adder.defaultCellSetting().alignHorizontallyCenter();
        compat::write(&__concrete, |__t| { let __s = __t.abstract_selection_popup_base_mut().grid_widget.default_cell_setting(); *__s = (*__s).align_horizontally_center(); });
        // java: addRenderableWidget(adder.addChild(new ItemList(326, (int) (height * 0.8f)), 2));
        let __filter_outer = compat::RenderThread::new(__outer.clone());
        let item_list = ItemList::new(326, (height as f32 * 0.8f32) as i32, expected_widget_width, Arc::new(move |input: String| {
            // java: return AbstractSelectionPopup.this.filterWidgets(input);
            let Some(outer) = __filter_outer.get().upgrade() else { return Vec::new(); };
            let widgets: Vec<W> = compat::read(&outer, |__t| __t.filter_widgets(&input));
            widgets.iter().map(|__w| __w.game_widget()).collect()
        }));
        let __list_widget = SearchableGridSlot::widget(item_list as compat::Shared<dyn SearchableGridWidget>);
        let (__lw, __lh) = (__list_widget.get_width(), __list_widget.get_height());
        let __list_index = compat::write(&__concrete, |__t| __t.abstract_popup_screen_base_mut().add_renderable_widget(__list_widget));
        compat::write(&__concrete, |__t| { let __g = &mut __t.abstract_selection_popup_base_mut().grid_widget; let _ = adder.add_child_width(__g, LayoutElement::widget(__list_index, __lw, __lh), 2); });
        // java: addRenderableWidget(adder.addChild(Button.builder(CommonComponents.GUI_CANCEL, _ -> { onClose(); onDone.accept(Optional.empty()); }).build()));
        let __cancel_outer = compat::RenderThread::new(__outer.clone());
        let __cancel = button::builder(mcrust::net::minecraft::network::chat::common_components::gui_cancel().get_string(), OnPress::Lambda(PressLambda::new(move |_: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, _: usize| {
            let Some(outer) = __cancel_outer.get().upgrade() else { return; };
            let __handle: compat::Shared<dyn AbstractPopupScreen> = outer.clone();
            let __f = compat::dispatch(&__handle, |__t| __t.on_close());
            __f(&__handle);
            let on_done = compat::read(&outer, |__t| __t.abstract_selection_popup_base().on_done.clone());
            on_done(None);
        }))).build();
        let (__cw, __ch) = (__cancel.get_width(), __cancel.get_height());
        let __cancel_index = compat::write(&__concrete, |__t| __t.abstract_popup_screen_base_mut().add_renderable_widget(__cancel));
        compat::write(&__concrete, |__t| { let __g = &mut __t.abstract_selection_popup_base_mut().grid_widget; let _ = adder.add_child(__g, LayoutElement::widget(__cancel_index, __cw, __ch)); });
        // java: doneButton = Button.builder(CommonComponents.GUI_DONE, _ -> { onClose(); onDone.accept(Optional.ofNullable(selectedItem)); }).build();
        let __done_outer = compat::RenderThread::new(__outer.clone());
        let mut __done = button::builder(mcrust::net::minecraft::network::chat::common_components::gui_done().get_string(), OnPress::Lambda(PressLambda::new(move |_: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, _: usize| {
            let Some(outer) = __done_outer.get().upgrade() else { return; };
            let __handle: compat::Shared<dyn AbstractPopupScreen> = outer.clone();
            let __f = compat::dispatch(&__handle, |__t| __t.on_close());
            __f(&__handle);
            let (on_done, selected) = compat::read(&outer, |__t| (__t.abstract_selection_popup_base().on_done.clone(), __t.abstract_selection_popup_base().selected_item.clone()));
            on_done(selected);
        }))).build();
        // java: doneButton.active = false;
        __done.active = false;
        // java: addRenderableWidget(adder.addChild(doneButton));
        let mut __done_ref = compat::WidgetRef::value(__done);
        let (__di, __dw, __dh) = compat::write(&__concrete, |__t| { let __s = __t.abstract_popup_screen_base_mut(); let __i = __s.__widgets.add(&mut __s.base, &mut __done_ref, true); let __w = __s.base.widget(__i); (__i, __w.get_width(), __w.get_height()) });
        compat::write(&__concrete, |__t| { let __b = __t.abstract_selection_popup_base_mut(); __b.done_button = Some(__done_ref); let _ = adder.add_child(&mut __b.grid_widget, LayoutElement::widget(__di, __dw, __dh)); });
        // java: gridWidget.arrangeElements(); repositionElements();
        compat::write(&__concrete, |__t| __t.abstract_selection_popup_base_mut().grid_widget.arrange_elements());
        let __f = compat::dispatch(__this, |__t| __t.reposition_elements());
        __f(__this);
    }

    /// java: @Override public void extractBackground(GuiGraphicsExtractor context, int mouseX, int mouseY, float delta) -
    /// AbstractSelectionPopup's body, as the dispatch of AbstractPopupScreen.extractBackground for the concrete class `__C`
    pub fn extract_background_from<__C: AbstractSelectionPopup<W> + 'static>(__this: &compat::Shared<dyn AbstractPopupScreen>, context: &mut GuiGraphicsExtractor, mouse_x: i32, mouse_y: i32, delta: f32) {
        // java: super.extractBackground(context, mouseX, mouseY, delta);
        <dyn AbstractPopupScreen>::extract_background_body(__this, context, mouse_x, mouse_y, delta);
        // java: extractPopupBackground(context, gridWidget.getX(), gridWidget.getY(), gridWidget.getWidth(), gridWidget.getHeight());
        let __concrete = compat::view_as::<__C, _, __C>(__this, |a| a);
        let (x, y, width, height) = compat::read(&__concrete, |__t| {
            let grid_widget = &__t.abstract_selection_popup_base().grid_widget;
            (grid_widget.get_x(), grid_widget.get_y(), grid_widget.get_width(), grid_widget.get_height())
        });
        AbstractPopupScreenBase::extract_popup_background(context, x, y, width, height);
    }

    /// java: @Override protected void repositionElements() - AbstractSelectionPopup's body, as the dispatch of
    /// AbstractPopupScreen.repositionElements for the concrete class `__C`
    pub fn reposition_elements_from<__C: AbstractSelectionPopup<W> + 'static>(__this: &compat::Shared<dyn AbstractPopupScreen>) {
        // java: super.repositionElements();
        <dyn AbstractPopupScreen>::reposition_elements_body(__this);
        // java: gridWidget.setPosition((width - gridWidget.getWidth()) / 2, (height - gridWidget.getHeight()) / 2);
        // (the grid's widgets are the screen's: they take the places the grid gives them)
        let __concrete = compat::view_as::<__C, _, __C>(__this, |a| a);
        compat::write(&__concrete, |__t| {
            let (width, height) = (__t.abstract_popup_screen_base().width, __t.abstract_popup_screen_base().height);
            let grid_widget = &mut __t.abstract_selection_popup_base_mut().grid_widget;
            grid_widget.x = (width.wrapping_sub(grid_widget.get_width())).wrapping_div(2);
            grid_widget.y = (height.wrapping_sub(grid_widget.get_height())).wrapping_div(2);
            grid_widget.arrange_elements();
            let mut placed: Vec<(usize, i32, i32)> = Vec::new();
            LayoutElement::Grid(grid_widget.clone()).visit_widgets(&mut placed);
            let screen = &mut __t.abstract_popup_screen_base_mut().base;
            for (__i, __x, __y) in placed {
                if __i < screen.children().len() {
                    screen.widget_mut(__i).set_position(__x, __y);
                }
            }
        });
    }
}

// AbstractSelectionPopup.init implements AbstractPopupScreen.init: it is emitted in each concrete subclass's `impl AbstractPopupScreen` (Rust has no inherited impls)
// AbstractSelectionPopup.extractBackground implements AbstractPopupScreen.extractBackground: it is emitted in each concrete subclass's `impl AbstractPopupScreen` (Rust has no inherited impls)
// AbstractSelectionPopup.repositionElements implements AbstractPopupScreen.repositionElements: it is emitted in each concrete subclass's `impl AbstractPopupScreen` (Rust has no inherited impls)
/// java: private class ItemList extends SearchableGridWidget
///
/// (an inner class: `AbstractSelectionPopup.this.filterWidgets(input)` is the `filter` the popup hands it)
pub struct ItemList {
    /// java: extends SearchableGridWidget
    pub base: SearchableGridWidgetBase,
    /// `AbstractSelectionPopup.this.filterWidgets(input)`, as the game's widgets
    pub filter: Arc<dyn Fn(String) -> Vec<AbstractWidget>>,
}

impl std::ops::Deref for ItemList {
    type Target = SearchableGridWidgetBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for ItemList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl ItemList {
    /// java: private ItemList(int width, int height)
    pub fn new(width: i32, height: i32, expected_widget_width: i32, filter: Arc<dyn Fn(String) -> Vec<AbstractWidget>>) -> compat::Shared<Self> {
        // java: super(0, 0, width, height, Component.literal("Item List"), expectedWidgetWidth, true);
        let (_, __this) = SearchableGridWidgetBase::new_7_in(0, 0, width, height, component::literal("Item List"), expected_widget_width, true, move |mut __base| {
            // java: @Override protected double scrollRate() { return 15; }
            if let Some(__c) = __base.widgets_container.as_mut() {
                __c.area.settings.scroll_rate = ItemList::scroll_rate();
            }
            let __h = compat::shared(Self { base: __base, filter });
            ((__h.clone() as compat::Shared<dyn SearchableGridWidget>), __h)
        });
        // java: setSearch("");
        ({ let __f = compat::dispatch(&__this, |__t| __t.set_search()); __f(&compat::view(&__this, |a| a as compat::Shared<dyn SearchableGridWidget>), "") });
        __this
    }

    /// java: @Override protected double scrollRate()
    pub fn scroll_rate() -> f64 {
        return 15.0;
    }

}

impl SearchableGridWidget for ItemList {
    fn searchable_grid_widget_base(&self) -> &SearchableGridWidgetBase {
        &self.base
    }
    fn searchable_grid_widget_base_mut(&mut self) -> &mut SearchableGridWidgetBase {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override protected Collection<? extends AbstractWidget> filterWidgets(String input) (dispatch: the body for this class)
    fn filter_widgets(&self) -> fn(&compat::Shared<dyn SearchableGridWidget>, String) -> Vec<AbstractWidget> {
        ItemList::filter_widgets_dyn
    }

}

impl ItemList {
    /// java: @Override protected Collection<? extends AbstractWidget> filterWidgets(String input)
    pub fn filter_widgets_impl(__this: &compat::Shared<Self>, input: String) -> Vec<AbstractWidget> {
        // java: return AbstractSelectionPopup.this.filterWidgets(input);
        let filter = compat::read(__this, |__t| __t.filter.clone());
        filter(input)
    }

    /// ItemList.filterWidgets as the dispatch of SearchableGridWidget.filterWidgets
    pub fn filter_widgets_dyn(__this: &compat::Shared<dyn SearchableGridWidget>, __p0: String) -> Vec<AbstractWidget> {
        Self::filter_widgets_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

}

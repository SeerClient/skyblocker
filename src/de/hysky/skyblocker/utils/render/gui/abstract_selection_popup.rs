//! Ported from de.hysky.skyblocker.utils.render.gui.AbstractSelectionPopup by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::render::gui::abstract_popup_screen::AbstractPopupScreen;
use crate::de::hysky::skyblocker::utils::render::gui::abstract_popup_screen::AbstractPopupScreenBase;
use crate::de::hysky::skyblocker::utils::render::gui::searchable_grid_widget::SearchableGridWidget;
use crate::de::hysky::skyblocker::utils::render::gui::searchable_grid_widget::SearchableGridWidgetBase;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::layouts::grid_layout::GridLayout;
use mcrust::net::minecraft::client::gui::layouts::abstract_layout::AbstractLayout as _;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

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
    pub fn new_in<__R>(mut title: MutableComponent, mut background_screen: mcrust::net::minecraft::client::gui::screens::screen::Screen, on_done: Arc<dyn Fn(Option<W>)>, mut expected_widget_width: i32, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn AbstractSelectionPopup<W>>, __R)) -> (compat::Shared<dyn AbstractSelectionPopup<W>>, __R) {
        let __init_on_done = on_done.clone();
        let __init_expected_widget_width = expected_widget_width;
        // TODO(fabric2seer): the mod keeps the game's `Screen` (Java holds a reference to the live object); the port's `Screen` is not a value to copy and the port lends the mod no handle to it
        //   java: super(title, backgroundScreen);
        let (__this, __r): (compat::Shared<dyn AbstractSelectionPopup<W>>, __R) = todo!("constructor");
        (__this, __r)
    }

}

/// java: the virtual methods of AbstractSelectionPopup (implemented by AbstractSelectionPopupBase and every subclass)
pub trait AbstractSelectionPopup<W>: AbstractPopupScreen {
    fn abstract_selection_popup_base(&self) -> &AbstractSelectionPopupBase<W>;
    fn abstract_selection_popup_base_mut(&mut self) -> &mut AbstractSelectionPopupBase<W>;

    /// java: protected void setSelectedItem(W selectedItem)
    fn set_selected_item(&mut self, mut selected_item: W) {
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

impl<W: 'static> dyn AbstractSelectionPopup<W> {
    /// java: @Override public void init() - AbstractSelectionPopup's body, as the dispatch of AbstractPopupScreen.init
    /// for the concrete class `__C`
    pub fn init_from<__C: AbstractSelectionPopup<W> + 'static>(__this: &compat::Shared<dyn AbstractPopupScreen>) {
        let _ = compat::view_as::<__C, _, __C>(__this, |a| a);
        // java: GridLayout.RowHelper adder = gridWidget.createRowHelper(2);
        //       adder.defaultCellSetting().alignHorizontallyCenter();
        //       addRenderableWidget(adder.addChild(new ItemList(326, (int) (height * 0.8f)), 2));
        //       addRenderableWidget(adder.addChild(Button.builder(CommonComponents.GUI_CANCEL, ..).build()));
        //       doneButton = Button.builder(CommonComponents.GUI_DONE, ..).build(); doneButton.active = false;
        //       addRenderableWidget(adder.addChild(doneButton)); gridWidget.arrangeElements(); repositionElements();
        // rust: port gap - `ItemList` is an inner (non-static) class whose constructor reads the enclosing popup's
        // `expectedWidgetWidth` and whose `filterWidgets` calls `AbstractSelectionPopup.this.filterWidgets`; the
        // converted `ItemList` has no handle to its enclosing popup (GAPS_batchA.md)
        todo!("rust: port gap - AbstractSelectionPopup.init needs ItemList's enclosing-instance handle")
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
        // (`LayoutElement.setPosition`'s default body: setX(x); setY(y))
        let __concrete = compat::view_as::<__C, _, __C>(__this, |a| a);
        compat::write(&__concrete, |__t| {
            let (width, height) = (__t.abstract_popup_screen_base().width, __t.abstract_popup_screen_base().height);
            let grid_widget = &mut __t.abstract_selection_popup_base_mut().grid_widget;
            let x = (width.wrapping_sub(grid_widget.get_width())).wrapping_div(2);
            let y = (height.wrapping_sub(grid_widget.get_height())).wrapping_div(2);
            grid_widget.set_x(x);
            grid_widget.set_y(y);
        });
    }
}

// AbstractSelectionPopup.init implements AbstractPopupScreen.init: it is emitted in each concrete subclass's `impl AbstractPopupScreen` (Rust has no inherited impls)
// AbstractSelectionPopup.extractBackground implements AbstractPopupScreen.extractBackground: it is emitted in each concrete subclass's `impl AbstractPopupScreen` (Rust has no inherited impls)
// AbstractSelectionPopup.repositionElements implements AbstractPopupScreen.repositionElements: it is emitted in each concrete subclass's `impl AbstractPopupScreen` (Rust has no inherited impls)
/// java: private class ItemList extends SearchableGridWidget
#[derive(Clone)]
pub struct ItemList {
    /// java: extends SearchableGridWidget
    pub base: SearchableGridWidgetBase,
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
    pub fn new(mut width: i32, mut height: i32) -> compat::Shared<Self> {
        // TODO(fabric2seer): instance field `expectedWidgetWidth` read in a static context
        //   java: super(0, 0, width, height, Component.literal("Item List"), expectedWidgetWidth, true);
        let __this: compat::Shared<Self> = todo!("constructor");
        ({ let __f = compat::dispatch(&__this, |__t| __t.set_search()); __f(&compat::view(&__this, |a| a as compat::Shared<dyn SearchableGridWidget>), "") });
        __this
    }

    /// java: @Override protected double scrollRate()
    pub fn scroll_rate(__this: &compat::Shared<Self>) -> f64 {
        return ((15) as f64) as f64;
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
    pub fn filter_widgets_impl(__this: &compat::Shared<Self>, mut input: String) -> Vec<AbstractWidget> {
        // TODO(fabric2seer): `AbstractSelectionPopup.this` (the enclosing instance of an inner class)
        //   java: return AbstractSelectionPopup.this.filterWidgets(input);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// ItemList.filterWidgets as the dispatch of SearchableGridWidget.filterWidgets
    pub fn filter_widgets_dyn(__this: &compat::Shared<dyn SearchableGridWidget>, __p0: String) -> Vec<AbstractWidget> {
        Self::filter_widgets_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

}

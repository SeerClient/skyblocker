//! Ported from de.hysky.skyblocker.utils.render.gui.AbstractSelectionPopup by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::render::gui::abstract_popup_screen::AbstractPopupScreen;
use crate::de::hysky::skyblocker::utils::render::gui::abstract_popup_screen::AbstractPopupScreenBase;
use crate::de::hysky::skyblocker::utils::render::gui::searchable_grid_widget::SearchableGridWidget;
use crate::de::hysky::skyblocker::utils::render::gui::searchable_grid_widget::SearchableGridWidgetBase;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::layouts::grid_layout::GridLayout;
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
    pub fn new_in<__R>(mut title: MutableComponent, mut background_screen: mcrust::net::minecraft::client::gui::screens::screen::Screen, on_done: Arc<dyn Fn(Option<W>)>, mut expected_widget_width: i32, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn AbstractSelectionPopup>, __R)) -> (compat::Shared<dyn AbstractSelectionPopup>, __R) {
        let __init_on_done = on_done.clone();
        let __init_expected_widget_width = expected_widget_width;
        // TODO(fabric2seer): the mod keeps the game's `Screen` (Java holds a reference to the live object); the port's `Screen` is not a value to copy and the port lends the mod no handle to it
        //   java: super(title, backgroundScreen);
        let (__this, __r): (compat::Shared<dyn AbstractSelectionPopup>, __R) = todo!("constructor");
        (__this, __r)
    }

}

/// java: the virtual methods of AbstractSelectionPopup (implemented by AbstractSelectionPopupBase and every subclass)
pub trait AbstractSelectionPopup<W>: AbstractPopupScreen {
    fn abstract_selection_popup_base(&self) -> &AbstractSelectionPopupBase<W>;
    fn abstract_selection_popup_base_mut(&mut self) -> &mut AbstractSelectionPopupBase<W>;

    /// java: protected void setSelectedItem(W selectedItem)
    fn set_selected_item(&mut self, mut selected_item: W) {
        self.abstract_selection_popup_base_mut().selected_item = Some(selected_item.clone());
        if self.abstract_selection_popup_base().done_button.is_some() {
            self.abstract_selection_popup_base_mut().done_button.as_mut().unwrap().active = true;
        }
    }

    /// java: protected abstract Collection<W> filterWidgets(String input);
    fn filter_widgets(&self, input: &str) -> Vec<W>;

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

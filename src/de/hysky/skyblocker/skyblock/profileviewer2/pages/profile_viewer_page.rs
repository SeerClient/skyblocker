//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.pages.ProfileViewerPage by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::loading_information::LoadingInformation;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::layouts::layout_element::LayoutElement;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public sealed interface ProfileViewerPage<T> permits CatacombsPage, CollectionsPage, InventoryPage, MiningPage, SkillsPage, CombatPage
/// - the members that do not name `T`: what a `ProfileViewerPage<?>` (the screen's `List<ProfileViewerPage<?>>`) answers
pub trait ProfileViewerPageAny: compat::DynAny {
    fn as_any(&self) -> &dyn std::any::Any;
    /// java: FlexibleItemStack getIcon();
    fn get_icon(&self) -> compat::Shared<FlexibleItemStack>;

    /// java: Component getName();
    fn get_name(&self) -> MutableComponent;

    /// java: CompletableFuture<LayoutElement> load(LoadingInformation info); (dispatch: the body for this class - it
    /// hands `this::buildWidgets` on, so it runs on the page's handle)
    fn load(&self) -> fn(&compat::Shared<dyn ProfileViewerPageAny>, &LoadingInformation) -> compat::LocalFuture<LayoutElement>;

    /// java: List<AbstractWidget> getWidgets();
    fn get_widgets(&self) -> Vec<AbstractWidget>;

    /// The list `getWidgets()` answers, by reference: the page's widgets are held in a `Screen` (the port's holder of
    /// game widgets, as a container row keeps its children), so the screen moves them where the content layout put
    /// them (Java's layout moves the widget objects themselves), draws them, and hands them their clicks - a press
    /// lambda of one of them is lent this list as its `screen`.
    fn widgets_mut(&mut self) -> &mut mcrust::net::minecraft::client::gui::screens::screen::Screen;

    /// java: default boolean centred()
    fn centred(&self) -> bool {
        return false;
    }

}

/// java: public sealed interface ProfileViewerPage<T> - `buildWidgets(T)`, the member that names `T`
pub trait ProfileViewerPage<T>: ProfileViewerPageAny {
    /// java: LayoutElement buildWidgets(T data);
    fn build_widgets(&self, data: &T) -> LayoutElement;

}

thread_local! {
    /// The widgets of the page whose `buildWidgets` is running (`layout.visitWidgets(widgets::add)`).
    static PAGE_WIDGETS: std::cell::RefCell<Vec<mcrust::net::minecraft::client::gui::screens::screen::Screen>> = const { std::cell::RefCell::new(Vec::new()) };
}

/// `buildWidgets(data)` with the widgets its layout holds collected: Java's `layout.visitWidgets(widgets::add)` at the
/// end of every `buildWidgets`. The port's layouts hold a widget as an index (`LayoutElement::Widget`); a widget a
/// page puts in its layout (`layout_widget`) is the next entry of the page's list, so `getWidgets()` holds every
/// widget the layout does, in the order they were put in it.
pub fn collect_widgets<R>(build: impl FnOnce() -> R) -> (R, mcrust::net::minecraft::client::gui::screens::screen::Screen) {
    PAGE_WIDGETS.with(|w| w.borrow_mut().push(mcrust::net::minecraft::client::gui::screens::screen::Screen::new(String::new())));
    struct Pop;
    impl Drop for Pop {
        fn drop(&mut self) {
            PAGE_WIDGETS.with(|w| w.borrow_mut().pop());
        }
    }
    let built = {
        let __pop_on_unwind = Pop;
        let built = build();
        std::mem::forget(__pop_on_unwind);
        built
    };
    let widgets = PAGE_WIDGETS.with(|w| w.borrow_mut().pop()).unwrap_or_default();
    (built, widgets)
}

/// The list of the page whose `buildWidgets` is running (`this.widgets` inside it, e.g. `selectTab(0, ..)`).
pub fn with_page_widgets<R>(f: impl FnOnce(&mut mcrust::net::minecraft::client::gui::screens::screen::Screen) -> R) -> R {
    PAGE_WIDGETS.with(|w| {
        let mut w = w.borrow_mut();
        f(w.last_mut().expect("a profile viewer page's widgets were reached outside its buildWidgets"))
    })
}

/// `layout.addChild(widget)` inside a page's `buildWidgets`: the widget joins the page's list and the layout holds
/// its index (its size is the widget's).
pub fn layout_widget(widget: AbstractWidget) -> LayoutElement {
    let (width, height) = (widget.get_width(), widget.get_height());
    let index = with_page_widgets(|list| list.add_renderable_widget(widget));
    LayoutElement::widget(index, width, height)
}

/// The list index a `layout_widget` element holds.
pub fn widget_index(element: &LayoutElement) -> usize {
    match element {
        LayoutElement::Widget { index, .. } => *index,
        _ => unreachable!("layout_widget answers a widget element"),
    }
}

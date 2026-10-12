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

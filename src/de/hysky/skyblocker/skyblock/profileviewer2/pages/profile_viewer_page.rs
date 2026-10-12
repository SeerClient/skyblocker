//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.pages.ProfileViewerPage by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::loading_information::LoadingInformation;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::layouts::layout_element::LayoutElement;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public sealed interface ProfileViewerPage<T> permits CatacombsPage, CollectionsPage, InventoryPage, MiningPage, SkillsPage, CombatPage
pub trait ProfileViewerPage<T> {
    fn as_any(&self) -> &dyn std::any::Any;
    /// java: FlexibleItemStack getIcon();
    fn get_icon(&self) -> compat::Shared<FlexibleItemStack>;

    /// java: Component getName();
    fn get_name(&self) -> MutableComponent;

    /// java: CompletableFuture<LayoutElement> load(LoadingInformation info);
    fn load(&self, info: &LoadingInformation) -> mcrust::java::util::concurrent::completable_future::CompletableFuture<LayoutElement>;

    /// java: LayoutElement buildWidgets(T data);
    fn build_widgets(&self, data: &T) -> LayoutElement;

    /// java: List<AbstractWidget> getWidgets();
    fn get_widgets(&self) -> Vec<AbstractWidget>;

    /// java: default boolean centred()
    fn centred(&self) -> bool {
        return false;
    }

}

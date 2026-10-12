//! Ported from de.hysky.skyblocker.skyblock.profileviewer.ProfileViewerPage by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::profileviewer::utils::sub_page_select_button::SubPageSelectButton;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;

/// java: public interface ProfileViewerPage
pub trait ProfileViewerPage: compat::DynAny {
    fn as_any(&self) -> &dyn std::any::Any;
    /// java: void extractRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float delta, int rootX, int rootY); (dispatch: the body for this class)
    fn extract_render_state(&self) -> fn(&compat::Shared<dyn ProfileViewerPage>, &mut GuiGraphicsExtractor, i32, i32, f32, i32, i32);

    /// java: default @Nullable List<AbstractWidget> getButtons() (dispatch: the body for this class)
    fn get_buttons(&self) -> fn(&compat::Shared<dyn ProfileViewerPage>) -> Option<Vec<AbstractWidget>> {
        <dyn ProfileViewerPage>::get_buttons_body
    }

    /// java: default void onNavButtonClick(SubPageSelectButton selectButton) (dispatch: the body for this class)
    fn on_nav_button_click(&self) -> fn(&compat::Shared<dyn ProfileViewerPage>, &compat::Shared<SubPageSelectButton>) {
        <dyn ProfileViewerPage>::on_nav_button_click_body
    }

    /// java: default void markWidgetsAsVisible() (dispatch: the body for this class)
    fn mark_widgets_as_visible(&self) -> fn(&compat::Shared<dyn ProfileViewerPage>) {
        <dyn ProfileViewerPage>::mark_widgets_as_visible_body
    }

    /// java: default void markWidgetsAsInvisible() (dispatch: the body for this class)
    fn mark_widgets_as_invisible(&self) -> fn(&compat::Shared<dyn ProfileViewerPage>) {
        <dyn ProfileViewerPage>::mark_widgets_as_invisible_body
    }

    /// java: default void nextPage() (dispatch: the body for this class)
    fn next_page(&self) -> fn(&compat::Shared<dyn ProfileViewerPage>) {
        <dyn ProfileViewerPage>::next_page_body
    }

    /// java: default void previousPage() (dispatch: the body for this class)
    fn previous_page(&self) -> fn(&compat::Shared<dyn ProfileViewerPage>) {
        <dyn ProfileViewerPage>::previous_page_body
    }

}

impl dyn ProfileViewerPage {
    /// java: default @Nullable List<AbstractWidget> getButtons()
    pub fn get_buttons_body(__this: &compat::Shared<dyn ProfileViewerPage>) -> Option<Vec<AbstractWidget>> {
        return None;
    }

    /// java: default void onNavButtonClick(SubPageSelectButton selectButton)
    pub fn on_nav_button_click_body(__this: &compat::Shared<dyn ProfileViewerPage>, select_button: &compat::Shared<SubPageSelectButton>) {
    }

    /// java: default void markWidgetsAsVisible()
    pub fn mark_widgets_as_visible_body(__this: &compat::Shared<dyn ProfileViewerPage>) {
    }

    /// java: default void markWidgetsAsInvisible()
    pub fn mark_widgets_as_invisible_body(__this: &compat::Shared<dyn ProfileViewerPage>) {
    }

    /// java: default void nextPage()
    pub fn next_page_body(__this: &compat::Shared<dyn ProfileViewerPage>) {
    }

    /// java: default void previousPage()
    pub fn previous_page_body(__this: &compat::Shared<dyn ProfileViewerPage>) {
    }

}

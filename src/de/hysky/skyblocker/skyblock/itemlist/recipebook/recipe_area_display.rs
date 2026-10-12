//! Ported from de.hysky.skyblocker.skyblock.itemlist.recipebook.RecipeAreaDisplay by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::itemlist::recipebook::filter_option::FilterOption;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;

/// java: public interface RecipeAreaDisplay
pub trait RecipeAreaDisplay: compat::DynAny {
    fn as_any(&self) -> &dyn std::any::Any;
    /// java: void initialize(Minecraft client, int parentLeft, int parentTop); (dispatch: the body for this class)
    fn initialize(&self) -> fn(&compat::Shared<dyn RecipeAreaDisplay>, &mut mcrust::net::minecraft::client::minecraft::Minecraft, i32, i32);

    /// java: void extractRenderState(GuiGraphicsExtractor graphics, int x, int y, int mouseX, int mouseY, float delta); (dispatch: the body for this class)
    fn extract_render_state(&self) -> fn(&compat::Shared<dyn RecipeAreaDisplay>, &GuiGraphicsExtractor, i32, i32, i32, i32, f32);

    /// java: void extractTooltip(GuiGraphicsExtractor graphics, int x, int y); (dispatch: the body for this class)
    fn extract_tooltip(&self) -> fn(&compat::Shared<dyn RecipeAreaDisplay>, &mut GuiGraphicsExtractor, i32, i32);

    /// java: boolean mouseClicked(MouseButtonEvent click, boolean doubled); (dispatch: the body for this class)
    fn mouse_clicked(&self) -> fn(&compat::Shared<dyn RecipeAreaDisplay>, &MouseButtonEvent, bool) -> bool;

    /// java: default boolean keyPressed(double mouseX, double mouseY, KeyEvent keyInput) (dispatch: the body for this class)
    fn key_pressed(&self) -> fn(&compat::Shared<dyn RecipeAreaDisplay>, f64, f64, &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        <dyn RecipeAreaDisplay>::key_pressed_body
    }

    /// java: default void updateSearchResults(String query, FilterOption filterOption) (dispatch: the body for this class)
    fn update_search_results(&self) -> fn(&compat::Shared<dyn RecipeAreaDisplay>, &str, FilterOption) {
        <dyn RecipeAreaDisplay>::update_search_results_body
    }

    /// java: void updateSearchResults(String query, FilterOption filterOption, boolean refresh); (dispatch: the body for this class)
    fn update_search_results_3(&self) -> fn(&compat::Shared<dyn RecipeAreaDisplay>, String, FilterOption, bool);

}

impl dyn RecipeAreaDisplay {
    /// java: default boolean keyPressed(double mouseX, double mouseY, KeyEvent keyInput)
    pub fn key_pressed_body(__this: &compat::Shared<dyn RecipeAreaDisplay>, mut mouse_x: f64, mut mouse_y: f64, key_input: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        return false;
    }

    /// java: default void updateSearchResults(String query, FilterOption filterOption)
    pub fn update_search_results_body(__this: &compat::Shared<dyn RecipeAreaDisplay>, query: &str, mut filter_option: FilterOption) {
        ({ let __f = compat::dispatch(__this, |__t| __t.update_search_results_3()); __f(__this, query.to_string(), filter_option, false) });
    }

}

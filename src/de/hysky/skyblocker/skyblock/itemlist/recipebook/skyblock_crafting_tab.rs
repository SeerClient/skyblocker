//! Ported from de.hysky.skyblocker.skyblock.itemlist.recipebook.SkyblockCraftingTab by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::itemlist::item_repository::ItemRepository;
use crate::de::hysky::skyblocker::skyblock::itemlist::recipebook::filter_option::FilterOption;
use crate::de::hysky::skyblocker::skyblock::itemlist::recipebook::recipe_area_display::RecipeAreaDisplay;
use crate::de::hysky::skyblocker::skyblock::itemlist::recipebook::recipe_tab::RecipeTab;
use crate::de::hysky::skyblocker::skyblock::itemlist::recipebook::skyblock_recipe_book_component;
use crate::de::hysky::skyblocker::skyblock::itemlist::recipebook::skyblock_recipe_book_component::SkyblockRecipeBookComponent;
use crate::de::hysky::skyblocker::skyblock::itemlist::recipebook::skyblock_recipe_results::SkyblockRecipeResults;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::util::mth;
use mcrust::net::minecraft::world::item::items;
use mcrust::net::minecraft::client::gui::components::renderable::Renderable as _;

/// java: static final ItemStack CRAFTING_TABLE = new ItemStack(Items.CRAFTING_TABLE);
pub static CRAFTING_TABLE: compat::MainThread<mcrust::net::minecraft::world::item::item_stack::ItemStack> = compat::MainThread::new(|| mcrust::net::minecraft::world::item::item_stack::ItemStack::of_one(items::items().get("crafting_table")));

/// java: record SkyblockCraftingTab(SkyblockRecipeBookComponent recipeBook, ItemStack icon, SkyblockRecipeResults results) implements RecipeTab
#[derive(Clone)]
pub struct SkyblockCraftingTab {
    /// java: SkyblockRecipeBookComponent recipeBook
    /// (the book owns this tab - `tabs` - so the tab's reference back is weak: Java's collector frees the pair, a strong
    /// pair of `Arc`s would never be)
    pub recipe_book: compat::WeakShared<SkyblockRecipeBookComponent>,
    /// java: ItemStack icon
    pub icon: mcrust::net::minecraft::world::item::item_stack::ItemStack,
    /// java: SkyblockRecipeResults results
    pub results: compat::Shared<SkyblockRecipeResults>,
}

impl SkyblockCraftingTab {
    /// java: the canonical record constructor
    pub fn new(mut recipe_book: &compat::Shared<SkyblockRecipeBookComponent>, mut icon: mcrust::net::minecraft::world::item::item_stack::ItemStack, mut results: compat::Shared<SkyblockRecipeResults>) -> Self {
        Self { recipe_book: std::sync::Arc::downgrade(recipe_book), icon, results }
    }

    /// java: the record accessor `recipeBook()`
    pub fn recipe_book(&self) -> compat::Shared<SkyblockRecipeBookComponent> {
        self.recipe_book.upgrade().expect("SkyblockCraftingTab: its recipe book is gone")
    }

    /// java: the record accessor `icon()`
    pub fn icon(&self) -> mcrust::net::minecraft::world::item::item_stack::ItemStack {
        self.icon.clone()
    }

    /// java: the record accessor `results()`
    pub fn results(&self) -> compat::Shared<SkyblockRecipeResults> {
        self.results.clone()
    }

}

impl RecipeTab for SkyblockCraftingTab {
    /// java: the record accessor `icon()` (it implements `RecipeTab.icon()`)
    fn icon(&self) -> fn(&compat::Shared<dyn RecipeTab>) -> mcrust::net::minecraft::world::item::item_stack::ItemStack {
        |__this| compat::read_as::<SkyblockCraftingTab, _, _>(__this, |__t| __t.icon())
    }
}

impl RecipeAreaDisplay for SkyblockCraftingTab {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void initialize(Minecraft client, int parentLeft, int parentTop) (dispatch: the body for this class)
    fn initialize(&self) -> fn(&compat::Shared<dyn RecipeAreaDisplay>, &mut mcrust::net::minecraft::client::minecraft::Minecraft, i32, i32) {
        SkyblockCraftingTab::initialize_dyn
    }

    /// java: @Override public void extractRenderState(GuiGraphicsExtractor graphics, int x, int y, int mouseX, int mouseY, float delta) (dispatch: the body for this class)
    fn extract_render_state(&self) -> fn(&compat::Shared<dyn RecipeAreaDisplay>, &GuiGraphicsExtractor, i32, i32, i32, i32, f32) {
        SkyblockCraftingTab::extract_render_state_dyn
    }

    /// java: @Override public void extractTooltip(GuiGraphicsExtractor context, int x, int y) (dispatch: the body for this class)
    fn extract_tooltip(&self) -> fn(&compat::Shared<dyn RecipeAreaDisplay>, &mut GuiGraphicsExtractor, i32, i32) {
        SkyblockCraftingTab::extract_tooltip_dyn
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled) (dispatch: the body for this class)
    fn mouse_clicked(&self) -> fn(&compat::Shared<dyn RecipeAreaDisplay>, &MouseButtonEvent, bool) -> bool {
        SkyblockCraftingTab::mouse_clicked_dyn
    }

    /// java: @Override public boolean keyPressed(double mouseX, double mouseY, KeyEvent input) (dispatch: the body for this class)
    fn key_pressed(&self) -> fn(&compat::Shared<dyn RecipeAreaDisplay>, f64, f64, &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        SkyblockCraftingTab::key_pressed_dyn
    }

    /// java: @Override public void updateSearchResults(String query, FilterOption filterOption, boolean refresh) (dispatch: the body for this class)
    fn update_search_results_3(&self) -> fn(&compat::Shared<dyn RecipeAreaDisplay>, String, FilterOption, bool) {
        SkyblockCraftingTab::update_search_results_3_dyn
    }

}

/// (each dispatch runs on a copy of the record - its fields are final references - so no borrow of the tab is held
/// while the call reaches back into the book, which may dispatch to this tab again: `filterOption`'s cycle listener)
impl SkyblockCraftingTab {
    /// java: @Override public void initialize(Minecraft client, int parentLeft, int parentTop)
    pub fn initialize_impl(&mut self, client: &mut mcrust::net::minecraft::client::minecraft::Minecraft, mut parent_left: i32, mut parent_top: i32) {
        ({ let __f = compat::dispatch(&self.results, |__t| __t.initialize()); __f(&compat::view(&self.results, |a| a as compat::Shared<dyn RecipeAreaDisplay>), client, parent_left, parent_top) });
    }

    /// SkyblockCraftingTab.initialize as the dispatch of RecipeAreaDisplay.initialize
    pub fn initialize_dyn(__this: &compat::Shared<dyn RecipeAreaDisplay>, __p0: &mut mcrust::net::minecraft::client::minecraft::Minecraft, __p1: i32, __p2: i32) {
        compat::read_as::<Self, _, _>(__this, |__s| __s.clone()).initialize_impl(__p0, __p1, __p2)
    }

    /// java: @Override public void extractRenderState(GuiGraphicsExtractor graphics, int x, int y, int mouseX, int mouseY, float delta)
    pub fn extract_render_state_impl(&mut self, graphics: &GuiGraphicsExtractor, mut x: i32, mut y: i32, mut mouse_x: i32, mut mouse_y: i32, mut delta: f32) {
        // java: assert recipeBook.searchBox != null;
        let recipe_book = self.recipe_book();
        if ItemRepository::files_imported() {
            // java: recipeBook.searchBox.extractRenderState(graphics, mouseX, mouseY, delta);
            SkyblockRecipeBookComponent::extract_search_box(&recipe_book, mouse_x, mouse_y);
            // java: recipeBook.filterOption.extractRenderState(graphics, mouseX, mouseY, delta);
            SkyblockRecipeBookComponent::extract_filter_option(&recipe_book, graphics, mouse_x, mouse_y, delta);
            ({ let __f = compat::dispatch(&self.results, |__t| __t.extract_render_state()); __f(&compat::view(&self.results, |a| a as compat::Shared<dyn RecipeAreaDisplay>), graphics, x, y, mouse_x, mouse_y, delta) });
        } else {
            gui_graphics_extractor::centered_text_string(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), "Loading...", x.wrapping_add((*skyblock_recipe_book_component::IMAGE_WIDTH).wrapping_div(2)), y.wrapping_add(68), 0xFFFFFFFFu32 as i32);
        }
    }

    /// SkyblockCraftingTab.extractRenderState as the dispatch of RecipeAreaDisplay.extractRenderState
    pub fn extract_render_state_dyn(__this: &compat::Shared<dyn RecipeAreaDisplay>, __p0: &GuiGraphicsExtractor, __p1: i32, __p2: i32, __p3: i32, __p4: i32, __p5: f32) {
        compat::read_as::<Self, _, _>(__this, |__s| __s.clone()).extract_render_state_impl(__p0, __p1, __p2, __p3, __p4, __p5)
    }

    /// java: @Override public void extractTooltip(GuiGraphicsExtractor context, int x, int y)
    pub fn extract_tooltip_impl(&self, context: &mut GuiGraphicsExtractor, mut x: i32, mut y: i32) {
        if ItemRepository::files_imported() {
            ({ let __f = compat::dispatch(&self.results, |__t| __t.extract_tooltip()); __f(&compat::view(&self.results, |a| a as compat::Shared<dyn RecipeAreaDisplay>), context, x, y) });
        }
    }

    /// SkyblockCraftingTab.extractTooltip as the dispatch of RecipeAreaDisplay.extractTooltip
    pub fn extract_tooltip_dyn(__this: &compat::Shared<dyn RecipeAreaDisplay>, __p0: &mut GuiGraphicsExtractor, __p1: i32, __p2: i32) {
        compat::read_as::<Self, _, _>(__this, |__s| __s.clone()).extract_tooltip_impl(__p0, __p1, __p2)
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    pub fn mouse_clicked_impl(&mut self, click: &MouseButtonEvent, mut doubled: bool) -> bool {
        if ItemRepository::files_imported() {
            if { let __f = compat::dispatch(&self.results, |__t| __t.mouse_clicked()); __f(&compat::view(&self.results, |a| a as compat::Shared<dyn RecipeAreaDisplay>), click, doubled) } {
                return true;
            } else {
                let recipe_book = self.recipe_book();
                // java: if (recipeBook.searchBox != null)
                if SkyblockRecipeBookComponent::has_search_box(&recipe_book) {
                    // java: boolean magnifyingGlassClicked = recipeBook.magnifierIconPlacement != null
                    //   && recipeBook.magnifierIconPlacement.containsPoint(Mth.floor(click.x()), Mth.floor(click.y()));
                    let magnifying_glass_clicked: bool = compat::read(&recipe_book, |__t| __t.magnifier_icon_placement).is_some_and(|placement| placement.contains_point(mth::floor_f64(click.x()), mth::floor_f64(click.y())));
                    // java: if (magnifyingGlassClicked || recipeBook.searchBox.mouseClicked(click, doubled))
                    if magnifying_glass_clicked || SkyblockRecipeBookComponent::search_box_mouse_clicked(&recipe_book, click, doubled) {
                        SkyblockRecipeResults::close_recipe_view(&self.results);
                        // java: recipeBook.searchBox.setFocused(true);
                        SkyblockRecipeBookComponent::set_search_box_focused(&recipe_book, true);
                        return true;
                    }
                    // java: recipeBook.searchBox.setFocused(false);
                    SkyblockRecipeBookComponent::set_search_box_focused(&recipe_book, false);
                    // java: return recipeBook.filterOption.mouseClicked(click, doubled);
                    return SkyblockRecipeBookComponent::filter_option_mouse_clicked(&recipe_book, click, doubled);
                }
            }
        }
        return false;
    }

    /// SkyblockCraftingTab.mouseClicked as the dispatch of RecipeAreaDisplay.mouseClicked
    pub fn mouse_clicked_dyn(__this: &compat::Shared<dyn RecipeAreaDisplay>, __p0: &MouseButtonEvent, __p1: bool) -> bool {
        compat::read_as::<Self, _, _>(__this, |__s| __s.clone()).mouse_clicked_impl(__p0, __p1)
    }

    /// java: @Override public boolean keyPressed(double mouseX, double mouseY, KeyEvent input)
    pub fn key_pressed_impl(&self, mut mouse_x: f64, mut mouse_y: f64, input: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        return (ItemRepository::files_imported() && { let __f = compat::dispatch(&self.results, |__t| __t.key_pressed()); __f(&compat::view(&self.results, |a| a as compat::Shared<dyn RecipeAreaDisplay>), mouse_x, mouse_y, input) });
    }

    /// SkyblockCraftingTab.keyPressed as the dispatch of RecipeAreaDisplay.keyPressed
    pub fn key_pressed_dyn(__this: &compat::Shared<dyn RecipeAreaDisplay>, __p0: f64, __p1: f64, __p2: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        compat::read_as::<Self, _, _>(__this, |__s| __s.clone()).key_pressed_impl(__p0, __p1, __p2)
    }

    /// java: @Override public void updateSearchResults(String query, FilterOption filterOption, boolean refresh)
    pub fn update_search_results_3_impl(&mut self, mut query: String, mut filter_option: FilterOption, mut refresh: bool) {
        if ItemRepository::files_imported() {
            ({ let __f = compat::dispatch(&self.results, |__t| __t.update_search_results_3()); __f(&compat::view(&self.results, |a| a as compat::Shared<dyn RecipeAreaDisplay>), query.clone(), filter_option, refresh) });
        }
    }

    /// SkyblockCraftingTab.updateSearchResults as the dispatch of RecipeAreaDisplay.updateSearchResults
    pub fn update_search_results_3_dyn(__this: &compat::Shared<dyn RecipeAreaDisplay>, __p0: String, __p1: FilterOption, __p2: bool) {
        compat::read_as::<Self, _, _>(__this, |__s| __s.clone()).update_search_results_3_impl(__p0, __p1, __p2)
    }

}

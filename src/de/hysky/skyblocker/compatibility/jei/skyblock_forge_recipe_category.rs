//! Ported from de.hysky.skyblocker.compatibility.jei.SkyblockForgeRecipeCategory by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::render::gui_helper::GuiHelper;
use crate::de::hysky::skyblocker::compatibility::jei::abstract_skyblock_recipe_category::AbstractSkyblockRecipeCategory;
use crate::de::hysky::skyblocker::compatibility::jei::abstract_skyblock_recipe_category::AbstractSkyblockRecipeCategoryBase;
use crate::de::hysky::skyblocker::skyblock::itemlist::recipes::skyblock_forge_recipe::SkyblockForgeRecipe;
use crate::de::hysky::skyblocker::utils::item_utils::ItemUtils;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

//   java: private static final IRecipeType<SkyblockForgeRecipe> RECIPE_TYPE = IRecipeType.create(SkyblockForgeRecipe.ID, SkyblockForgeRecipe.class);
/// java: private static final IRecipeType<SkyblockForgeRecipe> RECIPE_TYPE = IRecipeType.create(SkyblockForgeRecipe.ID, SkyblockForgeRecipe.class);
pub static RECIPE_TYPE: compat::MainThread<crate::unresolved::IRecipeType> = compat::MainThread::new(|| crate::de::hysky::skyblocker::compatibility::jei::abstract_skyblock_recipe_category::jei_recipe_type_create(crate::de::hysky::skyblocker::skyblock::itemlist::recipes::skyblock_forge_recipe::ID.get_mut().clone()));
/// java: private static final Component TITLE = Component.translatable("emi.category.skyblocker.skyblock_forge");
pub static TITLE: compat::MainThread<MutableComponent> = compat::MainThread::new(|| component::translatable("emi.category.skyblocker.skyblock_forge"));

/// java: public final class SkyblockForgeRecipeCategory extends AbstractSkyblockRecipeCategory<SkyblockForgeRecipe>
pub struct SkyblockForgeRecipeCategory {
    /// java: extends AbstractSkyblockRecipeCategory
    pub base: AbstractSkyblockRecipeCategoryBase<compat::Shared<SkyblockForgeRecipe>>,
}

impl std::ops::Deref for SkyblockForgeRecipeCategory {
    type Target = AbstractSkyblockRecipeCategoryBase<compat::Shared<SkyblockForgeRecipe>>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for SkyblockForgeRecipeCategory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl SkyblockForgeRecipeCategory {
    /// java: protected SkyblockForgeRecipeCategory(IGuiHelper guiHelper)
    pub fn new(mut gui_helper: crate::unresolved::IGuiHelper) -> Self {
        let mut this = Self {
            base: AbstractSkyblockRecipeCategoryBase::<compat::Shared<SkyblockForgeRecipe>>::new(gui_helper, RECIPE_TYPE.get(), TITLE.get_mut().clone(), ItemUtils::get_skyblocker_forge_stack()),
        };
        this
    }

}

impl AbstractSkyblockRecipeCategory<compat::Shared<crate::de::hysky::skyblocker::skyblock::itemlist::recipes::skyblock_forge_recipe::SkyblockForgeRecipe>> for SkyblockForgeRecipeCategory {
    fn abstract_skyblock_recipe_category_base(&self) -> &AbstractSkyblockRecipeCategoryBase<compat::Shared<crate::de::hysky::skyblocker::skyblock::itemlist::recipes::skyblock_forge_recipe::SkyblockForgeRecipe>> {
        &self.base
    }
    fn abstract_skyblock_recipe_category_base_mut(&mut self) -> &mut AbstractSkyblockRecipeCategoryBase<compat::Shared<crate::de::hysky::skyblocker::skyblock::itemlist::recipes::skyblock_forge_recipe::SkyblockForgeRecipe>> {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void draw(SkyblockForgeRecipe recipe, IRecipeSlotsView recipeSlotsView, GuiGraphicsExtractor graphics, double mouseX, double mouseY)
    fn draw(&self, recipe: &compat::Shared<SkyblockForgeRecipe>, recipe_slots_view: &crate::unresolved::IRecipeSlotsView, graphics: &GuiGraphicsExtractor, mut mouse_x: f64, mut mouse_y: f64) {
        //   java: IDrawableStatic recipeArrow = this.guiHelper.getRecipeArrow();
        let mut recipe_arrow: crate::unresolved::IDrawableStatic = crate::de::hysky::skyblocker::compatibility::jei::abstract_skyblock_recipe_category::jei_get_recipe_arrow(&self.gui_helper);
        //   java: int arrowYOffset = (this.getHeight() - recipeArrow.getHeight()) / 2;
        let mut arrow_y_offset: i32 = (self.get_height().wrapping_sub(crate::de::hysky::skyblocker::compatibility::jei::abstract_skyblock_recipe_category::jei_drawable_height(&recipe_arrow))).wrapping_div(2);
        //   java: recipeArrow.draw(graphics, 61, arrowYOffset);
        crate::de::hysky::skyblocker::compatibility::jei::abstract_skyblock_recipe_category::jei_drawable_draw(&recipe_arrow, graphics, 61, arrow_y_offset);
        //   java: if (GuiHelper.pointIsInArea(mouseX, mouseY, 61, arrowYOffset, 61 + recipeArrow.getWidth(), arrowYOffset + recipeArrow.getHeight()))
        if GuiHelper::point_is_in_area(mouse_x, mouse_y, 61.0, (arrow_y_offset) as f64, (61i32.wrapping_add(crate::de::hysky::skyblocker::compatibility::jei::abstract_skyblock_recipe_category::jei_drawable_width(&recipe_arrow))) as f64, (arrow_y_offset.wrapping_add(crate::de::hysky::skyblocker::compatibility::jei::abstract_skyblock_recipe_category::jei_drawable_height(&recipe_arrow))) as f64) {
            self.draw_tooltip(graphics, &component::literal(&SkyblockForgeRecipe::get_duration_string(recipe)), mouse_x, mouse_y);
        }
    }

    /// java: @Override protected int getCraftingGridLayoutSize()
    fn get_crafting_grid_layout_size(&self) -> i32 {
        return 2;
    }

}

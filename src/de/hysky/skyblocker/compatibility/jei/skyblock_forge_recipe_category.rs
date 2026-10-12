//! Ported from de.hysky.skyblocker.compatibility.jei.SkyblockForgeRecipeCategory by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::compatibility::jei::abstract_skyblock_recipe_category::AbstractSkyblockRecipeCategory;
use crate::de::hysky::skyblocker::compatibility::jei::abstract_skyblock_recipe_category::AbstractSkyblockRecipeCategoryBase;
use crate::de::hysky::skyblocker::skyblock::itemlist::recipes::skyblock_forge_recipe::SkyblockForgeRecipe;
use crate::de::hysky::skyblocker::utils::item_utils::ItemUtils;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

// TODO(fabric2seer): static call IRecipeType.create(..)
//   java: private static final IRecipeType<SkyblockForgeRecipe> RECIPE_TYPE = IRecipeType.create(SkyblockForgeRecipe.ID, SkyblockForgeRecipe.class);
/// java: private static final IRecipeType<SkyblockForgeRecipe> RECIPE_TYPE = IRecipeType.create(SkyblockForgeRecipe.ID, SkyblockForgeRecipe.class);
pub static RECIPE_TYPE: compat::MainThread<crate::unresolved::IRecipeType> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `RECIPE_TYPE` - see the TODO above"));
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
        // TODO(fabric2seer): `.getRecipeArrow(..)` on a IGuiHelper
        //   java: IDrawableStatic recipeArrow = this.guiHelper.getRecipeArrow();
        let mut recipe_arrow: crate::unresolved::IDrawableStatic = todo!("fabric2seer: the initialiser of `recipeArrow` (TODO above)");
        // TODO(fabric2seer): `.getHeight(..)` on a IDrawableStatic
        //   java: int arrowYOffset = (this.getHeight() - recipeArrow.getHeight()) / 2;
        let mut arrow_y_offset: i32 = todo!("fabric2seer: the initialiser of `arrowYOffset` (TODO above)");
        // TODO(fabric2seer): `.draw(..)` on a IDrawableStatic
        //   java: recipeArrow.draw(graphics, 61, arrowYOffset);
        // TODO(fabric2seer): the condition: `.getWidth(..)` on a IDrawableStatic
        //   java: GuiHelper.pointIsInArea(..)
        if todo!("fabric2seer: the condition (TODO above)") {
            self.draw_tooltip(&mut graphics, &component::literal(&SkyblockForgeRecipe::get_duration_string(recipe)), mouse_x, mouse_y);
        }
    }

    /// java: @Override protected int getCraftingGridLayoutSize()
    fn get_crafting_grid_layout_size(&self) -> i32 {
        return 2;
    }

}

//! Ported from de.hysky.skyblocker.compatibility.jei.SkyblockNpcShopRecipeCategory by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::compatibility::jei::abstract_skyblock_recipe_category::AbstractSkyblockRecipeCategory;
use crate::de::hysky::skyblocker::compatibility::jei::abstract_skyblock_recipe_category::AbstractSkyblockRecipeCategoryBase;
use crate::de::hysky::skyblocker::skyblock::itemlist::recipes::centered_recipe::CenteredRecipe;
use crate::de::hysky::skyblocker::skyblock::itemlist::recipes::skyblock_npc_shop_recipe::SkyblockNpcShopRecipe;
use crate::de::hysky::skyblocker::skyblock::tabhud::util::ico;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use crate::de::hysky::skyblocker::utils::render::gui_helper::GuiHelper;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

//   java: private static final IRecipeType<SkyblockNpcShopRecipe> RECIPE_TYPE = IRecipeType.create(SkyblockNpcShopRecipe.ID, SkyblockNpcShopRecipe.class);
/// java: private static final IRecipeType<SkyblockNpcShopRecipe> RECIPE_TYPE = IRecipeType.create(SkyblockNpcShopRecipe.ID, SkyblockNpcShopRecipe.class);
pub static RECIPE_TYPE: compat::MainThread<crate::unresolved::IRecipeType> = compat::MainThread::new(|| crate::de::hysky::skyblocker::compatibility::jei::abstract_skyblock_recipe_category::jei_recipe_type_create(crate::de::hysky::skyblocker::skyblock::itemlist::recipes::skyblock_npc_shop_recipe::ID.get_mut().clone()));
/// java: private static final Component TITLE = Component.translatable("emi.category.skyblocker.skyblock_npc_shop");
pub static TITLE: compat::MainThread<MutableComponent> = compat::MainThread::new(|| component::translatable("emi.category.skyblocker.skyblock_npc_shop"));
/// java: private static final int ITEM_SIZE = 16;
pub const ITEM_SIZE: i32 = 16;
/// java: private static final int NPC_ITEM_PADDING = 2;
pub const NPC_ITEM_PADDING: i32 = 2;

/// java: public final class SkyblockNpcShopRecipeCategory extends AbstractSkyblockRecipeCategory<SkyblockNpcShopRecipe>
pub struct SkyblockNpcShopRecipeCategory {
    /// java: extends AbstractSkyblockRecipeCategory
    pub base: AbstractSkyblockRecipeCategoryBase<compat::Shared<SkyblockNpcShopRecipe>>,
}

impl std::ops::Deref for SkyblockNpcShopRecipeCategory {
    type Target = AbstractSkyblockRecipeCategoryBase<compat::Shared<SkyblockNpcShopRecipe>>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for SkyblockNpcShopRecipeCategory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl SkyblockNpcShopRecipeCategory {
    /// java: protected SkyblockNpcShopRecipeCategory(IGuiHelper guiHelper)
    pub fn new(mut gui_helper: crate::unresolved::IGuiHelper) -> Self {
        let mut this = Self {
            base: AbstractSkyblockRecipeCategoryBase::<compat::Shared<SkyblockNpcShopRecipe>>::new(gui_helper, RECIPE_TYPE.get(), TITLE.get_mut().clone(), ico::GOLD_NUGGET.get_mut().clone()),
        };
        this
    }

}

impl AbstractSkyblockRecipeCategory<compat::Shared<crate::de::hysky::skyblocker::skyblock::itemlist::recipes::skyblock_npc_shop_recipe::SkyblockNpcShopRecipe>> for SkyblockNpcShopRecipeCategory {
    fn abstract_skyblock_recipe_category_base(&self) -> &AbstractSkyblockRecipeCategoryBase<compat::Shared<crate::de::hysky::skyblocker::skyblock::itemlist::recipes::skyblock_npc_shop_recipe::SkyblockNpcShopRecipe>> {
        &self.base
    }
    fn abstract_skyblock_recipe_category_base_mut(&mut self) -> &mut AbstractSkyblockRecipeCategoryBase<compat::Shared<crate::de::hysky::skyblocker::skyblock::itemlist::recipes::skyblock_npc_shop_recipe::SkyblockNpcShopRecipe>> {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void draw(SkyblockNpcShopRecipe recipe, IRecipeSlotsView recipeSlotsView, GuiGraphicsExtractor context, double mouseX, double mouseY)
    fn draw(&self, recipe: &compat::Shared<SkyblockNpcShopRecipe>, recipe_slots_view: &crate::unresolved::IRecipeSlotsView, context: &GuiGraphicsExtractor, mut mouse_x: f64, mut mouse_y: f64) {
        //   java: IDrawableStatic recipeArrow = this.guiHelper.getRecipeArrow();
        let mut recipe_arrow: crate::unresolved::IDrawableStatic = crate::de::hysky::skyblocker::compatibility::jei::abstract_skyblock_recipe_category::jei_get_recipe_arrow(&self.gui_helper);
        //   java: int arrowYOffset = (this.getHeight() - recipeArrow.getHeight()) / 2;
        let mut arrow_y_offset: i32 = (self.get_height().wrapping_sub(crate::de::hysky::skyblocker::compatibility::jei::abstract_skyblock_recipe_category::jei_drawable_height(&recipe_arrow))).wrapping_div(2);
        //   java: recipeArrow.draw(context, 61, arrowYOffset);
        crate::de::hysky::skyblocker::compatibility::jei::abstract_skyblock_recipe_category::jei_drawable_draw(&recipe_arrow, context, 61, arrow_y_offset);
        //   java: int itemX = 61 + ((recipeArrow.getWidth() - ITEM_SIZE) / 2);
        let mut item_x: i32 = 61i32.wrapping_add((crate::de::hysky::skyblocker::compatibility::jei::abstract_skyblock_recipe_category::jei_drawable_width(&recipe_arrow).wrapping_sub(ITEM_SIZE)).wrapping_div(2));
        let mut item_y: i32 = (arrow_y_offset.wrapping_sub(ITEM_SIZE)).wrapping_sub(NPC_ITEM_PADDING);
        context.item(&FlexibleItemStack::get_stack_or_throw(&({ let __f = compat::dispatch(recipe, |__t| __t.get_representative()); __f(&compat::view(recipe, |a| a as compat::Shared<dyn CenteredRecipe>)) }).unwrap()), item_x, item_y);
        if GuiHelper::point_is_in_area(mouse_x, mouse_y, (item_x) as f64, (item_y) as f64, ((item_x.wrapping_add(ITEM_SIZE)) as f64) as f64, ((item_y.wrapping_add(ITEM_SIZE)) as f64) as f64) {
            self.draw_tooltip(context, &compat::component_of((FlexibleItemStack::get_stack_or_throw(&({ let __f = compat::dispatch(recipe, |__t| __t.get_representative()); __f(&compat::view(recipe, |a| a as compat::Shared<dyn CenteredRecipe>)) }).unwrap())).get_hover_name()), mouse_x, mouse_y);
        }
    }

}

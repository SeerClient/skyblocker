//! Ported from de.hysky.skyblocker.compatibility.jei.SkyblockCraftingRecipeCategory by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::compatibility::jei::abstract_skyblock_recipe_category::AbstractSkyblockRecipeCategory;
use crate::de::hysky::skyblocker::compatibility::jei::abstract_skyblock_recipe_category::AbstractSkyblockRecipeCategoryBase;
use crate::de::hysky::skyblocker::skyblock::itemlist::recipes::skyblock_crafting_recipe::SkyblockCraftingRecipe;
use crate::de::hysky::skyblocker::utils::item_utils::ItemUtils;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

// TODO(fabric2seer): static call IRecipeType.create(..)
//   java: private static final IRecipeType<SkyblockCraftingRecipe> RECIPE_TYPE = IRecipeType.create(SkyblockCraftingRecipe.ID, SkyblockCraftingRecipe.class);
/// java: private static final IRecipeType<SkyblockCraftingRecipe> RECIPE_TYPE = IRecipeType.create(SkyblockCraftingRecipe.ID, SkyblockCraftingRecipe.class);
pub static RECIPE_TYPE: compat::MainThread<crate::unresolved::IRecipeType> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `RECIPE_TYPE` - see the TODO above"));
/// java: private static final Component TITLE = Component.translatable("emi.category.skyblocker.skyblock_crafting");
pub static TITLE: compat::MainThread<MutableComponent> = compat::MainThread::new(|| component::translatable("emi.category.skyblocker.skyblock_crafting"));

/// java: public final class SkyblockCraftingRecipeCategory extends AbstractSkyblockRecipeCategory<SkyblockCraftingRecipe>
pub struct SkyblockCraftingRecipeCategory {
    /// java: extends AbstractSkyblockRecipeCategory
    pub base: AbstractSkyblockRecipeCategoryBase<compat::Shared<SkyblockCraftingRecipe>>,
}

impl std::ops::Deref for SkyblockCraftingRecipeCategory {
    type Target = AbstractSkyblockRecipeCategoryBase<compat::Shared<SkyblockCraftingRecipe>>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for SkyblockCraftingRecipeCategory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl SkyblockCraftingRecipeCategory {
    /// java: protected SkyblockCraftingRecipeCategory(IGuiHelper guiHelper)
    pub fn new(mut gui_helper: crate::unresolved::IGuiHelper) -> Self {
        let mut this = Self {
            base: AbstractSkyblockRecipeCategoryBase::<compat::Shared<SkyblockCraftingRecipe>>::new(gui_helper.clone(), RECIPE_TYPE.get_mut().clone(), TITLE.get_mut().clone(), ItemUtils::get_skyblocker_stack()),
        };
        this
    }

}

impl AbstractSkyblockRecipeCategory<compat::Shared<crate::de::hysky::skyblocker::skyblock::itemlist::recipes::skyblock_crafting_recipe::SkyblockCraftingRecipe>> for SkyblockCraftingRecipeCategory {
    fn abstract_skyblock_recipe_category_base(&self) -> &AbstractSkyblockRecipeCategoryBase<compat::Shared<crate::de::hysky::skyblocker::skyblock::itemlist::recipes::skyblock_crafting_recipe::SkyblockCraftingRecipe>> {
        &self.base
    }
    fn abstract_skyblock_recipe_category_base_mut(&mut self) -> &mut AbstractSkyblockRecipeCategoryBase<compat::Shared<crate::de::hysky::skyblocker::skyblock::itemlist::recipes::skyblock_crafting_recipe::SkyblockCraftingRecipe>> {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}

//! Ported from de.hysky.skyblocker.compatibility.jei.AbstractSkyblockRecipeCategory by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::world::item::crafting::display::slot_display::SlotDisplay;
use mcrust::org::joml::vector2f::Vector2f;

/// java: public abstract sealed class AbstractSkyblockRecipeCategory<T extends SkyblockRecipe> extends AbstractRecipeCategory<T> permits SkyblockCraftingRecipeCategory, SkyblockForgeRecipeCategory, SkyblockKatUpgradeRecipeCategory, SkyblockNpcShopRe
pub struct AbstractSkyblockRecipeCategoryBase<T> {
    /// java: extends mezz.jei.api.recipe.category.AbstractRecipeCategory
    pub base: crate::unresolved::AbstractRecipeCategory,
    /// java: protected final IGuiHelper guiHelper;
    pub gui_helper: crate::unresolved::IGuiHelper,
    /// java: protected final ICraftingGridHelper craftingGridHelper;
    pub crafting_grid_helper: crate::unresolved::ICraftingGridHelper,
    /// java: the recipe type parameter `T` (used only in the methods)
    pub _recipe: std::marker::PhantomData<fn() -> T>,
}

impl<T: 'static> std::ops::Deref for AbstractSkyblockRecipeCategoryBase<T> {
    type Target = crate::unresolved::AbstractRecipeCategory;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<T: 'static> std::ops::DerefMut for AbstractSkyblockRecipeCategoryBase<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl<T: Clone + 'static> AbstractSkyblockRecipeCategoryBase<T> {
    /// java: protected AbstractSkyblockRecipeCategory(IGuiHelper guiHelper, IRecipeType<T> recipeType, Component title, FlexibleItemStack icon)
    pub fn new(mut gui_helper: crate::unresolved::IGuiHelper, recipe_type: &crate::unresolved::IRecipeType, mut title: MutableComponent, mut icon: compat::Shared<FlexibleItemStack>) -> Self {
        let __init_gui_helper = gui_helper;
        // TODO(fabric2seer): `.createCraftingGridHelper(..)` on a IGuiHelper
        //   java: this.craftingGridHelper = guiHelper.createCraftingGridHelper();
        // TODO(fabric2seer): `.createDrawableItemLike(..)` on a IGuiHelper
        //   java: super(recipeType, title, guiHelper.createDrawableItemLike(icon.typeHolder().value()), CraftingRecipeCategory.width, CraftingRecipeCategory.height);
        let mut this: Self = todo!("constructor");
        this
    }

}

/// java: the virtual methods of AbstractSkyblockRecipeCategory (implemented by AbstractSkyblockRecipeCategoryBase and every subclass)
pub trait AbstractSkyblockRecipeCategory<T> {
    fn abstract_skyblock_recipe_category_base(&self) -> &AbstractSkyblockRecipeCategoryBase<T>;
    fn abstract_skyblock_recipe_category_base_mut(&mut self) -> &mut AbstractSkyblockRecipeCategoryBase<T>;
    fn as_any(&self) -> &dyn std::any::Any;

    /// java: @Override public void draw(T recipe, IRecipeSlotsView recipeSlotsView, GuiGraphicsExtractor graphics, double mouseX, double mouseY)
    fn draw(&self, recipe: &T, recipe_slots_view: &crate::unresolved::IRecipeSlotsView, graphics: &GuiGraphicsExtractor, mut mouse_x: f64, mut mouse_y: f64) {
        // TODO(fabric2seer): `.getRecipeArrow(..)` on a IGuiHelper
        //   java: IDrawableStatic recipeArrow = this.guiHelper.getRecipeArrow();
        let mut recipe_arrow: crate::unresolved::IDrawableStatic = todo!("fabric2seer: the initialiser of `recipeArrow` (TODO above)");
        // TODO(fabric2seer): `.draw(..)` on a IDrawableStatic
        //   java: recipeArrow.draw(graphics, 61, (this.getHeight() - recipeArrow.getHeight()) / 2);
    }

    /// java: protected void drawTooltip(GuiGraphicsExtractor graphics, Component text, double mouseX, double mouseY)
    fn draw_tooltip(&self, graphics: &mut GuiGraphicsExtractor, text: &MutableComponent, mut mouse_x: f64, mut mouse_y: f64) {
        // TODO(fabric2seer): Matrix3x2fStack.transformPosition used as a value
        //   java: Vector2f transformedPosition = graphics.pose().transformPosition((int) mouseX, (int) mouseY, new Vector2f());
        let mut transformed_position: Vector2f = todo!("fabric2seer: the initialiser of `transformedPosition` (TODO above)");
        // TODO(fabric2seer): the port's `set_tooltip_for_next_frame` ports this Java method with other parameters (tooltip_for_next_frame, component for Java's component, x, y)
        // TODO(fabric2seer): `set_tooltip_for_next_frame` takes 2 argument(s) in the port (`tooltip_for_next_frame: &mut Option<DeferredTooltip>, component: crate::net::minecraft::network::chat::formatted_text::FormattedText`), Java passes 3
        // rust: gui_graphics_extractor::set_tooltip_for_next_frame(&mut text, ((transformed_position.x()) as i32), ((transformed_position.y()) as i32));
    }

    /// java: @Override public void setRecipe(IRecipeLayoutBuilder builder, T recipe, IFocusGroup focuses)
    fn set_recipe(&self, builder: &crate::unresolved::IRecipeLayoutBuilder, recipe: &T, focuses: &crate::unresolved::IFocusGroup) {
        // TODO(fabric2seer): `.getOutputs(..)` on a T
        //   java: SlotDisplay result = new SlotDisplay.ItemStackSlotDisplay(recipe.getOutputs().getFirst().toTemplate());
        let mut result: Arc<dyn SlotDisplay> = todo!("fabric2seer: the initialiser of `result` (TODO above)");
        // TODO(fabric2seer): `.getInputs(..)` on a T
        //   java: List<SlotDisplay> ingredients = recipe.getInputs().stream()
        //   java: 				.map(flexibleStack -> flexibleStack.isEmpty() ? SlotDisplay.Empty.INSTANCE : new SlotDisplay.ItemStackSlotDisplay(flexibleStack.toTemplate()))
        //   java: 				.map(SlotDisplay.class::cast)
        //   java: 				.toList();
        let mut ingredients: Vec<Arc<dyn SlotDisplay>> = todo!("fabric2seer: the initialiser of `ingredients` (TODO above)");
        // TODO(fabric2seer): `.createAndSetOutputs(..)` on a ICraftingGridHelper
        //   java: this.craftingGridHelper.createAndSetOutputs(builder, result);
        // TODO(fabric2seer): `.createAndSetIngredientsFromDisplays(..)` on a ICraftingGridHelper
        //   java: this.craftingGridHelper.createAndSetIngredientsFromDisplays(builder, ingredients, this.getCraftingGridLayoutSize(), this.getCraftingGridLayoutSize());
    }

    /// java: protected int getCraftingGridLayoutSize()
    fn get_crafting_grid_layout_size(&self) -> i32 {
        return 3;
    }

    /// java: @Override public Identifier getIdentifier(T recipe)
    fn get_identifier(&self, recipe: &T) -> Identifier {
        // TODO(fabric2seer): `.getRecipeIdentifier(..)` on a T
        //   java: return recipe.getRecipeIdentifier();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}

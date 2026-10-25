//! Ported from de.hysky.skyblocker.compatibility.jei.AbstractSkyblockRecipeCategory by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::world::item::crafting::display::slot_display::SlotDisplay;
use mcrust::net::minecraft::world::item::crafting::display::slot_display::{Empty, ItemStackSlotDisplay};
use crate::de::hysky::skyblocker::skyblock::itemlist::recipes::skyblock_recipe::SkyblockRecipe;
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
        //   java: super(recipeType, title, guiHelper.createDrawableItemLike(icon.typeHolder().value()), CraftingRecipeCategory.width, CraftingRecipeCategory.height);
        let __icon = jei_create_drawable_item_like(&gui_helper, FlexibleItemStack::type_holder(&icon).value());
        let __init_base = jei_abstract_recipe_category(recipe_type, title, __icon, JEI_CRAFTING_RECIPE_CATEGORY_WIDTH, JEI_CRAFTING_RECIPE_CATEGORY_HEIGHT);
        //   java: this.craftingGridHelper = guiHelper.createCraftingGridHelper();
        let __init_crafting_grid_helper = jei_create_crafting_grid_helper(&gui_helper);
        let mut this: Self = Self {
            base: __init_base,
            gui_helper,
            crafting_grid_helper: __init_crafting_grid_helper,
            _recipe: std::marker::PhantomData,
        };
        this
    }

}

/// What a JEI recipe of these categories is: a `SkyblockRecipe` (Java's `T extends SkyblockRecipe`).
pub trait JeiSkyblockRecipe {
    fn skyblock_recipe(&self) -> compat::Shared<dyn SkyblockRecipe>;
}

impl<R: SkyblockRecipe + 'static> JeiSkyblockRecipe for compat::Shared<R> {
    fn skyblock_recipe(&self) -> compat::Shared<dyn SkyblockRecipe> {
        self.clone() as compat::Shared<dyn SkyblockRecipe>
    }
}

/// java: the virtual methods of AbstractSkyblockRecipeCategory (implemented by AbstractSkyblockRecipeCategoryBase and every subclass)
pub trait AbstractSkyblockRecipeCategory<T: JeiSkyblockRecipe> {
    fn abstract_skyblock_recipe_category_base(&self) -> &AbstractSkyblockRecipeCategoryBase<T>;
    fn abstract_skyblock_recipe_category_base_mut(&mut self) -> &mut AbstractSkyblockRecipeCategoryBase<T>;
    fn as_any(&self) -> &dyn std::any::Any;

    /// `AbstractRecipeCategory.getHeight()`.
    fn get_height(&self) -> i32 {
        jei_category_height(&self.abstract_skyblock_recipe_category_base().base)
    }

    /// java: @Override public void draw(T recipe, IRecipeSlotsView recipeSlotsView, GuiGraphicsExtractor graphics, double mouseX, double mouseY)
    fn draw(&self, recipe: &T, recipe_slots_view: &crate::unresolved::IRecipeSlotsView, graphics: &GuiGraphicsExtractor, mut mouse_x: f64, mut mouse_y: f64) {
        //   java: IDrawableStatic recipeArrow = this.guiHelper.getRecipeArrow();
        let mut recipe_arrow: crate::unresolved::IDrawableStatic = jei_get_recipe_arrow(&self.abstract_skyblock_recipe_category_base().gui_helper);
        //   java: recipeArrow.draw(graphics, 61, (this.getHeight() - recipeArrow.getHeight()) / 2);
        jei_drawable_draw(&recipe_arrow, graphics, 61, (self.get_height().wrapping_sub(jei_drawable_height(&recipe_arrow))).wrapping_div(2));
    }

    /// java: protected void drawTooltip(GuiGraphicsExtractor graphics, Component text, double mouseX, double mouseY)
    fn draw_tooltip(&self, graphics: &GuiGraphicsExtractor, text: &MutableComponent, mut mouse_x: f64, mut mouse_y: f64) {
        //   java: Vector2f transformedPosition = graphics.pose().transformPosition((int) mouseX, (int) mouseY, new Vector2f());
        let (__tx, __ty) = gui_graphics_extractor::current_pose().transform_position(((mouse_x) as i32) as f32, ((mouse_y) as i32) as f32);
        let mut transformed_position: Vector2f = Vector2f::new_xy(__tx, __ty);
        //   java: graphics.setTooltipForNextFrame(text, (int) transformedPosition.x(), (int) transformedPosition.y());
        // (the frame's deferred tooltip draws at the mouse, which is `transformedPosition`)
        let _ = (graphics, &transformed_position);
        compat::batch_s_set_tooltip_for_next_frame(vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(text.clone())]);
    }

    /// java: @Override public void setRecipe(IRecipeLayoutBuilder builder, T recipe, IFocusGroup focuses)
    fn set_recipe(&self, builder: &crate::unresolved::IRecipeLayoutBuilder, recipe: &T, focuses: &crate::unresolved::IFocusGroup) {
        let __recipe = recipe.skyblock_recipe();
        //   java: SlotDisplay result = new SlotDisplay.ItemStackSlotDisplay(recipe.getOutputs().getFirst().toTemplate());
        let __outputs = ({ let __f = compat::dispatch(&__recipe, |__t| __t.get_outputs()); __f(&__recipe) });
        let mut result: Arc<dyn SlotDisplay> = Arc::new(ItemStackSlotDisplay::new(FlexibleItemStack::to_template(__outputs.first().expect("NoSuchElementException"))));
        //   java: List<SlotDisplay> ingredients = recipe.getInputs().stream()
        //   java: 				.map(flexibleStack -> flexibleStack.isEmpty() ? SlotDisplay.Empty.INSTANCE : new SlotDisplay.ItemStackSlotDisplay(flexibleStack.toTemplate()))
        //   java: 				.map(SlotDisplay.class::cast)
        //   java: 				.toList();
        let __inputs = ({ let __f = compat::dispatch(&__recipe, |__t| __t.get_inputs()); __f(&__recipe) });
        let mut ingredients: Vec<Arc<dyn SlotDisplay>> = __inputs.iter().map(|flexible_stack| if FlexibleItemStack::is_empty(flexible_stack) { Empty::instance() } else { Arc::new(ItemStackSlotDisplay::new(FlexibleItemStack::to_template(flexible_stack))) as Arc<dyn SlotDisplay> }).collect();
        let __base = self.abstract_skyblock_recipe_category_base();
        //   java: this.craftingGridHelper.createAndSetOutputs(builder, result);
        jei_create_and_set_outputs(&__base.crafting_grid_helper, builder, result);
        //   java: this.craftingGridHelper.createAndSetIngredientsFromDisplays(builder, ingredients, this.getCraftingGridLayoutSize(), this.getCraftingGridLayoutSize());
        jei_create_and_set_ingredients_from_displays(&__base.crafting_grid_helper, builder, ingredients, self.get_crafting_grid_layout_size(), self.get_crafting_grid_layout_size());
    }

    /// java: protected int getCraftingGridLayoutSize()
    fn get_crafting_grid_layout_size(&self) -> i32 {
        return 3;
    }

    /// java: @Override public Identifier getIdentifier(T recipe)
    fn get_identifier(&self, recipe: &T) -> Identifier {
        //   java: return recipe.getRecipeIdentifier();
        let __recipe = recipe.skyblock_recipe();
        let __f = compat::dispatch(&__recipe, |__t| __t.get_recipe_identifier());
        return __f(&__recipe);
    }

}

// JEI (`mezz.jei`) is a separate mod that SeerClient never loads: these categories are built only from
// `SkyblockerJEIPlugin.registerCategories`, which JEI calls - and `JEICompatibility.JEI_LOADED` is false. Its API types
// are the `crate::unresolved` names; the functions below are JEI's side of the boundary.

/// `mezz.jei.library.plugins.vanilla.crafting.CraftingRecipeCategory.width`.
pub const JEI_CRAFTING_RECIPE_CATEGORY_WIDTH: i32 = 116;
/// `mezz.jei.library.plugins.vanilla.crafting.CraftingRecipeCategory.height`.
pub const JEI_CRAFTING_RECIPE_CATEGORY_HEIGHT: i32 = 54;

/// `IRecipeType.create(uid, recipeClass)`.
pub(crate) fn jei_recipe_type_create(uid: Identifier) -> crate::unresolved::IRecipeType {
    let _ = uid;
    crate::unresolved::IRecipeType
}

/// `new AbstractRecipeCategory(recipeType, title, icon, width, height)`.
fn jei_abstract_recipe_category(recipe_type: &crate::unresolved::IRecipeType, title: MutableComponent, icon: crate::unresolved::IDrawableStatic, width: i32, height: i32) -> crate::unresolved::AbstractRecipeCategory {
    let _ = (recipe_type, title, icon, width, height);
    crate::unresolved::AbstractRecipeCategory
}

/// `AbstractRecipeCategory.getHeight()` - the `height` it was built with (every category here passes
/// `CraftingRecipeCategory.height`).
fn jei_category_height(category: &crate::unresolved::AbstractRecipeCategory) -> i32 {
    let _ = category;
    JEI_CRAFTING_RECIPE_CATEGORY_HEIGHT
}

/// `IGuiHelper.createDrawableItemLike(item)`.
fn jei_create_drawable_item_like<V>(gui_helper: &crate::unresolved::IGuiHelper, item: V) -> crate::unresolved::IDrawableStatic {
    let _ = (gui_helper, item);
    crate::unresolved::IDrawableStatic
}

/// `IGuiHelper.createCraftingGridHelper()`.
fn jei_create_crafting_grid_helper(gui_helper: &crate::unresolved::IGuiHelper) -> crate::unresolved::ICraftingGridHelper {
    let _ = gui_helper;
    crate::unresolved::ICraftingGridHelper
}

/// `IGuiHelper.getRecipeArrow()`.
pub(crate) fn jei_get_recipe_arrow(gui_helper: &crate::unresolved::IGuiHelper) -> crate::unresolved::IDrawableStatic {
    let _ = gui_helper;
    crate::unresolved::IDrawableStatic
}

/// The size of JEI's recipe arrow sprite (`getRecipeArrow()`): 24 x 17.
const JEI_RECIPE_ARROW_WIDTH: i32 = 24;
const JEI_RECIPE_ARROW_HEIGHT: i32 = 17;

/// `IDrawableStatic.getWidth()` of the recipe arrow.
pub(crate) fn jei_drawable_width(drawable: &crate::unresolved::IDrawableStatic) -> i32 {
    let _ = drawable;
    JEI_RECIPE_ARROW_WIDTH
}

/// `IDrawableStatic.getHeight()` of the recipe arrow.
pub(crate) fn jei_drawable_height(drawable: &crate::unresolved::IDrawableStatic) -> i32 {
    let _ = drawable;
    JEI_RECIPE_ARROW_HEIGHT
}

/// `IDrawableStatic.draw(graphics, x, y)`.
pub(crate) fn jei_drawable_draw(drawable: &crate::unresolved::IDrawableStatic, graphics: &GuiGraphicsExtractor, x: i32, y: i32) {
    let _ = (drawable, graphics, x, y);
}

/// `ICraftingGridHelper.createAndSetOutputs(builder, result)`.
fn jei_create_and_set_outputs(helper: &crate::unresolved::ICraftingGridHelper, builder: &crate::unresolved::IRecipeLayoutBuilder, result: Arc<dyn SlotDisplay>) {
    let _ = (helper, builder, result);
}

/// `ICraftingGridHelper.createAndSetIngredientsFromDisplays(builder, ingredients, width, height)`.
fn jei_create_and_set_ingredients_from_displays(helper: &crate::unresolved::ICraftingGridHelper, builder: &crate::unresolved::IRecipeLayoutBuilder, ingredients: Vec<Arc<dyn SlotDisplay>>, width: i32, height: i32) {
    let _ = (helper, builder, ingredients, width, height);
}

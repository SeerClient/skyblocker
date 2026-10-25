//! Ported from de.hysky.skyblocker.skyblock.itemlist.recipes.SkyblockCraftingRecipe by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::itemlist::recipes::skyblock_recipe::RecipeSlot;
use crate::de::hysky::skyblocker::skyblock::itemlist::recipes::skyblock_recipe::SkyblockRecipe;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use mcrust::java::text::decimal_format_symbols::Locale;
use mcrust::net::minecraft::client::gui::navigation::screen_position::ScreenPosition;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: public static final Identifier ID = SkyblockerMod.id("skyblock_crafting");
pub static ID: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("skyblock_crafting"));

/// java: public class SkyblockCraftingRecipe implements SkyblockRecipe
#[derive(Clone)]
pub struct SkyblockCraftingRecipe {
    /// java: private final Component craftText;
    pub craft_text: MutableComponent,
    /// java: private final List<FlexibleItemStack> grid = new ArrayList<>(9);
    pub grid: Vec<compat::Shared<FlexibleItemStack>>,
    /// java: private final FlexibleItemStack result;
    pub result: Option<compat::Shared<FlexibleItemStack>>,
}

impl SkyblockCraftingRecipe {
    /// java: public SkyblockCraftingRecipe(NEUCraftingRecipe neuCraftingRecipe)
    pub fn new(mut neu_crafting_recipe: crate::io::github::moulberry::repo::data::neu_crafting_recipe::NEUCraftingRecipe) -> compat::Shared<Self> {
        //   java: this.craftText = neuCraftingRecipe.getExtraText() != null ? Component.literal(neuCraftingRecipe.getExtraText()) : Component.empty();
        let __init_craft_text: MutableComponent = match neu_crafting_recipe.get_extra_text() {
            Some(extra_text) => mcrust::net::minecraft::network::chat::component::literal(extra_text),
            None => mcrust::net::minecraft::network::chat::component::empty(),
        };
        let __this = compat::shared(Self {
            craft_text: __init_craft_text,
            grid: Vec::with_capacity((9) as usize),
            result: None,
        });
        //   java: for (NEUIngredient input : neuCraftingRecipe.getInputs()) {
        //   java: 			grid.add(SkyblockRecipe.getItemStack(input));
        //   java: 		}
        // (a slot the recipe does not name is a null element, and `getItemStack(null)` throws - as in Java, the item's
        // `loadRecipes` is what fails)
        for input in neu_crafting_recipe.get_inputs().iter() {
            let input = input.as_ref().expect("java.lang.NullPointerException: NEUIngredient input");
            let __v = crate::de::hysky::skyblocker::skyblock::itemlist::recipes::skyblock_recipe::get_item_stack(input);
            compat::write(&__this, |__t| __t.grid.push(__v));
        }
        //   java: result = SkyblockRecipe.getItemStack(neuCraftingRecipe.getOutput());
        let __v = crate::de::hysky::skyblocker::skyblock::itemlist::recipes::skyblock_recipe::get_item_stack(neu_crafting_recipe.get_output().expect("java.lang.NullPointerException: the NEU recipe has no output"));
        compat::write(&__this, |__t| __t.result = Some(__v));
        __this
    }

    /// java: public List<FlexibleItemStack> getGrid()
    pub fn get_grid(__this: &compat::Shared<Self>) -> Vec<compat::Shared<FlexibleItemStack>> {
        return compat::read(__this, |__t| __t.grid.clone());
    }

    /// java: public FlexibleItemStack getResult()
    pub fn get_result(__this: &compat::Shared<Self>) -> compat::Shared<FlexibleItemStack> {
        return compat::read(__this, |__t| __t.result.clone()).unwrap();
    }

}

impl SkyblockRecipe for SkyblockCraftingRecipe {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public List<RecipeSlot> getInputSlots(int width, int height) (dispatch: the body for this class)
    fn get_input_slots(&self) -> fn(&compat::Shared<dyn SkyblockRecipe>, i32, i32) -> Vec<RecipeSlot> {
        SkyblockCraftingRecipe::get_input_slots_dyn
    }

    /// java: @Override public List<RecipeSlot> getOutputSlots(int width, int height) (dispatch: the body for this class)
    fn get_output_slots(&self) -> fn(&compat::Shared<dyn SkyblockRecipe>, i32, i32) -> Vec<RecipeSlot> {
        SkyblockCraftingRecipe::get_output_slots_dyn
    }

    /// java: @Override public List<FlexibleItemStack> getInputs() (dispatch: the body for this class)
    fn get_inputs(&self) -> fn(&compat::Shared<dyn SkyblockRecipe>) -> Vec<compat::Shared<FlexibleItemStack>> {
        SkyblockCraftingRecipe::get_inputs_dyn
    }

    /// java: @Override public List<FlexibleItemStack> getOutputs() (dispatch: the body for this class)
    fn get_outputs(&self) -> fn(&compat::Shared<dyn SkyblockRecipe>) -> Vec<compat::Shared<FlexibleItemStack>> {
        SkyblockCraftingRecipe::get_outputs_dyn
    }

    /// java: @Override public Component getExtraText() (dispatch: the body for this class)
    fn get_extra_text(&self) -> fn(&compat::Shared<dyn SkyblockRecipe>) -> MutableComponent {
        SkyblockCraftingRecipe::get_extra_text_dyn
    }

    /// java: @Override public Identifier getCategoryIdentifier() (dispatch: the body for this class)
    fn get_category_identifier(&self) -> fn(&compat::Shared<dyn SkyblockRecipe>) -> Identifier {
        SkyblockCraftingRecipe::get_category_identifier_dyn
    }

    /// java: @Override public Identifier getRecipeIdentifier() (dispatch: the body for this class)
    fn get_recipe_identifier(&self) -> fn(&compat::Shared<dyn SkyblockRecipe>) -> Identifier {
        SkyblockCraftingRecipe::get_recipe_identifier_dyn
    }

    /// java: @Override public @Nullable ScreenPosition getArrowLocation(int width, int height) (dispatch: the body for this class)
    fn get_arrow_location(&self) -> fn(&compat::Shared<dyn SkyblockRecipe>, i32, i32) -> Option<ScreenPosition> {
        SkyblockCraftingRecipe::get_arrow_location_dyn
    }

}

impl SkyblockCraftingRecipe {
    /// java: @Override public List<RecipeSlot> getInputSlots(int width, int height)
    pub fn get_input_slots_impl(__this: &compat::Shared<Self>, mut width: i32, mut height: i32) -> Vec<RecipeSlot> {
        let mut start: ScreenPosition = ScreenPosition::new((width.wrapping_div(2)).wrapping_sub(58), (height.wrapping_div(2)).wrapping_sub(if ((({ let __f = compat::dispatch(__this, |__t| __t.get_extra_text()); __f(&compat::view(__this, |a| a as compat::Shared<dyn SkyblockRecipe>)) })).get_string()).is_empty() { 27 } else { 32 }));
        let mut to_return: Vec<RecipeSlot> = Vec::with_capacity((9) as usize);
        for i in { 0..(__this.lock().borrow().grid.len() as i32) } {
            let mut x: i32 = i.wrapping_rem(3);
            let mut y: i32 = i.wrapping_div(3);
            to_return.push(RecipeSlot::new((start.x().wrapping_add(1)).wrapping_add(x.wrapping_mul(18)), (start.y().wrapping_add(1)).wrapping_add(y.wrapping_mul(18)), __this.lock().borrow().grid[(i) as usize].clone()));
        }
        return to_return;
    }

    /// SkyblockCraftingRecipe.getInputSlots as the dispatch of SkyblockRecipe.getInputSlots
    pub fn get_input_slots_dyn(__this: &compat::Shared<dyn SkyblockRecipe>, __p0: i32, __p1: i32) -> Vec<RecipeSlot> {
        Self::get_input_slots_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1)
    }

    /// java: @Override public List<RecipeSlot> getOutputSlots(int width, int height)
    pub fn get_output_slots_impl(__this: &compat::Shared<Self>, mut width: i32, mut height: i32) -> Vec<RecipeSlot> {
        let mut start: ScreenPosition = ScreenPosition::new((width.wrapping_div(2)).wrapping_sub(58), (height.wrapping_div(2)).wrapping_sub(if ((({ let __f = compat::dispatch(__this, |__t| __t.get_extra_text()); __f(&compat::view(__this, |a| a as compat::Shared<dyn SkyblockRecipe>)) })).get_string()).is_empty() { 26 } else { 31 }));
        return vec![RecipeSlot::new(start.x().wrapping_add(95), start.y().wrapping_add(19), compat::read(__this, |__t| __t.result.clone()).unwrap())];
    }

    /// SkyblockCraftingRecipe.getOutputSlots as the dispatch of SkyblockRecipe.getOutputSlots
    pub fn get_output_slots_dyn(__this: &compat::Shared<dyn SkyblockRecipe>, __p0: i32, __p1: i32) -> Vec<RecipeSlot> {
        Self::get_output_slots_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1)
    }

    /// java: @Override public List<FlexibleItemStack> getInputs()
    pub fn get_inputs_impl(__this: &compat::Shared<Self>) -> Vec<compat::Shared<FlexibleItemStack>> {
        return compat::read(__this, |__t| __t.grid.clone());
    }

    /// SkyblockCraftingRecipe.getInputs as the dispatch of SkyblockRecipe.getInputs
    pub fn get_inputs_dyn(__this: &compat::Shared<dyn SkyblockRecipe>) -> Vec<compat::Shared<FlexibleItemStack>> {
        Self::get_inputs_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public List<FlexibleItemStack> getOutputs()
    pub fn get_outputs_impl(__this: &compat::Shared<Self>) -> Vec<compat::Shared<FlexibleItemStack>> {
        return vec![compat::read(__this, |__t| __t.result.clone()).unwrap()];
    }

    /// SkyblockCraftingRecipe.getOutputs as the dispatch of SkyblockRecipe.getOutputs
    pub fn get_outputs_dyn(__this: &compat::Shared<dyn SkyblockRecipe>) -> Vec<compat::Shared<FlexibleItemStack>> {
        Self::get_outputs_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public Component getExtraText()
    pub fn get_extra_text_impl(__this: &compat::Shared<Self>) -> MutableComponent {
        return compat::read(__this, |__t| __t.craft_text.clone());
    }

    /// SkyblockCraftingRecipe.getExtraText as the dispatch of SkyblockRecipe.getExtraText
    pub fn get_extra_text_dyn(__this: &compat::Shared<dyn SkyblockRecipe>) -> MutableComponent {
        Self::get_extra_text_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public Identifier getCategoryIdentifier()
    pub fn get_category_identifier_impl(__this: &compat::Shared<Self>) -> Identifier {
        return ID.get_mut().clone();
    }

    /// SkyblockCraftingRecipe.getCategoryIdentifier as the dispatch of SkyblockRecipe.getCategoryIdentifier
    pub fn get_category_identifier_dyn(__this: &compat::Shared<dyn SkyblockRecipe>) -> Identifier {
        Self::get_category_identifier_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public Identifier getRecipeIdentifier()
    pub fn get_recipe_identifier_impl(__this: &compat::Shared<Self>) -> Identifier {
        return Identifier::from_namespace_and_path("skyblock", &format!("{}_{}", SkyblockCraftingRecipe::get_result(__this).lock().borrow().get_skyblock_id().to_lowercase().replace(';', &'_'.to_string()), FlexibleItemStack::count(&SkyblockCraftingRecipe::get_result(__this))));
    }

    /// SkyblockCraftingRecipe.getRecipeIdentifier as the dispatch of SkyblockRecipe.getRecipeIdentifier
    pub fn get_recipe_identifier_dyn(__this: &compat::Shared<dyn SkyblockRecipe>) -> Identifier {
        Self::get_recipe_identifier_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public @Nullable ScreenPosition getArrowLocation(int width, int height)
    pub fn get_arrow_location_impl(__this: &compat::Shared<Self>, mut width: i32, mut height: i32) -> Option<ScreenPosition> {
        let mut start: ScreenPosition = ScreenPosition::new((width.wrapping_div(2)).wrapping_sub(58), (height.wrapping_div(2)).wrapping_sub(if ((({ let __f = compat::dispatch(__this, |__t| __t.get_extra_text()); __f(&compat::view(__this, |a| a as compat::Shared<dyn SkyblockRecipe>)) })).get_string()).is_empty() { 26 } else { 31 }));
        return Some(ScreenPosition::new(start.x().wrapping_add(60), start.y().wrapping_add(18)));
    }

    /// SkyblockCraftingRecipe.getArrowLocation as the dispatch of SkyblockRecipe.getArrowLocation
    pub fn get_arrow_location_dyn(__this: &compat::Shared<dyn SkyblockRecipe>, __p0: i32, __p1: i32) -> Option<ScreenPosition> {
        Self::get_arrow_location_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1)
    }

}

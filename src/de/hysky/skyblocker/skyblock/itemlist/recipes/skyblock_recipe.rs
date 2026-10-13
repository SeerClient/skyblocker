//! Ported from de.hysky.skyblocker.skyblock.itemlist.recipes.SkyblockRecipe by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::itemlist::item_repository::ItemRepository;
use crate::de::hysky::skyblocker::skyblock::itemlist::recipes::recipe_item_stack_cache;
use crate::de::hysky::skyblocker::skyblock::itemlist::recipes::recipe_item_stack_cache::RecipeItemStackCache;
use crate::de::hysky::skyblocker::utils::flexible_item_stack;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use crate::de::hysky::skyblocker::utils::item_utils::ItemUtils;
use mcrust::java::text::decimal_format::DecimalFormat;
use mcrust::java::text::decimal_format_symbols::Locale;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::navigation::screen_position::ScreenPosition;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::world::item::component::item_lore::ItemLore;
use mcrust::net::minecraft::world::item::items;

// TODO(fabric2seer): static `NumberFormat.getInstance` has no Rust counterpart in the port
//   java: NumberFormat NUMBER_FORMAT = NumberFormat.getInstance(Locale.US);
/// java: NumberFormat NUMBER_FORMAT = NumberFormat.getInstance(Locale.US);
pub static NUMBER_FORMAT: compat::MainThread<DecimalFormat> = compat::MainThread::new(|| DecimalFormat::new("#,##0.###", mcrust::java::text::decimal_format_symbols::get_instance(Locale::Us)));

/// java: public interface SkyblockRecipe
pub trait SkyblockRecipe: compat::DynAny {
    fn as_any(&self) -> &dyn std::any::Any;
    /// java: List<RecipeSlot> getInputSlots(int width, int height); (dispatch: the body for this class)
    fn get_input_slots(&self) -> fn(&compat::Shared<dyn SkyblockRecipe>, i32, i32) -> Vec<RecipeSlot>;

    /// java: List<RecipeSlot> getOutputSlots(int width, int height); (dispatch: the body for this class)
    fn get_output_slots(&self) -> fn(&compat::Shared<dyn SkyblockRecipe>, i32, i32) -> Vec<RecipeSlot>;

    /// java: default @Nullable ScreenPosition getArrowLocation(int width, int height) (dispatch: the body for this class)
    fn get_arrow_location(&self) -> fn(&compat::Shared<dyn SkyblockRecipe>, i32, i32) -> Option<ScreenPosition> {
        <dyn SkyblockRecipe>::get_arrow_location_body
    }

    /// java: List<FlexibleItemStack> getInputs(); (dispatch: the body for this class)
    fn get_inputs(&self) -> fn(&compat::Shared<dyn SkyblockRecipe>) -> Vec<compat::Shared<FlexibleItemStack>>;

    /// java: List<FlexibleItemStack> getOutputs(); (dispatch: the body for this class)
    fn get_outputs(&self) -> fn(&compat::Shared<dyn SkyblockRecipe>) -> Vec<compat::Shared<FlexibleItemStack>>;

    /// java: default void extractRenderState(GuiGraphicsExtractor graphics, int width, int height, double mouseX, double mouseY) (dispatch: the body for this class)
    fn extract_render_state(&self) -> fn(&compat::Shared<dyn SkyblockRecipe>, &GuiGraphicsExtractor, i32, i32, f64, f64) {
        <dyn SkyblockRecipe>::extract_render_state_body
    }

    /// java: Component getExtraText(); (dispatch: the body for this class)
    fn get_extra_text(&self) -> fn(&compat::Shared<dyn SkyblockRecipe>) -> MutableComponent;

    /// java: Identifier getCategoryIdentifier(); (dispatch: the body for this class)
    fn get_category_identifier(&self) -> fn(&compat::Shared<dyn SkyblockRecipe>) -> Identifier;

    /// java: Identifier getRecipeIdentifier(); (dispatch: the body for this class)
    fn get_recipe_identifier(&self) -> fn(&compat::Shared<dyn SkyblockRecipe>) -> Identifier;

}

impl dyn SkyblockRecipe {
    /// java: default @Nullable ScreenPosition getArrowLocation(int width, int height)
    pub fn get_arrow_location_body(__this: &compat::Shared<dyn SkyblockRecipe>, mut width: i32, mut height: i32) -> Option<ScreenPosition> {
        return None;
    }

    /// java: default void extractRenderState(GuiGraphicsExtractor graphics, int width, int height, double mouseX, double mouseY)
    pub fn extract_render_state_body(__this: &compat::Shared<dyn SkyblockRecipe>, graphics: &GuiGraphicsExtractor, mut width: i32, mut height: i32, mut mouse_x: f64, mut mouse_y: f64) {
    }

}

/// java: static FlexibleItemStack getItemStack(NEUIngredient input)
pub fn get_item_stack(input: &crate::unresolved::NEUIngredient) -> compat::Shared<FlexibleItemStack> {
    // TODO(fabric2seer): the condition: static field NEUIngredient.SENTINEL_EMPTY
    //   java: input == NEUIngredient.SENTINEL_EMPTY
    if todo!("fabric2seer: the condition (TODO above)") {
        return flexible_item_stack::EMPTY.get_mut().clone();
    }
    // TODO(fabric2seer): `.getItemId(..)` on a NEUIngredient
    //   java: String id = input.getItemId();
    let mut id: String = todo!("fabric2seer: the initialiser of `id` (TODO above)");
    // TODO(fabric2seer): `.getAmount(..)` on a NEUIngredient
    //   java: int amount = (int) input.getAmount();
    let mut amount: i32 = todo!("fabric2seer: the initialiser of `amount` (TODO above)");
    let mut cache_key: String = RecipeItemStackCache::get_cache_key(&id, amount);
    let mut cached_stack: Option<compat::Shared<FlexibleItemStack>> = recipe_item_stack_cache::CACHE.get_mut().get(&cache_key).cloned();
    if let Some(cached_stack) = &cached_stack {
        return cached_stack.clone();
    }
    let mut base_stack: Option<compat::Shared<FlexibleItemStack>> = ItemRepository::get_item_stack(&id);
    let mut computed_stack: Option<compat::Shared<FlexibleItemStack>> = None;
    if let Some(base_stack) = &base_stack {
        if amount == FlexibleItemStack::count(base_stack) {
            computed_stack = Some(base_stack.clone());
        } else {
            computed_stack = Some(FlexibleItemStack::copy_with_count(base_stack, amount));
            if amount > 1 {
                FlexibleItemStack::set(computed_stack.as_ref().unwrap(), mcrust::net::minecraft::core::component::data_components::max_stack_size().clone(), amount);
            }
        }
    } else {
        if id == "SKYBLOCK_COIN" {
            computed_stack = Some(FlexibleItemStack::new2(items::items().get("gold_nugget")));
            FlexibleItemStack::set(computed_stack.as_ref().unwrap(), mcrust::net::minecraft::core::component::data_components::item_name().clone(), component::literal("Skyblock Coins").with_style_format(ChatFormatting::Gold));
            FlexibleItemStack::set(computed_stack.as_ref().unwrap(), mcrust::net::minecraft::core::component::data_components::enchantment_glint_override().clone(), true);
            let mut format: String = NUMBER_FORMAT.get_mut().format((amount) as f64);
            FlexibleItemStack::set(computed_stack.as_ref().unwrap(), mcrust::net::minecraft::core::component::data_components::lore().clone(), ItemLore::new(vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from((component::literal(&format).with_style_format(ChatFormatting::Gold).with_style_fn(|style| style.with_italic(Some(false)))).append(component::literal(" coins")))]).unwrap());
        } else {
            logger().warn(&format!("[Skyblocker Recipe] Unable to find item {}", id));
            computed_stack = Some(ItemUtils::get_named_placeholder(&id));
        }
    }
    recipe_item_stack_cache::CACHE.get_mut().insert(cache_key.clone(), computed_stack.clone().unwrap());
    return computed_stack.unwrap();
}

/// java: record RecipeSlot(int x, int y, FlexibleItemStack stack, boolean showBackground)
#[derive(Clone)]
pub struct RecipeSlot {
    /// java: int x
    pub x: i32,
    /// java: int y
    pub y: i32,
    /// java: FlexibleItemStack stack
    pub stack: compat::Shared<FlexibleItemStack>,
    /// java: boolean showBackground
    pub show_background: bool,
}

impl RecipeSlot {
    /// java: public RecipeSlot(int x, int y, FlexibleItemStack stack)
    pub fn new(mut x: i32, mut y: i32, mut stack: compat::Shared<FlexibleItemStack>) -> Self {
        let mut this = Self::new_canonical(x, y, stack.clone(), true);
        this
    }

    /// java: the canonical record constructor
    pub fn new_canonical(mut x: i32, mut y: i32, mut stack: compat::Shared<FlexibleItemStack>, mut show_background: bool) -> Self {
        Self { x, y, stack, show_background }
    }

    /// java: the record accessor `x()`
    pub fn x(&self) -> i32 {
        self.x
    }

    /// java: the record accessor `y()`
    pub fn y(&self) -> i32 {
        self.y
    }

    /// java: the record accessor `stack()`
    pub fn stack(&self) -> compat::Shared<FlexibleItemStack> {
        self.stack.clone()
    }

    /// java: the record accessor `showBackground()`
    pub fn show_background(&self) -> bool {
        self.show_background
    }

}

//! Ported from de.hysky.skyblocker.skyblock.itemlist.recipes.CenteredRecipe by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::itemlist::recipes::skyblock_recipe::RecipeSlot;
use crate::de::hysky::skyblocker::skyblock::itemlist::recipes::skyblock_recipe::SkyblockRecipe;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use mcrust::net::minecraft::client::gui::navigation::screen_position::ScreenPosition;

/// java: int SLOT_SIZE = 18;
pub const SLOT_SIZE: i32 = 18;
/// java: int ARROW_LENGTH = 24;
pub const ARROW_LENGTH: i32 = 24;
/// java: int ARROW_PADDING = 3;
pub const ARROW_PADDING: i32 = 3;

/// java: public interface CenteredRecipe extends SkyblockRecipe
pub trait CenteredRecipe: SkyblockRecipe {
    /// java: FlexibleItemStack getIcon(); (dispatch: the body for this class)
    fn get_icon(&self) -> fn(&compat::Shared<dyn CenteredRecipe>) -> compat::Shared<FlexibleItemStack>;

    /// java: @Nullable FlexibleItemStack getRepresentative(); (dispatch: the body for this class)
    fn get_representative(&self) -> fn(&compat::Shared<dyn CenteredRecipe>) -> Option<compat::Shared<FlexibleItemStack>>;

}

/// java: private static boolean shouldSplit(int size)
pub fn should_split(mut size: i32) -> bool {
    return (size > 3);
}

/// java: private static int getRowSize(int size)
pub fn get_row_size(mut size: i32) -> i32 {
    return (if should_split(size) { compat::floor_div_i32(size, 2) } else { size });
}

/// java: private static int getCenterX(int width, int size)
pub fn get_center_x(mut width: i32, mut size: i32) -> i32 {
    let mut center_x: i32 = width.wrapping_div(2);
    center_x = center_x.wrapping_add(((((get_row_size(size)).min(3)).wrapping_mul(SLOT_SIZE)).wrapping_div(2)).wrapping_sub(SLOT_SIZE.wrapping_div(2)));
    if size > 1 && should_offset_arrow(size) {
        center_x = center_x.wrapping_sub(SLOT_SIZE.wrapping_div(2));
    }
    return center_x;
}

/// java: private static int getCenterY(int height, boolean shouldOffset)
pub fn get_center_y(mut height: i32, mut should_offset: bool) -> i32 {
    if should_offset {
        height = height.wrapping_sub(SLOT_SIZE);
    }
    return height.wrapping_div(2);
}

/// java: static List<SkyblockRecipe.RecipeSlot> arrangeInputs(int width, int height, @Nullable FlexibleItemStack centeredItem, List<FlexibleItemStack> inputs)
pub fn arrange_inputs(mut width: i32, mut height: i32, mut centered_item: Option<compat::Shared<FlexibleItemStack>>, inputs: &Vec<compat::Shared<FlexibleItemStack>>) -> Vec<RecipeSlot> {
    let mut slots: Vec<RecipeSlot> = Vec::new();
    if let Some(centered_item) = &centered_item {
        slots.push(RecipeSlot::new((width.wrapping_sub(SLOT_SIZE)).wrapping_div(2), SLOT_SIZE.wrapping_div(2), centered_item.clone()));
    }
    let mut size: i32 = (inputs.len() as i32);
    let mut center_x: i32 = get_center_x(width, size);
    let mut center_y: i32 = get_center_y(height, centered_item.is_none());
    let mut on_second_row: bool = false;
    let mut row_size: i32 = get_row_size(size);
    let mut x: i32 = ((center_x.wrapping_sub(SLOT_SIZE.wrapping_mul(row_size.min(3)))).wrapping_sub(ARROW_LENGTH.wrapping_div(2))).wrapping_sub(ARROW_PADDING);
    let mut y: i32 = (if should_split(size) { (center_y.wrapping_sub(SLOT_SIZE.wrapping_div(2))).wrapping_add(3) } else { center_y });
    for i in 0..size {
        slots.push(RecipeSlot::new(x, y, inputs[(i) as usize].clone()));
        x = x.wrapping_add(SLOT_SIZE);
        if (i.wrapping_add(1)).wrapping_rem(row_size) == 0 && !on_second_row {
            on_second_row = true;
            x = x.wrapping_sub(row_size.wrapping_mul(SLOT_SIZE));
            y = y.wrapping_add(SLOT_SIZE);
        }
    }
    return slots;
}

/// java: static boolean shouldOffsetArrow(int size)
pub fn should_offset_arrow(mut size: i32) -> bool {
    return (should_split(size) && (size.wrapping_rem(2) == 1 || size >= 8));
}

/// java: static List<SkyblockRecipe.RecipeSlot> arrangeOutputs(int width, int height, boolean offsetY, int inputSize, FlexibleItemStack output)
pub fn arrange_outputs(mut width: i32, mut height: i32, mut offset_y: bool, mut input_size: i32, output: &compat::Shared<FlexibleItemStack>) -> Vec<RecipeSlot> {
    let mut center_x: i32 = get_center_x(width, input_size);
    let mut center_y: i32 = get_center_y(height, offset_y);
    if should_offset_arrow(input_size) {
        center_x = center_x.wrapping_add(SLOT_SIZE);
    }
    if should_split(input_size) {
        center_y = center_y.wrapping_add(2);
    }
    return vec![RecipeSlot::new((center_x.wrapping_add(ARROW_LENGTH.wrapping_div(2))).wrapping_add(ARROW_PADDING), center_y, output.clone())];
}

/// java: static ScreenPosition getArrowLocation(int width, int height, boolean offsetY, int inputSize)
pub fn get_arrow_location(mut width: i32, mut height: i32, mut offset_y: bool, mut input_size: i32) -> ScreenPosition {
    let mut center_x: i32 = get_center_x(width, input_size);
    let mut center_y: i32 = get_center_y(height, offset_y);
    if should_offset_arrow(input_size) {
        center_x = center_x.wrapping_add(SLOT_SIZE);
    }
    if should_split(input_size) {
        center_y = center_y.wrapping_add(3);
    }
    return ScreenPosition::new((center_x.wrapping_sub(ARROW_LENGTH.wrapping_div(2))).wrapping_sub(1), center_y.wrapping_sub(1));
}

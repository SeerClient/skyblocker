//! Ported from de.hysky.skyblocker.skyblock.dwarven.fossil.FossilCalculations by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::dwarven::fossil::fossil_types::FossilTypes;
use crate::de::hysky::skyblocker::skyblock::dwarven::fossil::structures::TileGrid;
use crate::de::hysky::skyblocker::skyblock::dwarven::fossil::structures::TileState;
use crate::de::hysky::skyblocker::skyblock::dwarven::fossil::structures::TransformationOptions;
use crate::de::hysky::skyblocker::skyblock::dwarven::fossil::structures::permutation;
use mcrust::net::minecraft::world::item::item::Item;

/// java: protected static final List<Structures.permutation> POSSIBLE_STATES = getAllPossibleStates();
pub static POSSIBLE_STATES: LazyLock<Vec<permutation>> = LazyLock::new(|| FossilCalculations::get_all_possible_states());
/// java: private static final int EXCAVATOR_WIDTH = 9;
pub const EXCAVATOR_WIDTH: i32 = 9;
/// java: private static final int EXCAVATOR_HEIGHT = 6;
pub const EXCAVATOR_HEIGHT: i32 = 6;
/// java: protected static int permutations = -1;
pub static PERMUTATIONS: LazyLock<Mutex<i32>> = LazyLock::new(|| Mutex::new(-1));
/// java: protected static int minimumTiles;
pub static MINIMUM_TILES: LazyLock<Mutex<i32>> = LazyLock::new(|| Mutex::new(0));
/// java: protected static String fossilName;
pub static FOSSIL_NAME: LazyLock<Mutex<Option<String>>> = LazyLock::new(|| Mutex::new(None));

/// java: public class FossilCalculations
#[derive(Clone)]
pub struct FossilCalculations {}

impl FossilCalculations {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: protected static double[] getFossilChance(Structures.TileGrid tiles, String percentage)
    pub fn get_fossil_chance(tiles: &TileGrid, percentage: Option<&str>) -> Vec<f64> {
        let mut total: Vec<i32> = vec![0; (EXCAVATOR_WIDTH.wrapping_mul(EXCAVATOR_HEIGHT)) as usize];
        compat::set_state(&MINIMUM_TILES, EXCAVATOR_WIDTH.wrapping_mul(EXCAVATOR_HEIGHT));
        let mut fossil_count: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);
        for row in tiles.state().iter().cloned() {
            for tile in row.iter().copied() {
                if tile == TileState::Fossil {
                    (fossil_count.fetch_add(1, std::sync::atomic::Ordering::SeqCst));
                }
            }
        }
        let mut valid_states: Vec<permutation> = Vec::new();
        for state in POSSIBLE_STATES.iter() {
            if state.is_valid(tiles, percentage) {
                valid_states.push(state.clone());
                let mut min: i32 = ((state.r#type()).fields().tile_count).wrapping_sub(fossil_count.load(std::sync::atomic::Ordering::SeqCst));
                if min < compat::get_state(&MINIMUM_TILES) {
                    compat::set_state(&MINIMUM_TILES, min);
                }
            }
        }
        compat::set_state(&PERMUTATIONS, (valid_states.len() as i32));
        if valid_states.is_empty() {
            compat::set_state(&FOSSIL_NAME, None);
        } else {
            compat::set_state(&FOSSIL_NAME, Some((valid_states[0].r#type()).fields().name.clone()));
            for fossil in valid_states.iter() {
                if !(Some(&*((fossil.r#type()).fields().name)) == compat::get_state(&FOSSIL_NAME).as_deref()) {
                    compat::set_state(&FOSSIL_NAME, None);
                    break;
                }
            }
        }
        let mut index: i32 = 0;
        for y in 0..EXCAVATOR_HEIGHT {
            for x in 0..EXCAVATOR_WIDTH {
                if tiles.get_slot(x, y) == TileState::Unknown {
                    for state in valid_states.iter() {
                        if state.is_fossil_collision(x, y) {
                            total[(index) as usize] = (total[(index) as usize]).wrapping_add(1);
                        }
                    }
                }
                index += 1;
            }
        }
        return total.iter().copied().map(|x| (((x) as f64) / ((valid_states.len() as i32) as f64))).collect::<Vec<_>>();
    }

    /// java: protected static Structures.TileGrid convertItemsToTiles(Int2ObjectMap<ItemStack> currentState)
    pub fn convert_items_to_tiles(current_state: &std::collections::BTreeMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> TileGrid {
        let mut output: TileGrid = TileGrid::of_size(EXCAVATOR_WIDTH, EXCAVATOR_HEIGHT);
        let mut index: i32 = 0;
        for y in 0..EXCAVATOR_HEIGHT {
            for x in 0..EXCAVATOR_WIDTH {
                let mut item: Arc<Item> = current_state.get(&index).cloned().unwrap().get_item();
                if std::ptr::addr_eq(std::sync::Arc::as_ptr(&item), std::sync::Arc::as_ptr(mcrust::net::minecraft::world::item::items::stained_glass_pane().white())) {
                    output.update_slot(x, y, TileState::Fossil);
                } else if std::ptr::addr_eq(std::sync::Arc::as_ptr(&item), std::sync::Arc::as_ptr(mcrust::net::minecraft::world::item::items::stained_glass_pane().brown())) {
                    output.update_slot(x, y, TileState::Unknown);
                } else {
                    output.update_slot(x, y, TileState::Empty);
                }
                index += 1;
            }
        }
        return output;
    }

    /// java: protected static List<Structures.permutation> getAllPossibleStates()
    pub fn get_all_possible_states() -> Vec<permutation> {
        let mut output: Vec<permutation> = Vec::new();
        for fossil in FossilTypes::values() {
            for rotation in fossil.fields().rotations.iter().copied() {
                let mut grid: TileGrid = FossilCalculations::transform_grid(&TileGrid::new(fossil.fields().grid.clone()), rotation);
                let mut max_x_offset: i32 = EXCAVATOR_WIDTH.wrapping_sub(grid.width());
                let mut max_y_offset: i32 = EXCAVATOR_HEIGHT.wrapping_sub(grid.height());
                for x in 0..=max_x_offset {
                    for y in 0..=max_y_offset {
                        output.push(permutation::new(fossil, grid.clone(), x, y));
                    }
                }
            }
        }
        return output;
    }

    /// java: private static Structures.TileGrid transformGrid(Structures.TileGrid grid, Structures.TransformationOptions transformation)
    pub fn transform_grid(grid: &TileGrid, mut transformation: TransformationOptions) -> TileGrid {
        match transformation {
            TransformationOptions::Rotated90 => {
                return FossilCalculations::rotate_grid(grid, 90);
            }
            TransformationOptions::Rotated180 => {
                return FossilCalculations::rotate_grid(grid, 180);
            }
            TransformationOptions::Rotated270 => {
                return FossilCalculations::rotate_grid(grid, 270);
            }
            TransformationOptions::FlipRotated0 => {
                return FossilCalculations::flip_grid(grid);
            }
            TransformationOptions::FlipRotated90 => {
                return FossilCalculations::rotate_grid(&FossilCalculations::flip_grid(grid), 90);
            }
            TransformationOptions::FlipRotated180 => {
                return FossilCalculations::rotate_grid(&FossilCalculations::flip_grid(grid), 180);
            }
            TransformationOptions::FlipRotated270 => {
                return FossilCalculations::rotate_grid(&FossilCalculations::flip_grid(grid), 270);
            }
            _ => {
                return grid.clone();
            }
        }
    }

    /// java: protected static Structures.TileGrid flipGrid(Structures.TileGrid grid)
    pub fn flip_grid(grid: &TileGrid) -> TileGrid {
        let mut output: TileGrid = TileGrid::of_size(grid.width(), grid.height());
        for x in 0..grid.width() {
            for y in 0..grid.height() {
                output.update_slot(x, y, grid.get_slot(x, (grid.height().wrapping_sub(1)).wrapping_sub(y)));
            }
        }
        return output;
    }

    /// java: protected static Structures.TileGrid rotateGrid(Structures.TileGrid grid, int rotation)
    pub fn rotate_grid(grid: &TileGrid, mut rotation: i32) -> TileGrid {
        let mut starting_width: i32 = grid.width().wrapping_sub(1);
        let mut starting_height: i32 = grid.height().wrapping_sub(1);
        match rotation {
            90 => {
                let mut output: TileGrid = TileGrid::of_size(grid.height(), grid.width());
                for original_x in 0..grid.width() {
                    for original_y in 0..grid.height() {
                        output.update_slot(starting_height.wrapping_sub(original_y), original_x, grid.get_slot(original_x, original_y));
                    }
                }
                return output.clone();
            }
            180 => {
                let mut output: TileGrid = TileGrid::of_size(grid.width(), grid.height());
                for x in 0..grid.width() {
                    for y in 0..grid.height() {
                        output.update_slot(starting_width.wrapping_sub(x), starting_height.wrapping_sub(y), grid.get_slot(x, y));
                    }
                }
                return output.clone();
            }
            270 => {
                let mut output: TileGrid = TileGrid::of_size(grid.height(), grid.width());
                for original_x in 0..grid.width() {
                    for original_y in 0..grid.height() {
                        output.update_slot(original_y, starting_width.wrapping_sub(original_x), grid.get_slot(original_x, original_y));
                    }
                }
                return output.clone();
            }
            _ => {
                return grid.clone();
            }
        }
    }

}

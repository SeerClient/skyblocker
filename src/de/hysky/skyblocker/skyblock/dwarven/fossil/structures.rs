//! Ported from de.hysky.skyblocker.skyblock.dwarven.fossil.Structures by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::dwarven::fossil::fossil_types::FossilTypes;

/// java: public class Structures
#[derive(Clone)]
pub struct Structures {}

impl Structures {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

}

/// java: protected enum TileState
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TileState {
    Unknown,
    Empty,
    Fossil,
}

impl TileState {
    pub const VALUES: [TileState; 3] = [TileState::Unknown, TileState::Empty, TileState::Fossil];

    /// java: values()
    pub fn values() -> Vec<TileState> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            TileState::Unknown => "UNKNOWN",
            TileState::Empty => "EMPTY",
            TileState::Fossil => "FOSSIL",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> TileState {
        match name {
            "UNKNOWN" => TileState::Unknown,
            "EMPTY" => TileState::Empty,
            "FOSSIL" => TileState::Fossil,
            _ => panic!("IllegalArgumentException: No enum constant TileState.{name}"),
        }
    }

}

impl std::fmt::Display for TileState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// java: protected enum TransformationOptions
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TransformationOptions {
    Rotated0,
    Rotated90,
    Rotated180,
    Rotated270,
    FlipRotated0,
    FlipRotated90,
    FlipRotated180,
    FlipRotated270,
}

impl TransformationOptions {
    pub const VALUES: [TransformationOptions; 8] = [TransformationOptions::Rotated0, TransformationOptions::Rotated90, TransformationOptions::Rotated180, TransformationOptions::Rotated270, TransformationOptions::FlipRotated0, TransformationOptions::FlipRotated90, TransformationOptions::FlipRotated180, TransformationOptions::FlipRotated270];

    /// java: values()
    pub fn values() -> Vec<TransformationOptions> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            TransformationOptions::Rotated0 => "ROTATED_0",
            TransformationOptions::Rotated90 => "ROTATED_90",
            TransformationOptions::Rotated180 => "ROTATED_180",
            TransformationOptions::Rotated270 => "ROTATED_270",
            TransformationOptions::FlipRotated0 => "FLIP_ROTATED_0",
            TransformationOptions::FlipRotated90 => "FLIP_ROTATED_90",
            TransformationOptions::FlipRotated180 => "FLIP_ROTATED_180",
            TransformationOptions::FlipRotated270 => "FLIP_ROTATED_270",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> TransformationOptions {
        match name {
            "ROTATED_0" => TransformationOptions::Rotated0,
            "ROTATED_90" => TransformationOptions::Rotated90,
            "ROTATED_180" => TransformationOptions::Rotated180,
            "ROTATED_270" => TransformationOptions::Rotated270,
            "FLIP_ROTATED_0" => TransformationOptions::FlipRotated0,
            "FLIP_ROTATED_90" => TransformationOptions::FlipRotated90,
            "FLIP_ROTATED_180" => TransformationOptions::FlipRotated180,
            "FLIP_ROTATED_270" => TransformationOptions::FlipRotated270,
            _ => panic!("IllegalArgumentException: No enum constant TransformationOptions.{name}"),
        }
    }

}

impl std::fmt::Display for TransformationOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// java: protected record TileGrid(TileState[][] state)
#[derive(Clone, Debug, PartialEq)]
pub struct TileGrid {
    /// java: TileState[][] state
    pub state: Vec<Vec<TileState>>,
}

impl TileGrid {
    /// java: the canonical record constructor
    pub fn new(mut state: Vec<Vec<TileState>>) -> Self {
        Self { state }
    }

    /// java: the record accessor `state()`
    pub fn state(&self) -> Vec<Vec<TileState>> {
        self.state.clone()
    }

    /// java: void updateSlot(int x, int y, TileState newState)
    pub fn update_slot(&self, mut x: i32, mut y: i32, mut new_state: TileState) {
        (self.state[(y) as usize])[(x) as usize] = new_state;
    }

    /// java: TileState getSlot(int x, int y)
    pub fn get_slot(&self, mut x: i32, mut y: i32) -> TileState {
        return (self.state[(y) as usize])[(x) as usize];
    }

    /// java: int width()
    pub fn width(&self) -> i32 {
        return ((self.state[(0) as usize]).len() as i32);
    }

    /// java: int height()
    pub fn height(&self) -> i32 {
        return (self.state.len() as i32);
    }

    /// java: @Override public boolean equals(Object o)
    pub fn equals(&self, o: &compat::Object) -> bool {
        if self == o {
            return true;
        }
        // TODO(fabric2seer): not translated (expected ')' at '(' (offset 949))
        //   java: if (o instanceof TileGrid(TileState[][] state2)) {
        //   java: 				return Arrays.deepEquals(state, state2);
        //   java: 			}
        return false;
    }

    /// java: @Override public int hashCode()
    pub fn hash_code(&self) -> i32 {
        // TODO(fabric2seer): Arrays.deepHashCode
        //   java: return Arrays.deepHashCode(state);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}

/// java: protected record permutation(FossilTypes type, TileGrid grid, int xOffset, int yOffset)
#[derive(Clone, Debug, PartialEq)]
pub struct permutation {
    /// java: FossilTypes type
    pub r#type: FossilTypes,
    /// java: TileGrid grid
    pub grid: TileGrid,
    /// java: int xOffset
    pub x_offset: i32,
    /// java: int yOffset
    pub y_offset: i32,
}

impl permutation {
    /// java: the canonical record constructor
    pub fn new(mut r#type: FossilTypes, mut grid: TileGrid, mut x_offset: i32, mut y_offset: i32) -> Self {
        Self { r#type, grid, x_offset, y_offset }
    }

    /// java: the record accessor `type()`
    pub fn r#type(&self) -> FossilTypes {
        self.r#type
    }

    /// java: the record accessor `grid()`
    pub fn grid(&self) -> TileGrid {
        self.grid.clone()
    }

    /// java: the record accessor `xOffset()`
    pub fn x_offset(&self) -> i32 {
        self.x_offset
    }

    /// java: the record accessor `yOffset()`
    pub fn y_offset(&self) -> i32 {
        self.y_offset
    }

    /// java: boolean isValid(TileGrid currentState, String percentage)
    pub fn is_valid(&self, current_state: &TileGrid, percentage: &str) -> bool {
        if true /* java: percentage != null: never null here */ && !(percentage == (self.r#type).fields().percentage) {
            return false;
        }
        for x in 0..current_state.width() {
            for y in 0..current_state.height() {
                let mut known_state: TileState = current_state.get_slot(x, y);
                match known_state {
                    TileState::Unknown => {
                        continue;
                    }
                    TileState::Fossil => {
                        if !(self.is_fossil_collision(x, y)) {
                            return false;
                        }
                    }
                    TileState::Empty => {
                        if !(self.is_empty_collision(x, y)) {
                            return false;
                        }
                    }
                    _ => {}
                }
            }
        }
        return true;
    }

    /// java: private boolean isEmptyCollision(int positionX, int positionY)
    pub fn is_empty_collision(&self, mut position_x: i32, mut position_y: i32) -> bool {
        // java: try {..} catch (IndexOutOfBoundsException _) - an exception here is a panic, caught by `catch_unwind`; the body's `return` is the closure's value
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            return self.is_state(position_x, position_y, TileState::Empty);
        })) {
            Ok(__v) => return __v,
            Err(_) => {
                return true;
            }
        }
    }

    /// java: boolean isFossilCollision(int positionX, int positionY)
    pub fn is_fossil_collision(&self, mut position_x: i32, mut position_y: i32) -> bool {
        // java: try {..} catch (IndexOutOfBoundsException _) - an exception here is a panic, caught by `catch_unwind`; the body's `return` is the closure's value
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            return self.is_state(position_x, position_y, TileState::Fossil);
        })) {
            Ok(__v) => return __v,
            Err(_) => {
                return false;
            }
        }
    }

    /// java: private boolean isState(int positionX, int positionY, TileState state)
    pub fn is_state(&self, mut position_x: i32, mut position_y: i32, mut state: TileState) -> bool {
        let mut x: i32 = position_x.wrapping_sub(self.x_offset);
        let mut y: i32 = position_y.wrapping_sub(self.y_offset);
        if x < 0 || x >= self.grid.width() || y < 0 || y >= self.grid.height() {
            panic!("IndexOutOfBoundsException: {}", "not in grid");
        }
        return (self.grid.get_slot(x, y) == state);
    }

}

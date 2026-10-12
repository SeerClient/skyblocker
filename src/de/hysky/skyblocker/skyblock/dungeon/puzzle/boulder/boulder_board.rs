//! Ported from de.hysky.skyblocker.skyblock.dungeon.puzzle.boulder.BoulderBoard by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::dungeon::puzzle::boulder::boulder;
use crate::de::hysky::skyblocker::skyblock::dungeon::puzzle::boulder::boulder_object::BoulderObject;
use mcrust::net::minecraft::core::block_pos::BlockPos;
use mcrust::net::minecraft::core::direction::Direction;

/// java: public class BoulderBoard
#[derive(Clone)]
pub struct BoulderBoard {
    /// java: private final int height;
    pub height: i32,
    /// java: private final int width;
    pub width: i32,
    /// java: private final BoulderObject[][] grid;
    pub grid: Vec<Vec<BoulderObject>>,
}

impl BoulderBoard {
    /// java: public BoulderBoard(int height, int width)
    pub fn new(mut height: i32, mut width: i32) -> Self {
        let __init_height = height;
        let __init_width = width;
        let __init_grid = vec![(0..(width) as usize).map(|_| Default::default()).collect::<Vec<_>>(); (height) as usize];
        let mut this = Self {
            height: __init_height,
            width: __init_width,
            grid: __init_grid,
        };
        for z in 0..width {
            (this.grid[(height.wrapping_sub(1)) as usize])[(z) as usize] = BoulderObject::new(24i32.wrapping_sub(3i32.wrapping_mul(z)), boulder::BASE_Y, 6, "P".to_string());
        }
        this
    }

    /// java: public BoulderObject getObjectAtPosition(int x, int y)
    pub fn get_object_at_position(&self, mut x: i32, mut y: i32) -> BoulderObject {
        if !(self.is_valid_position(x, y)) {
            panic!("IllegalArgumentException: {}", format!("Invalid position: ({}, {})", x, y).to_string());
        }
        return (self.grid[(x) as usize])[(y) as usize].clone();
    }

    /// java: public BlockPos getObject3DPosition(int x, int y)
    pub fn get_object3_d_position(&self, mut x: i32, mut y: i32) -> BlockPos {
        // TODO(fabric2seer): field `Y` of a value of unknown type
        //   java: return getObjectAtPosition(x, y).get3DPosition().relative(Direction.Axis.Y, -1);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public void placeObject(int x, int y, BoulderObject object)
    pub fn place_object(&self, mut x: i32, mut y: i32, mut object: BoulderObject) {
        (self.grid[(x) as usize])[(y) as usize] = object.clone();
    }

    /// java: public int getHeight()
    pub fn get_height(&self) -> i32 {
        return self.height;
    }

    /// java: public int getWidth()
    pub fn get_width(&self) -> i32 {
        return self.width;
    }

    /// java: private boolean isValidPosition(int x, int y)
    pub fn is_valid_position(&self, mut x: i32, mut y: i32) -> bool {
        return (x >= 0 && y >= 0 && x < self.height && y < self.width);
    }

    /// java: public char[][] getBoardCharArray()
    pub fn get_board_char_array(&self) -> Vec<Vec<char>> {
        let mut board_char_array: Vec<Vec<char>> = vec![vec!['\0'; (self.width) as usize]; (self.height) as usize];
        for x in 0..self.height {
            for y in 0..self.width {
                let mut boulder_object: Option<BoulderObject> = Some((self.grid[(x) as usize])[(y) as usize].clone());
                (board_char_array[(x) as usize])[(y) as usize] = (if boulder_object.is_some() { compat::char_at(&(boulder_object.as_ref().unwrap().r#type()), 0) } else { '.' });
            }
        }
        return board_char_array;
    }

    /// java: public String boardToString()
    pub fn board_to_string(&self) -> String {
        let mut sb: String = String::new();
        for x in 0..self.height {
            for y in 0..self.width {
                let mut boulder_object: Option<BoulderObject> = Some((self.grid[(x) as usize])[(y) as usize].clone());
                let mut display_char: String = (if boulder_object.is_some() { boulder_object.as_ref().unwrap().r#type() } else { ".".to_string() });
                { sb.push_str(&display_char); };
            }
            { sb.push_str("\n"); };
        }
        return sb.clone();
    }

}

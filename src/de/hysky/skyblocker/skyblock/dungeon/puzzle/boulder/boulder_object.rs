//! Ported from de.hysky.skyblocker.skyblock.dungeon.puzzle.boulder.BoulderObject by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::core::block_pos::BlockPos;

/// java: public record BoulderObject(int x, int y, int z, String type)
#[derive(Clone, Debug, PartialEq)]
pub struct BoulderObject {
    /// java: int x
    pub x: i32,
    /// java: int y
    pub y: i32,
    /// java: int z
    pub z: i32,
    /// java: String type
    pub r#type: String,
}

impl BoulderObject {
    /// java: the canonical record constructor
    pub fn new(mut x: i32, mut y: i32, mut z: i32, mut r#type: String) -> Self {
        Self { x, y, z, r#type }
    }

    /// java: the record accessor `x()`
    pub fn x(&self) -> i32 {
        self.x
    }

    /// java: the record accessor `y()`
    pub fn y(&self) -> i32 {
        self.y
    }

    /// java: the record accessor `z()`
    pub fn z(&self) -> i32 {
        self.z
    }

    /// java: the record accessor `type()`
    pub fn r#type(&self) -> String {
        self.r#type.clone()
    }

    /// java: public BlockPos get3DPosition()
    pub fn get3_d_position(&self) -> BlockPos {
        return BlockPos::new(self.x, self.y, self.z);
    }

}

//! Ported from de.hysky.skyblocker.skyblock.dungeon.preview.SkeletonBlock by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: public record SkeletonBlock(int x, int y, int z, byte blockType)
#[derive(Clone, Debug, PartialEq)]
pub struct SkeletonBlock {
    /// java: int x
    pub x: i32,
    /// java: int y
    pub y: i32,
    /// java: int z
    pub z: i32,
    /// java: byte blockType
    pub block_type: i8,
}

impl SkeletonBlock {
    /// java: the canonical record constructor
    pub fn new(mut x: i32, mut y: i32, mut z: i32, mut block_type: i8) -> Self {
        Self { x, y, z, block_type }
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

    /// java: the record accessor `blockType()`
    pub fn block_type(&self) -> i8 {
        self.block_type
    }

    /// java: public static SkeletonBlock from(int blockNum)
    pub fn from(mut block_num: i32) -> SkeletonBlock {
        return SkeletonBlock::new((block_num.wrapping_shr((24) as u32) & 255), (block_num.wrapping_shr((16) as u32) & 255), (block_num.wrapping_shr((8) as u32) & 255), (((block_num & 255)) as i8));
    }

    /// java: public int compress()
    pub fn compress(&self) -> i32 {
        return (((self.x.wrapping_shl((24) as u32) | self.y.wrapping_shl((16) as u32)) | self.z.wrapping_shl((8) as u32)) | (self.block_type as i32));
    }

}

//! Ported from de.hysky.skyblocker.skyblock.dungeon.preview.RoomStructure by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::dungeon::preview::skeleton_block::SkeletonBlock;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::dungeon_manager;
use mcrust::net::minecraft::nbt::int_tag;
use mcrust::net::minecraft::nbt::list_tag;
use mcrust::net::minecraft::nbt::tag::Tag;

/// java: public class RoomStructure
#[derive(Clone)]
pub struct RoomStructure {}

impl RoomStructure {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: static CompoundTag getCompound(int[] blocks)
    pub fn get_compound(blocks: &Vec<i32>) -> Tag {
        return RoomStructure::create_compound(&RoomStructure::create_block_list(blocks));
    }

    /// java: public static List<SkeletonBlock> createBlockList(int[] blocks)
    pub fn create_block_list(blocks: &Vec<i32>) -> Vec<SkeletonBlock> {
        return blocks.iter().copied().map(|__a0| SkeletonBlock::from(__a0)).collect::<Vec<_>>();
    }

    /// java: public static CompoundTag createCompound(List<SkeletonBlock> blockData)
    pub fn create_compound(block_data: &Vec<SkeletonBlock>) -> Tag {
        let mut structure: Tag = mcrust::net::minecraft::nbt::compound_tag::Tag::compound();
        structure.put("size", RoomStructure::get_size(block_data));
        let mut sorted_palette: Vec<Tag> = (0..((dungeon_manager::NUMERIC_ID.len() as i32)) as usize).map(|_| Default::default()).collect::<Vec<_>>();
        // TODO(fabric2seer): Map.object2ByteEntrySet(..)
        //   java: for (var entry : DungeonManager.NUMERIC_ID.object2ByteEntrySet()) {
        //   java: 			CompoundTag paletteBlock = new CompoundTag();
        //   java: 			paletteBlock.put("Name", StringTag.valueOf(entry.getKey()));
        //   java: 			sortedPalette[entry.getByteValue() - 1] = paletteBlock; // Subtract 1 because NUMERIC_ID starts at 1.
        //   java: 		}
        // TODO(fabric2seer): `new ListTag(..)`: the port has no `ListTag::new`
        // rust: let mut palette: Tag = Tag::new();
        let mut palette: Tag = todo!("fabric2seer: the initialiser of `palette` (TODO above)");
        // TODO(fabric2seer): `ListTag.addAll` has no Rust method in the port (looked for `add_all` in ListTag and JavaSrc)
        // rust: palette.add_all(&sorted_palette.to_vec());
        structure.put("palette", palette.clone());
        // TODO(fabric2seer): `new ListTag(..)`: the port has no `ListTag::new`
        // rust: let mut blocks: Tag = Tag::new();
        let mut blocks: Tag = todo!("fabric2seer: the initialiser of `blocks` (TODO above)");
        for block in block_data.iter() {
            let mut nbt_block: Tag = mcrust::net::minecraft::nbt::compound_tag::Tag::compound();
            // TODO(fabric2seer): `new ListTag(..)`: the port has no `ListTag::new`
            // rust: let mut pos_list: Tag = Tag::new();
            let mut pos_list: Tag = todo!("fabric2seer: the initialiser of `posList` (TODO above)");
            // TODO(fabric2seer): `add` takes 3 argument(s) in the port (`elements: &mut Vec<Tag>, index: usize, tag: Tag`), Java passes 1
            // rust: list_tag::add(&mut int_tag::value_of(block.x()));
            // TODO(fabric2seer): `add` takes 3 argument(s) in the port (`elements: &mut Vec<Tag>, index: usize, tag: Tag`), Java passes 1
            // rust: list_tag::add(&mut int_tag::value_of(block.y()));
            // TODO(fabric2seer): `add` takes 3 argument(s) in the port (`elements: &mut Vec<Tag>, index: usize, tag: Tag`), Java passes 1
            // rust: list_tag::add(&mut int_tag::value_of(block.z()));
            nbt_block.put("pos", pos_list.clone());
            nbt_block.put("state", int_tag::value_of((block.block_type() as i32).wrapping_sub(1)));
            // TODO(fabric2seer): `add` takes 3 argument(s) in the port (`elements: &mut Vec<Tag>, index: usize, tag: Tag`), Java passes 1
            // rust: list_tag::add(&mut nbt_block);
        }
        structure.put("blocks", blocks.clone());
        return structure;
    }

    /// java: static ListTag getSize(List<SkeletonBlock> blockData)
    pub fn get_size(block_data: &Vec<SkeletonBlock>) -> Tag {
        let mut min_x: i32 = block_data[0].x();
        let mut min_y: i32 = 0;
        let mut min_z: i32 = block_data[0].z();
        let mut max_x: i32 = (block_data[block_data.len() - 1]).x();
        let mut max_y: i32 = (block_data[block_data.len() - 1]).y();
        let mut max_z: i32 = (block_data[block_data.len() - 1]).z();
        for block in block_data.iter() {
            if block.x() > max_x {
                max_x = block.x();
            }
            if block.x() < min_x {
                min_x = block.x();
            }
            if block.y() > max_y {
                max_y = block.y();
            }
            if block.y() < min_y {
                min_y = block.y();
            }
            if block.z() > max_z {
                max_z = block.z();
            }
            if block.z() < min_z {
                min_z = block.z();
            }
        }
        // TODO(fabric2seer): `new ListTag(..)`: the port has no `ListTag::new`
        // rust: let mut size_list: Tag = Tag::new();
        let mut size_list: Tag = todo!("fabric2seer: the initialiser of `sizeList` (TODO above)");
        // TODO(fabric2seer): `add` takes 3 argument(s) in the port (`elements: &mut Vec<Tag>, index: usize, tag: Tag`), Java passes 1
        // rust: list_tag::add(&mut int_tag::value_of(max_x.wrapping_sub(min_x)));
        // TODO(fabric2seer): `add` takes 3 argument(s) in the port (`elements: &mut Vec<Tag>, index: usize, tag: Tag`), Java passes 1
        // rust: list_tag::add(&mut int_tag::value_of(max_y.wrapping_sub(min_y)));
        // TODO(fabric2seer): `add` takes 3 argument(s) in the port (`elements: &mut Vec<Tag>, index: usize, tag: Tag`), Java passes 1
        // rust: list_tag::add(&mut int_tag::value_of(max_z.wrapping_sub(min_z)));
        return size_list;
    }

}

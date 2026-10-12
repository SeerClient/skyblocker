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
        let mut structure: Tag = Tag::compound();
        structure.put("size", RoomStructure::get_size(block_data));
        // CompoundTag[] sortedPalette = new CompoundTag[DungeonManager.NUMERIC_ID.size()];
        let mut sorted_palette: Vec<Option<Tag>> = vec![None; dungeon_manager::NUMERIC_ID.len()];
        for (key, byte_value) in dungeon_manager::NUMERIC_ID.iter() {
            let mut palette_block: Tag = Tag::compound();
            palette_block.put("Name", Tag::String(key.clone()));
            // Subtract 1 because NUMERIC_ID starts at 1.
            sorted_palette[((*byte_value as i32) - 1) as usize] = Some(palette_block);
        }
        // ListTag palette = new ListTag(); palette.addAll(Arrays.asList(sortedPalette));
        let palette_elements: Vec<Tag> = sorted_palette.into_iter().map(|t| t.expect("NUMERIC_ID's ids are 1..=size")).collect();
        let palette: Tag = Tag::List(list_tag::identify_raw_element_type(&palette_elements), palette_elements.into());
        structure.put("palette", palette);
        let mut block_elements: Vec<Tag> = Vec::new();
        for block in block_data.iter() {
            let mut nbt_block: Tag = Tag::compound();
            let pos_elements: Vec<Tag> = vec![int_tag::value_of(block.x()), int_tag::value_of(block.y()), int_tag::value_of(block.z())];
            let pos_list: Tag = Tag::List(list_tag::identify_raw_element_type(&pos_elements), pos_elements.into());
            nbt_block.put("pos", pos_list);
            // Subtract 1 because NUMERIC_ID starts at 1.
            nbt_block.put("state", int_tag::value_of((block.block_type() as i32).wrapping_sub(1)));
            let index = block_elements.len();
            list_tag::add(&mut block_elements, index, nbt_block);
        }
        let blocks: Tag = Tag::List(list_tag::identify_raw_element_type(&block_elements), block_elements.into());
        structure.put("blocks", blocks);
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
        let size_elements: Vec<Tag> = vec![int_tag::value_of(max_x.wrapping_sub(min_x)), int_tag::value_of(max_y.wrapping_sub(min_y)), int_tag::value_of(max_z.wrapping_sub(min_z))];
        let mut size_list: Tag = Tag::List(list_tag::identify_raw_element_type(&size_elements), size_elements.into());
        return size_list;
    }

}

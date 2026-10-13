//! Ported from de.hysky.skyblocker.utils.networth.SkyblockItemMetadataRetriever by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::nbt::tag::Tag;

/// java: record SkyblockItemMetadataRetriever(IntList cakeBagCakeYears) implements ItemMetadataRetriever
#[derive(Clone, Debug, PartialEq)]
pub struct SkyblockItemMetadataRetriever {
    /// java: IntList cakeBagCakeYears
    pub cake_bag_cake_years: Vec<i32>,
}

impl SkyblockItemMetadataRetriever {
    /// java: the canonical record constructor
    pub fn new(mut cake_bag_cake_years: Vec<i32>) -> Self {
        Self { cake_bag_cake_years }
    }

    /// java: the record accessor `cakeBagCakeYears()`
    pub fn cake_bag_cake_years(&self) -> Vec<i32> {
        self.cake_bag_cake_years.clone()
    }

    /// java: static SkyblockItemMetadataRetriever of(CompoundTag customData, String itemId)
    pub fn of(custom_data: &Tag, item_id: &str) -> SkyblockItemMetadataRetriever {
        return SkyblockItemMetadataRetriever::new(SkyblockItemMetadataRetriever::get_cake_bag_cake_years(custom_data, item_id));
    }

    /// java: private static IntList getCakeBagCakeYears(CompoundTag customData, String itemId)
    pub fn get_cake_bag_cake_years(custom_data: &Tag, item_id: &str) -> Vec<i32> {
        if item_id == "NEW_YEAR_CAKE_BAG" && custom_data.contains("new_year_cake_bag_data") {
            // java: try {..} catch (Exception _) - an exception here is a panic, caught by `catch_unwind`; the body's `return` is the closure's value
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                // java: CompoundTag uncompressed = NbtIo.readCompressed(new ByteArrayInputStream(customData.getByteArray("new_year_cake_bag_data").orElse(new byte[0])), NbtAccounter.unlimitedHeap());
                let bytes: Vec<u8> = custom_data.get_byte_array("new_year_cake_bag_data").unwrap_or(&[]).iter().map(|b| *b as u8).collect();
                let mut uncompressed: Tag = mcrust::net::minecraft::nbt::nbt_io::read_compressed(&bytes, mcrust::net::minecraft::nbt::nbt_accounter::NbtAccounter::unlimited_heap()).unwrap_or_else(|e| panic!("IOException: {e}"));
                // java: ListTag items = uncompressed.getListOrEmpty("i");
                let mut items: Vec<Tag> = uncompressed.get_list_or_empty("i");
                let mut cake_years: Vec<i32> = Vec::new();
                for element in items.iter() {
                    // java: if (element instanceof CompoundTag compound && compound.getCompoundOrEmpty("tag").contains("ExtraAttributes"))
                    if let Some(compound) = Some(element).filter(|element| matches!(element, Tag::Compound(_))) && compound.get_compound_or_empty("tag").contains("ExtraAttributes") {
                        let mut extra_attributes: Tag = compound.get_compound_or_empty("tag").get_compound_or_empty("ExtraAttributes");
                        let mut cake_year: i32 = extra_attributes.get_int_or("new_years_cake", 0);
                        cake_years.push(cake_year);
                    }
                }
                return cake_years;
            })) {
                Ok(__v) => return __v,
                Err(_) => {
                }
            }
        }
        return vec![];
    }

}

/// java: record SkyblockItemMetadataRetriever(..) implements ItemMetadataRetriever
impl crate::net::azureaaron::networth::item::item_metadata_retriever::ItemMetadataRetriever for SkyblockItemMetadataRetriever {
    /// java: the record accessor `cakeBagCakeYears()`, which is the interface method
    fn cake_bag_cake_years(&self) -> Vec<i32> {
        self.cake_bag_cake_years.clone()
    }
}

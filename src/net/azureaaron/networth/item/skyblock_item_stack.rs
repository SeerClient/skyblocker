//! java: public record SkyblockItemStack(String id, int count, ItemMetadata metadata) (net.azureaaron.networth.item)
//!
//! To construct an instance, use the static factory provided. The record's signature is an implementation detail and
//! subject to change.

use mcrust::com::mojang::serialization::dynamic::Dynamic;

use super::dynamic_item_metadata::DynamicItemMetadata;
use super::item_metadata::ItemMetadata;
use super::item_metadata_retriever::ItemMetadataRetriever;

/// java: public record SkyblockItemStack(String id, int count, ItemMetadata metadata)
pub struct SkyblockItemStack {
    pub id: String,
    pub count: i32,
    pub metadata: Box<dyn ItemMetadata>,
}

impl SkyblockItemStack {
    /// java: the canonical record constructor
    pub fn new(id: String, count: i32, metadata: Box<dyn ItemMetadata>) -> Self {
        Self { id, count, metadata }
    }

    /// java: public static SkyblockItemStack of(String id, int count, Dynamic<?> customData, ItemMetadataRetriever retriever)
    ///
    /// Returns a SkyblockItemStack instance for use with the `ItemCalculator`. All parameters must not be null.
    pub fn of(
        id: &str,
        count: i32,
        custom_data: &Dynamic,
        retriever: &dyn ItemMetadataRetriever,
    ) -> SkyblockItemStack {
        // java: id.toUpperCase(Locale.CANADA)
        SkyblockItemStack::new(
            id.to_uppercase(),
            count,
            Box::new(DynamicItemMetadata::of(custom_data, retriever)),
        )
    }

    /// java: the record accessor `id()`
    pub fn id(&self) -> &str {
        &self.id
    }

    /// java: the record accessor `count()`
    pub fn count(&self) -> i32 {
        self.count
    }

    /// java: the record accessor `metadata()`
    pub fn metadata(&self) -> &dyn ItemMetadata {
        self.metadata.as_ref()
    }
}

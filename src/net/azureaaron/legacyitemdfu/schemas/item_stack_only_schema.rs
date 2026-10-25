//! Ported from net.azureaaron.legacyitemdfu.schemas.ItemStackOnlySchema (legacy-item-dfu 1.0.4+26.2).
//!
//! ============ THE ITEM STACK TYPE, AND WHAT THE GAME'S DFU CAN BUILD ============
//! The library's three schemas declare `LEGACY_ITEM_STACK` as `DSL.optionalFields("id", ITEM_NAME | int, "tag",
//! optionalFields("BlockEntityTag", remainder))` (and `"id", "components"` from version 12), with `ITEM_NAME` a
//! `Const.PrimitiveType` over `IdentifierNormalizingSchema.CODEC`. The game's DataFixerUpper port builds exactly two
//! kinds of type - `DSL.remainder()` and `DSL.named(..)` (`mcrust::com::mojang::datafixers::dsl`'s header: `and`,
//! `or`, `optional`, `field`, the primitive `Const.PrimitiveType`s and the recursive family are "NOT HERE"), and its
//! `Schema.registerType(true, ..)` panics. That crate is read-only from the mod.
//!
//! So every schema here registers the two references as what that DFU can express: `ITEM_NAME` as
//! `constType(remainderType())` and `LEGACY_ITEM_STACK` as `remainder()` (which `Schema.getTemplate` wraps in
//! `DSL.named("legacy_item_stack", ..)`, so the input and output types of every fix still compare equal, as the
//! library's do), registered non-recursive (`LEGACY_ITEM_STACK` never refers to itself, so the recursive family Java
//! builds for it has one member and changes no value). Each fix then reaches the fields it names through the
//! REMAINDER - the whole stack `Dynamic` - and does to `"id"` / `"tag"` / `"tag".BlockEntityTag` / `"components"`
//! what the library's optics do (each fix's file says how); and the one thing the typed read does on its own, the
//! `ITEM_NAME` read through `IdentifierNormalizingSchema.CODEC` (which normalises a string id), runs where the first
//! fix to read the id reads it (`numeric_item_id_fix.rs`). Every fix is still a `DataFix` registered on a
//! `DataFixerBuilder` and run by the game's `DataFixerUpper.update`.

use mcrust::com::mojang::datafixers::schemas::schema::{RegisterTypes, Schema};
use std::sync::Arc;

/// java: public sealed abstract class ItemStackOnlySchema extends Schema permits Schema1, Schema2, Schema11
///
/// The game's `Schema` carries its `registerTypes` override as a function (`Schema::with_register_types`), so a
/// subclass is the constructor below handed that subclass's `registerTypes`.
pub struct ItemStackOnlySchema;

impl ItemStackOnlySchema {
    /// java: public ItemStackOnlySchema(int versionKey, Schema parent) - `super(versionKey, parent)`, with the
    /// subclass's `registerTypes(Schema schema, Map entityTypes, Map blockEntityTypes)` (the abstract method).
    ///
    /// java: `registerEntities` / `registerBlockEntities` are `final` and answer `Map.of()`. The game's `Schema`
    /// constructor runs no entity or block-entity registration at all (schema.rs: the choice maps are not ported),
    /// which is what an empty map registers.
    pub fn new(version_key: i32, parent: Option<Arc<Schema>>, register_types: RegisterTypes) -> Schema {
        Schema::with_register_types(version_key, parent, register_types)
    }
}

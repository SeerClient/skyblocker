//! Ported from net.azureaaron.legacyitemdfu.schemas.Schema11 (legacy-item-dfu 1.0.4+26.2).

use super::item_stack_only_schema::ItemStackOnlySchema;
use crate::net::azureaaron::legacyitemdfu::type_references;
use mcrust::com::mojang::datafixers::dsl;
use mcrust::com::mojang::datafixers::schemas::schema::Schema;
use std::sync::Arc;

/// java: public final class Schema11 extends ItemStackOnlySchema - an `ItemStack` with components.
pub struct Schema11;

impl Schema11 {
    /// java: public Schema11(int versionKey, Schema parent)
    pub fn new(version_key: i32, parent: Option<Arc<Schema>>) -> Schema {
        ItemStackOnlySchema::new(version_key, parent, Arc::new(Schema11::register_types))
    }

    /// java: public void registerTypes(Schema schema, Map<String, Supplier<TypeTemplate>> entityTypes, Map<String, Supplier<TypeTemplate>> blockEntityTypes)
    ///
    /// java: `ITEM_NAME` = `DSL.constType(IdentifierNormalizingSchema.getIdentifierType())`; `LEGACY_ITEM_STACK`
    /// (recursive) = `DSL.optionalFields("id", DSL.constType(IdentifierNormalizingSchema.getIdentifierType()),
    /// "components", DSL.remainder())` - registered as the shapes the game's DFU builds, see
    /// `item_stack_only_schema.rs`.
    pub fn register_types(schema: &mut Schema) {
        schema.register_type(false, &type_references::ITEM_NAME, Arc::new(|_schema: &Schema| dsl::const_type(dsl::remainder_type())));
        schema.register_type(false, &type_references::LEGACY_ITEM_STACK, Arc::new(|_schema: &Schema| dsl::remainder()));
    }
}

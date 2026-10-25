//! Ported from net.azureaaron.legacyitemdfu.fixers.HypixelComponentsRenameFix (legacy-item-dfu 1.0.4+26.2).

use crate::net::azureaaron::legacyitemdfu::type_references;
use mcrust::com::mojang::datafixers::data_fix::{self, DataFix, DataFixFields};
use mcrust::com::mojang::datafixers::schemas::schema::Schema;
use mcrust::com::mojang::datafixers::type_rewrite_rule::TypeRewriteRule;
use mcrust::com::mojang::serialization::dynamic::Dynamic;
use std::sync::Arc;

/// java: public class HypixelComponentsRenameFix extends DataFix
///
/// Hypixel has begun storing components directly in the item data but we don't want those to be "lost" so they are
/// renamed to something else and then merged back in through a later data fix.
pub struct HypixelComponentsRenameFix {
    fields: DataFixFields,
}

impl HypixelComponentsRenameFix {
    /// java: public HypixelComponentsRenameFix(Schema outputSchema, boolean changesType)
    pub fn new(output_schema: Arc<Schema>, changes_type: bool) -> HypixelComponentsRenameFix {
        HypixelComponentsRenameFix { fields: DataFixFields::new(output_schema, changes_type) }
    }
}

impl DataFix for HypixelComponentsRenameFix {
    fn data_fix(&self) -> &DataFixFields {
        &self.fields
    }

    /// java: protected TypeRewriteRule makeRule()
    fn make_rule(&self) -> TypeRewriteRule {
        let input_type = self.get_input_schema().get_type(&type_references::LEGACY_ITEM_STACK).unwrap_or_else(|e| panic!("java.lang.IllegalArgumentException: {e}"));
        let output_type = self.get_output_schema().get_type(&type_references::LEGACY_ITEM_STACK).unwrap_or_else(|e| panic!("java.lang.IllegalArgumentException: {e}"));
        data_fix::write_fix_and_read(
            "HypixelComponentsRenameFix",
            input_type,
            output_type,
            Arc::new(|item_stack_dynamic: Dynamic| item_stack_dynamic.rename_field("components", "hypixelComponents")),
        )
    }
}

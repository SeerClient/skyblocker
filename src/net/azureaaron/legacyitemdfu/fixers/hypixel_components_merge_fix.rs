//! Ported from net.azureaaron.legacyitemdfu.fixers.HypixelComponentsMergeFix (legacy-item-dfu 1.0.4+26.2).

use crate::net::azureaaron::legacyitemdfu::type_references;
use mcrust::com::mojang::datafixers::data_fix::{self, DataFix, DataFixFields};
use mcrust::com::mojang::datafixers::schemas::schema::Schema;
use mcrust::com::mojang::datafixers::type_rewrite_rule::TypeRewriteRule;
use mcrust::com::mojang::serialization::dynamic::Dynamic;
use mcrust::com::mojang::serialization::dynamic_like::DynamicLike as _;
use std::sync::Arc;

/// java: public class HypixelComponentsMergeFix extends DataFix
pub struct HypixelComponentsMergeFix {
    fields: DataFixFields,
}

impl HypixelComponentsMergeFix {
    /// java: public HypixelComponentsMergeFix(Schema outputSchema, boolean changesType)
    pub fn new(output_schema: Arc<Schema>, changes_type: bool) -> HypixelComponentsMergeFix {
        HypixelComponentsMergeFix { fields: DataFixFields::new(output_schema, changes_type) }
    }

    /// java: the `writeFixAndRead` lambda of `makeRule`
    fn fix(mut item_stack_dynamic: Dynamic) -> Dynamic {
        // java: Map<Dynamic<?>, Dynamic<?>> hypixelComponents = itemStackDynamic.get("hypixelComponents").asMap(Function.identity(), Function.identity());
        let hypixel_components: Vec<(Dynamic, Dynamic)> = item_stack_dynamic.get("hypixelComponents").as_map(|k| k, |v| v);

        // If there are no components to merge in then skip
        // This ensures that the components key is not added unless it is needed
        if hypixel_components.is_empty() {
            return item_stack_dynamic;
        }

        let mut vanilla_components = item_stack_dynamic.get("components").or_else_empty_map();

        for (key, value) in &hypixel_components {
            let key = key.as_string_or("");
            // Override the vanilla component with Hypixel's
            vanilla_components = vanilla_components.set(&key, value);
        }

        // Update the components & remove the temporary hypixelComponents field
        item_stack_dynamic = item_stack_dynamic.set("components", &vanilla_components);
        item_stack_dynamic = item_stack_dynamic.remove("hypixelComponents");

        item_stack_dynamic
    }
}

impl DataFix for HypixelComponentsMergeFix {
    fn data_fix(&self) -> &DataFixFields {
        &self.fields
    }

    /// java: protected TypeRewriteRule makeRule()
    fn make_rule(&self) -> TypeRewriteRule {
        let input_type = self.get_input_schema().get_type(&type_references::LEGACY_ITEM_STACK).unwrap_or_else(|e| panic!("java.lang.IllegalArgumentException: {e}"));
        let output_type = self.get_output_schema().get_type(&type_references::LEGACY_ITEM_STACK).unwrap_or_else(|e| panic!("java.lang.IllegalArgumentException: {e}"));
        data_fix::write_fix_and_read("HypixelComponentsMergeFix", input_type, output_type, Arc::new(HypixelComponentsMergeFix::fix))
    }
}

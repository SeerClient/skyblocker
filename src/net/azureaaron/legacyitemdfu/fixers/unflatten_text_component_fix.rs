//! Ported from net.azureaaron.legacyitemdfu.fixers.UnflattenTextComponentFix (legacy-item-dfu 1.0.4+26.2).
//!
//! `itemStackTyped.updateTyped(componentsFinder, this::fix)` is `stack.update("components", ..)` over the stack's
//! remainder (`schemas/item_stack_only_schema.rs`; the `components` field's type is `DSL.remainder()`, and an absent
//! field is left alone). `JsonParser.parseString` is `compat::legacyitemdfu_json_parse_string`.

use crate::compat;
use crate::net::azureaaron::legacyitemdfu::type_references;
use mcrust::com::mojang::datafixers::data_fix::{self, DataFix, DataFixFields};
use mcrust::com::mojang::datafixers::dsl;
use mcrust::com::mojang::datafixers::schemas::schema::Schema;
use mcrust::com::mojang::datafixers::type_rewrite_rule::TypeRewriteRule;
use mcrust::com::mojang::datafixers::typed::Typed;
use mcrust::com::mojang::serialization::dynamic::Dynamic;
use mcrust::com::mojang::serialization::dynamic_like::DynamicLike as _;
use mcrust::com::mojang::serialization::dynamic_ops::Ops;
use mcrust::com::mojang::serialization::json_ops::JsonOps;
use mcrust::lwjgl::json::Value;
use std::sync::Arc;

/// java: public class UnflattenTextComponentFix extends DataFix
///
/// Text components in 1.21.5 were moved to a structure where they were no longer flattened into a singular json
/// string, so we need to unflatten them.
pub struct UnflattenTextComponentFix {
    fields: DataFixFields,
}

impl UnflattenTextComponentFix {
    /// java: public UnflattenTextComponentFix(Schema outputSchema, boolean changesType)
    pub fn new(output_schema: Arc<Schema>, changes_type: bool) -> UnflattenTextComponentFix {
        UnflattenTextComponentFix { fields: DataFixFields::new(output_schema, changes_type) }
    }

    /// java: private Typed<?> fix(Typed<?> componentsTyped) - `componentsTyped.update(DSL.remainderFinder(), dynamic -> { .. })`
    fn fix(mut dynamic: Dynamic) -> Dynamic {
        let item_name = dynamic.get("minecraft:item_name");

        if item_name.result().is_some() {
            dynamic = dynamic.set("minecraft:item_name", &UnflattenTextComponentFix::unflatten_text_component(item_name.get_ops(), &item_name.as_string_or("")));
        }

        let custom_name = dynamic.get("minecraft:custom_name");

        if custom_name.result().is_some() {
            dynamic = dynamic.set("minecraft:custom_name", &UnflattenTextComponentFix::unflatten_text_component(custom_name.get_ops(), &custom_name.as_string_or("")));
        }

        let lore = dynamic.get("minecraft:lore");

        if let Some(lore) = lore.result() {
            dynamic = dynamic.set(
                "minecraft:lore",
                &dynamic.create_list(
                    lore.as_stream()
                        .into_iter()
                        .map(|line_dynamic| UnflattenTextComponentFix::unflatten_text_component(line_dynamic.get_ops(), &line_dynamic.as_string_or("")))
                        .collect(),
                ),
            );
        }

        dynamic
    }

    /// java: private static <T> Dynamic<T> unflattenTextComponent(DynamicOps<T> outOps, String jsonText)
    fn unflatten_text_component(out_ops: &Ops, json_text: &str) -> Dynamic {
        // java: try { JsonElement json = JsonParser.parseString(jsonText); .. } catch (Exception e) { .. }
        match compat::legacyitemdfu_json_parse_string(json_text) {
            Ok(json) => {
                if json != Value::Null {
                    // java: new Dynamic<>(outOps, JsonOps.INSTANCE.convertTo(outOps, json))
                    let converted = JsonOps::instance().convert_to(out_ops, &mcrust::com::mojang::serialization::obj(json));
                    return Dynamic::new_with(out_ops.clone(), Some(converted));
                }
            }
            Err(e) => {
                // TODO add better logging later - I don't know why this would get fed non-json text to begin with anyways
                // java: System.out.println(String.format(..)); e.printStackTrace(); - stdout, then the trace on stderr
                println!("Failed to unflatten text component json: {json_text}");
                eprintln!("com.google.gson.JsonSyntaxException: {e}");
            }
        }

        Dynamic::new_with(out_ops.clone(), Some(out_ops.create_string(json_text)))
    }
}

impl DataFix for UnflattenTextComponentFix {
    fn data_fix(&self) -> &DataFixFields {
        &self.fields
    }

    /// java: protected TypeRewriteRule makeRule()
    fn make_rule(&self) -> TypeRewriteRule {
        let r#type = self.get_input_schema().get_type(&type_references::LEGACY_ITEM_STACK).unwrap_or_else(|e| panic!("java.lang.IllegalArgumentException: {e}"));
        data_fix::fix_type_everywhere_typed_same(
            "Unflatten Text Component Fix",
            r#type,
            Arc::new(|item_stack_typed: Typed| item_stack_typed.update(&dsl::remainder_finder(), &|stack: Dynamic| stack.update("components", UnflattenTextComponentFix::fix))),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::UnflattenTextComponentFix;
    use mcrust::com::mojang::serialization::dynamic::Dynamic;
    use mcrust::com::mojang::serialization::dynamic_like::DynamicLike as _;
    use mcrust::com::mojang::serialization::json_ops::JsonOps;

    #[test]
    fn a_json_string_component_becomes_a_tree() {
        let components = Dynamic::new(JsonOps::instance()).empty_map();
        let components = components.set("minecraft:custom_name", &components.create_string(r#"{"text":"Hi"}"#));
        let fixed = UnflattenTextComponentFix::fix(components);
        assert_eq!(fixed.get("minecraft:custom_name").get("text").as_string().result().as_deref(), Some("Hi"));
    }
}

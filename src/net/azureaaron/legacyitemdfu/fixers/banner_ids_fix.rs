//! Ported from net.azureaaron.legacyitemdfu.fixers.BannerIdsFix (legacy-item-dfu 1.0.4+26.2).
//!
//! The library's finders are read off the stack's remainder (`schemas/item_stack_only_schema.rs`): `itemIdFinder` is
//! the `"id"` string, `nbtTagFinder` the optional `"tag"` field, `blockEntityTagFinder` the optional
//! `"tag".BlockEntityTag` field (whose type is `DSL.remainder()`, so `getOrCreate(DSL.remainderFinder())` is the whole
//! compound), and the stack's own remainder - the stack without `"id"` and `"tag"` - is the stack, whose `"Damage"` is
//! the only field of it this fix writes.

use crate::net::azureaaron::legacyitemdfu::type_references;
use mcrust::com::mojang::datafixers::data_fix::{self, DataFix, DataFixFields};
use mcrust::com::mojang::datafixers::dsl;
use mcrust::com::mojang::datafixers::schemas::schema::Schema;
use mcrust::com::mojang::datafixers::type_rewrite_rule::TypeRewriteRule;
use mcrust::com::mojang::datafixers::typed::Typed;
use mcrust::com::mojang::serialization::dynamic::Dynamic;
use mcrust::com::mojang::serialization::dynamic_like::DynamicLike as _;
use std::sync::Arc;

/// java: public class BannerIdsFix extends DataFix
pub struct BannerIdsFix {
    fields: DataFixFields,
}

impl BannerIdsFix {
    /// java: public BannerIdsFix(Schema outputSchema, boolean changesType)
    pub fn new(output_schema: Arc<Schema>, changes_type: bool) -> BannerIdsFix {
        BannerIdsFix { fields: DataFixFields::new(output_schema, changes_type) }
    }

    /// java: the `fixTypeEverywhereTyped` lambda of `makeRule`, over the stack's remainder
    fn fix(item_stack: Dynamic) -> Dynamic {
        // java: Optional<Pair<String, String>> itemId = itemStackTyped.getOptional(itemIdFinder);
        let item_id = item_stack.get("id").as_string().result();

        if item_id.as_deref() == Some("minecraft:banner") {
            // java: Dynamic<?> stack = itemStackTyped.get(DSL.remainderFinder());
            let mut stack = item_stack.clone();
            // java: Optional<? extends Typed<?>> nbtTagOpt = itemStackTyped.getOptionalTyped(nbtTagFinder);
            let nbt_tag_opt = item_stack.get("tag").result();

            if let Some(nbt_tag_typed) = nbt_tag_opt {
                // java: Optional<? extends Typed<?>> blockEntityTagOpt = nbtTagTyped.getOptionalTyped(blockEntityTagFinder);
                let block_entity_tag_opt = nbt_tag_typed.get("BlockEntityTag").result();

                if let Some(block_entity_tag_typed) = block_entity_tag_opt {
                    // java: Dynamic<?> blockEntityTagDynamic = blockEntityTagTyped.getOrCreate(DSL.remainderFinder());
                    let mut block_entity_tag_dynamic = block_entity_tag_typed;

                    // Set base colour as damage
                    if block_entity_tag_dynamic.get("Base").as_number().result().is_some() {
                        stack = stack.set("Damage", &stack.create_short((block_entity_tag_dynamic.get("Base").as_int(0) & 15) as i16));
                        block_entity_tag_dynamic = block_entity_tag_dynamic.remove("Base");
                    }

                    // Update pattern colour ids
                    block_entity_tag_dynamic = block_entity_tag_dynamic.update("Patterns", |patterns_dynamic: Dynamic| {
                        // java: DataFixUtils.orElse(patternsDynamic.asStreamOpt().map(stream -> stream.map(..)).map(patternsDynamic::createList).result(), patternsDynamic)
                        let fixed: Option<Dynamic> = patterns_dynamic
                            .as_stream_opt()
                            .map(|stream| {
                                stream
                                    .into_iter()
                                    .map(|pattern_dynamic| pattern_dynamic.update("Color", |colour_dynamic: Dynamic| colour_dynamic.create_int(15 - colour_dynamic.as_int(0))))
                                    .collect::<Vec<_>>()
                            })
                            .map(|list| patterns_dynamic.create_list(list))
                            .result();
                        fixed.unwrap_or(patterns_dynamic)
                    });

                    // java: return itemStackTyped.set(DSL.remainderFinder(), stack).set(nbtTagFinder, nbtTagTyped.set(blockEntityTagFinder, blockEntityTagTyped.set(DSL.remainderFinder(), blockEntityTagDynamic)));
                    return stack.set("tag", &nbt_tag_typed.set("BlockEntityTag", &block_entity_tag_dynamic));
                }
            }

            // java: return itemStackTyped.set(DSL.remainderFinder(), stack);
            return stack;
        }

        item_stack
    }
}

impl DataFix for BannerIdsFix {
    fn data_fix(&self) -> &DataFixFields {
        &self.fields
    }

    /// java: protected TypeRewriteRule makeRule()
    fn make_rule(&self) -> TypeRewriteRule {
        let r#type = self.get_input_schema().get_type(&type_references::LEGACY_ITEM_STACK).unwrap_or_else(|e| panic!("java.lang.IllegalArgumentException: {e}"));
        data_fix::fix_type_everywhere_typed_same(
            "BannerIdsFix",
            r#type,
            Arc::new(|item_stack_typed: Typed| item_stack_typed.update(&dsl::remainder_finder(), &BannerIdsFix::fix)),
        )
    }
}

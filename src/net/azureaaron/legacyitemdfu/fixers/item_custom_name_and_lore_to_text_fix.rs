//! Ported from net.azureaaron.legacyitemdfu.fixers.ItemCustomNameAndLoreToTextFix (legacy-item-dfu 1.0.4+26.2).
//!
//! `itemStackTyped.updateTyped(nbtTagFinder, nbtTagTyped -> nbtTagTyped.update(DSL.remainderFinder(), f))` is
//! `stack.update("tag", f)` over the stack's remainder (`schemas/item_stack_only_schema.rs`): an absent tag is left
//! alone, as the optional field's traversal leaves it, and the tag's own remainder (the tag without
//! `BlockEntityTag`) differs from the tag only in a field `f` never reads or writes.

use crate::net::azureaaron::legacyitemdfu::fixers::fixes::text_fixes::TextFixes;
use crate::net::azureaaron::legacyitemdfu::type_references;
use mcrust::com::mojang::datafixers::data_fix::{self, DataFix, DataFixFields};
use mcrust::com::mojang::datafixers::dsl;
use mcrust::com::mojang::datafixers::schemas::schema::Schema;
use mcrust::com::mojang::datafixers::type_rewrite_rule::{self, TypeRewriteRule};
use mcrust::com::mojang::datafixers::typed::Typed;
use mcrust::com::mojang::serialization::dynamic::Dynamic;
use mcrust::com::mojang::serialization::dynamic_like::DynamicLike as _;
use std::sync::Arc;

/// java: public class ItemCustomNameAndLoreToTextFix extends DataFix
pub struct ItemCustomNameAndLoreToTextFix {
    fields: DataFixFields,
}

impl ItemCustomNameAndLoreToTextFix {
    /// java: public ItemCustomNameAndLoreToTextFix(Schema outputSchema, boolean changesType)
    pub fn new(output_schema: Arc<Schema>, changes_type: bool) -> ItemCustomNameAndLoreToTextFix {
        ItemCustomNameAndLoreToTextFix { fields: DataFixFields::new(output_schema, changes_type) }
    }

    /// java: private static Dynamic<?> fixCustomName(Dynamic<?> tagDynamic)
    fn fix_custom_name(tag_dynamic: Dynamic) -> Dynamic {
        let display_dynamic_opt = tag_dynamic.get("display").result();

        if let Some(mut display_dynamic) = display_dynamic_opt {
            let custom_name_opt = display_dynamic.get("Name").as_string().result();

            if let Some(custom_name) = custom_name_opt {
                display_dynamic = display_dynamic.set("Name", &TextFixes::text(display_dynamic.get_ops(), &custom_name));
            }

            tag_dynamic.set("display", &display_dynamic)
        } else {
            tag_dynamic
        }
    }

    /// java: the lore half - `nbtTagDynamic.update("display", displayTagDynamic -> displayTagDynamic.update("Lore",
    /// loreDynamic -> DataFixUtils.orElse(loreDynamic.asStreamOpt().map(ItemCustomNameAndLoreToTextFix::fixLore)
    /// .map(loreDynamic::createList).result(), loreDynamic)))`
    fn fix_lore_tag(nbt_tag_dynamic: Dynamic) -> Dynamic {
        nbt_tag_dynamic.update("display", |display_tag_dynamic: Dynamic| {
            display_tag_dynamic.update("Lore", |lore_dynamic: Dynamic| {
                let fixed: Option<Dynamic> = lore_dynamic
                    .as_stream_opt()
                    .map(ItemCustomNameAndLoreToTextFix::fix_lore)
                    .map(|list| lore_dynamic.create_list(list))
                    .result();
                fixed.unwrap_or(lore_dynamic)
            })
        })
    }

    /// java: private static <T> Stream<Dynamic<T>> fixLore(Stream<Dynamic<T>> nbt)
    fn fix_lore(nbt: Vec<Dynamic>) -> Vec<Dynamic> {
        nbt.into_iter().map(TextFixes::fix_text).collect()
    }
}

impl DataFix for ItemCustomNameAndLoreToTextFix {
    fn data_fix(&self) -> &DataFixFields {
        &self.fields
    }

    /// java: protected TypeRewriteRule makeRule()
    fn make_rule(&self) -> TypeRewriteRule {
        let r#type = self.get_input_schema().get_type(&type_references::LEGACY_ITEM_STACK).unwrap_or_else(|e| panic!("java.lang.IllegalArgumentException: {e}"));

        let custom_name_fix = data_fix::fix_type_everywhere_typed_same(
            "ItemStack Custom Name to Text",
            r#type.clone(),
            Arc::new(|item_stack_typed: Typed| {
                item_stack_typed.update(&dsl::remainder_finder(), &|stack: Dynamic| stack.update("tag", ItemCustomNameAndLoreToTextFix::fix_custom_name))
            }),
        );

        let lore_fix = data_fix::fix_type_everywhere_typed_same(
            "ItemStack Lore to Text",
            r#type,
            Arc::new(|item_stack_typed: Typed| {
                item_stack_typed.update(&dsl::remainder_finder(), &|stack: Dynamic| stack.update("tag", ItemCustomNameAndLoreToTextFix::fix_lore_tag))
            }),
        );

        // java: TypeRewriteRule.seq(customNameFix, loreFix)
        type_rewrite_rule::seq2(custom_name_fix, lore_fix)
    }
}

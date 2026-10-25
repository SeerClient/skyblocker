//! Ported from net.azureaaron.legacyitemdfu.fixers.ItemStackUuidsFix (legacy-item-dfu 1.0.4+26.2).
//!
//! Over the stack's remainder (`schemas/item_stack_only_schema.rs`): `itemIdFinder` is the `"id"` string, and
//! `updateTyped(nbtTagFinder, t -> t.update(DSL.remainderFinder(), f))` is `stack.update("tag", f)` (an absent tag is
//! left alone; the tag's own remainder differs from it only in `BlockEntityTag`, which `f` never touches).

use crate::net::azureaaron::legacyitemdfu::fixers::fixes::uuid_fixes::UuidFixes;
use crate::net::azureaaron::legacyitemdfu::type_references;
use mcrust::com::mojang::datafixers::data_fix::{self, DataFix, DataFixFields};
use mcrust::com::mojang::datafixers::dsl;
use mcrust::com::mojang::datafixers::schemas::schema::Schema;
use mcrust::com::mojang::datafixers::type_rewrite_rule::TypeRewriteRule;
use mcrust::com::mojang::datafixers::typed::Typed;
use mcrust::com::mojang::serialization::dynamic::Dynamic;
use mcrust::com::mojang::serialization::dynamic_like::DynamicLike as _;
use std::sync::Arc;

/// java: public class ItemStackUuidsFix extends DataFix
pub struct ItemStackUuidsFix {
    fields: DataFixFields,
}

impl ItemStackUuidsFix {
    /// java: public ItemStackUuidsFix(Schema outputSchema, boolean changesType)
    pub fn new(output_schema: Arc<Schema>, changes_type: bool) -> ItemStackUuidsFix {
        ItemStackUuidsFix { fields: DataFixFields::new(output_schema, changes_type) }
    }

    /// java: the `fixTypeEverywhereTyped` lambda of `makeRule`, over the stack's remainder
    fn fix(item_stack: Dynamic) -> Dynamic {
        // java: Optional<Pair<String, String>> itemId = itemStackTyped.getOptional(itemIdFinder); (read inside the tag lambda)
        let item_id = item_stack.get("id").as_string().result();
        item_stack.update("tag", |mut nbt_tag_dynamic: Dynamic| {
            nbt_tag_dynamic = ItemStackUuidsFix::fix_attribute_modifiers(&nbt_tag_dynamic);

            if item_id.as_deref() == Some("minecraft:player_head") {
                nbt_tag_dynamic = ItemStackUuidsFix::fix_player_head_owner(&nbt_tag_dynamic);
            }

            nbt_tag_dynamic
        })
    }

    /// java: private static Dynamic<?> fixAttributeModifiers(Dynamic<?> tagDynamic)
    fn fix_attribute_modifiers(tag_dynamic: &Dynamic) -> Dynamic {
        tag_dynamic.update("AttributeModifiers", |attribute_modifiers_dynamic: Dynamic| {
            tag_dynamic.create_list(
                attribute_modifiers_dynamic
                    .as_stream()
                    .into_iter()
                    .map(|attribute_modifier| UuidFixes::update_regular_most_least(&attribute_modifier, "UUID", "UUID").unwrap_or(attribute_modifier))
                    .collect(),
            )
        })
    }

    /// java: private Dynamic<?> fixPlayerHeadOwner(Dynamic<?> tagDynamic)
    fn fix_player_head_owner(tag_dynamic: &Dynamic) -> Dynamic {
        tag_dynamic.update("SkullOwner", |skull_owner: Dynamic| UuidFixes::update_string_uuid(&skull_owner, "Id", "Id").unwrap_or(skull_owner))
    }
}

impl DataFix for ItemStackUuidsFix {
    fn data_fix(&self) -> &DataFixFields {
        &self.fields
    }

    /// java: protected TypeRewriteRule makeRule()
    fn make_rule(&self) -> TypeRewriteRule {
        let r#type = self.get_input_schema().get_type(&type_references::LEGACY_ITEM_STACK).unwrap_or_else(|e| panic!("java.lang.IllegalArgumentException: {e}"));
        data_fix::fix_type_everywhere_typed_same(
            "ItemStack UUIDs Fix",
            r#type,
            Arc::new(|item_stack_typed: Typed| item_stack_typed.update(&dsl::remainder_finder(), &ItemStackUuidsFix::fix)),
        )
    }
}

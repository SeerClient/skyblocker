//! Ported from net.azureaaron.legacyitemdfu.fixers.PostFlatteningItemIdsFix (legacy-item-dfu 1.0.4+26.2).
//!
//! `itemIdFinder` (`DSL.fieldFinder("id", DSL.named(ITEM_NAME, getIdentifierType()))`) is the `"id"` string of the
//! stack's remainder (`schemas/item_stack_only_schema.rs`).

use crate::net::azureaaron::legacyitemdfu::type_references;
use mcrust::com::mojang::datafixers::data_fix::{self, DataFix, DataFixFields};
use mcrust::com::mojang::datafixers::dsl;
use mcrust::com::mojang::datafixers::schemas::schema::Schema;
use mcrust::com::mojang::datafixers::type_rewrite_rule::TypeRewriteRule;
use mcrust::com::mojang::datafixers::typed::Typed;
use mcrust::com::mojang::serialization::dynamic::Dynamic;
use mcrust::com::mojang::serialization::dynamic_like::DynamicLike as _;
use std::collections::HashMap;
use std::sync::{Arc, LazyLock};

/// java: private static final Object2ObjectMap<String, String> NEW_IDS = DataFixUtils.make(new Object2ObjectOpenHashMap<>(), map -> { .. });
static NEW_IDS: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
    let mut map = HashMap::new();
    // 1.13
    map.insert("minecraft:melon_block", "minecraft:melon");
    map.insert("minecraft:melon", "minecraft:melon_slice");
    map.insert("minecraft:speckled_melon", "minecraft:glistering_melon_slice");
    map.insert("minecraft:prismarine_bricks_slab", "minecraft:prismarine_brick_slab");
    map.insert("minecraft:prismarine_bricks_stairs", "minecraft:prismarine_brick_stairs");

    // 1.14
    map.insert("minecraft:cactus_green", "minecraft:green_dye");
    map.insert("minecraft:rose_red", "minecraft:red_dye");
    map.insert("minecraft:dandelion_yellow", "minecraft:yellow_dye");
    map.insert("minecraft:clownfish", "minecraft:tropical_fish");
    map.insert("minecraft:stone_slab", "minecraft:smooth_stone_slab");
    map.insert("minecraft:sign", "minecraft:oak_sign");

    // 1.20.5
    map.insert("minecraft:grass", "minecraft:short_grass");
    map.insert("minecraft:scute", "minecraft:turtle_scute");
    map
});

/// java: public class PostFlatteningItemIdsFix extends DataFix - fixes item ids that changed after the flattening
pub struct PostFlatteningItemIdsFix {
    fields: DataFixFields,
}

impl PostFlatteningItemIdsFix {
    /// java: public PostFlatteningItemIdsFix(Schema outputSchema, boolean changesType)
    pub fn new(output_schema: Arc<Schema>, changes_type: bool) -> PostFlatteningItemIdsFix {
        PostFlatteningItemIdsFix { fields: DataFixFields::new(output_schema, changes_type) }
    }

    /// java: the `fixTypeEverywhereTyped` lambda of `makeRule`, over the stack's remainder
    fn fix(item_stack: Dynamic) -> Dynamic {
        // java: Optional<Pair<String, String>> itemId = itemStackTyped.getOptional(itemIdFinder);
        let item_id = item_stack.get("id").as_string().result();

        if let Some(new_id) = item_id.as_deref().and_then(|id| NEW_IDS.get(id)) {
            // java: itemStackTyped = itemStackTyped.set(itemIdFinder, Pair.of(TypeReferences.ITEM_NAME.typeName(), newId));
            return item_stack.set("id", &item_stack.create_string(new_id));
        }

        item_stack
    }
}

impl DataFix for PostFlatteningItemIdsFix {
    fn data_fix(&self) -> &DataFixFields {
        &self.fields
    }

    /// java: protected TypeRewriteRule makeRule()
    fn make_rule(&self) -> TypeRewriteRule {
        let r#type = self.get_input_schema().get_type(&type_references::LEGACY_ITEM_STACK).unwrap_or_else(|e| panic!("java.lang.IllegalArgumentException: {e}"));
        data_fix::fix_type_everywhere_typed_same(
            "PostFlatteningItemIdsFix",
            r#type,
            Arc::new(|item_stack_typed: Typed| item_stack_typed.update(&dsl::remainder_finder(), &PostFlatteningItemIdsFix::fix)),
        )
    }
}

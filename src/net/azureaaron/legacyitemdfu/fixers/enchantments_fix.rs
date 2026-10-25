//! Ported from net.azureaaron.legacyitemdfu.fixers.EnchantmentsFix (legacy-item-dfu 1.0.4+26.2).
//!
//! `itemStackTyped.updateTyped(nbtTagFinder, nbtTagTyped -> nbtTagTyped.update(DSL.remainderFinder(), f))` is
//! `stack.update("tag", f)` over the stack's remainder (`schemas/item_stack_only_schema.rs`; the tag's own remainder
//! differs from the tag only in `BlockEntityTag`, which `f` never touches).

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

/// java: private static final Int2ObjectMap<String> ID_TO_ENCHANTMENTS_MAP = DataFixUtils.make(new Int2ObjectOpenHashMap<>(), map -> { .. });
static ID_TO_ENCHANTMENTS_MAP: LazyLock<HashMap<i32, &'static str>> = LazyLock::new(|| {
    let mut map = HashMap::new();
    map.insert(0, "minecraft:protection");
    map.insert(1, "minecraft:fire_protection");
    map.insert(2, "minecraft:feather_falling");
    map.insert(3, "minecraft:blast_protection");
    map.insert(4, "minecraft:projectile_protection");
    map.insert(5, "minecraft:respiration");
    map.insert(6, "minecraft:aqua_affinity");
    map.insert(7, "minecraft:thorns");
    map.insert(8, "minecraft:depth_strider");
    map.insert(9, "minecraft:frost_walker");
    map.insert(10, "minecraft:binding_curse");
    map.insert(16, "minecraft:sharpness");
    map.insert(17, "minecraft:smite");
    map.insert(18, "minecraft:bane_of_arthropods");
    map.insert(19, "minecraft:knockback");
    map.insert(20, "minecraft:fire_aspect");
    map.insert(21, "minecraft:looting");
    map.insert(22, "minecraft:sweeping");
    map.insert(32, "minecraft:efficiency");
    map.insert(33, "minecraft:silk_touch");
    map.insert(34, "minecraft:unbreaking");
    map.insert(35, "minecraft:fortune");
    map.insert(48, "minecraft:power");
    map.insert(49, "minecraft:punch");
    map.insert(50, "minecraft:flame");
    map.insert(51, "minecraft:infinity");
    map.insert(61, "minecraft:luck_of_the_sea");
    map.insert(62, "minecraft:lure");
    map.insert(65, "minecraft:loyalty");
    map.insert(66, "minecraft:impaling");
    map.insert(67, "minecraft:riptide");
    map.insert(68, "minecraft:channeling");
    map.insert(70, "minecraft:mending");
    map.insert(71, "minecraft:vanishing_curse");
    map
});

/// java: public class EnchantmentsFix extends DataFix
pub struct EnchantmentsFix {
    fields: DataFixFields,
}

impl EnchantmentsFix {
    /// java: public EnchantmentsFix(Schema outputSchema, boolean changesType)
    pub fn new(output_schema: Arc<Schema>, changes_type: bool) -> EnchantmentsFix {
        EnchantmentsFix { fields: DataFixFields::new(output_schema, changes_type) }
    }

    /// java: `enchantment.set("id", enchantment.createString(ID_TO_ENCHANTMENTS_MAP.getOrDefault(enchantment.get("id").asInt(0), "null")))`
    fn fix_enchantment_id(enchantment: Dynamic) -> Dynamic {
        let id = ID_TO_ENCHANTMENTS_MAP.get(&enchantment.get("id").as_int(0)).copied().unwrap_or("null");
        enchantment.set("id", &enchantment.create_string(id))
    }

    /// java: private static Dynamic<?> fixEnchantments(Dynamic<?> tagDynamic)
    fn fix_enchantments(mut tag_dynamic: Dynamic) -> Dynamic {
        let enchantments_opt: Option<Dynamic> = tag_dynamic
            .get("ench")
            .as_stream_opt()
            .map(|enchantments| enchantments.into_iter().map(EnchantmentsFix::fix_enchantment_id).collect::<Vec<_>>())
            .map(|list| tag_dynamic.create_list(list))
            .result();

        if let Some(enchantments) = enchantments_opt {
            tag_dynamic = tag_dynamic.remove("ench").set("Enchantments", &enchantments);
        }

        tag_dynamic.update("StoredEnchantments", |stored_enchantments_dynamic: Dynamic| {
            let fixed: Option<Dynamic> = stored_enchantments_dynamic
                .as_stream_opt()
                .map(|stored_enchantments| stored_enchantments.into_iter().map(EnchantmentsFix::fix_enchantment_id).collect::<Vec<_>>())
                .map(|list| stored_enchantments_dynamic.create_list(list))
                .result();
            fixed.unwrap_or(stored_enchantments_dynamic)
        })
    }
}

impl DataFix for EnchantmentsFix {
    fn data_fix(&self) -> &DataFixFields {
        &self.fields
    }

    /// java: protected TypeRewriteRule makeRule()
    fn make_rule(&self) -> TypeRewriteRule {
        let r#type = self.get_input_schema().get_type(&type_references::LEGACY_ITEM_STACK).unwrap_or_else(|e| panic!("java.lang.IllegalArgumentException: {e}"));
        data_fix::fix_type_everywhere_typed_same(
            "Enchantments Fix",
            r#type,
            Arc::new(|item_stack_typed: Typed| item_stack_typed.update(&dsl::remainder_finder(), &|stack: Dynamic| stack.update("tag", EnchantmentsFix::fix_enchantments))),
        )
    }
}

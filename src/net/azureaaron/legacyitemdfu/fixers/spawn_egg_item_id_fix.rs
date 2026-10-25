//! Ported from net.azureaaron.legacyitemdfu.fixers.SpawnEggItemIdFix (legacy-item-dfu 1.0.4+26.2).
//!
//! The library's `itemIdFinder` (`DSL.fieldFinder("id", DSL.named(ITEM_NAME, getIdentifierType()))`) and
//! `DSL.remainderFinder()` are both read off the stack's remainder here (`schemas/item_stack_only_schema.rs`): the id
//! is the `"id"` string, and the remainder - the stack without `"id"` and `"tag"` - is the stack itself, whose
//! `"Damage"` is the only field this fix touches.

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

/// java: private static final Short2ObjectMap<String> DAMAGE_2_ID = DataFixUtils.make(new Short2ObjectOpenHashMap<>(), map -> { ..; map.defaultReturnValue("minecraft:air"); });
static DAMAGE_2_ID: LazyLock<HashMap<i16, &'static str>> = LazyLock::new(|| {
    let mut map = HashMap::new();
    // Polar Bears shouldn't be damage 0 but Hypixel uses this for decoy spawn eggs and others
    map.insert(0, "minecraft:polar_bear_spawn_egg");
    map.insert(4, "minecraft:elder_guardian_spawn_egg");
    map.insert(5, "minecraft:wither_skeleton_spawn_egg");
    map.insert(6, "minecraft:stray_spawn_egg");
    map.insert(23, "minecraft:husk_spawn_egg");
    map.insert(27, "minecraft:zombie_villager_spawn_egg");
    map.insert(28, "minecraft:skeleton_horse_spawn_egg");
    map.insert(29, "minecraft:zombie_horse_spawn_egg");
    map.insert(50, "minecraft:creeper_spawn_egg");
    map.insert(51, "minecraft:skeleton_spawn_egg");
    map.insert(52, "minecraft:spider_spawn_egg");
    map.insert(54, "minecraft:zombie_spawn_egg");
    map.insert(55, "minecraft:slime_spawn_egg");
    map.insert(56, "minecraft:ghast_spawn_egg");
    map.insert(57, "minecraft:zombified_piglin_spawn_egg");
    map.insert(58, "minecraft:enderman_spawn_egg");
    map.insert(59, "minecraft:cave_spider_spawn_egg");
    map.insert(60, "minecraft:silverfish_spawn_egg");
    map.insert(61, "minecraft:blaze_spawn_egg");
    map.insert(62, "minecraft:magma_cube_spawn_egg");
    map.insert(65, "minecraft:bat_spawn_egg");
    map.insert(66, "minecraft:witch_spawn_egg");
    map.insert(67, "minecraft:endermite_spawn_egg");
    map.insert(68, "minecraft:guardian_spawn_egg");
    map.insert(69, "minecraft:shulker_spawn_egg");
    map.insert(90, "minecraft:pig_spawn_egg");
    map.insert(91, "minecraft:sheep_spawn_egg");
    map.insert(92, "minecraft:cow_spawn_egg");
    map.insert(93, "minecraft:chicken_spawn_egg");
    map.insert(94, "minecraft:squid_spawn_egg");
    map.insert(95, "minecraft:wolf_spawn_egg");
    map.insert(96, "minecraft:mooshroom_spawn_egg");
    map.insert(98, "minecraft:ocelot_spawn_egg");
    map.insert(100, "minecraft:horse_spawn_egg");
    map.insert(101, "minecraft:rabbit_spawn_egg");
    map.insert(120, "minecraft:villager_spawn_egg");
    map
});

/// java: public class SpawnEggItemIdFix extends DataFix - converts spawn eggs to their modern id
pub struct SpawnEggItemIdFix {
    fields: DataFixFields,
}

impl SpawnEggItemIdFix {
    /// java: public SpawnEggItemIdFix(Schema outputSchema, boolean changesType)
    pub fn new(output_schema: Arc<Schema>, changes_type: bool) -> SpawnEggItemIdFix {
        SpawnEggItemIdFix { fields: DataFixFields::new(output_schema, changes_type) }
    }

    /// java: the `fixTypeEverywhereTyped` lambda of `makeRule`, over the stack's remainder
    fn fix(item_stack: Dynamic) -> Dynamic {
        // java: Optional<Pair<String, String>> itemId = itemStackTyped.getOptional(itemIdFinder);
        let item_id = item_stack.get("id").as_string().result();

        if item_id.as_deref() == Some("minecraft:spawn_egg") {
            // java: Dynamic<?> stack = itemStackTyped.get(DSL.remainderFinder());
            let stack = item_stack;
            let damage = stack.get("Damage").as_short(0);
            // java: String newId = DAMAGE_2_ID.get(damage); - the map's default return value is "minecraft:air"
            let new_id = DAMAGE_2_ID.get(&damage).copied().unwrap_or("minecraft:air");

            // java: itemStackTyped = itemStackTyped.set(itemIdFinder, Pair.of(TypeReferences.ITEM_NAME.typeName(), newId));
            let stack = stack.set("id", &stack.create_string(new_id));

            // java: return itemStackTyped.set(DSL.remainderFinder(), stack.remove("Damage"));
            return stack.remove("Damage");
        }

        item_stack
    }
}

impl DataFix for SpawnEggItemIdFix {
    fn data_fix(&self) -> &DataFixFields {
        &self.fields
    }

    /// java: protected TypeRewriteRule makeRule()
    fn make_rule(&self) -> TypeRewriteRule {
        let r#type = self.get_input_schema().get_type(&type_references::LEGACY_ITEM_STACK).unwrap_or_else(|e| panic!("java.lang.IllegalArgumentException: {e}"));
        data_fix::fix_type_everywhere_typed_same(
            "SpawnEggItemIdFix",
            r#type,
            Arc::new(|item_stack_typed: Typed| item_stack_typed.update(&dsl::remainder_finder(), &SpawnEggItemIdFix::fix)),
        )
    }
}

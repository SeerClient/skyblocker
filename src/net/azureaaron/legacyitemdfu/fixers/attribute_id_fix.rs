//! Ported from net.azureaaron.legacyitemdfu.fixers.AttributeIdFix (legacy-item-dfu 1.0.4+26.2).
//!
//! `itemStackTyped.updateTyped(componentsFinder, t -> t.update(DSL.remainderFinder(), fixItemStack))` is
//! `stack.update("components", fixItemStack)` over the stack's remainder (`schemas/item_stack_only_schema.rs`; the
//! `components` field's type is `DSL.remainder()`, and an absent field is left alone).

use crate::net::azureaaron::legacyitemdfu::type_references;
use mcrust::com::mojang::datafixers::data_fix::{self, DataFix, DataFixFields};
use mcrust::com::mojang::datafixers::dsl;
use mcrust::com::mojang::datafixers::schemas::schema::Schema;
use mcrust::com::mojang::datafixers::type_rewrite_rule::TypeRewriteRule;
use mcrust::com::mojang::datafixers::typed::Typed;
use mcrust::com::mojang::serialization::dynamic::Dynamic;
use mcrust::com::mojang::serialization::dynamic_like::DynamicLike as _;
use mcrust::java::util::uuid::Uuid;
use std::collections::HashMap;
use std::sync::{Arc, LazyLock};

/// java: private static final Map<UUID, String> UUID_TO_ID = ImmutableMap.<UUID, String>builder().put(UUID.fromString(..), ..)...build();
static UUID_TO_ID: LazyLock<HashMap<Uuid, &'static str>> = LazyLock::new(|| {
    [
        ("736565d2-e1a7-403d-a3f8-1aeb3e302542", "minecraft:creative_mode_block_range"),
        ("98491ef6-97b1-4584-ae82-71a8cc85cf73", "minecraft:creative_mode_entity_range"),
        ("91AEAA56-376B-4498-935B-2F7F68070635", "minecraft:effect.speed"),
        ("7107DE5E-7CE8-4030-940E-514C1F160890", "minecraft:effect.slowness"),
        ("AF8B6E3F-3328-4C0A-AA36-5BA2BB9DBEF3", "minecraft:effect.haste"),
        ("55FCED67-E92A-486E-9800-B47F202C4386", "minecraft:effect.mining_fatigue"),
        ("648D7064-6A60-4F59-8ABE-C2C23A6DD7A9", "minecraft:effect.strength"),
        ("C0105BF3-AEF8-46B0-9EBC-92943757CCBE", "minecraft:effect.jump_boost"),
        ("22653B89-116E-49DC-9B6B-9971489B5BE5", "minecraft:effect.weakness"),
        ("5D6F0BA2-1186-46AC-B896-C61C5CEE99CC", "minecraft:effect.health_boost"),
        ("EAE29CF0-701E-4ED6-883A-96F798F3DAB5", "minecraft:effect.absorption"),
        ("03C3C89D-7037-4B42-869F-B146BCB64D2E", "minecraft:effect.luck"),
        ("CC5AF142-2BD2-4215-B636-2605AED11727", "minecraft:effect.unluck"),
        ("6555be74-63b3-41f1-a245-77833b3c2562", "minecraft:evil"),
        ("1eaf83ff-7207-4596-b37a-d7a07b3ec4ce", "minecraft:powder_snow"),
        ("662A6B8D-DA3E-4C1C-8813-96EA6097278D", "minecraft:sprinting"),
        ("020E0DFB-87AE-4653-9556-831010E291A0", "minecraft:attacking"),
        ("766bfa64-11f3-11ea-8d71-362b9e155667", "minecraft:baby"),
        ("7E0292F2-9434-48D5-A29F-9583AF7DF27F", "minecraft:covered"),
        ("9e362924-01de-4ddd-a2b2-d0f7a405a174", "minecraft:suffocating"),
        ("5CD17E52-A79A-43D3-A529-90FDE04B181E", "minecraft:drinking"),
        ("B9766B59-9566-4402-BC1F-2EE2A276D836", "minecraft:baby"),
        ("49455A49-7EC5-45BA-B886-3B90B23A1718", "minecraft:attacking"),
        ("845DB27C-C624-495F-8C9F-6020A9A58B6B", "minecraft:armor.boots"),
        ("D8499B04-0E66-4726-AB29-64469D734E0D", "minecraft:armor.leggings"),
        ("9F3D476D-C118-4544-8365-64846904B48E", "minecraft:armor.chestplate"),
        ("2AD3F246-FEE1-4E67-B886-69FD380BB150", "minecraft:armor.helmet"),
        ("C1C72771-8B8E-BA4A-ACE0-81A93C8928B2", "minecraft:armor.body"),
        ("b572ecd2-ac0c-4071-abde-9594af072a37", "minecraft:enchantment.fire_protection"),
        ("40a9968f-5c66-4e2f-b7f4-2ec2f4b3e450", "minecraft:enchantment.blast_protection"),
        ("07a65791-f64d-4e79-86c7-f83932f007ec", "minecraft:enchantment.respiration"),
        ("60b1b7db-fffd-4ad0-817c-d6c6a93d8a45", "minecraft:enchantment.aqua_affinity"),
        ("11dc269a-4476-46c0-aff3-9e17d7eb6801", "minecraft:enchantment.depth_strider"),
        ("87f46a96-686f-4796-b035-22e16ee9e038", "minecraft:enchantment.soul_speed"),
        ("b9716dbd-50df-4080-850e-70347d24e687", "minecraft:enchantment.soul_speed"),
        ("92437d00-c3a7-4f2e-8f6c-1f21585d5dd0", "minecraft:enchantment.swift_sneak"),
        ("5d3d087b-debe-4037-b53e-d84f3ff51f17", "minecraft:enchantment.sweeping_edge"),
        ("3ceb37c0-db62-46b5-bd02-785457b01d96", "minecraft:enchantment.efficiency"),
        ("CB3F55D3-645C-4F38-A497-9C13A33DB5CF", "minecraft:base_attack_damage"),
        ("FA233E1C-4180-4865-B01B-BCCE9785ACA3", "minecraft:base_attack_speed"),
    ]
    .into_iter()
    .map(|(uuid, id)| (Uuid::from_string(uuid).unwrap_or_else(|| panic!("java.lang.IllegalArgumentException: Invalid UUID string: {uuid}")), id))
    .collect()
});

/// java: private static final Map<String, String> NAME_TO_ID = Map.of(..);
static NAME_TO_ID: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
    [
        ("Random spawn bonus", "minecraft:random_spawn_bonus"),
        ("Random zombie-spawn bonus", "minecraft:zombie_random_spawn_bonus"),
        ("Leader zombie bonus", "minecraft:leader_zombie_bonus"),
        ("Zombie reinforcement callee charge", "minecraft:reinforcement_callee_charge"),
        ("Zombie reinforcement caller charge", "minecraft:reinforcement_caller_charge"),
    ]
    .into_iter()
    .collect()
});

/// java: public class AttributeIdFix extends DataFix
pub struct AttributeIdFix {
    fields: DataFixFields,
}

impl AttributeIdFix {
    /// java: public AttributeIdFix(Schema outputSchema, boolean changesType)
    pub fn new(output_schema: Arc<Schema>, changes_type: bool) -> AttributeIdFix {
        AttributeIdFix { fields: DataFixFields::new(output_schema, changes_type) }
    }

    /// java: private static Dynamic<?> fixItemStack(Dynamic<?> componentsDynamic)
    fn fix_item_stack(components_dynamic: Dynamic) -> Dynamic {
        components_dynamic.update("minecraft:attribute_modifiers", |attribute_modifiers_dynamic: Dynamic| {
            attribute_modifiers_dynamic.update("modifiers", |modifiers_dynamic: Dynamic| {
                // java: DataFixUtils.orElse(modifiersDynamic.asStreamOpt().result().map(AttributeIdFix::fixModifiersCasted).map(modifiersDynamic::createList), modifiersDynamic)
                let fixed: Option<Dynamic> = modifiers_dynamic
                    .as_stream_opt()
                    .result()
                    .map(AttributeIdFix::fix_modifiers_casted)
                    .map(|list| modifiers_dynamic.create_list(list));
                fixed.unwrap_or(modifiers_dynamic)
            })
        })
    }

    /// java: private static Stream<Dynamic<?>> fixModifiersCasted(Stream<?> stream) - the unchecked cast is the identity.
    fn fix_modifiers_casted(stream: Vec<Dynamic>) -> Vec<Dynamic> {
        AttributeIdFix::fix_modifiers(stream)
    }

    /// java: private static Stream<Dynamic<?>> fixModifiers(Stream<Dynamic<?>> dynamicStream)
    fn fix_modifiers(dynamic_stream: Vec<Dynamic>) -> Vec<Dynamic> {
        // java: Map<String, Dynamic<?>> map = new Object2ObjectArrayMap<>(); //Map of attribute ids to modifier dynamics
        // - an array map: insertion order, and a `put` on a present key replaces the value in place.
        let mut map: Vec<(String, Dynamic)> = Vec::new();
        fn put(map: &mut Vec<(String, Dynamic)>, key: String, value: Dynamic) {
            match map.iter_mut().find(|(k, _)| *k == key) {
                Some(entry) => entry.1 = value,
                None => map.push((key, value)),
            }
        }

        for mut modifier_dynamic in dynamic_stream {
            let uuid = AttributeIdFix::get_uuid_from_int_array(&modifier_dynamic.get("uuid").as_int_stream());
            let name = modifier_dynamic.get("name").as_string_or("");
            let id_from_uuid = uuid.and_then(|uuid| UUID_TO_ID.get(&uuid).copied());
            let id_from_name = NAME_TO_ID.get(name.as_str()).copied();

            if let Some(id_from_uuid) = id_from_uuid {
                modifier_dynamic = modifier_dynamic.set("id", &modifier_dynamic.create_string(id_from_uuid));
                put(&mut map, id_from_uuid.to_string(), modifier_dynamic.remove("uuid").remove("name"));
            } else if let Some(id_from_name) = id_from_name {
                let entry = map.iter().find(|(k, _)| k == id_from_name).map(|(_, v)| v.clone());

                match entry {
                    None => {
                        modifier_dynamic = modifier_dynamic.set("id", &modifier_dynamic.create_string(id_from_name));
                        put(&mut map, id_from_name.to_string(), modifier_dynamic.remove("uuid").remove("name"));
                    }
                    Some(entry) => {
                        let d = entry.get("amount").as_double(0.0);
                        let e = modifier_dynamic.get("amount").as_double(0.0);
                        put(&mut map, id_from_name.to_string(), entry.set("amount", &modifier_dynamic.create_double(d + e)));
                    }
                }
            } else {
                // java: "minecraft:" + (uuid != null ? uuid.toString().toLowerCase(Locale.ROOT) : "unknown")
                let new_id = format!("minecraft:{}", match uuid {
                    Some(uuid) => uuid.to_string().to_lowercase(),
                    None => "unknown".to_string(),
                });
                modifier_dynamic = modifier_dynamic.set("id", &modifier_dynamic.create_string(&new_id));

                put(&mut map, new_id, modifier_dynamic.remove("uuid").remove("name"));
            }
        }

        // java: return map.values().stream();
        map.into_iter().map(|(_, value)| value).collect()
    }

    /// java: private static @Nullable UUID getUuidFromIntArray(int[] uuidArray)
    fn get_uuid_from_int_array(uuid_array: &[i32]) -> Option<Uuid> {
        if uuid_array.len() != 4 {
            None
        } else {
            Some(Uuid::new(
                (uuid_array[0] as i64) << 32 | (uuid_array[1] as i64) & 4294967295,
                (uuid_array[2] as i64) << 32 | (uuid_array[3] as i64) & 4294967295,
            ))
        }
    }
}

impl DataFix for AttributeIdFix {
    fn data_fix(&self) -> &DataFixFields {
        &self.fields
    }

    /// java: protected TypeRewriteRule makeRule()
    fn make_rule(&self) -> TypeRewriteRule {
        let r#type = self.get_input_schema().get_type(&type_references::LEGACY_ITEM_STACK).unwrap_or_else(|e| panic!("java.lang.IllegalArgumentException: {e}"));
        data_fix::fix_type_everywhere_typed_same(
            "Attribute Id Fix (ItemStack)",
            r#type,
            Arc::new(|item_stack_typed: Typed| item_stack_typed.update(&dsl::remainder_finder(), &|stack: Dynamic| stack.update("components", AttributeIdFix::fix_item_stack))),
        )
    }
}

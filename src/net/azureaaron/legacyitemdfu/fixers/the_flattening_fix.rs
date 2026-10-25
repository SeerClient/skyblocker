//! Ported from net.azureaaron.legacyitemdfu.fixers.TheFlatteningFix (legacy-item-dfu 1.0.4+26.2).
//!
//! The library's finders are read off the stack's remainder (`schemas/item_stack_only_schema.rs`): `itemIdFinder` is
//! the `"id"` string, `nbtTagFinder` the optional `"tag"` field (`getOrCreateTyped` = the tag, or an empty compound),
//! and the stack's own remainder - the stack without `"id"` and `"tag"` - is the stack, whose `"Damage"` is the only
//! field of it this fix writes.

use crate::net::azureaaron::legacyitemdfu::type_references;
use mcrust::com::mojang::datafixers::data_fix::{self, DataFix, DataFixFields};
use mcrust::com::mojang::datafixers::dsl;
use mcrust::com::mojang::datafixers::schemas::schema::Schema;
use mcrust::com::mojang::datafixers::type_rewrite_rule::TypeRewriteRule;
use mcrust::com::mojang::datafixers::typed::Typed;
use mcrust::com::mojang::serialization::dynamic::Dynamic;
use mcrust::com::mojang::serialization::dynamic_like::DynamicLike as _;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, LazyLock};

/// java: the `map.put(..)` calls that fill `FLATTENING_MAP`
const FLATTENING_MAP_ENTRIES: &[(&str, &str)] = &[
    ("minecraft:stone.0", "minecraft:stone"),
    ("minecraft:stone.1", "minecraft:granite"),
    ("minecraft:stone.2", "minecraft:polished_granite"),
    ("minecraft:stone.3", "minecraft:diorite"),
    ("minecraft:stone.4", "minecraft:polished_diorite"),
    ("minecraft:stone.5", "minecraft:andesite"),
    ("minecraft:stone.6", "minecraft:polished_andesite"),
    ("minecraft:dirt.0", "minecraft:dirt"),
    ("minecraft:dirt.1", "minecraft:coarse_dirt"),
    ("minecraft:dirt.2", "minecraft:podzol"),
    ("minecraft:leaves.0", "minecraft:oak_leaves"),
    ("minecraft:leaves.1", "minecraft:spruce_leaves"),
    ("minecraft:leaves.2", "minecraft:birch_leaves"),
    ("minecraft:leaves.3", "minecraft:jungle_leaves"),
    ("minecraft:leaves2.0", "minecraft:acacia_leaves"),
    ("minecraft:leaves2.1", "minecraft:dark_oak_leaves"),
    ("minecraft:log.0", "minecraft:oak_log"),
    ("minecraft:log.1", "minecraft:spruce_log"),
    ("minecraft:log.2", "minecraft:birch_log"),
    ("minecraft:log.3", "minecraft:jungle_log"),
    ("minecraft:log2.0", "minecraft:acacia_log"),
    ("minecraft:log2.1", "minecraft:dark_oak_log"),
    ("minecraft:sapling.0", "minecraft:oak_sapling"),
    ("minecraft:sapling.1", "minecraft:spruce_sapling"),
    ("minecraft:sapling.2", "minecraft:birch_sapling"),
    ("minecraft:sapling.3", "minecraft:jungle_sapling"),
    ("minecraft:sapling.4", "minecraft:acacia_sapling"),
    ("minecraft:sapling.5", "minecraft:dark_oak_sapling"),
    ("minecraft:planks.0", "minecraft:oak_planks"),
    ("minecraft:planks.1", "minecraft:spruce_planks"),
    ("minecraft:planks.2", "minecraft:birch_planks"),
    ("minecraft:planks.3", "minecraft:jungle_planks"),
    ("minecraft:planks.4", "minecraft:acacia_planks"),
    ("minecraft:planks.5", "minecraft:dark_oak_planks"),
    ("minecraft:sand.0", "minecraft:sand"),
    ("minecraft:sand.1", "minecraft:red_sand"),
    ("minecraft:quartz_block.0", "minecraft:quartz_block"),
    ("minecraft:quartz_block.1", "minecraft:chiseled_quartz_block"),
    ("minecraft:quartz_block.2", "minecraft:quartz_pillar"),
    ("minecraft:anvil.0", "minecraft:anvil"),
    ("minecraft:anvil.1", "minecraft:chipped_anvil"),
    ("minecraft:anvil.2", "minecraft:damaged_anvil"),
    ("minecraft:wool.0", "minecraft:white_wool"),
    ("minecraft:wool.1", "minecraft:orange_wool"),
    ("minecraft:wool.2", "minecraft:magenta_wool"),
    ("minecraft:wool.3", "minecraft:light_blue_wool"),
    ("minecraft:wool.4", "minecraft:yellow_wool"),
    ("minecraft:wool.5", "minecraft:lime_wool"),
    ("minecraft:wool.6", "minecraft:pink_wool"),
    ("minecraft:wool.7", "minecraft:gray_wool"),
    ("minecraft:wool.8", "minecraft:light_gray_wool"),
    ("minecraft:wool.9", "minecraft:cyan_wool"),
    ("minecraft:wool.10", "minecraft:purple_wool"),
    ("minecraft:wool.11", "minecraft:blue_wool"),
    ("minecraft:wool.12", "minecraft:brown_wool"),
    ("minecraft:wool.13", "minecraft:green_wool"),
    ("minecraft:wool.14", "minecraft:red_wool"),
    ("minecraft:wool.15", "minecraft:black_wool"),
    ("minecraft:carpet.0", "minecraft:white_carpet"),
    ("minecraft:carpet.1", "minecraft:orange_carpet"),
    ("minecraft:carpet.2", "minecraft:magenta_carpet"),
    ("minecraft:carpet.3", "minecraft:light_blue_carpet"),
    ("minecraft:carpet.4", "minecraft:yellow_carpet"),
    ("minecraft:carpet.5", "minecraft:lime_carpet"),
    ("minecraft:carpet.6", "minecraft:pink_carpet"),
    ("minecraft:carpet.7", "minecraft:gray_carpet"),
    ("minecraft:carpet.8", "minecraft:light_gray_carpet"),
    ("minecraft:carpet.9", "minecraft:cyan_carpet"),
    ("minecraft:carpet.10", "minecraft:purple_carpet"),
    ("minecraft:carpet.11", "minecraft:blue_carpet"),
    ("minecraft:carpet.12", "minecraft:brown_carpet"),
    ("minecraft:carpet.13", "minecraft:green_carpet"),
    ("minecraft:carpet.14", "minecraft:red_carpet"),
    ("minecraft:carpet.15", "minecraft:black_carpet"),
    ("minecraft:hardened_clay.0", "minecraft:terracotta"),
    ("minecraft:stained_hardened_clay.0", "minecraft:white_terracotta"),
    ("minecraft:stained_hardened_clay.1", "minecraft:orange_terracotta"),
    ("minecraft:stained_hardened_clay.2", "minecraft:magenta_terracotta"),
    ("minecraft:stained_hardened_clay.3", "minecraft:light_blue_terracotta"),
    ("minecraft:stained_hardened_clay.4", "minecraft:yellow_terracotta"),
    ("minecraft:stained_hardened_clay.5", "minecraft:lime_terracotta"),
    ("minecraft:stained_hardened_clay.6", "minecraft:pink_terracotta"),
    ("minecraft:stained_hardened_clay.7", "minecraft:gray_terracotta"),
    ("minecraft:stained_hardened_clay.8", "minecraft:light_gray_terracotta"),
    ("minecraft:stained_hardened_clay.9", "minecraft:cyan_terracotta"),
    ("minecraft:stained_hardened_clay.10", "minecraft:purple_terracotta"),
    ("minecraft:stained_hardened_clay.11", "minecraft:blue_terracotta"),
    ("minecraft:stained_hardened_clay.12", "minecraft:brown_terracotta"),
    ("minecraft:stained_hardened_clay.13", "minecraft:green_terracotta"),
    ("minecraft:stained_hardened_clay.14", "minecraft:red_terracotta"),
    ("minecraft:stained_hardened_clay.15", "minecraft:black_terracotta"),
    ("minecraft:silver_glazed_terracotta.0", "minecraft:light_gray_glazed_terracotta"),
    ("minecraft:stained_glass.0", "minecraft:white_stained_glass"),
    ("minecraft:stained_glass.1", "minecraft:orange_stained_glass"),
    ("minecraft:stained_glass.2", "minecraft:magenta_stained_glass"),
    ("minecraft:stained_glass.3", "minecraft:light_blue_stained_glass"),
    ("minecraft:stained_glass.4", "minecraft:yellow_stained_glass"),
    ("minecraft:stained_glass.5", "minecraft:lime_stained_glass"),
    ("minecraft:stained_glass.6", "minecraft:pink_stained_glass"),
    ("minecraft:stained_glass.7", "minecraft:gray_stained_glass"),
    ("minecraft:stained_glass.8", "minecraft:light_gray_stained_glass"),
    ("minecraft:stained_glass.9", "minecraft:cyan_stained_glass"),
    ("minecraft:stained_glass.10", "minecraft:purple_stained_glass"),
    ("minecraft:stained_glass.11", "minecraft:blue_stained_glass"),
    ("minecraft:stained_glass.12", "minecraft:brown_stained_glass"),
    ("minecraft:stained_glass.13", "minecraft:green_stained_glass"),
    ("minecraft:stained_glass.14", "minecraft:red_stained_glass"),
    ("minecraft:stained_glass.15", "minecraft:black_stained_glass"),
    ("minecraft:stained_glass_pane.0", "minecraft:white_stained_glass_pane"),
    ("minecraft:stained_glass_pane.1", "minecraft:orange_stained_glass_pane"),
    ("minecraft:stained_glass_pane.2", "minecraft:magenta_stained_glass_pane"),
    ("minecraft:stained_glass_pane.3", "minecraft:light_blue_stained_glass_pane"),
    ("minecraft:stained_glass_pane.4", "minecraft:yellow_stained_glass_pane"),
    ("minecraft:stained_glass_pane.5", "minecraft:lime_stained_glass_pane"),
    ("minecraft:stained_glass_pane.6", "minecraft:pink_stained_glass_pane"),
    ("minecraft:stained_glass_pane.7", "minecraft:gray_stained_glass_pane"),
    ("minecraft:stained_glass_pane.8", "minecraft:light_gray_stained_glass_pane"),
    ("minecraft:stained_glass_pane.9", "minecraft:cyan_stained_glass_pane"),
    ("minecraft:stained_glass_pane.10", "minecraft:purple_stained_glass_pane"),
    ("minecraft:stained_glass_pane.11", "minecraft:blue_stained_glass_pane"),
    ("minecraft:stained_glass_pane.12", "minecraft:brown_stained_glass_pane"),
    ("minecraft:stained_glass_pane.13", "minecraft:green_stained_glass_pane"),
    ("minecraft:stained_glass_pane.14", "minecraft:red_stained_glass_pane"),
    ("minecraft:stained_glass_pane.15", "minecraft:black_stained_glass_pane"),
    ("minecraft:prismarine.0", "minecraft:prismarine"),
    ("minecraft:prismarine.1", "minecraft:prismarine_bricks"),
    ("minecraft:prismarine.2", "minecraft:dark_prismarine"),
    ("minecraft:concrete.0", "minecraft:white_concrete"),
    ("minecraft:concrete.1", "minecraft:orange_concrete"),
    ("minecraft:concrete.2", "minecraft:magenta_concrete"),
    ("minecraft:concrete.3", "minecraft:light_blue_concrete"),
    ("minecraft:concrete.4", "minecraft:yellow_concrete"),
    ("minecraft:concrete.5", "minecraft:lime_concrete"),
    ("minecraft:concrete.6", "minecraft:pink_concrete"),
    ("minecraft:concrete.7", "minecraft:gray_concrete"),
    ("minecraft:concrete.8", "minecraft:light_gray_concrete"),
    ("minecraft:concrete.9", "minecraft:cyan_concrete"),
    ("minecraft:concrete.10", "minecraft:purple_concrete"),
    ("minecraft:concrete.11", "minecraft:blue_concrete"),
    ("minecraft:concrete.12", "minecraft:brown_concrete"),
    ("minecraft:concrete.13", "minecraft:green_concrete"),
    ("minecraft:concrete.14", "minecraft:red_concrete"),
    ("minecraft:concrete.15", "minecraft:black_concrete"),
    ("minecraft:concrete_powder.0", "minecraft:white_concrete_powder"),
    ("minecraft:concrete_powder.1", "minecraft:orange_concrete_powder"),
    ("minecraft:concrete_powder.2", "minecraft:magenta_concrete_powder"),
    ("minecraft:concrete_powder.3", "minecraft:light_blue_concrete_powder"),
    ("minecraft:concrete_powder.4", "minecraft:yellow_concrete_powder"),
    ("minecraft:concrete_powder.5", "minecraft:lime_concrete_powder"),
    ("minecraft:concrete_powder.6", "minecraft:pink_concrete_powder"),
    ("minecraft:concrete_powder.7", "minecraft:gray_concrete_powder"),
    ("minecraft:concrete_powder.8", "minecraft:light_gray_concrete_powder"),
    ("minecraft:concrete_powder.9", "minecraft:cyan_concrete_powder"),
    ("minecraft:concrete_powder.10", "minecraft:purple_concrete_powder"),
    ("minecraft:concrete_powder.11", "minecraft:blue_concrete_powder"),
    ("minecraft:concrete_powder.12", "minecraft:brown_concrete_powder"),
    ("minecraft:concrete_powder.13", "minecraft:green_concrete_powder"),
    ("minecraft:concrete_powder.14", "minecraft:red_concrete_powder"),
    ("minecraft:concrete_powder.15", "minecraft:black_concrete_powder"),
    ("minecraft:cobblestone_wall.0", "minecraft:cobblestone_wall"),
    ("minecraft:cobblestone_wall.1", "minecraft:mossy_cobblestone_wall"),
    ("minecraft:sandstone.0", "minecraft:sandstone"),
    ("minecraft:sandstone.1", "minecraft:chiseled_sandstone"),
    ("minecraft:sandstone.2", "minecraft:cut_sandstone"),
    ("minecraft:red_sandstone.0", "minecraft:red_sandstone"),
    ("minecraft:red_sandstone.1", "minecraft:chiseled_red_sandstone"),
    ("minecraft:red_sandstone.2", "minecraft:cut_red_sandstone"),
    ("minecraft:stonebrick.0", "minecraft:stone_bricks"),
    ("minecraft:stonebrick.1", "minecraft:mossy_stone_bricks"),
    ("minecraft:stonebrick.2", "minecraft:cracked_stone_bricks"),
    ("minecraft:stonebrick.3", "minecraft:chiseled_stone_bricks"),
    ("minecraft:monster_egg.0", "minecraft:infested_stone"),
    ("minecraft:monster_egg.1", "minecraft:infested_cobblestone"),
    ("minecraft:monster_egg.2", "minecraft:infested_stone_bricks"),
    ("minecraft:monster_egg.3", "minecraft:infested_mossy_stone_bricks"),
    ("minecraft:monster_egg.4", "minecraft:infested_cracked_stone_bricks"),
    ("minecraft:monster_egg.5", "minecraft:infested_chiseled_stone_bricks"),
    ("minecraft:yellow_flower.0", "minecraft:dandelion"),
    ("minecraft:red_flower.0", "minecraft:poppy"),
    ("minecraft:red_flower.1", "minecraft:blue_orchid"),
    ("minecraft:red_flower.2", "minecraft:allium"),
    ("minecraft:red_flower.3", "minecraft:azure_bluet"),
    ("minecraft:red_flower.4", "minecraft:red_tulip"),
    ("minecraft:red_flower.5", "minecraft:orange_tulip"),
    ("minecraft:red_flower.6", "minecraft:white_tulip"),
    ("minecraft:red_flower.7", "minecraft:pink_tulip"),
    ("minecraft:red_flower.8", "minecraft:oxeye_daisy"),
    ("minecraft:double_plant.0", "minecraft:sunflower"),
    ("minecraft:double_plant.1", "minecraft:lilac"),
    ("minecraft:double_plant.2", "minecraft:tall_grass"),
    ("minecraft:double_plant.3", "minecraft:large_fern"),
    ("minecraft:double_plant.4", "minecraft:rose_bush"),
    ("minecraft:double_plant.5", "minecraft:peony"),
    ("minecraft:deadbush.0", "minecraft:dead_bush"),
    ("minecraft:tallgrass.0", "minecraft:dead_bush"),
    ("minecraft:tallgrass.1", "minecraft:grass"),
    ("minecraft:tallgrass.2", "minecraft:fern"),
    ("minecraft:sponge.0", "minecraft:sponge"),
    ("minecraft:sponge.1", "minecraft:wet_sponge"),
    ("minecraft:purpur_slab.0", "minecraft:purpur_slab"),
    ("minecraft:stone_slab.0", "minecraft:stone_slab"),
    ("minecraft:stone_slab.1", "minecraft:sandstone_slab"),
    ("minecraft:stone_slab.2", "minecraft:petrified_oak_slab"),
    ("minecraft:stone_slab.3", "minecraft:cobblestone_slab"),
    ("minecraft:stone_slab.4", "minecraft:brick_slab"),
    ("minecraft:stone_slab.5", "minecraft:stone_brick_slab"),
    ("minecraft:stone_slab.6", "minecraft:nether_brick_slab"),
    ("minecraft:stone_slab.7", "minecraft:quartz_slab"),
    ("minecraft:stone_slab2.0", "minecraft:red_sandstone_slab"),
    ("minecraft:wooden_slab.0", "minecraft:oak_slab"),
    ("minecraft:wooden_slab.1", "minecraft:spruce_slab"),
    ("minecraft:wooden_slab.2", "minecraft:birch_slab"),
    ("minecraft:wooden_slab.3", "minecraft:jungle_slab"),
    ("minecraft:wooden_slab.4", "minecraft:acacia_slab"),
    ("minecraft:wooden_slab.5", "minecraft:dark_oak_slab"),
    ("minecraft:coal.0", "minecraft:coal"),
    ("minecraft:coal.1", "minecraft:charcoal"),
    ("minecraft:fish.0", "minecraft:cod"),
    ("minecraft:fish.1", "minecraft:salmon"),
    ("minecraft:fish.2", "minecraft:clownfish"),
    ("minecraft:fish.3", "minecraft:pufferfish"),
    ("minecraft:cooked_fish.0", "minecraft:cooked_cod"),
    ("minecraft:cooked_fish.1", "minecraft:cooked_salmon"),
    ("minecraft:skull.0", "minecraft:skeleton_skull"),
    ("minecraft:skull.1", "minecraft:wither_skeleton_skull"),
    ("minecraft:skull.2", "minecraft:zombie_head"),
    ("minecraft:skull.3", "minecraft:player_head"),
    ("minecraft:skull.4", "minecraft:creeper_head"),
    ("minecraft:skull.5", "minecraft:dragon_head"),
    ("minecraft:golden_apple.0", "minecraft:golden_apple"),
    ("minecraft:golden_apple.1", "minecraft:enchanted_golden_apple"),
    ("minecraft:fireworks.0", "minecraft:firework_rocket"),
    ("minecraft:firework_charge.0", "minecraft:firework_star"),
    ("minecraft:dye.0", "minecraft:ink_sac"),
    ("minecraft:dye.1", "minecraft:rose_red"),
    ("minecraft:dye.2", "minecraft:cactus_green"),
    ("minecraft:dye.3", "minecraft:cocoa_beans"),
    ("minecraft:dye.4", "minecraft:lapis_lazuli"),
    ("minecraft:dye.5", "minecraft:purple_dye"),
    ("minecraft:dye.6", "minecraft:cyan_dye"),
    ("minecraft:dye.7", "minecraft:light_gray_dye"),
    ("minecraft:dye.8", "minecraft:gray_dye"),
    ("minecraft:dye.9", "minecraft:pink_dye"),
    ("minecraft:dye.10", "minecraft:lime_dye"),
    ("minecraft:dye.11", "minecraft:dandelion_yellow"),
    ("minecraft:dye.12", "minecraft:light_blue_dye"),
    ("minecraft:dye.13", "minecraft:magenta_dye"),
    ("minecraft:dye.14", "minecraft:orange_dye"),
    ("minecraft:dye.15", "minecraft:bone_meal"),
    ("minecraft:silver_shulker_box.0", "minecraft:light_gray_shulker_box"),
    ("minecraft:fence.0", "minecraft:oak_fence"),
    ("minecraft:fence_gate.0", "minecraft:oak_fence_gate"),
    ("minecraft:wooden_door.0", "minecraft:oak_door"),
    ("minecraft:boat.0", "minecraft:oak_boat"),
    ("minecraft:lit_pumpkin.0", "minecraft:jack_o_lantern"),
    ("minecraft:pumpkin.0", "minecraft:carved_pumpkin"),
    ("minecraft:trapdoor.0", "minecraft:oak_trapdoor"),
    ("minecraft:nether_brick.0", "minecraft:nether_bricks"),
    ("minecraft:red_nether_brick.0", "minecraft:red_nether_bricks"),
    ("minecraft:netherbrick.0", "minecraft:nether_brick"),
    ("minecraft:wooden_button.0", "minecraft:oak_button"),
    ("minecraft:wooden_pressure_plate.0", "minecraft:oak_pressure_plate"),
    ("minecraft:noteblock.0", "minecraft:note_block"),
    ("minecraft:bed.0", "minecraft:white_bed"),
    ("minecraft:bed.1", "minecraft:orange_bed"),
    ("minecraft:bed.2", "minecraft:magenta_bed"),
    ("minecraft:bed.3", "minecraft:light_blue_bed"),
    ("minecraft:bed.4", "minecraft:yellow_bed"),
    ("minecraft:bed.5", "minecraft:lime_bed"),
    ("minecraft:bed.6", "minecraft:pink_bed"),
    ("minecraft:bed.7", "minecraft:gray_bed"),
    ("minecraft:bed.8", "minecraft:light_gray_bed"),
    ("minecraft:bed.9", "minecraft:cyan_bed"),
    ("minecraft:bed.10", "minecraft:purple_bed"),
    ("minecraft:bed.11", "minecraft:blue_bed"),
    ("minecraft:bed.12", "minecraft:brown_bed"),
    ("minecraft:bed.13", "minecraft:green_bed"),
    ("minecraft:bed.14", "minecraft:red_bed"),
    ("minecraft:bed.15", "minecraft:black_bed"),
    ("minecraft:banner.15", "minecraft:white_banner"),
    ("minecraft:banner.14", "minecraft:orange_banner"),
    ("minecraft:banner.13", "minecraft:magenta_banner"),
    ("minecraft:banner.12", "minecraft:light_blue_banner"),
    ("minecraft:banner.11", "minecraft:yellow_banner"),
    ("minecraft:banner.10", "minecraft:lime_banner"),
    ("minecraft:banner.9", "minecraft:pink_banner"),
    ("minecraft:banner.8", "minecraft:gray_banner"),
    ("minecraft:banner.7", "minecraft:light_gray_banner"),
    ("minecraft:banner.6", "minecraft:cyan_banner"),
    ("minecraft:banner.5", "minecraft:purple_banner"),
    ("minecraft:banner.4", "minecraft:blue_banner"),
    ("minecraft:banner.3", "minecraft:brown_banner"),
    ("minecraft:banner.2", "minecraft:green_banner"),
    ("minecraft:banner.1", "minecraft:red_banner"),
    ("minecraft:banner.0", "minecraft:black_banner"),
    ("minecraft:grass.0", "minecraft:grass_block"),
    ("minecraft:brick_block.0", "minecraft:bricks"),
    ("minecraft:end_bricks.0", "minecraft:end_stone_bricks"),
    ("minecraft:golden_rail.0", "minecraft:powered_rail"),
    ("minecraft:magma.0", "minecraft:magma_block"),
    ("minecraft:quartz_ore.0", "minecraft:nether_quartz_ore"),
    ("minecraft:reeds.0", "minecraft:sugar_cane"),
    ("minecraft:slime.0", "minecraft:slime_block"),
    ("minecraft:stone_stairs.0", "minecraft:cobblestone_stairs"),
    ("minecraft:waterlily.0", "minecraft:lily_pad"),
    ("minecraft:web.0", "minecraft:cobweb"),
    ("minecraft:snow.0", "minecraft:snow_block"),
    ("minecraft:snow_layer.0", "minecraft:snow"),
    ("minecraft:record_11.0", "minecraft:music_disc_11"),
    ("minecraft:record_13.0", "minecraft:music_disc_13"),
    ("minecraft:record_blocks.0", "minecraft:music_disc_blocks"),
    ("minecraft:record_cat.0", "minecraft:music_disc_cat"),
    ("minecraft:record_chirp.0", "minecraft:music_disc_chirp"),
    ("minecraft:record_far.0", "minecraft:music_disc_far"),
    ("minecraft:record_mall.0", "minecraft:music_disc_mall"),
    ("minecraft:record_mellohi.0", "minecraft:music_disc_mellohi"),
    ("minecraft:record_stal.0", "minecraft:music_disc_stal"),
    ("minecraft:record_strad.0", "minecraft:music_disc_strad"),
    ("minecraft:record_wait.0", "minecraft:music_disc_wait"),
    ("minecraft:record_ward.0", "minecraft:music_disc_ward"),
    ("minecraft:mob_spawner.0", "minecraft:spawner"),
];

/// java: private static final Map<String, String> FLATTENING_MAP = DataFixUtils.make(Maps.<String, String>newHashMap(), map -> { .. });
static FLATTENING_MAP: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| FLATTENING_MAP_ENTRIES.iter().copied().collect());

/// java: private static final Set<String> ORIGINAL_ITEM_NAMES = FLATTENING_MAP.keySet().stream().map(oldId -> oldId.substring(0, oldId.indexOf(46))).collect(Collectors.toSet());
static ORIGINAL_ITEM_NAMES: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    FLATTENING_MAP
        .keys()
        .map(|old_id: &&'static str| -> &'static str {
            let old_id: &'static str = old_id;
            match old_id.find('.') {
            Some(index) => &old_id[..index],
            // java: `indexOf` answers -1 and `substring(0, -1)` throws StringIndexOutOfBoundsException - no key lacks a '.'
            None => panic!("java.lang.StringIndexOutOfBoundsException: begin 0, end -1, length {}", old_id.len()),
            }
        })
        .collect()
});

/// java: private static final Set<String> DAMAGEABLE_ITEMS = Sets.<String>newHashSet(..);
static DAMAGEABLE_ITEMS: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    [
        "minecraft:bow",
        "minecraft:carrot_on_a_stick",
        "minecraft:chainmail_boots",
        "minecraft:chainmail_chestplate",
        "minecraft:chainmail_helmet",
        "minecraft:chainmail_leggings",
        "minecraft:diamond_axe",
        "minecraft:diamond_boots",
        "minecraft:diamond_chestplate",
        "minecraft:diamond_helmet",
        "minecraft:diamond_hoe",
        "minecraft:diamond_leggings",
        "minecraft:diamond_pickaxe",
        "minecraft:diamond_shovel",
        "minecraft:diamond_sword",
        "minecraft:elytra",
        "minecraft:fishing_rod",
        "minecraft:flint_and_steel",
        "minecraft:golden_axe",
        "minecraft:golden_boots",
        "minecraft:golden_chestplate",
        "minecraft:golden_helmet",
        "minecraft:golden_hoe",
        "minecraft:golden_leggings",
        "minecraft:golden_pickaxe",
        "minecraft:golden_shovel",
        "minecraft:golden_sword",
        "minecraft:iron_axe",
        "minecraft:iron_boots",
        "minecraft:iron_chestplate",
        "minecraft:iron_helmet",
        "minecraft:iron_hoe",
        "minecraft:iron_leggings",
        "minecraft:iron_pickaxe",
        "minecraft:iron_shovel",
        "minecraft:iron_sword",
        "minecraft:leather_boots",
        "minecraft:leather_chestplate",
        "minecraft:leather_helmet",
        "minecraft:leather_leggings",
        "minecraft:shears",
        "minecraft:shield",
        "minecraft:stone_axe",
        "minecraft:stone_hoe",
        "minecraft:stone_pickaxe",
        "minecraft:stone_shovel",
        "minecraft:stone_sword",
        "minecraft:wooden_axe",
        "minecraft:wooden_hoe",
        "minecraft:wooden_pickaxe",
        "minecraft:wooden_shovel",
        "minecraft:wooden_sword",
    ]
    .into_iter()
    .collect()
});

/// java: public class TheFlatteningFix extends DataFix
pub struct TheFlatteningFix {
    fields: DataFixFields,
}

impl TheFlatteningFix {
    /// java: public TheFlatteningFix(Schema outputSchema, boolean changesType)
    pub fn new(output_schema: Arc<Schema>, changes_type: bool) -> TheFlatteningFix {
        TheFlatteningFix { fields: DataFixFields::new(output_schema, changes_type) }
    }

    /// java: the `fixTypeEverywhereTyped` lambda of `makeRule`, over the stack's remainder
    fn fix(item_stack: Dynamic) -> Dynamic {
        // java: Optional<Pair<String, String>> itemId = itemStackTyped.getOptional(itemIdFinder);
        let item_id = item_stack.get("id").as_string().result();

        if let Some(item_id) = item_id {
            // java: Dynamic<?> stack = itemStackTyped.get(DSL.remainderFinder());
            let mut stack = item_stack;
            // Gotta love Mojang using this for both item variants and item durability
            let damage = stack.get("Damage").as_int(0);
            let new_id = TheFlatteningFix::get_item(Some(&item_id), damage);

            if let Some(new_id) = new_id {
                // java: itemStackTyped = itemStackTyped.set(itemIdFinder, Pair.of(TypeReferences.ITEM_NAME.typeName(), newId));
                stack = stack.set("id", &stack.create_string(new_id));
            }

            if DAMAGEABLE_ITEMS.contains(item_id.as_str()) {
                // java: Typed<?> nbtTagTyped = itemStackTyped.getOrCreateTyped(nbtTagFinder);
                let mut nbt_tag_dynamic = stack.get("tag").or_else_empty_map();

                nbt_tag_dynamic = nbt_tag_dynamic.set("Damage", &nbt_tag_dynamic.create_int(damage));
                // java: itemStackTyped = itemStackTyped.set(nbtTagFinder, nbtTagTyped.set(DSL.remainderFinder(), nbtTagDynamic));
                stack = stack.set("tag", &nbt_tag_dynamic);
            }

            // java: return itemStackTyped.set(DSL.remainderFinder(), stack.remove("Damage"));
            return stack.remove("Damage");
        }

        item_stack
    }

    /// java: private static @Nullable String getItem(@Nullable String originalName, int damage)
    fn get_item(original_name: Option<&str>, damage: i32) -> Option<&'static str> {
        let original_name = original_name?;
        if ORIGINAL_ITEM_NAMES.contains(original_name) {
            let flattened_id = FLATTENING_MAP.get(format!("{original_name}.{damage}").as_str()).copied();

            match flattened_id {
                None => FLATTENING_MAP.get(format!("{original_name}.0").as_str()).copied(),
                flattened_id => flattened_id,
            }
        } else {
            None
        }
    }
}

impl DataFix for TheFlatteningFix {
    fn data_fix(&self) -> &DataFixFields {
        &self.fields
    }

    /// java: protected TypeRewriteRule makeRule()
    fn make_rule(&self) -> TypeRewriteRule {
        let r#type = self.get_input_schema().get_type(&type_references::LEGACY_ITEM_STACK).unwrap_or_else(|e| panic!("java.lang.IllegalArgumentException: {e}"));
        data_fix::fix_type_everywhere_typed_same(
            "TheFlatteningFix",
            r#type,
            Arc::new(|item_stack_typed: Typed| item_stack_typed.update(&dsl::remainder_finder(), &TheFlatteningFix::fix)),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::TheFlatteningFix;

    #[test]
    fn a_variant_flattens_and_an_unknown_damage_falls_back_to_zero() {
        assert_eq!(TheFlatteningFix::get_item(Some("minecraft:stone"), 1), Some("minecraft:granite"));
        assert_eq!(TheFlatteningFix::get_item(Some("minecraft:stone"), 99), Some("minecraft:stone"));
        assert_eq!(TheFlatteningFix::get_item(Some("minecraft:diamond"), 0), None);
    }
}

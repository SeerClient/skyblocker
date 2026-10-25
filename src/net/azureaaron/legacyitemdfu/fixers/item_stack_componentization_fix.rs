//! Ported from net.azureaaron.legacyitemdfu.fixers.ItemStackComponentizationFix (legacy-item-dfu 1.0.4+26.2).
//!
//! A `writeFixAndRead` over the whole stack, so the library itself works on the written `Dynamic` here, exactly as
//! the game's DFU runs it. The inner class `StackData` is in this file (one Java class = one file, inner classes in
//! the outer class's file).
//!
//! RUST SHAPES: Java's overloads get a suffix - `setComponent(String, OptionalDynamic)` is `set_component_optional`,
//! `moveToComponent(String, String, Dynamic)` is `move_to_component_with_default` and `moveToComponent(String,
//! Dynamic, String)` is `move_to_component_into`. A lambda that writes `data` while `applyFixer` runs is handed the
//! `StackData` as its first argument (Java's lambda captures the same object). A `Map` collected through
//! `Collectors.toMap` / `asMap` is a `Vec` of pairs in encounter order (the DFU port's own shape for one,
//! `dynamic_like.rs`).

use crate::net::azureaaron::legacyitemdfu::fixers::banner_pattern_format_fix::BannerPatternFormatFix;
use crate::net::azureaaron::legacyitemdfu::fixers::fixes::text_fixes::TextFixes;
use crate::net::azureaaron::legacyitemdfu::schemas::identifier_normalizing_schema::IdentifierNormalizingSchema;
use crate::net::azureaaron::legacyitemdfu::type_references;
use mcrust::com::mojang::datafixers::data_fix::{self, DataFix, DataFixFields};
use mcrust::com::mojang::datafixers::schemas::schema::Schema;
use mcrust::com::mojang::datafixers::type_rewrite_rule::TypeRewriteRule;
use mcrust::com::mojang::serialization::dynamic::Dynamic;
use mcrust::com::mojang::serialization::dynamic_like::DynamicLike as _;
use mcrust::com::mojang::serialization::optional_dynamic::OptionalDynamic;
use std::collections::HashSet;
use std::sync::{Arc, LazyLock};

/// java: private static final int HIDE_ENCHANTMENTS_FLAG = 1;
const HIDE_ENCHANTMENTS_FLAG: i32 = 1;
/// java: private static final int HIDE_MODIFIERS_FLAG = 2;
const HIDE_MODIFIERS_FLAG: i32 = 2;
/// java: private static final int HIDE_UNBREAKABLE_FLAG = 4;
const HIDE_UNBREAKABLE_FLAG: i32 = 4;
/// java: private static final int HIDE_CAN_DESTROY_FLAG = 8;
const HIDE_CAN_DESTROY_FLAG: i32 = 8;
/// java: private static final int HIDE_CAN_PLACE_FLAG = 16;
const HIDE_CAN_PLACE_FLAG: i32 = 16;
/// java: private static final int HIDE_ADDITIONAL_FLAG = 32;
const HIDE_ADDITIONAL_FLAG: i32 = 32;
/// java: private static final int HIDE_DYED_FLAG = 64;
const HIDE_DYED_FLAG: i32 = 64;
/// java: private static final int HIDE_UPGRADE_FLAG = 128;
const HIDE_UPGRADE_FLAG: i32 = 128;

/// java: private static final Set<String> POTION_ITEM_IDS = Set.of(..);
static POTION_ITEM_IDS: LazyLock<HashSet<&'static str>> =
    LazyLock::new(|| ["minecraft:potion", "minecraft:splash_potion", "minecraft:lingering_potion", "minecraft:tipped_arrow"].into_iter().collect());

/// java: private static final Set<String> ENTITY_BUCKET_ITEM_IDS = Set.of(..);
static ENTITY_BUCKET_ITEM_IDS: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    [
        "minecraft:pufferfish_bucket",
        "minecraft:salmon_bucket",
        "minecraft:cod_bucket",
        "minecraft:tropical_fish_bucket",
        "minecraft:axolotl_bucket",
        "minecraft:tadpole_bucket",
    ]
    .into_iter()
    .collect()
});

/// java: private static final List<String> RELEVANT_ENTITY_NBT_KEYS = List.of(..);
const RELEVANT_ENTITY_NBT_KEYS: &[&str] = &["NoAI", "Silent", "NoGravity", "Glowing", "Invulnerable", "Health", "Age", "Variant", "HuntingCooldown", "BucketVariantTag"];

/// java: private static final Set<String> BOOLEAN_BLOCK_STATE_PROPERTIES = Set.of(..);
static BOOLEAN_BLOCK_STATE_PROPERTIES: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    [
        "attached",
        "bottom",
        "conditional",
        "disarmed",
        "drag",
        "enabled",
        "extended",
        "eye",
        "falling",
        "hanging",
        "has_bottle_0",
        "has_bottle_1",
        "has_bottle_2",
        "has_record",
        "has_book",
        "inverted",
        "in_wall",
        "lit",
        "locked",
        "occupied",
        "open",
        "persistent",
        "powered",
        "short",
        "signal_fire",
        "snowy",
        "triggered",
        "unstable",
        "waterlogged",
        "berries",
        "bloom",
        "shrieking",
        "can_summon",
        "up",
        "down",
        "north",
        "east",
        "south",
        "west",
        "slot_0_occupied",
        "slot_1_occupied",
        "slot_2_occupied",
        "slot_3_occupied",
        "slot_4_occupied",
        "slot_5_occupied",
        "cracked",
        "crafting",
    ]
    .into_iter()
    .collect()
});

/// java: private static final Splitter COMMA_SPLITTER = Splitter.on(',');
///
/// `Splitter.on(',').split(s)` (no `omitEmptyStrings`, no `trimResults`) is every piece between commas, empty ones
/// included - `str::split(',')`.
fn comma_splitter_split(s: &str) -> std::str::Split<'_, char> {
    s.split(',')
}

/// java: public class ItemStackComponentizationFix extends DataFix
pub struct ItemStackComponentizationFix {
    fields: DataFixFields,
}

impl ItemStackComponentizationFix {
    /// java: public ItemStackComponentizationFix(Schema outputSchema, boolean changesType)
    pub fn new(output_schema: Arc<Schema>, changes_type: bool) -> ItemStackComponentizationFix {
        ItemStackComponentizationFix { fields: DataFixFields::new(output_schema, changes_type) }
    }

    /// java: private static void fixStack(StackData data, Dynamic<?> dynamic)
    fn fix_stack(data: &mut StackData, dynamic: &Dynamic) {
        let hide_flags = data.get_and_remove("HideFlags").as_int(0);

        data.move_to_component_with_default("Damage", "minecraft:damage", &dynamic.create_int(0));
        data.move_to_component_with_default("RepairCost", "minecraft:repair_cost", &dynamic.create_int(0));
        data.move_to_component("CustomModelData", "minecraft:custom_model_data");

        // Block, Entity and Block Entity tag fixes
        if let Some(block_state_tag_dynamic) = data.get_and_remove("BlockStateTag").result() {
            data.set_component("minecraft:block_state", &ItemStackComponentizationFix::fix_block_state_tag(&block_state_tag_dynamic));
        }
        data.move_to_component("EntityTag", "minecraft:entity_data");
        data.apply_fixer("BlockEntityTag", false, |data: &mut StackData, mut block_entity_tag_dynamic: Dynamic| {
            let id = IdentifierNormalizingSchema::normalize(&block_entity_tag_dynamic.get("id").as_string_or(&data.item_id));
            block_entity_tag_dynamic = ItemStackComponentizationFix::fix_block_entity_data(data, block_entity_tag_dynamic, &id);
            let new_block_entity_tag_dynamic = block_entity_tag_dynamic.remove("id");

            if new_block_entity_tag_dynamic == block_entity_tag_dynamic.empty_map() { new_block_entity_tag_dynamic } else { block_entity_tag_dynamic }
        });
        data.move_to_component("BlockEntityTag", "minecraft:block_entity_data");

        // Unbreakable
        if data.get_and_remove("Unbreakable").as_boolean_or(false) {
            let mut unbreakable_component = dynamic.empty_map();
            if (hide_flags & HIDE_UNBREAKABLE_FLAG) != 0 {
                unbreakable_component = unbreakable_component.set("show_in_tooltip", &dynamic.create_boolean(false));
            }

            data.set_component("minecraft:unbreakable", &unbreakable_component);
        }

        // Enchantments (applied to items and enchanted books
        ItemStackComponentizationFix::fix_enchantments(data, dynamic, "Enchantments", "minecraft:enchantments", (hide_flags & HIDE_ENCHANTMENTS_FLAG) != 0);
        if data.item_equals("minecraft:enchanted_book") {
            ItemStackComponentizationFix::fix_enchantments(data, dynamic, "StoredEnchantments", "minecraft:stored_enchantments", (hide_flags & HIDE_ADDITIONAL_FLAG) != 0);
        }

        // Fix display - item custom name, lore, item dye colour
        data.apply_fixer("display", false, |data: &mut StackData, display_dynamic: Dynamic| ItemStackComponentizationFix::fix_display(data, display_dynamic, hide_flags));

        // Fix can place/destroy predicates and attribute modifiers
        ItemStackComponentizationFix::fix_adventure_mode_predicates(data, dynamic, hide_flags);
        ItemStackComponentizationFix::fix_attribute_modifiers(data, dynamic, hide_flags);

        // Armour Trims
        let trim_opt = data.get_and_remove("Trim").result();
        if let Some(mut trim_component_dynamic) = trim_opt {
            if (hide_flags & HIDE_UPGRADE_FLAG) != 0 {
                trim_component_dynamic = trim_component_dynamic.set("show_in_tooltip", &trim_component_dynamic.create_boolean(false));
            }

            data.set_component("minecraft:trim", &trim_component_dynamic);
        }

        if (hide_flags & HIDE_ADDITIONAL_FLAG) != 0 {
            data.set_component("minecraft:hide_additional_tooltip", &dynamic.empty_map());
        }

        // Item Specific Data Fixes
        if data.item_equals("minecraft:crossbow") {
            data.get_and_remove("Charged");
            data.move_to_component_with_default("ChargedProjectiles", "minecraft:charged_projectiles", &dynamic.create_list(Vec::new()));
        }

        if data.item_equals("minecraft:bundle") {
            data.move_to_component_with_default("Items", "minecraft:bundle_contents", &dynamic.create_list(Vec::new()));
        }

        if data.item_equals("minecraft:filled_map") {
            data.move_to_component("map", "minecraft:map_id");
            // java: Collectors.toMap(Pair::getFirst, Pair::getSecond, (dynamicx, _) -> dynamicx) - the FIRST value of a repeated key is kept
            let mut map: Vec<(Dynamic, Dynamic)> = Vec::new();
            for (key, value) in data.get_and_remove("Decorations").as_stream().into_iter().map(|d| ItemStackComponentizationFix::fix_map_decorations(&d)) {
                if !map.iter().any(|(k, _)| *k == key) {
                    map.push((key, value));
                }
            }
            if !map.is_empty() {
                data.set_component("minecraft:map_decorations", &dynamic.create_map(map));
            }
        }

        if data.item_matches(&POTION_ITEM_IDS) {
            ItemStackComponentizationFix::fix_potion_contents(data, dynamic);
        }

        if data.item_equals("minecraft:writable_book") {
            ItemStackComponentizationFix::fix_writable_book_content(data, dynamic);
        }

        if data.item_equals("minecraft:written_book") {
            ItemStackComponentizationFix::fix_written_book_content(data, dynamic);
        }

        if data.item_equals("minecraft:suspicious_stew") {
            data.move_to_component("effects", "minecraft:suspicious_stew_effects");
        }

        if data.item_equals("minecraft:debug_stick") {
            data.move_to_component("DebugProperty", "minecraft:debug_stick_state");
        }

        if data.item_matches(&ENTITY_BUCKET_ITEM_IDS) {
            ItemStackComponentizationFix::fix_bucket_entity_data(data, dynamic);
        }

        if data.item_equals("minecraft:goat_horn") {
            data.move_to_component("instrument", "minecraft:instrument");
        }

        if data.item_equals("minecraft:knowledge_book") {
            data.move_to_component("Recipes", "minecraft:recipes");
        }

        if data.item_equals("minecraft:compass") {
            ItemStackComponentizationFix::fix_lodestone_target(data, dynamic);
        }

        if data.item_equals("minecraft:firework_rocket") {
            ItemStackComponentizationFix::fix_fireworks(data);
        }

        if data.item_equals("minecraft:firework_star") {
            ItemStackComponentizationFix::fix_explosion_data(data);
        }

        if data.item_equals("minecraft:player_head") {
            if let Some(skull_owner_dynamic) = data.get_and_remove("SkullOwner").result() {
                data.set_component("minecraft:profile", &ItemStackComponentizationFix::create_profile_dynamic(&skull_owner_dynamic));
            }
        }
    }

    /// java: private static Dynamic<?> fixBlockStateTag(Dynamic<?> dynamic)
    fn fix_block_state_tag(dynamic: &Dynamic) -> Dynamic {
        let fixed: Option<Dynamic> = dynamic
            .as_map_opt()
            .result()
            .map(|stream| {
                stream
                    .into_iter()
                    .map(|(key, dynamicx)| {
                        let string = key.as_string_or("");
                        if BOOLEAN_BLOCK_STATE_PROPERTIES.contains(string.as_str()) {
                            let optional = dynamicx.as_boolean().result();
                            if let Some(optional) = optional {
                                // java: dynamicx.createString(String.valueOf(optional.get()))
                                let value = dynamicx.create_string(if optional { "true" } else { "false" });
                                return (key, value);
                            }
                        }

                        let optional = dynamicx.as_number().result();
                        // java: dynamicx.createString(optional.get().toString()) - `Number.toString()` of the boxed number
                        let value = match optional {
                            Some(number) => dynamicx.create_string(&number.to_java_string()),
                            None => dynamicx,
                        };
                        (key, value)
                    })
                    .collect::<Vec<_>>()
            })
            .map(|map| dynamic.create_map(map));
        fixed.unwrap_or_else(|| dynamic.clone())
    }

    /// java: private static <T> Dynamic<T> fixBlockEntityData(ItemStackComponentizationFix.StackData data, Dynamic<T> dynamic, String blockEntityId)
    fn fix_block_entity_data(data: &mut StackData, mut dynamic: Dynamic, block_entity_id: &str) -> Dynamic {
        data.set_component_optional("minecraft:lock", &dynamic.get("Lock"));
        dynamic = dynamic.remove("Lock");
        let optional = dynamic.get("LootTable").result();
        if let Some(optional) = optional {
            let mut dynamic2 = dynamic.empty_map().set("loot_table", &optional);
            let l = dynamic.get("LootTableSeed").as_long(0);
            if l != 0 {
                dynamic2 = dynamic2.set("seed", &dynamic.create_long(l));
            }

            data.set_component("minecraft:container_loot", &dynamic2);
            dynamic = dynamic.remove("LootTable").remove("LootTableSeed");
        }
        match block_entity_id {
            "minecraft:skull" => {
                data.set_component_optional("minecraft:note_block_sound", &dynamic.get("note_block_sound"));
                dynamic.remove("note_block_sound")
            }

            "minecraft:decorated_pot" => {
                data.set_component_optional("minecraft:pot_decorations", &dynamic.get("sherds"));
                let optional2 = dynamic.get("item").result();
                if let Some(optional2) = optional2 {
                    data.set_component("minecraft:container", &dynamic.create_list(vec![dynamic.empty_map().set("slot", &dynamic.create_int(0)).set("item", &optional2)]));
                }

                dynamic.remove("sherds").remove("item")
            }

            // Modify this so that it matches all possible banner ids
            s if s.starts_with("minecraft:") && s.ends_with("banner") => {
                data.set_component_optional("minecraft:banner_patterns", &dynamic.get("patterns"));
                let base_opt = dynamic.get("Base").as_number().result();
                if let Some(base) = base_opt {
                    data.set_component("minecraft:base_color", &dynamic.create_string(BannerPatternFormatFix::get_colour_from_int(base.int_value())));
                }

                // TODO see if this matters that we removed base earlier, prob not
                dynamic.remove("patterns").remove("Base")
            }

            "minecraft:shulker_box" | "minecraft:chest" | "minecraft:trapped_chest" | "minecraft:furnace" | "minecraft:ender_chest" | "minecraft:dispenser" | "minecraft:dropper" | "minecraft:brewing_stand" | "minecraft:hopper" | "minecraft:barrel" | "minecraft:smoker" | "minecraft:blast_furnace" | "minecraft:campfire" | "minecraft:chiseled_bookshelf" | "minecraft:crafter" => {
                let items: Vec<Dynamic> = dynamic.get("Items").as_list(|items_dynamic: Dynamic| {
                    items_dynamic
                        .empty_map()
                        .set("slot", &items_dynamic.create_int((items_dynamic.get("Slot").as_byte(0) as i32) & 255))
                        .set("item", &items_dynamic.remove("Slot"))
                });
                if !items.is_empty() {
                    data.set_component("minecraft:container", &dynamic.create_list(items));
                }

                dynamic.remove("Items")
            }

            "minecraft:beehive" => {
                data.set_component_optional("minecraft:bees", &dynamic.get("bees"));
                dynamic.remove("bees")
            }

            _ => dynamic,
        }
    }

    /// java: private static void fixEnchantments(ItemStackComponentizationFix.StackData data, Dynamic<?> dynamic, String nbtKey, String componentId, boolean hideInTooltip)
    fn fix_enchantments(data: &mut StackData, dynamic: &Dynamic, nbt_key: &str, component_id: &str, hide_in_tooltip: bool) {
        let enchantments_list_dynamic = data.get_and_remove(nbt_key);
        let enchantments_applied: Vec<(String, i32)> = enchantments_list_dynamic
            .as_list(|d| d)
            .into_iter()
            .flat_map(|enchantments_dynamic| ItemStackComponentizationFix::get_enchantment_and_level_pair(&enchantments_dynamic))
            .collect();
        if !enchantments_applied.is_empty() || hide_in_tooltip {
            let mut component_dynamic = dynamic.empty_map();
            let mut levels_dynamic = dynamic.empty_map();

            for (enchantment, level) in &enchantments_applied {
                levels_dynamic = levels_dynamic.set(enchantment, &dynamic.create_int(*level));
            }

            component_dynamic = component_dynamic.set("levels", &levels_dynamic);
            if hide_in_tooltip {
                component_dynamic = component_dynamic.set("show_in_tooltip", &dynamic.create_boolean(false));
            }

            data.set_component(component_id, &component_dynamic);
        }

        if enchantments_list_dynamic.result().is_some() && enchantments_applied.is_empty() {
            data.set_component("minecraft:enchantment_glint_override", &dynamic.create_boolean(true));
        }
    }

    /// java: private static Optional<Pair<String, Integer>> getEnchantmentAndLevelPair(Dynamic<?> dynamic)
    fn get_enchantment_and_level_pair(dynamic: &Dynamic) -> Option<(String, i32)> {
        dynamic
            .get("id")
            .as_string()
            // java: Pair.of(enchantmentId, Math.clamp(level.intValue(), 0, 255))
            .apply2stable(|enchantment_id, level: mcrust::java::lang::number::Number| (enchantment_id, level.int_value().clamp(0, 255)), dynamic.get("lvl").as_number())
            .result()
    }

    /// java: private static Dynamic<?> fixDisplay(ItemStackComponentizationFix.StackData data, Dynamic<?> dynamic, int hideFlags)
    fn fix_display(data: &mut StackData, mut dynamic: Dynamic, hide_flags: i32) -> Dynamic {
        data.set_component_optional("minecraft:custom_name", &dynamic.get("Name"));
        data.set_component_optional("minecraft:lore", &dynamic.get("Lore"));

        let dye_colour_optional: Option<i32> = dynamic.get("color").as_number().result().map(|n| n.int_value());
        let hide_dye_colour = (hide_flags & HIDE_DYED_FLAG) != 0;

        if dye_colour_optional.is_some() || hide_dye_colour {
            let mut dye_colour_component_dynamic = dynamic.empty_map().set("rgb", &dynamic.create_int(dye_colour_optional.unwrap_or(10511680)));
            if hide_dye_colour {
                dye_colour_component_dynamic = dye_colour_component_dynamic.set("show_in_tooltip", &dynamic.create_boolean(false));
            }

            data.set_component("minecraft:dyed_color", &dye_colour_component_dynamic);
        }

        let localized_name_opt = dynamic.get("LocName").as_string().result();
        if let Some(localized_name) = localized_name_opt {
            data.set_component("minecraft:item_name", &TextFixes::translate(dynamic.get_ops(), &localized_name));
        }

        if data.item_equals("minecraft:filled_map") {
            data.set_component_optional("minecraft:map_color", &dynamic.get("MapColor"));
            dynamic = dynamic.remove("MapColor");
        }

        dynamic.remove("Name").remove("Lore").remove("color").remove("LocName")
    }

    /// java: private static void fixAdventureModePredicates(ItemStackComponentizationFix.StackData data, Dynamic<?> dynamic, int hideFlags)
    fn fix_adventure_mode_predicates(data: &mut StackData, dynamic: &Dynamic, hide_flags: i32) {
        ItemStackComponentizationFix::fix_block_predicate_list(data, dynamic, "CanDestroy", "minecraft:can_break", (hide_flags & HIDE_CAN_DESTROY_FLAG) != 0);
        ItemStackComponentizationFix::fix_block_predicate_list(data, dynamic, "CanPlaceOn", "minecraft:can_place_on", (hide_flags & HIDE_CAN_PLACE_FLAG) != 0);
    }

    /// java: private static void fixBlockPredicateList(ItemStackComponentizationFix.StackData data, Dynamic<?> dynamic, String nbtKey, String componentId, boolean hideInTooltip)
    fn fix_block_predicate_list(data: &mut StackData, dynamic: &Dynamic, nbt_key: &str, component_id: &str, hide_in_tooltip: bool) {
        let optional = data.get_and_remove(nbt_key).result();
        if let Some(optional) = optional {
            let mut component_dynamic = dynamic.empty_map().set(
                "predicates",
                &dynamic.create_list(
                    optional
                        .as_stream()
                        .into_iter()
                        .map(|predicates_dynamic| {
                            // java: DataFixUtils.orElse(predicatesDynamic.asString().map(string -> createBlockPredicateListDynamic(predicatesDynamic, string)).result(), predicatesDynamic)
                            let fixed: Option<Dynamic> = predicates_dynamic
                                .as_string()
                                .map(|string| ItemStackComponentizationFix::create_block_predicate_list_dynamic(&predicates_dynamic, &string))
                                .result();
                            fixed.unwrap_or(predicates_dynamic)
                        })
                        .collect(),
                ),
            );
            if hide_in_tooltip {
                component_dynamic = component_dynamic.set("show_in_tooltip", &dynamic.create_boolean(false));
            }

            data.set_component(component_id, &component_dynamic);
        }
    }

    /// java: private static Dynamic<?> createBlockPredicateListDynamic(Dynamic<?> dynamic, String listAsString)
    ///
    /// Every `indexOf` is of an ASCII character (`[` 91, `{` 123, `]` 93, `=` 61, `}` 125), so the byte offsets `str::find`
    /// answers slice where Java's char offsets do.
    fn create_block_predicate_list_dynamic(dynamic: &Dynamic, list_as_string: &str) -> Dynamic {
        let i = list_as_string.find('[');
        let j = list_as_string.find('{');
        let mut k = list_as_string.len();
        if let Some(i) = i {
            k = i;
        }

        if let Some(j) = j {
            k = k.min(j);
        }

        let string = &list_as_string[..k];
        // java: string.trim() - `String.trim` strips every char <= ' ' from both ends
        let mut dynamic2 = dynamic.empty_map().set("blocks", &dynamic.create_string(string.trim_matches(|c: char| c <= ' ')));
        let l = list_as_string.find(']');
        if let (Some(i), Some(l)) = (i, l) {
            let mut dynamic3 = dynamic.empty_map();

            // java: listAsString.substring(i + 1, l) - throws StringIndexOutOfBoundsException when the first ']' comes before the first '['
            let inner = list_as_string
                .get(i + 1..l)
                .unwrap_or_else(|| panic!("java.lang.StringIndexOutOfBoundsException: begin {}, end {}, length {}", i + 1, l, list_as_string.len()));
            for string2 in comma_splitter_split(inner) {
                let m = string2.find('=');
                if let Some(m) = m {
                    let string3 = string2[..m].trim_matches(|c: char| c <= ' ');
                    let string4 = string2[m + 1..].trim_matches(|c: char| c <= ' ');
                    dynamic3 = dynamic3.set(string3, &dynamic.create_string(string4));
                }
            }

            dynamic2 = dynamic2.set("state", &dynamic3);
        }

        let n = list_as_string.find('}');
        if let (Some(j), Some(n)) = (j, n) {
            // java: listAsString.substring(j, n + 1) - throws when the first '}' comes before the first '{'
            let nbt = list_as_string
                .get(j..n + 1)
                .unwrap_or_else(|| panic!("java.lang.StringIndexOutOfBoundsException: begin {}, end {}, length {}", j, n + 1, list_as_string.len()));
            dynamic2 = dynamic2.set("nbt", &dynamic.create_string(nbt));
        }

        dynamic2
    }

    /// java: private static void fixAttributeModifiers(ItemStackComponentizationFix.StackData data, Dynamic<?> dynamic, int hideFlags)
    fn fix_attribute_modifiers(data: &mut StackData, dynamic: &Dynamic, hide_flags: i32) {
        let old_attribute_modifiers = data.get_and_remove("AttributeModifiers");

        if old_attribute_modifiers.result().is_some() {
            let hide_modifiers_from_tooltip = (hide_flags & HIDE_MODIFIERS_FLAG) != 0;
            let modifiers: Vec<Dynamic> = old_attribute_modifiers.as_list(|d| ItemStackComponentizationFix::fix_attribute_modifier(&d));
            let mut attribute_modifiers_component_dynamic = dynamic.empty_map().set("modifiers", &dynamic.create_list(modifiers));

            if hide_modifiers_from_tooltip {
                attribute_modifiers_component_dynamic = attribute_modifiers_component_dynamic.set("show_in_tooltip", &dynamic.create_boolean(false));
            }

            data.set_component("minecraft:attribute_modifiers", &attribute_modifiers_component_dynamic);
        }
    }

    /// java: private static Dynamic<?> fixAttributeModifier(Dynamic<?> dynamic)
    fn fix_attribute_modifier(dynamic: &Dynamic) -> Dynamic {
        let mut modifier = dynamic
            .empty_map()
            .set("name", &dynamic.create_string(""))
            .set("amount", &dynamic.create_double(0.0))
            .set("operation", &dynamic.create_string("add_value"));
        modifier = Dynamic::copy_field(dynamic, "AttributeName", &modifier, "type");
        modifier = Dynamic::copy_field(dynamic, "Slot", &modifier, "slot");
        modifier = Dynamic::copy_field(dynamic, "UUID", &modifier, "uuid");
        modifier = Dynamic::copy_field(dynamic, "Name", &modifier, "name");
        modifier = Dynamic::copy_field(dynamic, "Amount", &modifier, "amount");

        Dynamic::copy_and_fix_field(dynamic, "Operation", &modifier, "operation", |operation_dynamic: Dynamic| {
            operation_dynamic.create_string(match operation_dynamic.as_int(0) {
                1 => "add_multiplied_base",
                2 => "add_multiplied_total",
                _ => "add_value",
            })
        })
    }

    /// java: private static Pair<Dynamic<?>, Dynamic<?>> fixMapDecorations(Dynamic<?> dynamic)
    fn fix_map_decorations(dynamic: &Dynamic) -> (Dynamic, Dynamic) {
        let id_dynamic = dynamic.get("id").result().unwrap_or_else(|| dynamic.create_string(""));
        let dynamic3 = dynamic
            .empty_map()
            .set("type", &dynamic.create_string(ItemStackComponentizationFix::get_map_decoration_name(dynamic.get("type").as_int(0))))
            .set("x", &dynamic.create_double(dynamic.get("x").as_double(0.0)))
            .set("z", &dynamic.create_double(dynamic.get("z").as_double(0.0)))
            .set("rotation", &dynamic.create_float(dynamic.get("rot").as_double(0.0) as f32));
        (id_dynamic, dynamic3)
    }

    /// java: private static String getMapDecorationName(int index)
    fn get_map_decoration_name(index: i32) -> &'static str {
        match index {
            1 => "frame",
            2 => "red_marker",
            3 => "blue_marker",
            4 => "target_x",
            5 => "target_point",
            6 => "player_off_map",
            7 => "player_off_limits",
            8 => "mansion",
            9 => "monument",
            10 => "banner_white",
            11 => "banner_orange",
            12 => "banner_magenta",
            13 => "banner_light_blue",
            14 => "banner_yellow",
            15 => "banner_lime",
            16 => "banner_pink",
            17 => "banner_gray",
            18 => "banner_light_gray",
            19 => "banner_cyan",
            20 => "banner_purple",
            21 => "banner_blue",
            22 => "banner_brown",
            23 => "banner_green",
            24 => "banner_red",
            25 => "banner_black",
            26 => "red_x",
            27 => "village_desert",
            28 => "village_plains",
            29 => "village_savanna",
            30 => "village_snowy",
            31 => "village_taiga",
            32 => "jungle_temple",
            33 => "swamp_hut",
            _ => "player",
        }
    }

    /// java: private static void fixPotionContents(ItemStackComponentizationFix.StackData data, Dynamic<?> dynamic)
    fn fix_potion_contents(data: &mut StackData, dynamic: &Dynamic) {
        let mut potion_contents_component = dynamic.empty_map();
        let potion = data.get_and_remove("Potion").as_string().result().filter(|potion_id| potion_id != "minecraft:empty");

        if let Some(potion) = potion {
            potion_contents_component = potion_contents_component.set("potion", &dynamic.create_string(&potion));
        }

        potion_contents_component = data.move_to_component_into("CustomPotionColor", &potion_contents_component, "custom_color");
        potion_contents_component = data.move_to_component_into("custom_potion_effects", &potion_contents_component, "custom_effects");
        if potion_contents_component != dynamic.empty_map() {
            data.set_component("minecraft:potion_contents", &potion_contents_component);
        }
    }

    /// java: private static void fixWritableBookContent(ItemStackComponentizationFix.StackData data, Dynamic<?> dynamic)
    fn fix_writable_book_content(data: &mut StackData, dynamic: &Dynamic) {
        let writable_book_content_component = ItemStackComponentizationFix::fix_book_pages(data, dynamic);

        if let Some(writable_book_content_component) = writable_book_content_component {
            data.set_component("minecraft:writable_book_content", &dynamic.empty_map().set("pages", &writable_book_content_component));
        }
    }

    /// java: private static void fixWrittenBookContent(ItemStackComponentizationFix.StackData data, Dynamic<?> dynamic)
    fn fix_written_book_content(data: &mut StackData, dynamic: &Dynamic) {
        let pages = ItemStackComponentizationFix::fix_book_pages(data, dynamic);
        let title = data.get_and_remove("title").as_string_or("");
        let filtered_title_opt = data.get_and_remove("filtered_title").as_string().result();
        let mut written_book_content_component = dynamic.empty_map();

        written_book_content_component = written_book_content_component.set("title", &ItemStackComponentizationFix::create_filterable_text_dynamic(dynamic, &title, filtered_title_opt.as_deref()));
        written_book_content_component = data.move_to_component_into("author", &written_book_content_component, "author");
        written_book_content_component = data.move_to_component_into("resolved", &written_book_content_component, "resolved");
        written_book_content_component = data.move_to_component_into("generation", &written_book_content_component, "generation");

        if let Some(pages) = pages {
            written_book_content_component = written_book_content_component.set("pages", &pages);
        }

        // Fix case where the book had no author and it would fail
        if written_book_content_component.get("author").result().is_none() {
            written_book_content_component = written_book_content_component.set("author", &dynamic.create_string("Unknown"));
        }

        data.set_component("minecraft:written_book_content", &written_book_content_component);
    }

    /// java: private static @Nullable Dynamic<?> fixBookPages(ItemStackComponentizationFix.StackData data, Dynamic<?> dynamic)
    fn fix_book_pages(data: &mut StackData, dynamic: &Dynamic) -> Option<Dynamic> {
        let pages: Vec<String> = data.get_and_remove("pages").as_list(|pages_dynamic| pages_dynamic.as_string_or(""));
        let filtered_pages: Vec<(String, String)> = data
            .get_and_remove("filtered_pages")
            .as_map(|filtered_pages_key_dynamic| filtered_pages_key_dynamic.as_string_or("0"), |filtered_pages_value_dynamic| filtered_pages_value_dynamic.as_string_or(""));
        if pages.is_empty() {
            None
        } else {
            let mut list2: Vec<Dynamic> = Vec::with_capacity(pages.len());

            for (i, string) in pages.iter().enumerate() {
                let key = i.to_string();
                let string2 = filtered_pages.iter().find(|(k, _)| *k == key).map(|(_, v)| v.as_str());
                list2.push(ItemStackComponentizationFix::create_filterable_text_dynamic(dynamic, string, string2));
            }

            Some(dynamic.create_list(list2))
        }
    }

    /// java: private static Dynamic<?> createFilterableTextDynamic(Dynamic<?> dynamic, String unfiltered, Optional<String> filtered)
    fn create_filterable_text_dynamic(dynamic: &Dynamic, unfiltered: &str, filtered: Option<&str>) -> Dynamic {
        let mut dynamic2 = dynamic.empty_map().set("raw", &dynamic.create_string(unfiltered));
        if let Some(filtered) = filtered {
            dynamic2 = dynamic2.set("filtered", &dynamic.create_string(filtered));
        }

        dynamic2
    }

    /// java: private static void fixBucketEntityData(ItemStackComponentizationFix.StackData data, Dynamic<?> dynamic)
    fn fix_bucket_entity_data(data: &mut StackData, dynamic: &Dynamic) {
        let mut bucket_entity_data_component_dynamic = dynamic.empty_map();

        for string in RELEVANT_ENTITY_NBT_KEYS {
            bucket_entity_data_component_dynamic = data.move_to_component_into(string, &bucket_entity_data_component_dynamic, string);
        }

        if bucket_entity_data_component_dynamic != dynamic.empty_map() {
            data.set_component("minecraft:bucket_entity_data", &bucket_entity_data_component_dynamic);
        }
    }

    /// java: private static void fixLodestoneTarget(ItemStackComponentizationFix.StackData data, Dynamic<?> dynamic)
    fn fix_lodestone_target(data: &mut StackData, dynamic: &Dynamic) {
        let lodestone_pos_opt = data.get_and_remove("LodestonePos").result();
        let lodestone_dimension_opt = data.get_and_remove("LodestoneDimension").result();

        if lodestone_pos_opt.is_some() || lodestone_dimension_opt.is_some() {
            let lodestone_tracked = data.get_and_remove("LodestoneTracked").as_boolean_or(true);
            let mut lodestone_tracker_component_dynamic = dynamic.empty_map();

            if let (Some(lodestone_pos), Some(lodestone_dimension)) = (&lodestone_pos_opt, &lodestone_dimension_opt) {
                lodestone_tracker_component_dynamic = lodestone_tracker_component_dynamic.set("target", &dynamic.empty_map().set("pos", lodestone_pos).set("dimension", lodestone_dimension));
            }

            if !lodestone_tracked {
                lodestone_tracker_component_dynamic = lodestone_tracker_component_dynamic.set("tracked", &dynamic.create_boolean(false));
            }

            data.set_component("minecraft:lodestone_tracker", &lodestone_tracker_component_dynamic);
        }
    }

    /// java: private static void fixExplosion(ItemStackComponentizationFix.StackData data) - the `(StackData)` overload.
    fn fix_explosion_data(data: &mut StackData) {
        data.apply_fixer("Explosion", true, |data: &mut StackData, explosion_dynamic: Dynamic| {
            data.set_component("minecraft:firework_explosion", &ItemStackComponentizationFix::fix_explosion(explosion_dynamic.clone()));
            explosion_dynamic.remove("Type").remove("Colors").remove("FadeColors").remove("Trail").remove("Flicker")
        });
    }

    /// java: private static void fixFireworks(ItemStackComponentizationFix.StackData data)
    fn fix_fireworks(data: &mut StackData) {
        data.apply_fixer("Fireworks", true, |data: &mut StackData, fireworks_dynamic: Dynamic| {
            let stream: Vec<Dynamic> = fireworks_dynamic.get("Explosions").as_stream().into_iter().map(ItemStackComponentizationFix::fix_explosion).collect();
            let flight_duration = fireworks_dynamic.get("Flight").as_int(0);
            data.set_component(
                "minecraft:fireworks",
                &fireworks_dynamic.empty_map().set("explosions", &fireworks_dynamic.create_list(stream)).set("flight_duration", &fireworks_dynamic.create_byte(flight_duration as i8)),
            );
            fireworks_dynamic.remove("Explosions").remove("Flight")
        });
    }

    /// java: private static Dynamic<?> fixExplosion(Dynamic<?> dynamic) - the `(Dynamic)` overload.
    fn fix_explosion(mut dynamic: Dynamic) -> Dynamic {
        let shape = match dynamic.get("Type").as_int(0) {
            1 => "large_ball",
            2 => "star",
            3 => "creeper",
            4 => "burst",
            _ => "small_ball",
        };
        dynamic = dynamic.set("shape", &dynamic.create_string(shape)).remove("Type");
        dynamic = dynamic.rename_field("Colors", "colors");
        dynamic = dynamic.rename_field("FadeColors", "fade_colors");
        dynamic = dynamic.rename_field("Trail", "has_trail");
        dynamic.rename_field("Flicker", "has_twinkle")
    }

    /// java: public static Dynamic<?> createProfileDynamic(Dynamic<?> dynamic)
    pub fn create_profile_dynamic(dynamic: &Dynamic) -> Dynamic {
        let optional = dynamic.as_string().result();
        if let Some(optional) = optional {
            if ItemStackComponentizationFix::is_valid_username(&optional) { dynamic.empty_map().set("name", &dynamic.create_string(&optional)) } else { dynamic.empty_map() }
        } else {
            let name = dynamic.get("Name").as_string_or("");
            let optional2 = dynamic.get("Id").result();
            let properties_dynamic = ItemStackComponentizationFix::create_properties_dynamic(&dynamic.get("Properties"));
            let mut profile_component_dynamic = dynamic.empty_map();
            if ItemStackComponentizationFix::is_valid_username(&name) {
                profile_component_dynamic = profile_component_dynamic.set("name", &dynamic.create_string(&name));
            }

            if let Some(optional2) = optional2 {
                profile_component_dynamic = profile_component_dynamic.set("id", &optional2);
            }

            if let Some(properties_dynamic) = properties_dynamic {
                profile_component_dynamic = profile_component_dynamic.set("properties", &properties_dynamic);
            }

            profile_component_dynamic
        }
    }

    /// java: private static boolean isValidUsername(String username) - `length()` and `chars()` are UTF-16 units.
    fn is_valid_username(username: &str) -> bool {
        if username.encode_utf16().count() > 16 { false } else { !username.encode_utf16().any(|c| c <= 32 || c >= 127) }
    }

    /// java: @Nullable private static Dynamic<?> createPropertiesDynamic(OptionalDynamic<?> propertiesDynamic)
    fn create_properties_dynamic(properties_dynamic: &OptionalDynamic) -> Option<Dynamic> {
        let map: Vec<(String, Vec<(String, Option<String>)>)> = properties_dynamic.as_map(
            |dynamic| dynamic.as_string_or(""),
            |dynamic| {
                dynamic.as_list(|dynamicx| {
                    let string = dynamicx.get("Value").as_string_or("");
                    let optional = dynamicx.get("Signature").as_string().result();
                    (string, optional)
                })
            },
        );
        if map.is_empty() {
            None
        } else {
            Some(
                properties_dynamic.create_list(
                    map.iter()
                        .flat_map(|(key, value)| {
                            value.iter().map(move |(property_value, signature)| {
                                let property_dynamic = properties_dynamic.empty_map().set("name", &properties_dynamic.create_string(key)).set("value", &properties_dynamic.create_string(property_value));
                                match signature {
                                    Some(signature) => property_dynamic.set("signature", &properties_dynamic.create_string(signature)),
                                    None => property_dynamic,
                                }
                            })
                        })
                        .collect(),
                ),
            )
        }
    }
}

impl DataFix for ItemStackComponentizationFix {
    fn data_fix(&self) -> &DataFixFields {
        &self.fields
    }

    /// java: protected TypeRewriteRule makeRule()
    fn make_rule(&self) -> TypeRewriteRule {
        let input_type = self.get_input_schema().get_type(&type_references::LEGACY_ITEM_STACK).unwrap_or_else(|e| panic!("java.lang.IllegalArgumentException: {e}"));
        let output_type = self.get_output_schema().get_type(&type_references::LEGACY_ITEM_STACK).unwrap_or_else(|e| panic!("java.lang.IllegalArgumentException: {e}"));
        data_fix::write_fix_and_read(
            "ItemStack componentization",
            input_type,
            output_type,
            Arc::new(|item_stack_dynamic: Dynamic| {
                let optional = StackData::from_dynamic(&item_stack_dynamic).map(|mut data| {
                    let nbt = data.nbt.clone();
                    ItemStackComponentizationFix::fix_stack(&mut data, &nbt);

                    data.finalize_fix()
                });

                // java: DataFixUtils.orElse(optional, itemStackDynamic)
                optional.unwrap_or(item_stack_dynamic)
            }),
        )
    }
}

/// java: private static class StackData
struct StackData {
    /// java: private final String itemId;
    item_id: String,
    /// java: private final int count;
    count: i32,
    /// java: private Dynamic<?> components;
    components: Dynamic,
    /// java: private Dynamic<?> nbt;
    nbt: Dynamic,
    /// java: private final Dynamic<?> leftoverNbt;
    leftover_nbt: Dynamic,
}

impl StackData {
    /// java: StackData(String itemId, int count, Dynamic<?> dynamic)
    fn new(item_id: &str, count: i32, dynamic: &Dynamic) -> StackData {
        StackData {
            item_id: IdentifierNormalizingSchema::normalize(item_id),
            count,
            components: dynamic.empty_map(),
            nbt: dynamic.get("tag").or_else_empty_map(),
            leftover_nbt: dynamic.remove("tag"),
        }
    }

    /// java: static Optional<StackData> fromDynamic(Dynamic<?> dynamic)
    fn from_dynamic(dynamic: &Dynamic) -> Option<StackData> {
        dynamic
            .get("id")
            .as_string()
            .apply2stable(
                |item_id, count: mcrust::java::lang::number::Number| StackData::new(&item_id, count.int_value(), &dynamic.remove("id").remove("Count")),
                dynamic.get("Count").as_number(),
            )
            .result()
    }

    /// java: OptionalDynamic<?> getAndRemove(String key)
    fn get_and_remove(&mut self, key: &str) -> OptionalDynamic {
        let element = self.nbt.get(key);
        self.nbt = self.nbt.remove(key);
        element
    }

    /// java: void setComponent(String key, Dynamic<?> value)
    fn set_component(&mut self, key: &str, value: &Dynamic) {
        self.components = self.components.set(key, value);
    }

    /// java: void setComponent(String key, OptionalDynamic<?> optionalValue)
    fn set_component_optional(&mut self, key: &str, optional_value: &OptionalDynamic) {
        if let Some(value) = optional_value.result() {
            self.components = self.components.set(key, &value);
        }
    }

    /// java: void moveToComponent(String nbtKey, String componentId, Dynamic<?> defaultValue)
    fn move_to_component_with_default(&mut self, nbt_key: &str, component_id: &str, default_value: &Dynamic) {
        let optional = self.get_and_remove(nbt_key).result();

        if let Some(optional) = optional {
            if optional != *default_value {
                self.set_component(component_id, &optional);
            }
        }
    }

    /// java: void moveToComponent(String nbtKey, String componentId)
    fn move_to_component(&mut self, nbt_key: &str, component_id: &str) {
        if let Some(nbt) = self.get_and_remove(nbt_key).result() {
            self.set_component(component_id, &nbt);
        }
    }

    /// java: void applyFixer(String nbtKey, boolean removeIfEmpty, UnaryOperator<Dynamic<?>> fixer)
    fn apply_fixer(&mut self, nbt_key: &str, remove_if_empty: bool, fixer: impl FnOnce(&mut StackData, Dynamic) -> Dynamic) {
        let optional_dynamic = self.nbt.get(nbt_key);
        if !remove_if_empty || optional_dynamic.result().is_some() {
            let mut dynamic = optional_dynamic.or_else_empty_map();
            dynamic = fixer(self, dynamic);
            if dynamic == dynamic.empty_map() {
                self.nbt = self.nbt.remove(nbt_key);
            } else {
                self.nbt = self.nbt.set(nbt_key, &dynamic);
            }
        }
    }

    /// java: Dynamic<?> moveToComponent(String nbtKey, Dynamic<?> components, String componentId)
    fn move_to_component_into(&mut self, nbt_key: &str, components: &Dynamic, component_id: &str) -> Dynamic {
        let optional = self.get_and_remove(nbt_key).result();
        match optional {
            Some(optional) => components.set(component_id, &optional),
            None => components.clone(),
        }
    }

    /// java: Dynamic<?> finalizeFix()
    fn finalize_fix(mut self) -> Dynamic {
        let mut stack = self.nbt.empty_map().set("id", &self.nbt.create_string(&self.item_id)).set("count", &self.nbt.create_int(self.count));

        if self.nbt != self.nbt.empty_map() {
            self.components = self.components.set("minecraft:custom_data", &self.nbt);
        }

        if self.components != self.nbt.empty_map() {
            stack = stack.set("components", &self.components);
        }

        StackData::merge_leftover_nbt(&stack, &self.leftover_nbt)
    }

    /// java: static <T> Dynamic<T> mergeLeftoverNbt(Dynamic<T> data, Dynamic<?> leftoverNbt)
    fn merge_leftover_nbt(data: &Dynamic, leftover_nbt: &Dynamic) -> Dynamic {
        let ops = data.get_ops();

        // java: ops.getMap(data.getValue()).flatMap(mapLike -> ops.mergeToMap(leftoverNbt.convert(ops).getValue(), mapLike)).map(object -> new Dynamic<>(ops, object)).result().orElse(data)
        ops.get_map(ops, data.get_value())
            .flat_map(|map_like| ops.merge_to_map_all(leftover_nbt.convert(ops).get_value(), &map_like))
            .map(|object| Dynamic::new_with(ops.clone(), Some(object)))
            .result()
            .unwrap_or_else(|| data.clone())
    }

    /// java: boolean itemEquals(String itemId)
    fn item_equals(&self, item_id: &str) -> bool {
        self.item_id == item_id
    }

    /// java: boolean itemMatches(Set<String> itemIds)
    fn item_matches(&self, item_ids: &HashSet<&'static str>) -> bool {
        item_ids.contains(self.item_id.as_str())
    }

    /// java: @SuppressWarnings("unused") boolean itemContains(String componentId)
    #[allow(dead_code)]
    fn item_contains(&self, component_id: &str) -> bool {
        self.components.get(component_id).result().is_some()
    }
}

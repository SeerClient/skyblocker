//! Ported from net.azureaaron.legacyitemdfu.fixers.BannerPatternFormatFix (legacy-item-dfu 1.0.4+26.2).
//!
//! The library's finders are read off the stack's remainder (`schemas/item_stack_only_schema.rs`): `itemIdFinder` is
//! the `"id"` string, `nbtTagFinder` the optional `"tag"` field and `blockEntityTagFinder` the optional
//! `"tag".BlockEntityTag` field (whose type is `DSL.remainder()`, so `getOrCreate(DSL.remainderFinder())` is the whole
//! compound). `stack` - the stack's own remainder - is set back unchanged.

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

/// java: private static final Set<String> BANNER_IDS = Set.of(..);
static BANNER_IDS: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    [
        "minecraft:white_banner",
        "minecraft:light_gray_banner",
        "minecraft:gray_banner",
        "minecraft:black_banner",
        "minecraft:brown_banner",
        "minecraft:red_banner",
        "minecraft:orange_banner",
        "minecraft:yellow_banner",
        "minecraft:lime_banner",
        "minecraft:green_banner",
        "minecraft:cyan_banner",
        "minecraft:light_blue_banner",
        "minecraft:blue_banner",
        "minecraft:purple_banner",
        "minecraft:magenta_banner",
        "minecraft:pink_banner",
    ]
    .into_iter()
    .collect()
});

/// java: private static final Map<String, String> OLD_TO_NEW_PATTERNS = Map.ofEntries(..);
static OLD_TO_NEW_PATTERNS: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
    [
        ("b", "minecraft:base"),
        ("bl", "minecraft:square_bottom_left"),
        ("br", "minecraft:square_bottom_right"),
        ("tl", "minecraft:square_top_left"),
        ("tr", "minecraft:square_top_right"),
        ("bs", "minecraft:stripe_bottom"),
        ("ts", "minecraft:stripe_top"),
        ("ls", "minecraft:stripe_left"),
        ("rs", "minecraft:stripe_right"),
        ("cs", "minecraft:stripe_center"),
        ("ms", "minecraft:stripe_middle"),
        ("drs", "minecraft:stripe_downright"),
        ("dls", "minecraft:stripe_downleft"),
        ("ss", "minecraft:small_stripes"),
        ("cr", "minecraft:cross"),
        ("sc", "minecraft:straight_cross"),
        ("bt", "minecraft:triangle_bottom"),
        ("tt", "minecraft:triangle_top"),
        ("bts", "minecraft:triangles_bottom"),
        ("tts", "minecraft:triangles_top"),
        ("ld", "minecraft:diagonal_left"),
        ("rd", "minecraft:diagonal_up_right"),
        ("lud", "minecraft:diagonal_up_left"),
        ("rud", "minecraft:diagonal_right"),
        ("mc", "minecraft:circle"),
        ("mr", "minecraft:rhombus"),
        ("vh", "minecraft:half_vertical"),
        ("hh", "minecraft:half_horizontal"),
        ("vhr", "minecraft:half_vertical_right"),
        ("hhb", "minecraft:half_horizontal_bottom"),
        ("bo", "minecraft:border"),
        ("cbo", "minecraft:curly_border"),
        ("gra", "minecraft:gradient"),
        ("gru", "minecraft:gradient_up"),
        ("bri", "minecraft:bricks"),
        ("glb", "minecraft:globe"),
        ("cre", "minecraft:creeper"),
        ("sku", "minecraft:skull"),
        ("flo", "minecraft:flower"),
        ("moj", "minecraft:mojang"),
        ("pig", "minecraft:piglin"),
    ]
    .into_iter()
    .collect()
});

/// java: public class BannerPatternFormatFix extends DataFix
pub struct BannerPatternFormatFix {
    fields: DataFixFields,
}

impl BannerPatternFormatFix {
    /// java: public BannerPatternFormatFix(Schema outputSchema, boolean changesType)
    pub fn new(output_schema: Arc<Schema>, changes_type: bool) -> BannerPatternFormatFix {
        BannerPatternFormatFix { fields: DataFixFields::new(output_schema, changes_type) }
    }

    /// java: the `fixTypeEverywhereTyped` lambda of `makeRule`, over the stack's remainder
    fn fix(item_stack: Dynamic) -> Dynamic {
        // java: Optional<Pair<String, String>> itemId = itemStackTyped.getOptional(itemIdFinder);
        let item_id = item_stack.get("id").as_string().result();

        if item_id.as_deref().is_some_and(|id| BANNER_IDS.contains(id)) {
            // java: Optional<? extends Typed<?>> nbtTagOpt = itemStackTyped.getOptionalTyped(nbtTagFinder);
            if let Some(nbt_tag_typed) = item_stack.get("tag").result() {
                // java: Optional<? extends Typed<?>> blockEntityTagOpt = nbtTagTyped.getOptionalTyped(blockEntityTagFinder);
                if let Some(block_entity_tag_typed) = nbt_tag_typed.get("BlockEntityTag").result() {
                    // java: Dynamic<?> blockEntityTagDynamic = blockEntityTagTyped.getOrCreate(DSL.remainderFinder());
                    let block_entity_tag_dynamic = block_entity_tag_typed.rename_and_fix_field("Patterns", "patterns", |patterns_dynamic: Dynamic| {
                        patterns_dynamic.create_list(patterns_dynamic.as_stream().into_iter().map(BannerPatternFormatFix::replace_pattern_and_colour).collect())
                    });

                    // java: return itemStackTyped.set(DSL.remainderFinder(), stack).set(nbtTagFinder, nbtTagTyped.set(blockEntityTagFinder, blockEntityTagTyped.set(DSL.remainderFinder(), blockEntityTagDynamic)));
                    return item_stack.set("tag", &nbt_tag_typed.set("BlockEntityTag", &block_entity_tag_dynamic));
                }
            }
        }

        item_stack
    }

    /// java: private static Dynamic<?> replacePatternAndColour(Dynamic<?> dynamic)
    fn replace_pattern_and_colour(mut dynamic: Dynamic) -> Dynamic {
        dynamic = dynamic.rename_and_fix_field("Pattern", "pattern", |pattern_dynamic: Dynamic| {
            // java: DataFixUtils.orElse(patternDynamic.asString().map(patternId -> OLD_TO_NEW_PATTERNS.getOrDefault(patternId, patternId)).map(patternDynamic::createString).result(), patternDynamic)
            let fixed: Option<Dynamic> = pattern_dynamic
                .as_string()
                .map(|pattern_id| OLD_TO_NEW_PATTERNS.get(pattern_id.as_str()).map(|new| new.to_string()).unwrap_or(pattern_id))
                .map(|pattern_id| pattern_dynamic.create_string(&pattern_id))
                .result();
            fixed.unwrap_or(pattern_dynamic)
        });
        dynamic = dynamic.set("color", &dynamic.create_string(BannerPatternFormatFix::get_colour_from_int(dynamic.get("Color").as_int(0))));
        dynamic.remove("Color")
    }

    /// java: static String getColourFromInt(int color)
    pub(crate) fn get_colour_from_int(color: i32) -> &'static str {
        match color {
            1 => "orange",
            2 => "magenta",
            3 => "light_blue",
            4 => "yellow",
            5 => "lime",
            6 => "pink",
            7 => "gray",
            8 => "light_gray",
            9 => "cyan",
            10 => "purple",
            11 => "blue",
            12 => "brown",
            13 => "green",
            14 => "red",
            15 => "black",
            _ => "white",
        }
    }
}

impl DataFix for BannerPatternFormatFix {
    fn data_fix(&self) -> &DataFixFields {
        &self.fields
    }

    /// java: protected TypeRewriteRule makeRule()
    fn make_rule(&self) -> TypeRewriteRule {
        let r#type = self.get_input_schema().get_type(&type_references::LEGACY_ITEM_STACK).unwrap_or_else(|e| panic!("java.lang.IllegalArgumentException: {e}"));
        data_fix::fix_type_everywhere_typed_same(
            "BannerPatternFormatFix",
            r#type,
            Arc::new(|item_stack_typed: Typed| item_stack_typed.update(&dsl::remainder_finder(), &BannerPatternFormatFix::fix)),
        )
    }
}

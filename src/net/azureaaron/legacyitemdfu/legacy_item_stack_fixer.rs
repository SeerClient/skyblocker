//! Ported from net.azureaaron.legacyitemdfu.LegacyItemStackFixer (legacy-item-dfu 1.0.4+26.2).
//!
//! The fixer is the game's DataFixerUpper (`mcrust::com::mojang::datafixers`): a `DataFixerBuilder` holding the
//! library's sixteen schemas and fifteen fixes, built once into a `DataFixer`.

use crate::net::azureaaron::legacyitemdfu::fixers::attribute_id_fix::AttributeIdFix;
use crate::net::azureaaron::legacyitemdfu::fixers::banner_ids_fix::BannerIdsFix;
use crate::net::azureaaron::legacyitemdfu::fixers::banner_pattern_format_fix::BannerPatternFormatFix;
use crate::net::azureaaron::legacyitemdfu::fixers::enchantments_fix::EnchantmentsFix;
use crate::net::azureaaron::legacyitemdfu::fixers::hypixel_components_merge_fix::HypixelComponentsMergeFix;
use crate::net::azureaaron::legacyitemdfu::fixers::hypixel_components_rename_fix::HypixelComponentsRenameFix;
use crate::net::azureaaron::legacyitemdfu::fixers::item_custom_name_and_lore_to_text_fix::ItemCustomNameAndLoreToTextFix;
use crate::net::azureaaron::legacyitemdfu::fixers::item_stack_componentization_fix::ItemStackComponentizationFix;
use crate::net::azureaaron::legacyitemdfu::fixers::item_stack_uuids_fix::ItemStackUuidsFix;
use crate::net::azureaaron::legacyitemdfu::fixers::numeric_item_id_fix::NumericItemIdFix;
use crate::net::azureaaron::legacyitemdfu::fixers::post_flattening_item_ids_fix::PostFlatteningItemIdsFix;
use crate::net::azureaaron::legacyitemdfu::fixers::spawn_egg_item_id_fix::SpawnEggItemIdFix;
use crate::net::azureaaron::legacyitemdfu::fixers::the_flattening_fix::TheFlatteningFix;
use crate::net::azureaaron::legacyitemdfu::fixers::tooltip_display_fix::TooltipDisplayFix;
use crate::net::azureaaron::legacyitemdfu::fixers::unflatten_text_component_fix::UnflattenTextComponentFix;
use crate::net::azureaaron::legacyitemdfu::schemas::schema1::Schema1;
use crate::net::azureaaron::legacyitemdfu::schemas::schema2::Schema2;
use crate::net::azureaaron::legacyitemdfu::schemas::schema11::Schema11;
use mcrust::com::mojang::datafixers::data_fixer::DataFixer;
use mcrust::com::mojang::datafixers::data_fixer_builder::DataFixerBuilder;
use mcrust::com::mojang::datafixers::schemas::schema::Schema;
use std::sync::{Arc, LazyLock};

/// java: private static final int FIRST_VERSION = 1;
const FIRST_VERSION: i32 = 1;
/// java: private static final int LATEST_VERSION = 16;
const LATEST_VERSION: i32 = 16;
/// java: private static final DataFixer FIXER = build();
static FIXER: LazyLock<Arc<dyn DataFixer>> = LazyLock::new(LegacyItemStackFixer::build);

/// java: public final class LegacyItemStackFixer
pub struct LegacyItemStackFixer;

impl LegacyItemStackFixer {
    /// java: private static DataFixer build()
    fn build() -> Arc<dyn DataFixer> {
        let mut builder = DataFixerBuilder::new(LATEST_VERSION);

        builder.add_schema(1, Schema1::new);

        let numeric_item_id_fix_schema = builder.add_schema(2, Schema2::new);
        builder.add_fixer(Arc::new(NumericItemIdFix::new(numeric_item_id_fix_schema, true)));

        let hypixel_components_rename_fix_schema = builder.add_schema(3, Schema::new);
        builder.add_fixer(Arc::new(HypixelComponentsRenameFix::new(hypixel_components_rename_fix_schema, true)));

        let spawn_egg_item_id_fix_schema = builder.add_schema(4, Schema::new);
        builder.add_fixer(Arc::new(SpawnEggItemIdFix::new(spawn_egg_item_id_fix_schema, true)));

        let banner_ids_fix_schema = builder.add_schema(5, Schema::new);
        builder.add_fixer(Arc::new(BannerIdsFix::new(banner_ids_fix_schema, true)));

        let the_flattening_fix_schema = builder.add_schema(6, Schema::new);
        builder.add_fixer(Arc::new(TheFlatteningFix::new(the_flattening_fix_schema, true)));

        let post_flattening_fix_schema = builder.add_schema(7, Schema::new);
        builder.add_fixer(Arc::new(PostFlatteningItemIdsFix::new(post_flattening_fix_schema, true)));

        let item_custom_name_and_lore_to_text_fix_schema = builder.add_schema(8, Schema::new);
        builder.add_fixer(Arc::new(ItemCustomNameAndLoreToTextFix::new(item_custom_name_and_lore_to_text_fix_schema, true)));

        let enchantments_fix_schema = builder.add_schema(9, Schema::new);
        builder.add_fixer(Arc::new(EnchantmentsFix::new(enchantments_fix_schema, true)));

        let item_stack_uuids_fix_schema = builder.add_schema(10, Schema::new);
        builder.add_fixer(Arc::new(ItemStackUuidsFix::new(item_stack_uuids_fix_schema, true)));

        let banner_pattern_format_fix_schema = builder.add_schema(11, Schema::new);
        builder.add_fixer(Arc::new(BannerPatternFormatFix::new(banner_pattern_format_fix_schema, true)));

        let item_stack_componentization_fix_schema = builder.add_schema(12, Schema11::new);
        builder.add_fixer(Arc::new(ItemStackComponentizationFix::new(item_stack_componentization_fix_schema, true)));

        // This is here instead of the end since some tests relied on this being done sooner
        // it really does not matter as long as the merged in components from Hypixel
        // can be data fixed as needed for newer Minecraft versions
        let hypixel_components_merge_fix_schema = builder.add_schema(13, Schema::new);
        builder.add_fixer(Arc::new(HypixelComponentsMergeFix::new(hypixel_components_merge_fix_schema, true)));

        let attribute_id_fix_schema = builder.add_schema(14, Schema::new);
        builder.add_fixer(Arc::new(AttributeIdFix::new(attribute_id_fix_schema, true)));

        let unflatten_text_component_fix_schema = builder.add_schema(15, Schema::new);
        builder.add_fixer(Arc::new(UnflattenTextComponentFix::new(unflatten_text_component_fix_schema, true)));

        let tooltip_display_fix_schema = builder.add_schema(16, Schema::new);
        builder.add_fixer(Arc::new(TooltipDisplayFix::new(tooltip_display_fix_schema, true)));

        builder.build().fixer()
    }

    /// java: public static DataFixer getFixer()
    pub fn get_fixer() -> &'static Arc<dyn DataFixer> {
        &FIXER
    }

    /// java: public static int getFirstVersion()
    pub fn get_first_version() -> i32 {
        FIRST_VERSION
    }

    /// java: public static int getLatestVersion()
    pub fn get_latest_version() -> i32 {
        LATEST_VERSION
    }
}

#[cfg(test)]
mod tests {
    use super::LegacyItemStackFixer;
    use crate::net::azureaaron::legacyitemdfu::type_references;
    use mcrust::com::mojang::serialization::dynamic::Dynamic;
    use mcrust::com::mojang::serialization::dynamic_like::DynamicLike as _;
    use mcrust::com::mojang::serialization::obj;
    use mcrust::net::minecraft::nbt::nbt_ops::NbtOps;
    use mcrust::net::minecraft::nbt::tag::Tag;

    /// A 1.8 diamond sword (numeric id 276) with a legacy name, lore, `HideFlags` and `ExtraAttributes` - the NEU
    /// repository's shape - comes out as a 26.2 component stack.
    #[test]
    fn a_legacy_numeric_stack_becomes_a_component_stack() {
        let mut display = Tag::compound();
        display.put_string("Name", "§6Aspect of the End");
        let mut tag = Tag::compound();
        tag.put("display", display);
        tag.put_int("HideFlags", 254);
        let mut stack = Tag::compound();
        stack.put_short("id", 276);
        stack.put_int("Count", 1);
        stack.put_short("Damage", 0);
        stack.put("tag", tag);

        let input = Dynamic::new_with(NbtOps::instance(), Some(obj(stack)));
        let fixed = LegacyItemStackFixer::get_fixer()
            .update(&type_references::LEGACY_ITEM_STACK, input, LegacyItemStackFixer::get_first_version(), LegacyItemStackFixer::get_latest_version())
            .expect("the fixes throw nothing");

        assert_eq!(fixed.get("id").as_string().result().as_deref(), Some("minecraft:diamond_sword"));
        assert_eq!(fixed.get("count").as_int(0), 1);
        let components = fixed.get("components");
        assert!(components.get("minecraft:custom_name").get("text").as_string().result().is_some());
        assert!(components.get("minecraft:tooltip_display").get("hidden_components").as_stream().len() > 0);
    }
}

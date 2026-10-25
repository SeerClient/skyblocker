//! Ported from net.azureaaron.legacyitemdfu.fixers.TooltipDisplayFix (legacy-item-dfu 1.0.4+26.2).
//!
//! `itemStackTyped.updateTyped(componentsFinder, this::fix)` is `stack.update("components", ..)` over the stack's
//! remainder (`schemas/item_stack_only_schema.rs`; the `components` field's type is `DSL.remainder()`, and an absent
//! field is left alone).

use crate::net::azureaaron::legacyitemdfu::type_references;
use mcrust::com::mojang::datafixers::data_fix::{self, DataFix, DataFixFields};
use mcrust::com::mojang::datafixers::dsl;
use mcrust::com::mojang::datafixers::schemas::schema::Schema;
use mcrust::com::mojang::datafixers::type_rewrite_rule::TypeRewriteRule;
use mcrust::com::mojang::datafixers::typed::Typed;
use mcrust::com::mojang::serialization::dynamic::Dynamic;
use mcrust::com::mojang::serialization::dynamic_like::DynamicLike as _;
use std::cell::RefCell;
use std::sync::Arc;

/// java: private static final List<String> HIDE_ADDITIONAL_TOOLTIP_COMPONENTS = List.of(..);
///
/// Components whose tooltips were hidden by the pre-1.21.5 `hide_additional_tooltip` component.
const HIDE_ADDITIONAL_TOOLTIP_COMPONENTS: &[&str] = &[
    "minecraft:banner_patterns",
    "minecraft:bees",
    "minecraft:block_entity_data",
    "minecraft:block_state",
    "minecraft:bundle_contents",
    "minecraft:charged_projectiles",
    "minecraft:container",
    "minecraft:container_loot",
    "minecraft:firework_explosion",
    "minecraft:fireworks",
    "minecraft:instrument",
    "minecraft:map_id",
    "minecraft:painting/variant",
    "minecraft:pot_decorations",
    "minecraft:potion_contents",
    "minecraft:tropical_fish/pattern",
    "minecraft:written_book_content",
];

/// java: public class TooltipDisplayFix extends DataFix
pub struct TooltipDisplayFix {
    fields: DataFixFields,
}

impl TooltipDisplayFix {
    /// java: public TooltipDisplayFix(Schema outputSchema, boolean changesType)
    pub fn new(output_schema: Arc<Schema>, changes_type: bool) -> TooltipDisplayFix {
        TooltipDisplayFix { fields: DataFixFields::new(output_schema, changes_type) }
    }

    /// java: private Typed<?> fix(Typed<?> componentsTyped) - `componentsTyped.update(DSL.remainderFinder(), dynamic -> { .. })`
    fn fix(mut dynamic: Dynamic) -> Dynamic {
        // Lists ensures a consistent order
        let hidden_components: RefCell<Vec<String>> = RefCell::new(Vec::new());

        dynamic = TooltipDisplayFix::fix_component(&dynamic, "minecraft:can_break", &hidden_components);
        dynamic = TooltipDisplayFix::fix_component(&dynamic, "minecraft:can_place_on", &hidden_components);
        dynamic = TooltipDisplayFix::fix_component(&dynamic, "minecraft:trim", &hidden_components);
        dynamic = TooltipDisplayFix::fix_component(&dynamic, "minecraft:unbreakable", &hidden_components);
        dynamic = TooltipDisplayFix::fix_and_inline_component(&dynamic, "minecraft:dyed_color", "rgb", &hidden_components);
        dynamic = TooltipDisplayFix::fix_and_inline_component(&dynamic, "minecraft:attribute_modifiers", "modifiers", &hidden_components);
        dynamic = TooltipDisplayFix::fix_and_inline_component(&dynamic, "minecraft:enchantments", "levels", &hidden_components);
        dynamic = TooltipDisplayFix::fix_and_inline_component(&dynamic, "minecraft:stored_enchantments", "levels", &hidden_components);

        let should_hide_entire_tooltip = dynamic.get("minecraft:hide_tooltip").result().is_some();
        dynamic = dynamic.remove("minecraft:hide_tooltip");

        let should_hide_additional_tooltips = dynamic.get("minecraft:hide_additional_tooltip").result().is_some();
        dynamic = dynamic.remove("minecraft:hide_additional_tooltip");

        let mut hidden_components = hidden_components.into_inner();

        // Handle all the components hidden by the legacy hide_additional_tooltip component
        if should_hide_additional_tooltips {
            for id in HIDE_ADDITIONAL_TOOLTIP_COMPONENTS {
                if dynamic.get(id).result().is_some() {
                    hidden_components.push(id.to_string());
                }
            }
        }

        // Construct the new tooltip_display component if necessary
        if hidden_components.is_empty() && !should_hide_entire_tooltip {
            dynamic
        } else {
            // java: dynamic.createMap(Map.of("hide_tooltip", .., "hidden_components", ..))
            let tooltip_display = dynamic.create_map(vec![
                (dynamic.create_string("hide_tooltip"), dynamic.create_boolean(should_hide_entire_tooltip)),
                (
                    dynamic.create_string("hidden_components"),
                    dynamic.create_list(hidden_components.iter().map(|id| dynamic.create_string(id)).collect()),
                ),
            ]);
            dynamic.set("minecraft:tooltip_display", &tooltip_display)
        }
    }

    /// java: private static Dynamic<?> fixComponent(Dynamic<?> dynamic, String id, List<String> hiddenComponents)
    fn fix_component(dynamic: &Dynamic, id: &str, hidden_components: &RefCell<Vec<String>>) -> Dynamic {
        TooltipDisplayFix::fix_component_with(dynamic, id, hidden_components, &|component: Dynamic| component)
    }

    /// java: private static Dynamic<?> fixAndInlineComponent(Dynamic<?> dynamic, String id, String fieldToInline, List<String> hiddenComponents)
    fn fix_and_inline_component(dynamic: &Dynamic, id: &str, field_to_inline: &str, hidden_components: &RefCell<Vec<String>>) -> Dynamic {
        TooltipDisplayFix::fix_component_with(dynamic, id, hidden_components, &|component: Dynamic| {
            // java: DataFixUtils.orElse(component.get(fieldToInline).result(), component)
            component.get(field_to_inline).result().unwrap_or(component)
        })
    }

    /// java: private static Dynamic<?> fixComponent(Dynamic<?> dynamic, String id, List<String> hiddenComponents, UnaryOperator<Dynamic<?>> fixer)
    fn fix_component_with(dynamic: &Dynamic, id: &str, hidden_components: &RefCell<Vec<String>>, fixer: &dyn Fn(Dynamic) -> Dynamic) -> Dynamic {
        dynamic.update(id, |component: Dynamic| {
            let show_tooltip = component.get("show_in_tooltip").as_boolean_or(true);

            if !show_tooltip {
                hidden_components.borrow_mut().push(id.to_string());
            }

            fixer(component.remove("show_in_tooltip"))
        })
    }
}

impl DataFix for TooltipDisplayFix {
    fn data_fix(&self) -> &DataFixFields {
        &self.fields
    }

    /// java: protected TypeRewriteRule makeRule()
    fn make_rule(&self) -> TypeRewriteRule {
        let r#type = self.get_input_schema().get_type(&type_references::LEGACY_ITEM_STACK).unwrap_or_else(|e| panic!("java.lang.IllegalArgumentException: {e}"));
        data_fix::fix_type_everywhere_typed_same(
            "Tooltip Display Fix",
            r#type,
            Arc::new(|item_stack_typed: Typed| item_stack_typed.update(&dsl::remainder_finder(), &|stack: Dynamic| stack.update("components", TooltipDisplayFix::fix))),
        )
    }
}

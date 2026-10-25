//! Ported from net.azureaaron.legacyitemdfu.TypeReferences (legacy-item-dfu 1.0.4+26.2).

use mcrust::com::mojang::datafixers::dsl::TypeReference;

/// java: public final class TypeReferences
pub struct TypeReferences;

/// java: public static final DSL.TypeReference LEGACY_ITEM_STACK = () -> "legacy_item_stack";
pub const LEGACY_ITEM_STACK: TypeReference = TypeReference("legacy_item_stack");

/// java: public static final DSL.TypeReference ITEM_NAME = () -> "legacy_dfu_item_name";
pub const ITEM_NAME: TypeReference = TypeReference("legacy_dfu_item_name");

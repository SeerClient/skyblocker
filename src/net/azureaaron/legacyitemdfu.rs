//! package net.azureaaron.legacyitemdfu - Aaron's Legacy Item DFU (`net.azureaaron:legacy-item-dfu:1.0.4+26.2`,
//! github.com/AzureAaron/legacy-item-dfu), which Skyblocker's `utils.datafixer.LegacyItemStackFixer` runs to turn the
//! 1.8-era item NBT of the NEU repository and the Hypixel API into modern component stacks. Ported from its source
//! alongside the mod, as Fabric loads it beside it (package-info.java: `@NullMarked`).
//!
//! It is a set of DataFixerUpper schemas and fixes, built on the GAME's port of DFU (`mcrust::com::mojang::datafixers`):
//! `DataFixerBuilder`, `Schema`, the `DataFix` trait, `fix_type_everywhere_typed`, `write_fix_and_read`,
//! `Typed.update(DSL.remainderFinder(), ..)` and `TypeRewriteRule.seq`. See `schemas/item_stack_only_schema.rs` for
//! the one place the type algebra the library declares is wider than the one the game's DFU has.
pub mod fixers;
pub mod legacy_item_stack_fixer;
pub mod schemas;
pub mod type_references;
pub mod utils;

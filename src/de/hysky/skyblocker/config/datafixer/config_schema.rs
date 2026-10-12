//! Ported from de.hysky.skyblocker.config.datafixer.ConfigSchema by fabric2seer.
//!
//! `extends Schema`: the port's `Schema` is not subclassable; a class that overrides
//! `registerTypes` is `Schema::with_register_types(versionKey, parent, <the override>)`
//! (schema.rs header), so `new` answers that `Schema`.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::datafixer::config_data_fixer;
use mcrust::com::mojang::datafixers::dsl;
use mcrust::com::mojang::datafixers::schemas::schema::Schema;
use mcrust::com::mojang::datafixers::types::templates::type_template::TypeTemplate;

/// java: public class ConfigSchema extends Schema
pub struct ConfigSchema;

impl ConfigSchema {
    /// java: public ConfigSchema(int versionKey, Schema parent)
    pub fn new(version_key: i32, parent: Option<Arc<Schema>>) -> Schema {
        Schema::with_register_types(version_key, parent, Arc::new(|schema: &mut Schema| ConfigSchema::register_types(schema)))
    }

    /// java: @Override public void registerTypes(Schema schema, Map<String, Supplier<TypeTemplate>> entityTypes, Map<String, Supplier<TypeTemplate>> blockEntityTypes)
    pub fn register_types(schema: &mut Schema) {
        // rust: port gap - Java registers these three with `recursive = true`; the port's
        // `Schema.registerType(true, ..)` panics (no RecursiveTypeFamily). A `DSL.remainder()`
        // template references no type, so its recursive and non-recursive types are the same
        // `Named(name, remainder)`; registered non-recursively until the port folds recursive types.
        schema.register_type(false, config_data_fixer::CONFIG_TYPE.get(), Arc::new(|_: &Schema| dsl::remainder()));
        schema.register_type(false, config_data_fixer::CHAT_RULES_TYPE.get(), Arc::new(|_: &Schema| dsl::remainder()));
        schema.register_type(false, config_data_fixer::HUD_WIDGETS_TYPE.get(), Arc::new(|_: &Schema| dsl::remainder()));
    }

    /// java: @Override public Map<String, Supplier<TypeTemplate>> registerEntities(Schema schema)
    pub fn register_entities(_schema: &Schema) -> std::collections::HashMap<String, Arc<dyn Fn() -> TypeTemplate>> {
        return std::collections::HashMap::new();
    }

    /// java: @Override public Map<String, Supplier<TypeTemplate>> registerBlockEntities(Schema schema)
    pub fn register_block_entities(_schema: &Schema) -> std::collections::HashMap<String, Arc<dyn Fn() -> TypeTemplate>> {
        return std::collections::HashMap::new();
    }
}

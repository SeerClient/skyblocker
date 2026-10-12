//! Ported from de.hysky.skyblocker.config.datafixer.ConfigSchema by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::datafixer::config_data_fixer;
use mcrust::com::mojang::datafixers::dsl;
use mcrust::com::mojang::datafixers::schemas::schema::Schema;
use mcrust::com::mojang::datafixers::types::templates::type_template::TypeTemplate;

/// java: public class ConfigSchema extends Schema
#[derive(Clone)]
pub struct ConfigSchema {
    /// java: extends com.mojang.datafixers.schemas.Schema
    pub base: Arc<Schema>,
}

impl std::ops::Deref for ConfigSchema {
    type Target = Arc<Schema>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for ConfigSchema {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl ConfigSchema {
    /// java: public ConfigSchema(int versionKey, Schema parent)
    pub fn new(mut version_key: i32, mut parent: Arc<Schema>) -> Self {
        let mut this = Self {
            base: Schema::new(version_key, Some(parent.clone())),
        };
        this
    }

    /// java: @Override public void registerTypes(Schema schema, Map<String, Supplier<TypeTemplate>> entityTypes, Map<String, Supplier<TypeTemplate>> blockEntityTypes)
    pub fn register_types(&self, schema: &mut Arc<Schema>, entity_types: &std::collections::HashMap<String, Arc<dyn Fn() -> TypeTemplate>>, block_entity_types: &std::collections::HashMap<String, Arc<dyn Fn() -> TypeTemplate>>) {
        // TODO(fabric2seer): a lambda for a parameter the port does not type as a closure (`Supplier<com.mojang.datafixers.types.templates.TypeTemplate>`)
        // rust: schema.register_type(true, config_data_fixer::CONFIG_TYPE.get_mut(), || dsl::remainder());
        // TODO(fabric2seer): a lambda for a parameter the port does not type as a closure (`Supplier<com.mojang.datafixers.types.templates.TypeTemplate>`)
        // rust: schema.register_type(true, config_data_fixer::CHAT_RULES_TYPE.get_mut(), || dsl::remainder());
        // TODO(fabric2seer): a lambda for a parameter the port does not type as a closure (`Supplier<com.mojang.datafixers.types.templates.TypeTemplate>`)
        // rust: schema.register_type(true, config_data_fixer::HUD_WIDGETS_TYPE.get_mut(), || dsl::remainder());
    }

    /// java: @Override public Map<String, Supplier<TypeTemplate>> registerEntities(Schema schema)
    pub fn register_entities(&self, schema: &Arc<Schema>) -> std::collections::HashMap<String, Arc<dyn Fn() -> TypeTemplate>> {
        return std::collections::HashMap::from([]);
    }

    /// java: @Override public Map<String, Supplier<TypeTemplate>> registerBlockEntities(Schema schema)
    pub fn register_block_entities(&self, schema: &Arc<Schema>) -> std::collections::HashMap<String, Arc<dyn Fn() -> TypeTemplate>> {
        return std::collections::HashMap::from([]);
    }

}

// NOTE(fabric2seer): registerBlockEntities, registerEntities, registerTypes override(s) of Schema - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them

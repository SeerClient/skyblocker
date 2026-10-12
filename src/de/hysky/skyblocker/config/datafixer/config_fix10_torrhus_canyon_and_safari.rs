//! Ported from de.hysky.skyblocker.config.datafixer.ConfigFix10TorrhusCanyonAndSafari by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::datafixer::config_data_fix::ConfigDataFix;
use crate::de::hysky::skyblocker::config::datafixer::config_data_fix::ConfigDataFixBase;
use crate::de::hysky::skyblocker::config::datafixer::config_data_fixer;
use mcrust::com::mojang::datafixers::data_fix::{self, DataFix, DataFixFields};
use mcrust::com::mojang::datafixers::dsl;
use mcrust::com::mojang::datafixers::schemas::schema::Schema;
use mcrust::com::mojang::datafixers::type_rewrite_rule::{self, TypeRewriteRule};
use mcrust::com::mojang::datafixers::typed::Typed;
use mcrust::com::mojang::serialization::dynamic::Dynamic;
use mcrust::com::mojang::serialization::dynamic_like::DynamicLike as _;

/// java: public class ConfigFix10TorrhusCanyonAndSafari extends ConfigDataFix
pub struct ConfigFix10TorrhusCanyonAndSafari {
    /// java: extends ConfigDataFix
    pub base: ConfigDataFixBase,
    /// java: `this`, captured by the rule's lambdas (`this::fix`)
    self_ref: std::sync::Weak<ConfigFix10TorrhusCanyonAndSafari>,
}

impl ConfigFix10TorrhusCanyonAndSafari {
    /// java: public ConfigFix10TorrhusCanyonAndSafari(Schema outputSchema, boolean changesType)
    pub fn new(output_schema: Arc<Schema>, changes_type: bool) -> Arc<Self> {
        Arc::new_cyclic(|self_ref: &std::sync::Weak<ConfigFix10TorrhusCanyonAndSafari>| Self {
            base: ConfigDataFixBase::new(output_schema, changes_type),
            self_ref: self_ref.clone(),
        })
    }

    /// java: private <T> Dynamic<T> fix(Dynamic<T> dynamic)
    pub fn fix(&self, dynamic: &Dynamic) -> Dynamic {
        let mut dynamic = self.fix_version(dynamic);
        dynamic = dynamic.update("foraging", |foraging_dynamic| foraging_dynamic.rename_field("galatea", "moongladeMarsh"));
        dynamic = dynamic.update("hunting", |hunting_dynamic| hunting_dynamic.rename_field("huntingMobs", "moongladeMobs"));

        return dynamic;
    }
}

impl DataFix for ConfigFix10TorrhusCanyonAndSafari {
    fn data_fix(&self) -> &DataFixFields {
        &self.base.fields
    }

    /// java: @Override protected TypeRewriteRule makeRule()
    fn make_rule(&self) -> TypeRewriteRule {
        let this = self.self_ref.clone();
        return data_fix::fix_type_everywhere_typed(
            "ConfigFix10TorrhusCanyonAndSafari" /* java: this.getClass().getSimpleName() */,
            self.get_input_schema()
                .get_type(config_data_fixer::CONFIG_TYPE.get())
                .unwrap_or_else(|e| panic!("java.lang.IllegalArgumentException: {e}")),
            self.get_output_schema()
                .get_type(config_data_fixer::CONFIG_TYPE.get())
                .unwrap_or_else(|e| panic!("java.lang.IllegalArgumentException: {e}")),
            Arc::new(move |typed: Typed| {
                let Some(this) = this.upgrade() else { return Err("ConfigFix10TorrhusCanyonAndSafari: the fix was dropped".to_string()) };
                typed.update(&dsl::remainder_finder(), &|dynamic| this.fix(&dynamic))
            }),
        );
    }
}

impl ConfigDataFix for ConfigFix10TorrhusCanyonAndSafari {
    fn config_data_fix_base(&self) -> &ConfigDataFixBase {
        &self.base
    }
}

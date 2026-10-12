//! Ported from de.hysky.skyblocker.config.datafixer.ConfigFix8ItemList by fabric2seer.

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

/// java: public class ConfigFix8ItemList extends ConfigDataFix
pub struct ConfigFix8ItemList {
    /// java: extends ConfigDataFix
    pub base: ConfigDataFixBase,
    /// java: `this`, captured by the rule's lambdas (`this::fix`)
    self_ref: std::sync::Weak<ConfigFix8ItemList>,
}

impl ConfigFix8ItemList {
    /// java: public ConfigFix8ItemList(Schema outputSchema, boolean changesType)
    pub fn new(output_schema: Arc<Schema>, changes_type: bool) -> Arc<Self> {
        Arc::new_cyclic(|self_ref: &std::sync::Weak<ConfigFix8ItemList>| Self {
            base: ConfigDataFixBase::new(output_schema, changes_type),
            self_ref: self_ref.clone(),
        })
    }

    /// java: private Dynamic<?> fix(Dynamic<?> dynamic)
    pub fn fix(&self, dynamic: &Dynamic) -> Dynamic {
        return self.fix_version(dynamic).update("general", |general| {
            general.update("itemList", |item_list| item_list.set_field_if_present("enableRecipeBook", item_list.get("enableItemList").result()))
        });
    }
}

impl DataFix for ConfigFix8ItemList {
    fn data_fix(&self) -> &DataFixFields {
        &self.base.fields
    }

    /// java: @Override protected TypeRewriteRule makeRule()
    fn make_rule(&self) -> TypeRewriteRule {
        let this = self.self_ref.clone();
        return data_fix::fix_type_everywhere_typed_same(
            "ConfigFix8ItemList" /* java: getClass().getSimpleName() */,
            self.get_input_schema()
                .get_type(config_data_fixer::CONFIG_TYPE.get())
                .unwrap_or_else(|e| panic!("java.lang.IllegalArgumentException: {e}")),
            Arc::new(move |typed: Typed| {
                let Some(this) = this.upgrade() else { return Err("ConfigFix8ItemList: the fix was dropped".to_string()) };
                typed.update(&dsl::remainder_finder(), &|dynamic| this.fix(&dynamic))
            }),
        );
    }
}

impl ConfigDataFix for ConfigFix8ItemList {
    fn config_data_fix_base(&self) -> &ConfigDataFixBase {
        &self.base
    }
}

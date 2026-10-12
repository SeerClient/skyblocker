//! Ported from de.hysky.skyblocker.config.datafixer.ConfigFix7Farming by fabric2seer.

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

/// java: public class ConfigFix7Farming extends ConfigDataFix
pub struct ConfigFix7Farming {
    /// java: extends ConfigDataFix
    pub base: ConfigDataFixBase,
    /// java: `this`, captured by the rule's lambdas (`this::fix`)
    self_ref: std::sync::Weak<ConfigFix7Farming>,
}

impl ConfigFix7Farming {
    /// java: public ConfigFix7Farming(Schema outputSchema, boolean changesType)
    pub fn new(output_schema: Arc<Schema>, changes_type: bool) -> Arc<Self> {
        Arc::new_cyclic(|self_ref: &std::sync::Weak<ConfigFix7Farming>| Self {
            base: ConfigDataFixBase::new(output_schema, changes_type),
            self_ref: self_ref.clone(),
        })
    }

    /// java: private Dynamic<?> fix(Dynamic<?> dynamic)
    pub fn fix(&self, dynamic: &Dynamic) -> Dynamic {
        return self.fix_version(dynamic).update("farming", |farming| {
            farming
                .set(
                    "farmingHud",
                    &dynamic
                        .empty_map()
                        .set_field_if_present("enabled", farming.get("garden").get("farmingHud").get("enableHud").result())
                        .set_field_if_present("type", farming.get("garden").get("farmingHud").get("type").result()),
                )
                .set(
                    "pestHighlighter",
                    &dynamic
                        .empty_map()
                        .set_field_if_present("enabled", farming.get("garden").get("pestHighlighter").result())
                        .set_field_if_present("vinylHighlighter", farming.get("garden").get("vinylHighlighter").result())
                        .set_field_if_present("enableStereoHarmonyHelperForContest", farming.get("garden").get("enableStereoHarmonyHelperForContest").result()),
                )
                .set(
                    "mouseLock",
                    &dynamic
                        .empty_map()
                        .set_field_if_present("lockMouseTool", farming.get("garden").get("lockMouseTool").result())
                        .set_field_if_present("lockMouseGroundOnly", farming.get("garden").get("lockMouseGroundOnly").result()),
                )
                .set(
                    "plotsWidget",
                    &dynamic
                        .empty_map()
                        .set_field_if_present("enabled", farming.get("garden").get("gardenPlotsWidget").result())
                        .set_field_if_present("closeScreenOnPlotClick", farming.get("garden").get("closeScreenOnPlotClick").result()),
                )
                .update("visitorHelper", |visitor_helper| {
                    visitor_helper
                        .rename_field("visitorHelper", "enabled")
                        .rename_field("visitorHelperGardenOnly", "showInGardenOnly")
                        .rename_field("showStacksInVisitorHelper", "showInStacks")
                })
                .remove("garden")
        });
    }
}

impl DataFix for ConfigFix7Farming {
    fn data_fix(&self) -> &DataFixFields {
        &self.base.fields
    }

    /// java: @Override protected TypeRewriteRule makeRule()
    fn make_rule(&self) -> TypeRewriteRule {
        let this = self.self_ref.clone();
        return data_fix::fix_type_everywhere_typed_same(
            "ConfigFix7Farming" /* java: getClass().getSimpleName() */,
            self.get_input_schema()
                .get_type(config_data_fixer::CONFIG_TYPE.get())
                .unwrap_or_else(|e| panic!("java.lang.IllegalArgumentException: {e}")),
            Arc::new(move |typed: Typed| {
                let Some(this) = this.upgrade() else { return Err("ConfigFix7Farming: the fix was dropped".to_string()) };
                typed.update(&dsl::remainder_finder(), &|dynamic| this.fix(&dynamic))
            }),
        );
    }
}

impl ConfigDataFix for ConfigFix7Farming {
    fn config_data_fix_base(&self) -> &ConfigDataFixBase {
        &self.base
    }
}

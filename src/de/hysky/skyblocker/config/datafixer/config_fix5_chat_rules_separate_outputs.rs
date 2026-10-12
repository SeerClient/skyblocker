//! Ported from de.hysky.skyblocker.config.datafixer.ConfigFix5ChatRulesSeparateOutputs by fabric2seer.

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
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;

/// java: public class ConfigFix5ChatRulesSeparateOutputs extends ConfigDataFix
pub struct ConfigFix5ChatRulesSeparateOutputs {
    /// java: extends ConfigDataFix
    pub base: ConfigDataFixBase,
    /// java: `this`, captured by the rule's lambdas (`this::fix`)
    self_ref: std::sync::Weak<ConfigFix5ChatRulesSeparateOutputs>,
}

impl ConfigFix5ChatRulesSeparateOutputs {
    /// java: public ConfigFix5ChatRulesSeparateOutputs(Schema outputSchema, boolean changesType)
    pub fn new(output_schema: Arc<Schema>, changes_type: bool) -> Arc<Self> {
        Arc::new_cyclic(|self_ref: &std::sync::Weak<ConfigFix5ChatRulesSeparateOutputs>| Self {
            base: ConfigDataFixBase::new(output_schema, changes_type),
            self_ref: self_ref.clone(),
        })
    }

    /// java: private <T> Dynamic<T> fix(Dynamic<T> dynamic)
    pub fn fix(&self, dynamic: &Dynamic) -> Dynamic {
        return self.fix_version(dynamic).update("rules", |rules| {
            rules.create_list(
                rules
                    .as_stream()
                    .into_iter()
                    .map(|rule| {
                        rule.rename_field("isPartialMatch", "partialMatch")
                            .rename_field("isRegex", "regex")
                            .rename_field("isIgnoreCase", "ignoreCase")
                            .rename_field("validLocations", "locations")
                            .rename_field("hideMessage", "hideOriginalMessage")
                            .rename_field("replaceMessage", "chatMessage")
                            .set_field_if_present(
                                "actionBarMessage",
                                rule.get("showActionBar").result().and_then(|dynamic1| {
                                    if dynamic1.as_boolean_or(false) { rule.get("replaceMessage").result() } else { None }
                                }),
                            )
                            .remove("showActionBar")
                            .set_field_if_present(
                                "announcementMessage",
                                rule.get("showAnnouncement").result().and_then(|dynamic1| {
                                    if dynamic1.as_boolean_or(false) {
                                        rule.get("replaceMessage").result().map(|message| {
                                            dynamic1.create_map(vec![
                                                (dynamic1.create_string("message"), message),
                                                (
                                                    dynamic1.create_string("displayDuration"),
                                                    dynamic1.create_long((SkyblockerConfigManager::get().chat.chat_rule_config.announcement_length as i64).wrapping_mul(50)),
                                                ),
                                            ])
                                        })
                                    } else {
                                        None
                                    }
                                }),
                            )
                            .remove("showAnnouncement")
                    })
                    .collect(),
            )
        });
    }
}

impl DataFix for ConfigFix5ChatRulesSeparateOutputs {
    fn data_fix(&self) -> &DataFixFields {
        &self.base.fields
    }

    /// java: @Override protected TypeRewriteRule makeRule()
    fn make_rule(&self) -> TypeRewriteRule {
        let this = self.self_ref.clone();
        return data_fix::fix_type_everywhere_typed_same(
            "ConfigFix5ChatRulesSeparateOutputs" /* java: getClass().getSimpleName() */,
            self.get_input_schema()
                .get_type(config_data_fixer::CHAT_RULES_TYPE.get())
                .unwrap_or_else(|e| panic!("java.lang.IllegalArgumentException: {e}")),
            Arc::new(move |typed: Typed| {
                let Some(this) = this.upgrade() else { return Err("ConfigFix5ChatRulesSeparateOutputs: the fix was dropped".to_string()) };
                typed.update(&dsl::remainder_finder(), &|dynamic| this.fix(&dynamic))
            }),
        );
    }
}

impl ConfigDataFix for ConfigFix5ChatRulesSeparateOutputs {
    fn config_data_fix_base(&self) -> &ConfigDataFixBase {
        &self.base
    }
}

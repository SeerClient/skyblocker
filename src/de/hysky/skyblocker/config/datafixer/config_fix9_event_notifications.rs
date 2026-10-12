//! Ported from de.hysky.skyblocker.config.datafixer.ConfigFix9EventNotifications by fabric2seer.

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

/// java: public class ConfigFix9EventNotifications extends ConfigDataFix
pub struct ConfigFix9EventNotifications {
    /// java: extends ConfigDataFix
    pub base: ConfigDataFixBase,
    /// java: `this`, captured by the rule's lambdas (`this::fix`)
    self_ref: std::sync::Weak<ConfigFix9EventNotifications>,
}

impl ConfigFix9EventNotifications {
    /// java: public ConfigFix9EventNotifications(Schema outputSchema, boolean changesType)
    pub fn new(output_schema: Arc<Schema>, changes_type: bool) -> Arc<Self> {
        Arc::new_cyclic(|self_ref: &std::sync::Weak<ConfigFix9EventNotifications>| Self {
            base: ConfigDataFixBase::new(output_schema, changes_type),
            self_ref: self_ref.clone(),
        })
    }

    /// java: private <T> Dynamic<T> fix(Dynamic<T> dynamic)
    pub fn fix(&self, dynamic: &Dynamic) -> Dynamic {
        return self.fix_version(dynamic).update("eventNotifications", |event_notifications| {
            event_notifications.rename_and_fix_field("eventsReminderTimes", "events", |events| {
                events.update_map_values(|pair| {
                    // java: Pair.of(pair.getFirst(), dynamic.createMap(Map.of(..)))
                    let enabled = dynamic.create_boolean(!pair.1.as_list(|d| d).is_empty());
                    let value = dynamic.create_map(vec![
                        (dynamic.create_string("enabled"), enabled),
                        (dynamic.create_string("reminderTimes"), pair.1.clone()),
                    ]);
                    (pair.0, value)
                })
            })
        });
    }
}

impl DataFix for ConfigFix9EventNotifications {
    fn data_fix(&self) -> &DataFixFields {
        &self.base.fields
    }

    /// java: @Override protected TypeRewriteRule makeRule()
    fn make_rule(&self) -> TypeRewriteRule {
        let this = self.self_ref.clone();
        return data_fix::fix_type_everywhere_typed_same(
            "ConfigFix9EventNotifications" /* java: getClass().getSimpleName() */,
            self.get_input_schema()
                .get_type(config_data_fixer::CONFIG_TYPE.get())
                .unwrap_or_else(|e| panic!("java.lang.IllegalArgumentException: {e}")),
            Arc::new(move |typed: Typed| {
                let Some(this) = this.upgrade() else { return Err("ConfigFix9EventNotifications: the fix was dropped".to_string()) };
                typed.update(&dsl::remainder_finder(), &|dynamic| this.fix(&dynamic))
            }),
        );
    }
}

impl ConfigDataFix for ConfigFix9EventNotifications {
    fn config_data_fix_base(&self) -> &ConfigDataFixBase {
        &self.base
    }
}

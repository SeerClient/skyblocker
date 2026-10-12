//! Ported from de.hysky.skyblocker.config.datafixer.ConfigFix2QuickNav by fabric2seer.

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
use mcrust::net::minecraft::resources::identifier::Identifier;

/// java: public class ConfigFix2QuickNav extends ConfigDataFix
pub struct ConfigFix2QuickNav {
    /// java: extends ConfigDataFix
    pub base: ConfigDataFixBase,
    /// java: `this`, captured by the rule's lambdas (`this::fix`)
    self_ref: std::sync::Weak<ConfigFix2QuickNav>,
}

impl ConfigFix2QuickNav {
    /// java: public ConfigFix2QuickNav(Schema outputSchema, boolean changesType)
    pub fn new(output_schema: Arc<Schema>, changes_type: bool) -> Arc<Self> {
        Arc::new_cyclic(|self_ref: &std::sync::Weak<ConfigFix2QuickNav>| Self {
            base: ConfigDataFixBase::new(output_schema, changes_type),
            self_ref: self_ref.clone(),
        })
    }

    /// java: private <T> Dynamic<T> fix(Dynamic<T> dynamic)
    pub fn fix(&self, dynamic: &Dynamic) -> Dynamic {
        return self.fix_version(dynamic).update("quickNav", |quick_nav| {
            quick_nav
                .rename_field("button12", "button13")
                .rename_field("button11", "button12")
                .rename_field("button10", "button11")
                .rename_field("button9", "button10")
                .rename_field("button8", "button9")
                .rename_field("button7", "button8")
                .update_map_values(|button| {
                    // java: button.getFirst().asString().getOrThrow() - throws on a non-string key
                    if button.0.as_string().get_or_throw().unwrap_or_else(|e| panic!("java.lang.IllegalStateException: {e}")).starts_with("button") {
                        // java: button.mapSecond(this::fixButton)
                        let second = self.fix_button(&button.1);
                        (button.0, second)
                    } else {
                        button
                    }
                })
        });
    }

    /// java: private <T> Dynamic<T> fixButton(Dynamic<T> button)
    pub fn fix_button(&self, button: &Dynamic) -> Dynamic {
        return button.rename_and_fix_field("item", "itemData", |item_data| {
            item_data.rename_and_fix_field("id", "item", |id| {
                id.create_string(&Identifier::parse(&id.as_string().get_or_throw().unwrap_or_else(|e| panic!("java.lang.IllegalStateException: {e}"))).to_string())
            })
        });
    }
}

impl DataFix for ConfigFix2QuickNav {
    fn data_fix(&self) -> &DataFixFields {
        &self.base.fields
    }

    /// java: @Override protected TypeRewriteRule makeRule()
    fn make_rule(&self) -> TypeRewriteRule {
        let this = self.self_ref.clone();
        return data_fix::fix_type_everywhere_typed_same(
            "ConfigFix2QuickNav",
            self.get_input_schema()
                .get_type(config_data_fixer::CONFIG_TYPE.get())
                .unwrap_or_else(|e| panic!("java.lang.IllegalArgumentException: {e}")),
            Arc::new(move |typed: Typed| {
                let Some(this) = this.upgrade() else { return Err("ConfigFix2QuickNav: the fix was dropped".to_string()) };
                typed.update(&dsl::remainder_finder(), &|dynamic| this.fix(&dynamic))
            }),
        );
    }
}

impl ConfigDataFix for ConfigFix2QuickNav {
    fn config_data_fix_base(&self) -> &ConfigDataFixBase {
        &self.base
    }
}

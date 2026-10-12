//! Ported from de.hysky.skyblocker.config.datafixer.ConfigFix3AnimatedDyeAndItemBackground by fabric2seer.

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
use mcrust::net::minecraft::util::argb;

/// java: public class ConfigFix3AnimatedDyeAndItemBackground extends ConfigDataFix
pub struct ConfigFix3AnimatedDyeAndItemBackground {
    /// java: extends ConfigDataFix
    pub base: ConfigDataFixBase,
    /// java: `this`, captured by the rule's lambdas (`this::fix`)
    self_ref: std::sync::Weak<ConfigFix3AnimatedDyeAndItemBackground>,
}

impl ConfigFix3AnimatedDyeAndItemBackground {
    /// java: public ConfigFix3AnimatedDyeAndItemBackground(Schema outputSchema, boolean changesType)
    pub fn new(output_schema: Arc<Schema>, changes_type: bool) -> Arc<Self> {
        Arc::new_cyclic(|self_ref: &std::sync::Weak<ConfigFix3AnimatedDyeAndItemBackground>| Self {
            base: ConfigDataFixBase::new(output_schema, changes_type),
            self_ref: self_ref.clone(),
        })
    }

    /// java: private <T> Dynamic<T> fix(Dynamic<T> dynamic)
    pub fn fix(&self, dynamic: &Dynamic) -> Dynamic {
        return self.fix_version(dynamic).update("general", |general| {
            general
                .update("customAnimatedDyes", |custom_animated_dyes| {
                    // java: customAnimatedDye.mapSecond(this::fixCustomAnimatedDye)
                    custom_animated_dyes.update_map_values(|custom_animated_dye| {
                        let second = self.fix_custom_animated_dye(&custom_animated_dye.1);
                        (custom_animated_dye.0, second)
                    })
                })
                .update("itemInfoDisplay", |item_info_display| {
                    item_info_display
                        .rename_field("itemRarityBackgroundStyle", "itemBackgroundStyle")
                        .rename_field("itemRarityBackgroundsOpacity", "itemBackgroundOpacity")
                })
        });
    }

    /// java: private <T> Dynamic<T> fixCustomAnimatedDye(Dynamic<T> customAnimatedDye)
    pub fn fix_custom_animated_dye(&self, custom_animated_dye: &Dynamic) -> Dynamic {
        return custom_animated_dye
            .set(
                "keyframes",
                &custom_animated_dye.create_list(vec![
                    custom_animated_dye
                        .empty_map()
                        .set("color", &custom_animated_dye.create_int(argb::opaque(custom_animated_dye.get("color1").as_int(0))))
                        .set("time", &custom_animated_dye.create_float(0.0)),
                    custom_animated_dye
                        .empty_map()
                        .set("color", &custom_animated_dye.create_int(argb::opaque(custom_animated_dye.get("color2").as_int(0))))
                        .set("time", &custom_animated_dye.create_float(1.0)),
                ]),
            )
            .remove("color1")
            .remove("color2")
            // Samples is how many steps the animation has, and tickDelay is how long each step takes in ticks.
            .set(
                "duration",
                &custom_animated_dye.create_float(
                    custom_animated_dye.get("samples").as_int(20).wrapping_mul(custom_animated_dye.get("tickDelay").as_int(1)) as f32 / 20.0f32,
                ),
            )
            .remove("samples")
            .remove("tickDelay");
    }
}

impl DataFix for ConfigFix3AnimatedDyeAndItemBackground {
    fn data_fix(&self) -> &DataFixFields {
        &self.base.fields
    }

    /// java: @Override protected TypeRewriteRule makeRule()
    fn make_rule(&self) -> TypeRewriteRule {
        let this = self.self_ref.clone();
        return data_fix::fix_type_everywhere_typed_same(
            "ConfigFix3AnimatedDyeAndItemBackground",
            self.get_input_schema()
                .get_type(config_data_fixer::CONFIG_TYPE.get())
                .unwrap_or_else(|e| panic!("java.lang.IllegalArgumentException: {e}")),
            Arc::new(move |typed: Typed| {
                let Some(this) = this.upgrade() else { return Err("ConfigFix3AnimatedDyeAndItemBackground: the fix was dropped".to_string()) };
                typed.update(&dsl::remainder_finder(), &|dynamic| this.fix(&dynamic))
            }),
        );
    }
}

impl ConfigDataFix for ConfigFix3AnimatedDyeAndItemBackground {
    fn config_data_fix_base(&self) -> &ConfigDataFixBase {
        &self.base
    }
}

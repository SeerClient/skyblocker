//! Ported from de.hysky.skyblocker.config.datafixer.ConfigFix7Farming by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::datafixer::config_data_fix::ConfigDataFix;
use crate::de::hysky::skyblocker::config::datafixer::config_data_fix::ConfigDataFixBase;
use mcrust::com::mojang::datafixers::schemas::schema::Schema;
use mcrust::com::mojang::datafixers::type_rewrite_rule::TypeRewriteRule;
use mcrust::com::mojang::serialization::dynamic::Dynamic;
use mcrust::com::mojang::serialization::dynamic_like::DynamicLike as _;

/// java: public class ConfigFix7Farming extends ConfigDataFix
pub struct ConfigFix7Farming {
    /// java: extends ConfigDataFix
    pub base: ConfigDataFixBase,
}

impl std::ops::Deref for ConfigFix7Farming {
    type Target = ConfigDataFixBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for ConfigFix7Farming {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl ConfigFix7Farming {
    /// java: public ConfigFix7Farming(Schema outputSchema, boolean changesType)
    pub fn new(mut output_schema: Arc<Schema>, mut changes_type: bool) -> compat::Shared<Self> {
        let (_, __this) = ConfigDataFixBase::new_in(output_schema.clone(), changes_type, move |__base| {
            let __h = compat::shared(Self {
                base: __base,
            });
            ((__h.clone() as compat::Shared<dyn ConfigDataFix>), __h)
        });
        __this
    }

    /// java: @Override protected TypeRewriteRule makeRule()
    pub fn make_rule(__this: &compat::Shared<Self>) -> TypeRewriteRule {
        // TODO(fabric2seer): `.getSimpleName(..)` on a value of unknown type
        //   java: return fixTypeEverywhereTyped(
        //   java: 				getClass().getSimpleName(),
        //   java: 				getInputSchema().getType(ConfigDataFixer.CONFIG_TYPE),
        //   java: 				typed -> typed.update(DSL.remainderFinder(), this::fix)
        //   java: 		);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: private Dynamic<?> fix(Dynamic<?> dynamic)
    pub fn fix(__this: &compat::Shared<Self>, dynamic: &mut Dynamic) -> Dynamic {
        return { let (__q0, __q1,) = ("farming", |farming| (farming.set("farmingHud", &(dynamic.empty_map().set_field_if_present("enabled", farming.get("garden").get("farmingHud").get("enableHud").result())).set_field_if_present("type", farming.get("garden").get("farmingHud").get("type").result()))).set("pestHighlighter", &(dynamic.empty_map().set_field_if_present("enabled", farming.get("garden").get("pestHighlighter").result())).set_field_if_present("vinylHighlighter", farming.get("garden").get("vinylHighlighter").result()).set_field_if_present("enableStereoHarmonyHelperForContest", farming.get("garden").get("enableStereoHarmonyHelperForContest").result())).set("mouseLock", &(dynamic.empty_map().set_field_if_present("lockMouseTool", farming.get("garden").get("lockMouseTool").result())).set_field_if_present("lockMouseGroundOnly", farming.get("garden").get("lockMouseGroundOnly").result())).set("plotsWidget", &(dynamic.empty_map().set_field_if_present("enabled", farming.get("garden").get("gardenPlotsWidget").result())).set_field_if_present("closeScreenOnPlotClick", farming.get("garden").get("closeScreenOnPlotClick").result())).update("visitorHelper", |visitor_helper| (visitor_helper.rename_field("visitorHelper", "enabled")).rename_field("visitorHelperGardenOnly", "showInGardenOnly").rename_field("showStacksInVisitorHelper", "showInStacks")).remove("garden"),); __this.lock().borrow().fix_version(dynamic).update(__q0, __q1) };
    }

}

impl ConfigDataFix for ConfigFix7Farming {
    fn config_data_fix_base(&self) -> &ConfigDataFixBase {
        &self.base
    }
    fn config_data_fix_base_mut(&mut self) -> &mut ConfigDataFixBase {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}

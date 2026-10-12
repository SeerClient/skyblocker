//! Ported from de.hysky.skyblocker.config.datafixer.ConfigFix6BuildersWandConfig by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::datafixer::config_data_fix::ConfigDataFix;
use crate::de::hysky::skyblocker::config::datafixer::config_data_fix::ConfigDataFixBase;
use crate::de::hysky::skyblocker::config::datafixer::config_data_fixer;
use mcrust::com::mojang::datafixers::data_fix;
use mcrust::com::mojang::datafixers::dsl;
use mcrust::com::mojang::datafixers::schemas::schema::Schema;
use mcrust::com::mojang::datafixers::type_rewrite_rule::TypeRewriteRule;
use mcrust::com::mojang::serialization::dynamic::Dynamic;
use mcrust::com::mojang::datafixers::data_fix::DataFix as _;
use mcrust::com::mojang::serialization::dynamic_like::DynamicLike as _;

/// java: public class ConfigFix6BuildersWandConfig extends ConfigDataFix
pub struct ConfigFix6BuildersWandConfig {
    /// java: extends ConfigDataFix
    pub base: ConfigDataFixBase,
}

impl std::ops::Deref for ConfigFix6BuildersWandConfig {
    type Target = ConfigDataFixBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for ConfigFix6BuildersWandConfig {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl ConfigFix6BuildersWandConfig {
    /// java: public ConfigFix6BuildersWandConfig(Schema outputSchema, boolean changesType)
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
        let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        return data_fix::fix_type_everywhere_typed_same("ConfigFix6BuildersWandConfig", { let (__q0,) = (config_data_fixer::CONFIG_TYPE.get_mut(),); __this.lock().borrow().get_input_schema().get_type(__q0) }.unwrap(), Arc::new(move |typed| typed.update(&dsl::remainder_finder(), &|__a0| { let Some(__this_1) = __this_w1.get().upgrade() else { return Default::default(); }; ConfigFix6BuildersWandConfig::fix((&__this_1), &mut __a0) }).unwrap()));
    }

    /// java: private <T> Dynamic<T> fix(Dynamic<T> dynamic)
    pub fn fix<T: 'static>(__this: &compat::Shared<Self>, dynamic: &mut Dynamic) -> Dynamic {
        return { let (__q0, __q1,) = ("helpers", |helpers| helpers.set("buildersWand", &dynamic.empty_map().set_field_if_present("enableBuildersWandPreview", helpers.get("enableBuildersWandPreview").result())),); __this.lock().borrow().fix_version(dynamic).update(__q0, __q1) };
    }

}

impl ConfigDataFix for ConfigFix6BuildersWandConfig {
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

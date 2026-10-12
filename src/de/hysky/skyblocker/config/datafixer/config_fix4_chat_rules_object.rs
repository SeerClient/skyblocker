//! Ported from de.hysky.skyblocker.config.datafixer.ConfigFix4ChatRulesObject by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::datafixer::config_data_fix::ConfigDataFix;
use crate::de::hysky::skyblocker::config::datafixer::config_data_fix::ConfigDataFixBase;
use mcrust::com::mojang::datafixers::schemas::schema::Schema;
use mcrust::com::mojang::datafixers::type_rewrite_rule::TypeRewriteRule;
use mcrust::com::mojang::serialization::dynamic::Dynamic;
use mcrust::com::mojang::serialization::dynamic_like::DynamicLike as _;

/// java: public class ConfigFix4ChatRulesObject extends ConfigDataFix
pub struct ConfigFix4ChatRulesObject {
    /// java: extends ConfigDataFix
    pub base: ConfigDataFixBase,
}

impl std::ops::Deref for ConfigFix4ChatRulesObject {
    type Target = ConfigDataFixBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for ConfigFix4ChatRulesObject {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl ConfigFix4ChatRulesObject {
    /// java: public ConfigFix4ChatRulesObject(Schema outputSchema, boolean changesType)
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
        //   java: 				getInputSchema().getType(ConfigDataFixer.CHAT_RULES_TYPE),
        //   java: 				typed -> typed.update(DSL.remainderFinder(), this::fix)
        //   java: 		);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: private <T> Dynamic<T> fix(Dynamic<T> dynamic)
    pub fn fix<T: 'static>(__this: &compat::Shared<Self>, mut dynamic: Dynamic) -> Dynamic {
        if dynamic.as_stream_opt().result().is_some() {
            dynamic = dynamic.empty_map().set("rules", &dynamic);
        }
        return __this.lock().borrow().fix_version(&mut dynamic);
    }

}

impl ConfigDataFix for ConfigFix4ChatRulesObject {
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

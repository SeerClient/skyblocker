//! Ported from de.hysky.skyblocker.config.datafixer.ConfigFix5ChatRulesSeparateOutputs by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::datafixer::config_data_fix::ConfigDataFix;
use crate::de::hysky::skyblocker::config::datafixer::config_data_fix::ConfigDataFixBase;
use mcrust::com::mojang::datafixers::schemas::schema::Schema;
use mcrust::com::mojang::datafixers::type_rewrite_rule::TypeRewriteRule;
use mcrust::com::mojang::serialization::dynamic::Dynamic;
use mcrust::com::mojang::serialization::dynamic_like::DynamicLike as _;

/// java: public class ConfigFix5ChatRulesSeparateOutputs extends ConfigDataFix
pub struct ConfigFix5ChatRulesSeparateOutputs {
    /// java: extends ConfigDataFix
    pub base: ConfigDataFixBase,
}

impl std::ops::Deref for ConfigFix5ChatRulesSeparateOutputs {
    type Target = ConfigDataFixBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for ConfigFix5ChatRulesSeparateOutputs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl ConfigFix5ChatRulesSeparateOutputs {
    /// java: public ConfigFix5ChatRulesSeparateOutputs(Schema outputSchema, boolean changesType)
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
    pub fn fix<T: 'static>(__this: &compat::Shared<Self>, dynamic: &mut Dynamic) -> Dynamic {
        // TODO(fabric2seer): Optional.asBoolean
        //   java: return fixVersion(dynamic).update("rules", rules -> rules.createList(rules.asStream().map(rule -> rule
        //   java: 				.renameField("isPartialMatch", "partialMatch")
        //   java: 				.renameField("isRegex", "regex")
        //   java: 				.renameField("isIgnoreCase", "ignoreCase")
        //   java: 				.renameField("validLocations", "locations")
        //   java: 				.renameField("hideMessage", "hideOriginalMessage")
        //   java: 				.renameField("replaceMessage", "chatMessage")
        //   java: 				.setFieldIfPresent("actionBarMessage", rule.get("showActionBar").result().flatMap(dynamic1 ->
        //   java: 					dynamic1.asBoolean(false) ? rule.get("replaceMessage").result() : Optional.empty()
        //   java: 				))
        //   java: 				.remove("showActionBar")
        //   java: 				.setFieldIfPresent("announcementMessage", rule.get("showAnnouncement").result().flatMap(dynamic1 ->
        //   java: 					dynamic1.asBoolean(false) ? rule.get("replaceMessage").result().map(message -> dynamic1.createMap(
        //   java: 							Map.of(dynamic1.createString("message"), message, dynamic1.createString("displayDuration"), dynamic1.createLong(SkyblockerConfigManager.get().chat.chatRuleConfig.announcementLength * 50L))
        //   java: 					)) : Optional.empty()
        //   java: 				))
        //   java: 				.remove("showAnnouncement")
        //   java: 		)));
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}

impl ConfigDataFix for ConfigFix5ChatRulesSeparateOutputs {
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

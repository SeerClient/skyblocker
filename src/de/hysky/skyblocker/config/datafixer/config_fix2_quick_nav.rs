//! Ported from de.hysky.skyblocker.config.datafixer.ConfigFix2QuickNav by fabric2seer.

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
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::com::mojang::datafixers::data_fix::DataFix as _;
use mcrust::com::mojang::serialization::dynamic_like::DynamicLike as _;

/// java: public class ConfigFix2QuickNav extends ConfigDataFix
pub struct ConfigFix2QuickNav {
    /// java: extends ConfigDataFix
    pub base: ConfigDataFixBase,
}

impl std::ops::Deref for ConfigFix2QuickNav {
    type Target = ConfigDataFixBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for ConfigFix2QuickNav {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl ConfigFix2QuickNav {
    /// java: public ConfigFix2QuickNav(Schema outputSchema, boolean changesType)
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
        return data_fix::fix_type_everywhere_typed_same("ConfigFix2QuickNav", { let (__q0,) = (config_data_fixer::CONFIG_TYPE.get_mut(),); __this.lock().borrow().get_input_schema().get_type(__q0) }.unwrap(), Arc::new(move |typed| typed.update(&dsl::remainder_finder(), &|__a0| { let Some(__this_1) = __this_w1.get().upgrade() else { return Default::default(); }; ConfigFix2QuickNav::fix((&__this_1), &mut __a0) }).unwrap()));
    }

    /// java: private <T> Dynamic<T> fix(Dynamic<T> dynamic)
    pub fn fix<T: 'static>(__this: &compat::Shared<Self>, dynamic: &mut Dynamic) -> Dynamic {
        // TODO(fabric2seer): Map.Entry.mapSecond
        //   java: return fixVersion(dynamic).update("quickNav", quickNav -> quickNav
        //   java: 				.renameField("button12", "button13")
        //   java: 				.renameField("button11", "button12")
        //   java: 				.renameField("button10", "button11")
        //   java: 				.renameField("button9", "button10")
        //   java: 				.renameField("button8", "button9")
        //   java: 				.renameField("button7", "button8")
        //   java: 				.updateMapValues(button -> button.getFirst().asString().getOrThrow().startsWith("button") ? button.mapSecond(this::fixButton) : button)
        //   java: 		);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: private <T> Dynamic<T> fixButton(Dynamic<T> button)
    pub fn fix_button<T: 'static>(__this: &compat::Shared<Self>, button: &Dynamic) -> Dynamic {
        // TODO(fabric2seer): `fmt` takes 1 argument(s) in the port (`f: &mut fmt::Formatter<'_>`), Java passes 0
        // rust: return button.rename_and_fix_field("item", "itemData", |item_data| item_data.rename_and_fix_field("id", "item", |id| id.create_string(&Identifier::parse(&id.as_string().get_or_throw().unwrap()).fmt())));
        return todo!("fabric2seer: see the TODO above");
    }

}

impl ConfigDataFix for ConfigFix2QuickNav {
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

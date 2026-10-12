//! Ported from de.hysky.skyblocker.config.datafixer.ConfigFix3AnimatedDyeAndItemBackground by fabric2seer.

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
use mcrust::net::minecraft::util::argb;
use mcrust::com::mojang::datafixers::data_fix::DataFix as _;
use mcrust::com::mojang::serialization::dynamic_like::DynamicLike as _;

/// java: public class ConfigFix3AnimatedDyeAndItemBackground extends ConfigDataFix
pub struct ConfigFix3AnimatedDyeAndItemBackground {
    /// java: extends ConfigDataFix
    pub base: ConfigDataFixBase,
}

impl std::ops::Deref for ConfigFix3AnimatedDyeAndItemBackground {
    type Target = ConfigDataFixBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for ConfigFix3AnimatedDyeAndItemBackground {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl ConfigFix3AnimatedDyeAndItemBackground {
    /// java: public ConfigFix3AnimatedDyeAndItemBackground(Schema outputSchema, boolean changesType)
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
        return data_fix::fix_type_everywhere_typed_same("ConfigFix3AnimatedDyeAndItemBackground", { let (__q0,) = (config_data_fixer::CONFIG_TYPE.get_mut(),); __this.lock().borrow().get_input_schema().get_type(__q0) }.unwrap(), Arc::new(move |typed| typed.update(&dsl::remainder_finder(), &|__a0| { let Some(__this_1) = __this_w1.get().upgrade() else { return Default::default(); }; ConfigFix3AnimatedDyeAndItemBackground::fix((&__this_1), &mut __a0) }).unwrap()));
    }

    /// java: private <T> Dynamic<T> fix(Dynamic<T> dynamic)
    pub fn fix<T: 'static>(__this: &compat::Shared<Self>, dynamic: &mut Dynamic) -> Dynamic {
        // TODO(fabric2seer): Map.Entry.mapSecond
        //   java: return fixVersion(dynamic).update("general", general -> general
        //   java: 				.update("customAnimatedDyes", customAnimatedDyes -> customAnimatedDyes
        //   java: 						.updateMapValues(customAnimatedDye -> customAnimatedDye.mapSecond(this::fixCustomAnimatedDye))
        //   java: 				)
        //   java: 				.update("itemInfoDisplay", itemInfoDisplay -> itemInfoDisplay
        //   java: 						.renameField("itemRarityBackgroundStyle", "itemBackgroundStyle")
        //   java: 						.renameField("itemRarityBackgroundsOpacity", "itemBackgroundOpacity")
        //   java: 				)
        //   java: 		);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: private <T> Dynamic<T> fixCustomAnimatedDye(Dynamic<T> customAnimatedDye)
    pub fn fix_custom_animated_dye<T: 'static>(__this: &compat::Shared<Self>, custom_animated_dye: &mut Dynamic) -> Dynamic {
        // TODO(fabric2seer): `Dynamic.createList` has no Rust method in the port (looked for `create_list` in Dynamic and its supertypes)
        // rust: return (((((custom_animated_dye.set("keyframes", &custom_animated_dye.create_list(&vec![(custom_animated_dye.empty_map().set("color", &custom_animated_dye.create_int(argb::opaque(custom_animated_dye.get("color1").as_int(0))))).set("time", &custom_animated_dye.create_float((0) as f32)), (custom_animated_dye.empty_map().set("color", &custom_animated_dye.create_int(argb::opaque(custom_animated_dye.get("color2").as_int(0))))).set("time", &custom_animated_dye.create_float((1) as f32))].into_iter()))).remove("color1")).remove("color2")).set("duration", &custom_animated_dye.create_float((((custom_animated_dye.get("samples").as_int(20)).wrapping_mul(custom_animated_dye.get("tickDelay").as_int(1)) as f32) / 20.0f32)))).remove("samples")).remove("tickDelay");
        return todo!("fabric2seer: see the TODO above");
    }

}

impl ConfigDataFix for ConfigFix3AnimatedDyeAndItemBackground {
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

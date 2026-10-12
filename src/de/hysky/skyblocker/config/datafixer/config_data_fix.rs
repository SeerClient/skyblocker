//! Ported from de.hysky.skyblocker.config.datafixer.ConfigDataFix by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::datafixer::config_data_fixer;
use mcrust::com::mojang::datafixers::data_fix::DataFix;
use mcrust::com::mojang::datafixers::data_fix_utils;
use mcrust::com::mojang::datafixers::schemas::schema::Schema;
use mcrust::com::mojang::serialization::dynamic::Dynamic;
use mcrust::com::mojang::serialization::dynamic_like::DynamicLike as _;

/// java: public abstract class ConfigDataFix extends DataFix
pub struct ConfigDataFixBase {
    // TODO(fabric2seer): extends DataFix - a TRAIT in the port: implement it for ConfigDataFixBase instead of holding a base
}

impl ConfigDataFixBase {
    /// java: public ConfigDataFix(Schema outputSchema, boolean changesType) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_in<__R>(mut output_schema: Arc<Schema>, mut changes_type: bool, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn ConfigDataFix>, __R)) -> (compat::Shared<dyn ConfigDataFix>, __R) {
        let this = Self {
        };
        let (__h, __r) = __wrap(this);
        let __this = __h;
        (__this, __r)
    }

}

/// java: the virtual methods of ConfigDataFix (implemented by ConfigDataFixBase and every subclass)
pub trait ConfigDataFix: compat::DynAny {
    fn config_data_fix_base(&self) -> &ConfigDataFixBase;
    fn config_data_fix_base_mut(&mut self) -> &mut ConfigDataFixBase;
    fn as_any(&self) -> &dyn std::any::Any;

    /// java: protected <T> Dynamic<T> fixVersion(Dynamic<T> dynamic)
    fn fix_version<T: 'static>(&self, dynamic: &mut Dynamic) -> Dynamic where Self: Sized {
        return dynamic.set(config_data_fixer::VERSION_KEY, &dynamic.create_int(data_fix_utils::get_version(self.config_data_fix_base().get_version_key())));
    }

}

//! Ported from de.hysky.skyblocker.config.datafixer.ConfigDataFix by fabric2seer.
//!
//! `extends DataFix`: the port's `DataFix` is a TRAIT over the abstract class's fields
//! (`DataFixFields`), so the base holds those fields and every subclass implements `DataFix`
//! (its `makeRule`) and this trait. A fix is handed to `DataFixerBuilder.addFixer` as an
//! `Arc<dyn DataFix>`, so each subclass lives behind an `Arc` (built with `Arc::new_cyclic`; the
//! rule's lambdas, stored in the fix's own rule cache, capture its `Weak`).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::datafixer::config_data_fixer;
use mcrust::com::mojang::datafixers::data_fix::{DataFix, DataFixFields};
use mcrust::com::mojang::datafixers::data_fix_utils;
use mcrust::com::mojang::datafixers::schemas::schema::Schema;
use mcrust::com::mojang::serialization::dynamic::Dynamic;
use mcrust::com::mojang::serialization::dynamic_like::DynamicLike as _;

/// java: public abstract class ConfigDataFix extends DataFix
pub struct ConfigDataFixBase {
    /// java: extends com.mojang.datafixers.DataFix - its fields (outputSchema, changesType, rule)
    pub fields: DataFixFields,
}

impl ConfigDataFixBase {
    /// java: public ConfigDataFix(Schema outputSchema, boolean changesType)
    pub fn new(output_schema: Arc<Schema>, changes_type: bool) -> Self {
        Self { fields: DataFixFields::new(output_schema, changes_type) }
    }
}

/// java: the virtual methods of ConfigDataFix (implemented by every subclass)
pub trait ConfigDataFix: DataFix {
    fn config_data_fix_base(&self) -> &ConfigDataFixBase;

    /// java: protected <T> Dynamic<T> fixVersion(Dynamic<T> dynamic)
    fn fix_version(&self, dynamic: &Dynamic) -> Dynamic {
        dynamic.set(config_data_fixer::VERSION_KEY, &dynamic.create_int(data_fix_utils::get_version(self.get_version_key())))
    }
}

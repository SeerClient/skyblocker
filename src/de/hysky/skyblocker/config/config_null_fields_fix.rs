//! Ported from de.hysky.skyblocker.config.ConfigNullFieldsFix by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config::SkyblockerConfig;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;

/// java: public class ConfigNullFieldsFix
#[derive(Clone)]
pub struct ConfigNullFieldsFix {}

impl ConfigNullFieldsFix {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static void init()
    pub fn init() {
        SkyblockerConfigManager::update(Arc::new(move |mut config| {
            // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
            let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                ConfigNullFieldsFix::fix_null_fields(config, &SkyblockerConfig::new());
            }));
            if let Err(e) = __result {
                logger().error(&format!("[Skyblocker Config Null Fields Fixer] Failed to ensure that the config has no null fields! You may encounter crashes :("));
            }
        }));
    }


    /// java: private static void fixNullFields(Object config, Object defaultConfig) throws Exception
    ///
    /// The generated config classes answer reflection through their `ClassOf`: `to_gson` is every declared field with
    /// its value (`serializeNulls` - a null field is a JSON `null`) and `read_gson` builds the object back from those
    /// fields. `field.set(config, defaultValue)` on each null field is the same field set to the default's value in
    /// that reflected form, and the object rebuilt from it.
    pub fn fix_null_fields(config: &mut SkyblockerConfig, default_config: &SkyblockerConfig) {
        let class = SkyblockerConfig::__class_of();
        let mut config_value: dandelion::compat::GsonValue = (class.to_gson)(config);
        let default_value: dandelion::compat::GsonValue = (class.to_gson)(default_config);
        if !ConfigNullFieldsFix::fix_null_fields_in(&mut config_value, &default_value) {
            return;
        }
        let json: serde_json::Value = serde_json::from_str(&dandelion::compat::gson_text(&config_value, false)).unwrap_or_else(|e| panic!("JsonSyntaxException: {e}"));
        *config = (class.read_gson)(&json).unwrap_or_else(|| panic!("IllegalStateException: the fixed config does not read back"));
    }

    /// The field walk of `fixNullFields` over the reflected form; `true` when a field was set.
    fn fix_null_fields_in(config: &mut dandelion::compat::GsonValue, default_config: &dandelion::compat::GsonValue) -> bool {
        use dandelion::compat::GsonValue;
        let (GsonValue::Obj(fields), GsonValue::Obj(default_fields)) = (config, default_config) else {
            return false;
        };
        let mut changed = false;
        //   java: for (Field field : config.getClass().getDeclaredFields()) {
        for (name, config_field_value) in fields.iter_mut() {
            //   java: Object configValue = field.get(config); Object defaultValue = field.get(defaultConfig);
            let Some((_, default_field_value)) = default_fields.iter().find(|(key, _)| key == name) else { continue };
            let config_is_null = *config_field_value == GsonValue::Null;
            let default_is_null = *default_field_value == GsonValue::Null;
            //   java: if (configValue == null && defaultValue != null) field.set(config, defaultValue);
            if config_is_null && !default_is_null {
                *config_field_value = default_field_value.clone();
                changed = true;
            //   java: else if (configValue != null && defaultValue != null && SkyblockerConfigManager.isConfigClass(field.getType())) fixNullFields(configValue, defaultValue);
            // (a nested config class is a nested object in the reflected form)
            } else if !config_is_null && !default_is_null && matches!(config_field_value, GsonValue::Obj(_)) {
                changed |= ConfigNullFieldsFix::fix_null_fields_in(config_field_value, default_field_value);
            }
        }
        changed
    }

}

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
    /// `Object` parameters: the objects themselves (`&dyn Any`).
    /// rust: port gap - the body walks `getDeclaredFields()` (Java reflection), which Rust has no counterpart for (GAPS_batchG).
    pub fn fix_null_fields(config: &mut dyn std::any::Any, default_config: &dyn std::any::Any) {
        // TODO(fabric2seer): `.getDeclaredFields(..)` on a Class
        //   java: for (Field field : config.getClass().getDeclaredFields()) {
        //   java: 			field.setAccessible(true);
        //   java:
        //   java: 			Object configValue = field.get(config);
        //   java: 			Object defaultValue = field.get(defaultConfig);
        //   java:
        //   java: 			if (configValue == null && defaultValue != null) {
        //   java: 				field.set(config, defaultValue);
        //   java: 			} else if (configValue != null && defaultValue != null && SkyblockerConfigManager.isConfigClass(field.getType())) {
        //   java: 				fixNullFields(configValue, defaultValue);
        //   java: 			}
        //   java: 		}
    }

}

//! Ported from de.hysky.skyblocker.config.categories.MiscCategory by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config::SkyblockerConfig;

/// java: public class MiscCategory
#[derive(Clone)]
pub struct MiscCategory {}

impl MiscCategory {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static ConfigCategory create(SkyblockerConfig defaults, SkyblockerConfig config)
    pub fn create(defaults: &SkyblockerConfig, config: &SkyblockerConfig) -> crate::unresolved::ConfigCategory {
        // TODO(fabric2seer): static call ConfigCategory.createBuilder(..)
        //   java: return ConfigCategory.createBuilder()
        //   java: 				.id(SkyblockerMod.id("config/misc"))
        //   java: 				.name(Component.translatable("skyblocker.config.misc"))
        //   java:
        //   java: 				//Uncategorized Options
        //   java: 				.option(Option.<ConfigType>createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.misc.configBackend"))
        //   java: 						.description(Component.translatable("skyblocker.config.misc.configBackend.@Tooltip"))
        //   java: 						.binding(defaults.misc.configBackend,
        //   java: 								() -> config.misc.configBackend,
        //   java: 								newValue -> config.misc.configBackend = newValue)
        //   java: 						.controller(ConfigUtils.createEnumController())
        //   java: 						.build())
        //   java: 				.option(Option.<Boolean>createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.misc.cat"))
        //   java: 						.description(Component.translatable("skyblocker.config.misc.cat.@Tooltip"))
        //   java: 						.binding(
        //   java: 								defaults.misc.cat,
        //   java: 								() -> config.misc.cat,
        //   java: 								newValue -> config.misc.cat = newValue)
        //   java: 						.controller(ConfigUtils.createBooleanController())
        //   java: 						.build()
        //   java: 				)
        //   java:
        //   java: 				//Discord RPC
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.misc.richPresence"))
        //   java: 						.collapsed(true)
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.misc.richPresence.enableRichPresence"))
        //   java: 								.binding(defaults.misc.richPresence.enableRichPresence,
        //   java: 										() -> config.misc.richPresence.enableRichPresence,
        //   java: 										newValue -> config.misc.richPresence.enableRichPresence = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<MiscConfig.Info>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.misc.richPresence.info"))
        //   java: 								.description(Component.translatable("skyblocker.config.misc.richPresence.info.@Tooltip"))
        //   java: 								.binding(defaults.misc.richPresence.info,
        //   java: 										() -> config.misc.richPresence.info,
        //   java: 										newValue -> config.misc.richPresence.info = newValue)
        //   java: 								.controller(ConfigUtils.createEnumController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.misc.richPresence.cycleMode"))
        //   java: 								.binding(defaults.misc.richPresence.cycleMode,
        //   java: 										() -> config.misc.richPresence.cycleMode,
        //   java: 										newValue -> config.misc.richPresence.cycleMode = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<String>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.misc.richPresence.customMessage"))
        //   java: 								.binding(defaults.misc.richPresence.customMessage,
        //   java: 										() -> config.misc.richPresence.customMessage,
        //   java: 										newValue -> config.misc.richPresence.customMessage = newValue)
        //   java: 								.controller(StringController.createBuilder().build())
        //   java: 								.build())
        //   java: 						.build())
        //   java: 				.build();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}

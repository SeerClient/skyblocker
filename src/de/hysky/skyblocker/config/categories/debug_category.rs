//! Ported from de.hysky.skyblocker.config.categories.DebugCategory by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config::SkyblockerConfig;

/// java: public class DebugCategory
#[derive(Clone)]
pub struct DebugCategory {}

impl DebugCategory {
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
        //   java: 				.id(SkyblockerMod.id("config/debug"))
        //   java: 				.name(Component.translatable("skyblocker.config.debug"))
        //   java: 				.optionIf(Debug.dumpHoveredItemKey != null, () -> KeyMappingOption.createBuilder()
        //   java: 						.name(Component.translatable("key.skyblocker.debug.dumpHoveredItem"))
        //   java: 						.tags(CommonTags.KEY_MAPPING)
        //   java: 						.keyMapping(Debug.dumpHoveredItemKey)
        //   java: 						.build())
        //   java: 				.optionIf(Debug.dumpNearbyEntitiesKey != null, () -> KeyMappingOption.createBuilder()
        //   java: 						.name(Component.translatable("key.skyblocker.debug.dumpNearbyEntities"))
        //   java: 						.tags(CommonTags.KEY_MAPPING)
        //   java: 						.keyMapping(Debug.dumpNearbyEntitiesKey)
        //   java: 						.build())
        //   java: 				.option(Option.<Integer>createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.debug.dumpRange"))
        //   java: 						.description(Component.translatable("skyblocker.config.debug.dumpRange.@Tooltip"))
        //   java: 						.binding(defaults.debug.dumpRange,
        //   java: 								() -> config.debug.dumpRange,
        //   java: 								newValue -> config.debug.dumpRange = newValue)
        //   java: 						.controller(IntegerController.createBuilder().range(1, 25).slider(1).build())
        //   java: 						.build())
        //   java: 				.option(Option.<Boolean>createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.debug.showInvisibleArmorStands"))
        //   java: 						.binding(defaults.debug.showInvisibleArmorStands,
        //   java: 								() -> config.debug.showInvisibleArmorStands,
        //   java: 								newValue -> config.debug.showInvisibleArmorStands = newValue)
        //   java: 						.controller(ConfigUtils.createBooleanController())
        //   java: 						.build())
        //   java: 				.option(Option.<Boolean>createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.debug.debugWebSockets"))
        //   java: 						.binding(defaults.debug.webSocketDebug,
        //   java: 								() -> config.debug.webSocketDebug,
        //   java: 								newValue -> config.debug.webSocketDebug = newValue)
        //   java: 						.controller(ConfigUtils.createBooleanController())
        //   java: 						.build())
        //   java: 				.option(Option.<Debug.DumpFormat>createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.debug.dumpFormat"))
        //   java: 						.description(Component.translatable("skyblocker.config.debug.dumpFormat.@Tooltip"))
        //   java: 						.binding(defaults.debug.dumpFormat,
        //   java: 								() -> config.debug.dumpFormat,
        //   java: 								newValue -> config.debug.dumpFormat = newValue)
        //   java: 						.controller(ConfigUtils.createEnumController())
        //   java: 						.build())
        //   java: 				.option(Option.<Boolean>createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.debug.corpseFinderDebug"))
        //   java: 						.binding(defaults.debug.corpseFinderDebug,
        //   java: 								() -> config.debug.corpseFinderDebug,
        //   java: 								newValue -> config.debug.corpseFinderDebug = newValue)
        //   java: 						.controller(ConfigUtils.createBooleanController())
        //   java: 						.build())
        //   java: 				.option(Option.<Boolean>createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.debug.enableRepoDev"))
        //   java: 						.binding(defaults.debug.enableRepoDev,
        //   java: 								() -> config.debug.enableRepoDev,
        //   java: 								newValue -> config.debug.enableRepoDev = newValue)
        //   java: 						.controller(ConfigUtils.createBooleanController())
        //   java: 						.build())
        //   java:
        //   java: 				// SkyBlock Resource Pack
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.debug.skyblockResourcePack"))
        //   java: 						.collapsed(true)
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.debug.skyblockResourcePack.downloadResourcePack"))
        //   java: 								.description(Component.translatable("skyblocker.config.debug.skyblockResourcePack.downloadResourcePack.@Tooltip"))
        //   java: 								.binding(defaults.debug.skyblockResourcePack.downloadResourcePack,
        //   java: 										() -> config.debug.skyblockResourcePack.downloadResourcePack,
        //   java: 										newValue -> config.debug.skyblockResourcePack.downloadResourcePack = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.listener((option, updateType) -> {
        //   java: 									if (updateType == OptionListener.UpdateType.VALUE_CHANGE && option.binding().get()) {
        //   java: 										SkyBlockResourcePackDownloader.downloadResourcePack();
        //   java: 									}
        //   java: 								})
        //   java: 								.build())
        //   java: 						.build())
        //   java: 				.build();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}

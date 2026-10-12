//! Ported from de.hysky.skyblocker.config.categories.HuntingCategory by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config::SkyblockerConfig;

/// java: public class HuntingCategory
#[derive(Clone)]
pub struct HuntingCategory {}

impl HuntingCategory {
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
        //   java: 				.id(SkyblockerMod.id("config/hunting"))
        //   java: 				.name(Component.translatable("skyblocker.config.hunting"))
        //   java: 				.option(Option.<Boolean>createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.hunting.huntingBoxHelper"))
        //   java: 						.binding(defaults.hunting.huntingBox.enabled,
        //   java: 								() -> config.hunting.huntingBox.enabled,
        //   java: 								value -> config.hunting.huntingBox.enabled = value)
        //   java: 						.controller(ConfigUtils.createBooleanController())
        //   java: 						.description(Component.translatable("skyblocker.config.hunting.huntingBoxHelper.@Tooltip"))
        //   java: 						.build())
        //   java:
        //   java: 				// Moonglade Mob Features
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.hunting.moongladeMobs"))
        //   java: 						.collapsed(false)
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.hunting.moongladeMobs.silencePhantoms"))
        //   java: 								.description(Component.translatable("skyblocker.config.hunting.moongladeMobs.silencePhantoms.@Tooltip"))
        //   java: 								.binding(defaults.hunting.moongladeMobs.silencePhantoms,
        //   java: 										() -> config.hunting.moongladeMobs.silencePhantoms,
        //   java: 										newValue -> config.hunting.moongladeMobs.silencePhantoms = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.hunting.moongladeMobs.highlightHideonleaf"))
        //   java: 								.description(Component.translatable("skyblocker.config.hunting.moongladeMobs.highlightHideonleaf.@Tooltip"))
        //   java: 								.binding(defaults.hunting.moongladeMobs.highlightHideonleaf,
        //   java: 										() -> config.hunting.moongladeMobs.highlightHideonleaf,
        //   java: 										newValue -> config.hunting.moongladeMobs.highlightHideonleaf = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Color>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.hunting.moongladeMobs.hideonleafHighlightColor"))
        //   java: 								.binding(defaults.hunting.moongladeMobs.hideonleafGlowColor,
        //   java: 										() -> config.hunting.moongladeMobs.hideonleafGlowColor,
        //   java: 										newValue -> {
        //   java: 											config.hunting.moongladeMobs.hideonleafGlowColor = newValue;
        //   java: 										})
        //   java: 								.controller(ColourController.createBuilder().hasAlpha(false).build())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.hunting.moongladeMobs.highlightShellwise"))
        //   java: 								.description(Component.translatable("skyblocker.config.hunting.moongladeMobs.highlightShellwise.@Tooltip"))
        //   java: 								.binding(defaults.hunting.moongladeMobs.highlightShellwise,
        //   java: 										() -> config.hunting.moongladeMobs.highlightShellwise,
        //   java: 										newValue -> config.hunting.moongladeMobs.highlightShellwise = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Color>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.hunting.moongladeMobs.shellwiseHighlightColor"))
        //   java: 								.binding(defaults.hunting.moongladeMobs.shellwiseGlowColor,
        //   java: 										() -> config.hunting.moongladeMobs.shellwiseGlowColor,
        //   java: 										newValue -> {
        //   java: 											config.hunting.moongladeMobs.shellwiseGlowColor = newValue;
        //   java: 										})
        //   java: 								.controller(ColourController.createBuilder().hasAlpha(false).build())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.hunting.moongladeMobs.highlightCoralot"))
        //   java: 								.description(Component.translatable("skyblocker.config.hunting.moongladeMobs.highlightCoralot.@Tooltip"))
        //   java: 								.binding(defaults.hunting.moongladeMobs.highlightCoralot,
        //   java: 										() -> config.hunting.moongladeMobs.highlightCoralot,
        //   java: 										newValue -> config.hunting.moongladeMobs.highlightCoralot = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Color>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.hunting.moongladeMobs.coralotHighlightColor"))
        //   java: 								.binding(defaults.hunting.moongladeMobs.coralotGlowColor,
        //   java: 										() -> config.hunting.moongladeMobs.coralotGlowColor,
        //   java: 										newValue -> {
        //   java: 											config.hunting.moongladeMobs.coralotGlowColor = newValue;
        //   java: 										})
        //   java: 								.controller(ColourController.createBuilder().hasAlpha(false).build())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.hunting.moongladeMobs.highlightInvisibug"))
        //   java: 								.description(Component.translatable("skyblocker.config.hunting.moongladeMobs.highlightInvisibug.@Tooltip"))
        //   java: 								.binding(defaults.hunting.moongladeMobs.highlightInvisibug,
        //   java: 										() -> config.hunting.moongladeMobs.highlightInvisibug,
        //   java: 										newValue -> config.hunting.moongladeMobs.highlightInvisibug = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Color>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.hunting.moongladeMobs.highlightInvisibugColor"))
        //   java: 								.binding(defaults.hunting.moongladeMobs.invisibugGlowColor,
        //   java: 										() -> config.hunting.moongladeMobs.invisibugGlowColor,
        //   java: 										newValue -> config.hunting.moongladeMobs.invisibugGlowColor = newValue)
        //   java: 								.controller(ColourController.createBuilder().hasAlpha(false).build())
        //   java: 								.build())
        //   java: 						.build())
        //   java:
        //   java: 				// Torrhus Mobs
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.hunting.torrhusMobs"))
        //   java: 						.collapsed(false)
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.hunting.torrhusMobs.highlightHideonsun"))
        //   java: 								.description(Component.translatable("skyblocker.config.hunting.torrhusMobs.highlightHideonsun.@Tooltip"))
        //   java: 								.tags(CommonTags.ADDED_IN_6_9_0)
        //   java: 								.binding(defaults.hunting.torrhusMobs.highlightHideonsun,
        //   java: 										() -> config.hunting.torrhusMobs.highlightHideonsun,
        //   java: 										newValue -> config.hunting.torrhusMobs.highlightHideonsun = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Color>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.hunting.torrhusMobs.hideonsunHighlightColor"))
        //   java: 								.tags(CommonTags.ADDED_IN_6_9_0)
        //   java: 								.binding(defaults.hunting.torrhusMobs.hideonsunHighlightColor,
        //   java: 										() -> config.hunting.torrhusMobs.hideonsunHighlightColor,
        //   java: 										newValue -> config.hunting.torrhusMobs.hideonsunHighlightColor = newValue)
        //   java: 								.controller(ColourController.createBuilder().hasAlpha(false).build())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.hunting.torrhusMobs.highlightBlueJay"))
        //   java: 								.description(Component.translatable("skyblocker.config.hunting.torrhusMobs.highlightBlueJay.@Tooltip"))
        //   java: 								.tags(CommonTags.ADDED_IN_6_9_0)
        //   java: 								.binding(defaults.hunting.torrhusMobs.highlightBlueJay,
        //   java: 										() -> config.hunting.torrhusMobs.highlightBlueJay,
        //   java: 										newValue -> config.hunting.torrhusMobs.highlightBlueJay = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Color>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.hunting.torrhusMobs.blueJayHighlightColor"))
        //   java: 								.tags(CommonTags.ADDED_IN_6_9_0)
        //   java: 								.binding(defaults.hunting.torrhusMobs.blueJayHighlightColor,
        //   java: 										() -> config.hunting.torrhusMobs.blueJayHighlightColor,
        //   java: 										newValue -> config.hunting.torrhusMobs.blueJayHighlightColor = newValue)
        //   java: 								.controller(ColourController.createBuilder().hasAlpha(false).build())
        //   java: 								.build())
        //   java: 						.build())
        //   java:
        //   java: 				// Safari
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.hunting.safari"))
        //   java: 						.collapsed(false)
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.hunting.safari.silencePhantoms"))
        //   java: 								.description(Component.translatable("skyblocker.config.hunting.safari.silencePhantoms.@Tooltip"))
        //   java: 								.tags(CommonTags.ADDED_IN_6_9_0)
        //   java: 								.binding(defaults.hunting.safari.silencePhantoms,
        //   java: 										() -> config.hunting.safari.silencePhantoms,
        //   java: 										newValue -> config.hunting.safari.silencePhantoms = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.hunting.safari.highlightSparklingCritters"))
        //   java: 								.description(Component.translatable("skyblocker.config.hunting.safari.highlightSparklingCritters.@Tooltip"))
        //   java: 								.tags(CommonTags.ADDED_IN_6_9_1)
        //   java: 								.binding(defaults.hunting.safari.highlightSparklingCritters,
        //   java: 										() -> config.hunting.safari.highlightSparklingCritters,
        //   java: 										newValue -> config.hunting.safari.highlightSparklingCritters = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Color>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.hunting.safari.sparklingCritterHighlightColor"))
        //   java: 								.tags(CommonTags.ADDED_IN_6_9_1)
        //   java: 								.binding(defaults.hunting.safari.sparklingCritterHighlightColor,
        //   java: 										() -> config.hunting.safari.sparklingCritterHighlightColor,
        //   java: 										newValue -> config.hunting.safari.sparklingCritterHighlightColor = newValue)
        //   java: 								.controller(ColourController.createBuilder().hasAlpha(false).build())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.hunting.safari.critterCapsuleHelper"))
        //   java: 								.description(Component.translatable("skyblocker.config.hunting.safari.critterCapsuleHelper.@Tooltip"))
        //   java: 								.binding(defaults.hunting.safari.critterCapsuleHelper,
        //   java: 										() -> config.hunting.safari.critterCapsuleHelper,
        //   java: 										newValue -> config.hunting.safari.critterCapsuleHelper = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Color>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.hunting.safari.critterCapsuleHelperColor"))
        //   java: 								.binding(defaults.hunting.safari.critterCapsuleHelperColor,
        //   java: 										() -> config.hunting.safari.critterCapsuleHelperColor,
        //   java: 										newValue -> config.hunting.safari.critterCapsuleHelperColor = newValue)
        //   java: 								.controller(ColourController.createBuilder().hasAlpha(false).build())
        //   java: 								.build())
        //   java: 						.build())
        //   java:
        //   java: 				// Cavern Biome
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.hunting.cavernBiome"))
        //   java: 						.collapsed(false)
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.hunting.cavernBiome.highlightRockmiteMounds"))
        //   java: 								.description(Component.translatable("skyblocker.config.hunting.cavernBiome.highlightRockmiteMounds.@Tooltip"))
        //   java: 								.tags(CommonTags.ADDED_IN_6_9_0)
        //   java: 								.binding(defaults.hunting.cavernBiome.highlightRockmiteMounds,
        //   java: 										() -> config.hunting.cavernBiome.highlightRockmiteMounds,
        //   java: 										newValue -> config.hunting.cavernBiome.highlightRockmiteMounds = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Color>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.hunting.cavernBiome.rockmiteMoundHighlightColor"))
        //   java: 								.tags(CommonTags.ADDED_IN_6_9_0)
        //   java: 								.binding(defaults.hunting.cavernBiome.rockmiteMoundHighlightColor,
        //   java: 										() -> config.hunting.cavernBiome.rockmiteMoundHighlightColor,
        //   java: 										newValue -> config.hunting.cavernBiome.rockmiteMoundHighlightColor = newValue)
        //   java: 								.controller(ColourController.createBuilder().hasAlpha(false).build())
        //   java: 								.build())
        //   java: 						.build())
        //   java:
        //   java: 				// Forest Biome
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.hunting.forestBiome"))
        //   java: 						.collapsed(false)
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.hunting.forestBiome.highlightHideonfloor"))
        //   java: 								.description(Component.translatable("skyblocker.config.hunting.forestBiome.highlightHideonfloor.@Tooltip"))
        //   java: 								.tags(CommonTags.ADDED_IN_6_9_0)
        //   java: 								.binding(defaults.hunting.forestBiome.highlightHideonfloor,
        //   java: 										() -> config.hunting.forestBiome.highlightHideonfloor,
        //   java: 										newValue -> config.hunting.forestBiome.highlightHideonfloor = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Color>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.hunting.forestBiome.hideonfloorHighlightColor"))
        //   java: 								.tags(CommonTags.ADDED_IN_6_9_0)
        //   java: 								.binding(defaults.hunting.forestBiome.hideonfloorHighlightColor,
        //   java: 										() -> config.hunting.forestBiome.hideonfloorHighlightColor,
        //   java: 										newValue -> config.hunting.forestBiome.hideonfloorHighlightColor = newValue)
        //   java: 								.controller(ColourController.createBuilder().hasAlpha(false).build())
        //   java: 								.build())
        //   java: 						.build())
        //   java:
        //   java: 				// Haunted Biome
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.hunting.hauntedBiome"))
        //   java: 						.collapsed(false)
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.hunting.hauntedBiome.highlightDuplico"))
        //   java: 								.description(Component.translatable("skyblocker.config.hunting.hauntedBiome.highlightDuplico.@Tooltip"))
        //   java: 								.tags(CommonTags.ADDED_IN_6_9_0)
        //   java: 								.binding(defaults.hunting.hauntedBiome.highlightDuplico,
        //   java: 										() -> config.hunting.hauntedBiome.highlightDuplico,
        //   java: 										newValue -> config.hunting.hauntedBiome.highlightDuplico = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Color>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.hunting.hauntedBiome.duplicoHighlightColor"))
        //   java: 								.tags(CommonTags.ADDED_IN_6_9_0)
        //   java: 								.binding(defaults.hunting.hauntedBiome.duplicoHighlightColor,
        //   java: 										() -> config.hunting.hauntedBiome.duplicoHighlightColor,
        //   java: 										newValue -> config.hunting.hauntedBiome.duplicoHighlightColor = newValue)
        //   java: 								.controller(ColourController.createBuilder().hasAlpha(false).build())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.hunting.hauntedBiome.highlightBloodbat"))
        //   java: 								.description(Component.translatable("skyblocker.config.hunting.hauntedBiome.highlightBloodbat.@Tooltip"))
        //   java: 								.tags(CommonTags.ADDED_IN_6_9_1)
        //   java: 								.binding(defaults.hunting.hauntedBiome.highlightBloodbat,
        //   java: 										() -> config.hunting.hauntedBiome.highlightBloodbat,
        //   java: 										newValue -> config.hunting.hauntedBiome.highlightBloodbat = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Color>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.hunting.hauntedBiome.bloodbatHighlightColor"))
        //   java: 								.tags(CommonTags.ADDED_IN_6_9_1)
        //   java: 								.binding(defaults.hunting.hauntedBiome.bloodbatHighlightColor,
        //   java: 										() -> config.hunting.hauntedBiome.bloodbatHighlightColor,
        //   java: 										newValue -> config.hunting.hauntedBiome.bloodbatHighlightColor = newValue)
        //   java: 								.controller(ColourController.createBuilder().hasAlpha(false).build())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.hunting.hauntedBiome.ignoreSlotLockingForShiningCoins"))
        //   java: 								.description(Component.translatable("skyblocker.config.hunting.hauntedBiome.ignoreSlotLockingForShiningCoins.@Tooltip"))
        //   java: 								.tags(CommonTags.ADDED_IN_6_9_1)
        //   java: 								.binding(defaults.hunting.hauntedBiome.ignoreSlotLockingForShiningCoins,
        //   java: 										() -> config.hunting.hauntedBiome.ignoreSlotLockingForShiningCoins,
        //   java: 										newValue -> config.hunting.hauntedBiome.ignoreSlotLockingForShiningCoins = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.build())
        //   java:
        //   java: 				// Icy Biome
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.hunting.icyBiome"))
        //   java: 						.collapsed(false)
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.hunting.icyBiome.hideColdOverlay"))
        //   java: 								.description(Component.translatable("skyblocker.config.hunting.icyBiome.hideColdOverlay.@Tooltip"))
        //   java: 								.tags(CommonTags.ADDED_IN_6_9_0)
        //   java: 								.binding(defaults.hunting.icyBiome.hideColdOverlay,
        //   java: 										() -> config.hunting.icyBiome.hideColdOverlay,
        //   java: 										newValue -> config.hunting.icyBiome.hideColdOverlay = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.build())
        //   java:
        //   java: 				// Floor Drops
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.hunting.floorDrops"))
        //   java: 						.collapsed(false)
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.hunting.floorDrops.highlightFloorDrops"))
        //   java: 								.description(Component.translatable("skyblocker.config.hunting.floorDrops.highlightFloorDrops.@Tooltip"))
        //   java: 								.tags(CommonTags.ADDED_IN_6_9_0)
        //   java: 								.binding(defaults.hunting.floorDrops.highlightFloorDrops,
        //   java: 										() -> config.hunting.floorDrops.highlightFloorDrops,
        //   java: 										newValue -> config.hunting.floorDrops.highlightFloorDrops = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Color>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.hunting.floorDrops.floorDropHighlightColor"))
        //   java: 								.tags(CommonTags.ADDED_IN_6_9_0)
        //   java: 								.binding(defaults.hunting.floorDrops.floorDropHighlightColor,
        //   java: 										() -> config.hunting.floorDrops.floorDropHighlightColor,
        //   java: 										newValue -> config.hunting.floorDrops.floorDropHighlightColor = newValue)
        //   java: 								.controller(ColourController.createBuilder().hasAlpha(false).build())
        //   java: 								.build())
        //   java: 						.build())
        //   java:
        //   java: 				// Lasso Hud
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.hunting.lassoHud"))
        //   java: 						.collapsed(false)
        //   java: 						.option(ButtonOption.createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.hunting.lassoHud.hud.screen"))
        //   java: 								.description(Component.translatable("skyblocker.config.hud.movedMessage"))
        //   java: 								.prompt(Component.translatable("text.skyblocker.open"))
        //   java: 								.action(screen -> Minecraft.getInstance().gui.setScreen(new WidgetsConfigurationScreen(Location.GALATEA, screen)))
        //   java: 								.build())
        //   java:
        //   java: 						.build())
        //   java: 				.build();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}

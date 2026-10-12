//! Ported from de.hysky.skyblocker.config.categories.SlayersCategory by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config::SkyblockerConfig;

/// java: public class SlayersCategory
#[derive(Clone)]
pub struct SlayersCategory {}

impl SlayersCategory {
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
        //   java: 				.id(SkyblockerMod.id("config/slayers"))
        //   java: 				.name(Component.translatable("skyblocker.config.slayer"))
        //   java:
        //   java: 				//General Slayers Options
        //   java: 				.option(Option.<SlayersConfig.HighlightSlayerEntities>createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.slayer.highlightMinis"))
        //   java: 						.description(Component.translatable("skyblocker.config.slayer.highlightMinis.@Tooltip[0]"),
        //   java: 								Component.translatable("skyblocker.config.slayer.highlightMinis.@Tooltip[1]"),
        //   java: 								Component.translatable("skyblocker.config.slayer.highlightMinis.@Tooltip[2]"))
        //   java: 						.binding(defaults.slayers.highlightMinis,
        //   java: 								() -> config.slayers.highlightMinis,
        //   java: 								newValue -> config.slayers.highlightMinis = newValue)
        //   java: 						.controller(ConfigUtils.createEnumController())
        //   java: 						.build())
        //   java: 				.option(Option.<SlayersConfig.HighlightSlayerEntities>createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.slayer.highlightBosses"))
        //   java: 						.description(Component.translatable("skyblocker.config.slayer.highlightBosses.@Tooltip[0]"),
        //   java: 								Component.translatable("skyblocker.config.slayer.highlightBosses.@Tooltip[1]"),
        //   java: 								Component.translatable("skyblocker.config.slayer.highlightBosses.@Tooltip[2]"))
        //   java: 						.binding(defaults.slayers.highlightBosses,
        //   java: 								() -> config.slayers.highlightBosses,
        //   java: 								newValue -> config.slayers.highlightBosses = newValue)
        //   java: 						.controller(ConfigUtils.createEnumController())
        //   java: 						.build())
        //   java: 				.option(Option.<Color>createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.slayer.highlightColor"))
        //   java: 						.tags(CommonTags.ADDED_IN_6_0_0)
        //   java: 						.binding(defaults.slayers.highlightColor,
        //   java: 								() -> config.slayers.highlightColor,
        //   java: 								newValue -> config.slayers.highlightColor = newValue)
        //   java: 						.controller(ColourController.createBuilder().hasAlpha(true).build())
        //   java: 						.build())
        //   java: 				.option(ButtonOption.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.slayer.slayerHud"))
        //   java: 						.description(Component.translatable("skyblocker.config.hud.movedMessage"))
        //   java: 						.prompt(Component.translatable("text.skyblocker.open"))
        //   java: 						.action(screen -> Minecraft.getInstance().gui.setScreen(new WidgetsConfigurationScreen(Location.HUB, screen)))
        //   java: 						.build())
        //   java: 				.option(Option.<Boolean>createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.slayer.bossbar"))
        //   java: 						.description(Component.translatable("skyblocker.config.slayer.bossbar.@Tooltip"))
        //   java: 						.binding(defaults.slayers.displayBossbar,
        //   java: 								() -> config.slayers.displayBossbar,
        //   java: 								newValue -> config.slayers.displayBossbar = newValue)
        //   java: 						.controller(ConfigUtils.createBooleanController())
        //   java: 						.build())
        //   java: 				.option(Option.<Boolean>createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.slayer.bossSpawnAlert"))
        //   java: 						.description(Component.translatable("skyblocker.config.slayer.bossSpawnAlert.@Tooltip"))
        //   java: 						.binding(defaults.slayers.bossSpawnAlert,
        //   java: 								() -> config.slayers.bossSpawnAlert,
        //   java: 								newValue -> config.slayers.bossSpawnAlert = newValue)
        //   java: 						.controller(ConfigUtils.createBooleanController())
        //   java: 						.build())
        //   java: 				.option(Option.<Boolean>createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.slayer.minibossSpawnAlert"))
        //   java: 						.description(Component.translatable("skyblocker.config.slayer.minibossSpawnAlert.@Tooltip"))
        //   java: 						.binding(defaults.slayers.miniBossSpawnAlert,
        //   java: 								() -> config.slayers.miniBossSpawnAlert,
        //   java: 								newValue -> config.slayers.miniBossSpawnAlert = newValue)
        //   java: 						.controller(ConfigUtils.createBooleanController())
        //   java: 						.build())
        //   java: 				.option(Option.<Boolean>createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.slayer.alertOtherMinibosses"))
        //   java: 						.description(Component.translatable("skyblocker.config.slayer.alertOtherMinibosses.@Tooltip"))
        //   java: 						.tags(CommonTags.ADDED_IN_6_0_0)
        //   java: 						.binding(defaults.slayers.alertOtherMinibosses,
        //   java: 								() -> config.slayers.alertOtherMinibosses,
        //   java: 								newValue -> config.slayers.alertOtherMinibosses = newValue)
        //   java: 						.controller(ConfigUtils.createBooleanController())
        //   java: 						.build())
        //   java: 				.option(Option.<Boolean>createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.slayer.showMiniBossNameInAlert"))
        //   java: 						.description(Component.translatable("skyblocker.config.slayer.showMiniBossNameInAlert.@Tooltip"))
        //   java: 						.tags(CommonTags.ADDED_IN_6_0_0)
        //   java: 						.binding(defaults.slayers.showMiniBossNameInAlert,
        //   java: 								() -> config.slayers.showMiniBossNameInAlert,
        //   java: 								newValue -> config.slayers.showMiniBossNameInAlert = newValue)
        //   java: 						.controller(ConfigUtils.createBooleanController())
        //   java: 						.build())
        //   java: 				.option(Option.<Boolean>createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.slayer.slainTime"))
        //   java: 						.description(Component.translatable("skyblocker.config.slayer.slainTime.@Tooltip"))
        //   java: 						.binding(defaults.slayers.slainTime,
        //   java: 								() -> config.slayers.slainTime,
        //   java: 								newValue -> config.slayers.slainTime = newValue)
        //   java: 						.controller(ConfigUtils.createBooleanController())
        //   java: 						.build())
        //   java: 				.option(Option.<ChatFilterResult>createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.slayer.hideSlayerMinibossSpawn"))
        //   java: 						.tags(CommonTags.ADDED_IN_6_0_0)
        //   java: 						.binding(defaults.slayers.hideSlayerMinibossSpawn,
        //   java: 								() -> config.slayers.hideSlayerMinibossSpawn,
        //   java: 								newValue -> config.slayers.hideSlayerMinibossSpawn = newValue)
        //   java: 						.controller(ConfigUtils.createEnumController())
        //   java: 						.build())
        //   java: 				.option(Option.<Boolean>createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.slayer.highlightUnclaimedRewards"))
        //   java: 						.description(Component.translatable("skyblocker.config.slayer.highlightUnclaimedRewards.@Tooltip"))
        //   java: 						.tags(CommonTags.ADDED_IN_6_0_0)
        //   java: 						.binding(defaults.slayers.highlightUnclaimedRewards,
        //   java: 								() -> config.slayers.highlightUnclaimedRewards,
        //   java: 								newValue -> config.slayers.highlightUnclaimedRewards = newValue)
        //   java: 						.controller(ConfigUtils.createBooleanController())
        //   java: 						.build())
        //   java:
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.slayer.callMaddox"))
        //   java: 						.collapsed(true)
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.slayer.callMaddox.sendMessageOnFail"))
        //   java: 								.description(Component.translatable("skyblocker.config.slayer.callMaddox.sendMessageOnFail.@Tooltip"))
        //   java: 								.binding(defaults.slayers.callMaddox.sendMessageOnFail,
        //   java: 										() -> config.slayers.callMaddox.sendMessageOnFail,
        //   java: 										newValue -> config.slayers.callMaddox.sendMessageOnFail = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.slayer.callMaddox.sendMessageOnKill"))
        //   java: 								.description(Component.translatable("skyblocker.config.slayer.callMaddox.sendMessageOnFail.@Tooltip")) // Same tooltip as above
        //   java: 								.binding(defaults.slayers.callMaddox.sendMessageOnKill,
        //   java: 										() -> config.slayers.callMaddox.sendMessageOnKill,
        //   java: 										newValue -> config.slayers.callMaddox.sendMessageOnKill = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.build())
        //   java:
        //   java: 				//Spider Slayer
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.slayer.spiderSlayer"))
        //   java: 						.collapsed(true)
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.slayer.spiderSlayer.muteSpiderSounds"))
        //   java: 								.description(Component.translatable("skyblocker.config.slayer.spiderSlayer.muteSpiderSounds.@Tooltip"))
        //   java: 								.tags(CommonTags.ADDED_IN_6_0_0)
        //   java: 								.binding(defaults.slayers.spiderSlayer.muteSpiderSounds,
        //   java: 										() -> config.slayers.spiderSlayer.muteSpiderSounds,
        //   java: 										newValue -> config.slayers.spiderSlayer.muteSpiderSounds = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.build())
        //   java:
        //   java: 				//Wolf Slayer
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.slayer.wolfSlayer"))
        //   java: 						.collapsed(true)
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.slayer.wolfSlayer.muteWolfSounds"))
        //   java: 								.tags(CommonTags.ADDED_IN_6_0_0)
        //   java: 								.binding(defaults.slayers.wolfSlayer.muteWolfSounds,
        //   java: 										() -> config.slayers.wolfSlayer.muteWolfSounds,
        //   java: 										newValue -> config.slayers.wolfSlayer.muteWolfSounds = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.slayer.wolfSlayer.hideSvenPupNametag"))
        //   java: 								.tags(CommonTags.ADDED_IN_6_0_0)
        //   java: 								.binding(defaults.slayers.wolfSlayer.hideSvenPupNametag,
        //   java: 										() -> config.slayers.wolfSlayer.hideSvenPupNametag,
        //   java: 										newValue -> config.slayers.wolfSlayer.hideSvenPupNametag = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.build())
        //   java:
        //   java: 				//Enderman Slayer
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.slayer.endermanSlayer"))
        //   java: 						.collapsed(true)
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.slayer.endermanSlayer.enableYangGlyphsNotification"))
        //   java: 								.binding(defaults.slayers.endermanSlayer.enableYangGlyphsNotification,
        //   java: 										() -> config.slayers.endermanSlayer.enableYangGlyphsNotification,
        //   java: 										newValue -> config.slayers.endermanSlayer.enableYangGlyphsNotification = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.slayer.endermanSlayer.highlightBeacons"))
        //   java: 								.description(Component.translatable("skyblocker.config.slayer.endermanSlayer.highlightBeacons.@Toolip"))
        //   java: 								.binding(defaults.slayers.endermanSlayer.highlightBeacons,
        //   java: 										() -> config.slayers.endermanSlayer.highlightBeacons,
        //   java: 										newValue -> config.slayers.endermanSlayer.highlightBeacons = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.slayer.endermanSlayer.highlightNukekubiHeads"))
        //   java: 								.binding(defaults.slayers.endermanSlayer.highlightNukekubiHeads,
        //   java: 										() -> config.slayers.endermanSlayer.highlightNukekubiHeads,
        //   java: 										newValue -> config.slayers.endermanSlayer.highlightNukekubiHeads = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.slayer.endermanSlayer.lazerTimer"))
        //   java: 								.binding(defaults.slayers.endermanSlayer.lazerTimer,
        //   java: 										() -> config.slayers.endermanSlayer.lazerTimer,
        //   java: 										newValue -> config.slayers.endermanSlayer.lazerTimer = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.build())
        //   java:
        //   java: 				//Vampire Slayer
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.slayer.vampireSlayer"))
        //   java: 						.collapsed(true)
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.slayer.vampireSlayer.enableEffigyWaypoints"))
        //   java: 								.binding(defaults.slayers.vampireSlayer.enableEffigyWaypoints,
        //   java: 										() -> config.slayers.vampireSlayer.enableEffigyWaypoints,
        //   java: 										newValue -> config.slayers.vampireSlayer.enableEffigyWaypoints = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.slayer.vampireSlayer.compactEffigyWaypoints"))
        //   java: 								.binding(defaults.slayers.vampireSlayer.compactEffigyWaypoints,
        //   java: 										() -> config.slayers.vampireSlayer.compactEffigyWaypoints,
        //   java: 										newValue -> config.slayers.vampireSlayer.compactEffigyWaypoints = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Integer>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.slayer.vampireSlayer.effigyUpdateFrequency"))
        //   java: 								.description(Component.translatable("skyblocker.config.slayer.vampireSlayer.effigyUpdateFrequency.@Tooltip"))
        //   java: 								.binding(defaults.slayers.vampireSlayer.effigyUpdateFrequency,
        //   java: 										() -> config.slayers.vampireSlayer.effigyUpdateFrequency,
        //   java: 										newValue -> config.slayers.vampireSlayer.effigyUpdateFrequency = newValue)
        //   java: 								.controller(IntegerController.createBuilder().range(1, 10).slider(1).build())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.slayer.vampireSlayer.enableHolyIceIndicator"))
        //   java: 								.binding(defaults.slayers.vampireSlayer.enableHolyIceIndicator,
        //   java: 										() -> config.slayers.vampireSlayer.enableHolyIceIndicator,
        //   java: 										newValue -> config.slayers.vampireSlayer.enableHolyIceIndicator = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Integer>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.slayer.vampireSlayer.holyIceIndicatorTickDelay"))
        //   java: 								.binding(defaults.slayers.vampireSlayer.holyIceIndicatorTickDelay,
        //   java: 										() -> config.slayers.vampireSlayer.holyIceIndicatorTickDelay,
        //   java: 										newValue -> config.slayers.vampireSlayer.holyIceIndicatorTickDelay = newValue)
        //   java: 								.controller(IntegerController.createBuilder().build())
        //   java: 								.build())
        //   java: 						.option(Option.<Integer>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.slayer.vampireSlayer.holyIceUpdateFrequency"))
        //   java: 								.description(Component.translatable("skyblocker.config.slayer.vampireSlayer.holyIceUpdateFrequency.@Tooltip"))
        //   java: 								.binding(defaults.slayers.vampireSlayer.holyIceUpdateFrequency,
        //   java: 										() -> config.slayers.vampireSlayer.holyIceUpdateFrequency,
        //   java: 										newValue -> config.slayers.vampireSlayer.holyIceUpdateFrequency = newValue)
        //   java: 								.controller(IntegerController.createBuilder().range(1, 10).slider(1).build())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.slayer.vampireSlayer.enableHealingMelonIndicator"))
        //   java: 								.binding(defaults.slayers.vampireSlayer.enableHealingMelonIndicator,
        //   java: 										() -> config.slayers.vampireSlayer.enableHealingMelonIndicator,
        //   java: 										newValue -> config.slayers.vampireSlayer.enableHealingMelonIndicator = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Float>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.slayer.vampireSlayer.healingMelonHealthThreshold"))
        //   java: 								.binding(defaults.slayers.vampireSlayer.healingMelonHealthThreshold,
        //   java: 										() -> config.slayers.vampireSlayer.healingMelonHealthThreshold,
        //   java: 										newValue -> config.slayers.vampireSlayer.healingMelonHealthThreshold = newValue)
        //   java: 								.controller(FloatController.createBuilder().build())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.slayer.vampireSlayer.enableSteakStakeIndicator"))
        //   java: 								.binding(defaults.slayers.vampireSlayer.enableSteakStakeIndicator,
        //   java: 										() -> config.slayers.vampireSlayer.enableSteakStakeIndicator,
        //   java: 										newValue -> config.slayers.vampireSlayer.enableSteakStakeIndicator = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Integer>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.slayer.vampireSlayer.steakStakeUpdateFrequency"))
        //   java: 								.description(Component.translatable("skyblocker.config.slayer.vampireSlayer.steakStakeUpdateFrequency.@Tooltip"))
        //   java: 								.binding(defaults.slayers.vampireSlayer.steakStakeUpdateFrequency,
        //   java: 										() -> config.slayers.vampireSlayer.steakStakeUpdateFrequency,
        //   java: 										newValue -> config.slayers.vampireSlayer.steakStakeUpdateFrequency = newValue)
        //   java: 								.controller(IntegerController.createBuilder().range(1, 10).slider(1).build())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.slayer.vampireSlayer.enableManiaIndicator"))
        //   java: 								.binding(defaults.slayers.vampireSlayer.enableManiaIndicator,
        //   java: 										() -> config.slayers.vampireSlayer.enableManiaIndicator,
        //   java: 										newValue -> config.slayers.vampireSlayer.enableManiaIndicator = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Integer>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.slayer.vampireSlayer.maniaUpdateFrequency"))
        //   java: 								.description(Component.translatable("skyblocker.config.slayer.vampireSlayer.maniaUpdateFrequency.@Tooltip"))
        //   java: 								.binding(defaults.slayers.vampireSlayer.maniaUpdateFrequency,
        //   java: 										() -> config.slayers.vampireSlayer.maniaUpdateFrequency,
        //   java: 										newValue -> config.slayers.vampireSlayer.maniaUpdateFrequency = newValue)
        //   java: 								.controller(IntegerController.createBuilder().range(1, 10).slider(1).build())
        //   java: 								.build())
        //   java: 						.build())
        //   java:
        //   java: 				//Blaze Slayer
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.slayer.blazeSlayer"))
        //   java: 						.collapsed(true)
        //   java: 						.option(Option.<SlayersConfig.BlazeSlayer.FirePillar>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.slayer.blazeSlayer.enableFirePillarAnnouncer"))
        //   java: 								.description(Component.translatable("skyblocker.config.slayer.blazeSlayer.enableFirePillarAnnouncer.@Tooltip"))
        //   java: 								.binding(defaults.slayers.blazeSlayer.firePillarCountdown,
        //   java: 										() -> config.slayers.blazeSlayer.firePillarCountdown,
        //   java: 										newValue -> config.slayers.blazeSlayer.firePillarCountdown = newValue)
        //   java: 								.controller(ConfigUtils.createEnumController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.slayer.blazeSlayer.attunementHighlights"))
        //   java: 								.description(Component.translatable("skyblocker.config.slayer.blazeSlayer.attunementHighlights.@Tooltip"))
        //   java: 								.binding(defaults.slayers.blazeSlayer.attunementHighlights,
        //   java: 										() -> config.slayers.blazeSlayer.attunementHighlights,
        //   java: 										newValue -> config.slayers.blazeSlayer.attunementHighlights = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.slayer.blazeSlayer.muteBlazeSounds"))
        //   java: 								.description(Component.translatable("skyblocker.config.slayer.blazeSlayer.muteBlazeSounds.@Tooltip"))
        //   java: 								.tags(CommonTags.ADDED_IN_6_0_0)
        //   java: 								.binding(defaults.slayers.blazeSlayer.muteBlazeSounds,
        //   java: 										() -> config.slayers.blazeSlayer.muteBlazeSounds,
        //   java: 										newValue -> config.slayers.blazeSlayer.muteBlazeSounds = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<ChatFilterResult>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.slayer.blazeSlayer.hideHellionShield"))
        //   java: 								.tags(CommonTags.ADDED_IN_6_0_0)
        //   java: 								.binding(defaults.slayers.blazeSlayer.hideHellionShield,
        //   java: 										() -> config.slayers.blazeSlayer.hideHellionShield,
        //   java: 										newValue -> config.slayers.blazeSlayer.hideHellionShield = newValue)
        //   java: 								.controller(ConfigUtils.createEnumController())
        //   java: 								.build())
        //   java: 						.build())
        //   java:
        //   java: 				.build();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}

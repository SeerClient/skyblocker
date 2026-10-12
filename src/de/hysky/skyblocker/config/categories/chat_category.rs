//! Ported from de.hysky.skyblocker.config.categories.ChatCategory by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config::SkyblockerConfig;

/// java: public class ChatCategory
#[derive(Clone)]
pub struct ChatCategory {}

impl ChatCategory {
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
        //   java: 				.id(SkyblockerMod.id("config/chat"))
        //   java: 				.name(Component.translatable("skyblocker.config.chat"))
        //   java: 				.option(Option.<Boolean>createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.chat.skyblockXpMessages"))
        //   java: 						.description(Component.translatable("skyblocker.config.chat.skyblockXpMessages.@Tooltip"))
        //   java: 						.binding(defaults.chat.skyblockXpMessages,
        //   java: 								() -> config.chat.skyblockXpMessages,
        //   java: 								newValue -> config.chat.skyblockXpMessages = newValue)
        //   java: 						.controller(ConfigUtils.createBooleanController())
        //   java: 						.build())
        //   java: 				.option(Option.<Boolean>createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.chat.confirmationPromptHelper"))
        //   java: 						.description(Component.translatable("skyblocker.config.chat.confirmationPromptHelper.@Tooltip"))
        //   java: 						.binding(defaults.chat.confirmationPromptHelper,
        //   java: 								() -> config.chat.confirmationPromptHelper,
        //   java: 								newValue -> config.chat.confirmationPromptHelper = newValue)
        //   java: 						.controller(ConfigUtils.createBooleanController())
        //   java: 						.build())
        //   java: 				.option(Option.<Float>createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.chat.toastDuration"))
        //   java: 						.description(Component.translatable("skyblocker.config.chat.toastDuration.@Tooltip"))
        //   java: 						.tags(CommonTags.ADDED_IN_6_0_0)
        //   java: 						.binding(defaults.chat.toastDisplayDuration,
        //   java: 								() -> config.chat.toastDisplayDuration,
        //   java: 								newValue -> config.chat.toastDisplayDuration = newValue)
        //   java: 						.controller(FloatController.createBuilder().range(1f, 10f).slider(0.1f).build())
        //   java: 						.build())
        //   java:
        //   java: 				//Uncategorized Options
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.chat.filter"))
        //   java: 						.option(Option.<ChatFilterResult>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.chat.filter.hideAbility"))
        //   java: 								.binding(defaults.chat.hideAbility,
        //   java: 										() -> config.chat.hideAbility,
        //   java: 										newValue -> config.chat.hideAbility = newValue)
        //   java: 								.controller(ConfigUtils.createEnumController())
        //   java: 								.build())
        //   java: 						.option(Option.<ChatFilterResult>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.chat.filter.hideHeal"))
        //   java: 								.binding(defaults.chat.hideHeal,
        //   java: 										() -> config.chat.hideHeal,
        //   java: 										newValue -> config.chat.hideHeal = newValue)
        //   java: 								.controller(ConfigUtils.createEnumController())
        //   java: 								.build())
        //   java: 						.option(Option.<ChatFilterResult>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.chat.filter.hideAOTE"))
        //   java: 								.description(Component.translatable("skyblocker.config.chat.filter.hideAOTE.@Tooltip"))
        //   java: 								.binding(defaults.chat.hideAOTE,
        //   java: 										() -> config.chat.hideAOTE,
        //   java: 										newValue -> config.chat.hideAOTE = newValue)
        //   java: 								.controller(ConfigUtils.createEnumController())
        //   java: 								.build())
        //   java: 						.option(Option.<ChatFilterResult>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.chat.filter.hideImplosion"))
        //   java: 								.binding(defaults.chat.hideImplosion,
        //   java: 										() -> config.chat.hideImplosion,
        //   java: 										newValue -> config.chat.hideImplosion = newValue)
        //   java: 								.controller(ConfigUtils.createEnumController())
        //   java: 								.build())
        //   java: 						.option(Option.<ChatFilterResult>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.chat.filter.hideSpiritSceptre"))
        //   java: 								.tags(CommonTags.ADDED_IN_6_4_0)
        //   java: 								.binding(defaults.chat.hideSpiritSceptre,
        //   java: 										() -> config.chat.hideSpiritSceptre,
        //   java: 										newValue -> config.chat.hideSpiritSceptre = newValue)
        //   java: 								.controller(ConfigUtils.createEnumController())
        //   java: 								.build())
        //   java: 						.option(Option.<ChatFilterResult>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.chat.filter.hideMoltenWave"))
        //   java: 								.binding(defaults.chat.hideMoltenWave,
        //   java: 										() -> config.chat.hideMoltenWave,
        //   java: 										newValue -> config.chat.hideMoltenWave = newValue)
        //   java: 								.controller(ConfigUtils.createEnumController())
        //   java: 								.build())
        //   java: 						.option(Option.<ChatFilterResult>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.chat.filter.hideRareCrops"))
        //   java: 								.binding(defaults.chat.hideRareCrops,
        //   java: 										() -> config.chat.hideRareCrops,
        //   java: 										newValue -> config.chat.hideRareCrops = newValue)
        //   java: 								.controller(ConfigUtils.createEnumController())
        //   java: 								.build())
        //   java: 						.option(Option.<ChatFilterResult>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.chat.filter.hideAds"))
        //   java: 								.binding(defaults.chat.hideAds,
        //   java: 										() -> config.chat.hideAds,
        //   java: 										newValue -> config.chat.hideAds = newValue)
        //   java: 								.controller(ConfigUtils.createEnumController())
        //   java: 								.build())
        //   java: 						.option(Option.<ChatFilterResult>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.chat.filter.hideTeleportPad"))
        //   java: 								.binding(defaults.chat.hideTeleportPad,
        //   java: 										() -> config.chat.hideTeleportPad,
        //   java: 										newValue -> config.chat.hideTeleportPad = newValue)
        //   java: 								.controller(ConfigUtils.createEnumController())
        //   java: 								.build())
        //   java: 						.option(Option.<ChatFilterResult>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.chat.filter.hideCombo"))
        //   java: 								.binding(defaults.chat.hideCombo,
        //   java: 										() -> config.chat.hideCombo,
        //   java: 										newValue -> config.chat.hideCombo = newValue)
        //   java: 								.controller(ConfigUtils.createEnumController())
        //   java: 								.build())
        //   java: 						.option(Option.<ChatFilterResult>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.chat.filter.hideAutopet"))
        //   java: 								.binding(defaults.chat.hideAutopet,
        //   java: 										() -> config.chat.hideAutopet,
        //   java: 										newValue -> config.chat.hideAutopet = newValue)
        //   java: 								.controller(ConfigUtils.createEnumController())
        //   java: 								.build())
        //   java: 						.option(Option.<ChatFilterResult>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.chat.filter.hideShowOff"))
        //   java: 								.description(Component.translatable("skyblocker.config.chat.filter.hideShowOff.@Tooltip"))
        //   java: 								.binding(defaults.chat.hideShowOff,
        //   java: 										() -> config.chat.hideShowOff,
        //   java: 										newValue -> config.chat.hideShowOff = newValue)
        //   java: 								.controller(ConfigUtils.createEnumController())
        //   java: 								.build())
        //   java: 						.option(Option.<ChatFilterResult>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.chat.filter.hideToggleSkyMall"))
        //   java: 								.description(Component.translatable("skyblocker.config.chat.filter.hideToggleSkyMall.@Tooltip"))
        //   java: 								.binding(defaults.chat.hideToggleSkyMall,
        //   java: 										() -> config.chat.hideToggleSkyMall,
        //   java: 										newValue -> config.chat.hideToggleSkyMall = newValue)
        //   java: 								.controller(ConfigUtils.createEnumController())
        //   java: 								.build())
        //   java: 						.option(Option.<ChatFilterResult>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.chat.filter.hideToggleLottery"))
        //   java: 								.description(Component.translatable("skyblocker.config.chat.filter.hideToggleLottery.@Tooltip"))
        //   java: 								.binding(defaults.chat.hideToggleLottery,
        //   java: 										() -> config.chat.hideToggleLottery,
        //   java: 										newValue -> config.chat.hideToggleLottery = newValue)
        //   java: 								.controller(ConfigUtils.createEnumController())
        //   java: 								.build())
        //   java: 						.option(Option.<ChatFilterResult>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.chat.filter.hideToggleBeekeeper"))
        //   java: 								.description(Component.translatable("skyblocker.config.chat.filter.hideToggleBeekeeper.@Tooltip"))
        //   java: 								.tags(CommonTags.ADDED_IN_6_9_0)
        //   java: 								.binding(defaults.chat.hideToggleBeekeeper,
        //   java: 										() -> config.chat.hideToggleBeekeeper,
        //   java: 										newValue -> config.chat.hideToggleBeekeeper = newValue)
        //   java: 								.controller(ConfigUtils.createEnumController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.chat.filter.hideMana"))
        //   java: 								.description(Component.translatable("skyblocker.config.chat.filter.hideMana.@Tooltip"))
        //   java: 								.binding(defaults.chat.hideMana,
        //   java: 										() -> config.chat.hideMana,
        //   java: 										newValue -> config.chat.hideMana = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<ChatFilterResult>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.chat.filter.hideMimicKill"))
        //   java: 								.description(Component.translatable("skyblocker.config.chat.filter.hideMimicKill.@Tooltip"))
        //   java: 								.binding(defaults.chat.hideMimicKill,
        //   java: 										() -> config.chat.hideMimicKill,
        //   java: 										newValue -> config.chat.hideMimicKill = newValue)
        //   java: 								.controller(ConfigUtils.createEnumController())
        //   java: 								.build())
        //   java: 						.option(Option.<ChatFilterResult>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.chat.filter.hideDeath"))
        //   java: 								.description(Component.translatable("skyblocker.config.chat.filter.hideDeath.@Tooltip"))
        //   java: 								.binding(defaults.chat.hideDeath,
        //   java: 										() -> config.chat.hideDeath,
        //   java: 										newValue -> config.chat.hideDeath = newValue)
        //   java: 								.controller(ConfigUtils.createEnumController())
        //   java: 								.build())
        //   java: 						.option(Option.<ChatFilterResult>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.chat.filter.hideDungeonBreaker"))
        //   java: 								.binding(defaults.chat.hideDungeonBreaker,
        //   java: 										() -> config.chat.hideDungeonBreaker,
        //   java: 										newValue -> config.chat.hideDungeonBreaker = newValue)
        //   java: 								.controller(ConfigUtils.createEnumController())
        //   java: 								.build())
        //   java: 						.option(Option.<ChatFilterResult>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.chat.filter.hideCritterCapture"))
        //   java: 								.description(Component.translatable("skyblocker.config.chat.filter.hideCritterCapture.@Tooltip"))
        //   java: 								.tags(CommonTags.ADDED_IN_6_10_0)
        //   java: 								.binding(defaults.chat.hideCritterCapture,
        //   java: 										() -> config.chat.hideCritterCapture,
        //   java: 										newValue -> config.chat.hideCritterCapture = newValue)
        //   java: 								.controller(ConfigUtils.createEnumController())
        //   java: 								.build())
        //   java: 						.build())
        //   java:
        //   java: 				//chat rules options
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.chat.chatRules"))
        //   java: 						.collapsed(false)
        //   java: 						.option(ButtonOption.createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.chat.chatRules.screen"))
        //   java: 								.prompt(Component.translatable("text.skyblocker.open"))
        //   java: 								.action(screen -> Minecraft.getInstance().gui.setScreen(new ChatRulesConfigScreen(screen)))
        //   java: 								.build())
        //   java: 						.option(Option.<Integer>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.chat.chatRules.announcementLength"))
        //   java: 								.description(Component.translatable("skyblocker.config.chat.chatRules.announcementLength.@Tooltip"))
        //   java: 								.binding(defaults.chat.chatRuleConfig.announcementLength,
        //   java: 										() -> config.chat.chatRuleConfig.announcementLength,
        //   java: 										newValue -> config.chat.chatRuleConfig.announcementLength = newValue)
        //   java: 								.controller(IntegerController.createBuilder().range(5, 200).slider(1).build())
        //   java: 								.build())
        //   java: 						.build())
        //   java: 				.build();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}

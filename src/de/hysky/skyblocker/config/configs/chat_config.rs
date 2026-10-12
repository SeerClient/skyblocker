//! Ported from de.hysky.skyblocker.config.configs.ChatConfig by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::chat::chat_filter_result::ChatFilterResult;

/// java: public class ChatConfig
#[derive(Clone)]
pub struct ChatConfig {
    /// java: public boolean skyblockXpMessages = true;
    pub skyblock_xp_messages: bool,
    /// java: public boolean confirmationPromptHelper = false;
    pub confirmation_prompt_helper: bool,
    /// java: public float toastDisplayDuration = 4;
    pub toast_display_duration: f32,
    /// java: public ChatFilterResult hideAbility = ChatFilterResult.PASS;
    pub hide_ability: ChatFilterResult,
    /// java: public ChatFilterResult hideHeal = ChatFilterResult.PASS;
    pub hide_heal: ChatFilterResult,
    /// java: public ChatFilterResult hideAOTE = ChatFilterResult.PASS;
    pub hide_aote: ChatFilterResult,
    /// java: public ChatFilterResult hideImplosion = ChatFilterResult.PASS;
    pub hide_implosion: ChatFilterResult,
    /// java: public ChatFilterResult hideSpiritSceptre = ChatFilterResult.PASS;
    pub hide_spirit_sceptre: ChatFilterResult,
    /// java: public ChatFilterResult hideMoltenWave = ChatFilterResult.PASS;
    pub hide_molten_wave: ChatFilterResult,
    /// java: public ChatFilterResult hideRareCrops = ChatFilterResult.PASS;
    pub hide_rare_crops: ChatFilterResult,
    /// java: public ChatFilterResult hideAds = ChatFilterResult.PASS;
    pub hide_ads: ChatFilterResult,
    /// java: public ChatFilterResult hideTeleportPad = ChatFilterResult.PASS;
    pub hide_teleport_pad: ChatFilterResult,
    /// java: public ChatFilterResult hideCombo = ChatFilterResult.PASS;
    pub hide_combo: ChatFilterResult,
    /// java: public ChatFilterResult hideAutopet = ChatFilterResult.PASS;
    pub hide_autopet: ChatFilterResult,
    /// java: public ChatFilterResult hideShowOff = ChatFilterResult.PASS;
    pub hide_show_off: ChatFilterResult,
    /// java: public ChatFilterResult hideToggleSkyMall = ChatFilterResult.PASS;
    pub hide_toggle_sky_mall: ChatFilterResult,
    /// java: public ChatFilterResult hideToggleLottery = ChatFilterResult.PASS;
    pub hide_toggle_lottery: ChatFilterResult,
    /// java: public ChatFilterResult hideToggleBeekeeper = ChatFilterResult.PASS;
    pub hide_toggle_beekeeper: ChatFilterResult,
    /// java: public ChatFilterResult hideMimicKill = ChatFilterResult.PASS;
    pub hide_mimic_kill: ChatFilterResult,
    /// java: public ChatFilterResult hideDeath = ChatFilterResult.PASS;
    pub hide_death: ChatFilterResult,
    /// java: public boolean hideMana = false;
    pub hide_mana: bool,
    /// java: @Deprecated public transient ChatFilterResult hideDicer = ChatFilterResult.PASS;
    pub hide_dicer: ChatFilterResult,
    /// java: public ChatFilterResult hideDungeonBreaker = ChatFilterResult.PASS;
    pub hide_dungeon_breaker: ChatFilterResult,
    /// java: public ChatFilterResult hideCritterCapture = ChatFilterResult.PASS;
    pub hide_critter_capture: ChatFilterResult,
    /// java: public ChatRuleConfig chatRuleConfig = new ChatRuleConfig();
    pub chat_rule_config: ChatRuleConfig,
}

impl ChatConfig {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            skyblock_xp_messages: true,
            confirmation_prompt_helper: false,
            toast_display_duration: ((4) as f32) as f32,
            hide_ability: ChatFilterResult::Pass,
            hide_heal: ChatFilterResult::Pass,
            hide_aote: ChatFilterResult::Pass,
            hide_implosion: ChatFilterResult::Pass,
            hide_spirit_sceptre: ChatFilterResult::Pass,
            hide_molten_wave: ChatFilterResult::Pass,
            hide_rare_crops: ChatFilterResult::Pass,
            hide_ads: ChatFilterResult::Pass,
            hide_teleport_pad: ChatFilterResult::Pass,
            hide_combo: ChatFilterResult::Pass,
            hide_autopet: ChatFilterResult::Pass,
            hide_show_off: ChatFilterResult::Pass,
            hide_toggle_sky_mall: ChatFilterResult::Pass,
            hide_toggle_lottery: ChatFilterResult::Pass,
            hide_toggle_beekeeper: ChatFilterResult::Pass,
            hide_mimic_kill: ChatFilterResult::Pass,
            hide_death: ChatFilterResult::Pass,
            hide_mana: false,
            hide_dicer: ChatFilterResult::Pass,
            hide_dungeon_breaker: ChatFilterResult::Pass,
            hide_critter_capture: ChatFilterResult::Pass,
            chat_rule_config: ChatRuleConfig::new(),
        };
        this
    }

}

/// java: public static class ChatRuleConfig
#[derive(Clone)]
pub struct ChatRuleConfig {
    /// java: public int announcementLength = 60;
    pub announcement_length: i32,
    /// java: @Deprecated public transient int announcementScale = 3;
    pub announcement_scale: i32,
}

impl ChatRuleConfig {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            announcement_length: 60,
            announcement_scale: 3,
        };
        this
    }

}

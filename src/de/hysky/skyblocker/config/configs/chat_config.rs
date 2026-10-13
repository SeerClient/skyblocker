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

// Generated Gson forms for src/de/hysky/skyblocker/config/configs/chat_config.rs (fabric2seer round 24): APPEND to that file.
// Needs `serde_json` in Cargo.toml and the `compat` of this round (dandelion::compat::GsonValue, ClassOf, ...).

/// ChatConfig's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl ChatConfig {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("skyblockXpMessages".to_string(), dandelion::compat::GsonValue::Bool(*(&self.skyblock_xp_messages))),
            ("confirmationPromptHelper".to_string(), dandelion::compat::GsonValue::Bool(*(&self.confirmation_prompt_helper))),
            ("toastDisplayDuration".to_string(), dandelion::compat::GsonValue::float(*(&self.toast_display_duration))),
            ("hideAbility".to_string(), dandelion::compat::GsonValue::str(((&self.hide_ability)).name())),
            ("hideHeal".to_string(), dandelion::compat::GsonValue::str(((&self.hide_heal)).name())),
            ("hideAOTE".to_string(), dandelion::compat::GsonValue::str(((&self.hide_aote)).name())),
            ("hideImplosion".to_string(), dandelion::compat::GsonValue::str(((&self.hide_implosion)).name())),
            ("hideSpiritSceptre".to_string(), dandelion::compat::GsonValue::str(((&self.hide_spirit_sceptre)).name())),
            ("hideMoltenWave".to_string(), dandelion::compat::GsonValue::str(((&self.hide_molten_wave)).name())),
            ("hideRareCrops".to_string(), dandelion::compat::GsonValue::str(((&self.hide_rare_crops)).name())),
            ("hideAds".to_string(), dandelion::compat::GsonValue::str(((&self.hide_ads)).name())),
            ("hideTeleportPad".to_string(), dandelion::compat::GsonValue::str(((&self.hide_teleport_pad)).name())),
            ("hideCombo".to_string(), dandelion::compat::GsonValue::str(((&self.hide_combo)).name())),
            ("hideAutopet".to_string(), dandelion::compat::GsonValue::str(((&self.hide_autopet)).name())),
            ("hideShowOff".to_string(), dandelion::compat::GsonValue::str(((&self.hide_show_off)).name())),
            ("hideToggleSkyMall".to_string(), dandelion::compat::GsonValue::str(((&self.hide_toggle_sky_mall)).name())),
            ("hideToggleLottery".to_string(), dandelion::compat::GsonValue::str(((&self.hide_toggle_lottery)).name())),
            ("hideToggleBeekeeper".to_string(), dandelion::compat::GsonValue::str(((&self.hide_toggle_beekeeper)).name())),
            ("hideMimicKill".to_string(), dandelion::compat::GsonValue::str(((&self.hide_mimic_kill)).name())),
            ("hideDeath".to_string(), dandelion::compat::GsonValue::str(((&self.hide_death)).name())),
            ("hideMana".to_string(), dandelion::compat::GsonValue::Bool(*(&self.hide_mana))),
            ("hideDungeonBreaker".to_string(), dandelion::compat::GsonValue::str(((&self.hide_dungeon_breaker)).name())),
            ("hideCritterCapture".to_string(), dandelion::compat::GsonValue::str(((&self.hide_critter_capture)).name())),
            ("chatRuleConfig".to_string(), ((&self.chat_rule_config)).__gson_write()),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<ChatConfig> {
        let __o = __v.as_object()?;
        let mut __this = ChatConfig::new();
        if let Some(__v) = __o.get("skyblockXpMessages") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.skyblock_xp_messages = __x; }
        }
        if let Some(__v) = __o.get("confirmationPromptHelper") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.confirmation_prompt_helper = __x; }
        }
        if let Some(__v) = __o.get("toastDisplayDuration") {
            if let Some(__x) = dandelion::compat::gson_f64(__v).map(|__n| __n as f32) { __this.toast_display_duration = __x; }
        }
        if let Some(__v) = __o.get("hideAbility") {
            if let Some(__x) = (__v).as_str().and_then(|__s| ChatFilterResult::values().into_iter().find(|__c| __c.name() == __s)) { __this.hide_ability = __x; }
        }
        if let Some(__v) = __o.get("hideHeal") {
            if let Some(__x) = (__v).as_str().and_then(|__s| ChatFilterResult::values().into_iter().find(|__c| __c.name() == __s)) { __this.hide_heal = __x; }
        }
        if let Some(__v) = __o.get("hideAOTE") {
            if let Some(__x) = (__v).as_str().and_then(|__s| ChatFilterResult::values().into_iter().find(|__c| __c.name() == __s)) { __this.hide_aote = __x; }
        }
        if let Some(__v) = __o.get("hideImplosion") {
            if let Some(__x) = (__v).as_str().and_then(|__s| ChatFilterResult::values().into_iter().find(|__c| __c.name() == __s)) { __this.hide_implosion = __x; }
        }
        if let Some(__v) = __o.get("hideSpiritSceptre") {
            if let Some(__x) = (__v).as_str().and_then(|__s| ChatFilterResult::values().into_iter().find(|__c| __c.name() == __s)) { __this.hide_spirit_sceptre = __x; }
        }
        if let Some(__v) = __o.get("hideMoltenWave") {
            if let Some(__x) = (__v).as_str().and_then(|__s| ChatFilterResult::values().into_iter().find(|__c| __c.name() == __s)) { __this.hide_molten_wave = __x; }
        }
        if let Some(__v) = __o.get("hideRareCrops") {
            if let Some(__x) = (__v).as_str().and_then(|__s| ChatFilterResult::values().into_iter().find(|__c| __c.name() == __s)) { __this.hide_rare_crops = __x; }
        }
        if let Some(__v) = __o.get("hideAds") {
            if let Some(__x) = (__v).as_str().and_then(|__s| ChatFilterResult::values().into_iter().find(|__c| __c.name() == __s)) { __this.hide_ads = __x; }
        }
        if let Some(__v) = __o.get("hideTeleportPad") {
            if let Some(__x) = (__v).as_str().and_then(|__s| ChatFilterResult::values().into_iter().find(|__c| __c.name() == __s)) { __this.hide_teleport_pad = __x; }
        }
        if let Some(__v) = __o.get("hideCombo") {
            if let Some(__x) = (__v).as_str().and_then(|__s| ChatFilterResult::values().into_iter().find(|__c| __c.name() == __s)) { __this.hide_combo = __x; }
        }
        if let Some(__v) = __o.get("hideAutopet") {
            if let Some(__x) = (__v).as_str().and_then(|__s| ChatFilterResult::values().into_iter().find(|__c| __c.name() == __s)) { __this.hide_autopet = __x; }
        }
        if let Some(__v) = __o.get("hideShowOff") {
            if let Some(__x) = (__v).as_str().and_then(|__s| ChatFilterResult::values().into_iter().find(|__c| __c.name() == __s)) { __this.hide_show_off = __x; }
        }
        if let Some(__v) = __o.get("hideToggleSkyMall") {
            if let Some(__x) = (__v).as_str().and_then(|__s| ChatFilterResult::values().into_iter().find(|__c| __c.name() == __s)) { __this.hide_toggle_sky_mall = __x; }
        }
        if let Some(__v) = __o.get("hideToggleLottery") {
            if let Some(__x) = (__v).as_str().and_then(|__s| ChatFilterResult::values().into_iter().find(|__c| __c.name() == __s)) { __this.hide_toggle_lottery = __x; }
        }
        if let Some(__v) = __o.get("hideToggleBeekeeper") {
            if let Some(__x) = (__v).as_str().and_then(|__s| ChatFilterResult::values().into_iter().find(|__c| __c.name() == __s)) { __this.hide_toggle_beekeeper = __x; }
        }
        if let Some(__v) = __o.get("hideMimicKill") {
            if let Some(__x) = (__v).as_str().and_then(|__s| ChatFilterResult::values().into_iter().find(|__c| __c.name() == __s)) { __this.hide_mimic_kill = __x; }
        }
        if let Some(__v) = __o.get("hideDeath") {
            if let Some(__x) = (__v).as_str().and_then(|__s| ChatFilterResult::values().into_iter().find(|__c| __c.name() == __s)) { __this.hide_death = __x; }
        }
        if let Some(__v) = __o.get("hideMana") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.hide_mana = __x; }
        }
        if let Some(__v) = __o.get("hideDungeonBreaker") {
            if let Some(__x) = (__v).as_str().and_then(|__s| ChatFilterResult::values().into_iter().find(|__c| __c.name() == __s)) { __this.hide_dungeon_breaker = __x; }
        }
        if let Some(__v) = __o.get("hideCritterCapture") {
            if let Some(__x) = (__v).as_str().and_then(|__s| ChatFilterResult::values().into_iter().find(|__c| __c.name() == __s)) { __this.hide_critter_capture = __x; }
        }
        if let Some(__v) = __o.get("chatRuleConfig") {
            if let Some(__x) = ChatRuleConfig::__gson_read(__v) { __this.chat_rule_config = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "skyblockXpMessages" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.skyblock_xp_messages = *__b; true } else { false },
                "confirmationPromptHelper" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.confirmation_prompt_helper = *__b; true } else { false },
                "hideAbility" => if let dandelion::compat::PatchValue::Enum(__s) = __value { match ChatFilterResult::values().into_iter().find(|__c| __c.name() == __s.as_str()) { Some(__c) => { self.hide_ability = __c; true } None => false } } else { false },
                "hideHeal" => if let dandelion::compat::PatchValue::Enum(__s) = __value { match ChatFilterResult::values().into_iter().find(|__c| __c.name() == __s.as_str()) { Some(__c) => { self.hide_heal = __c; true } None => false } } else { false },
                "hideAOTE" => if let dandelion::compat::PatchValue::Enum(__s) = __value { match ChatFilterResult::values().into_iter().find(|__c| __c.name() == __s.as_str()) { Some(__c) => { self.hide_aote = __c; true } None => false } } else { false },
                "hideImplosion" => if let dandelion::compat::PatchValue::Enum(__s) = __value { match ChatFilterResult::values().into_iter().find(|__c| __c.name() == __s.as_str()) { Some(__c) => { self.hide_implosion = __c; true } None => false } } else { false },
                "hideSpiritSceptre" => if let dandelion::compat::PatchValue::Enum(__s) = __value { match ChatFilterResult::values().into_iter().find(|__c| __c.name() == __s.as_str()) { Some(__c) => { self.hide_spirit_sceptre = __c; true } None => false } } else { false },
                "hideMoltenWave" => if let dandelion::compat::PatchValue::Enum(__s) = __value { match ChatFilterResult::values().into_iter().find(|__c| __c.name() == __s.as_str()) { Some(__c) => { self.hide_molten_wave = __c; true } None => false } } else { false },
                "hideRareCrops" => if let dandelion::compat::PatchValue::Enum(__s) = __value { match ChatFilterResult::values().into_iter().find(|__c| __c.name() == __s.as_str()) { Some(__c) => { self.hide_rare_crops = __c; true } None => false } } else { false },
                "hideAds" => if let dandelion::compat::PatchValue::Enum(__s) = __value { match ChatFilterResult::values().into_iter().find(|__c| __c.name() == __s.as_str()) { Some(__c) => { self.hide_ads = __c; true } None => false } } else { false },
                "hideTeleportPad" => if let dandelion::compat::PatchValue::Enum(__s) = __value { match ChatFilterResult::values().into_iter().find(|__c| __c.name() == __s.as_str()) { Some(__c) => { self.hide_teleport_pad = __c; true } None => false } } else { false },
                "hideCombo" => if let dandelion::compat::PatchValue::Enum(__s) = __value { match ChatFilterResult::values().into_iter().find(|__c| __c.name() == __s.as_str()) { Some(__c) => { self.hide_combo = __c; true } None => false } } else { false },
                "hideAutopet" => if let dandelion::compat::PatchValue::Enum(__s) = __value { match ChatFilterResult::values().into_iter().find(|__c| __c.name() == __s.as_str()) { Some(__c) => { self.hide_autopet = __c; true } None => false } } else { false },
                "hideShowOff" => if let dandelion::compat::PatchValue::Enum(__s) = __value { match ChatFilterResult::values().into_iter().find(|__c| __c.name() == __s.as_str()) { Some(__c) => { self.hide_show_off = __c; true } None => false } } else { false },
                "hideToggleSkyMall" => if let dandelion::compat::PatchValue::Enum(__s) = __value { match ChatFilterResult::values().into_iter().find(|__c| __c.name() == __s.as_str()) { Some(__c) => { self.hide_toggle_sky_mall = __c; true } None => false } } else { false },
                "hideToggleLottery" => if let dandelion::compat::PatchValue::Enum(__s) = __value { match ChatFilterResult::values().into_iter().find(|__c| __c.name() == __s.as_str()) { Some(__c) => { self.hide_toggle_lottery = __c; true } None => false } } else { false },
                "hideToggleBeekeeper" => if let dandelion::compat::PatchValue::Enum(__s) = __value { match ChatFilterResult::values().into_iter().find(|__c| __c.name() == __s.as_str()) { Some(__c) => { self.hide_toggle_beekeeper = __c; true } None => false } } else { false },
                "hideMimicKill" => if let dandelion::compat::PatchValue::Enum(__s) = __value { match ChatFilterResult::values().into_iter().find(|__c| __c.name() == __s.as_str()) { Some(__c) => { self.hide_mimic_kill = __c; true } None => false } } else { false },
                "hideDeath" => if let dandelion::compat::PatchValue::Enum(__s) = __value { match ChatFilterResult::values().into_iter().find(|__c| __c.name() == __s.as_str()) { Some(__c) => { self.hide_death = __c; true } None => false } } else { false },
                "hideMana" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.hide_mana = *__b; true } else { false },
                "hideDungeonBreaker" => if let dandelion::compat::PatchValue::Enum(__s) = __value { match ChatFilterResult::values().into_iter().find(|__c| __c.name() == __s.as_str()) { Some(__c) => { self.hide_dungeon_breaker = __c; true } None => false } } else { false },
                "hideCritterCapture" => if let dandelion::compat::PatchValue::Enum(__s) = __value { match ChatFilterResult::values().into_iter().find(|__c| __c.name() == __s.as_str()) { Some(__c) => { self.hide_critter_capture = __c; true } None => false } } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                "chatRuleConfig" => self.chat_rule_config.__gson_patch(__rest, __value),
                _ => false,
            },
            [] => false,
        }
    }

    /// `ChatConfig.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<ChatConfig> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.ChatConfig",
            new: ChatConfig::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: ChatConfig::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// ChatRuleConfig's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl ChatRuleConfig {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("announcementLength".to_string(), dandelion::compat::GsonValue::int(*(&self.announcement_length) as i64)),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<ChatRuleConfig> {
        let __o = __v.as_object()?;
        let mut __this = ChatRuleConfig::new();
        if let Some(__v) = __o.get("announcementLength") {
            if let Some(__x) = dandelion::compat::gson_i64(__v).map(|__n| __n as i32) { __this.announcement_length = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `ChatRuleConfig.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<ChatRuleConfig> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.ChatConfig$ChatRuleConfig",
            new: ChatRuleConfig::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: ChatRuleConfig::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

//! Ported from de.hysky.skyblocker.config.configs.SlayersConfig by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::chat::chat_filter_result::ChatFilterResult;
use mcrust::java::awt::color::Color;

/// java: public class SlayersConfig
#[derive(Clone)]
pub struct SlayersConfig {
    /// java: public HighlightSlayerEntities highlightMinis = HighlightSlayerEntities.GLOW;
    pub highlight_minis: HighlightSlayerEntities,
    /// java: public HighlightSlayerEntities highlightBosses = HighlightSlayerEntities.GLOW;
    pub highlight_bosses: HighlightSlayerEntities,
    /// java: public Color highlightColor = new Color(0xFFFF4800, true);
    pub highlight_color: Color,
    /// java: public boolean displayBossbar = true;
    pub display_bossbar: bool,
    /// java: public boolean bossSpawnAlert = true;
    pub boss_spawn_alert: bool,
    /// java: public boolean miniBossSpawnAlert = true;
    pub mini_boss_spawn_alert: bool,
    /// java: public boolean alertOtherMinibosses = false;
    pub alert_other_minibosses: bool,
    /// java: public boolean showMiniBossNameInAlert = false;
    pub show_mini_boss_name_in_alert: bool,
    /// java: public boolean slainTime = true;
    pub slain_time: bool,
    /// java: public boolean enableHud = true;
    pub enable_hud: bool,
    /// java: public ChatFilterResult hideSlayerMinibossSpawn = ChatFilterResult.PASS;
    pub hide_slayer_miniboss_spawn: ChatFilterResult,
    /// java: public boolean highlightUnclaimedRewards = true;
    pub highlight_unclaimed_rewards: bool,
    /// java: public CallMaddox callMaddox = new CallMaddox();
    pub call_maddox: CallMaddox,
    /// java: public EndermanSlayer endermanSlayer = new EndermanSlayer();
    pub enderman_slayer: EndermanSlayer,
    /// java: public VampireSlayer vampireSlayer = new VampireSlayer();
    pub vampire_slayer: VampireSlayer,
    /// java: public BlazeSlayer blazeSlayer = new BlazeSlayer();
    pub blaze_slayer: BlazeSlayer,
    /// java: public WolfSlayer wolfSlayer = new WolfSlayer();
    pub wolf_slayer: WolfSlayer,
    /// java: public SpiderSlayer spiderSlayer = new SpiderSlayer();
    pub spider_slayer: SpiderSlayer,
}

impl SlayersConfig {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            highlight_minis: HighlightSlayerEntities::Glow,
            highlight_bosses: HighlightSlayerEntities::Glow,
            highlight_color: Color::from_rgba((0xFFFF4800_u32 as i32), true),
            display_bossbar: true,
            boss_spawn_alert: true,
            mini_boss_spawn_alert: true,
            alert_other_minibosses: false,
            show_mini_boss_name_in_alert: false,
            slain_time: true,
            enable_hud: true,
            hide_slayer_miniboss_spawn: ChatFilterResult::Pass,
            highlight_unclaimed_rewards: true,
            call_maddox: CallMaddox::new(),
            enderman_slayer: EndermanSlayer::new(),
            vampire_slayer: VampireSlayer::new(),
            blaze_slayer: BlazeSlayer::new(),
            wolf_slayer: WolfSlayer::new(),
            spider_slayer: SpiderSlayer::new(),
        };
        this
    }

}

/// java: public enum HighlightSlayerEntities
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum HighlightSlayerEntities {
    Off,
    Glow,
    Hitbox,
}

impl HighlightSlayerEntities {
    pub const VALUES: [HighlightSlayerEntities; 3] = [HighlightSlayerEntities::Off, HighlightSlayerEntities::Glow, HighlightSlayerEntities::Hitbox];

    /// java: values()
    pub fn values() -> Vec<HighlightSlayerEntities> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            HighlightSlayerEntities::Off => "OFF",
            HighlightSlayerEntities::Glow => "GLOW",
            HighlightSlayerEntities::Hitbox => "HITBOX",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> HighlightSlayerEntities {
        match name {
            "OFF" => HighlightSlayerEntities::Off,
            "GLOW" => HighlightSlayerEntities::Glow,
            "HITBOX" => HighlightSlayerEntities::Hitbox,
            _ => panic!("IllegalArgumentException: No enum constant HighlightSlayerEntities.{name}"),
        }
    }

    /// java: @Override public String toString()
    pub fn to_string(&self) -> String {
        return mcrust::net::minecraft::client::resources::language::i18n::get(&format!("skyblocker.config.slayer.highlightBosses.{}", self.name()), &[]);
    }

}

impl std::fmt::Display for HighlightSlayerEntities {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", HighlightSlayerEntities::to_string(self))
    }
}

/// java: public static class CallMaddox
#[derive(Clone)]
pub struct CallMaddox {
    /// java: public boolean sendMessageOnFail = true;
    pub send_message_on_fail: bool,
    /// java: public boolean sendMessageOnKill = false;
    pub send_message_on_kill: bool,
}

impl CallMaddox {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            send_message_on_fail: true,
            send_message_on_kill: false,
        };
        this
    }

}

/// java: public static class SpiderSlayer
#[derive(Clone)]
pub struct SpiderSlayer {
    /// java: public boolean muteSpiderSounds = false;
    pub mute_spider_sounds: bool,
}

impl SpiderSlayer {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            mute_spider_sounds: false,
        };
        this
    }

}

/// java: public static class WolfSlayer
#[derive(Clone)]
pub struct WolfSlayer {
    /// java: public boolean muteWolfSounds = false;
    pub mute_wolf_sounds: bool,
    /// java: public boolean hideSvenPupNametag = true;
    pub hide_sven_pup_nametag: bool,
}

impl WolfSlayer {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            mute_wolf_sounds: false,
            hide_sven_pup_nametag: true,
        };
        this
    }

}

/// java: public static class EndermanSlayer
#[derive(Clone)]
pub struct EndermanSlayer {
    /// java: public boolean enableYangGlyphsNotification = true;
    pub enable_yang_glyphs_notification: bool,
    /// java: public boolean highlightBeacons = true;
    pub highlight_beacons: bool,
    /// java: public boolean highlightNukekubiHeads = true;
    pub highlight_nukekubi_heads: bool,
    /// java: public boolean lazerTimer = true;
    pub lazer_timer: bool,
}

impl EndermanSlayer {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            enable_yang_glyphs_notification: true,
            highlight_beacons: true,
            highlight_nukekubi_heads: true,
            lazer_timer: true,
        };
        this
    }

}

/// java: public static class VampireSlayer
#[derive(Clone)]
pub struct VampireSlayer {
    /// java: public boolean enableEffigyWaypoints = true;
    pub enable_effigy_waypoints: bool,
    /// java: public boolean compactEffigyWaypoints = false;
    pub compact_effigy_waypoints: bool,
    /// java: public int effigyUpdateFrequency = 5;
    pub effigy_update_frequency: i32,
    /// java: public boolean enableHolyIceIndicator = true;
    pub enable_holy_ice_indicator: bool,
    /// java: public int holyIceIndicatorTickDelay = 5;
    pub holy_ice_indicator_tick_delay: i32,
    /// java: public int holyIceUpdateFrequency = 5;
    pub holy_ice_update_frequency: i32,
    /// java: public boolean enableHealingMelonIndicator = true;
    pub enable_healing_melon_indicator: bool,
    /// java: public float healingMelonHealthThreshold = 4f;
    pub healing_melon_health_threshold: f32,
    /// java: public boolean enableSteakStakeIndicator = true;
    pub enable_steak_stake_indicator: bool,
    /// java: public int steakStakeUpdateFrequency = 5;
    pub steak_stake_update_frequency: i32,
    /// java: public boolean enableManiaIndicator = true;
    pub enable_mania_indicator: bool,
    /// java: public int maniaUpdateFrequency = 5;
    pub mania_update_frequency: i32,
}

impl VampireSlayer {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            enable_effigy_waypoints: true,
            compact_effigy_waypoints: false,
            effigy_update_frequency: 5,
            enable_holy_ice_indicator: true,
            holy_ice_indicator_tick_delay: 5,
            holy_ice_update_frequency: 5,
            enable_healing_melon_indicator: true,
            healing_melon_health_threshold: 4.0f32,
            enable_steak_stake_indicator: true,
            steak_stake_update_frequency: 5,
            enable_mania_indicator: true,
            mania_update_frequency: 5,
        };
        this
    }

}

/// java: public static class BlazeSlayer
#[derive(Clone)]
pub struct BlazeSlayer {
    /// java: public FirePillar firePillarCountdown = FirePillar.SOUND_AND_VISUAL;
    pub fire_pillar_countdown: FirePillar,
    /// java: public boolean attunementHighlights = true;
    pub attunement_highlights: bool,
    /// java: public boolean muteBlazeSounds = false;
    pub mute_blaze_sounds: bool,
    /// java: public ChatFilterResult hideHellionShield = ChatFilterResult.PASS;
    pub hide_hellion_shield: ChatFilterResult,
}

impl BlazeSlayer {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            fire_pillar_countdown: FirePillar::SoundAndVisual,
            attunement_highlights: true,
            mute_blaze_sounds: false,
            hide_hellion_shield: ChatFilterResult::Pass,
        };
        this
    }

}

/// java: public enum FirePillar
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FirePillar {
    Off,
    Visual,
    SoundAndVisual,
}

impl FirePillar {
    pub const VALUES: [FirePillar; 3] = [FirePillar::Off, FirePillar::Visual, FirePillar::SoundAndVisual];

    /// java: values()
    pub fn values() -> Vec<FirePillar> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            FirePillar::Off => "OFF",
            FirePillar::Visual => "VISUAL",
            FirePillar::SoundAndVisual => "SOUND_AND_VISUAL",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> FirePillar {
        match name {
            "OFF" => FirePillar::Off,
            "VISUAL" => FirePillar::Visual,
            "SOUND_AND_VISUAL" => FirePillar::SoundAndVisual,
            _ => panic!("IllegalArgumentException: No enum constant FirePillar.{name}"),
        }
    }

    /// java: @Override public String toString()
    pub fn to_string(&self) -> String {
        return mcrust::net::minecraft::client::resources::language::i18n::get(&format!("skyblocker.config.slayer.blazeSlayer.enableFirePillarAnnouncer.mode.{}", self.name()), &[]);
    }

}

impl std::fmt::Display for FirePillar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", FirePillar::to_string(self))
    }
}

// Generated Gson forms for src/de/hysky/skyblocker/config/configs/slayers_config.rs (fabric2seer round 24): APPEND to that file.
// Needs `serde_json` in Cargo.toml and the `compat` of this round (dandelion::compat::GsonValue, ClassOf, ...).

/// SlayersConfig's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl SlayersConfig {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("highlightMinis".to_string(), dandelion::compat::GsonValue::str(((&self.highlight_minis)).name())),
            ("highlightBosses".to_string(), dandelion::compat::GsonValue::str(((&self.highlight_bosses)).name())),
            ("highlightColor".to_string(), dandelion::compat::GsonValue::int(((&self.highlight_color)).get_rgb() as i64)),
            ("displayBossbar".to_string(), dandelion::compat::GsonValue::Bool(*(&self.display_bossbar))),
            ("bossSpawnAlert".to_string(), dandelion::compat::GsonValue::Bool(*(&self.boss_spawn_alert))),
            ("miniBossSpawnAlert".to_string(), dandelion::compat::GsonValue::Bool(*(&self.mini_boss_spawn_alert))),
            ("alertOtherMinibosses".to_string(), dandelion::compat::GsonValue::Bool(*(&self.alert_other_minibosses))),
            ("showMiniBossNameInAlert".to_string(), dandelion::compat::GsonValue::Bool(*(&self.show_mini_boss_name_in_alert))),
            ("slainTime".to_string(), dandelion::compat::GsonValue::Bool(*(&self.slain_time))),
            ("enableHud".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_hud))),
            ("hideSlayerMinibossSpawn".to_string(), dandelion::compat::GsonValue::str(((&self.hide_slayer_miniboss_spawn)).name())),
            ("highlightUnclaimedRewards".to_string(), dandelion::compat::GsonValue::Bool(*(&self.highlight_unclaimed_rewards))),
            ("callMaddox".to_string(), ((&self.call_maddox)).__gson_write()),
            ("endermanSlayer".to_string(), ((&self.enderman_slayer)).__gson_write()),
            ("vampireSlayer".to_string(), ((&self.vampire_slayer)).__gson_write()),
            ("blazeSlayer".to_string(), ((&self.blaze_slayer)).__gson_write()),
            ("wolfSlayer".to_string(), ((&self.wolf_slayer)).__gson_write()),
            ("spiderSlayer".to_string(), ((&self.spider_slayer)).__gson_write()),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<SlayersConfig> {
        let __o = __v.as_object()?;
        let mut __this = SlayersConfig::new();
        if let Some(__v) = __o.get("highlightMinis") {
            if let Some(__x) = (__v).as_str().and_then(|__s| HighlightSlayerEntities::values().into_iter().find(|__c| __c.name() == __s)) { __this.highlight_minis = __x; }
        }
        if let Some(__v) = __o.get("highlightBosses") {
            if let Some(__x) = (__v).as_str().and_then(|__s| HighlightSlayerEntities::values().into_iter().find(|__c| __c.name() == __s)) { __this.highlight_bosses = __x; }
        }
        if let Some(__v) = __o.get("highlightColor") {
            if let Some(__x) = dandelion::compat::gson_i64(__v).map(|__n| Color::from_rgba(__n as i32, true)) { __this.highlight_color = __x; }
        }
        if let Some(__v) = __o.get("displayBossbar") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.display_bossbar = __x; }
        }
        if let Some(__v) = __o.get("bossSpawnAlert") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.boss_spawn_alert = __x; }
        }
        if let Some(__v) = __o.get("miniBossSpawnAlert") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.mini_boss_spawn_alert = __x; }
        }
        if let Some(__v) = __o.get("alertOtherMinibosses") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.alert_other_minibosses = __x; }
        }
        if let Some(__v) = __o.get("showMiniBossNameInAlert") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.show_mini_boss_name_in_alert = __x; }
        }
        if let Some(__v) = __o.get("slainTime") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.slain_time = __x; }
        }
        if let Some(__v) = __o.get("enableHud") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_hud = __x; }
        }
        if let Some(__v) = __o.get("hideSlayerMinibossSpawn") {
            if let Some(__x) = (__v).as_str().and_then(|__s| ChatFilterResult::values().into_iter().find(|__c| __c.name() == __s)) { __this.hide_slayer_miniboss_spawn = __x; }
        }
        if let Some(__v) = __o.get("highlightUnclaimedRewards") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.highlight_unclaimed_rewards = __x; }
        }
        if let Some(__v) = __o.get("callMaddox") {
            if let Some(__x) = CallMaddox::__gson_read(__v) { __this.call_maddox = __x; }
        }
        if let Some(__v) = __o.get("endermanSlayer") {
            if let Some(__x) = EndermanSlayer::__gson_read(__v) { __this.enderman_slayer = __x; }
        }
        if let Some(__v) = __o.get("vampireSlayer") {
            if let Some(__x) = VampireSlayer::__gson_read(__v) { __this.vampire_slayer = __x; }
        }
        if let Some(__v) = __o.get("blazeSlayer") {
            if let Some(__x) = BlazeSlayer::__gson_read(__v) { __this.blaze_slayer = __x; }
        }
        if let Some(__v) = __o.get("wolfSlayer") {
            if let Some(__x) = WolfSlayer::__gson_read(__v) { __this.wolf_slayer = __x; }
        }
        if let Some(__v) = __o.get("spiderSlayer") {
            if let Some(__x) = SpiderSlayer::__gson_read(__v) { __this.spider_slayer = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "highlightMinis" => if let dandelion::compat::PatchValue::Enum(__s) = __value { match HighlightSlayerEntities::values().into_iter().find(|__c| __c.name() == __s.as_str()) { Some(__c) => { self.highlight_minis = __c; true } None => false } } else { false },
                "highlightBosses" => if let dandelion::compat::PatchValue::Enum(__s) = __value { match HighlightSlayerEntities::values().into_iter().find(|__c| __c.name() == __s.as_str()) { Some(__c) => { self.highlight_bosses = __c; true } None => false } } else { false },
                "displayBossbar" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.display_bossbar = *__b; true } else { false },
                "bossSpawnAlert" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.boss_spawn_alert = *__b; true } else { false },
                "miniBossSpawnAlert" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.mini_boss_spawn_alert = *__b; true } else { false },
                "alertOtherMinibosses" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.alert_other_minibosses = *__b; true } else { false },
                "showMiniBossNameInAlert" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.show_mini_boss_name_in_alert = *__b; true } else { false },
                "slainTime" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.slain_time = *__b; true } else { false },
                "enableHud" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_hud = *__b; true } else { false },
                "hideSlayerMinibossSpawn" => if let dandelion::compat::PatchValue::Enum(__s) = __value { match ChatFilterResult::values().into_iter().find(|__c| __c.name() == __s.as_str()) { Some(__c) => { self.hide_slayer_miniboss_spawn = __c; true } None => false } } else { false },
                "highlightUnclaimedRewards" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.highlight_unclaimed_rewards = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                "callMaddox" => self.call_maddox.__gson_patch(__rest, __value),
                "endermanSlayer" => self.enderman_slayer.__gson_patch(__rest, __value),
                "vampireSlayer" => self.vampire_slayer.__gson_patch(__rest, __value),
                "blazeSlayer" => self.blaze_slayer.__gson_patch(__rest, __value),
                "wolfSlayer" => self.wolf_slayer.__gson_patch(__rest, __value),
                "spiderSlayer" => self.spider_slayer.__gson_patch(__rest, __value),
                _ => false,
            },
            [] => false,
        }
    }

    /// `SlayersConfig.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<SlayersConfig> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.SlayersConfig",
            new: SlayersConfig::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: SlayersConfig::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// HighlightSlayerEntities's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl HighlightSlayerEntities {
    pub fn __class_of() -> dandelion::compat::ClassOf<HighlightSlayerEntities> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.SlayersConfig$HighlightSlayerEntities",
            new: || panic!("InstantiationException: an enum has no constructor to call"),
            to_gson: |__e| dandelion::compat::GsonValue::str(__e.name()),
            read_gson: |__v| __v.as_str().and_then(|__s| HighlightSlayerEntities::values().into_iter().find(|__c| __c.name() == __s)),
            patch: |_, _, _| false,
            is_enum: true,
            enum_constants: || HighlightSlayerEntities::values().into_iter().map(|__c| __c.name().to_string()).collect(),
        }
    }
}

/// CallMaddox's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl CallMaddox {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("sendMessageOnFail".to_string(), dandelion::compat::GsonValue::Bool(*(&self.send_message_on_fail))),
            ("sendMessageOnKill".to_string(), dandelion::compat::GsonValue::Bool(*(&self.send_message_on_kill))),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<CallMaddox> {
        let __o = __v.as_object()?;
        let mut __this = CallMaddox::new();
        if let Some(__v) = __o.get("sendMessageOnFail") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.send_message_on_fail = __x; }
        }
        if let Some(__v) = __o.get("sendMessageOnKill") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.send_message_on_kill = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "sendMessageOnFail" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.send_message_on_fail = *__b; true } else { false },
                "sendMessageOnKill" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.send_message_on_kill = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `CallMaddox.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<CallMaddox> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.SlayersConfig$CallMaddox",
            new: CallMaddox::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: CallMaddox::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// SpiderSlayer's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl SpiderSlayer {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("muteSpiderSounds".to_string(), dandelion::compat::GsonValue::Bool(*(&self.mute_spider_sounds))),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<SpiderSlayer> {
        let __o = __v.as_object()?;
        let mut __this = SpiderSlayer::new();
        if let Some(__v) = __o.get("muteSpiderSounds") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.mute_spider_sounds = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "muteSpiderSounds" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.mute_spider_sounds = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `SpiderSlayer.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<SpiderSlayer> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.SlayersConfig$SpiderSlayer",
            new: SpiderSlayer::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: SpiderSlayer::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// WolfSlayer's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl WolfSlayer {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("muteWolfSounds".to_string(), dandelion::compat::GsonValue::Bool(*(&self.mute_wolf_sounds))),
            ("hideSvenPupNametag".to_string(), dandelion::compat::GsonValue::Bool(*(&self.hide_sven_pup_nametag))),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<WolfSlayer> {
        let __o = __v.as_object()?;
        let mut __this = WolfSlayer::new();
        if let Some(__v) = __o.get("muteWolfSounds") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.mute_wolf_sounds = __x; }
        }
        if let Some(__v) = __o.get("hideSvenPupNametag") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.hide_sven_pup_nametag = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "muteWolfSounds" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.mute_wolf_sounds = *__b; true } else { false },
                "hideSvenPupNametag" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.hide_sven_pup_nametag = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `WolfSlayer.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<WolfSlayer> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.SlayersConfig$WolfSlayer",
            new: WolfSlayer::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: WolfSlayer::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// EndermanSlayer's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl EndermanSlayer {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("enableYangGlyphsNotification".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_yang_glyphs_notification))),
            ("highlightBeacons".to_string(), dandelion::compat::GsonValue::Bool(*(&self.highlight_beacons))),
            ("highlightNukekubiHeads".to_string(), dandelion::compat::GsonValue::Bool(*(&self.highlight_nukekubi_heads))),
            ("lazerTimer".to_string(), dandelion::compat::GsonValue::Bool(*(&self.lazer_timer))),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<EndermanSlayer> {
        let __o = __v.as_object()?;
        let mut __this = EndermanSlayer::new();
        if let Some(__v) = __o.get("enableYangGlyphsNotification") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_yang_glyphs_notification = __x; }
        }
        if let Some(__v) = __o.get("highlightBeacons") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.highlight_beacons = __x; }
        }
        if let Some(__v) = __o.get("highlightNukekubiHeads") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.highlight_nukekubi_heads = __x; }
        }
        if let Some(__v) = __o.get("lazerTimer") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.lazer_timer = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "enableYangGlyphsNotification" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_yang_glyphs_notification = *__b; true } else { false },
                "highlightBeacons" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.highlight_beacons = *__b; true } else { false },
                "highlightNukekubiHeads" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.highlight_nukekubi_heads = *__b; true } else { false },
                "lazerTimer" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.lazer_timer = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `EndermanSlayer.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<EndermanSlayer> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.SlayersConfig$EndermanSlayer",
            new: EndermanSlayer::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: EndermanSlayer::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// VampireSlayer's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl VampireSlayer {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("enableEffigyWaypoints".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_effigy_waypoints))),
            ("compactEffigyWaypoints".to_string(), dandelion::compat::GsonValue::Bool(*(&self.compact_effigy_waypoints))),
            ("effigyUpdateFrequency".to_string(), dandelion::compat::GsonValue::int(*(&self.effigy_update_frequency) as i64)),
            ("enableHolyIceIndicator".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_holy_ice_indicator))),
            ("holyIceIndicatorTickDelay".to_string(), dandelion::compat::GsonValue::int(*(&self.holy_ice_indicator_tick_delay) as i64)),
            ("holyIceUpdateFrequency".to_string(), dandelion::compat::GsonValue::int(*(&self.holy_ice_update_frequency) as i64)),
            ("enableHealingMelonIndicator".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_healing_melon_indicator))),
            ("healingMelonHealthThreshold".to_string(), dandelion::compat::GsonValue::float(*(&self.healing_melon_health_threshold))),
            ("enableSteakStakeIndicator".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_steak_stake_indicator))),
            ("steakStakeUpdateFrequency".to_string(), dandelion::compat::GsonValue::int(*(&self.steak_stake_update_frequency) as i64)),
            ("enableManiaIndicator".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_mania_indicator))),
            ("maniaUpdateFrequency".to_string(), dandelion::compat::GsonValue::int(*(&self.mania_update_frequency) as i64)),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<VampireSlayer> {
        let __o = __v.as_object()?;
        let mut __this = VampireSlayer::new();
        if let Some(__v) = __o.get("enableEffigyWaypoints") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_effigy_waypoints = __x; }
        }
        if let Some(__v) = __o.get("compactEffigyWaypoints") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.compact_effigy_waypoints = __x; }
        }
        if let Some(__v) = __o.get("effigyUpdateFrequency") {
            if let Some(__x) = dandelion::compat::gson_i64(__v).map(|__n| __n as i32) { __this.effigy_update_frequency = __x; }
        }
        if let Some(__v) = __o.get("enableHolyIceIndicator") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_holy_ice_indicator = __x; }
        }
        if let Some(__v) = __o.get("holyIceIndicatorTickDelay") {
            if let Some(__x) = dandelion::compat::gson_i64(__v).map(|__n| __n as i32) { __this.holy_ice_indicator_tick_delay = __x; }
        }
        if let Some(__v) = __o.get("holyIceUpdateFrequency") {
            if let Some(__x) = dandelion::compat::gson_i64(__v).map(|__n| __n as i32) { __this.holy_ice_update_frequency = __x; }
        }
        if let Some(__v) = __o.get("enableHealingMelonIndicator") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_healing_melon_indicator = __x; }
        }
        if let Some(__v) = __o.get("healingMelonHealthThreshold") {
            if let Some(__x) = dandelion::compat::gson_f64(__v).map(|__n| __n as f32) { __this.healing_melon_health_threshold = __x; }
        }
        if let Some(__v) = __o.get("enableSteakStakeIndicator") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_steak_stake_indicator = __x; }
        }
        if let Some(__v) = __o.get("steakStakeUpdateFrequency") {
            if let Some(__x) = dandelion::compat::gson_i64(__v).map(|__n| __n as i32) { __this.steak_stake_update_frequency = __x; }
        }
        if let Some(__v) = __o.get("enableManiaIndicator") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_mania_indicator = __x; }
        }
        if let Some(__v) = __o.get("maniaUpdateFrequency") {
            if let Some(__x) = dandelion::compat::gson_i64(__v).map(|__n| __n as i32) { __this.mania_update_frequency = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "enableEffigyWaypoints" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_effigy_waypoints = *__b; true } else { false },
                "compactEffigyWaypoints" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.compact_effigy_waypoints = *__b; true } else { false },
                "enableHolyIceIndicator" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_holy_ice_indicator = *__b; true } else { false },
                "enableHealingMelonIndicator" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_healing_melon_indicator = *__b; true } else { false },
                "enableSteakStakeIndicator" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_steak_stake_indicator = *__b; true } else { false },
                "enableManiaIndicator" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_mania_indicator = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `VampireSlayer.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<VampireSlayer> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.SlayersConfig$VampireSlayer",
            new: VampireSlayer::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: VampireSlayer::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// BlazeSlayer's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl BlazeSlayer {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("firePillarCountdown".to_string(), dandelion::compat::GsonValue::str(((&self.fire_pillar_countdown)).name())),
            ("attunementHighlights".to_string(), dandelion::compat::GsonValue::Bool(*(&self.attunement_highlights))),
            ("muteBlazeSounds".to_string(), dandelion::compat::GsonValue::Bool(*(&self.mute_blaze_sounds))),
            ("hideHellionShield".to_string(), dandelion::compat::GsonValue::str(((&self.hide_hellion_shield)).name())),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<BlazeSlayer> {
        let __o = __v.as_object()?;
        let mut __this = BlazeSlayer::new();
        if let Some(__v) = __o.get("firePillarCountdown") {
            if let Some(__x) = (__v).as_str().and_then(|__s| FirePillar::values().into_iter().find(|__c| __c.name() == __s)) { __this.fire_pillar_countdown = __x; }
        }
        if let Some(__v) = __o.get("attunementHighlights") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.attunement_highlights = __x; }
        }
        if let Some(__v) = __o.get("muteBlazeSounds") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.mute_blaze_sounds = __x; }
        }
        if let Some(__v) = __o.get("hideHellionShield") {
            if let Some(__x) = (__v).as_str().and_then(|__s| ChatFilterResult::values().into_iter().find(|__c| __c.name() == __s)) { __this.hide_hellion_shield = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "firePillarCountdown" => if let dandelion::compat::PatchValue::Enum(__s) = __value { match FirePillar::values().into_iter().find(|__c| __c.name() == __s.as_str()) { Some(__c) => { self.fire_pillar_countdown = __c; true } None => false } } else { false },
                "attunementHighlights" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.attunement_highlights = *__b; true } else { false },
                "muteBlazeSounds" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.mute_blaze_sounds = *__b; true } else { false },
                "hideHellionShield" => if let dandelion::compat::PatchValue::Enum(__s) = __value { match ChatFilterResult::values().into_iter().find(|__c| __c.name() == __s.as_str()) { Some(__c) => { self.hide_hellion_shield = __c; true } None => false } } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `BlazeSlayer.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<BlazeSlayer> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.SlayersConfig$BlazeSlayer",
            new: BlazeSlayer::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: BlazeSlayer::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// FirePillar's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl FirePillar {
    pub fn __class_of() -> dandelion::compat::ClassOf<FirePillar> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.SlayersConfig$BlazeSlayer$FirePillar",
            new: || panic!("InstantiationException: an enum has no constructor to call"),
            to_gson: |__e| dandelion::compat::GsonValue::str(__e.name()),
            read_gson: |__v| __v.as_str().and_then(|__s| FirePillar::values().into_iter().find(|__c| __c.name() == __s)),
            patch: |_, _, _| false,
            is_enum: true,
            enum_constants: || FirePillar::values().into_iter().map(|__c| __c.name().to_string()).collect(),
        }
    }
}

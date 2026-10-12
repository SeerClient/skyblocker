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

//! Ported from de.hysky.skyblocker.config.SkyblockerConfig by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::configs::chat_config::ChatConfig;
use crate::de::hysky::skyblocker::config::configs::crimson_isle_config::CrimsonIsleConfig;
use crate::de::hysky::skyblocker::config::configs::debug_config::DebugConfig;
use crate::de::hysky::skyblocker::config::configs::dungeons_config::DungeonsConfig;
use crate::de::hysky::skyblocker::config::configs::event_notifications_config::EventNotificationsConfig;
use crate::de::hysky::skyblocker::config::configs::farming_config::FarmingConfig;
use crate::de::hysky::skyblocker::config::configs::foraging_config::ForagingConfig;
use crate::de::hysky::skyblocker::config::configs::general_config::GeneralConfig;
use crate::de::hysky::skyblocker::config::configs::helper_config::HelperConfig;
use crate::de::hysky::skyblocker::config::configs::hunting_config::HuntingConfig;
use crate::de::hysky::skyblocker::config::configs::mining_config::MiningConfig;
use crate::de::hysky::skyblocker::config::configs::misc_config::MiscConfig;
use crate::de::hysky::skyblocker::config::configs::other_locations_config::OtherLocationsConfig;
use crate::de::hysky::skyblocker::config::configs::quick_navigation_config::QuickNavigationConfig;
use crate::de::hysky::skyblocker::config::configs::slayers_config::SlayersConfig;
use crate::de::hysky::skyblocker::config::configs::ui_and_visuals_config::UIAndVisualsConfig;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager;

/// java: public class SkyblockerConfig
#[derive(Clone)]
pub struct SkyblockerConfig {
    /// java: public int version = SkyblockerConfigManager.CONFIG_VERSION;
    pub version: i32,
    /// java: public GeneralConfig general = new GeneralConfig();
    pub general: GeneralConfig,
    /// java: public UIAndVisualsConfig uiAndVisuals = new UIAndVisualsConfig();
    pub ui_and_visuals: UIAndVisualsConfig,
    /// java: public HelperConfig helpers = new HelperConfig();
    pub helpers: HelperConfig,
    /// java: public DungeonsConfig dungeons = new DungeonsConfig();
    pub dungeons: DungeonsConfig,
    /// java: public ForagingConfig foraging = new ForagingConfig();
    pub foraging: ForagingConfig,
    /// java: public CrimsonIsleConfig crimsonIsle = new CrimsonIsleConfig();
    pub crimson_isle: CrimsonIsleConfig,
    /// java: public MiningConfig mining = new MiningConfig();
    pub mining: MiningConfig,
    /// java: public FarmingConfig farming = new FarmingConfig();
    pub farming: FarmingConfig,
    /// java: public HuntingConfig hunting = new HuntingConfig();
    pub hunting: HuntingConfig,
    /// java: public OtherLocationsConfig otherLocations = new OtherLocationsConfig();
    pub other_locations: OtherLocationsConfig,
    /// java: public SlayersConfig slayers = new SlayersConfig();
    pub slayers: SlayersConfig,
    /// java: public ChatConfig chat = new ChatConfig();
    pub chat: ChatConfig,
    /// java: public QuickNavigationConfig quickNav = new QuickNavigationConfig();
    pub quick_nav: QuickNavigationConfig,
    /// java: public EventNotificationsConfig eventNotifications = new EventNotificationsConfig();
    pub event_notifications: EventNotificationsConfig,
    /// java: public MiscConfig misc = new MiscConfig();
    pub misc: MiscConfig,
    /// java: public DebugConfig debug = new DebugConfig();
    pub debug: DebugConfig,
}

impl SkyblockerConfig {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            version: skyblocker_config_manager::CONFIG_VERSION,
            general: GeneralConfig::new(),
            ui_and_visuals: UIAndVisualsConfig::new(),
            helpers: HelperConfig::new(),
            dungeons: DungeonsConfig::new(),
            foraging: ForagingConfig::new(),
            crimson_isle: CrimsonIsleConfig::new(),
            mining: MiningConfig::new(),
            farming: FarmingConfig::new(),
            hunting: HuntingConfig::new(),
            other_locations: OtherLocationsConfig::new(),
            slayers: SlayersConfig::new(),
            chat: ChatConfig::new(),
            quick_nav: QuickNavigationConfig::new(),
            event_notifications: EventNotificationsConfig::new(),
            misc: MiscConfig::new(),
            debug: DebugConfig::new(),
        };
        this
    }

}

// Generated Gson forms for src/de/hysky/skyblocker/config/skyblocker_config.rs (fabric2seer round 24): APPEND to that file.
// Needs `serde_json` in Cargo.toml and the `compat` of this round (dandelion::compat::GsonValue, ClassOf, ...).

/// SkyblockerConfig's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl SkyblockerConfig {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("version".to_string(), dandelion::compat::GsonValue::int(*(&self.version) as i64)),
            ("general".to_string(), ((&self.general)).__gson_write()),
            ("uiAndVisuals".to_string(), ((&self.ui_and_visuals)).__gson_write()),
            ("helpers".to_string(), ((&self.helpers)).__gson_write()),
            ("dungeons".to_string(), ((&self.dungeons)).__gson_write()),
            ("foraging".to_string(), ((&self.foraging)).__gson_write()),
            ("crimsonIsle".to_string(), ((&self.crimson_isle)).__gson_write()),
            ("mining".to_string(), ((&self.mining)).__gson_write()),
            ("farming".to_string(), ((&self.farming)).__gson_write()),
            ("hunting".to_string(), ((&self.hunting)).__gson_write()),
            ("otherLocations".to_string(), ((&self.other_locations)).__gson_write()),
            ("slayers".to_string(), ((&self.slayers)).__gson_write()),
            ("chat".to_string(), ((&self.chat)).__gson_write()),
            ("quickNav".to_string(), ((&self.quick_nav)).__gson_write()),
            ("eventNotifications".to_string(), ((&self.event_notifications)).__gson_write()),
            ("misc".to_string(), ((&self.misc)).__gson_write()),
            ("debug".to_string(), ((&self.debug)).__gson_write()),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<SkyblockerConfig> {
        let __o = __v.as_object()?;
        let mut __this = SkyblockerConfig::new();
        if let Some(__v) = __o.get("version") {
            if let Some(__x) = dandelion::compat::gson_i64(__v).map(|__n| __n as i32) { __this.version = __x; }
        }
        if let Some(__v) = __o.get("general") {
            if let Some(__x) = GeneralConfig::__gson_read(__v) { __this.general = __x; }
        }
        if let Some(__v) = __o.get("uiAndVisuals") {
            if let Some(__x) = UIAndVisualsConfig::__gson_read(__v) { __this.ui_and_visuals = __x; }
        }
        if let Some(__v) = __o.get("helpers") {
            if let Some(__x) = HelperConfig::__gson_read(__v) { __this.helpers = __x; }
        }
        if let Some(__v) = __o.get("dungeons") {
            if let Some(__x) = DungeonsConfig::__gson_read(__v) { __this.dungeons = __x; }
        }
        if let Some(__v) = __o.get("foraging") {
            if let Some(__x) = ForagingConfig::__gson_read(__v) { __this.foraging = __x; }
        }
        if let Some(__v) = __o.get("crimsonIsle") {
            if let Some(__x) = CrimsonIsleConfig::__gson_read(__v) { __this.crimson_isle = __x; }
        }
        if let Some(__v) = __o.get("mining") {
            if let Some(__x) = MiningConfig::__gson_read(__v) { __this.mining = __x; }
        }
        if let Some(__v) = __o.get("farming") {
            if let Some(__x) = FarmingConfig::__gson_read(__v) { __this.farming = __x; }
        }
        if let Some(__v) = __o.get("hunting") {
            if let Some(__x) = HuntingConfig::__gson_read(__v) { __this.hunting = __x; }
        }
        if let Some(__v) = __o.get("otherLocations") {
            if let Some(__x) = OtherLocationsConfig::__gson_read(__v) { __this.other_locations = __x; }
        }
        if let Some(__v) = __o.get("slayers") {
            if let Some(__x) = SlayersConfig::__gson_read(__v) { __this.slayers = __x; }
        }
        if let Some(__v) = __o.get("chat") {
            if let Some(__x) = ChatConfig::__gson_read(__v) { __this.chat = __x; }
        }
        if let Some(__v) = __o.get("quickNav") {
            if let Some(__x) = QuickNavigationConfig::__gson_read(__v) { __this.quick_nav = __x; }
        }
        if let Some(__v) = __o.get("eventNotifications") {
            if let Some(__x) = EventNotificationsConfig::__gson_read(__v) { __this.event_notifications = __x; }
        }
        if let Some(__v) = __o.get("misc") {
            if let Some(__x) = MiscConfig::__gson_read(__v) { __this.misc = __x; }
        }
        if let Some(__v) = __o.get("debug") {
            if let Some(__x) = DebugConfig::__gson_read(__v) { __this.debug = __x; }
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
                "general" => self.general.__gson_patch(__rest, __value),
                "uiAndVisuals" => self.ui_and_visuals.__gson_patch(__rest, __value),
                "helpers" => self.helpers.__gson_patch(__rest, __value),
                "dungeons" => self.dungeons.__gson_patch(__rest, __value),
                "foraging" => self.foraging.__gson_patch(__rest, __value),
                "crimsonIsle" => self.crimson_isle.__gson_patch(__rest, __value),
                "mining" => self.mining.__gson_patch(__rest, __value),
                "farming" => self.farming.__gson_patch(__rest, __value),
                "hunting" => self.hunting.__gson_patch(__rest, __value),
                "otherLocations" => self.other_locations.__gson_patch(__rest, __value),
                "slayers" => self.slayers.__gson_patch(__rest, __value),
                "chat" => self.chat.__gson_patch(__rest, __value),
                "quickNav" => self.quick_nav.__gson_patch(__rest, __value),
                "eventNotifications" => self.event_notifications.__gson_patch(__rest, __value),
                "misc" => self.misc.__gson_patch(__rest, __value),
                "debug" => self.debug.__gson_patch(__rest, __value),
                _ => false,
            },
            [] => false,
        }
    }

    /// `SkyblockerConfig.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<SkyblockerConfig> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.SkyblockerConfig",
            new: SkyblockerConfig::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: SkyblockerConfig::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

#[cfg(test)]
mod __gson_round_trip_skyblockerconfig {
    #[allow(unused_imports)]
    use crate::prelude::*;
    #[test]
    fn writes_what_it_reads() {
        let __c = super::SkyblockerConfig::__class_of();
        let __d = (__c.new)();
        let __text = dandelion::compat::gson_text(&(__c.to_gson)(&__d), true);
        let __back = (__c.read_gson)(&serde_json::from_str(&__text).unwrap()).unwrap();
        assert_eq!(dandelion::compat::gson_text(&(__c.to_gson)(&__back), true), __text);
        let __partial = (__c.read_gson)(&serde_json::json!({"__unknown__": 1})).unwrap();
        assert_eq!(dandelion::compat::gson_text(&(__c.to_gson)(&__partial), true), __text);
    }
}

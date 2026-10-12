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

//! Ported from de.hysky.skyblocker.config.configs.DebugConfig by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::debug::debug::DumpFormat;

/// java: public class DebugConfig
#[derive(Clone)]
pub struct DebugConfig {
    /// java: public int dumpRange = 5;
    pub dump_range: i32,
    /// java: public Debug.DumpFormat dumpFormat = Debug.DumpFormat.SNBT;
    pub dump_format: DumpFormat,
    /// java: public boolean showInvisibleArmorStands = false;
    pub show_invisible_armor_stands: bool,
    /// java: public boolean webSocketDebug = false;
    pub web_socket_debug: bool,
    /// java: public boolean corpseFinderDebug = false;
    pub corpse_finder_debug: bool,
    /// java: public boolean enableRepoDev = false;
    pub enable_repo_dev: bool,
    /// java: public SkyBlockResourcePack skyblockResourcePack = new SkyBlockResourcePack();
    pub skyblock_resource_pack: SkyBlockResourcePack,
}

impl DebugConfig {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            dump_range: 5,
            dump_format: DumpFormat::Snbt,
            show_invisible_armor_stands: false,
            web_socket_debug: false,
            corpse_finder_debug: false,
            enable_repo_dev: false,
            skyblock_resource_pack: SkyBlockResourcePack::new(),
        };
        this
    }

}

/// java: public static class SkyBlockResourcePack
#[derive(Clone)]
pub struct SkyBlockResourcePack {
    /// java: public boolean downloadResourcePack = false;
    pub download_resource_pack: bool,
}

impl SkyBlockResourcePack {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            download_resource_pack: false,
        };
        this
    }

}

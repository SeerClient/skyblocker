//! Ported from de.hysky.skyblocker.skyblock.slayers.boss.sven.MuteWolfSounds by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::events::play_sound_events;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::net::minecraft::sounds::sound_event::SoundEvent;

/// java: public class MuteWolfSounds
#[derive(Clone)]
pub struct MuteWolfSounds {}

impl MuteWolfSounds {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        play_sound_events::ALLOW_SOUND.register(Arc::new(move |__a0| MuteWolfSounds::on_sound(&__a0)));
    }

    /// java: private static boolean onSound(SoundEvent sound)
    pub fn on_sound(sound: &SoundEvent) -> bool {
        if SkyblockerConfigManager::get().slayers.wolf_slayer.mute_wolf_sounds && (Utils::is_in_park() || Utils::is_in_hub()) {
            return !(sound.location().to_string().contains("minecraft:entity.wolf."));
        }
        return true;
    }

}

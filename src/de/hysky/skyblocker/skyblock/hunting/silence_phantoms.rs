//! Ported from de.hysky.skyblocker.skyblock.hunting.SilencePhantoms by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::events::play_sound_events;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::net::minecraft::sounds::sound_event::SoundEvent;

/// java: public class SilencePhantoms
#[derive(Clone)]
pub struct SilencePhantoms {}

impl SilencePhantoms {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        play_sound_events::ALLOW_SOUND.register(Arc::new(move |__a0| SilencePhantoms::on_sound(&__a0)));
    }

    /// java: private static boolean shouldProcess()
    pub fn should_process() -> bool {
        if Utils::is_in_galatea() {
            return SkyblockerConfigManager::get().hunting.moonglade_mobs.silence_phantoms;
        } else if Utils::is_in_safari() {
            return SkyblockerConfigManager::get().hunting.safari.silence_phantoms;
        }
        return false;
    }

    /// java: private static boolean onSound(SoundEvent sound)
    pub fn on_sound(sound: &SoundEvent) -> bool {
        return !(SilencePhantoms::should_process() && sound.location().path().starts_with("entity.phantom"));
    }

}

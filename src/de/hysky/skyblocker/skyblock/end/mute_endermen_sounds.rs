//! Ported from de.hysky.skyblocker.skyblock.end.MuteEndermenSounds by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::events::play_sound_events;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::sounds::sound_event::SoundEvent;

// TODO(fabric2seer): static call ObjectSet.of(..)
//   java: private static final ObjectSet<Identifier> MUTED_SOUNDS = ObjectSet.of(
//   java: 			SoundEvents.ENDERMAN_AMBIENT.location(),
//   java: 			SoundEvents.ENDERMAN_DEATH.location(),
//   java: 			SoundEvents.ENDERMAN_HURT.location(),
//   java: 			SoundEvents.ENDERMAN_SCREAM.location(),
//   java: 			SoundEvents.ENDERMAN_STARE.location()
//   java: 	);
/// java: private static final ObjectSet<Identifier> MUTED_SOUNDS = ObjectSet.of( SoundEvents.ENDERMAN_AMBIENT.location(), SoundEvents.ENDERMAN_DEATH.location(), SoundEvents.ENDERMAN_HURT.location(), SoundEvent
pub static MUTED_SOUNDS: compat::MainThread<std::collections::HashSet<Identifier>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `MUTED_SOUNDS` - see the TODO above"));

/// java: public class MuteEndermenSounds
#[derive(Clone)]
pub struct MuteEndermenSounds {}

impl MuteEndermenSounds {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        play_sound_events::ALLOW_SOUND.register(Arc::new(move |__a0| MuteEndermenSounds::on_sound(&__a0)));
    }

    /// java: private static boolean onSound(SoundEvent sound)
    pub fn on_sound(sound: &SoundEvent) -> bool {
        if SkyblockerConfigManager::get().other_locations.end.mute_enderman_sounds && Utils::is_in_the_end() {
            return !(MUTED_SOUNDS.get_mut().contains(sound.location()));
        }
        return true;
    }

}

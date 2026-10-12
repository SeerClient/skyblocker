//! Ported from de.hysky.skyblocker.skyblock.slayers.boss.tarantula.MuteSpiderSounds by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::events::play_sound_events;
use crate::de::hysky::skyblocker::skyblock::slayers::slayer_manager::SlayerManager;
use crate::de::hysky::skyblocker::skyblock::slayers::slayer_type::SlayerType;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::sounds::sound_event::SoundEvent;

// TODO(fabric2seer): static call ObjectSet.of(..)
//   java: private static final ObjectSet<Identifier> MUTED_SOUNDS = ObjectSet.of(
//   java: 			SoundEvents.SPIDER_AMBIENT.location(),
//   java: 			SoundEvents.SPIDER_DEATH.location(),
//   java: 			SoundEvents.SPIDER_HURT.location(),
//   java: 			SoundEvents.SPIDER_STEP.location(),
//   java: 			SoundEvents.BAT_HURT.location(),
//   java: 			SoundEvents.SILVERFISH_HURT.location(),
//   java: 			SoundEvents.SILVERFISH_DEATH.location(),
//   java: 			SoundEvents.SILVERFISH_AMBIENT.location(),
//   java: 			SoundEvents.SKELETON_AMBIENT.location(),
//   java: 			SoundEvents.SKELETON_DEATH.location(),
//   java: 			SoundEvents.SKELETON_HURT.location(),
//   java: 			SoundEvents.SKELETON_STEP.location()
//   java: 	);
/// java: private static final ObjectSet<Identifier> MUTED_SOUNDS = ObjectSet.of( SoundEvents.SPIDER_AMBIENT.location(), SoundEvents.SPIDER_DEATH.location(), SoundEvents.SPIDER_HURT.location(), SoundEvents.SPID
pub static MUTED_SOUNDS: compat::MainThread<std::collections::HashSet<Identifier>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `MUTED_SOUNDS` - see the TODO above"));

/// java: public class MuteSpiderSounds
#[derive(Clone)]
pub struct MuteSpiderSounds {}

impl MuteSpiderSounds {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        play_sound_events::ALLOW_SOUND.register(Arc::new(move |__a0| MuteSpiderSounds::on_sound(&__a0)));
    }

    /// java: private static boolean onSound(SoundEvent sound)
    pub fn on_sound(sound: &SoundEvent) -> bool {
        if SkyblockerConfigManager::get().slayers.spider_slayer.mute_spider_sounds && (Utils::is_in_spiders_den() || Utils::is_in_crimson()) {
            if !(SlayerManager::is_fighting_slayer_type(SlayerType::Demonlord)) {
                return !(MUTED_SOUNDS.get_mut().contains(sound.location()));
            }
        }
        return true;
    }

}

//! Ported from de.hysky.skyblocker.skyblock.slayers.boss.demonlord.MuteBlazeSounds by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::events::play_sound_events;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::sounds::sound_event::SoundEvent;

// TODO(fabric2seer): static call ObjectSet.of(..)
//   java: private static final ObjectSet<Identifier> MUTED_SOUNDS = ObjectSet.of(
//   java: 			SoundEvents.BLAZE_BURN.location(),
//   java: 			SoundEvents.BLAZE_AMBIENT.location(),
//   java: 			SoundEvents.BLAZE_HURT.location(),
//   java: 			SoundEvents.GHAST_SHOOT.location(),
//   java: 			SoundEvents.FIRE_AMBIENT.location(),
//   java: 			SoundEvents.LAVA_POP.location(),
//   java: 			SoundEvents.LIGHTNING_BOLT_THUNDER.location(),
//   java: 			SoundEvents.LIGHTNING_BOLT_IMPACT.location()
//   java: 	);
/// java: private static final ObjectSet<Identifier> MUTED_SOUNDS = ObjectSet.of( SoundEvents.BLAZE_BURN.location(), SoundEvents.BLAZE_AMBIENT.location(), SoundEvents.BLAZE_HURT.location(), SoundEvents.GHAST_SH
pub static MUTED_SOUNDS: compat::MainThread<std::collections::HashSet<Identifier>> = compat::MainThread::new(|| { use mcrust::net::minecraft::sounds::sound_events as se; [se::blaze_burn(), se::blaze_ambient(), se::blaze_hurt(), se::ghast_shoot(), se::fire_ambient(), se::lava_pop(), se::lightning_bolt_thunder(), se::lightning_bolt_impact()].into_iter().map(|e| e.location().clone()).collect() });

/// java: @SuppressWarnings("unused") public class MuteBlazeSounds
#[derive(Clone)]
pub struct MuteBlazeSounds {}

impl MuteBlazeSounds {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        play_sound_events::ALLOW_SOUND.register(Arc::new(move |__a0| MuteBlazeSounds::on_sound(&__a0)));
    }

    /// java: private static boolean onSound(SoundEvent sound)
    pub fn on_sound(sound: &SoundEvent) -> bool {
        if SkyblockerConfigManager::get().slayers.blaze_slayer.mute_blaze_sounds && Utils::is_in_crimson() {
            return !(MUTED_SOUNDS.get_mut().contains(sound.location()));
        }
        return true;
    }

}

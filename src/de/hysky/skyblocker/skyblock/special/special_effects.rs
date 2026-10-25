//! Ported from de.hysky.skyblocker.skyblock.special.SpecialEffects by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::core::particles::simple_particle_type::SimpleParticleType;
use mcrust::net::minecraft::world::entity::entity::Entity as _;

/// java: public static WeakReference<ItemStack> ITEM = new WeakReference<>(ItemStack.EMPTY);
/// (the referent: a `duplicate` of the stack keeps Java's identity, which is what `ITEM.get() != itemActivationItem`
/// compares; the reference not keeping the stack alive is not observable here)
pub static ITEM: compat::MainThread<mcrust::net::minecraft::world::item::item_stack::ItemStack> = compat::MainThread::new(|| mcrust::net::minecraft::world::item::item_stack::empty());

/// java: public class SpecialEffects
#[derive(Clone)]
pub struct SpecialEffects {}

impl SpecialEffects {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: protected static void displaySpecialEffect(ItemStack stack, SimpleParticleType particle)
    pub fn display_special_effect(stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, particle: &Arc<SimpleParticleType>) {
        // java: ITEM = new WeakReference<>(SkyblockerConfigManager.get().general.specialEffects.displayItemName ? stack : ItemStack.EMPTY);
        *ITEM.get_mut() = if crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().general.special_effects.display_item_name { stack.duplicate() } else { mcrust::net::minecraft::world::item::item_stack::empty() };
        // java (26.2): client.gameRenderer.displayItemActivation(stack); - 26.3 moved it to the player
        // (`LocalPlayer.displayItemActivation:244-246`, `itemActivation.activate(itemStack, this.random)`)
        mcrust::mcrust::modding::game::with_player(|player| player.display_item_activation(stack));
        if mcrust::mcrust::modding::game::with_player(|_| ()).is_some() {
            // java: client.particleEngine.createTrackingEmitter(client.player, particle, 30);
            // (the emitter holds the level's shared handle of the player entity, found by its id)
            let player_id: Option<i32> = mcrust::mcrust::modding::game::with_player(|player| player.base.base.base.base.get_id());
            if let (Some(level), Some(player_id)) = (mcrust::mcrust::modding::game::level(), player_id)
                && let Some(entity) = level.get_entity(player_id)
                && let Some(engine) = compat::minecraft().particle_engine.as_mut()
            {
                engine.create_tracking_emitter_with_life_time(entity, Arc::clone(particle) as Arc<dyn mcrust::net::minecraft::core::particles::particle_options::ParticleOptions>, 30);
            }
            mcrust::mcrust::modding::game::with_player(|player| { player.play_sound(mcrust::net::minecraft::sounds::sound_events::ui_toast_challenge_complete().clone(), (1) as f32, 1.0f32); });
        }
    }

}

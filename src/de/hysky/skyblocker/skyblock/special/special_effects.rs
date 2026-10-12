//! Ported from de.hysky.skyblocker.skyblock.special.SpecialEffects by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::core::particles::simple_particle_type::SimpleParticleType;
use mcrust::net::minecraft::world::entity::entity::Entity as _;

// TODO(fabric2seer): `new WeakReference(..)`
//   java: public static WeakReference<ItemStack> ITEM = new WeakReference<>(ItemStack.EMPTY);
/// java: public static WeakReference<ItemStack> ITEM = new WeakReference<>(ItemStack.EMPTY);
pub static ITEM: compat::MainThread<crate::unresolved::WeakReference> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `ITEM` - see the TODO above"));

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
    pub fn display_special_effect(stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, particle: &SimpleParticleType) {
        // TODO(fabric2seer): `new WeakReference(..)`
        //   java: ITEM = new WeakReference<>(SkyblockerConfigManager.get().general.specialEffects.displayItemName ? stack : ItemStack.EMPTY);
        compat::minecraft().game_renderer.display_item_activation(stack.clone());
        if mcrust::mcrust::modding::game::with_player(|_| ()).is_some() {
            // TODO(fabric2seer): the mod keeps the game's `LocalPlayer` (Java holds a reference to the live object); the port's `LocalPlayer` is not a value to copy and the port lends the mod no handle to it
            // TODO(fabric2seer): the mod keeps the game's `SimpleParticleType` (Java holds a reference to the live object); the port's `SimpleParticleType` is not a value to copy and the port lends the mod no handle to it
            // rust: mcrust::mcrust::modding::game::with_player(|player| { compat::minecraft().particle_engine.as_mut().unwrap().create_tracking_emitter_with_life_time(player.clone(), particle.clone(), 30); });
            mcrust::mcrust::modding::game::with_player(|player| { player.play_sound(mcrust::net::minecraft::sounds::sound_events::ui_toast_challenge_complete().clone(), (1) as f32, 1.0f32); });
        }
    }

}

//! Ported from de.hysky.skyblocker.skyblock.entity.glow.adder.HubGlowAdder by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::entity::mob_glow_adder::MobGlowAdder;
use crate::de::hysky::skyblocker::skyblock::entity::mob_glow_adder::MobGlowAdderBase;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::net::minecraft::world::entity::entity::Entity;

/// java: @SuppressWarnings("unused") private static final HubGlowAdder INSTANCE = new HubGlowAdder();
pub static INSTANCE: compat::MainThread<compat::Shared<HubGlowAdder>> = compat::MainThread::new(|| HubGlowAdder::new());

/// java: public class HubGlowAdder extends MobGlowAdder
#[derive(Clone)]
pub struct HubGlowAdder {
    /// java: extends MobGlowAdder
    pub base: MobGlowAdderBase,
}

impl std::ops::Deref for HubGlowAdder {
    type Target = MobGlowAdderBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for HubGlowAdder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl HubGlowAdder {
    /// java: the implicit no-argument constructor
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = MobGlowAdderBase::new_in(move |__base| {
            let __h = compat::shared(Self {
                base: __base,
            });
            ((__h.clone() as compat::Shared<dyn MobGlowAdder>), __h)
        });
        __this
    }

    /// java: @Init public static void init()
    pub fn init() {
    }

}

impl MobGlowAdder for HubGlowAdder {
    fn mob_glow_adder_base(&self) -> &MobGlowAdderBase {
        &self.base
    }
    fn mob_glow_adder_base_mut(&mut self) -> &mut MobGlowAdderBase {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public int computeColour(Entity entity) (dispatch: the body for this class)
    fn compute_colour(&self) -> fn(&compat::Shared<dyn MobGlowAdder>, &dyn Entity) -> i32 {
        HubGlowAdder::compute_colour_dyn
    }

    /// java: @Override public boolean isEnabled() (dispatch: the body for this class)
    fn is_enabled(&self) -> fn(&compat::Shared<dyn MobGlowAdder>) -> bool {
        HubGlowAdder::is_enabled_dyn
    }

}

impl HubGlowAdder {
    /// java: @Override public int computeColour(Entity entity)
    pub fn compute_colour_impl(__this: &compat::Shared<Self>, entity: &dyn Entity) -> i32 {
        // java: return switch (entity) { case Zombie zombie when ZombieShootout.isInZombieShootout() -> ZombieShootout.getZombieGlowColor(zombie);
        //   case ArmorStand armorStand when CatchAFish.isInCatchAFish() -> CatchAFish.getFishGlowColor(armorStand); default -> NO_GLOW; };
        use crate::de::hysky::skyblocker::skyblock::carnival::catch_a_fish::CatchAFish;
        use crate::de::hysky::skyblocker::skyblock::carnival::zombie_shootout::ZombieShootout;
        if let Some(zombie) = entity.as_any().unwrap_or(&() as &dyn std::any::Any).downcast_ref::<mcrust::net::minecraft::world::entity::monster::zombie::zombie::Zombie>() && ZombieShootout::is_in_zombie_shootout() {
            return ZombieShootout::get_zombie_glow_color(zombie);
        }
        if let Some(armor_stand) = entity.as_any().unwrap_or(&() as &dyn std::any::Any).downcast_ref::<mcrust::net::minecraft::world::entity::decoration::armor_stand::ArmorStand>() && CatchAFish::is_in_catch_a_fish() {
            return CatchAFish::get_fish_glow_color(armor_stand);
        }
        return *crate::de::hysky::skyblocker::skyblock::entity::mob_glow::NO_GLOW;
    }

    /// HubGlowAdder.computeColour as the dispatch of MobGlowAdder.computeColour
    pub fn compute_colour_dyn(__this: &compat::Shared<dyn MobGlowAdder>, __p0: &dyn Entity) -> i32 {
        Self::compute_colour_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public boolean isEnabled()
    pub fn is_enabled_impl(__this: &compat::Shared<Self>) -> bool {
        return Utils::is_in_hub();
    }

    /// HubGlowAdder.isEnabled as the dispatch of MobGlowAdder.isEnabled
    pub fn is_enabled_dyn(__this: &compat::Shared<dyn MobGlowAdder>) -> bool {
        Self::is_enabled_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}

//! Ported from de.hysky.skyblocker.skyblock.entity.glow.adder.CrimsonGlowAdder by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::entity::mob_glow_adder::MobGlowAdder;
use crate::de::hysky::skyblocker::skyblock::entity::mob_glow_adder::MobGlowAdderBase;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::net::minecraft::world::entity::entity::Entity;
use mcrust::net::minecraft::world::entity::monster::zombie::zombie::Zombie;

/// java: @SuppressWarnings("unused") private static final CrimsonGlowAdder INSTANCE = new CrimsonGlowAdder();
pub static INSTANCE: compat::MainThread<compat::Shared<CrimsonGlowAdder>> = compat::MainThread::new(|| CrimsonGlowAdder::new());

/// java: public class CrimsonGlowAdder extends MobGlowAdder
#[derive(Clone)]
pub struct CrimsonGlowAdder {
    /// java: extends MobGlowAdder
    pub base: MobGlowAdderBase,
}

impl std::ops::Deref for CrimsonGlowAdder {
    type Target = MobGlowAdderBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for CrimsonGlowAdder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl CrimsonGlowAdder {
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

impl MobGlowAdder for CrimsonGlowAdder {
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
        CrimsonGlowAdder::compute_colour_dyn
    }

    /// java: @Override public boolean isEnabled() (dispatch: the body for this class)
    fn is_enabled(&self) -> fn(&compat::Shared<dyn MobGlowAdder>) -> bool {
        CrimsonGlowAdder::is_enabled_dyn
    }

}

impl CrimsonGlowAdder {
    /// java: @Override public int computeColour(Entity entity)
    pub fn compute_colour_impl(__this: &compat::Shared<Self>, entity: &dyn Entity) -> i32 {
        // java: return entity instanceof Zombie zombie && DojoManager.inArena && DojoManager.shouldGlow(MobGlow.getArmorStandName(zombie)) ? DojoManager.getColor() : NO_GLOW;
        use crate::de::hysky::skyblocker::skyblock::crimson::dojo::dojo_manager::{self, DojoManager};
        if entity.as_any().unwrap_or(&() as &dyn std::any::Any).is::<mcrust::net::minecraft::world::entity::monster::zombie::zombie::Zombie>() && compat::get_state(&dojo_manager::IN_ARENA) && DojoManager::should_glow(&crate::de::hysky::skyblocker::skyblock::entity::mob_glow::MobGlow::get_armor_stand_name(entity)) {
            return DojoManager::get_color();
        }
        return *crate::de::hysky::skyblocker::skyblock::entity::mob_glow::NO_GLOW;
    }

    /// CrimsonGlowAdder.computeColour as the dispatch of MobGlowAdder.computeColour
    pub fn compute_colour_dyn(__this: &compat::Shared<dyn MobGlowAdder>, __p0: &dyn Entity) -> i32 {
        Self::compute_colour_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public boolean isEnabled()
    pub fn is_enabled_impl(__this: &compat::Shared<Self>) -> bool {
        return Utils::is_in_crimson();
    }

    /// CrimsonGlowAdder.isEnabled as the dispatch of MobGlowAdder.isEnabled
    pub fn is_enabled_dyn(__this: &compat::Shared<dyn MobGlowAdder>) -> bool {
        Self::is_enabled_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}

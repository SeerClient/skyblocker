//! Ported from de.hysky.skyblocker.skyblock.entity.MobGlowAdder by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::entity::mob_glow;
use crate::de::hysky::skyblocker::skyblock::entity::mob_glow::MobGlow;
use mcrust::net::minecraft::world::entity::entity::Entity;

/// java: protected static final int NO_GLOW = MobGlow.NO_GLOW;
pub static NO_GLOW: LazyLock<i32> = LazyLock::new(|| *mob_glow::NO_GLOW);

/// java: public abstract class MobGlowAdder
#[derive(Clone)]
pub struct MobGlowAdderBase {}

impl MobGlowAdderBase {
    /// java: protected MobGlowAdder() (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_in<__R>(__wrap: impl FnOnce(Self) -> (compat::Shared<dyn MobGlowAdder>, __R)) -> (compat::Shared<dyn MobGlowAdder>, __R) {
        let this = Self {
        };
        let (__h, __r) = __wrap(this);
        let __this = __h;
        MobGlow::register_glow_adder(__this.clone());
        (__this, __r)
    }

}

/// java: the virtual methods of MobGlowAdder (implemented by MobGlowAdderBase and every subclass)
pub trait MobGlowAdder: compat::DynAny {
    fn mob_glow_adder_base(&self) -> &MobGlowAdderBase;
    fn mob_glow_adder_base_mut(&mut self) -> &mut MobGlowAdderBase;
    fn as_any(&self) -> &dyn std::any::Any;

    /// java: public abstract int computeColour(Entity entity); (dispatch: the body for this class)
    fn compute_colour(&self) -> fn(&compat::Shared<dyn MobGlowAdder>, &dyn Entity) -> i32;

    /// java: public abstract boolean isEnabled(); (dispatch: the body for this class)
    fn is_enabled(&self) -> fn(&compat::Shared<dyn MobGlowAdder>) -> bool;

}

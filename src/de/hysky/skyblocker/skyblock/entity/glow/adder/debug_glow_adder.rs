//! Ported from de.hysky.skyblocker.skyblock.entity.glow.adder.DebugGlowAdder by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::debug::snapshot_debug::SnapshotDebug;
use crate::de::hysky::skyblocker::skyblock::entity::mob_glow_adder;
use crate::de::hysky::skyblocker::skyblock::entity::mob_glow_adder::MobGlowAdder;
use crate::de::hysky::skyblocker::skyblock::entity::mob_glow_adder::MobGlowAdderBase;
use mcrust::net::minecraft::world::entity::animal::chicken::chicken::Chicken;
use mcrust::net::minecraft::world::entity::entity::Entity;
use mcrust::net::minecraft::world::entity::living_entity::LivingEntity as _;

/// java: @SuppressWarnings("unused") private static final DebugGlowAdder INSTANCE = new DebugGlowAdder();
pub static INSTANCE: compat::MainThread<compat::Shared<DebugGlowAdder>> = compat::MainThread::new(|| DebugGlowAdder::new());

/// java: public class DebugGlowAdder extends MobGlowAdder
#[derive(Clone)]
pub struct DebugGlowAdder {
    /// java: extends MobGlowAdder
    pub base: MobGlowAdderBase,
}

impl std::ops::Deref for DebugGlowAdder {
    type Target = MobGlowAdderBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for DebugGlowAdder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl DebugGlowAdder {
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

impl MobGlowAdder for DebugGlowAdder {
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
        DebugGlowAdder::compute_colour_dyn
    }

    /// java: @Override public boolean isEnabled() (dispatch: the body for this class)
    fn is_enabled(&self) -> fn(&compat::Shared<dyn MobGlowAdder>) -> bool {
        DebugGlowAdder::is_enabled_dyn
    }

}

impl DebugGlowAdder {
    /// java: @Override public int computeColour(Entity entity)
    pub fn compute_colour_impl(__this: &compat::Shared<Self>, entity: &dyn Entity) -> i32 {
        if let Some(chicken) = entity.as_any().unwrap_or(&() as &dyn std::any::Any).downcast_ref::<Chicken>() && chicken.is_baby() {
            // TODO(fabric2seer): `.getRGB(..)` on a value of unknown type
            //   java: return Color.YELLOW.getRGB();
        }
        return *mob_glow_adder::NO_GLOW;
    }

    /// DebugGlowAdder.computeColour as the dispatch of MobGlowAdder.computeColour
    pub fn compute_colour_dyn(__this: &compat::Shared<dyn MobGlowAdder>, __p0: &dyn Entity) -> i32 {
        Self::compute_colour_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public boolean isEnabled()
    pub fn is_enabled_impl(__this: &compat::Shared<Self>) -> bool {
        return (crate::de::hysky::skyblocker::debug::debug::Debug::debug_enabled() && SnapshotDebug::is_in_snapshot());
    }

    /// DebugGlowAdder.isEnabled as the dispatch of MobGlowAdder.isEnabled
    pub fn is_enabled_dyn(__this: &compat::Shared<dyn MobGlowAdder>) -> bool {
        Self::is_enabled_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}

//! Ported from de.hysky.skyblocker.skyblock.entity.glow.adder.GalateaGlowAdder by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::entity::mob_glow_adder::MobGlowAdder;
use crate::de::hysky::skyblocker::skyblock::entity::mob_glow_adder::MobGlowAdderBase;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::net::minecraft::world::entity::entity::Entity;

/// java: @SuppressWarnings("unused") private static final GalateaGlowAdder INSTANCE = new GalateaGlowAdder();
pub static INSTANCE: compat::MainThread<compat::Shared<GalateaGlowAdder>> = compat::MainThread::new(|| GalateaGlowAdder::new());

/// java: public class GalateaGlowAdder extends MobGlowAdder
#[derive(Clone)]
pub struct GalateaGlowAdder {
    /// java: extends MobGlowAdder
    pub base: MobGlowAdderBase,
}

impl std::ops::Deref for GalateaGlowAdder {
    type Target = MobGlowAdderBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for GalateaGlowAdder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl GalateaGlowAdder {
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

impl MobGlowAdder for GalateaGlowAdder {
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
        GalateaGlowAdder::compute_colour_dyn
    }

    /// java: @Override public boolean isEnabled() (dispatch: the body for this class)
    fn is_enabled(&self) -> fn(&compat::Shared<dyn MobGlowAdder>) -> bool {
        GalateaGlowAdder::is_enabled_dyn
    }

}

impl GalateaGlowAdder {
    /// java: @Override public int computeColour(Entity entity)
    pub fn compute_colour_impl(__this: &compat::Shared<Self>, entity: &dyn Entity) -> i32 {
        // TODO(fabric2seer): a pattern switch expression
        //   java: return switch (entity) {
        //   java: 			case Shulker shulker when shulker.getColor() == DyeColor.GREEN && SkyblockerConfigManager.get().hunting.moongladeMobs.highlightHideonleaf -> SkyblockerConfigManager.get().hunting.moongladeMobs.hideonleafGlowColor.getRGB();
        //   java: 			case Turtle _ when SkyblockerConfigManager.get().hunting.moongladeMobs.highlightShellwise -> SkyblockerConfigManager.get().hunting.moongladeMobs.shellwiseGlowColor.getRGB();
        //   java: 			case Axolotl _ when SkyblockerConfigManager.get().hunting.moongladeMobs.highlightCoralot -> SkyblockerConfigManager.get().hunting.moongladeMobs.coralotGlowColor.getRGB();
        //   java: 			default -> NO_GLOW;
        //   java: 		};
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// GalateaGlowAdder.computeColour as the dispatch of MobGlowAdder.computeColour
    pub fn compute_colour_dyn(__this: &compat::Shared<dyn MobGlowAdder>, __p0: &dyn Entity) -> i32 {
        Self::compute_colour_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public boolean isEnabled()
    pub fn is_enabled_impl(__this: &compat::Shared<Self>) -> bool {
        return Utils::is_in_galatea();
    }

    /// GalateaGlowAdder.isEnabled as the dispatch of MobGlowAdder.isEnabled
    pub fn is_enabled_dyn(__this: &compat::Shared<dyn MobGlowAdder>) -> bool {
        Self::is_enabled_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}

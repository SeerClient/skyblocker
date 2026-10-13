//! Ported from de.hysky.skyblocker.skyblock.entity.glow.adder.TorrhusCanyonGlowAdder by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::entity::mob_glow_adder::MobGlowAdder;
use crate::de::hysky::skyblocker::skyblock::entity::mob_glow_adder::MobGlowAdderBase;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::net::minecraft::world::entity::entity::Entity;

/// java: @SuppressWarnings("unused") private static final TorrhusCanyonGlowAdder INSTANCE = new TorrhusCanyonGlowAdder();
pub static INSTANCE: compat::MainThread<compat::Shared<TorrhusCanyonGlowAdder>> = compat::MainThread::new(|| TorrhusCanyonGlowAdder::new());

/// java: public class TorrhusCanyonGlowAdder extends MobGlowAdder
#[derive(Clone)]
pub struct TorrhusCanyonGlowAdder {
    /// java: extends MobGlowAdder
    pub base: MobGlowAdderBase,
}

impl std::ops::Deref for TorrhusCanyonGlowAdder {
    type Target = MobGlowAdderBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for TorrhusCanyonGlowAdder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl TorrhusCanyonGlowAdder {
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

impl MobGlowAdder for TorrhusCanyonGlowAdder {
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
        TorrhusCanyonGlowAdder::compute_colour_dyn
    }

    /// java: @Override public boolean isEnabled() (dispatch: the body for this class)
    fn is_enabled(&self) -> fn(&compat::Shared<dyn MobGlowAdder>) -> bool {
        TorrhusCanyonGlowAdder::is_enabled_dyn
    }

}

impl TorrhusCanyonGlowAdder {
    /// java: @Override public int computeColour(Entity entity)
    pub fn compute_colour_impl(__this: &compat::Shared<Self>, entity: &dyn Entity) -> i32 {
        // java: return switch (entity) { case Shulker shulker when ..highlightHideonsun && (color == YELLOW || ORANGE || BROWN) -> ..hideonsunHighlightColor.getRGB();
        //   case Parrot parrot when ..highlightBlueJay && parrot.getVariant() == Parrot.Variant.BLUE -> ..blueJayHighlightColor.getRGB(); default -> MobGlow.NO_GLOW; };
        use mcrust::net::minecraft::world::item::dye_color::DyeColor;
        let config = &crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().hunting.torrhus_mobs;
        if let Some(shulker) = entity.as_any().unwrap_or(&() as &dyn std::any::Any).downcast_ref::<mcrust::net::minecraft::world::entity::monster::shulker::Shulker>() && config.highlight_hideonsun && matches!(shulker.get_color(), Some(DyeColor::Yellow) | Some(DyeColor::Orange) | Some(DyeColor::Brown)) {
            return config.hideonsun_highlight_color.get_rgb();
        }
        if let Some(parrot) = entity.as_any().unwrap_or(&() as &dyn std::any::Any).downcast_ref::<mcrust::net::minecraft::world::entity::animal::parrot::parrot::Parrot>() && config.highlight_blue_jay && parrot.get_variant() == mcrust::net::minecraft::world::entity::animal::parrot::parrot::Variant::Blue {
            return config.blue_jay_highlight_color.get_rgb();
        }
        return *crate::de::hysky::skyblocker::skyblock::entity::mob_glow::NO_GLOW;
    }

    /// TorrhusCanyonGlowAdder.computeColour as the dispatch of MobGlowAdder.computeColour
    pub fn compute_colour_dyn(__this: &compat::Shared<dyn MobGlowAdder>, __p0: &dyn Entity) -> i32 {
        Self::compute_colour_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public boolean isEnabled()
    pub fn is_enabled_impl(__this: &compat::Shared<Self>) -> bool {
        return Utils::is_in_torrhus_canyon();
    }

    /// TorrhusCanyonGlowAdder.isEnabled as the dispatch of MobGlowAdder.isEnabled
    pub fn is_enabled_dyn(__this: &compat::Shared<dyn MobGlowAdder>) -> bool {
        Self::is_enabled_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}

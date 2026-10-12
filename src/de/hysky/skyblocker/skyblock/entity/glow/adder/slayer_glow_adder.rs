//! Ported from de.hysky.skyblocker.skyblock.entity.glow.adder.SlayerGlowAdder by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::configs::slayers_config::HighlightSlayerEntities;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::entity::mob_glow;
use crate::de::hysky::skyblocker::skyblock::entity::mob_glow_adder::MobGlowAdder;
use crate::de::hysky::skyblocker::skyblock::entity::mob_glow_adder::MobGlowAdderBase;
use crate::de::hysky::skyblocker::skyblock::item::head_textures;
use crate::de::hysky::skyblocker::skyblock::slayers::boss::demonlord::attunement_colors::AttunementColors;
use crate::de::hysky::skyblocker::skyblock::slayers::slayer_manager::SlayerManager;
use crate::de::hysky::skyblocker::skyblock::slayers::slayer_type::SlayerType;
use crate::de::hysky::skyblocker::utils::item_utils::ItemUtils;
use mcrust::net::minecraft::world::entity::decoration::armor_stand::ArmorStand;
use mcrust::net::minecraft::world::entity::entity::Entity;
use mcrust::net::minecraft::world::entity::equipment_slot::EquipmentSlot;
use mcrust::net::minecraft::world::entity::living_entity::LivingEntity;
use mcrust::net::minecraft::world::entity::monster::blaze::Blaze;
use mcrust::net::minecraft::world::entity::monster::skeleton::wither_skeleton::WitherSkeleton;
use mcrust::net::minecraft::world::entity::monster::zombie::zombified_piglin::ZombifiedPiglin;

/// java: @SuppressWarnings("unused") private static final SlayerGlowAdder INSTANCE = new SlayerGlowAdder();
pub static INSTANCE: compat::MainThread<compat::Shared<SlayerGlowAdder>> = compat::MainThread::new(|| SlayerGlowAdder::new());
/// java: private static final int NUKEKUBI_COLOUR = 0xFF00FF00;
pub const NUKEKUBI_COLOUR: i32 = 0xFF00FF00u32 as i32;

/// java: public class SlayerGlowAdder extends MobGlowAdder
#[derive(Clone)]
pub struct SlayerGlowAdder {
    /// java: extends MobGlowAdder
    pub base: MobGlowAdderBase,
}

impl std::ops::Deref for SlayerGlowAdder {
    type Target = MobGlowAdderBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for SlayerGlowAdder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl SlayerGlowAdder {
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

    /// java: private static boolean isNukekubiHead(ArmorStand entity)
    pub fn is_nukekubi_head(entity: &ArmorStand) -> bool {
        return (entity.has_item_in_slot(EquipmentSlot::Head) && ItemUtils::get_head_texture(entity.get_item_by_slot(EquipmentSlot::Head)) == head_textures::NUKEKUBI);
    }

}

impl MobGlowAdder for SlayerGlowAdder {
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
        SlayerGlowAdder::compute_colour_dyn
    }

    /// java: @Override public boolean isEnabled() (dispatch: the body for this class)
    fn is_enabled(&self) -> fn(&compat::Shared<dyn MobGlowAdder>) -> bool {
        SlayerGlowAdder::is_enabled_dyn
    }

}

impl SlayerGlowAdder {
    /// java: @Override public int computeColour(Entity entity)
    pub fn compute_colour_impl(__this: &compat::Shared<Self>, entity: &dyn Entity) -> i32 {
        if SkyblockerConfigManager::get().slayers.blaze_slayer.attunement_highlights && SlayerManager::is_fighting_slayer_type(SlayerType::Demonlord) && entity.as_any().unwrap_or(&() as &dyn std::any::Any).is::<Blaze>() || entity.as_any().unwrap_or(&() as &dyn std::any::Any).is::<WitherSkeleton>() || entity.as_any().unwrap_or(&() as &dyn std::any::Any).is::<ZombifiedPiglin>() {
            let mut color: i32 = AttunementColors::get_color(entity);
            if color != *mob_glow::NO_GLOW {
                return color;
            }
        }
        if SlayerManager::should_glow(entity, HighlightSlayerEntities::Glow) {
            // TODO(fabric2seer): `.getRGB(..)` on a Color
            //   java: return SkyblockerConfigManager.get().slayers.highlightColor.getRGB();
        }
        if SkyblockerConfigManager::get().slayers.enderman_slayer.highlight_nukekubi_heads && SlayerManager::is_fighting_slayer_type(SlayerType::Voidgloom) && let Some(r#as) = entity.as_any().unwrap_or(&() as &dyn std::any::Any).downcast_ref::<ArmorStand>() && (r#as).is_marker() && SlayerGlowAdder::is_nukekubi_head(r#as) {
            return NUKEKUBI_COLOUR;
        }
        return *mob_glow::NO_GLOW;
    }

    /// SlayerGlowAdder.computeColour as the dispatch of MobGlowAdder.computeColour
    pub fn compute_colour_dyn(__this: &compat::Shared<dyn MobGlowAdder>, __p0: &dyn Entity) -> i32 {
        Self::compute_colour_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public boolean isEnabled()
    pub fn is_enabled_impl(__this: &compat::Shared<Self>) -> bool {
        return (SlayerManager::is_in_slayer_quest() || SlayerManager::is_fighting_slayer());
    }

    /// SlayerGlowAdder.isEnabled as the dispatch of MobGlowAdder.isEnabled
    pub fn is_enabled_dyn(__this: &compat::Shared<dyn MobGlowAdder>) -> bool {
        Self::is_enabled_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}

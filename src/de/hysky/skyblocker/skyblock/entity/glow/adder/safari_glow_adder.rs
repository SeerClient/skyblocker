//! Ported from de.hysky.skyblocker.skyblock.entity.glow.adder.SafariGlowAdder by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::configs::hunting_config::HuntingConfig;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::entity::mob_glow::MobGlow;
use crate::de::hysky::skyblocker::skyblock::entity::mob_glow_adder::MobGlowAdder;
use crate::de::hysky::skyblocker::skyblock::entity::mob_glow_adder::MobGlowAdderBase;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::world::entity::decoration::armor_stand::ArmorStand;
use mcrust::net::minecraft::world::entity::display::ItemDisplay;
use mcrust::net::minecraft::world::entity::display::TextDisplay;
use mcrust::net::minecraft::world::entity::entity::Entity;
use mcrust::net::minecraft::world::entity::monster::shulker::Shulker;
use mcrust::net::minecraft::world::item::dye_color::DyeColor;
use mcrust::net::minecraft::world::item::items;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: @SuppressWarnings("unused") private static final SafariGlowAdder INSTANCE = new SafariGlowAdder();
pub static INSTANCE: compat::MainThread<compat::Shared<SafariGlowAdder>> = compat::MainThread::new(|| SafariGlowAdder::new());

/// java: public class SafariGlowAdder extends MobGlowAdder
#[derive(Clone)]
pub struct SafariGlowAdder {
    /// java: extends MobGlowAdder
    pub base: MobGlowAdderBase,
}

impl std::ops::Deref for SafariGlowAdder {
    type Target = MobGlowAdderBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for SafariGlowAdder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl SafariGlowAdder {
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

    /// java: private static boolean isNotDuplico(ItemDisplay display)
    pub fn is_not_duplico(display: &ItemDisplay) -> bool {
        let mut stack: mcrust::net::minecraft::world::item::item_stack::ItemStack = display.get_item_stack();
        return (!(stack.is_item(&items::items().get("player_head"))) && !(stack.is_item(mcrust::net::minecraft::world::item::items::dyed_shulker_box().purple())));
    }

    /// java: private static boolean isSparkling(Entity entity)
    pub fn is_sparkling(entity: &dyn Entity) -> bool {
        if entity.as_any().unwrap_or(&() as &dyn std::any::Any).is::<ArmorStand>() || entity.as_any().unwrap_or(&() as &dyn std::any::Any).is::<TextDisplay>() {
            return false;
        }
        if let Some(shulker) = entity.as_any().unwrap_or(&() as &dyn std::any::Any).downcast_ref::<Shulker>() && shulker.get_color() == Some(DyeColor::Purple) {
            return false;
        }
        if let Some(display) = entity.as_any().unwrap_or(&() as &dyn std::any::Any).downcast_ref::<ItemDisplay>() {
            let mut stack: mcrust::net::minecraft::world::item::item_stack::ItemStack = display.get_item_stack();
            if !(stack.is_item(&items::items().get("player_head"))) && !(stack.is_item(mcrust::net::minecraft::world::item::items::dyed_shulker_box().green())) {
                return false;
            }
        }
        // the stands by id
        let mut armor_stands: Vec<i32> = MobGlow::get_armor_stands(entity);
        let mut custom_name: Option<MutableComponent> = (if !armor_stands.is_empty() { mcrust::mcrust::modding::game::with_entity(armor_stands[0], |e| e.base().get_custom_name().map(compat::component_of)).flatten() } else { None });
        return (custom_name.is_some() && custom_name.as_ref().unwrap().get_string().contains("SPARKLING"));
    }

}

impl MobGlowAdder for SafariGlowAdder {
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
        SafariGlowAdder::compute_colour_dyn
    }

    /// java: @Override public boolean isEnabled() (dispatch: the body for this class)
    fn is_enabled(&self) -> fn(&compat::Shared<dyn MobGlowAdder>) -> bool {
        SafariGlowAdder::is_enabled_dyn
    }

}

impl SafariGlowAdder {
    /// java: @Override public int computeColour(Entity entity)
    pub fn compute_colour_impl(__this: &compat::Shared<Self>, entity: &dyn Entity) -> i32 {
        let hunting_config = &SkyblockerConfigManager::get().hunting;
        // java: return switch (entity) { .. } - in Java's case order
        use crate::de::hysky::skyblocker::skyblock::hunting::safari::safari_utils::SafariUtils;
        use mcrust::net::minecraft::world::entity::display::Display as _;
        let any = entity.as_any().unwrap_or(&() as &dyn std::any::Any);
        //Critter Capsule
        if crate::de::hysky::skyblocker::skyblock::foraging::torrhus::critter_capsule_helper::HIGHLIGHTED.get().is_some_and(|__id| __id == entity.base().get_id()) {
            return hunting_config.safari.critter_capsule_helper_color.get_rgb();
        }
        // Cavern Biome
        if let Some(display) = any.downcast_ref::<ItemDisplay>() && hunting_config.cavern_biome.highlight_rockmite_mounds && SafariUtils::is_in_cavern_biome() && crate::de::hysky::skyblocker::utils::item_utils::ItemUtils::get_head_texture(&display.get_item_stack()) == crate::de::hysky::skyblocker::skyblock::item::head_textures::ROCKMITE_MOUND && display.get_pos_rot_interpolation_duration() == 0 {
            return hunting_config.cavern_biome.rockmite_mound_highlight_color.get_rgb();
        }
        // Forest Biome
        if let Some(shulker) = any.downcast_ref::<Shulker>() && hunting_config.forest_biome.highlight_hideonfloor && SafariUtils::is_in_forest_biome() && shulker.get_color() == Some(DyeColor::Green) {
            return hunting_config.forest_biome.hideonfloor_highlight_color.get_rgb();
        }
        if let Some(display) = any.downcast_ref::<ItemDisplay>() && hunting_config.forest_biome.highlight_hideonfloor && SafariUtils::is_in_forest_biome() && display.get_item_stack().is_item(mcrust::net::minecraft::world::item::items::dyed_shulker_box().green()) {
            return hunting_config.forest_biome.hideonfloor_highlight_color.get_rgb();
        }
        // Haunted Biome
        if let Some(display) = any.downcast_ref::<ItemDisplay>() && hunting_config.haunted_biome.highlight_duplico && SafariUtils::is_in_haunted_biome() && display.get_pos_rot_interpolation_duration() == 3 && SafariGlowAdder::is_not_duplico(display) {
            return hunting_config.haunted_biome.duplico_highlight_color.get_rgb();
        }
        if any.is::<mcrust::net::minecraft::world::entity::ambient::bat::Bat>() && hunting_config.haunted_biome.highlight_bloodbat && SafariUtils::is_in_haunted_biome() {
            return hunting_config.haunted_biome.bloodbat_highlight_color.get_rgb();
        }
        // Sparkling Critters
        if hunting_config.safari.highlight_sparkling_critters && SafariGlowAdder::is_sparkling(entity) {
            return hunting_config.safari.sparkling_critter_highlight_color.get_rgb();
        }
        return *crate::de::hysky::skyblocker::skyblock::entity::mob_glow::NO_GLOW;
    }

    /// SafariGlowAdder.computeColour as the dispatch of MobGlowAdder.computeColour
    pub fn compute_colour_dyn(__this: &compat::Shared<dyn MobGlowAdder>, __p0: &dyn Entity) -> i32 {
        Self::compute_colour_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public boolean isEnabled()
    pub fn is_enabled_impl(__this: &compat::Shared<Self>) -> bool {
        return Utils::is_in_safari();
    }

    /// SafariGlowAdder.isEnabled as the dispatch of MobGlowAdder.isEnabled
    pub fn is_enabled_dyn(__this: &compat::Shared<dyn MobGlowAdder>) -> bool {
        Self::is_enabled_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}

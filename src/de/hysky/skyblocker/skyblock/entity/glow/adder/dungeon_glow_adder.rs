//! Ported from de.hysky.skyblocker.skyblock.entity.glow.adder.DungeonGlowAdder by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::entity::mob_glow::MobGlow;
use crate::de::hysky::skyblocker::skyblock::entity::mob_glow_adder::MobGlowAdder;
use crate::de::hysky::skyblocker::skyblock::entity::mob_glow_adder::MobGlowAdderBase;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::net::minecraft::world::entity::decoration::armor_stand::ArmorStand;
use mcrust::net::minecraft::world::entity::entity::Entity;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: public static final DungeonGlowAdder INSTANCE = new DungeonGlowAdder();
pub static INSTANCE: compat::MainThread<compat::Shared<DungeonGlowAdder>> = compat::MainThread::new(|| DungeonGlowAdder::new());
/// java: protected static final int STARRED_COLOUR = 0xF57738;
pub const STARRED_COLOUR: i32 = 0xF57738;
/// java: private static final int LOST_ADVENTURER_COLOUR = 0xFEE15C;
pub const LOST_ADVENTURER_COLOUR: i32 = 0xFEE15C;
/// java: private static final int SHADOW_ASSASSIN_COLOUR = 0x5B2CB2;
pub const SHADOW_ASSASSIN_COLOUR: i32 = 0x5B2CB2;
/// java: private static final int ANGRY_ARCHAEOLOGIST_COLOUR = 0x57C2F7;
pub const ANGRY_ARCHAEOLOGIST_COLOUR: i32 = 0x57C2F7;
/// java: private static final int ENDERMAN_EYE_COLOUR = 0xCC00FA;
pub const ENDERMAN_EYE_COLOUR: i32 = 0xCC00FA;

/// java: public class DungeonGlowAdder extends MobGlowAdder
#[derive(Clone)]
pub struct DungeonGlowAdder {
    /// java: extends MobGlowAdder
    pub base: MobGlowAdderBase,
}

impl std::ops::Deref for DungeonGlowAdder {
    type Target = MobGlowAdderBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for DungeonGlowAdder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl DungeonGlowAdder {
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

    /// java: public static boolean isStarred(Entity entity)
    pub fn is_starred(entity: &dyn Entity) -> bool {
        // the stands by id
        let mut armor_stands: Vec<i32> = MobGlow::get_armor_stands(entity);
        return (!armor_stands.is_empty() && mcrust::mcrust::modding::game::with_entity(armor_stands[0], |e| e.get_name().get_string().contains("✯")).unwrap_or(false));
    }

}

impl MobGlowAdder for DungeonGlowAdder {
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
        DungeonGlowAdder::compute_colour_dyn
    }

    /// java: @Override public boolean isEnabled() (dispatch: the body for this class)
    fn is_enabled(&self) -> fn(&compat::Shared<dyn MobGlowAdder>) -> bool {
        DungeonGlowAdder::is_enabled_dyn
    }

}

impl DungeonGlowAdder {
    /// java: @Override public int computeColour(Entity entity)
    pub fn compute_colour_impl(__this: &compat::Shared<Self>, entity: &dyn Entity) -> i32 {
        let mut name: String = compat::component_of(entity.get_name()).get_string();
        // java: return switch (entity) { .. } - in Java's case order
        use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::dungeon_manager::DungeonManager;
        use crate::de::hysky::skyblocker::skyblock::item::head_textures;
        use crate::de::hysky::skyblocker::utils::item_utils::ItemUtils;
        use mcrust::net::minecraft::world::entity::equipment_slot::EquipmentSlot;
        use mcrust::net::minecraft::world::entity::living_entity::LivingEntity as _;
        let config = crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get();
        let any = entity.as_any().unwrap_or(&() as &dyn std::any::Any);
        let player = entity.as_living_entity().and_then(|__l| __l.as_player());
        // Minibosses
        if player.is_some() && config.dungeons.starred_mob_glow && !DungeonManager::get_boss().is_floor(4) {
            match name.as_str() {
                "Lost Adventurer" => return LOST_ADVENTURER_COLOUR,
                "Shadow Assassin" => return SHADOW_ASSASSIN_COLOUR,
                "Diamond Guy" => return ANGRY_ARCHAEOLOGIST_COLOUR,
                _ => {}
            }
        }
        if player.is_some() && entity.base().get_id() == crate::de::hysky::skyblocker::skyblock::dungeon::livid_color::LividColor::get_correct_livid_id() && crate::de::hysky::skyblocker::skyblock::dungeon::livid_color::LividColor::should_glow(&name) {
            return crate::de::hysky::skyblocker::skyblock::dungeon::livid_color::LividColor::get_glow_color(&name);
        }
        // Bats
        if any.is::<mcrust::net::minecraft::world::entity::ambient::bat::Bat>() && config.dungeons.starred_mob_glow {
            return STARRED_COLOUR;
        }
        if let Some(r#as) = any.downcast_ref::<ArmorStand>() {
            // Fel Heads
            if config.dungeons.starred_mob_glow && r#as.is_marker() && r#as.has_item_in_slot(EquipmentSlot::Head) && ItemUtils::get_head_texture(r#as.get_item_by_slot(EquipmentSlot::Head)) == head_textures::FEL {
                return ENDERMAN_EYE_COLOUR;
            }
            // Wither & Blood Keys
            if config.dungeons.highlight_door_keys && r#as.has_item_in_slot(EquipmentSlot::Head) {
                let texture = ItemUtils::get_head_texture(r#as.get_item_by_slot(EquipmentSlot::Head));
                return if texture == head_textures::WITHER_KEY || texture == head_textures::BLOOD_KEY { mcrust::net::minecraft::world::item::dye_color::DyeColor::Cyan.get_text_color() } else { *crate::de::hysky::skyblocker::skyblock::entity::mob_glow::NO_GLOW };
            }
            // Armor Stands
            return 0;
        }
        // Regular Mobs
        if config.dungeons.starred_mob_glow && DungeonGlowAdder::is_starred(entity) {
            return STARRED_COLOUR;
        }
        // Class-based glow
        if let Some(p) = player && config.dungeons.class_based_player_glow && crate::de::hysky::skyblocker::skyblock::dungeon::dungeon_score::DungeonScore::is_dungeon_started() {
            return crate::de::hysky::skyblocker::skyblock::dungeon::secrets::dungeon_player_manager::DungeonPlayerManager::get_class_from_player(p).glow_color();
        }
        return *crate::de::hysky::skyblocker::skyblock::entity::mob_glow::NO_GLOW;
    }

    /// DungeonGlowAdder.computeColour as the dispatch of MobGlowAdder.computeColour
    pub fn compute_colour_dyn(__this: &compat::Shared<dyn MobGlowAdder>, __p0: &dyn Entity) -> i32 {
        Self::compute_colour_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public boolean isEnabled()
    pub fn is_enabled_impl(__this: &compat::Shared<Self>) -> bool {
        return Utils::is_in_dungeons();
    }

    /// DungeonGlowAdder.isEnabled as the dispatch of MobGlowAdder.isEnabled
    pub fn is_enabled_dyn(__this: &compat::Shared<dyn MobGlowAdder>) -> bool {
        Self::is_enabled_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}

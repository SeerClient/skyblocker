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
        // TODO(fabric2seer): a pattern switch expression
        //   java: return switch (entity) {
        //   java: 			// Minibosses
        //   java: 			case Player _ when SkyblockerConfigManager.get().dungeons.starredMobGlow && !DungeonManager.getBoss().isFloor(4) && name.equals("Lost Adventurer") -> LOST_ADVENTURER_COLOUR;
        //   java: 			case Player _ when SkyblockerConfigManager.get().dungeons.starredMobGlow && !DungeonManager.getBoss().isFloor(4) && name.equals("Shadow Assassin") -> SHADOW_ASSASSIN_COLOUR;
        //   java: 			case Player _ when SkyblockerConfigManager.get().dungeons.starredMobGlow && !DungeonManager.getBoss().isFloor(4) && name.equals("Diamond Guy") -> ANGRY_ARCHAEOLOGIST_COLOUR;
        //   java: 			case Player _ when entity.getId() == LividColor.getCorrectLividId() && LividColor.shouldGlow(name) -> LividColor.getGlowColor(name);
        //   java:
        //   java: 			// Bats
        //   java: 			case Bat _ when SkyblockerConfigManager.get().dungeons.starredMobGlow -> STARRED_COLOUR;
        //   java:
        //   java: 			// Fel Heads
        //   java: 			case ArmorStand as when SkyblockerConfigManager.get().dungeons.starredMobGlow && as.isMarker() && as.hasItemInSlot(EquipmentSlot.HEAD) && ItemUtils.getHeadTexture(as.getItemBySlot(EquipmentSlot.HEAD)).equals(HeadTextures.FEL) -> ENDERMAN_EYE_COLOUR;
        //   java:
        //   java: 			// Wither & Blood Keys
        //   java: 			case ArmorStand as when SkyblockerConfigManager.get().dungeons.highlightDoorKeys && as.hasItemInSlot(EquipmentSlot.HEAD) -> {
        //   java: 				yield switch (ItemUtils.getHeadTexture(as.getItemBySlot(EquipmentSlot.HEAD))) {
        //   java: 					case String s when s.equals(HeadTextures.WITHER_KEY) -> DyeColor.CYAN.getTextColor();
        //   java: 					case String s when s.equals(HeadTextures.BLOOD_KEY) -> DyeColor.CYAN.getTextColor();
        //   java: 					default -> NO_GLOW;
        //   java: 				};
        //   java: 			}
        //   java:
        //   java: 			// Armor Stands
        //   java: 			case ArmorStand _ -> 0;
        //   java:
        //   java: 			// Regular Mobs
        //   java: 			case Entity _ when SkyblockerConfigManager.get().dungeons.starredMobGlow && isStarred(entity) -> STARRED_COLOUR;
        //   java:
        //   java: 			//Class-based glow
        //   java: 			//This goes after regular mobs to ensure starred player entities like dreadlords have the glow applied
        //   java: 			case Player p when SkyblockerConfigManager.get().dungeons.classBasedPlayerGlow && DungeonScore.isDungeonStarted() -> DungeonPlayerManager.getClassFromPlayer(p).glowColor();
        //   java:
        //   java: 			default -> NO_GLOW;
        //   java: 		};
        todo!("fabric2seer: the value of this function is in a TODO above")
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

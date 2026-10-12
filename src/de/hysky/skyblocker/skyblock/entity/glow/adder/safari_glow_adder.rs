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
        if let Some(shulker) = entity.as_any().unwrap_or(&() as &dyn std::any::Any).downcast_ref::<Shulker>() && shulker.get_color().unwrap() == DyeColor::Purple {
            return false;
        }
        if let Some(display) = entity.as_any().unwrap_or(&() as &dyn std::any::Any).downcast_ref::<ItemDisplay>() {
            let mut stack: mcrust::net::minecraft::world::item::item_stack::ItemStack = display.get_item_stack();
            if !(stack.is_item(&items::items().get("player_head"))) && !(stack.is_item(mcrust::net::minecraft::world::item::items::dyed_shulker_box().green())) {
                return false;
            }
        }
        let mut armor_stands: Vec<ArmorStand> = MobGlow::get_armor_stands(entity);
        let mut custom_name: Option<MutableComponent> = (if !armor_stands.is_empty() { armor_stands[0].base.base.get_custom_name() } else { None });
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
        // TODO(fabric2seer): a pattern switch expression
        //   java: return switch (entity) {
        //   java: 			//Critter Capsule
        //   java: 			case Entity target when CritterCapsuleHelper.highlighted != null && CritterCapsuleHelper.highlighted.equals(target) -> huntingConfig.safari.critterCapsuleHelperColor.getRGB();
        //   java: 			// Cavern Biome
        //   java: 			case ItemDisplay display when huntingConfig.cavernBiome.highlightRockmiteMounds && SafariUtils.isInCavernBiome() && ItemUtils.getHeadTexture(display.getItemStack()).equals(HeadTextures.ROCKMITE_MOUND) && display.getPosRotInterpolationDuration() == 0 -> huntingConfig.cavernBiome.rockmiteMoundHighlightColor.getRGB();
        //   java:
        //   java: 			// Forest Biome
        //   java: 			case Shulker shulker when huntingConfig.forestBiome.highlightHideonfloor && SafariUtils.isInForestBiome() && shulker.getColor() == DyeColor.GREEN -> huntingConfig.forestBiome.hideonfloorHighlightColor.getRGB();
        //   java: 			case ItemDisplay display when huntingConfig.forestBiome.highlightHideonfloor && SafariUtils.isInForestBiome() && display.getItemStack().is(Items.DYED_SHULKER_BOX.green()) -> huntingConfig.forestBiome.hideonfloorHighlightColor.getRGB();
        //   java:
        //   java: 			// Haunted Biome
        //   java: 			case ItemDisplay display when huntingConfig.hauntedBiome.highlightDuplico && SafariUtils.isInHauntedBiome() && display.getPosRotInterpolationDuration() == 3 && isNotDuplico(display) -> huntingConfig.hauntedBiome.duplicoHighlightColor.getRGB();
        //   java: 			case Bat _ when huntingConfig.hauntedBiome.highlightBloodbat && SafariUtils.isInHauntedBiome() -> huntingConfig.hauntedBiome.bloodbatHighlightColor.getRGB();
        //   java:
        //   java: 			// Sparkling Critters
        //   java: 			case Entity _ when huntingConfig.safari.highlightSparklingCritters && isSparkling(entity) -> huntingConfig.safari.sparklingCritterHighlightColor.getRGB();
        //   java:
        //   java: 			default -> NO_GLOW;
        //   java: 		};
        todo!("fabric2seer: the value of this function is in a TODO above")
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

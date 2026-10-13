//! Ported from de.hysky.skyblocker.skyblock.entity.glow.adder.MushroomDesertGlowAdder by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::entity::mob_glow::MobGlow;
use crate::de::hysky::skyblocker::skyblock::entity::mob_glow_adder;
use crate::de::hysky::skyblocker::skyblock::entity::mob_glow_adder::MobGlowAdder;
use crate::de::hysky::skyblocker::skyblock::entity::mob_glow_adder::MobGlowAdderBase;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::net::minecraft::world::entity::entity::Entity;

/// java: @SuppressWarnings("unused") private static final MushroomDesertGlowAdder INSTANCE = new MushroomDesertGlowAdder();
pub static INSTANCE: compat::MainThread<compat::Shared<MushroomDesertGlowAdder>> = compat::MainThread::new(|| MushroomDesertGlowAdder::new());
/// java: private static final int TRACKABLE_COLOR = 0xFFFFFF;
pub const TRACKABLE_COLOR: i32 = 0xFFFFFF;
/// java: private static final int UNTRACKABLE_COLOR = 0x55FF55;
pub const UNTRACKABLE_COLOR: i32 = 0x55FF55;
/// java: private static final int UNDETECTED_COLOR = 0x5555FF;
pub const UNDETECTED_COLOR: i32 = 0x5555FF;
/// java: private static final int ENDANGERED_COLOR = 0xAA00AA;
pub const ENDANGERED_COLOR: i32 = 0xAA00AA;
/// java: private static final int ELUSIVE_COLOR = 0xFFAA00;
pub const ELUSIVE_COLOR: i32 = 0xFFAA00;

/// java: public class MushroomDesertGlowAdder extends MobGlowAdder
#[derive(Clone)]
pub struct MushroomDesertGlowAdder {
    /// java: extends MobGlowAdder
    pub base: MobGlowAdderBase,
}

impl std::ops::Deref for MushroomDesertGlowAdder {
    type Target = MobGlowAdderBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for MushroomDesertGlowAdder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl MushroomDesertGlowAdder {
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

    /// java: private static boolean isPeltAnimal(String name, String animal)
    pub fn is_pelt_animal(name: &str, animal: &str) -> bool {
        return (name.contains(&format!("Trackable {}", animal)) || name.contains(&format!("Untrackable {}", animal)) || name.contains(&format!("Undetected {}", animal)) || name.contains(&format!("Endangered {}", animal)) || name.contains(&format!("Elusive {}", animal)));
    }

    /// java: private int getGlowColor(String name)
    pub fn get_glow_color(__this: &compat::Shared<Self>, name: &str) -> i32 {
        if name.contains("Trackable") {
            return TRACKABLE_COLOR;
        }
        if name.contains("Untrackable") {
            return UNTRACKABLE_COLOR;
        }
        if name.contains("Undetected") {
            return UNDETECTED_COLOR;
        }
        if name.contains("Endangered") {
            return ENDANGERED_COLOR;
        }
        if name.contains("Elusive") {
            return ELUSIVE_COLOR;
        }
        return *mob_glow_adder::NO_GLOW;
    }

}

impl MobGlowAdder for MushroomDesertGlowAdder {
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
        MushroomDesertGlowAdder::compute_colour_dyn
    }

    /// java: @Override public boolean isEnabled() (dispatch: the body for this class)
    fn is_enabled(&self) -> fn(&compat::Shared<dyn MobGlowAdder>) -> bool {
        MushroomDesertGlowAdder::is_enabled_dyn
    }

}

impl MushroomDesertGlowAdder {
    /// java: @Override public int computeColour(Entity entity)
    pub fn compute_colour_impl(__this: &compat::Shared<Self>, entity: &dyn Entity) -> i32 {
        if !(SkyblockerConfigManager::get().other_locations.barn.enable_pelt_animal_highlighter) {
            return *mob_glow_adder::NO_GLOW;
        }
        let mut name: String = MobGlow::get_armor_stand_name(entity);
        // java: return switch (entity) { case Cow _ when isPeltAnimal(name, "Cow") -> getGlowColor(name); .. Pig, Sheep, Rabbit, Chicken, Horse ..; default -> NO_GLOW; };
        let any = entity.as_any().unwrap_or(&() as &dyn std::any::Any);
        let animal: Option<&str> = if any.is::<mcrust::net::minecraft::world::entity::animal::cow::cow::Cow>() { Some("Cow") }
            else if any.is::<mcrust::net::minecraft::world::entity::animal::pig::pig::Pig>() { Some("Pig") }
            else if any.is::<mcrust::net::minecraft::world::entity::animal::sheep::sheep::Sheep>() { Some("Sheep") }
            else if any.is::<mcrust::net::minecraft::world::entity::animal::rabbit::rabbit::Rabbit>() { Some("Rabbit") }
            else if any.is::<mcrust::net::minecraft::world::entity::animal::chicken::chicken::Chicken>() { Some("Chicken") }
            else if any.is::<mcrust::net::minecraft::world::entity::animal::equine::horse::Horse>() { Some("Horse") }
            else { None };
        if let Some(animal) = animal && MushroomDesertGlowAdder::is_pelt_animal(&name, animal) {
            return MushroomDesertGlowAdder::get_glow_color(__this, &name);
        }
        return *mob_glow_adder::NO_GLOW;
    }

    /// MushroomDesertGlowAdder.computeColour as the dispatch of MobGlowAdder.computeColour
    pub fn compute_colour_dyn(__this: &compat::Shared<dyn MobGlowAdder>, __p0: &dyn Entity) -> i32 {
        Self::compute_colour_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public boolean isEnabled()
    pub fn is_enabled_impl(__this: &compat::Shared<Self>) -> bool {
        return Utils::is_in_farm();
    }

    /// MushroomDesertGlowAdder.isEnabled as the dispatch of MobGlowAdder.isEnabled
    pub fn is_enabled_dyn(__this: &compat::Shared<dyn MobGlowAdder>) -> bool {
        Self::is_enabled_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}

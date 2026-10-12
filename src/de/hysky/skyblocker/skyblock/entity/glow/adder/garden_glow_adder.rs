//! Ported from de.hysky.skyblocker.skyblock.entity.glow.adder.GardenGlowAdder by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::entity::mob_glow_adder::MobGlowAdder;
use crate::de::hysky::skyblocker::skyblock::entity::mob_glow_adder::MobGlowAdderBase;
use crate::de::hysky::skyblocker::skyblock::garden::current_jacob_crop;
use crate::de::hysky::skyblocker::skyblock::garden::garden_constants;
use crate::de::hysky::skyblocker::skyblock::garden::vacuum_cache::VacuumCache;
use crate::de::hysky::skyblocker::skyblock::item::head_textures;
use crate::de::hysky::skyblocker::utils::item_utils::ItemUtils;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler::Scheduler;
use crate::de::hysky::skyblocker::utils::sky_block_icons;
use crate::de::hysky::skyblocker::utils::utils;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::net::minecraft::world::entity::decoration::armor_stand::ArmorStand;
use mcrust::net::minecraft::world::entity::entity::Entity;
use mcrust::net::minecraft::world::entity::equipment_slot::EquipmentSlot;
use mcrust::net::minecraft::world::entity::living_entity::LivingEntity as _;

/// java: private static final GardenGlowAdder INSTANCE = new GardenGlowAdder();
pub static INSTANCE: compat::MainThread<compat::Shared<GardenGlowAdder>> = compat::MainThread::new(|| GardenGlowAdder::new());
/// java: private static final int PEST_COLOUR = 0xB62F00;
pub const PEST_COLOUR: i32 = 0xB62F00;
/// java: private static final Pattern CURRENT_CROP_PATTERN = Pattern.compile(String.format("^ [○%s] (?<crop>.+) .+$", SkyBlockIcons.FARMING_FORTUNE));
pub static CURRENT_CROP_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile(&format!("^ [○{}] (?<crop>.+) .+$", sky_block_icons::FARMING_FORTUNE)));

/// java: public class GardenGlowAdder extends MobGlowAdder
#[derive(Clone)]
pub struct GardenGlowAdder {
    /// java: extends MobGlowAdder
    pub base: MobGlowAdderBase,
}

impl std::ops::Deref for GardenGlowAdder {
    type Target = MobGlowAdderBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for GardenGlowAdder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl GardenGlowAdder {
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
        ({ let __r = scheduler::INSTANCE.get_mut(); let __f = compat::dispatch(__r, |__t| __t.schedule_cyclic()); __f(__r, Arc::new(move || { GardenGlowAdder::update(); }), 20) });
    }

    /// java: private static boolean isPestHead(ArmorStand entity)
    pub fn is_pest_head(entity: &ArmorStand) -> bool {
        return (entity.has_item_in_slot(EquipmentSlot::Head) && head_textures::PEST_HEADS.contains(&ItemUtils::get_head_texture(entity.get_item_by_slot(EquipmentSlot::Head))));
    }

    /// java: private static void update()
    pub fn update() {
        if !({ let __r = INSTANCE.get_mut(); let __f = compat::dispatch(__r, |__t| __t.is_enabled()); __f(&compat::view(__r, |a| a as compat::Shared<dyn MobGlowAdder>)) }) || !compat::lock(&utils::STRING_SCOREBOARD).iter().cloned().any(|s| s.contains("Jacob's Contest")) {
            compat::set_state(&current_jacob_crop::CURRENT_CROP_CONTEST, None);
            return;
        }
        for line in compat::get_state(&utils::STRING_SCOREBOARD) {
            let mut matcher: compat::regex::Matcher = CURRENT_CROP_PATTERN.matcher(&line);
            if matcher.matches() {
                let mut crop: String = compat::trim(&matcher.group_named("crop").unwrap());
                if !(compat::get_state(&current_jacob_crop::CURRENT_CROP_CONTEST).as_deref() == Some(&*(crop))) {
                    compat::set_state(&current_jacob_crop::CURRENT_CROP_CONTEST, Some(crop.clone()));
                }
            }
        }
    }

    /// java: public static boolean doesPestMatchCurrentContest(ArmorStand entity)
    pub fn does_pest_match_current_contest(entity: &ArmorStand) -> bool {
        if !(GardenGlowAdder::contest_enabled(INSTANCE.get_mut())) {
            return false;
        }
        // TODO(fabric2seer): `.get(..)` on a Multimap
        //   java: return GardenConstants.PEST_HEAD_BY_CROP
        //   java: 				.get(CurrentJacobCrop.CURRENT_CROP_CONTEST)
        //   java: 				.contains(ItemUtils.getHeadTexture(entity.getItemBySlot(EquipmentSlot.HEAD)));
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public static boolean doesPestMatchCurrentVinyl(ArmorStand entity)
    pub fn does_pest_match_current_vinyl(entity: &ArmorStand) -> bool {
        if !(SkyblockerConfigManager::get().farming.pest_highlighter.vinyl_highlighter) {
            return false;
        }
        let mut vinyl: String = VacuumCache::get_vinyl();
        if GardenGlowAdder::contest_enabled(INSTANCE.get_mut()) || vinyl.is_empty() {
            return false;
        }
        // TODO(fabric2seer): `.get(..)` on a Multimap
        //   java: return GardenConstants.PEST_HEAD_BY_CROP
        //   java: 				.get(GardenConstants.CROP_BY_VINYL.get(vinyl))
        //   java: 				.contains(ItemUtils.getHeadTexture(entity.getItemBySlot(EquipmentSlot.HEAD)));
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public boolean contestEnabled()
    pub fn contest_enabled(__this: &compat::Shared<Self>) -> bool {
        // TODO(fabric2seer): static call StringUtils.isEmpty(..)
        //   java: return SkyblockerConfigManager.get().farming.pestHighlighter.contestHighlighter && !StringUtils.isEmpty(CurrentJacobCrop.CURRENT_CROP_CONTEST);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}

impl MobGlowAdder for GardenGlowAdder {
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
        GardenGlowAdder::compute_colour_dyn
    }

    /// java: @Override public boolean isEnabled() (dispatch: the body for this class)
    fn is_enabled(&self) -> fn(&compat::Shared<dyn MobGlowAdder>) -> bool {
        GardenGlowAdder::is_enabled_dyn
    }

}

impl GardenGlowAdder {
    /// java: @Override public int computeColour(Entity entity)
    pub fn compute_colour_impl(__this: &compat::Shared<Self>, entity: &dyn Entity) -> i32 {
        // TODO(fabric2seer): a pattern switch expression
        //   java: return switch (entity) {
        //   java: 			case ArmorStand as when isPestHead(as) ->
        //   java: 					doesPestMatchCurrentContest(as) ?
        //   java: 							// Pests but during Jacob's Contest
        //   java: 							TextColor.GREEN.getValue() :
        //   java: 							// Pests from currently playing vinyl
        //   java: 							doesPestMatchCurrentVinyl(as) ?
        //   java: 									TextColor.DARK_AQUA.getValue() :
        //   java: 									// Default color
        //   java: 									PEST_COLOUR;
        //   java: 			default -> NO_GLOW;
        //   java: 		};
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// GardenGlowAdder.computeColour as the dispatch of MobGlowAdder.computeColour
    pub fn compute_colour_dyn(__this: &compat::Shared<dyn MobGlowAdder>, __p0: &dyn Entity) -> i32 {
        Self::compute_colour_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public boolean isEnabled()
    pub fn is_enabled_impl(__this: &compat::Shared<Self>) -> bool {
        return (SkyblockerConfigManager::get().farming.pest_highlighter.enabled && Utils::is_in_garden());
    }

    /// GardenGlowAdder.isEnabled as the dispatch of MobGlowAdder.isEnabled
    pub fn is_enabled_dyn(__this: &compat::Shared<dyn MobGlowAdder>) -> bool {
        Self::is_enabled_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}

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
        // java: return GardenConstants.PEST_HEAD_BY_CROP.get(CurrentJacobCrop.CURRENT_CROP_CONTEST).contains(ItemUtils.getHeadTexture(entity.getItemBySlot(EquipmentSlot.HEAD)));
        // (a `Multimap.get` of an absent key is the empty collection)
        let crop: Option<String> = compat::get_state(&current_jacob_crop::CURRENT_CROP_CONTEST);
        let texture = ItemUtils::get_head_texture(entity.get_item_by_slot(EquipmentSlot::Head));
        return crop.and_then(|__c| garden_constants::PEST_HEAD_BY_CROP.get(&__c).map(|__heads| __heads.contains(&texture))).unwrap_or(false);
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
        // java: return GardenConstants.PEST_HEAD_BY_CROP.get(GardenConstants.CROP_BY_VINYL.get(vinyl)).contains(ItemUtils.getHeadTexture(entity.getItemBySlot(EquipmentSlot.HEAD)));
        let texture = ItemUtils::get_head_texture(entity.get_item_by_slot(EquipmentSlot::Head));
        return garden_constants::CROP_BY_VINYL.get(&vinyl).and_then(|__c| garden_constants::PEST_HEAD_BY_CROP.get(__c).map(|__heads| __heads.contains(&texture))).unwrap_or(false);
    }

    /// java: public boolean contestEnabled()
    pub fn contest_enabled(__this: &compat::Shared<Self>) -> bool {
        // java: return SkyblockerConfigManager.get().farming.pestHighlighter.contestHighlighter && !StringUtils.isEmpty(CurrentJacobCrop.CURRENT_CROP_CONTEST);
        let _ = __this;
        return SkyblockerConfigManager::get().farming.pest_highlighter.contest_highlighter && compat::get_state(&current_jacob_crop::CURRENT_CROP_CONTEST).is_some_and(|__c: String| !__c.is_empty());
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
        // java: return switch (entity) { case ArmorStand as when isPestHead(as) -> doesPestMatchCurrentContest(as) ? TextColor.GREEN.getValue()
        //   : doesPestMatchCurrentVinyl(as) ? TextColor.DARK_AQUA.getValue() : PEST_COLOUR; default -> NO_GLOW; };
        let _ = __this;
        if let Some(r#as) = entity.as_any().unwrap_or(&() as &dyn std::any::Any).downcast_ref::<ArmorStand>() && GardenGlowAdder::is_pest_head(r#as) {
            return if GardenGlowAdder::does_pest_match_current_contest(r#as) {
                // Pests but during Jacob's Contest
                mcrust::net::minecraft::network::chat::text_color::TextColor::GREEN.get_value()
            } else if GardenGlowAdder::does_pest_match_current_vinyl(r#as) {
                // Pests from currently playing vinyl
                mcrust::net::minecraft::network::chat::text_color::TextColor::DARK_AQUA.get_value()
            } else {
                // Default color
                PEST_COLOUR
            };
        }
        return *crate::de::hysky::skyblocker::skyblock::entity::mob_glow::NO_GLOW;
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

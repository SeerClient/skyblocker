//! Ported from de.hysky.skyblocker.skyblock.slayers.boss.vampire.TwinClawsIndicator by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::slayers::slayer_manager::SlayerManager;
use crate::de::hysky::skyblocker::skyblock::slayers::slayer_type::SlayerType;
use crate::de::hysky::skyblocker::utils::render::title::title::Title;
use crate::de::hysky::skyblocker::utils::render::title::title_container::TitleContainer;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler::Scheduler;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::world::entity::entity::Entity as _;

/// java: private static final Title title = new Title("skyblocker.rift.iceNow", ChatFormatting.AQUA);
pub static TITLE: compat::MainThread<compat::Shared<Title>> = compat::MainThread::new(|| Title::new("skyblocker.rift.iceNow".to_string(), ChatFormatting::Aqua));
/// java: private static boolean scheduled = false;
pub static SCHEDULED: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));

/// java: public class TwinClawsIndicator
#[derive(Clone)]
pub struct TwinClawsIndicator {}

impl TwinClawsIndicator {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static void updateIce()
    pub fn update_ice() {
        if !(SkyblockerConfigManager::get().slayers.vampire_slayer.enable_holy_ice_indicator) || !(SlayerManager::is_fighting_slayer_type(SlayerType::Vampire)) {
            TitleContainer::remove_title(TITLE.get_mut());
            return;
        }
        let Some(mut slayer_entity) = SlayerManager::get_slayer_armor_stand() else {
            return;
        };
        let mut any_claws: bool = false;
        for armor_stand_entity in SlayerManager::get_entity_armor_stands(&slayer_entity, 2.5f32) {
            // TODO(fabric2seer): `Component.toString` has no Rust method in the port (looked for `to_string` in Component and JavaSrc)
            // TODO(fabric2seer): the condition: `.contains(..)` on a value of unknown type
            //   java: armorStandEntity.getName(..).toString(..).contains(..)
            if todo!("fabric2seer: the condition (TODO above)") {
                any_claws = true;
                if !(TitleContainer::contains_title(TITLE.get_mut())) && !(compat::get_state(&SCHEDULED)) {
                    compat::set_state(&SCHEDULED, true);
                    ({ let __r = scheduler::INSTANCE.get_mut(); let __f = compat::dispatch(__r, |__t| __t.schedule()); __f(__r, Arc::new(move || {
                        TitleContainer::add_title_and_play_sound(TITLE.get_mut());
                        compat::set_state(&SCHEDULED, false);
                    }), SkyblockerConfigManager::get().slayers.vampire_slayer.holy_ice_indicator_tick_delay) });
                }
            }
        }
        if !any_claws {
            TitleContainer::remove_title(TITLE.get_mut());
        }
    }

}

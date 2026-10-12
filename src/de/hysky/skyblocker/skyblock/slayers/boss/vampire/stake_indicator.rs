//! Ported from de.hysky.skyblocker.skyblock.slayers.boss.vampire.StakeIndicator by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::slayers::slayer_manager::SlayerManager;
use crate::de::hysky::skyblocker::skyblock::slayers::slayer_type::SlayerType;
use crate::de::hysky::skyblocker::utils::render::title::title::Title;
use crate::de::hysky::skyblocker::utils::render::title::title_container::TitleContainer;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::world::entity::entity::Entity;

/// java: private static final Title title = new Title("skyblocker.rift.stakeNow", ChatFormatting.RED);
pub static TITLE: compat::MainThread<compat::Shared<Title>> = compat::MainThread::new(|| Title::new("skyblocker.rift.stakeNow".to_string(), ChatFormatting::Red));

/// java: public class StakeIndicator
#[derive(Clone)]
pub struct StakeIndicator {}

impl StakeIndicator {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static void updateStake()
    pub fn update_stake() {
        if !(SkyblockerConfigManager::get().slayers.vampire_slayer.enable_steak_stake_indicator) || !(SlayerManager::is_fighting_slayer_type(SlayerType::Vampire)) {
            TitleContainer::remove_title(TITLE.get_mut());
            return;
        }
        let mut slayer_entity: Option<Box<dyn Entity>> = SlayerManager::get_slayer_armor_stand();
        // TODO(fabric2seer): `Component.toString` has no Rust method in the port (looked for `to_string` in Component and JavaSrc)
        // TODO(fabric2seer): the condition: `.contains(..)` on a value of unknown type
        //   java: slayerEntity != null && slayerEntity.getName().toString().contains("҉")
        if todo!("fabric2seer: the condition (TODO above)") {
            TitleContainer::add_title_and_play_sound(TITLE.get_mut());
        } else {
            TitleContainer::remove_title(TITLE.get_mut());
        }
    }

}

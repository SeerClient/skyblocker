//! Ported from de.hysky.skyblocker.skyblock.slayers.boss.vampire.ManiaIndicator by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::slayers::slayer_manager::SlayerManager;
use crate::de::hysky::skyblocker::skyblock::slayers::slayer_type::SlayerType;
use crate::de::hysky::skyblocker::utils::render::title::title::Title;
use crate::de::hysky::skyblocker::utils::render::title::title_container::TitleContainer;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::core::block_pos::BlockPos;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::world::entity::entity::Entity as _;

/// java: private static final Title title = new Title("skyblocker.rift.mania", ChatFormatting.RED);
pub static TITLE: compat::MainThread<compat::Shared<Title>> = compat::MainThread::new(|| Title::new("skyblocker.rift.mania".to_string(), ChatFormatting::Red));

/// java: public class ManiaIndicator
#[derive(Clone)]
pub struct ManiaIndicator {}

impl ManiaIndicator {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static void updateMania()
    pub fn update_mania() {
        let client = compat::minecraft();
        if !(SkyblockerConfigManager::get().slayers.vampire_slayer.enable_mania_indicator) || !(SlayerManager::is_fighting_slayer_type(SlayerType::Vampire)) || mcrust::mcrust::modding::game::with_player(|_| ()).is_none() || mcrust::mcrust::modding::game::level().is_none() {
            TitleContainer::remove_title(TITLE.get_mut());
            return;
        }
        let Some(mut slayer_entity) = SlayerManager::get_slayer_armor_stand() else {
            return;
        };
        let mut any_mania: bool = false;
        for armor_stand_entity in SlayerManager::get_entity_armor_stands(&slayer_entity, 2.5f32) {
            // TODO(fabric2seer): `Component.toString` has no Rust method in the port (looked for `to_string` in Component and JavaSrc)
            // TODO(fabric2seer): the condition: `.contains(..)` on a value of unknown type
            //   java: armorStandEntity.getName(..).toString(..).contains(..)
            if todo!("fabric2seer: the condition (TODO above)") {
                any_mania = true;
                let mut pos: BlockPos = mcrust::mcrust::modding::game::with_player(|player| player.base.base.base.base.block_position().below()).unwrap();
                let mut is_green: bool = std::ptr::addr_eq(std::sync::Arc::as_ptr(&mcrust::mcrust::modding::game::level().unwrap().get_block_state_typed(&pos).get_block()), std::sync::Arc::as_ptr(mcrust::net::minecraft::world::level::block::blocks::dyed_terracotta().green()));
                Title::set_text(TITLE.get_mut(), component::translatable("skyblocker.rift.mania").with_style_format((if is_green { ChatFormatting::Green } else { ChatFormatting::Red })));
                TitleContainer::add_title_and_play_sound(TITLE.get_mut());
            }
        }
        if !any_mania {
            TitleContainer::remove_title(TITLE.get_mut());
        }
    }

}

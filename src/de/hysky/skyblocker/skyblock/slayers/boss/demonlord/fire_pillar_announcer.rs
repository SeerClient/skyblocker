//! Ported from de.hysky.skyblocker.skyblock.slayers.boss.demonlord.FirePillarAnnouncer by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::configs::slayers_config::FirePillar;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::slayers::slayer_manager::SlayerManager;
use crate::de::hysky::skyblocker::skyblock::slayers::slayer_type::SlayerType;
use crate::de::hysky::skyblocker::utils::render::title::title::Title;
use crate::de::hysky::skyblocker::utils::render::title::title_container::TitleContainer;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::world::entity::entity::Entity;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final Pattern FIRE_PILLAR_PATTERN = Pattern.compile("(\\d+)s \\d+ hits");
pub static FIRE_PILLAR_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("(\\d+)s \\d+ hits"));

/// java: public class FirePillarAnnouncer
#[derive(Clone)]
pub struct FirePillarAnnouncer {}

impl FirePillarAnnouncer {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static void checkFirePillar(Entity entity)
    pub fn check_fire_pillar(entity: &dyn Entity) {
        if SkyblockerConfigManager::get().slayers.blaze_slayer.fire_pillar_countdown == FirePillar::Off {
            return;
        }
        if Utils::is_in_crimson() && SlayerManager::is_fighting_slayer_type(SlayerType::Demonlord) {
            let mut entity_name: String = compat::component_of(entity.get_name()).get_string();
            let mut matcher: compat::regex::Matcher = FIRE_PILLAR_PATTERN.matcher(&entity_name);
            if matcher.matches() {
                let mut seconds: i32 = matcher.group(1).unwrap().trim().parse::<i32>().expect("NumberFormatException");
                if seconds > 5 {
                    return;
                }
                let mut reference_entity: Option<Box<dyn Entity>> = SlayerManager::get_slayer_armor_stand();
                // TODO(fabric2seer): the condition: `Minecraft.player` held as a value: the port lends the player only inside `game::with_player(|player| ..)` (a mod must never lock it)
                //   java: !(referenceEntity != null ? referenceEntity : Minecraft.getInstance().player).blockPosition().closerToCenterThan(entity.position(), 22)
                if todo!("fabric2seer: the condition (TODO above)") {
                    return;
                }
                FirePillarAnnouncer::announce_fire_pillar_details(&entity_name);
            }
        }
    }

    /// java: private static void announceFirePillarDetails(String entityName)
    pub fn announce_fire_pillar_details(entity_name: &str) {
        let mut title: compat::Shared<Title> = Title::new_1(component::literal(entity_name).with_style_formats(&[ChatFormatting::Bold, ChatFormatting::DarkPurple]));
        if SkyblockerConfigManager::get().slayers.blaze_slayer.fire_pillar_countdown == FirePillar::SoundAndVisual {
            TitleContainer::add_title_and_play_sound_2(&title, 15);
        } else {
            TitleContainer::add_title_2(&title, 15);
        }
    }

}

//! Ported from de.hysky.skyblocker.skyblock.slayers.boss.demonlord.AttunementColors by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::entity::mob_glow;
use crate::de::hysky::skyblocker::skyblock::slayers::slayer_manager::SlayerManager;
use mcrust::net::minecraft::world::entity::living_entity::LivingEntity;
use mcrust::net::minecraft::network::chat::component::Component as _;
use mcrust::net::minecraft::world::entity::entity::Entity as _;

/// java: private static final Pattern COLOR_PATTERN = Pattern.compile("ASHEN|SPIRIT|CRYSTAL|AURIC|IMMUNE");
pub static COLOR_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("ASHEN|SPIRIT|CRYSTAL|AURIC|IMMUNE"));

/// java: public class AttunementColors
#[derive(Clone)]
pub struct AttunementColors {}

impl AttunementColors {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static int getColor(LivingEntity entity)
    pub fn get_color(entity: &dyn LivingEntity) -> i32 {
        for armor_stand_entity in SlayerManager::get_entity_armor_stands(entity, 2.5f32) {
            let mut matcher: compat::regex::Matcher = COLOR_PATTERN.matcher(&compat::component_of(armor_stand_entity.get_name()).get_string());
            if matcher.find() {
                // TODO(fabric2seer): `.getRGB(..)` on a value of unknown type
                //   java: return switch (matcher.group()) {
                //   java: 					case "ASHEN" -> Color.DARK_GRAY.getRGB();
                //   java: 					case "SPIRIT" -> Color.WHITE.getRGB();
                //   java: 					case "CRYSTAL" -> Color.CYAN.getRGB();
                //   java: 					case "AURIC" -> Color.YELLOW.getRGB();
                //   java: 					case "IMMUNE" -> Color.RED.getRGB();
                //   java: 					default -> MobGlow.NO_GLOW;
                //   java: 				};
            }
        }
        return *mob_glow::NO_GLOW;
    }

}

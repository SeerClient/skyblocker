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
        // the stands by id (`entity` is held by the caller and is the sweep's `except`)
        for armor_stand_entity in SlayerManager::get_entity_armor_stands(entity, 2.5f32) {
            let Some(name) = mcrust::mcrust::modding::game::with_entity(armor_stand_entity, |e| e.get_name().get_string()) else {
                continue;
            };
            let mut matcher: compat::regex::Matcher = COLOR_PATTERN.matcher(&name);
            if matcher.find() {
                return match matcher.group(0).unwrap().as_str() {
                    "ASHEN" => mcrust::java::awt::color::dark_gray().get_rgb(),
                    "SPIRIT" => mcrust::java::awt::color::white().get_rgb(),
                    "CRYSTAL" => mcrust::java::awt::color::cyan().get_rgb(),
                    "AURIC" => mcrust::java::awt::color::yellow().get_rgb(),
                    "IMMUNE" => mcrust::java::awt::color::red().get_rgb(),
                    _ => *mob_glow::NO_GLOW,
                };
            }
        }
        return *mob_glow::NO_GLOW;
    }

}

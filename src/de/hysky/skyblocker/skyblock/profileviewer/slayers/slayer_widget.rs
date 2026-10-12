//! Ported from de.hysky.skyblocker.skyblock.profileviewer.slayers.SlayerWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::profileviewer::utils::level_finder::LevelFinder;
use crate::de::hysky::skyblocker::skyblock::profileviewer::utils::level_finder::LevelInfo;
use crate::de::hysky::skyblocker::skyblock::tabhud::util::ico;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use crate::de::hysky::skyblocker::utils::formatters;
use crate::de::hysky::skyblocker::utils::render::gui_helper::GuiHelper;
use mcrust::java::awt::color::Color;
use mcrust::java::text::decimal_format_symbols::Locale;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::font::Font;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;

/// java: private static final Identifier TEXTURE = SkyblockerMod.id("textures/gui/profile_viewer/icon_data_widget.png");
pub static TEXTURE: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("textures/gui/profile_viewer/icon_data_widget.png"));
/// java: private static final Identifier BAR_FILL = SkyblockerMod.id("bars/bar_fill");
pub static BAR_FILL: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("bars/bar_fill"));
/// java: private static final Identifier BAR_BACK = SkyblockerMod.id("bars/bar_back");
pub static BAR_BACK: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("bars/bar_back"));
/// java: public static final Map<String, Identifier> HEAD_ICON = Map.ofEntries( Map.entry("Zombie", SkyblockerMod.id("textures/gui/profile_viewer/zombie.png")), Map.entry("Spider", SkyblockerMod.id("textures/g
pub static HEAD_ICON: compat::MainThread<std::collections::HashMap<String, Identifier>> = compat::MainThread::new(|| std::collections::HashMap::from([("Zombie".to_string(), crate::entry::skyblocker_mod::id("textures/gui/profile_viewer/zombie.png")), ("Spider".to_string(), crate::entry::skyblocker_mod::id("textures/gui/profile_viewer/spider.png")), ("Wolf".to_string(), crate::entry::skyblocker_mod::id("textures/gui/profile_viewer/wolf.png")), ("Enderman".to_string(), crate::entry::skyblocker_mod::id("textures/gui/profile_viewer/enderman.png")), ("Vampire".to_string(), crate::entry::skyblocker_mod::id("textures/gui/profile_viewer/vampire.png")), ("Blaze".to_string(), crate::entry::skyblocker_mod::id("textures/gui/profile_viewer/blaze.png"))]));
/// java: public static final Map<String, FlexibleItemStack> DROP_ICON = Map.ofEntries( Map.entry("Zombie", Ico.FLESH), Map.entry("Spider", Ico.STRING), Map.entry("Wolf", Ico.MUTTON), Map.entry("Enderman", Ico.
pub static DROP_ICON: compat::MainThread<std::collections::HashMap<String, compat::Shared<FlexibleItemStack>>> = compat::MainThread::new(|| std::collections::HashMap::from([("Zombie".to_string(), ico::FLESH.get_mut().clone()), ("Spider".to_string(), ico::STRING.get_mut().clone()), ("Wolf".to_string(), ico::MUTTON.get_mut().clone()), ("Enderman".to_string(), ico::E_PEARL.get_mut().clone()), ("Vampire".to_string(), ico::REDSTONE.get_mut().clone()), ("Blaze".to_string(), ico::B_POWDER.get_mut().clone())]));

/// java: public class SlayerWidget
#[derive(Clone)]
pub struct SlayerWidget {
    /// java: private final String slayerName;
    pub slayer_name: String,
    /// java: private final LevelFinder.LevelInfo slayerLevel;
    pub slayer_level: compat::Shared<LevelInfo>,
    /// java: private JsonObject slayerData = null;
    pub slayer_data: Option<serde_json::Map<String, serde_json::Value>>,
    /// java: private final Identifier item;
    pub item: Identifier,
    /// java: private final FlexibleItemStack drop;
    pub drop: compat::Shared<FlexibleItemStack>,
}

impl SlayerWidget {
    /// java: public SlayerWidget(String slayer, long xp, JsonObject playerProfile)
    pub fn new(mut slayer: String, mut xp: i64, mut player_profile: serde_json::Map<String, serde_json::Value>) -> Self {
        let __init_slayer_name = slayer.clone();
        let __init_slayer_level = LevelFinder::get_level_info(&slayer, xp);
        let __init_item = HEAD_ICON.get_mut().get(&slayer).cloned().unwrap();
        let __init_drop = DROP_ICON.get_mut().get(&slayer).cloned().unwrap_or_else(|| ico::BARRIER.get_mut().clone());
        let mut this = Self {
            slayer_name: __init_slayer_name,
            slayer_level: __init_slayer_level,
            slayer_data: None,
            item: __init_item,
            drop: __init_drop,
        };
        // java: try {..} catch (Exception _) - an exception here is a panic, caught by `catch_unwind`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            this.slayer_data = Some(compat::json::get_as_json_object_of(&(compat::json::get_as_json_object_of(&(compat::json::get_as_json_object_of(&player_profile, "slayer")), "slayer_bosses")), &this.slayer_name.to_lowercase()));
        }));
        if let Err(_) = __result {
        }
        this
    }

    /// java: public void extractRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, int x, int y)
    pub fn extract_render_state(&mut self, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut x: i32, mut y: i32) {
        let mut font: Font = compat::font();
        gui_graphics_extractor::blit_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), TEXTURE.get_mut(), x, y, ((0) as f32) as f32, ((0) as f32) as f32, 109, 26, 109, 26);
        gui_graphics_extractor::blit_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), &self.item, x.wrapping_add(1), y.wrapping_add(3), ((0) as f32) as f32, ((0) as f32) as f32, 20, 20, 20, 20);
        gui_graphics_extractor::text_string_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), Some(&format!("{} {}", self.slayer_name, compat::read(&self.slayer_level, |__t| __t.level))), x.wrapping_add(31), y.wrapping_add(5), compat::hash_code(&mcrust::java::awt::color::white()), false);
        let mut col2: i32 = x.wrapping_add(113);
        gui_graphics_extractor::blit_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), TEXTURE.get_mut(), col2, y, ((0) as f32) as f32, ((0) as f32) as f32, 109, 26, 109, 26);
        graphics.item(&FlexibleItemStack::get_stack_or_throw(&self.drop), col2.wrapping_add(3), y.wrapping_add(5));
        gui_graphics_extractor::text_string_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), Some(&format!("§aKills: §r{}", self.find_total_kills())), col2.wrapping_add(30), y.wrapping_add(4), compat::hash_code(&mcrust::java::awt::color::white()), true);
        gui_graphics_extractor::text_string_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), Some(&self.find_top_tier_kills()), (if self.find_top_tier_kills() == "No Data" { col2.wrapping_add(30) } else { col2.wrapping_add(29) }), y.wrapping_add(15), compat::hash_code(&mcrust::java::awt::color::white()), true);
        gui_graphics_extractor::blit_sprite_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), BAR_BACK.get_mut(), x.wrapping_add(30), y.wrapping_add(15), 75, 6);
        let mut fill_color: Color = (if compat::read(&self.slayer_level, |__t| __t.fill) == (1 as f64) { mcrust::java::awt::color::magenta() } else { mcrust::java::awt::color::green() });
        GuiHelper::nine_slice_colored_gui_graphics_extractor_identifier_int_int_int_int_color(graphics, BAR_FILL.get_mut(), x.wrapping_add(30), y.wrapping_add(15), ((((75 as f64) * compat::read(&self.slayer_level, |__t| __t.fill))) as i32), 6, &fill_color);
        if mouse_x > x.wrapping_add(30) && mouse_x < x.wrapping_add(105) && mouse_y > y.wrapping_add(12) && mouse_y < y.wrapping_add(22) {
            let mut tooltip_text: Vec<MutableComponent> = Vec::new();
            tooltip_text.push(component::literal(&self.slayer_name).with_style_format(ChatFormatting::Green));
            tooltip_text.push((component::literal(&format!("XP: {}", formatters::INTEGER_NUMBERS.get_mut().format((compat::read(&self.slayer_level, |__t| __t.xp)) as f64)))).with_style_format(ChatFormatting::Gold));
            // TODO(fabric2seer): the port's `set_component_tooltip_for_next_frame` ports this Java method with other parameters (tooltip_for_next_frame, lines for Java's lines, xo, yo)
            // TODO(fabric2seer): `set_component_tooltip_for_next_frame` takes 2 argument(s) in the port (`tooltip_for_next_frame: &mut Option<DeferredTooltip>, lines: Vec<crate::net::minecraft::network::chat::formatted_text::FormattedText>`), Java passes 4
            // rust: gui_graphics_extractor::set_component_tooltip_for_next_frame(&mut font, tooltip_text.clone(), mouse_x, mouse_y);
        }
    }

    /// java: private int findTotalKills()
    pub fn find_total_kills(&self) -> i32 {
        // java: try {..} catch (Exception _) - an exception here is a panic, caught by `catch_unwind`; the body's `return` is the closure's value
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut total_kills: i32 = 0;
            for key in self.slayer_data.as_ref().unwrap().keys().cloned().collect::<Vec<String>>() {
                if key.starts_with("boss_kills_tier_") {
                    total_kills = total_kills.wrapping_add(compat::json::get_as_int(&self.slayer_data.as_ref().unwrap().get(&key).cloned().unwrap()));
                }
            }
            return total_kills;
        })) {
            Ok(__v) => return __v,
            Err(_) => {
                return 0;
            }
        }
    }

    /// java: private String findTopTierKills()
    pub fn find_top_tier_kills(&self) -> String {
        // java: try {..} catch (Exception _) - the body runs inline: a Rust body does not throw checked exceptions (a panic in it propagates)
        {
            {
                let mut tier: i32 = 4;
                while tier >= 0 {
                    let mut key: String = format!("boss_kills_tier_{}", tier);
                    if compat::json::has(&self.slayer_data.as_ref().unwrap(), &key) {
                        return format!("§cT{} Kills: §r{}", tier.wrapping_add(1), compat::json::get_as_int(&self.slayer_data.as_ref().unwrap().get(&key).cloned().unwrap()));
                    }
                    tier -= 1;
                }
            }
        }
        // TODO(fabric2seer): the `catch (Exception _)` block is not reached in Rust - the try body returns, breaks or continues, so it cannot run inside `catch_unwind`
        //   java: {}
        return "No Data".to_string();
    }

}

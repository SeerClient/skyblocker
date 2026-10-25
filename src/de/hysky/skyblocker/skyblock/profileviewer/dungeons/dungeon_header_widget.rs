//! Ported from de.hysky.skyblocker.skyblock.profileviewer.dungeons.DungeonHeaderWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::profileviewer::utils::level_finder::LevelFinder;
use crate::de::hysky::skyblocker::skyblock::profileviewer::utils::level_finder::LevelInfo;
use mcrust::java::text::decimal_format::DecimalFormat;
use mcrust::java::text::decimal_format_symbols::Locale;
use mcrust::net::minecraft::client::gui::font::Font;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::resources::identifier::Identifier;

/// java: private static final Font textRenderer = Minecraft.getInstance().font;
pub static TEXT_RENDERER: compat::LiveFont = compat::LiveFont;
/// java: private static final DecimalFormat DF = new DecimalFormat("#.##");
pub static DF: compat::MainThread<DecimalFormat> = compat::MainThread::new(|| DecimalFormat::new("#.##", mcrust::java::text::decimal_format_symbols::get_instance(Locale::Us)));
/// java: private static final Identifier TEXTURE = SkyblockerMod.id("textures/gui/profile_viewer/dungeons_header.png");
pub static TEXTURE: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("textures/gui/profile_viewer/dungeons_header.png"));

/// java: public class DungeonHeaderWidget
#[derive(Clone)]
pub struct DungeonHeaderWidget {
    /// java: private LevelFinder.LevelInfo classLevel;
    pub class_level: Option<compat::Shared<LevelInfo>>,
    /// java: private float classAvg;
    pub class_avg: f32,
}

impl DungeonHeaderWidget {
    /// java: public DungeonHeaderWidget(JsonObject playerProfile, String[] classes)
    pub fn new(mut player_profile: serde_json::Map<String, serde_json::Value>, mut classes: Vec<String>) -> Self {
        let mut this = Self {
            class_level: None,
            class_avg: 0.0,
        };
        // java: try {..} catch (Exception _) - an exception here is a panic, caught by `catch_unwind`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut dungeons_profile: serde_json::Map<String, serde_json::Value> = compat::json::get_as_json_object_of(&(compat::json::get_as_json_object_of(&(compat::json::get_as_json_object_of(&player_profile, "dungeons")), "dungeon_types")), "catacombs");
            this.class_level = Some(LevelFinder::get_level_info("Catacombs", compat::json::get_as_long(&dungeons_profile.get("experience").cloned().unwrap())));
            let mut avg: f32 = ((0) as f32) as f32;
            let mut class_data: serde_json::Map<String, serde_json::Value> = compat::json::get_as_json_object_of(&(compat::json::get_as_json_object_of(&player_profile, "dungeons")), "player_classes");
            for element in classes.iter() {
                avg = (avg + (compat::read(&LevelFinder::get_level_info("Catacombs", compat::json::get_as_long(&(compat::json::get_as_json_object_of(&class_data, &element.to_lowercase())).get("experience").cloned().unwrap())), |__t| __t.level) as f32));
            }
            this.class_avg = (avg / ((classes.len() as i32) as f32));
        }));
        if let Err(_) = __result {
            this.class_level = Some(LevelFinder::get_level_info("", ((0) as i64) as i64));
            this.class_avg = ((0) as f32) as f32;
        }
        this
    }

    /// java: public void extractRenderState(GuiGraphicsExtractor graphics, int x, int y)
    pub fn extract_render_state(&self, graphics: &GuiGraphicsExtractor, mut x: i32, mut y: i32) {
        gui_graphics_extractor::blit_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), TEXTURE.get_mut(), x, y, ((0) as f32) as f32, ((0) as f32) as f32, 109, 26, 109, 26);
        // java: graphics.text(textRenderer, "§i§6§lCatacombs §r" + this.classLevel.level, x + 3, y + 4, Color.WHITE.getRGB(), true);
        gui_graphics_extractor::text_string_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), Some(&format!("§i§6§lCatacombs §r{}", compat::read(self.class_level.as_ref().unwrap(), |__t| __t.level))), x.wrapping_add(3), y.wrapping_add(4), mcrust::java::awt::color::white().get_rgb(), true);
        // java: graphics.text(textRenderer, "§eClass Average §r" + DF.format(this.classAvg), x + 3, y + 14, Color.WHITE.getRGB(), true);
        gui_graphics_extractor::text_string_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), Some(&format!("§eClass Average §r{}", DF.get().format(self.class_avg as f64))), x.wrapping_add(3), y.wrapping_add(14), mcrust::java::awt::color::white().get_rgb(), true);
    }

}

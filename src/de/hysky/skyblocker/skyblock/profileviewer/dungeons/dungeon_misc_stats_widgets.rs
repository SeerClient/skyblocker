//! Ported from de.hysky.skyblocker.skyblock.profileviewer.dungeons.DungeonMiscStatsWidgets by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::tabhud::util::ico;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use mcrust::java::text::decimal_format::DecimalFormat;
use mcrust::net::minecraft::client::gui::font::Font;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::resources::identifier::Identifier;

/// java: private static final Identifier TEXTURE = SkyblockerMod.id("textures/gui/profile_viewer/icon_data_widget.png");
pub static TEXTURE: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("textures/gui/profile_viewer/icon_data_widget.png"));
/// java: private static final Identifier RUN_ICON = SkyblockerMod.id("textures/gui/profile_viewer/run_icon.png");
pub static RUN_ICON: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("textures/gui/profile_viewer/run_icon.png"));
/// java: private static final Font textRenderer = Minecraft.getInstance().font;
pub static TEXT_RENDERER: compat::LiveFont = compat::LiveFont;
/// java: private static final DecimalFormat DF = new DecimalFormat("#.##");
pub static DF: compat::MainThread<DecimalFormat> = compat::MainThread::new(|| DecimalFormat::new("#.##", mcrust::java::text::decimal_format_symbols::get_instance(mcrust::java::text::decimal_format_symbols::Locale::Us)));
/// java: private static final String[] DUNGEONS = {"catacombs", "master_catacombs"};
pub static DUNGEONS: LazyLock<Vec<String>> = LazyLock::new(|| vec!["catacombs".to_string(), "master_catacombs".to_string()]);

/// java: public class DungeonMiscStatsWidgets
#[derive(Clone)]
pub struct DungeonMiscStatsWidgets {
    /// java: private final Object2IntMap<String> dungeonRuns = new Object2IntOpenHashMap<>();
    pub dungeon_runs: std::collections::HashMap<String, i32>,
    /// java: private int secrets = 0;
    pub secrets: i32,
    /// java: private int totalRuns = 0;
    pub total_runs: i32,
}

impl DungeonMiscStatsWidgets {
    /// java: public DungeonMiscStatsWidgets(JsonObject pProfile)
    pub fn new(mut p_profile: serde_json::Map<String, serde_json::Value>) -> Self {
        let mut dungeons_data: serde_json::Map<String, serde_json::Value> = compat::json::get_as_json_object_of(&p_profile, "dungeons");
        let mut this = Self {
            dungeon_runs: std::collections::HashMap::new(),
            secrets: 0,
            total_runs: 0,
        };
        // java: try {..} catch (Exception _) - an exception here is a panic, caught by `catch_unwind`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            this.secrets = compat::json::get_as_int(&dungeons_data.get("secrets").cloned().unwrap());
            for dungeon in DUNGEONS.iter() {
                let mut dungeon_data: serde_json::Map<String, serde_json::Value> = compat::json::get_as_json_object_of(&(compat::json::get_as_json_object_of(&(compat::json::get_as_json_object_of(&dungeons_data, "dungeon_types")), dungeon)), "tier_completions");
                let mut runs: i32 = 0;
                for entry in dungeon_data.iter().map(|(k, v)| (k.clone(), v.clone())) {
                    let mut key: String = entry.0.clone();
                    if key == "total" {
                        continue;
                    }
                    runs = runs.wrapping_add(compat::json::get_as_int(&entry.1));
                }
                this.dungeon_runs.insert(dungeon.to_string(), runs);
                this.total_runs = this.total_runs.wrapping_add(runs);
            }
        }));
        if let Err(_) = __result {
        }
        this
    }

    /// java: public void extractRenderState(GuiGraphicsExtractor graphics, int x, int y)
    pub fn extract_render_state(&self, graphics: &GuiGraphicsExtractor, mut x: i32, mut y: i32) {
        gui_graphics_extractor::blit_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), TEXTURE.get_mut(), x, y, ((0) as f32) as f32, ((0) as f32) as f32, 109, 26, 109, 26);
        graphics.item(&FlexibleItemStack::get_stack_or_throw(ico::FEATHER.get_mut()), x.wrapping_add(2), y.wrapping_add(4));
        // java: graphics.text(textRenderer, "Secrets " + secrets, x + 30, y + 4, Color.WHITE.getRGB(), true);
        gui_graphics_extractor::text_string_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), Some(&format!("Secrets {}", self.secrets)), x.wrapping_add(30), y.wrapping_add(4), mcrust::java::awt::color::white().get_rgb(), true);
        // java: graphics.text(textRenderer, "Avg " + (totalRuns > 0 ? DF.format(secrets / (float) totalRuns) : 0) + "/Run", x + 30, y + 14, Color.WHITE.getRGB(), true);
        gui_graphics_extractor::text_string_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), Some(&format!("Avg {}/Run", if self.total_runs > 0 { DF.get().format(((self.secrets as f32) / (self.total_runs as f32)) as f64) } else { "0".to_string() })), x.wrapping_add(30), y.wrapping_add(14), mcrust::java::awt::color::white().get_rgb(), true);
        gui_graphics_extractor::blit_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), TEXTURE.get_mut(), x, y.wrapping_add(28), ((0) as f32) as f32, ((0) as f32) as f32, 109, 26, 109, 26);
        gui_graphics_extractor::blit_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), RUN_ICON.get_mut(), x.wrapping_add(4), y.wrapping_add(33), ((0) as f32) as f32, ((0) as f32) as f32, 14, 16, 14, 16);
        // java: graphics.text(textRenderer, "§aNormal §r" + dungeonRuns.getOrDefault("catacombs", 0), x + 30, y + 32, Color.WHITE.getRGB(), true);
        gui_graphics_extractor::text_string_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), Some(&format!("§aNormal §r{}", self.dungeon_runs.get("catacombs").copied().unwrap_or(0))), x.wrapping_add(30), y.wrapping_add(32), mcrust::java::awt::color::white().get_rgb(), true);
        // java: graphics.text(textRenderer, "§cMaster §r" + dungeonRuns.getOrDefault("master_catacombs", 0), x + 30, y + 42, Color.WHITE.getRGB(), true);
        gui_graphics_extractor::text_string_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), Some(&format!("§cMaster §r{}", self.dungeon_runs.get("master_catacombs").copied().unwrap_or(0))), x.wrapping_add(30), y.wrapping_add(42), mcrust::java::awt::color::white().get_rgb(), true);
    }

}

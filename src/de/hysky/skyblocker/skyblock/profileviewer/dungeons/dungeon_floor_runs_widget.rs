//! Ported from de.hysky.skyblocker.skyblock.profileviewer.dungeons.DungeonFloorRunsWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::font::Font;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;

/// java: private static final Font textRenderer = Minecraft.getInstance().font;
pub static TEXT_RENDERER: compat::LiveFont = compat::LiveFont;
/// java: private static final Identifier TEXTURE = SkyblockerMod.id("textures/gui/profile_viewer/dungeons_body.png");
pub static TEXTURE: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("textures/gui/profile_viewer/dungeons_body.png"));
/// java: private static final String[] DUNGEONS = {"catacombs", "master_catacombs"};
pub static DUNGEONS: LazyLock<Vec<String>> = LazyLock::new(|| vec!["catacombs".to_string(), "master_catacombs".to_string()]);

/// java: public class DungeonFloorRunsWidget
#[derive(Clone)]
pub struct DungeonFloorRunsWidget {
    /// java: private JsonObject dungeonsStats;
    pub dungeons_stats: Option<serde_json::Map<String, serde_json::Value>>,
}

impl DungeonFloorRunsWidget {
    /// java: public DungeonFloorRunsWidget(JsonObject pProfile)
    pub fn new(mut p_profile: serde_json::Map<String, serde_json::Value>) -> Self {
        let mut this = Self {
            dungeons_stats: None,
        };
        // java: try {..} catch (Exception _) - an exception here is a panic, caught by `catch_unwind`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            this.dungeons_stats = Some(compat::json::get_as_json_object_of(&(compat::json::get_as_json_object_of(&p_profile, "dungeons")), "dungeon_types"));
        }));
        if let Err(_) = __result {
        }
        this
    }

    /// java: public void extractRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, int x, int y)
    pub fn extract_render_state(&self, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut x: i32, mut y: i32) {
        gui_graphics_extractor::blit_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), TEXTURE.get_mut(), x, y, ((0) as f32) as f32, ((0) as f32) as f32, 109, 110, 109, 110);
        // java: graphics.text(textRenderer, Component.literal("Floor Runs").withStyle(ChatFormatting.BOLD), x + 6, y + 4, Color.WHITE.getRGB(), true);
        gui_graphics_extractor::draw_formatted_text_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::literal("Floor Runs").with_style_format(ChatFormatting::Bold)), x.wrapping_add(6), y.wrapping_add(4), mcrust::java::awt::color::white().get_rgb(), true);
        let mut column_x: i32 = x.wrapping_add(4);
        let mut element_y: i32 = y.wrapping_add(15);
        for dungeon in DUNGEONS.iter() {
            // java: JsonObject dungeonData; try {..} catch (Exception _) - an exception here is a panic, caught by
            // `catch_unwind` (the variable is only read inside the try)
            let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let dungeon_data: serde_json::Map<String, serde_json::Value> = compat::json::get_as_json_object_of(&(compat::json::get_as_json_object_of(&self.dungeons_stats.as_ref().unwrap(), dungeon)), "tier_completions");
                let mut entries: Vec<(String, serde_json::Value)> = dungeon_data.iter().map(|(k, v)| (k.clone(), v.clone())).collect::<Vec<_>>();
                // java: entries.sort(Comparator.comparing(Map.Entry::getKey)); - String.compareTo
                entries.sort_by(|a, b| compat::compare_to(&a.0, &b.0).cmp(&0));
                for entry in entries.iter() {
                    if entry.0 == "total" {
                        continue;
                    }
                    // java: String textToRender = String.format((dungeon.equals("catacombs") ? "§aF" : "§cM") + "%s§r %s", entry.getKey(), entry.getValue().getAsInt());
                    let mut text_to_render: String = compat::string_format(&format!("{}%s§r %s", if dungeon == "catacombs" { "§aF" } else { "§cM" }), &[compat::FormatArg::from(entry.0.clone()), compat::FormatArg::from(compat::json::get_as_int(&entry.1))]);
                    // java: graphics.text(textRenderer, textToRender, columnX + 2, elementY + 2, Color.WHITE.getRGB(), true);
                    gui_graphics_extractor::draw_formatted_text_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::literal(&text_to_render)), column_x.wrapping_add(2), element_y.wrapping_add(2), mcrust::java::awt::color::white().get_rgb(), true);
                    if !(entry.0 == "0") && mouse_x >= column_x && mouse_x <= column_x.wrapping_add(40) && mouse_y >= element_y && mouse_y <= element_y.wrapping_add(9) {
                        let mut tooltip_text: Vec<MutableComponent> = Vec::new();
                        tooltip_text.push(component::literal("Personal Bests").with_style_formats(&[ChatFormatting::Bold, ChatFormatting::LightPurple]));
                        let mut fastest_times: Option<serde_json::Map<String, serde_json::Value>> = Some(compat::json::get_as_json_object_of(&(compat::json::get_as_json_object_of(&self.dungeons_stats.as_ref().unwrap(), dungeon)), "fastest_time_s"));
                        if fastest_times.is_some() && compat::json::has(&fastest_times.as_ref().unwrap(), &entry.0) {
                            tooltip_text.push((component::literal(&format!("S Run:  {}", self.format_time(compat::json::get_as_long(&fastest_times.as_ref().unwrap().get(&entry.0).cloned().unwrap()))))).with_style_format(ChatFormatting::Gold));
                        }
                        fastest_times = Some(compat::json::get_as_json_object_of(&(compat::json::get_as_json_object_of(&self.dungeons_stats.as_ref().unwrap(), dungeon)), "fastest_time_s_plus"));
                        if fastest_times.is_some() && compat::json::has(&fastest_times.as_ref().unwrap(), &entry.0) {
                            tooltip_text.push((component::literal(&format!("S+ Run: {}", self.format_time(compat::json::get_as_long(&fastest_times.as_ref().unwrap().get(&entry.0).cloned().unwrap()))))).with_style_format(ChatFormatting::Gold));
                        }
                        fastest_times = Some(compat::json::get_as_json_object_of(&(compat::json::get_as_json_object_of(&self.dungeons_stats.as_ref().unwrap(), dungeon)), "fastest_time"));
                        if fastest_times.is_some() && compat::json::has(&fastest_times.as_ref().unwrap(), &entry.0) && (tooltip_text.len() as i32) == 1 {
                            tooltip_text.push((component::literal(&format!("Completion:  {}", self.format_time(compat::json::get_as_long(&fastest_times.as_ref().unwrap().get(&entry.0).cloned().unwrap()))))).with_style_format(ChatFormatting::Gold));
                        }
                        // java: graphics.setComponentTooltipForNextFrame(textRenderer, tooltipText, mouseX, mouseY); - the frame's deferred tooltip (drawn at the mouse)
                        compat::batch_s_set_tooltip_for_next_frame(tooltip_text.iter().cloned().map(mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from).collect());
                    }
                    element_y = element_y.wrapping_add(11);
                }
                column_x = column_x.wrapping_add(52);
                element_y = y.wrapping_add(26);
            }));
            if let Err(_) = __result {
                return;
            }
        }
    }

    /// java: private String formatTime(long milliseconds)
    pub fn format_time(&self, mut milliseconds: i64) -> String {
        let mut seconds: i64 = milliseconds.wrapping_div(1000 as i64);
        let mut minutes: i64 = seconds.wrapping_div(60 as i64);
        seconds = seconds.wrapping_rem(60 as i64);
        return format!("{:2}:{:02}", minutes, seconds);
    }

}

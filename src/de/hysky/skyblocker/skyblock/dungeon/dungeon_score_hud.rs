//! Ported from de.hysky.skyblocker.skyblock.dungeon.DungeonScoreHUD by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::dungeon::dungeon_score::DungeonScore;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::org::joml::matrix3x2f_stack::Matrix3x2fStack;

/// java: private static final Identifier DUNGEON_SCORE = SkyblockerMod.id("dungeon_score");
pub static DUNGEON_SCORE: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("dungeon_score"));
/// java: private static final Component extraSpace = Component.literal(" ").append(Component.literal(" ").withStyle(ChatFormatting.BOLD));
pub static EXTRA_SPACE: compat::MainThread<MutableComponent> = compat::MainThread::new(|| component::literal(" ").append(component::literal(" ").with_style_format(ChatFormatting::Bold)));

/// java: public class DungeonScoreHUD
#[derive(Clone)]
pub struct DungeonScoreHUD {}

impl DungeonScoreHUD {
    /// java: private DungeonScoreHUD()
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        hud_render_callback::EVENT.register(Arc::new(|ctx: &mut hud_render_callback::HudRenderContext<'_>| {
            DungeonScoreHUD::extract_render_state(&ctx.graphics);
        }));
    }

    /// java: private static void extractRenderState(GuiGraphicsExtractor graphics)
    pub fn extract_render_state(graphics: &GuiGraphicsExtractor) {
        if Utils::is_in_dungeons() && DungeonScore::is_dungeon_started() && compat::read(&SkyblockerConfigManager::get().dungeons.dungeon_score, |__t| __t.enable_score_hud) {
            let mut x: i32 = compat::read(&SkyblockerConfigManager::get().dungeons.dungeon_score, |__t| __t.score_x);
            let mut y: i32 = compat::read(&SkyblockerConfigManager::get().dungeons.dungeon_score, |__t| __t.score_y);
            DungeonScoreHUD::extract_render_state_3(graphics, x, y);
        }
    }

    /// java: public static void extractRenderState(GuiGraphicsExtractor graphics, int x, int y)
    pub fn extract_render_state_3(graphics: &GuiGraphicsExtractor, mut x: i32, mut y: i32) {
        let mut scale: f32 = compat::read(&SkyblockerConfigManager::get().dungeons.dungeon_score, |__t| __t.score_scaling);
        let __pose_1 = gui_graphics_extractor::push_pose();
        gui_graphics_extractor::pose_scale((scale) as f32, (scale) as f32);
        gui_graphics_extractor::draw_formatted_text(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), ((((x as f32) / scale)) as i32), ((((y as f32) / scale)) as i32), &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(DungeonScoreHUD::get_formatted_score_text()), 0xFFFFFFFFu32 as i32);
        gui_graphics_extractor::pop_pose(__pose_1);
    }

    /// java: public static Component getFormattedScoreText()
    pub fn get_formatted_score_text() -> MutableComponent {
        return component::translatable_args("skyblocker.dungeons.dungeonScore.scoreText", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(DungeonScoreHUD::format_score(DungeonScore::get_score()))]);
    }

    /// java: private static Component formatScore(int score)
    pub fn format_score(mut score: i32) -> MutableComponent {
        if score < 100 {
            return (component::literal(&format!("{:03}", score))).with_color(0xDC1A1A).append(component::literal(" (D)").with_style_format(ChatFormatting::Gray)).append(EXTRA_SPACE.get_mut().clone());
        }
        if score < 160 {
            return (component::literal(&format!("{:03}", score))).with_color(0x4141FF).append(component::literal(" (C)").with_style_format(ChatFormatting::Gray)).append(EXTRA_SPACE.get_mut().clone());
        }
        if score < 230 {
            return (component::literal(&format!("{:03}", score))).with_color(0x7FCC19).append(component::literal(" (B)").with_style_format(ChatFormatting::Gray)).append(EXTRA_SPACE.get_mut().clone());
        }
        if score < 270 {
            return (component::literal(&format!("{:03}", score))).with_color(0x7F3FB2).append(component::literal(" (A)").with_style_format(ChatFormatting::Gray)).append(EXTRA_SPACE.get_mut().clone());
        }
        if score < 300 {
            return (component::literal(&format!("{:03}", score))).with_color(0xF1E252).append(component::literal(" (S)").with_style_format(ChatFormatting::Gray)).append(EXTRA_SPACE.get_mut().clone());
        }
        return (component::literal("").append((component::literal(&format!("{:03}", score))).with_color(0xF1E252).with_style_format(ChatFormatting::Bold))).append((component::literal(" (S+)")).with_style_format(ChatFormatting::Gray));
    }

}

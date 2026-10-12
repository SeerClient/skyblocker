//! Ported from de.hysky.skyblocker.skyblock.profileviewer.skills.SkillWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::profileviewer::utils::level_finder::LevelFinder;
use crate::de::hysky::skyblocker::skyblock::profileviewer::utils::level_finder::LevelInfo;
use crate::de::hysky::skyblocker::skyblock::profileviewer::utils::profile_viewer_utils::ProfileViewerUtils;
use crate::de::hysky::skyblocker::skyblock::tabhud::util::ico;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use crate::de::hysky::skyblocker::utils::formatters;
use crate::de::hysky::skyblocker::utils::render::gui_helper::GuiHelper;
use mcrust::java::awt::color::Color;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::font::Font;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;

/// java: private static final Identifier BAR_FILL = SkyblockerMod.id("bars/bar_fill");
pub static BAR_FILL: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("bars/bar_fill"));
/// java: private static final Identifier BAR_BACK = SkyblockerMod.id("bars/bar_back");
pub static BAR_BACK: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("bars/bar_back"));
/// java: private static final Font textRenderer = Minecraft.getInstance().font;
pub static TEXT_RENDERER: compat::MainThread<Font> = compat::MainThread::new(|| compat::font());
/// java: private static final Map<String, FlexibleItemStack> SKILL_LOGO = Map.ofEntries( Map.entry("Combat", Ico.STONE_SWORD), Map.entry("Farming", Ico.GOLDEN_HOE), Map.entry("Mining", Ico.STONE_PICKAXE), Map.
pub static SKILL_LOGO: compat::MainThread<std::collections::HashMap<String, compat::Shared<FlexibleItemStack>>> = compat::MainThread::new(|| std::collections::HashMap::from([("Combat".to_string(), ico::STONE_SWORD.get_mut().clone()), ("Farming".to_string(), ico::GOLDEN_HOE.get_mut().clone()), ("Mining".to_string(), ico::STONE_PICKAXE.get_mut().clone()), ("Foraging".to_string(), ico::JUNGLE_SAPLING.get_mut().clone()), ("Fishing".to_string(), ico::FISH_ROD.get_mut().clone()), ("Enchanting".to_string(), ico::ENCHANTING_TABLE.get_mut().clone()), ("Alchemy".to_string(), ico::BREWING_STAND.get_mut().clone()), ("Taming".to_string(), ico::SPAWN_EGG.get_mut().clone()), ("Carpentry".to_string(), ico::CRAFTING_TABLE.get_mut().clone()), ("Catacombs".to_string(), ProfileViewerUtils::create_skull("eyJ0ZXh0dXJlcyI6eyJTS0lOIjp7InVybCI6Imh0dHBzOi8vdGV4dHVyZXMubWluZWNyYWZ0Lm5ldC90ZXh0dXJlLzliNTY4OTViOTY1OTg5NmFkNjQ3ZjU4NTk5MjM4YWY1MzJkNDZkYjljMWIwMzg5YjhiYmViNzA5OTlkYWIzM2QifX19")), ("Runecraft".to_string(), ico::MAGMA_CREAM.get_mut().clone()), ("Social".to_string(), ico::EMERALD.get_mut().clone())]));
// TODO(fabric2seer): static call Object2IntMap.ofEntries(..)
//   java: private static final Object2IntMap<String> SKILL_CAP = Object2IntMap.ofEntries(
//   java: 			Object2IntMap.entry("Combat", 60),
//   java: 			Object2IntMap.entry("Farming", 60),
//   java: 			Object2IntMap.entry("Mining", 60),
//   java: 			Object2IntMap.entry("Foraging", 50),
//   java: 			Object2IntMap.entry("Fishing", 50),
//   java: 			Object2IntMap.entry("Enchanting", 60),
//   java: 			Object2IntMap.entry("Alchemy", 50),
//   java: 			Object2IntMap.entry("Taming", 60),
//   java: 			Object2IntMap.entry("Carpentry", 50),
//   java: 			Object2IntMap.entry("Catacombs", 50),
//   java: 			Object2IntMap.entry("Runecraft", 25),
//   java: 			Object2IntMap.entry("Social", 25)
//   java: 	);
/// java: private static final Object2IntMap<String> SKILL_CAP = Object2IntMap.ofEntries( Object2IntMap.entry("Combat", 60), Object2IntMap.entry("Farming", 60), Object2IntMap.entry("Mining", 60), Object2IntMap.
pub static SKILL_CAP: LazyLock<std::collections::HashMap<String, i32>> = LazyLock::new(|| todo!("fabric2seer: the initialiser of `SKILL_CAP` - see the TODO above"));
// TODO(fabric2seer): static call Object2IntMap.ofEntries(..)
//   java: private static final Object2IntMap<String> SOFT_SKILL_CAP = Object2IntMap.ofEntries(
//   java: 			Object2IntMap.entry("Taming", 50),
//   java: 			Object2IntMap.entry("Farming", 50)
//   java: 	);
/// java: private static final Object2IntMap<String> SOFT_SKILL_CAP = Object2IntMap.ofEntries( Object2IntMap.entry("Taming", 50), Object2IntMap.entry("Farming", 50) );
pub static SOFT_SKILL_CAP: LazyLock<std::collections::HashMap<String, i32>> = LazyLock::new(|| todo!("fabric2seer: the initialiser of `SOFT_SKILL_CAP` - see the TODO above"));
// TODO(fabric2seer): static call Object2IntMaps.singleton(..)
//   java: private static final Object2IntMap<String> INFINITE = Object2IntMaps.singleton("Catacombs", 0);
/// java: private static final Object2IntMap<String> INFINITE = Object2IntMaps.singleton("Catacombs", 0);
pub static INFINITE: LazyLock<std::collections::HashMap<String, i32>> = LazyLock::new(|| todo!("fabric2seer: the initialiser of `INFINITE` - see the TODO above"));

/// java: public class SkillWidget
#[derive(Clone)]
pub struct SkillWidget {
    /// java: private final String SKILL_NAME;
    pub skill_name: String,
    /// java: private final LevelFinder.LevelInfo SKILL_LEVEL;
    pub skill_level: compat::Shared<LevelInfo>,
    /// java: private final FlexibleItemStack stack;
    pub stack: Option<compat::Shared<FlexibleItemStack>>,
}

impl SkillWidget {
    /// java: public SkillWidget(String skill, long xp, int playerCap)
    pub fn new(mut skill: String, mut xp: i64, mut player_cap: i32) -> Self {
        let __init_skill_name = skill.clone();
        let __init_skill_level = LevelFinder::get_level_info(&skill, xp);
        let mut this = Self {
            skill_name: __init_skill_name,
            skill_level: __init_skill_level,
            stack: None,
        };
        if compat::read(&this.skill_level, |__t| __t.level) >= SKILL_CAP.get(&skill).copied().unwrap_or_default() && !(INFINITE.contains_key(&skill)) {
            { let __v = ((1) as f64) as f64; compat::write(&this.skill_level, |__t| __t.fill = __v); }
            { let __v = SKILL_CAP.get(&skill).copied().unwrap_or_default(); compat::write(&this.skill_level, |__t| __t.level = __v); }
        }
        this.stack = Some(SKILL_LOGO.get_mut().get(&skill).cloned().unwrap_or_else(|| ico::BARRIER.get_mut().clone()));
        if player_cap != -1 {
            // TODO(fabric2seer): static field SKILL_LEVEL.level
            //   java: this.SKILL_LEVEL.level = Math.min(SKILL_LEVEL.level, (SOFT_SKILL_CAP.getInt(this.SKILL_NAME) + playerCap));
        }
        this
    }

    /// java: public void extractRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, int x, int y)
    pub fn extract_render_state(&mut self, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut x: i32, mut y: i32) {
        graphics.item(&FlexibleItemStack::get_stack_or_throw(self.stack.as_ref().unwrap()), x.wrapping_add(3), y.wrapping_add(2));
        gui_graphics_extractor::text_string_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), Some(&format!("{} {}", self.skill_name, compat::read(&self.skill_level, |__t| __t.level))), x.wrapping_add(31), y.wrapping_add(2), compat::hash_code(&mcrust::java::awt::color::white()), false);
        let mut fill_color: Color = mcrust::java::awt::color::green();
        if compat::read(&self.skill_level, |__t| __t.level) >= SKILL_CAP.get(&self.skill_name).copied().unwrap_or_default() {
            fill_color = mcrust::java::awt::color::magenta();
        }
        if SOFT_SKILL_CAP.contains_key(&self.skill_name) && compat::read(&self.skill_level, |__t| __t.level) > SOFT_SKILL_CAP.get(&self.skill_name).copied().unwrap_or_default() && compat::read(&self.skill_level, |__t| __t.level) < SKILL_CAP.get(&self.skill_name).copied().unwrap_or_default() && compat::read(&self.skill_level, |__t| __t.fill) == (1 as f64) {
            fill_color = mcrust::java::awt::color::yellow();
        }
        gui_graphics_extractor::blit_sprite_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), BAR_BACK.get_mut(), x.wrapping_add(30), y.wrapping_add(12), 75, 6);
        GuiHelper::nine_slice_colored_gui_graphics_extractor_identifier_int_int_int_int_color(graphics, BAR_FILL.get_mut(), x.wrapping_add(30), y.wrapping_add(12), ((((75 as f64) * compat::read(&self.skill_level, |__t| __t.fill))) as i32), 6, &fill_color);
        if mouse_x > x.wrapping_add(30) && mouse_x < x.wrapping_add(105) && mouse_y > y.wrapping_add(10) && mouse_y < y.wrapping_add(19) {
            let mut tooltip_text: Vec<MutableComponent> = Vec::new();
            tooltip_text.push(component::literal(&self.skill_name).with_style_format(ChatFormatting::Green));
            // TODO(fabric2seer): static field SKILL_LEVEL.xp
            //   java: tooltipText.add(Component.literal("XP: " + Formatters.INTEGER_NUMBERS.format(this.SKILL_LEVEL.xp)).withStyle(ChatFormatting.GOLD));
            // TODO(fabric2seer): the port's `set_component_tooltip_for_next_frame` ports this Java method with other parameters (tooltip_for_next_frame, lines for Java's lines, xo, yo)
            // TODO(fabric2seer): `set_component_tooltip_for_next_frame` takes 2 argument(s) in the port (`tooltip_for_next_frame: &mut Option<DeferredTooltip>, lines: Vec<crate::net::minecraft::network::chat::formatted_text::FormattedText>`), Java passes 4
            // rust: gui_graphics_extractor::set_component_tooltip_for_next_frame(TEXT_RENDERER.get_mut(), tooltip_text.clone(), mouse_x, mouse_y);
        }
    }

}

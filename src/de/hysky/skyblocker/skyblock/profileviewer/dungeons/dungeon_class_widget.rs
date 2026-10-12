//! Ported from de.hysky.skyblocker.skyblock.profileviewer.dungeons.DungeonClassWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::item::item_protection::ItemProtection;
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

/// java: private static final int CLASS_CAP = 50;
pub const CLASS_CAP: i32 = 50;
/// java: private static final Identifier TEXTURE = SkyblockerMod.id("textures/gui/profile_viewer/icon_data_widget.png");
pub static TEXTURE: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("textures/gui/profile_viewer/icon_data_widget.png"));
/// java: private static final Identifier BAR_FILL = SkyblockerMod.id("bars/bar_fill");
pub static BAR_FILL: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("bars/bar_fill"));
/// java: private static final Identifier BAR_BACK = SkyblockerMod.id("bars/bar_back");
pub static BAR_BACK: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("bars/bar_back"));
/// java: private static final Font textRenderer = Minecraft.getInstance().font;
pub static TEXT_RENDERER: compat::MainThread<Font> = compat::MainThread::new(|| compat::font());
/// java: private static final Map<String, FlexibleItemStack> CLASS_ICON = Map.ofEntries( Map.entry("Healer", Ico.S_POTION), Map.entry("Mage", Ico.B_ROD), Map.entry("Berserk", Ico.IRON_SWORD), Map.entry("Archer
pub static CLASS_ICON: compat::MainThread<std::collections::HashMap<String, compat::Shared<FlexibleItemStack>>> = compat::MainThread::new(|| std::collections::HashMap::from([("Healer".to_string(), ico::S_POTION.get_mut().clone()), ("Mage".to_string(), ico::B_ROD.get_mut().clone()), ("Berserk".to_string(), ico::IRON_SWORD.get_mut().clone()), ("Archer".to_string(), ico::BOW.get_mut().clone()), ("Tank".to_string(), ico::CHESTPLATE.get_mut().clone())]));

/// java: public class DungeonClassWidget
#[derive(Clone)]
pub struct DungeonClassWidget {
    /// java: private final String className;
    pub class_name: String,
    /// java: private LevelFinder.LevelInfo classLevel;
    pub class_level: Option<compat::Shared<LevelInfo>>,
    /// java: private JsonObject classData;
    pub class_data: Option<serde_json::Map<String, serde_json::Value>>,
    /// java: private final FlexibleItemStack stack;
    pub stack: compat::Shared<FlexibleItemStack>,
    /// java: private boolean active = false;
    pub active: bool,
}

impl DungeonClassWidget {
    /// java: public DungeonClassWidget(String className, JsonObject playerProfile)
    pub fn new(mut class_name: String, mut player_profile: serde_json::Map<String, serde_json::Value>) -> Self {
        let __init_class_name = class_name.clone();
        let __init_stack = CLASS_ICON.get_mut().get(&class_name).cloned().unwrap_or_else(|| ico::BARRIER.get_mut().clone());
        let mut this = Self {
            class_name: __init_class_name,
            class_level: None,
            class_data: None,
            stack: __init_stack,
            active: false,
        };
        // java: try {..} catch (Exception _) - an exception here is a panic, caught by `catch_unwind`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            this.class_data = Some(compat::json::get_as_json_object_of(&(compat::json::get_as_json_object_of(&(compat::json::get_as_json_object_of(&player_profile, "dungeons")), "player_classes")), &this.class_name.to_lowercase()));
            this.class_level = Some(LevelFinder::get_level_info("Catacombs", compat::json::get_as_long(&this.class_data.as_ref().unwrap().get("experience").cloned().unwrap())));
            this.active = (compat::json::get_as_string(&(compat::json::get_as_json_object_of(&player_profile, "dungeons")).get("selected_dungeon_class").cloned().unwrap()) == class_name.to_lowercase());
        }));
        if let Err(_) = __result {
            this.class_level = Some(LevelFinder::get_level_info("", ((0) as i64) as i64));
        }
        this
    }

    /// java: public void extractRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, int x, int y)
    pub fn extract_render_state(&mut self, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut x: i32, mut y: i32) {
        gui_graphics_extractor::blit_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), TEXTURE.get_mut(), x, y, ((0) as f32) as f32, ((0) as f32) as f32, 109, 26, 109, 26);
        graphics.item(&FlexibleItemStack::get_stack_or_throw(&self.stack), x.wrapping_add(3), y.wrapping_add(5));
        if self.active {
            ItemProtection::draw_slot_icon(graphics, x.wrapping_add(3), y.wrapping_add(5));
        }
        // TODO(fabric2seer): `.getRGB(..)` on a value of unknown type
        //   java: graphics.text(textRenderer, className + " " + classLevel.level, x + 31, y + 5, Color.WHITE.getRGB(), false);
        let mut fill_color: Color = (if compat::read(self.class_level.as_ref().unwrap(), |__t| __t.level) >= CLASS_CAP { mcrust::java::awt::color::magenta() } else { mcrust::java::awt::color::green() });
        gui_graphics_extractor::blit_sprite_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), BAR_BACK.get_mut(), x.wrapping_add(30), y.wrapping_add(15), 75, 6);
        GuiHelper::nine_slice_colored_gui_graphics_extractor_identifier_int_int_int_int_color(graphics, BAR_FILL.get_mut(), x.wrapping_add(30), y.wrapping_add(15), ((((75 as f64) * compat::read(self.class_level.as_ref().unwrap(), |__t| __t.fill))) as i32), 6, &fill_color);
        if mouse_x > x.wrapping_add(30) && mouse_x < x.wrapping_add(105) && mouse_y > y.wrapping_add(12) && mouse_y < y.wrapping_add(22) {
            let mut tooltip_text: Vec<MutableComponent> = Vec::new();
            tooltip_text.push(component::literal(&self.class_name).with_style_format(ChatFormatting::Green));
            tooltip_text.push((component::literal(&format!("XP: {}", formatters::INTEGER_NUMBERS.get_mut().format((compat::read(self.class_level.as_ref().unwrap(), |__t| __t.xp)) as f64)))).with_style_format(ChatFormatting::Gold));
            // TODO(fabric2seer): the port's `set_component_tooltip_for_next_frame` ports this Java method with other parameters (tooltip_for_next_frame, lines for Java's lines, xo, yo)
            // TODO(fabric2seer): `set_component_tooltip_for_next_frame` takes 2 argument(s) in the port (`tooltip_for_next_frame: &mut Option<DeferredTooltip>, lines: Vec<crate::net::minecraft::network::chat::formatted_text::FormattedText>`), Java passes 4
            // rust: gui_graphics_extractor::set_component_tooltip_for_next_frame(TEXT_RENDERER.get_mut(), tooltip_text.clone(), mouse_x, mouse_y);
        }
    }

}

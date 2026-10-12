//! Ported from de.hysky.skyblocker.skyblock.dungeon.DungeonMapConfigScreen by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::dungeon::dungeon_score_hud::DungeonScoreHUD;
use crate::de::hysky::skyblocker::utils::render::gui_helper::GuiHelper;
use mcrust::mcrust::modding::screens::ModScreen;
use mcrust::mcrust::modding::screens::ScreenRenderContext;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::client::input::mouse_button_info::MouseButtonInfo;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final Identifier EXAMPLE_MAP = SkyblockerMod.id("textures/gui/example_dungeon_map.png");
pub static EXAMPLE_MAP: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("textures/gui/example_dungeon_map.png"));

/// java: public class DungeonMapConfigScreen extends Screen
pub struct DungeonMapConfigScreen {
    /// java: extends net.minecraft.client.gui.screens.Screen
    pub base: mcrust::net::minecraft::client::gui::screens::screen::Screen,
    /// java: private int mapX = SkyblockerConfigManager.get().dungeons.dungeonMap.mapX;
    pub map_x: i32,
    /// java: private int mapY = SkyblockerConfigManager.get().dungeons.dungeonMap.mapY;
    pub map_y: i32,
    /// java: private int scoreX = SkyblockerConfigManager.get().dungeons.dungeonScore.scoreX;
    pub score_x: i32,
    /// java: private int scoreY = SkyblockerConfigManager.get().dungeons.dungeonScore.scoreY;
    pub score_y: i32,
    /// java: private final @Nullable Screen parent;
    pub parent: Option<mcrust::net::minecraft::client::gui::screens::screen::Screen>,
    /// the screen's widgets as this object last saw them (see `compat::WidgetCache`)
    pub __widgets: compat::WidgetCache,
}

impl std::ops::Deref for DungeonMapConfigScreen {
    type Target = mcrust::net::minecraft::client::gui::screens::screen::Screen;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for DungeonMapConfigScreen {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl DungeonMapConfigScreen {
    /// java: protected DungeonMapConfigScreen()
    pub fn new() -> compat::Shared<Self> {
        let __this = Self::new_1(None);
        __this
    }

    /// java: public DungeonMapConfigScreen(@Nullable Screen parent)
    pub fn new_1(mut parent: Option<mcrust::net::minecraft::client::gui::screens::screen::Screen>) -> compat::Shared<Self> {
        let __init_parent = parent;
        let __this = compat::shared(Self {
            base: mcrust::net::minecraft::client::gui::screens::screen::Screen::new(component::literal("Dungeon Map Config").get_string()),
            __widgets: Default::default(),
            map_x: SkyblockerConfigManager::get().dungeons.dungeon_map.map_x,
            map_y: SkyblockerConfigManager::get().dungeons.dungeon_map.map_y,
            score_x: compat::read(&SkyblockerConfigManager::get().dungeons.dungeon_score, |__t| __t.score_x),
            score_y: compat::read(&SkyblockerConfigManager::get().dungeons.dungeon_score, |__t| __t.score_y),
            parent: __init_parent,
        });
        __this
    }

    /// java: @Override public void extractRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_render_state(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        ();
        DungeonMapConfigScreen::extract_hud_map(__this, graphics, compat::read(__this, |__t| __t.map_x), compat::read(__this, |__t| __t.map_y));
        DungeonMapConfigScreen::extract_hud_score(__this, graphics, compat::read(__this, |__t| __t.score_x), compat::read(__this, |__t| __t.score_y));
        // TODO(fabric2seer): `.getRGB(..)` on a value of unknown type
        //   java: graphics.centeredText(font, "Right Click To Reset Position", width >> 1, height >> 1, Color.GRAY.getRGB());
    }

    /// java: @Override public boolean mouseDragged(MouseButtonEvent click, double offsetX, double offsetY)
    pub fn mouse_dragged(__this: &compat::Shared<Self>, click: &MouseButtonEvent, mut offset_x: f64, mut offset_y: f64) -> bool {
        let mut map_size: i32 = ((((128 as f32) * SkyblockerConfigManager::get().dungeons.dungeon_map.map_scaling)) as i32);
        let mut score_scaling: f32 = compat::read(&SkyblockerConfigManager::get().dungeons.dungeon_score, |__t| __t.score_scaling);
        let mut score_width: i32 = ((((compat::font().width_of_cached(&mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(DungeonScoreHUD::get_formatted_score_text())) as f32) * score_scaling)) as i32);
        let mut score_height: i32 = ((((mcrust::net::minecraft::client::gui::font::LINE_HEIGHT as f32) * score_scaling)) as i32);
        if GuiHelper::point_is_in_area(click.x(), click.y(), ((compat::read(__this, |__t| __t.map_x)) as f64) as f64, ((compat::read(__this, |__t| __t.map_y)) as f64) as f64, (((compat::read(__this, |__t| __t.map_x)).wrapping_add(map_size)) as f64) as f64, (((compat::read(__this, |__t| __t.map_y)).wrapping_add(map_size)) as f64) as f64) && click.button() == 0 {
            { let __v = ((((click.x() - (map_size.wrapping_shr((1) as u32) as f64)).min(((compat::read(__this, |__t| __t.width)).wrapping_sub(map_size) as f64))).max((0 as f64))) as i32); compat::write(__this, |__t| __t.map_x = __v); }
            { let __v = ((((click.y() - (map_size.wrapping_shr((1) as u32) as f64)).min(((compat::read(__this, |__t| __t.height)).wrapping_sub(map_size) as f64))).max((0 as f64))) as i32); compat::write(__this, |__t| __t.map_y = __v); }
        } else if GuiHelper::point_is_in_area(click.x(), click.y(), ((compat::read(__this, |__t| __t.score_x)) as f64) as f64, ((compat::read(__this, |__t| __t.score_y)) as f64) as f64, (((compat::read(__this, |__t| __t.score_x)).wrapping_add(score_width)) as f64) as f64, (((compat::read(__this, |__t| __t.score_y)).wrapping_add(score_height)) as f64) as f64) && click.button() == 0 {
            { let __v = ((((click.x() - (score_width.wrapping_shr((1) as u32) as f64)).min(((compat::read(__this, |__t| __t.width)).wrapping_sub(score_width) as f64))).max((0 as f64))) as i32); compat::write(__this, |__t| __t.score_x = __v); }
            { let __v = ((((click.y() - (score_height.wrapping_shr((1) as u32) as f64)).min(((compat::read(__this, |__t| __t.height)).wrapping_sub(score_height) as f64))).max((0 as f64))) as i32); compat::write(__this, |__t| __t.score_y = __v); }
        }
        return false;
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    pub fn mouse_clicked(__this: &compat::Shared<Self>, click: &MouseButtonEvent, mut doubled: bool) -> bool {
        if click.button() == 1 {
            { let __v = 2; compat::write(__this, |__t| __t.map_x = __v); }
            { let __v = 2; compat::write(__this, |__t| __t.map_y = __v); }
            { let __v = (((((compat::read(__this, |__t| __t.map_x) as f32) + ((64 as f32) * SkyblockerConfigManager::get().dungeons.dungeon_map.map_scaling)) - (((compat::font().width_of_cached(&mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(DungeonScoreHUD::get_formatted_score_text())) as f32) * compat::read(&SkyblockerConfigManager::get().dungeons.dungeon_score, |__t| __t.score_scaling)) / (2 as f32)))) as i32).max(0); compat::write(__this, |__t| __t.score_x = __v); }
            { let __v = (((((compat::read(__this, |__t| __t.map_y) as f32) + ((128 as f32) * SkyblockerConfigManager::get().dungeons.dungeon_map.map_scaling)) + (4 as f32))) as i32); compat::write(__this, |__t| __t.score_y = __v); }
        }
        return false;
    }

    /// java: @Override public void onClose()
    pub fn on_close(__this: &compat::Shared<Self>) {
        let __this_1 = compat::RenderThread::new(__this.clone());
        SkyblockerConfigManager::update(Arc::new(move |mut config| {
            config.dungeons.dungeon_map.map_x = compat::read(__this_1.get(), |__t| __t.map_x);
            config.dungeons.dungeon_map.map_y = compat::read(__this_1.get(), |__t| __t.map_y);
            { let __v = compat::read(__this_1.get(), |__t| __t.score_x); compat::write(&config.dungeons.dungeon_score, |__t| __t.score_x = __v); }
            { let __v = compat::read(__this_1.get(), |__t| __t.score_y); compat::write(&config.dungeons.dungeon_score, |__t| __t.score_y = __v); }
        }));
        mcrust::mcrust::modding::game::set_screen_to_parent();
    }

    /// java: public void extractHUDMap(GuiGraphicsExtractor graphics, int x, int y)
    pub fn extract_hud_map(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, mut x: i32, mut y: i32) {
        let mut scaling: f32 = SkyblockerConfigManager::get().dungeons.dungeon_map.map_scaling;
        let mut size: i32 = ((((128 as f32) * scaling)) as i32);
        gui_graphics_extractor::blit_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), EXAMPLE_MAP.get_mut(), x, y, ((0) as f32) as f32, ((0) as f32) as f32, size, size, size, size);
        if SkyblockerConfigManager::get().dungeons.dungeon_map.show_outline {
            GuiHelper::border(graphics, x, y, size, size, mcrust::net::minecraft::util::common_colors::LIGHT_GRAY);
        }
    }

    /// java: public void extractHUDScore(GuiGraphicsExtractor graphics, int x, int y)
    pub fn extract_hud_score(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, mut x: i32, mut y: i32) {
        DungeonScoreHUD::extract_render_state_3(graphics, x, y);
    }

}

/// DungeonMapConfigScreen's `Screen` half (`super`'s fields and widget tree), lent by the game for each call
impl compat::ScreenHalf for DungeonMapConfigScreen {
    fn screen_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::screens::screen::Screen {
        &mut self.base
    }
    fn screen_parts(&mut self) -> (&mut mcrust::net::minecraft::client::gui::screens::screen::Screen, Option<&mut compat::WidgetCache>) {
        let __s = self;
        (&mut __s.base, Some(&mut __s.__widgets))
    }
}

/// DungeonMapConfigScreen as the game's screen: `game::set_screen(Box::new(compat::ScreenAdapter::new(..)))`
impl ModScreen for compat::ScreenAdapter<DungeonMapConfigScreen> {
    fn get_title(&self) -> String {
        self.title()
    }

    /// java: Screen.tick (the base body)
    fn tick(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        self.lend(screen, |_| ())
    }

    /// java: @Override public void extractRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_render_state(&mut self, screen: &mcrust::net::minecraft::client::gui::screens::screen::Screen, context: &mut ScreenRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.lend_ref(screen, |__h| DungeonMapConfigScreen::extract_render_state(__h, &mut context.graphics, mouse_x, mouse_y, a))
    }

    /// java: Screen.keyPressed (the base body)
    fn key_pressed(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, event: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        self.lend(screen, |_| ());
        if event.is_escape() && self.should_close_on_esc() {
            self.on_close();
            return true;
        }
        false
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent click, boolean doubled)
    fn mouse_clicked(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, x: f64, y: f64, button: i32) -> bool {
        self.lend(screen, |__h| DungeonMapConfigScreen::mouse_clicked(__h, &MouseButtonEvent::new(x, y, MouseButtonInfo::new(button, 0)), false))
    }

    /// java: @Override public boolean mouseDragged(MouseButtonEvent click, double offsetX, double offsetY)
    fn mouse_dragged(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, x: f64, y: f64, button: i32, dx: f64, dy: f64) -> bool {
        self.lend(screen, |__h| DungeonMapConfigScreen::mouse_dragged(__h, &MouseButtonEvent::new(x, y, MouseButtonInfo::new(button, 0)), dx, dy))
    }

    /// java: @Override public void onClose()
    fn on_close(&mut self) {
        self.call(|__h| DungeonMapConfigScreen::on_close(__h))
    }

}

// NOTE(fabric2seer): extractRenderState, mouseClicked, mouseDragged, onClose override(s) of Screen - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them

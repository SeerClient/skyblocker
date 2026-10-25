//! Ported from de.hysky.skyblocker.skyblock.dwarven.GlaciteColdOverlay by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler::Scheduler;
use crate::de::hysky::skyblocker::utils::utils;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::util::argb;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final Identifier POWDER_SNOW_OUTLINE = Identifier.withDefaultNamespace("textures/misc/powder_snow_outline.png");
pub static POWDER_SNOW_OUTLINE: compat::MainThread<Identifier> = compat::MainThread::new(|| Identifier::with_default_namespace("textures/misc/powder_snow_outline.png"));
/// java: private static final Pattern COLD_PATTERN = Pattern.compile("Cold: -(\\d+)❄");
pub static COLD_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("Cold: -(\\d+)❄"));
/// java: private static int cold = 0;
pub static COLD: LazyLock<Mutex<i32>> = LazyLock::new(|| Mutex::new(0));
/// java: private static long resetTime = System.currentTimeMillis();
pub static RESET_TIME: LazyLock<Mutex<i64>> = LazyLock::new(|| Mutex::new(compat::millis()));

/// java: public class GlaciteColdOverlay
#[derive(Clone)]
pub struct GlaciteColdOverlay {}

impl GlaciteColdOverlay {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        ({ let __r = scheduler::INSTANCE.get_mut(); let __f = compat::dispatch(__r, |__t| __t.schedule_cyclic()); __f(__r, Arc::new(move || { GlaciteColdOverlay::update(); }), 20) });
        // java: ClientReceiveMessageEvents.ALLOW_GAME.register(GlaciteColdOverlay::coldReset);
        compat::register_allow_game(|text: &mcrust::net::minecraft::network::chat::mutable_component::MutableComponent, overlay: bool| GlaciteColdOverlay::cold_reset(text, overlay));
        hud_render_callback::EVENT.register(Arc::new(|ctx: &mut hud_render_callback::HudRenderContext<'_>| {
            // SAFETY: `ctx` outlives every call made through `graphics` (this callback).
            let graphics = unsafe { ScriptGraphics::new(ctx.gl, &*ctx.minecraft, ctx.gui_width, ctx.gui_height) };
            let _session = graphics.session();
            // java: GlaciteColdOverlay::extract, the HUD element - the port's GuiGraphicsExtractor is stateless; the script
            // graphics session above scopes the pose stack for the duration of the element
            GlaciteColdOverlay::extract(&mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor::new());
        }));
    }

    /// java: private static boolean coldReset(Component text, boolean b)
    pub fn cold_reset(text: &MutableComponent, mut b: bool) -> bool {
        if !(Utils::is_in_dwarven_mines()) || b {
            return true;
        }
        let mut message: String = text.get_string();
        if message == "The warmth of the campfire reduced your ❄ Cold to 0!" {
            compat::set_state(&COLD, 0);
            compat::set_state(&RESET_TIME, compat::millis());
        }
        return true;
    }

    /// java: private static void update()
    pub fn update() {
        if !(Utils::is_in_dwarven_mines()) || (compat::millis()).wrapping_sub(compat::get_state(&RESET_TIME)) < (3000 as i64) || !(SkyblockerConfigManager::get().mining.glacite.cold_overlay) {
            compat::set_state(&COLD, 0);
            return;
        }
        for line in compat::get_state(&utils::STRING_SCOREBOARD) {
            let mut cold_matcher: compat::regex::Matcher = COLD_PATTERN.matcher(&line);
            if cold_matcher.matches() {
                let mut value: String = cold_matcher.group(1).unwrap();
                compat::set_state(&COLD, value.parse::<i32>().expect("NumberFormatException"));
                return;
            }
        }
        compat::set_state(&COLD, 0);
    }

    /// java: private static void extractOverlay(GuiGraphicsExtractor graphics, Identifier texture, float opacity)
    pub fn extract_overlay(graphics: &GuiGraphicsExtractor, texture: &Identifier, mut opacity: f32) {
        let mut white: i32 = argb::white_f32(opacity);
        gui_graphics_extractor::blit_identifier_with_color(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), texture, 0, 0, 0.0f32, 0.0f32, gui_graphics_extractor::gui_width(&*compat::minecraft()), gui_graphics_extractor::gui_height(&*compat::minecraft()), gui_graphics_extractor::gui_width(&*compat::minecraft()), gui_graphics_extractor::gui_height(&*compat::minecraft()), white);
    }

    /// java: public static void extract(GuiGraphicsExtractor graphics)
    pub fn extract(graphics: &GuiGraphicsExtractor) {
        if Utils::is_in_dwarven_mines() && SkyblockerConfigManager::get().mining.glacite.cold_overlay {
            GlaciteColdOverlay::extract_overlay(graphics, POWDER_SNOW_OUTLINE.get_mut(), ((compat::get_state(&COLD) as f32) / 100.0f32));
        }
    }

}

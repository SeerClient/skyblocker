//! Ported from de.hysky.skyblocker.skyblock.dungeon.FireFreezeStaffTimer by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::events::server_tick_callback;
use mcrust::net::minecraft::chat_formatting;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::font::Font;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final Identifier FIRE_FREEZE_STAFF_TIMER = SkyblockerMod.id("fire_freeze_staff_timer");
pub static FIRE_FREEZE_STAFF_TIMER: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("fire_freeze_staff_timer"));
/// java: private static long fireFreezeTimer;
pub static FIRE_FREEZE_TIMER: LazyLock<Mutex<i64>> = LazyLock::new(|| Mutex::new(0));
/// java: private static boolean timerActive = false;
pub static TIMER_ACTIVE: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));

/// java: public class FireFreezeStaffTimer
#[derive(Clone)]
pub struct FireFreezeStaffTimer {}

impl FireFreezeStaffTimer {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        hud_render_callback::EVENT.register(Arc::new(|ctx: &mut hud_render_callback::HudRenderContext<'_>| {
            let mut client = (&mut *ctx.minecraft);
            if client.gui.screen().is_some() {
                return;
            }
            if SkyblockerConfigManager::get().dungeons.the_professor.fire_freeze_staff_timer && compat::get_state(&FIRE_FREEZE_TIMER) != (0 as i64) {
                if compat::get_state(&FIRE_FREEZE_TIMER) <= (-5000 as i64) {
                    FireFreezeStaffTimer::reset();
                    return;
                }
                let mut message: MutableComponent;
                if compat::get_state(&FIRE_FREEZE_TIMER) > (0 as i64) {
                    message = component::literal("in ").append((component::literal(&format!("{}s", format!("{:.2}", (((compat::get_state(&FIRE_FREEZE_TIMER)) as f32) / (1000 as f32)))))).with_style_format(ChatFormatting::Yellow));
                } else {
                    message = component::literal("NOW").with_style_format(ChatFormatting::Red);
                }
                let mut renderer: Font = compat::font();
                let mut width: i32 = (client.get_window().get_gui_scaled_width()).wrapping_div(2);
                let mut height: i32 = (client.get_window().get_gui_scaled_height()).wrapping_div(2);
                mcrust::net::minecraft::client::gui::gui_graphics_extractor::centered_text(ctx.gl, &*client, &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::literal("Fire Freeze ").append(message.clone())), width, height, mcrust::net::minecraft::util::common_colors::WHITE);
            }
        }));
        // TODO(fabric2seer): ClientReceiveMessageEvents.ALLOW_GAME.register: no SeerClient equivalent yet
        //   java: ClientReceiveMessageEvents.ALLOW_GAME.register(FireFreezeStaffTimer::onChatMessage);
        client_play_connection_events::JOIN.register(Arc::new(|listener: &mut ClientPacketListener| {
            FireFreezeStaffTimer::reset();
        }));
        server_tick_callback::EVENT.register(Arc::new(move || { FireFreezeStaffTimer::on_server_tick(); }));
    }

    /// java: private static void onServerTick()
    pub fn on_server_tick() {
        if compat::get_state(&TIMER_ACTIVE) {
            compat::set_state(&FIRE_FREEZE_TIMER, (compat::get_state(&FIRE_FREEZE_TIMER)).wrapping_sub(50 as i64));
        }
    }

    /// java: private static void extractRenderState(GuiGraphicsExtractor graphics, DeltaTracker tickCounter)
    pub fn extract_render_state(graphics: &GuiGraphicsExtractor, tick_counter: &crate::unresolved::DeltaTracker) {
        let client = compat::minecraft();
        if client.gui.screen().is_some() {
            return;
        }
        if SkyblockerConfigManager::get().dungeons.the_professor.fire_freeze_staff_timer && compat::get_state(&FIRE_FREEZE_TIMER) != (0 as i64) {
            if compat::get_state(&FIRE_FREEZE_TIMER) <= (-5000 as i64) {
                FireFreezeStaffTimer::reset();
                return;
            }
            let mut message: MutableComponent;
            if compat::get_state(&FIRE_FREEZE_TIMER) > (0 as i64) {
                message = component::literal("in ").append((component::literal(&format!("{}s", format!("{:.2}", (((compat::get_state(&FIRE_FREEZE_TIMER)) as f32) / (1000 as f32)))))).with_style_format(ChatFormatting::Yellow));
            } else {
                message = component::literal("NOW").with_style_format(ChatFormatting::Red);
            }
            let mut renderer: Font = compat::font();
            let mut width: i32 = (client.get_window().get_gui_scaled_width()).wrapping_div(2);
            let mut height: i32 = (client.get_window().get_gui_scaled_height()).wrapping_div(2);
            gui_graphics_extractor::centered_text(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::literal("Fire Freeze ").append(message.clone())), width, height, mcrust::net::minecraft::util::common_colors::WHITE);
        }
    }

    /// java: private static void reset()
    pub fn reset() {
        compat::set_state(&FIRE_FREEZE_TIMER, ((0) as i64) as i64);
        compat::set_state(&TIMER_ACTIVE, false);
    }

    /// java: private static boolean onChatMessage(Component text, boolean overlay)
    pub fn on_chat_message(text: &MutableComponent, mut overlay: bool) -> bool {
        if !overlay && SkyblockerConfigManager::get().dungeons.the_professor.fire_freeze_staff_timer && chat_formatting::strip_formatting(&text.get_string()) == "[BOSS] The Professor: Oh? You found my Guardians' one weakness?" {
            compat::set_state(&FIRE_FREEZE_TIMER, 5700);
            compat::set_state(&TIMER_ACTIVE, true);
        }
        return true;
    }

}

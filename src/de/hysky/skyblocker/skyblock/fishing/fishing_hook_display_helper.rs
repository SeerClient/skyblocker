//! Ported from de.hysky.skyblocker.skyblock.fishing.FishingHookDisplayHelper by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::configs::helper_config::FishingHookDisplay;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::fishing::fishing_hud_widget::FishingHudWidget;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element_based_widget::ElementBasedWidget;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::player::local_player::LocalPlayer;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::world::entity::decoration::armor_stand::ArmorStand;
use mcrust::net::minecraft::network::chat::component::Component as _;
use mcrust::net::minecraft::world::entity::entity::Entity as _;

/// java: protected static ArmorStand fishingHookArmorStand;
pub static FISHING_HOOK_ARMOR_STAND: compat::MainThread<Option<ArmorStand>> = compat::MainThread::new(|| None);
/// java: private static final Identifier FISHING_HOOK_DISPLAY = SkyblockerMod.id("fishing_hook_display");
pub static FISHING_HOOK_DISPLAY: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("fishing_hook_display"));
/// java: static Pattern pattern = Pattern.compile("\\d.\\d");
pub static PATTERN: LazyLock<Mutex<compat::regex::Pattern>> = LazyLock::new(|| Mutex::new(compat::regex::Pattern::compile("\\d.\\d")));

/// java: public class FishingHookDisplayHelper
#[derive(Clone)]
pub struct FishingHookDisplayHelper {}

impl FishingHookDisplayHelper {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        client_play_connection_events::JOIN.register(Arc::new(|listener: &mut ClientPacketListener| {
            *FISHING_HOOK_ARMOR_STAND.get_mut() = None;
        }));
        hud_render_callback::EVENT.register(Arc::new(|ctx: &mut hud_render_callback::HudRenderContext<'_>| {
            // SAFETY: `ctx` outlives every call made through `graphics` (this callback).
            let graphics = unsafe { ScriptGraphics::new(ctx.gl, &*ctx.minecraft, ctx.gui_width, ctx.gui_height) };
            let _session = graphics.session();
            if SkyblockerConfigManager::get().helpers.fishing.fishing_hook_display == FishingHookDisplay::Off {
                return;
            }
            if FISHING_HOOK_ARMOR_STAND.get_mut().is_none() || !(FISHING_HOOK_ARMOR_STAND.get_mut().as_ref().unwrap().base.is_alive()) || !(FISHING_HOOK_ARMOR_STAND.get_mut().as_ref().unwrap().base.base.has_custom_name()) {
                *FISHING_HOOK_ARMOR_STAND.get_mut() = None;
                return;
            }
            let mut client = (&mut *ctx.minecraft);
            // TODO(fabric2seer): `LocalPlayer.fishing` has no Rust field in the port (searched net.minecraft.client.player.LocalPlayer and its supertypes)
            // TODO(fabric2seer): the mod keeps the game's `FishingHook` (Java holds a reference to the live object); the port's `FishingHook` is not a value to copy and the port lends the mod no handle to it
            if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() || mcrust::mcrust::modding::game::with_player(|player| player.fishing.clone()).is_none() || FISHING_HOOK_ARMOR_STAND.get_mut().is_none() {
                return;
            }
            if SkyblockerConfigManager::get().helpers.fishing.fishing_hook_display == FishingHookDisplay::Crosshair {
                let mut armor_stand_name: MutableComponent = compat::component_of(FISHING_HOOK_ARMOR_STAND.get_mut().as_ref().unwrap().get_name());
                let mut screen_width: i32 = client.get_window().get_gui_scaled_width();
                let mut screen_height: i32 = client.get_window().get_gui_scaled_height();
                let mut x: i32 = screen_width.wrapping_div(2);
                let mut y: i32 = screen_height.wrapping_div(2);
                graphics.push_pose();
                graphics.scale((3.0f32) as f32, (3.0f32) as f32);
                graphics.centered_text(armor_stand_name.to_string(), ((((x as f32) / 3.0f32)) as i32), ((((y as f32) / 3.0f32)) as i32), 0xFFFFFF00u32 as i32);
                let _ = graphics.pop_pose();
            } else {
                ({ let __r = &FishingHudWidget::get_instance(); let __f = compat::dispatch(__r, |__t| __t.update()); __f(&compat::view(__r, |a| a as compat::Shared<dyn ElementBasedWidget>)) });
            }
        }));
    }

    /// java: public static void render(GuiGraphicsExtractor graphics, DeltaTracker tickDelta)
    pub fn render(graphics: &GuiGraphicsExtractor, tick_delta: &crate::unresolved::DeltaTracker) {
        if SkyblockerConfigManager::get().helpers.fishing.fishing_hook_display == FishingHookDisplay::Off {
            return;
        }
        if FISHING_HOOK_ARMOR_STAND.get_mut().is_none() || !(FISHING_HOOK_ARMOR_STAND.get_mut().as_ref().unwrap().base.is_alive()) || !(FISHING_HOOK_ARMOR_STAND.get_mut().as_ref().unwrap().base.base.has_custom_name()) {
            *FISHING_HOOK_ARMOR_STAND.get_mut() = None;
            return;
        }
        let client = compat::minecraft();
        // TODO(fabric2seer): `LocalPlayer.fishing` has no Rust field in the port (searched net.minecraft.client.player.LocalPlayer and its supertypes)
        // TODO(fabric2seer): the mod keeps the game's `FishingHook` (Java holds a reference to the live object); the port's `FishingHook` is not a value to copy and the port lends the mod no handle to it
        if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() || mcrust::mcrust::modding::game::with_player(|player| player.fishing.clone()).is_none() || FISHING_HOOK_ARMOR_STAND.get_mut().is_none() {
            return;
        }
        if SkyblockerConfigManager::get().helpers.fishing.fishing_hook_display == FishingHookDisplay::Crosshair {
            let mut armor_stand_name: MutableComponent = compat::component_of(FISHING_HOOK_ARMOR_STAND.get_mut().as_ref().unwrap().get_name());
            let mut screen_width: i32 = client.get_window().get_gui_scaled_width();
            let mut screen_height: i32 = client.get_window().get_gui_scaled_height();
            let mut x: i32 = screen_width.wrapping_div(2);
            let mut y: i32 = screen_height.wrapping_div(2);
            let __pose_1 = gui_graphics_extractor::push_pose();
            gui_graphics_extractor::pose_scale((3.0f32) as f32, (3.0f32) as f32);
            gui_graphics_extractor::centered_text(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(armor_stand_name.clone()), ((((x as f32) / 3.0f32)) as i32), ((((y as f32) / 3.0f32)) as i32), 0xFFFFFF00u32 as i32);
            gui_graphics_extractor::pop_pose(__pose_1);
        } else {
            ({ let __r = &FishingHudWidget::get_instance(); let __f = compat::dispatch(__r, |__t| __t.update()); __f(&compat::view(__r, |a| a as compat::Shared<dyn ElementBasedWidget>)) });
        }
    }

    /// java: public static void onArmorStandSpawn(ArmorStand armorStand)
    pub fn on_armor_stand_spawn(mut armor_stand: ArmorStand) {
        if SkyblockerConfigManager::get().helpers.fishing.fishing_hook_display == FishingHookDisplay::Off {
            return;
        }
        if FISHING_HOOK_ARMOR_STAND.get_mut().is_some() {
            return;
        }
        let client = compat::minecraft();
        // TODO(fabric2seer): `LocalPlayer.fishing` has no Rust field in the port (searched net.minecraft.client.player.LocalPlayer and its supertypes)
        // TODO(fabric2seer): the mod keeps the game's `FishingHook` (Java holds a reference to the live object); the port's `FishingHook` is not a value to copy and the port lends the mod no handle to it
        if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() || mcrust::mcrust::modding::game::with_player(|player| player.fishing.clone()).is_none() {
            return;
        }
        // TODO(fabric2seer): `LocalPlayer.fishing` has no Rust field in the port (searched net.minecraft.client.player.LocalPlayer and its supertypes)
        // rust: let mut distance: f64 = armor_stand.base.base.position().distance_to(&mcrust::mcrust::modding::game::with_player(|player| player.fishing.base.base.position()).unwrap());
        let mut distance: f64 = todo!("fabric2seer: the initialiser of `distance` (TODO above)");
        if distance > 0.1 {
            return;
        }
        let mut matcher: compat::regex::Matcher = compat::lock(&PATTERN).matcher(&compat::component_of(armor_stand.get_name()).get_string());
        if armor_stand.base.base.has_custom_name() && matcher.matches() {
            // TODO(fabric2seer): the mod keeps the game's `ArmorStand` (Java holds a reference to the live object); the port's `ArmorStand` is not a value to copy and the port lends the mod no handle to it
            // rust: *FISHING_HOOK_ARMOR_STAND.get_mut() = Some(armor_stand.clone());
        } else {
            *FISHING_HOOK_ARMOR_STAND.get_mut() = None;
        }
    }

}

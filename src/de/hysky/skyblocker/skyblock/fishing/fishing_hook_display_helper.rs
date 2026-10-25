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
/// (the armor stand's entity id: the level keeps the entity, `game::with_entity` reaches it)
pub static FISHING_HOOK_ARMOR_STAND: compat::MainThread<Option<i32>> = compat::MainThread::new(|| None);
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
        // java: HudElementRegistry.attachElementAfter(VanillaHudElements.TITLE_AND_SUBTITLE, FISHING_HOOK_DISPLAY, FishingHookDisplayHelper::render);
        hud_element_registry::attach_element_after(vanilla_hud_elements::TITLE_AND_SUBTITLE, FISHING_HOOK_DISPLAY.get(), Arc::new(|ctx: &mut hud_render_callback::HudRenderContext<'_>| { FishingHookDisplayHelper::render(ctx) }));
    }

    /// java: public static void render(GuiGraphicsExtractor graphics, DeltaTracker tickDelta)
    pub fn render(ctx: &mut hud_render_callback::HudRenderContext<'_>) {
        // SAFETY: `ctx` outlives every call made through `graphics` (this callback).
        let graphics = unsafe { ScriptGraphics::new(ctx.gl, &*ctx.minecraft, ctx.gui_width, ctx.gui_height) };
        let _session = graphics.session();
        if SkyblockerConfigManager::get().helpers.fishing.fishing_hook_display == FishingHookDisplay::Off {
            return;
        }
        if !FishingHookDisplayHelper::armor_stand_valid() {
            *FISHING_HOOK_ARMOR_STAND.get_mut() = None;
            return;
        }
        let client = &*ctx.minecraft;
        if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() || FishingHookDisplayHelper::player_fishing().is_none() || FISHING_HOOK_ARMOR_STAND.get_mut().is_none() {
            return;
        }
        if SkyblockerConfigManager::get().helpers.fishing.fishing_hook_display == FishingHookDisplay::Crosshair {
            let mut armor_stand_name: MutableComponent = FishingHookDisplayHelper::armor_stand_name();
            let mut screen_width: i32 = client.get_window().get_gui_scaled_width();
            let mut screen_height: i32 = client.get_window().get_gui_scaled_height();
            let mut x: i32 = screen_width.wrapping_div(2);
            let mut y: i32 = screen_height.wrapping_div(2);
            graphics.push_pose();
            graphics.scale((3.0f32) as f32, (3.0f32) as f32);
            gui_graphics_extractor::centered_text(ctx.gl, &*ctx.minecraft, &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(armor_stand_name.clone()), ((((x as f32) / 3.0f32)) as i32), ((((y as f32) / 3.0f32)) as i32), 0xFFFFFF00u32 as i32);
            let _ = graphics.pop_pose();
        } else {
            ({ let __r = &FishingHudWidget::get_instance(); let __f = compat::dispatch(__r, |__t| __t.update()); __f(&compat::view(__r, |a| a as compat::Shared<dyn ElementBasedWidget>)) });
        }
    }

    /// java: public static void onArmorStandSpawn(ArmorStand armorStand)
    pub fn on_armor_stand_spawn(armor_stand: &ArmorStand) {
        if SkyblockerConfigManager::get().helpers.fishing.fishing_hook_display == FishingHookDisplay::Off {
            return;
        }
        if FISHING_HOOK_ARMOR_STAND.get_mut().is_some() {
            return;
        }
        let Some(fishing) = FishingHookDisplayHelper::player_fishing() else {
            return;
        };
        // java: double distance = armorStand.position().distanceTo(client.player.fishing.position());
        let Some(fishing_position) = mcrust::mcrust::modding::game::with_entity(fishing, |hook| hook.base().position()) else {
            return;
        };
        let mut distance: f64 = armor_stand.base.base.position().distance_to(&fishing_position);
        if distance > 0.1 {
            return;
        }
        let mut matcher: compat::regex::Matcher = compat::lock(&PATTERN).matcher(&compat::component_of(armor_stand.get_name()).get_string());
        if armor_stand.base.base.has_custom_name() && matcher.matches() {
            *FISHING_HOOK_ARMOR_STAND.get_mut() = Some(armor_stand.base.base.get_id());
        } else {
            *FISHING_HOOK_ARMOR_STAND.get_mut() = None;
        }
    }

    /// `client.player.fishing`: the id of the player's fishing hook (the port keeps the hook by id).
    pub fn player_fishing() -> Option<i32> {
        use mcrust::net::minecraft::world::entity::player::player::Player as _;
        mcrust::mcrust::modding::game::with_player(|player| player.player().fishing).flatten()
    }

    /// java: `fishingHookArmorStand != null && fishingHookArmorStand.isAlive() && fishingHookArmorStand.hasCustomName()`
    /// (an armor stand the level no longer has is removed, so not alive)
    fn armor_stand_valid() -> bool {
        let Some(id) = *FISHING_HOOK_ARMOR_STAND.get() else {
            return false;
        };
        mcrust::mcrust::modding::game::with_entity(id, |stand| mcrust::net::minecraft::world::entity::entity_selector::is_alive(stand) && stand.base().has_custom_name()).unwrap_or(false)
    }

    /// java: `fishingHookArmorStand.getName()`
    pub fn armor_stand_name() -> MutableComponent {
        let id = FISHING_HOOK_ARMOR_STAND.get().expect("the fishing hook armor stand");
        compat::component_of(mcrust::mcrust::modding::game::with_entity(id, |stand| stand.get_name()).unwrap_or(mcrust::net::minecraft::network::chat::formatted_text::FormattedText::Empty))
    }

}

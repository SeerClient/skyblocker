//! Ported from de.hysky.skyblocker.skyblock.fancybars.FancyStatusBars by fabric2seer.

#[allow(unused_imports)]
use mcrust::com::mojang::brigadier::builder::argument_builder::ArgumentBuilder as _;
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::configs::ui_and_visuals_config::IntelligenceDisplay;
use crate::de::hysky::skyblocker::config::configs::ui_and_visuals_config::LegacyBarPosition;
use crate::de::hysky::skyblocker::config::configs::ui_and_visuals_config::LegacyBarPositions;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::events::skyblock_events;
use crate::de::hysky::skyblocker::skyblock::fancybars::bar_positioner::BarAnchor;
use crate::de::hysky::skyblocker::skyblock::fancybars::bar_positioner::BarPositioner;
use crate::de::hysky::skyblocker::skyblock::fancybars::bar_positioner::SizeRule;
use crate::de::hysky::skyblocker::skyblock::fancybars::status_bar::StatusBar;
use crate::de::hysky::skyblocker::skyblock::fancybars::status_bar_type::StatusBarType;
use crate::de::hysky::skyblocker::skyblock::status_bar_tracker::EstimatedResource;
use crate::de::hysky::skyblocker::skyblock::status_bar_tracker::Resource;
use crate::de::hysky::skyblocker::skyblock::status_bar_tracker::StatusBarTracker;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use crate::entry::skyblocker_mod;
use mcrust::java::util::concurrent::completable_future::CompletableFuture;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::navigation::screen_position::ScreenPosition;
use mcrust::net::minecraft::client::player::local_player::LocalPlayer;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::util::mth;
use mcrust::net::minecraft::world::entity::entity::Entity as _;

/// java: private static final Identifier HUD_LAYER = SkyblockerMod.id("fancy_status_bars");
pub static HUD_LAYER: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("fancy_status_bars"));
/// java: private static final Path FILE = SkyblockerMod.CONFIG_DIR.resolve("status_bars.json");
/// (read by `loadBarConfig` on the worker: a `LazyLock`, first touched in `init` on the render thread, where
/// `CONFIG_DIR` lives)
pub static FILE: LazyLock<std::path::PathBuf> = LazyLock::new(|| skyblocker_mod::CONFIG_DIR.get_mut().join("status_bars.json"));

/// Fabric's `HudElement` - the registry's (`extractRenderState(GuiGraphicsExtractor, DeltaTracker)` as one context)
use mcrust::mcrust::modding::hud_element_registry::HudElement;
/// java: public static final FancyStatusBars INSTANCE = new FancyStatusBars();
pub static INSTANCE: compat::MainThread<compat::Shared<FancyStatusBars>> = compat::MainThread::new(|| FancyStatusBars::new());

/// OPTIMIZED BUT NOT FULLY VANILLA (perf round 1, P02-mods-runtime): `SKYBLOCKER_VANILLA_FANCY_BAR_READS=1` makes
/// `extractRenderState` read the player with one `with_player` per read, as before.
static VANILLA_FANCY_BAR_READS: compat::EnvSwitch = compat::EnvSwitch::new("SKYBLOCKER_VANILLA_FANCY_BAR_READS");

/// java: public class FancyStatusBars
#[derive(Clone)]
pub struct FancyStatusBars {
    /// java: public BarPositioner barPositioner = new BarPositioner();
    pub bar_positioner: BarPositioner,
    /// java: public Map<StatusBarType, StatusBar> statusBars = new EnumMap<>(StatusBarType.class);
    /// (an EnumMap iterates in ordinal order: `StatusBarType`'s `Ord` is its declaration order)
    pub status_bars: std::collections::BTreeMap<StatusBarType, compat::Shared<dyn StatusBar>>,
    /// java: private boolean updatePositionsNextFrame;
    pub update_positions_next_frame: bool,
    /// java: private boolean configLoaded = false;
    pub config_loaded: bool,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl FancyStatusBars {
    /// java: the implicit no-argument constructor
    pub fn new() -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            self_ref: std::sync::Weak::new(),
            bar_positioner: BarPositioner::new(),
            status_bars: std::collections::BTreeMap::new(),
            update_positions_next_frame: false,
            config_loaded: false,
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        __this
    }

    /// java: @Init public static void initStatic()
    pub fn init_static() {
        FancyStatusBars::init(INSTANCE.get_mut());
    }

    /// java: @SuppressWarnings("incomplete-switch") private static void updateLegacyPosition(StatusBar bar, int[] counts, UIAndVisualsConfig.LegacyBarPosition position)
    pub fn update_legacy_position(bar: &compat::Shared<dyn StatusBar>, counts: &mut Vec<i32>, mut position: LegacyBarPosition) {
        match position {
            LegacyBarPosition::Right => {
                { let __v = Some(BarAnchor::HotbarRight); compat::write(bar, |__t| __t.status_bar_base_mut().anchor = __v); }
                { let __v = 0; compat::write(bar, |__t| __t.status_bar_base_mut().grid_y = __v); }
                { let __v = { let __v = counts[(position.ordinal()) as usize]; counts[(position.ordinal()) as usize] += 1; __v }; compat::write(bar, |__t| __t.status_bar_base_mut().grid_x = __v); }
            }
            LegacyBarPosition::Layer1 => {
                { let __v = Some(BarAnchor::HotbarTop); compat::write(bar, |__t| __t.status_bar_base_mut().anchor = __v); }
                { let __v = 0; compat::write(bar, |__t| __t.status_bar_base_mut().grid_y = __v); }
                { let __v = { let __v = counts[(position.ordinal()) as usize]; counts[(position.ordinal()) as usize] += 1; __v }; compat::write(bar, |__t| __t.status_bar_base_mut().grid_x = __v); }
            }
            LegacyBarPosition::Layer2 => {
                { let __v = Some(BarAnchor::HotbarTop); compat::write(bar, |__t| __t.status_bar_base_mut().anchor = __v); }
                { let __v = 1; compat::write(bar, |__t| __t.status_bar_base_mut().grid_y = __v); }
                { let __v = { let __v = counts[(position.ordinal()) as usize]; counts[(position.ordinal()) as usize] += 1; __v }; compat::write(bar, |__t| __t.status_bar_base_mut().grid_x = __v); }
            }
            _ => {}
        }
    }

    /// java: public static @Nullable JsonObject loadBarConfig()
    pub fn load_bar_config() -> Option<serde_json::Map<String, serde_json::Value>> {
        // java: try (BufferedReader reader = Files.newBufferedReader(FILE)) { return SkyblockerMod.GSON.fromJson(reader, JsonObject.class); }
        match std::fs::read_to_string(&*FILE) {
            Ok(content) => {
                // GSON.fromJson answers null for an empty document, and throws for one that is not an object
                if content.trim().is_empty() {
                    return None;
                }
                match serde_json::from_str::<serde_json::Value>(&content) {
                    Ok(serde_json::Value::Object(object)) => return Some(object),
                    Ok(serde_json::Value::Null) => return None,
                    Ok(other) => {
                        logger().error(&format!("[Skyblocker] Failed to load status bars config {}", format!("Expected a com.google.gson.JsonObject but was {}", other)));
                    }
                    Err(e) => {
                        logger().error(&format!("[Skyblocker] Failed to load status bars config {e}"));
                    }
                }
            }
            // java: catch (NoSuchFileException _)
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                logger().warn("[Skyblocker] No status bar config file found, using defaults");
            }
            // java: catch (Exception e)
            Err(e) => {
                logger().error(&format!("[Skyblocker] Failed to load status bars config {e}"));
            }
        }
        return None;
    }

    /// java: public boolean isHealthFancyBarEnabled()
    pub fn is_health_fancy_bar_enabled(__this: &compat::Shared<Self>) -> bool {
        return FancyStatusBars::is_bar_enabled(__this, StatusBarType::Health);
    }

    /// java: public boolean isExperienceFancyBarEnabled()
    pub fn is_experience_fancy_bar_enabled(__this: &compat::Shared<Self>) -> bool {
        return FancyStatusBars::is_bar_enabled(__this, StatusBarType::Experience);
    }

    /// java: public boolean isBarEnabled(StatusBarType type)
    pub fn is_bar_enabled(__this: &compat::Shared<Self>, mut r#type: StatusBarType) -> bool {
        let mut status_bar: compat::Shared<dyn StatusBar> = __this.lock().borrow().status_bars.get(&r#type).cloned().unwrap();
        return (crate::de::hysky::skyblocker::debug::debug::Debug::is_test_environment() || compat::read(&status_bar, |__t| __t.status_bar_base().enabled) || compat::read(&status_bar, |__t| __t.status_bar_base().in_mouse));
    }

    /// java: public void makeVitalityVisible()
    pub fn make_vitality_visible(__this: &compat::Shared<Self>) {
        { let __v = true; compat::write(&__this.lock().borrow().status_bars.get(&StatusBarType::Vitality).cloned().unwrap(), |__t| __t.status_bar_base_mut().visible = __v); }
        { let __v = true; compat::write(__this, |__t| __t.update_positions_next_frame = __v); }
    }

    /// java: @SuppressWarnings("deprecation") public void init()
    pub fn init(__this: &compat::Shared<Self>) {
        // java: Function<HudElement, HudElement> hideIfFancyStatusBarsEnabled = hudElement -> {
        //   if (shouldProcess()) return (_, _) -> {}; return hudElement; };
        let __this_1 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        let hide_if_fancy_status_bars_enabled: Arc<hud_element_registry::HudElementReplacer> = Arc::new(move |hud_element: Arc<HudElement>| -> Arc<HudElement> {
            let Some(this) = __this_1.get().upgrade() else { return hud_element; };
            if FancyStatusBars::should_process(&this) {
                // OPTIMIZED BUT NOT FULLY VANILLA (perf round 1, P02-mods-runtime): the shared empty element
                // (`compat::empty_hud_element`). Was:
                //     let empty: Arc<HudElement> = Arc::new(|_: &mut hud_render_callback::HudRenderContext<'_>| {});
                return compat::empty_hud_element();
            }
            hud_element
        });
        // java: HudElementRegistry.replaceElement(VanillaHudElements.HEALTH_BAR, hudElement -> {
        //   if (!shouldProcess()) return hudElement;
        //   if (isHealthFancyBarEnabled()) { return (_, _) -> {}; }
        //   else if (isExperienceFancyBarEnabled()) { return (context, tickCounter) -> { Matrix3x2fStack pose = context.pose();
        //     pose.pushMatrix(); pose.translate(0, 6); hudElement.extractRenderState(context, tickCounter); pose.popMatrix(); }; }
        //   return hudElement; });
        let __this_health = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        hud_element_registry::replace_element(vanilla_hud_elements::HEALTH_BAR, Arc::new(move |hud_element: Arc<HudElement>| -> Arc<HudElement> {
            let Some(this) = __this_health.get().upgrade() else { return hud_element; };
            if !FancyStatusBars::should_process(&this) {
                return hud_element;
            }
            if FancyStatusBars::is_health_fancy_bar_enabled(&this) {
                // OPTIMIZED BUT NOT FULLY VANILLA (perf round 1, P02-mods-runtime): the shared empty element. Was:
                //     let empty: Arc<HudElement> = Arc::new(|_: &mut hud_render_callback::HudRenderContext<'_>| {});
                return compat::empty_hud_element();
            } else if FancyStatusBars::is_experience_fancy_bar_enabled(&this) {
                let moved: Arc<HudElement> = Arc::new(move |context: &mut hud_render_callback::HudRenderContext<'_>| {
                    let saved = mcrust::net::minecraft::client::gui::gui_graphics_extractor::push_pose();
                    mcrust::net::minecraft::client::gui::gui_graphics_extractor::pose_translate(0.0, 6.0);
                    hud_element(context);
                    mcrust::net::minecraft::client::gui::gui_graphics_extractor::pop_pose(saved);
                });
                return moved;
            }
            hud_element
        }));
        // java: HudElementRegistry.replaceElement(VanillaHudElements.EXPERIENCE_LEVEL, hudElement -> {
        //   if (!shouldProcess() || !isExperienceFancyBarEnabled()) return hudElement; return (_, _) -> {}; });
        // java: HudElementRegistry.replaceElement(VanillaHudElements.INFO_BAR, hudElement -> {
        //   if (!shouldProcess() || !isExperienceFancyBarEnabled()) return hudElement; return (_, _) -> {}; });
        for element in [vanilla_hud_elements::EXPERIENCE_LEVEL, vanilla_hud_elements::INFO_BAR] {
            let __this_experience = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
            hud_element_registry::replace_element(element, Arc::new(move |hud_element: Arc<HudElement>| -> Arc<HudElement> {
                let Some(this) = __this_experience.get().upgrade() else { return hud_element; };
                if !FancyStatusBars::should_process(&this) || !FancyStatusBars::is_experience_fancy_bar_enabled(&this) {
                    return hud_element;
                }
                // OPTIMIZED BUT NOT FULLY VANILLA (perf round 1, P02-mods-runtime): the shared empty element. Was:
                //     let empty: Arc<HudElement> = Arc::new(|_: &mut hud_render_callback::HudRenderContext<'_>| {});
                //     empty
                compat::empty_hud_element()
            }));
        }
        // java: HudElementRegistry.replaceElement(VanillaHudElements.ARMOR_BAR, hideIfFancyStatusBarsEnabled);
        hud_element_registry::replace_element(vanilla_hud_elements::ARMOR_BAR, hide_if_fancy_status_bars_enabled.clone());
        // java: HudElementRegistry.replaceElement(VanillaHudElements.MOUNT_HEALTH, hideIfFancyStatusBarsEnabled);
        hud_element_registry::replace_element(vanilla_hud_elements::MOUNT_HEALTH, hide_if_fancy_status_bars_enabled.clone());
        // java: HudElementRegistry.replaceElement(VanillaHudElements.FOOD_BAR, hideIfFancyStatusBarsEnabled);
        hud_element_registry::replace_element(vanilla_hud_elements::FOOD_BAR, hide_if_fancy_status_bars_enabled.clone());
        // java: HudElementRegistry.replaceElement(VanillaHudElements.AIR_BAR, hideIfFancyStatusBarsEnabled);
        hud_element_registry::replace_element(vanilla_hud_elements::AIR_BAR, hide_if_fancy_status_bars_enabled.clone());
        // java: HudElementRegistry.attachElementAfter(VanillaHudElements.HOTBAR, HUD_LAYER, (context, _) -> {
        //   if (shouldProcess()) extractRenderState(context, Minecraft.getInstance()); });
        let __this_hud = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        hud_element_registry::attach_element_after(vanilla_hud_elements::HOTBAR, HUD_LAYER.get(), Arc::new(move |ctx: &mut hud_render_callback::HudRenderContext<'_>| {
            // PERF (port-only, same result): `shouldProcess()` first - off Skyblock (or with the bars off) the
            // element draws nothing, so the graphics session it would open is not set up at all.
            let Some(this) = __this_hud.get().upgrade() else { return; };
            if !FancyStatusBars::should_process(&this) {
                return;
            }
            // SAFETY: `ctx` outlives every call made through `graphics` (this element).
            let graphics = unsafe { ScriptGraphics::new(ctx.gl, &*ctx.minecraft, ctx.gui_width, ctx.gui_height) };
            let _session = graphics.session();
            FancyStatusBars::extract_render_state(&this, &ctx.graphics, &mut *ctx.minecraft);
        }));
        for r#type in StatusBarType::values() {
            let __h0 = (r#type).new_status_bar();
            __this.lock().borrow_mut().status_bars.insert(r#type, __h0);
        }
        { let __v = SkyblockerConfigManager::get().ui_and_visuals.bars.has_seen_vitality_at_least_once; compat::write(&__this.lock().borrow().status_bars.get(&StatusBarType::Vitality).cloned().unwrap(), |__t| __t.status_bar_base_mut().visible = __v); }
        FancyStatusBars::reset_bar_positions(__this);
        let mut counts: Vec<i32> = vec![0; (3) as usize];
        let _ = &*FILE; // (FILE is first resolved here, on the render thread - see its declaration)
        let bar_positions = &SkyblockerConfigManager::get().ui_and_visuals.bars.bar_positions;
        FancyStatusBars::update_legacy_position(&compat::read(__this, |__t| __t.status_bars.get(&StatusBarType::Health).cloned().unwrap()), &mut counts, bar_positions.health_bar_position);
        FancyStatusBars::update_legacy_position(&compat::read(__this, |__t| __t.status_bars.get(&StatusBarType::Intelligence).cloned().unwrap()), &mut counts, bar_positions.mana_bar_position);
        FancyStatusBars::update_legacy_position(&compat::read(__this, |__t| __t.status_bars.get(&StatusBarType::Defense).cloned().unwrap()), &mut counts, bar_positions.defence_bar_position);
        FancyStatusBars::update_legacy_position(&compat::read(__this, |__t| __t.status_bars.get(&StatusBarType::Experience).cloned().unwrap()), &mut counts, bar_positions.experience_bar_position);
        let __this_2 = compat::RenderThread::new(__this.clone());
        (CompletableFuture::supply_async(move || Ok((|| FancyStatusBars::load_bar_config())()), compat::executor)).then_accept_async(move |object: &Option<serde_json::Map<String, serde_json::Value>>| {
            // java: an exception in this stage completes it exceptionally, which `.exceptionally(..)` below logs
            let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                if let Some(object) = object {
                    for (s, value) in object.iter() {
                        let r#type: StatusBarType = StatusBarType::from(s);
                        let status_bar = compat::read(__this_2.get(), |__t| __t.status_bars.get(&r#type).cloned());
                        if let Some(status_bar) = status_bar {
                            // java: try { statusBars.get(type).loadFromJson(object.get(s).getAsJsonObject()); } catch (Exception e) {..}
                            let __load = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                                let __f = compat::dispatch(&status_bar, |__t| __t.load_from_json());
                                __f(&status_bar, &compat::json::get_as_json_object(value));
                            }));
                            if let Err(e) = __load {
                                logger().error(&format!("[Skyblocker] Failed to load {} status bar {}", s, compat::panic_message(&e)));
                            }
                        } else {
                            logger().warn(&format!("[Skyblocker] Unknown status bar: {}", s));
                        }
                    }
                }
                FancyStatusBars::place_bars_in_positioner(__this_2.get());
                { let __v = true; compat::write(__this_2.get(), |__t| __t.config_loaded = __v); }
            }));
            if __result.is_err() {
                logger().error("[Skyblocker] Failed reading status bars config");
            }
        }, std::sync::Arc::new(|__t: Box<dyn FnOnce() + Send>| crate::compat::execute(__t))).exceptionally(|throwable| {
            logger().error(&format!("[Skyblocker] Failed reading status bars config"));
            return Default::default(); // java: return null
        });
        // java: ClientLifecycleEvents.CLIENT_STOPPING.register(_ -> saveBarConfig()); - HEAD of `Minecraft#stop`
        let __this_stop = compat::RenderThread::new(__this.clone());
        report(hook::inject("net.minecraft.client.Minecraft#stop", At::Head, move |_: &mut CallbackInfo<'_>| {
            FancyStatusBars::save_bar_config(__this_stop.get());
        }));
        // java: ClientCommandRegistrationCallback.EVENT.register((dispatcher, _) -> dispatcher.register(ClientCommands.literal(SkyblockerMod.NAMESPACE)
        //   .then(ClientCommands.literal("bars").executes(Scheduler.queueOpenScreenCommand(() -> new StatusBarsConfigScreen(this))))));
        let __this_bars = compat::RenderThread::new(__this.clone());
        let __this_bars = Arc::new(__this_bars);
        client_command_registration_callback::EVENT.register(Arc::new(
            move |dispatcher: &mut client_command_registration_callback::ClientCommandDispatcher| {
                let __this_screen = __this_bars.clone();
                let mut bars = client_command_registration_callback::literal("bars");
                bars.executes(crate::de::hysky::skyblocker::utils::scheduler::scheduler::SchedulerBase::queue_open_screen_command(Arc::new(move || {
                    Box::new(compat::ScreenAdapter::new(crate::de::hysky::skyblocker::skyblock::fancybars::status_bars_config_screen::StatusBarsConfigScreen::new(__this_screen.get().clone()))) as Box<dyn mcrust::mcrust::modding::screens::ModScreen>
                })));
                let mut root = client_command_registration_callback::literal(crate::entry::skyblocker_mod::NAMESPACE);
                root.then(&bars);
                dispatcher.dispatcher().register(&root);
            },
        ));
        let __this_3 = compat::RenderThread::new(__this.clone());
        skyblock_events::LOCATION_CHANGE.register(Arc::new(move |_| { { let __v = true; compat::write(__this_3.get(), |__t| __t.update_positions_next_frame = __v); } }));
    }

    /// java: @VisibleForTesting public void placeBarsInPositioner()
    pub fn place_bars_in_positioner(__this: &compat::Shared<Self>) {
        compat::write(__this, |__t| __t.bar_positioner.clear());
        for bar_anchor in BarAnchor::all_anchors() {
            // java: statusBars.values().stream().filter(bar -> bar.anchor == barAnchor)
            //   .sorted(Comparator.<StatusBar>comparingInt(bar -> bar.gridY).thenComparingInt(bar -> bar.gridX)).toList()
            let all_bars: Vec<compat::Shared<dyn StatusBar>> = compat::read(__this, |__t| __t.status_bars.values().cloned().collect());
            let mut bar_list: Vec<compat::Shared<dyn StatusBar>> = all_bars.into_iter().filter(|bar| compat::read(bar, |__t| __t.status_bar_base().anchor) == Some(bar_anchor)).collect();
            bar_list.sort_by_key(|bar| compat::read(bar, |__t| (__t.status_bar_base().grid_y, __t.status_bar_base().grid_x)));
            if bar_list.is_empty() {
                continue;
            }
            let mut y: i32 = -1;
            let mut row_num: i32 = -1;
            for status_bar in bar_list.iter() {
                if compat::read(status_bar, |__t| __t.status_bar_base().grid_y) > y {
                    compat::write(__this, |__t| __t.bar_positioner.add_row(bar_anchor));
                    row_num += 1;
                    y = compat::read(status_bar, |__t| __t.status_bar_base().grid_y);
                }
                let __h0 = status_bar.clone();
                compat::write(__this, |__t| __t.bar_positioner.add_bar(bar_anchor, row_num, __h0));
            }
        }
    }

    /// java: public void resetBarPositions()
    pub fn reset_bar_positions(__this: &compat::Shared<Self>) {
        for (r#type, bar) in compat::read(__this, |__t| __t.status_bars.clone()).iter() {
            { let __v = Some((r#type).get_default_anchor()); compat::write(bar, |__t| __t.status_bar_base_mut().anchor = __v); }
            { let __v = (r#type).get_default_grid_y(); compat::write(bar, |__t| __t.status_bar_base_mut().grid_y = __v); }
            { let __v = (r#type).get_default_anchor().get_size_rule().min_size(); compat::write(bar, |__t| __t.status_bar_base_mut().size = __v); }
        }
        FancyStatusBars::place_bars_in_positioner(__this);
    }

    /// java: public void saveBarConfig()
    pub fn save_bar_config(__this: &compat::Shared<Self>) {
        let mut output: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
        for (s, status_bar) in compat::read(__this, |__t| __t.status_bars.clone()).iter() {
            output.insert(s.get_serialized_name(), compat::json::to_value(({ let __f = compat::dispatch(status_bar, |__t| __t.to_json()); __f(status_bar) })));
        }
        // java: try (BufferedWriter writer = Files.newBufferedWriter(FILE)) { SkyblockerMod.GSON.toJson(output, writer); LOGGER.info(..); } catch (IOException e) {..}
        // (SkyblockerMod.GSON is pretty-printing)
        let __json = serde_json::to_string_pretty(&serde_json::Value::Object(output)).expect("a JSON tree serialises");
        match std::fs::write(&*FILE, __json) {
            Ok(()) => logger().info("[Skyblocker] Saved status bars config"),
            Err(e) => logger().error(&format!("[Skyblocker] Failed to save status bars config {e}")),
        }
    }

    /// java: public void updatePositions(boolean ignoreVisibility)
    pub fn update_positions(__this: &compat::Shared<Self>, mut ignore_visibility: bool) {
        if !(compat::read(__this, |__t| __t.config_loaded)) {
            return;
        }
        let mut width: i32 = compat::minecraft().get_window().get_gui_scaled_width();
        let mut height: i32 = compat::minecraft().get_window().get_gui_scaled_height();
        let mut offset: i32 = 0;
        for status_bar in compat::read(__this, |__t| __t.status_bars.values().cloned().collect::<Vec<_>>()) {
            if !(compat::read(&status_bar, |__t| __t.status_bar_base().enabled)) {
                ({ let __f = compat::dispatch(&status_bar, |__t| __t.set_x()); __f(&status_bar, 5) });
                ({ let __f = compat::dispatch(&status_bar, |__t| __t.set_y()); __f(&status_bar, 50i32.wrapping_add(offset)) });
                ({ let __f = compat::dispatch(&status_bar, |__t| __t.set_width()); __f(&status_bar, 30) });
                offset = offset.wrapping_add({ let __f = compat::dispatch(&status_bar, |__t| __t.get_height()); __f(&status_bar) });
            } else if compat::read(&status_bar, |__t| __t.status_bar_base().anchor.clone()).is_none() {
                { let __v = (compat::read(&status_bar, |__t| __t.status_bar_base().width)).clamp((30.0f32 / (width as f32)), (1 as f32)); compat::write(&status_bar, |__t| __t.status_bar_base_mut().width = __v); }
                { let __v = (compat::read(&status_bar, |__t| __t.status_bar_base().x)).clamp((0 as f32), ((1 as f32) - compat::read(&status_bar, |__t| __t.status_bar_base().width))); compat::write(&status_bar, |__t| __t.status_bar_base_mut().x = __v); }
                { let __v = (compat::read(&status_bar, |__t| __t.status_bar_base().y)).clamp((0 as f32), ((1 as f32) - (((({ let __f = compat::dispatch(&status_bar, |__t| __t.get_height()); __f(&status_bar) })) as f32) / (height as f32)))); compat::write(&status_bar, |__t| __t.status_bar_base_mut().y = __v); }
                ({ let __f = compat::dispatch(&status_bar, |__t| __t.set_x()); __f(&status_bar, (((compat::read(&status_bar, |__t| __t.status_bar_base().x) * (width as f32))) as i32)) });
                ({ let __f = compat::dispatch(&status_bar, |__t| __t.set_y()); __f(&status_bar, (((compat::read(&status_bar, |__t| __t.status_bar_base().y) * (height as f32))) as i32)) });
                ({ let __f = compat::dispatch(&status_bar, |__t| __t.set_width()); __f(&status_bar, (((compat::read(&status_bar, |__t| __t.status_bar_base().width) * (width as f32))) as i32)) });
            }
        }
        for bar_anchor in BarAnchor::all_anchors() {
            let mut anchor_position: ScreenPosition = bar_anchor.get_anchor_position(width, height);
            let mut size_rule: SizeRule = bar_anchor.get_size_rule();
            let mut target_size: i32 = size_rule.target_size();
            let mut visible_health_move: bool = (bar_anchor == BarAnchor::HotbarTop && !(FancyStatusBars::is_health_fancy_bar_enabled(__this)));
            if visible_health_move {
                target_size = target_size.wrapping_div(2);
            }
            if size_rule.is_target_size() {
                for row in 0..compat::read(__this, |__t| __t.bar_positioner.get_row_count(bar_anchor)) {
                    let mut bar_row: Vec<compat::Shared<dyn StatusBar>> = compat::read(__this, |__t| __t.bar_positioner.get_row(bar_anchor, row));
                    if bar_row.is_empty() {
                        continue;
                    }
                    let mut total_size: i32 = 0;
                    for status_bar in bar_row.iter() {
                        // java: totalSize += (statusBar.size = Math.clamp(statusBar.size, sizeRule.minSize(), sizeRule.maxSize()));
                        let size: i32 = compat::read(status_bar, |__t| __t.status_bar_base().size).clamp(size_rule.min_size(), size_rule.max_size());
                        compat::write(status_bar, |__t| __t.status_bar_base_mut().size = size);
                        total_size += size;
                    }
                    'while_loop: while total_size != target_size {
                        if total_size > target_size {
                            for status_bar in bar_row.iter() {
                                if compat::read(status_bar, |__t| __t.status_bar_base().size) > size_rule.min_size() {
                                    { let __v = compat::read(status_bar, |__t| __t.status_bar_base().size) - 1; compat::write(status_bar, |__t| __t.status_bar_base_mut().size = __v); }
                                    total_size -= 1;
                                    if total_size == target_size {
                                        break 'while_loop;
                                    }
                                }
                            }
                        } else {
                            for status_bar in bar_row.iter() {
                                if compat::read(status_bar, |__t| __t.status_bar_base().size) < size_rule.max_size() {
                                    { let __v = compat::read(status_bar, |__t| __t.status_bar_base().size) + 1; compat::write(status_bar, |__t| __t.status_bar_base_mut().size = __v); }
                                    total_size += 1;
                                    if total_size == target_size {
                                        break 'while_loop;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            let mut row: i32 = 0;
            for i in 0..compat::read(__this, |__t| __t.bar_positioner.get_row_count(bar_anchor)) {
                let mut bar_row: Vec<compat::Shared<dyn StatusBar>> = compat::read(__this, |__t| __t.bar_positioner.get_row(bar_anchor, i)).into_iter().collect::<Vec<_>>();
                bar_row.retain(|status_bar| !((!(compat::read(status_bar, |__t| __t.status_bar_base().visible)) && !ignore_visibility)));
                if bar_row.is_empty() {
                    continue;
                }
                let mut width_per_size: f32;
                if size_rule.is_target_size() {
                    let mut size: i32 = 0;
                    for bar in bar_row.iter() {
                        size = size.wrapping_add(compat::read(bar, |__t| __t.status_bar_base().size));
                    }
                    width_per_size = (((size_rule.total_width()) as f32) / (size as f32));
                } else {
                    width_per_size = ((size_rule.width_per_size()) as f32) as f32;
                }
                if visible_health_move {
                    width_per_size = (width_per_size / (2 as f32));
                }
                let mut curr_size: i32 = 0;
                let mut row_size: i32 = (bar_row.len() as i32);
                for j in 0..row_size {
                    let mut offset_x: i32 = 0;
                    let mut less_width: i32 = 0;
                    if !size_rule.is_target_size() {
                        offset_x = 1;
                        less_width = 2;
                    } else if row_size > 1 {
                        if j == 0 {
                            less_width = 1;
                        } else if j == row_size.wrapping_sub(1) {
                            less_width = 1;
                            offset_x = 1;
                        } else {
                            less_width = 2;
                            offset_x = 1;
                        }
                    }
                    let mut status_bar: compat::Shared<dyn StatusBar> = bar_row[(j) as usize].clone();
                    { let __v = (compat::read(&status_bar, |__t| __t.status_bar_base().size)).clamp(size_rule.min_size(), size_rule.max_size()); compat::write(&status_bar, |__t| __t.status_bar_base_mut().size = __v); }
                    let mut x: f32 = (if bar_anchor.is_right() { (((anchor_position.x() as f32) + (if visible_health_move { ((size_rule.total_width() as f32) / 2.0f32) } else { (0) as f32 })) + ((curr_size as f32) * width_per_size)) } else { (((anchor_position.x() as f32) - ((curr_size as f32) * width_per_size)) - ((compat::read(&status_bar, |__t| __t.status_bar_base().size) as f32) * width_per_size)) });
                    ({ let __f = compat::dispatch(&status_bar, |__t| __t.set_x()); __f(&status_bar, (mth::ceil_f32(x)).wrapping_add(offset_x)) });
                    let mut y: i32 = (if bar_anchor.is_up() { anchor_position.y().wrapping_sub((row.wrapping_add(1)).wrapping_mul(({ let __f = compat::dispatch(&status_bar, |__t| __t.get_height()); __f(&status_bar) }).wrapping_add(1))) } else { anchor_position.y().wrapping_add(row.wrapping_mul(({ let __f = compat::dispatch(&status_bar, |__t| __t.get_height()); __f(&status_bar) }).wrapping_add(1))) });
                    ({ let __f = compat::dispatch(&status_bar, |__t| __t.set_y()); __f(&status_bar, y) });
                    ({ let __f = compat::dispatch(&status_bar, |__t| __t.set_width()); __f(&status_bar, (mth::floor_f32(((compat::read(&status_bar, |__t| __t.status_bar_base().size) as f32) * width_per_size))).wrapping_sub(less_width)) });
                    curr_size = curr_size.wrapping_add(compat::read(&status_bar, |__t| __t.status_bar_base().size));
                }
                if curr_size > 0 {
                    row += 1;
                }
            }
        }
    }

    /// java: public boolean shouldProcess()
    pub fn should_process(__this: &compat::Shared<Self>) -> bool {
        return (Utils::is_on_skyblock() && FancyStatusBars::is_enabled(__this));
    }

    /// java: public boolean isEnabled()
    pub fn is_enabled(__this: &compat::Shared<Self>) -> bool {
        return (SkyblockerConfigManager::get().ui_and_visuals.bars.enable_bars && !(Utils::is_in_the_rift())) || SkyblockerConfigManager::get().ui_and_visuals.bars.enable_bars_rift;
    }

    /// java: public boolean extractRenderState(GuiGraphicsExtractor graphics, Minecraft client)
    pub fn extract_render_state(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, client: &mut mcrust::net::minecraft::client::minecraft::Minecraft) -> bool {
        if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() {
            return false;
        }
        let mut bar_collection: Vec<compat::Shared<dyn StatusBar>> = compat::read(__this, |__t| __t.status_bars.values().cloned().collect::<Vec<_>>());
        // PERF (port-only, same result): `!enabled || !visible` read under one borrow, and the method to call
        // fetched under that same borrow - one lock per bar per pass instead of three.
        for status_bar in bar_collection.iter() {
            let Some(__f) = compat::read(status_bar, |__t| (__t.status_bar_base().enabled && __t.status_bar_base().visible).then(|| __t.extract_bar())) else {
                continue;
            };
            __f(status_bar, graphics);
        }
        for status_bar in bar_collection.iter() {
            let Some(__f) = compat::read(status_bar, |__t| (__t.status_bar_base().enabled && __t.status_bar_base().visible).then(|| __t.extract_text())) else {
                continue;
            };
            __f(status_bar, graphics);
        }
        let mut defense_bar: compat::Shared<dyn StatusBar> = __this.lock().borrow().status_bars.get(&StatusBarType::Defense).cloned().unwrap();
        let mut vitality_bar: compat::Shared<dyn StatusBar> = __this.lock().borrow().status_bars.get(&StatusBarType::Vitality).cloned().unwrap();
        if Utils::is_in_the_rift() {
            let mut div: i32 = (if SkyblockerConfigManager::get().ui_and_visuals.bars.rift_health_hp { 1 } else { 2 });
            if VANILLA_FANCY_BAR_READS.on() {
                ({ let __r = &__this.lock().borrow().status_bars.get(&StatusBarType::Health).cloned().unwrap(); let __f = compat::dispatch(__r, |__t| __t.update_values()); __f(__r, ((compat::java_round_f32(mcrust::mcrust::modding::game::with_player(|player| player.base.base.base.get_health()).unwrap()) as f32) / mcrust::mcrust::modding::game::with_player(|player| player.base.base.base.get_max_health()).unwrap()), ((0) as f32) as f32, (compat::java_round_f32(mcrust::mcrust::modding::game::with_player(|player| player.base.base.base.get_health()).unwrap())).wrapping_div(div), Some((compat::java_round_f32(mcrust::mcrust::modding::game::with_player(|player| player.base.base.base.get_max_health()).unwrap())).wrapping_div(div)), None) });
            } else {
                // OPTIMIZED BUT NOT FULLY VANILLA (perf round 1, P02-mods-runtime): the argument list's four player reads
                // in ONE `with_player`, in the order the arguments made them (getHealth, getMaxHealth, getHealth,
                // getMaxHealth); between them and their old place only the bar's lookup ran, which calls nothing hooked.
                let (health_0, max_health_1, health_2, max_health_3): (f32, f32, f32, f32) = mcrust::mcrust::modding::game::with_player(|player| {
                    let health_0 = player.base.base.base.get_health();
                    let max_health_1 = player.base.base.base.get_max_health();
                    let health_2 = player.base.base.base.get_health();
                    let max_health_3 = player.base.base.base.get_max_health();
                    (health_0, max_health_1, health_2, max_health_3)
                }).unwrap();
                ({ let __r = &__this.lock().borrow().status_bars.get(&StatusBarType::Health).cloned().unwrap(); let __f = compat::dispatch(__r, |__t| __t.update_values()); __f(__r, ((compat::java_round_f32(health_0) as f32) / max_health_1), ((0) as f32) as f32, (compat::java_round_f32(health_2)).wrapping_div(div), Some((compat::java_round_f32(max_health_3)).wrapping_div(div)), None) });
            }
            { let __v = false; compat::write(&defense_bar, |__t| __t.status_bar_base_mut().visible = __v); }
            { let __v = false; compat::write(&vitality_bar, |__t| __t.status_bar_base_mut().visible = __v); }
        } else {
            let mut health: Resource = StatusBarTracker::get_health();
            ({ let __r = &__this.lock().borrow().status_bars.get(&StatusBarType::Health).cloned().unwrap(); let __f = compat::dispatch(__r, |__t| __t.update_with_resource()); __f(__r, &health) });
            let mut defense: i32 = StatusBarTracker::get_defense();
            { let __v = true; compat::write(&defense_bar, |__t| __t.status_bar_base_mut().visible = __v); }
            ({ let __f = compat::dispatch(&defense_bar, |__t| __t.update_values()); __f(&defense_bar, ((defense as f32) / ((defense as f32) + 100.0f32)), ((0) as f32) as f32, defense, None, None) });
            let mut vitality: compat::Shared<EstimatedResource> = StatusBarTracker::get_vitality();
            { let __v = true; compat::write(&vitality_bar, |__t| __t.status_bar_base_mut().visible = __v); }
            ({ let __f = compat::dispatch(&vitality_bar, |__t| __t.update_with_resource()); __f(&vitality_bar, &EstimatedResource::resource(&vitality)) });
        }
        let mut intelligence: compat::Shared<EstimatedResource> = StatusBarTracker::get_mana();
        // (the config read once for the frame: Java reads the same field)
        let __bars_config = &SkyblockerConfigManager::get().ui_and_visuals.bars;
        if __bars_config.intelligence_display == IntelligenceDisplay::Accurate {
            let mut total_intelligence: f32 = (((EstimatedResource::max(&intelligence)) as f32) + (EstimatedResource::overflow(&intelligence) as f32));
            ({ let __r = &__this.lock().borrow().status_bars.get(&StatusBarType::Intelligence).cloned().unwrap(); let __f = compat::dispatch(__r, |__t| __t.update_values()); __f(__r, (((EstimatedResource::value(&intelligence) as f32) / total_intelligence) + ((EstimatedResource::overflow(&intelligence) as f32) / total_intelligence)), ((EstimatedResource::overflow(&intelligence) as f32) / total_intelligence), EstimatedResource::value(&intelligence), Some(EstimatedResource::max(&intelligence)), Some(EstimatedResource::overflow(&intelligence))) });
        } else {
            ({ let __r = &__this.lock().borrow().status_bars.get(&StatusBarType::Intelligence).cloned().unwrap(); let __f = compat::dispatch(__r, |__t| __t.update_with_resource()); __f(__r, &EstimatedResource::resource(&intelligence)) });
        }
        let mut speed: Resource = StatusBarTracker::get_speed();
        ({ let __r = &__this.lock().borrow().status_bars.get(&StatusBarType::Speed).cloned().unwrap(); let __f = compat::dispatch(__r, |__t| __t.update_with_resource()); __f(__r, &speed) });
        // java: statusBars.get(StatusBarType.EXPERIENCE).updateValues(player.experienceProgress, 0, player.experienceLevel, null, null);
        // (Player.experienceProgress / experienceLevel are PlayerBase fields: LocalPlayer -> AbstractClientPlayer -> Player)
        // OPTIMIZED BUT NOT FULLY VANILLA (perf round 1, P02-mods-runtime): `player.isUnderWater()` read in this same
        // `with_player` - between here and its old place below only the experience bar's update, the air tracker and the
        // air bar's update run, none of which calls anything hooked or touches the player. Was:
        //     let (experience_progress, experience_level) = mcrust::mcrust::modding::game::with_player(|player| (player.base.base.experience_progress, player.base.base.experience_level)).unwrap();
        let (experience_progress, experience_level, __under_water_read): (f32, i32, Option<bool>) = if VANILLA_FANCY_BAR_READS.on() {
            let (experience_progress, experience_level) = mcrust::mcrust::modding::game::with_player(|player| (player.base.base.experience_progress, player.base.base.experience_level)).unwrap();
            (experience_progress, experience_level, None)
        } else {
            mcrust::mcrust::modding::game::with_player(|player| (player.base.base.experience_progress, player.base.base.experience_level, Some(player.is_under_water()))).unwrap()
        };
        ({ let __r = &compat::read(__this, |__t| __t.status_bars.get(&StatusBarType::Experience).cloned().unwrap()); let __f = compat::dispatch(__r, |__t| __t.update_values()); __f(__r, experience_progress, ((0) as f32) as f32, experience_level, None, None) });
        let mut air: Resource = StatusBarTracker::get_air();
        let mut air_bar: compat::Shared<dyn StatusBar> = __this.lock().borrow().status_bars.get(&StatusBarType::Air).cloned().unwrap();
        ({ let __f = compat::dispatch(&air_bar, |__t| __t.update_with_resource()); __f(&air_bar, &air) });
        // (`player.isUnderWater()` evaluated once: Java's two calls read the same field within the statement pair)
        // Was: let __under_water: bool = mcrust::mcrust::modding::game::with_player(|player| player.is_under_water()).unwrap();
        let __under_water: bool = match __under_water_read {
            Some(under_water) => under_water,
            None => mcrust::mcrust::modding::game::with_player(|player| player.is_under_water()).unwrap(),
        };
        if __under_water != compat::read(&air_bar, |__t| __t.status_bar_base().visible) {
            { let __v = __under_water; compat::write(&air_bar, |__t| __t.status_bar_base_mut().visible = __v); }
            { let __v = true; compat::write(__this, |__t| __t.update_positions_next_frame = __v); }
        }
        if compat::read(__this, |__t| __t.update_positions_next_frame) {
            FancyStatusBars::update_positions(__this, false);
            { let __v = false; compat::write(__this, |__t| __t.update_positions_next_frame = __v); }
        }
        return true;
    }

}

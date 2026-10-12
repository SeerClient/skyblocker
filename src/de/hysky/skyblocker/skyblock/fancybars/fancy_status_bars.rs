//! Ported from de.hysky.skyblocker.skyblock.fancybars.FancyStatusBars by fabric2seer.

#[allow(unused_imports)]
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
pub static FILE: compat::MainThread<std::path::PathBuf> = compat::MainThread::new(|| skyblocker_mod::CONFIG_DIR.get_mut().join("status_bars.json"));
/// java: public static final FancyStatusBars INSTANCE = new FancyStatusBars();
pub static INSTANCE: compat::MainThread<compat::Shared<FancyStatusBars>> = compat::MainThread::new(|| FancyStatusBars::new());

/// java: public class FancyStatusBars
#[derive(Clone)]
pub struct FancyStatusBars {
    /// java: public BarPositioner barPositioner = new BarPositioner();
    pub bar_positioner: BarPositioner,
    /// java: public Map<StatusBarType, StatusBar> statusBars = new EnumMap<>(StatusBarType.class);
    pub status_bars: std::collections::HashMap<StatusBarType, compat::Shared<dyn StatusBar>>,
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
            status_bars: std::any::TypeId::of::<StatusBarType>(),
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
    pub fn update_legacy_position(bar: &compat::Shared<dyn StatusBar>, counts: &Vec<i32>, mut position: LegacyBarPosition) {
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
        {
            // TODO(fabric2seer): Files.newBufferedReader
            //   java: BufferedReader reader = Files.newBufferedReader(..);
            let mut reader: crate::unresolved::BufferedReader = todo!("fabric2seer: the initialiser of `reader` (TODO above)");
            // java: try {..} catch (NoSuchFileException _) catch (Exception e) - the body runs inline: a Rust body does not throw checked exceptions (a panic in it propagates)
            {
                // TODO(fabric2seer): `.fromJson(..)` on a Gson
                //   java: return SkyblockerMod.GSON.fromJson(reader, JsonObject.class);
            }
            // TODO(fabric2seer): the `catch (NoSuchFileException _)` block is not reached in Rust - the try body returns, breaks or continues, so it cannot run inside `catch_unwind`
            //   java: {
            //   java: 			LOGGER.warn("[Skyblocker] No status bar config file found, using defaults");
            //   java: 		}
            // TODO(fabric2seer): the `catch (Exception e)` block is not reached in Rust - the try body returns, breaks or continues, so it cannot run inside `catch_unwind`
            //   java: {
            //   java: 			LOGGER.error("[Skyblocker] Failed to load status bars config", e);
            //   java: 		}
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
        let __this_1 = compat::RenderThread::new(__this.clone());
        let mut hide_if_fancy_status_bars_enabled: Arc<dyn Fn(crate::unresolved::HudElement) -> crate::unresolved::HudElement> = Arc::new(move |hud_element| {
            if FancyStatusBars::should_process(__this_1.get()) {
                return Arc::new(move |_, _| {

                });
            }
            return hud_element;
        });
        // TODO(fabric2seer): HudElementRegistry.replaceElement: no SeerClient equivalent yet
        //   java: HudElementRegistry.replaceElement(VanillaHudElements.HEALTH_BAR, hudElement -> {
        //   java: 			if (!shouldProcess()) return hudElement;
        //   java: 			if (isHealthFancyBarEnabled()) {
        //   java: 				return (_, _) -> {};
        //   java: 			} else if (isExperienceFancyBarEnabled()) {
        //   java: 				return (context, tickCounter) -> {
        //   java: 					Matrix3x2fStack pose = context.pose();
        //   java: 					pose.pushMatrix();
        //   java: 					pose.translate(0, 6);
        //   java: 					hudElement.extractRenderState(context, tickCounter);
        //   java: 					pose.popMatrix();
        //   java: 				};
        //   java: 			}
        //   java: 			return hudElement;
        //   java: 		});
        // TODO(fabric2seer): HudElementRegistry.replaceElement: no SeerClient equivalent yet
        //   java: HudElementRegistry.replaceElement(VanillaHudElements.EXPERIENCE_LEVEL, hudElement -> {
        //   java: 			if (!shouldProcess() || !isExperienceFancyBarEnabled()) return hudElement;
        //   java: 			return (_, _) -> {};
        //   java: 		});
        // TODO(fabric2seer): HudElementRegistry.replaceElement: no SeerClient equivalent yet
        //   java: HudElementRegistry.replaceElement(VanillaHudElements.INFO_BAR, hudElement -> {
        //   java: 			if (!shouldProcess() || !isExperienceFancyBarEnabled()) return hudElement;
        //   java: 			return (_, _) -> {};
        //   java: 		});
        // TODO(fabric2seer): HudElementRegistry.replaceElement: no SeerClient equivalent yet
        //   java: HudElementRegistry.replaceElement(VanillaHudElements.ARMOR_BAR, hideIfFancyStatusBarsEnabled);
        // TODO(fabric2seer): HudElementRegistry.replaceElement: no SeerClient equivalent yet
        //   java: HudElementRegistry.replaceElement(VanillaHudElements.MOUNT_HEALTH, hideIfFancyStatusBarsEnabled);
        // TODO(fabric2seer): HudElementRegistry.replaceElement: no SeerClient equivalent yet
        //   java: HudElementRegistry.replaceElement(VanillaHudElements.FOOD_BAR, hideIfFancyStatusBarsEnabled);
        // TODO(fabric2seer): HudElementRegistry.replaceElement: no SeerClient equivalent yet
        //   java: HudElementRegistry.replaceElement(VanillaHudElements.AIR_BAR, hideIfFancyStatusBarsEnabled);
        hud_render_callback::EVENT.register(Arc::new(|ctx: &mut hud_render_callback::HudRenderContext<'_>| {
            // SAFETY: `ctx` outlives every call made through `graphics` (this callback).
            let graphics = unsafe { ScriptGraphics::new(ctx.gl, &*ctx.minecraft, ctx.gui_width, ctx.gui_height) };
            let _session = graphics.session();
            if FancyStatusBars::should_process(__this) {
                FancyStatusBars::extract_render_state(__this, &graphics, &mut (&mut *ctx.minecraft));
            }
        }));
        for r#type in StatusBarType::values() {
            let __h0 = (r#type).new_status_bar();
            __this.lock().borrow_mut().status_bars.insert(r#type, __h0);
        }
        { let __v = SkyblockerConfigManager::get().ui_and_visuals.bars.has_seen_vitality_at_least_once; compat::write(&__this.lock().borrow().status_bars.get(&StatusBarType::Vitality).cloned().unwrap(), |__t| __t.status_bar_base_mut().visible = __v); }
        FancyStatusBars::reset_bar_positions(__this);
        let mut counts: Vec<i32> = vec![0; (3) as usize];
        let bar_positions = &SkyblockerConfigManager::get().ui_and_visuals.bars.bar_positions;
        FancyStatusBars::update_legacy_position(&__this.lock().borrow().status_bars.get(&StatusBarType::Health).cloned().unwrap(), &counts, bar_positions.health_bar_position);
        FancyStatusBars::update_legacy_position(&__this.lock().borrow().status_bars.get(&StatusBarType::Intelligence).cloned().unwrap(), &counts, bar_positions.mana_bar_position);
        FancyStatusBars::update_legacy_position(&__this.lock().borrow().status_bars.get(&StatusBarType::Defense).cloned().unwrap(), &counts, bar_positions.defence_bar_position);
        FancyStatusBars::update_legacy_position(&__this.lock().borrow().status_bars.get(&StatusBarType::Experience).cloned().unwrap(), &counts, bar_positions.experience_bar_position);
        let __this_2 = compat::RenderThread::new(__this.clone());
        (CompletableFuture::supply_async(move || Ok((|| FancyStatusBars::load_bar_config())()), compat::executor)).then_accept_async(|object| {
            if true /* java: object != null: never null here */ {
                // TODO(fabric2seer): `.keySet(..)` on a value of unknown type
                //   java: for (String s : object.keySet()) {
                //   java: 					StatusBarType type = StatusBarType.from(s);
                //   java: 					if (statusBars.containsKey(type)) {
                //   java: 						try {
                //   java: 							statusBars.get(type).loadFromJson(object.get(s).getAsJsonObject());
                //   java: 						} catch (Exception e) {
                //   java: 							LOGGER.error("[Skyblocker] Failed to load {} status bar", s, e);
                //   java: 						}
                //   java: 					} else {
                //   java: 						LOGGER.warn("[Skyblocker] Unknown status bar: {}", s);
                //   java: 					}
                //   java: 				}
            }
            FancyStatusBars::place_bars_in_positioner(__this_2.get());
            { let __v = true; compat::write(__this_2.get(), |__t| __t.config_loaded = __v); }
        }, std::sync::Arc::new(|__t: Box<dyn FnOnce() + Send>| mcrust::mcrust::modding::game::execute(__t))).exceptionally(|throwable| {
            logger().error(&format!("[Skyblocker] Failed reading status bars config"));
            return Default::default(); // java: return null
        });
        // TODO(fabric2seer): no SeerClient event yet: hook HEAD of Minecraft#stop (`net.minecraft.client.Minecraft#stop`: exists)
        //   java: ClientLifecycleEvents.CLIENT_STOPPING.register(_ -> saveBarConfig());
        client_command_registration_callback::EVENT.register(Arc::new(
            |dispatcher: &mut client_command_registration_callback::ClientCommandDispatcher| {
                // TODO(fabric2seer): a command tree this tool cannot read (arguments, redirects, requires) - SeerClient commands receive the words after the name in `ctx.args`
                //   java: dispatcher.register(..);
            },
        ));
        let __this_3 = compat::RenderThread::new(__this.clone());
        skyblock_events::LOCATION_CHANGE.register(Arc::new(move |_| { { let __v = true; compat::write(__this_3.get(), |__t| __t.update_positions_next_frame = __v); } }));
    }

    /// java: @VisibleForTesting public void placeBarsInPositioner()
    pub fn place_bars_in_positioner(__this: &compat::Shared<Self>) {
        __this.lock().borrow_mut().bar_positioner.clear();
        for bar_anchor in BarAnchor::all_anchors() {
            let mut bar_list: Vec<compat::Shared<dyn StatusBar>> = compat::sorted_by(__this.lock().borrow().status_bars.values().cloned().filter(|bar| (compat::read(bar, |__t| __t.status_bar_base().anchor.clone()).unwrap() == bar_anchor)), |a, b| (|a, b| { let key = |bar| compat::read(bar, |__t| __t.status_bar_base().grid_y); key(a).cmp(&key(b)) })(a, b).then_with(|| (|a, b| { let key = |bar| compat::read(bar, |__t| __t.status_bar_base().grid_x); key(a).cmp(&key(b)) })(a, b))).collect::<Vec<_>>();
            if bar_list.is_empty() {
                continue;
            }
            let mut y: i32 = -1;
            let mut row_num: i32 = -1;
            for status_bar in bar_list.iter() {
                if compat::read(status_bar, |__t| __t.status_bar_base().grid_y) > y {
                    __this.lock().borrow().bar_positioner.add_row(bar_anchor);
                    row_num += 1;
                    y = compat::read(status_bar, |__t| __t.status_bar_base().grid_y);
                }
                let __h0 = status_bar.clone();
                __this.lock().borrow().bar_positioner.add_bar(bar_anchor, row_num, __h0);
            }
        }
    }

    /// java: public void resetBarPositions()
    pub fn reset_bar_positions(__this: &compat::Shared<Self>) {
        for (r#type, bar) in __this.lock().borrow().status_bars.iter() {
            { let __v = Some((r#type).get_default_anchor()); compat::write(bar, |__t| __t.status_bar_base_mut().anchor = __v); }
            { let __v = (r#type).get_default_grid_y(); compat::write(bar, |__t| __t.status_bar_base_mut().grid_y = __v); }
            { let __v = (r#type).get_default_anchor().get_size_rule().min_size(); compat::write(bar, |__t| __t.status_bar_base_mut().size = __v); }
        }
        FancyStatusBars::place_bars_in_positioner(__this);
    }

    /// java: public void saveBarConfig()
    pub fn save_bar_config(__this: &compat::Shared<Self>) {
        let mut output: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
        for (s, status_bar) in __this.lock().borrow().status_bars.iter() {
            output.insert(s.get_serialized_name(), compat::json::to_value(({ let __f = compat::dispatch(status_bar, |__t| __t.to_json()); __f(status_bar) })));
        }
        {
            // TODO(fabric2seer): Files.newBufferedWriter
            //   java: BufferedWriter writer = Files.newBufferedWriter(..);
            let mut writer: crate::unresolved::BufferedWriter = todo!("fabric2seer: the initialiser of `writer` (TODO above)");
            // java: try {..} catch (IOException e) - an exception here is a panic, caught by `catch_unwind`
            let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                // TODO(fabric2seer): `.toJson(..)` on a Gson
                //   java: SkyblockerMod.GSON.toJson(output, writer);
                logger().info("[Skyblocker] Saved status bars config");
            }));
            if let Err(e) = __result {
                logger().error(&format!("[Skyblocker] Failed to save status bars config"));
            }
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
        for status_bar in __this.lock().borrow().status_bars.values().cloned() {
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
                for row in 0..__this.lock().borrow().bar_positioner.get_row_count(bar_anchor) {
                    let mut bar_row: Vec<compat::Shared<dyn StatusBar>> = __this.lock().borrow().bar_positioner.get_row(bar_anchor, row);
                    if bar_row.is_empty() {
                        continue;
                    }
                    let mut total_size: i32 = 0;
                    for status_bar in bar_row.iter_mut() {
                        // TODO(fabric2seer): an assignment used as a value
                        //   java: totalSize += (statusBar.size = Math.clamp(statusBar.size, sizeRule.minSize(), sizeRule.maxSize()));
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
            for i in 0..__this.lock().borrow().bar_positioner.get_row_count(bar_anchor) {
                let mut bar_row: Vec<compat::Shared<dyn StatusBar>> = __this.lock().borrow().bar_positioner.get_row(bar_anchor, i).into_iter().collect::<Vec<_>>();
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
        return (SkyblockerConfigManager::get().ui_and_visuals.bars.enable_bars && !(Utils::is_in_the_rift()) || SkyblockerConfigManager::get().ui_and_visuals.bars.enable_bars_rift);
    }

    /// java: public boolean extractRenderState(GuiGraphicsExtractor graphics, Minecraft client)
    pub fn extract_render_state(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, client: &mut mcrust::net::minecraft::client::minecraft::Minecraft) -> bool {
        if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() {
            return false;
        }
        let mut bar_collection: Vec<compat::Shared<dyn StatusBar>> = __this.lock().borrow().status_bars.values().cloned().collect::<Vec<_>>();
        for status_bar in bar_collection.iter() {
            if !(compat::read(status_bar, |__t| __t.status_bar_base().enabled)) || !(compat::read(status_bar, |__t| __t.status_bar_base().visible)) {
                continue;
            }
            ({ let __f = compat::dispatch(status_bar, |__t| __t.extract_bar()); __f(status_bar, graphics) });
        }
        for status_bar in bar_collection.iter() {
            if !(compat::read(status_bar, |__t| __t.status_bar_base().enabled)) || !(compat::read(status_bar, |__t| __t.status_bar_base().visible)) {
                continue;
            }
            ({ let __f = compat::dispatch(status_bar, |__t| __t.extract_text()); __f(status_bar, graphics) });
        }
        let mut defense_bar: compat::Shared<dyn StatusBar> = __this.lock().borrow().status_bars.get(&StatusBarType::Defense).cloned().unwrap();
        let mut vitality_bar: compat::Shared<dyn StatusBar> = __this.lock().borrow().status_bars.get(&StatusBarType::Vitality).cloned().unwrap();
        if Utils::is_in_the_rift() {
            let mut div: i32 = (if SkyblockerConfigManager::get().ui_and_visuals.bars.rift_health_hp { 1 } else { 2 });
            ({ let __r = &__this.lock().borrow().status_bars.get(&StatusBarType::Health).cloned().unwrap(); let __f = compat::dispatch(__r, |__t| __t.update_values()); __f(__r, ((compat::java_round_f32(mcrust::mcrust::modding::game::with_player(|player| player.base.base.base.get_health()).unwrap()) as f32) / mcrust::mcrust::modding::game::with_player(|player| player.base.base.base.get_max_health()).unwrap()), ((0) as f32) as f32, (compat::java_round_f32(mcrust::mcrust::modding::game::with_player(|player| player.base.base.base.get_health()).unwrap())).wrapping_div(div), (compat::java_round_f32(mcrust::mcrust::modding::game::with_player(|player| player.base.base.base.get_max_health()).unwrap())).wrapping_div(div), None) });
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
        if SkyblockerConfigManager::get().ui_and_visuals.bars.intelligence_display == IntelligenceDisplay::Accurate {
            let mut total_intelligence: f32 = (((EstimatedResource::max(&intelligence)) as f32) + (EstimatedResource::overflow(&intelligence) as f32));
            ({ let __r = &__this.lock().borrow().status_bars.get(&StatusBarType::Intelligence).cloned().unwrap(); let __f = compat::dispatch(__r, |__t| __t.update_values()); __f(__r, (((EstimatedResource::value(&intelligence) as f32) / total_intelligence) + ((EstimatedResource::overflow(&intelligence) as f32) / total_intelligence)), ((EstimatedResource::overflow(&intelligence) as f32) / total_intelligence), EstimatedResource::value(&intelligence), EstimatedResource::max(&intelligence), EstimatedResource::overflow(&intelligence)) });
        } else {
            ({ let __r = &__this.lock().borrow().status_bars.get(&StatusBarType::Intelligence).cloned().unwrap(); let __f = compat::dispatch(__r, |__t| __t.update_with_resource()); __f(__r, &EstimatedResource::resource(&intelligence)) });
        }
        let mut speed: Resource = StatusBarTracker::get_speed();
        ({ let __r = &__this.lock().borrow().status_bars.get(&StatusBarType::Speed).cloned().unwrap(); let __f = compat::dispatch(__r, |__t| __t.update_with_resource()); __f(__r, &speed) });
        // TODO(fabric2seer): `LocalPlayer.experienceProgress` has no Rust field in the port (searched net.minecraft.client.player.LocalPlayer and its supertypes)
        // TODO(fabric2seer): `LocalPlayer.experienceLevel` has no Rust field in the port (searched net.minecraft.client.player.LocalPlayer and its supertypes)
        // rust: ({ let __r = &__this.lock().borrow().status_bars.get(&StatusBarType::Experience).cloned().unwrap(); let __f = compat::dispatch(__r, |__t| __t.update_values()); __f(__r, mcrust::mcrust::modding::game::with_player(|player| player.experience_progress).unwrap(), ((0) as f32) as f32, mcrust::mcrust::modding::game::with_player(|player| player.experience_level).unwrap(), None, None) });
        let mut air: Resource = StatusBarTracker::get_air();
        let mut air_bar: compat::Shared<dyn StatusBar> = __this.lock().borrow().status_bars.get(&StatusBarType::Air).cloned().unwrap();
        ({ let __f = compat::dispatch(&air_bar, |__t| __t.update_with_resource()); __f(&air_bar, &air) });
        if mcrust::mcrust::modding::game::with_player(|player| player.is_under_water()).unwrap() != compat::read(&air_bar, |__t| __t.status_bar_base().visible) {
            { let __v = mcrust::mcrust::modding::game::with_player(|player| player.is_under_water()).unwrap(); compat::write(&air_bar, |__t| __t.status_bar_base_mut().visible = __v); }
            { let __v = true; compat::write(__this, |__t| __t.update_positions_next_frame = __v); }
        }
        if compat::read(__this, |__t| __t.update_positions_next_frame) {
            FancyStatusBars::update_positions(__this, false);
            { let __v = false; compat::write(__this, |__t| __t.update_positions_next_frame = __v); }
        }
        return true;
    }

}

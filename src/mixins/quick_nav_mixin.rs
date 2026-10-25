//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.QuickNavMixin by fabric2seer.
//! @Mixin(net.minecraft.client.gui.screens.inventory.AbstractContainerScreen)

#[allow(unused_imports)]
use crate::prelude::*;

/// java: @Unique private @Nullable List<QuickNavButton> quickNavButtons;
/// One value per AbstractContainerScreen OBJECT (a Mixin field): keyed by the screen object's identity
/// (`Screen::mod_event_id`), written by `init` (TAIL) and dropped by `ScreenEvents.remove(screen)` - the field dies
/// with its object. A missing key is the field's `null`.
/// Each button with the index of the game's widget `instance.addWidget(quickNavButton)` made for it in the screen's
/// widget list (the game keeps the live widget - its box, `visible`, `isHovered` - and lends it to the button for each
/// of the button's own methods; see `compat::WidgetAdapter`).
pub static QUICK_NAV_BUTTONS: compat::MainThread<std::collections::HashMap<u64, Vec<(compat::Shared<dyn crate::de::hysky::skyblocker::skyblock::quicknav::quick_nav_button::QuickNavButton>, usize)>>> = compat::MainThread::new(|| std::collections::HashMap::new());

/// The button's box (`setX` / `setY` in its `updateCoordinates`) onto the game's widget.
fn sync_box(screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, quick_nav_button: &compat::Shared<dyn crate::de::hysky::skyblocker::skyblock::quicknav::quick_nav_button::QuickNavButton>, index: usize) {
    let (x, y, width, height) = compat::read(quick_nav_button, |__t| {
        let base = &__t.quick_nav_button_base().base;
        (base.get_x(), base.get_y(), base.width, base.height)
    });
    let widget = screen.widget_mut(index);
    if widget.get_x() != x {
        widget.set_x(x);
    }
    if widget.get_y() != y {
        widget.set_y(y);
    }
    widget.width = width;
    widget.height = height;
}

thread_local! {
    /// The container screen whose tabs the frame draws: the LIVE screen object's `Screen::mod_event_id` (the key
    /// `init` TAIL stored the buttons under) and the frame's `mouseX` / `mouseY`, recorded at `Gui#extractRenderState`
    /// HEAD, the one point of the frame that is handed the live `Screen`.
    ///
    /// **NOT the `AbstractContainerScreen` the draw hooks are handed**: every open site (`ScreenExtraBuild::into_extra`,
    /// the inventory key) MOVES the initialised `Screen` (with the tabs' widgets and the id) out of the container
    /// screen into the frame's screen slot and leaves a fresh husk `Screen` behind, so `this.screen.mod_event_id()` in
    /// `extractContents` named a different object - no tabs were ever found - and `this.screen.children()` was empty.
    static FRAME_SCREEN: std::cell::Cell<Option<(u64, i32, i32)>> = const { std::cell::Cell::new(None) };
    /// The frame's partial tick (Java's `a`), from `extractContents`.
    static FRAME_A: std::cell::Cell<f32> = const { std::cell::Cell::new(0.0) };
}

/// The button is drawn behind the main inventory background when it is not toggled or still fading in.
fn is_unselected(quick_nav_button: &compat::Shared<dyn crate::de::hysky::skyblocker::skyblock::quicknav::quick_nav_button::QuickNavButton>) -> bool {
    !({ let __f = compat::dispatch(quick_nav_button, |__t| __t.toggled()); __f(quick_nav_button) }) || ({ let __f = compat::dispatch(quick_nav_button, |__t| __t.get_alpha()); __f(quick_nav_button) }) < (255 as f32)
}

/// `AbstractWidget.extractRenderState:55-61`'s draw half (`if (this.visible) this.extractWidgetRenderState(..)`) for
/// a button whose game widget `Gui#extractRenderState` HEAD has already brought up to date (box, `isHovered`).
fn draw_quick_nav_button(quick_nav_button: &compat::Shared<dyn crate::de::hysky::skyblocker::skyblock::quicknav::quick_nav_button::QuickNavButton>, mouse_x: i32, mouse_y: i32, a: f32) {
    if !compat::read(quick_nav_button, |__t| __t.quick_nav_button_base().base.visible) {
        return;
    }
    let graphics = mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor::new();
    ({ let __f = compat::dispatch(quick_nav_button, |__t| __t.extract_widget_render_state()); __f(quick_nav_button, &graphics, mouse_x, mouse_y, a) });
}

/// The tabs of the frame's container screen, with the mouse.
fn frame_buttons() -> Option<(Vec<(compat::Shared<dyn crate::de::hysky::skyblocker::skyblock::quicknav::quick_nav_button::QuickNavButton>, usize)>, i32, i32, f32)> {
    let (id, mouse_x, mouse_y) = FRAME_SCREEN.with(|cell| cell.get())?;
    let buttons = QUICK_NAV_BUTTONS.get().get(&id).cloned()?;
    Some((buttons, mouse_x, mouse_y, FRAME_A.with(|cell| cell.get())))
}

/// @Override
///
/// Draws the unselected tabs in front of the background, but behind the main inventory, similar to creative inventory tabs.
/// At `Hud.extractDeferredSubtitles` (`Screen.extractBackground:387`), between the backdrop and the panel, as Java.
pub fn skyblocker_extract_unselected_quick_nav_buttons_backdrop() {
    if !compat::container_screen_open() {
        return;
    }
    let Some((quick_nav_buttons, mouse_x, mouse_y, a)) = frame_buttons() else { return; };
    for (quick_nav_button, _index) in quick_nav_buttons.iter() {
        if is_unselected(quick_nav_button) {
            ({ let __f = compat::dispatch(quick_nav_button, |__t| __t.set_render_in_front()); __f(quick_nav_button, false) });
            draw_quick_nav_button(quick_nav_button, mouse_x, mouse_y, a);
        }
    }
}

/// The screen-holding half of `AbstractWidget.extractRenderState:55-61` for every tab of the LIVE screen (the
/// button's box onto the game's widget, then `isHovered`), and the frame record the two draw passes read. Runs at
/// `Gui#extractRenderState` HEAD, which is handed the live `Screen` and the frame's mouse; the box is the one the
/// previous draw's `updateCoordinates` left, which is what Java's `isHovered` reads too.
pub fn skyblocker_update_quick_nav_buttons_live(screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, mouse_x: i32, mouse_y: i32) {
    let id = screen.mod_event_id();
    let Some(quick_nav_buttons) = QUICK_NAV_BUTTONS.get().get(&id).cloned() else {
        FRAME_SCREEN.with(|cell| cell.set(None));
        return;
    };
    FRAME_SCREEN.with(|cell| cell.set(Some((id, mouse_x, mouse_y))));
    for (quick_nav_button, index) in quick_nav_buttons.iter() {
        if *index >= screen.children().len() {
            continue;
        }
        sync_box(screen, quick_nav_button, *index);
        let widget = screen.widget_mut(*index);
        widget.set_hovered_from_extract(mouse_x, mouse_y);
        // Diagnostic, once: what the game's tooltip sweep (`tooltip::draw_hovered_tooltip`) will see for a hovered tab.
        static DIAG_HOVER: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
        if widget.is_hovered() && !DIAG_HOVER.swap(true, std::sync::atomic::Ordering::Relaxed) {
            logger().warn(&format!("[Skyblocker diag] quick nav tab {} hovered: box=({},{} {}x{}) visible={} tooltip={:?} clip={:?}", index, widget.get_x(), widget.get_y(), widget.width, widget.height, widget.visible, widget.tooltip_string(), widget.visible_through_clip()));
        }
        let visible = widget.visible;
        compat::write(quick_nav_button, |__t| __t.quick_nav_button_base_mut().base.visible = visible);
    }
}

/// `extractContents` HEAD: the frame's partial tick for the two draws.
pub fn skyblocker_record_partial_tick(a: f32) {
    FRAME_A.with(|cell| cell.set(a));
}

/// @Override
///
/// Draws the selected tab in front of the background and the main inventory, similar to creative inventory tabs.
pub fn skyblocker_extract_selected_quick_nav_buttons() {
    let Some((quick_nav_buttons, mouse_x, mouse_y, a)) = frame_buttons() else { return; };
    for (quick_nav_button, _index) in quick_nav_buttons.iter() {
        if { let __f = compat::dispatch(quick_nav_button, |__t| __t.toggled()); __f(quick_nav_button) } {
            ({ let __f = compat::dispatch(quick_nav_button, |__t| __t.set_render_in_front()); __f(quick_nav_button, true) });
            draw_quick_nav_button(quick_nav_button, mouse_x, mouse_y, a);
        }
    }
}

/// @Inject skyblocker$initQuickNav
///   java: @Inject(method = "init()V", at = @At(value = "TAIL"))
/// target: net.minecraft.client.gui.screens.inventory.AbstractContainerScreen#init  [exists]
/// Rust: net/minecraft/client/gui/screens/inventory/abstract_container_screen.rs:1374 fn init(&mut self, width: i32, height: i32)
fn skyblocker_init_quick_nav(ci: &mut CallbackInfo<'_>) {
    let Some(this) = compat::detach(ci.this_mut::<mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::AbstractContainerScreen>()) else { return; };
    let instance: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen = &mut this.screen;
    if crate::de::hysky::skyblocker::utils::utils::Utils::is_on_skyblock() && crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().quick_nav.enable_quick_nav && mcrust::mcrust::modding::game::has_player() && !(mcrust::mcrust::modding::game::with_player(|player| player.base.base.is_creative()).unwrap_or(false)) && !(crate::entry::catharsis_compatibility::is_gui_element_hidden("skyblocker:quickNavigation")) {
        // `this.quickNavButtons = QuickNav.init(instance.getTitle().getString().trim())`
        let quick_nav_buttons: Vec<compat::Shared<dyn crate::de::hysky::skyblocker::skyblock::quicknav::quick_nav_button::QuickNavButton>> = crate::de::hysky::skyblocker::skyblock::quicknav::quick_nav::QuickNav::init(compat::trim(instance.get_title()).as_str());
        let mut added = Vec::with_capacity(quick_nav_buttons.len());
        for quick_nav_button in quick_nav_buttons.iter() {
            //   java: instance.addWidget(quickNavButton);
            let index = instance.add_widget(crate::de::hysky::skyblocker::skyblock::quicknav::quick_nav_button::QuickNavButtonBase::widget(quick_nav_button.clone()));
            added.push((quick_nav_button.clone(), index));
        }
        let id = instance.mod_event_id();
        QUICK_NAV_BUTTONS.get_mut().insert(id, added);
        // the field dies with the screen object
        screen_events::remove(&*instance, Arc::new(move |_screen: &Screen| {
            QUICK_NAV_BUTTONS.get_mut().remove(&id);
        }));
    } else {
        // `this.quickNavButtons` is not assigned: a re-init that no longer qualifies keeps nothing drawn (a new
        // object's field is null).
        QUICK_NAV_BUTTONS.get_mut().remove(&instance.mod_event_id());
    }
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    report(hook::register_injector("net.minecraft.client.gui.screens.inventory.AbstractContainerScreen#init", InjectOptions::at(At::Tail).kind(InjectorKind::Inject).priority(1000), Arc::new(skyblocker_init_quick_nav)));
}

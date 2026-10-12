//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.QuickNavMixin by fabric2seer.
//! @Mixin(net.minecraft.client.gui.screens.inventory.AbstractContainerScreen)

#[allow(unused_imports)]
use crate::prelude::*;

/// java: @Unique private @Nullable List<QuickNavButton> quickNavButtons;
/// One value per AbstractContainerScreen OBJECT (a Mixin field).
/// rust: port gap - the port's `Screen` is no `AttachmentTarget` (attachments reach entities, item stacks and levels
/// only; GAPS_batchD.md), so the field is kept for the one container screen that is open: `init` (TAIL) writes it for
/// the screen being initialised, which is the screen the extract handlers then run for. A screen that is shown again
/// without a new `init` reads the last initialised screen's buttons.
/// Each button with the index of the game's widget `instance.addWidget(quickNavButton)` made for it in the screen's
/// widget list (the game keeps the live widget - its box, `visible`, `isHovered` - and lends it to the button for each
/// of the button's own methods; see `compat::WidgetAdapter`).
pub static QUICK_NAV_BUTTONS: compat::MainThread<Option<Vec<(compat::Shared<dyn crate::de::hysky::skyblocker::skyblock::quicknav::quick_nav_button::QuickNavButton>, usize)>>> = compat::MainThread::new(|| None);

/// `quickNavButton.extractRenderState(graphics, mouseX, mouseY, a)` - `AbstractWidget.extractRenderState:55-61` for a
/// button the screen holds as a child (`addWidget`) and the mixin draws itself:
/// `if (this.visible) { this.isHovered = ..; this.extractWidgetRenderState(..); this.tooltip.refresh..(..) }`.
/// `isHovered` is set on the game's widget, where the screen's tooltip sweep (the holder's refresh) and the cursor
/// read it; the button's box is brought over from the button first (its `updateCoordinates` moves it during the draw,
/// and Java's `isHovered` reads the box as the last draw left it) and again after, so the next click hits it there.
fn extract_quick_nav_button(state: &mut mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::AbstractContainerScreen, quick_nav_button: &compat::Shared<dyn crate::de::hysky::skyblocker::skyblock::quicknav::quick_nav_button::QuickNavButton>, index: usize, mouse_x: i32, mouse_y: i32, a: f32) {
    if index >= state.screen.children().len() {
        return;
    }
    sync_box(state, quick_nav_button, index);
    let widget = state.screen.widget_mut(index);
    widget.set_hovered_from_extract(mouse_x, mouse_y);
    if !widget.visible {
        return;
    }
    let graphics = mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor::new();
    ({ let __f = compat::dispatch(quick_nav_button, |__t| __t.extract_widget_render_state()); __f(quick_nav_button, &graphics, mouse_x, mouse_y, a) });
    sync_box(state, quick_nav_button, index);
}

/// The button's box (`setX` / `setY` in its `updateCoordinates`) onto the game's widget.
fn sync_box(state: &mut mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::AbstractContainerScreen, quick_nav_button: &compat::Shared<dyn crate::de::hysky::skyblocker::skyblock::quicknav::quick_nav_button::QuickNavButton>, index: usize) {
    let (x, y, width, height) = compat::read(quick_nav_button, |__t| {
        let base = &__t.quick_nav_button_base().base;
        (base.get_x(), base.get_y(), base.width, base.height)
    });
    let widget = state.screen.widget_mut(index);
    if widget.get_x() != x {
        widget.set_x(x);
    }
    if widget.get_y() != y {
        widget.set_y(y);
    }
    widget.width = width;
    widget.height = height;
}

/// @Override
///
/// Draws the unselected tabs in front of the background, but behind the main inventory, similar to creative inventory tabs.
pub fn skyblocker_extract_unselected_quick_nav_buttons(this: &mut mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::AbstractContainerScreen, mouse_x: i32, mouse_y: i32, a: f32) {
    if let Some(quick_nav_buttons) = QUICK_NAV_BUTTONS.get().clone() {
        for (quick_nav_button, index) in quick_nav_buttons.iter() {
            // Render the button behind the main inventory background if it's not toggled or if it's still fading in
            if !({ let __f = compat::dispatch(quick_nav_button, |__t| __t.toggled()); __f(quick_nav_button) }) || ({ let __f = compat::dispatch(quick_nav_button, |__t| __t.get_alpha()); __f(quick_nav_button) }) < (255 as f32) {
                ({ let __f = compat::dispatch(quick_nav_button, |__t| __t.set_render_in_front()); __f(quick_nav_button, false) });
                extract_quick_nav_button(this, quick_nav_button, *index, mouse_x, mouse_y, a);
            }
        }
    }
}

/// @Override
///
/// Draws the selected tab in front of the background and the main inventory, similar to creative inventory tabs.
pub fn skyblocker_extract_selected_quick_nav_buttons(this: &mut mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::AbstractContainerScreen, mouse_x: i32, mouse_y: i32, a: f32) {
    if let Some(quick_nav_buttons) = QUICK_NAV_BUTTONS.get().clone() {
        for (quick_nav_button, index) in quick_nav_buttons.iter() {
            if { let __f = compat::dispatch(quick_nav_button, |__t| __t.toggled()); __f(quick_nav_button) } {
                ({ let __f = compat::dispatch(quick_nav_button, |__t| __t.set_render_in_front()); __f(quick_nav_button, true) });
                extract_quick_nav_button(this, quick_nav_button, *index, mouse_x, mouse_y, a);
            }
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
    if crate::de::hysky::skyblocker::utils::utils::Utils::is_on_skyblock() && crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().quick_nav.enable_quick_nav && mcrust::mcrust::modding::game::with_player(|_| ()).is_some() && !(mcrust::mcrust::modding::game::with_player(|player| player.base.base.is_creative()).unwrap_or(false)) && !(crate::entry::catharsis_compatibility::is_gui_element_hidden("skyblocker:quickNavigation")) {
        // `this.quickNavButtons = QuickNav.init(instance.getTitle().getString().trim())`
        let quick_nav_buttons: Vec<compat::Shared<dyn crate::de::hysky::skyblocker::skyblock::quicknav::quick_nav_button::QuickNavButton>> = crate::de::hysky::skyblocker::skyblock::quicknav::quick_nav::QuickNav::init(compat::trim(instance.get_title()).as_str());
        let mut added = Vec::with_capacity(quick_nav_buttons.len());
        for quick_nav_button in quick_nav_buttons.iter() {
            //   java: instance.addWidget(quickNavButton);
            let index = instance.add_widget(crate::de::hysky::skyblocker::skyblock::quicknav::quick_nav_button::QuickNavButtonBase::widget(quick_nav_button.clone()));
            added.push((quick_nav_button.clone(), index));
        }
        *QUICK_NAV_BUTTONS.get_mut() = Some(added);
    } else {
        // A new screen object's field is null; the static stands for the screen being initialised.
        *QUICK_NAV_BUTTONS.get_mut() = None;
    }
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    report(hook::register_injector("net.minecraft.client.gui.screens.inventory.AbstractContainerScreen#init", InjectOptions::at(At::Tail).kind(InjectorKind::Inject).priority(1000), Arc::new(skyblocker_init_quick_nav)));
}

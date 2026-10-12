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
pub static QUICK_NAV_BUTTONS: compat::MainThread<Option<Vec<compat::Shared<dyn crate::de::hysky::skyblocker::skyblock::quicknav::quick_nav_button::QuickNavButton>>>> = compat::MainThread::new(|| None);

/// @Override
fn skyblocker_extract_unselected_quick_nav_buttons(this: &mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::AbstractContainerScreen, graphics: &mut mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor, mouse_x: i32, mouse_y: i32, a: f32, ci: /* TODO: CallbackInfo */ ()) {
    let _ = this;
    if let Some(quick_nav_buttons) = QUICK_NAV_BUTTONS.get().clone() {
        for quick_nav_button in quick_nav_buttons.iter() {
            if !({ let __f = compat::dispatch(quick_nav_button, |__t| __t.toggled()); __f(quick_nav_button) }) || ({ let __f = compat::dispatch(quick_nav_button, |__t| __t.get_alpha()); __f(quick_nav_button) }) < (255 as f32) {
                ({ let __f = compat::dispatch(quick_nav_button, |__t| __t.set_render_in_front()); __f(quick_nav_button, false) });
                // TODO(fabric2seer): `extract_render_state` takes 5 argument(s) in the port (`gl: &Gl, gfx: &Minecraft, mouse_x: i32, mouse_y: i32, a: f32`), Java passes 4
                // TODO(fabric2seer): `extract_render_state` is a method of the port's trait `Renderable`, which the port's `AbstractWidget` does not implement
                // rust: let __wa1 = graphics.clone();
                // rust: let __wa2 = mouse_x;
                // rust: let __wa3 = mouse_y;
                // rust: let __wa4 = a;
                // rust: let __o1 = quick_nav_button.clone();
                // rust: compat::write(&__o1, |__t| <_ as mcrust::net::minecraft::client::gui::components::renderable::Renderable>::extract_render_state(__t.quick_nav_button_base_mut(), &__wa1, &__wa2, __wa3, __wa4));
            }
        }
    }
}

/// @Override
fn skyblocker_extract_selected_quick_nav_buttons(this: &mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::AbstractContainerScreen, graphics: &mut mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor, mouse_x: i32, mouse_y: i32, a: f32, ci: /* TODO: CallbackInfo */ ()) {
    let _ = this;
    if let Some(quick_nav_buttons) = QUICK_NAV_BUTTONS.get().clone() {
        for quick_nav_button in quick_nav_buttons.iter() {
            if { let __f = compat::dispatch(quick_nav_button, |__t| __t.toggled()); __f(quick_nav_button) } {
                ({ let __f = compat::dispatch(quick_nav_button, |__t| __t.set_render_in_front()); __f(quick_nav_button, true) });
                // TODO(fabric2seer): `extract_render_state` takes 5 argument(s) in the port (`gl: &Gl, gfx: &Minecraft, mouse_x: i32, mouse_y: i32, a: f32`), Java passes 4
                // TODO(fabric2seer): `extract_render_state` is a method of the port's trait `Renderable`, which the port's `AbstractWidget` does not implement
                // rust: let __wa1 = graphics.clone();
                // rust: let __wa2 = mouse_x;
                // rust: let __wa3 = mouse_y;
                // rust: let __wa4 = a;
                // rust: let __o1 = quick_nav_button.clone();
                // rust: compat::write(&__o1, |__t| <_ as mcrust::net::minecraft::client::gui::components::renderable::Renderable>::extract_render_state(__t.quick_nav_button_base_mut(), &__wa1, &__wa2, __wa3, __wa4));
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
        *QUICK_NAV_BUTTONS.get_mut() = Some(quick_nav_buttons.clone());
        for quick_nav_button in quick_nav_buttons.iter() {
            // TODO(batchD): cross-package - `QuickNavButton` (a mod `AbstractWidget` subclass held as
            // `Shared<dyn QuickNavButton>`) has no `WidgetHalf` / widget adapter, so it cannot be handed to the game's
            // widget list; `compat::WidgetAdapter::widget` needs a concrete class.
            //   java: instance.addWidget(quickNavButton);
            let _ = quick_nav_button;
        }
    } else {
        // A new screen object's field is null; the static stands for the screen being initialised.
        *QUICK_NAV_BUTTONS.get_mut() = None;
    }
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    report(hook::register_injector("net.minecraft.client.gui.screens.inventory.AbstractContainerScreen#init", InjectOptions::at(At::Tail).kind(InjectorKind::Inject).priority(1000), Arc::new(skyblocker_init_quick_nav)));
}

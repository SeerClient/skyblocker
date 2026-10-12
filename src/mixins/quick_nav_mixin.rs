//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.QuickNavMixin by fabric2seer.
//! @Mixin(net.minecraft.client.gui.screens.inventory.AbstractContainerScreen)

#[allow(unused_imports)]
use crate::prelude::*;

/// java: @Unique private @Nullable List<QuickNavButton> quickNavButtons;
/// One value per AbstractContainerScreen OBJECT (a Mixin field): an attachment.
pub static ATTACH_QUICK_NAV_BUTTONS: AttachmentType<Vec<compat::Shared<dyn crate::de::hysky::skyblocker::skyblock::quicknav::quick_nav_button::QuickNavButton>>> = AttachmentType::new("skyblocker:quick_nav_mixin_quick_nav_buttons");

/// @Override
fn skyblocker_extract_unselected_quick_nav_buttons(this: &mut mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::AbstractContainerScreen, graphics: &mut mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor, mouse_x: i32, mouse_y: i32, a: f32, ci: /* TODO: CallbackInfo */ ()) {
    if this.get_attached(&ATTACH_QUICK_NAV_BUTTONS).is_some() {
        for quick_nav_button in this.get_attached(&ATTACH_QUICK_NAV_BUTTONS).unwrap().iter_mut() {
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
fn skyblocker_extract_selected_quick_nav_buttons(this: &mut mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::AbstractContainerScreen, graphics: &mut mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor, mouse_x: i32, mouse_y: i32, a: f32, ci: /* TODO: CallbackInfo */ ()) {
    if this.get_attached(&ATTACH_QUICK_NAV_BUTTONS).is_some() {
        for quick_nav_button in this.get_attached(&ATTACH_QUICK_NAV_BUTTONS).unwrap().iter_mut() {
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
    let mut instance: mcrust::net::minecraft::client::gui::screens::screen::Screen = this;
    if crate::de::hysky::skyblocker::utils::utils::Utils::is_on_skyblock() && crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().quick_nav.enable_quick_nav && mcrust::mcrust::modding::game::with_player(|_| ()).is_some() && !(mcrust::mcrust::modding::game::with_player(|player| player.base.base.is_creative()).unwrap_or(false)) && !(crate::entry::catharsis_compatibility::is_gui_element_hidden("skyblocker:quickNavigation")) {
        // TODO(fabric2seer): an assignment used as a value
        //   java: for (QuickNavButton quickNavButton : this.quickNavButtons = QuickNav.init(instance.getTitle().getString().trim())) {
        //   java: 				instance.addWidget(quickNavButton);
        //   java: 			}
    }
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    report(hook::register_injector("net.minecraft.client.gui.screens.inventory.AbstractContainerScreen#init", InjectOptions::at(At::Tail).kind(InjectorKind::Inject).priority(1000), Arc::new(skyblocker_init_quick_nav)));
}

//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.ContainerScreenMixin by fabric2seer.
//! @Mixin(net.minecraft.client.gui.screens.inventory.ContainerScreen)

#[allow(unused_imports)]
use crate::prelude::*;

/// @SuppressWarnings("unused")
/// `this` is the `ContainerScreen`'s `AbstractContainerScreen` half (`ContainerScreen.container`), which holds every
/// field the override reads: `width` (its `Screen`), `imageWidth`, `leftPos`, `topPos`.
fn init(this: &mut mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::AbstractContainerScreen) {
    // java: super.init(); - the override runs at the TAIL of `AbstractContainerScreen#init` (see `register`).
    if crate::de::hysky::skyblocker::utils::utils::Utils::is_on_skyblock() && crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().ui_and_visuals.museum_overlay && mcrust::mcrust::modding::game::has_player() && this.screen.get_title().to_string().contains("Museum") {
        let mut overlay_width: i32 = crate::de::hysky::skyblocker::skyblock::museum::museum_manager::BACKGROUND_WIDTH;
        let mut spacing: i32 = crate::de::hysky::skyblocker::skyblock::museum::museum_manager::SPACING;
        // Default: center inventory
        let mut inventory_x: i32 = (this.screen.width.wrapping_sub(this.image_width)).wrapping_div(2);
        // If overlay would go off the right edge, shift inventory left
        if ((inventory_x.wrapping_add(this.image_width)).wrapping_add(spacing)).wrapping_add(overlay_width) > this.screen.width {
            inventory_x = this.screen.width.wrapping_sub((this.image_width.wrapping_add(overlay_width)).wrapping_add(spacing));
            if inventory_x < 0 {
                inventory_x = 0;
            }
        }
        this.left_pos = inventory_x;
        //   java: new MuseumManager(this, this.leftPos, this.topPos, this.imageWidth);
        crate::de::hysky::skyblocker::skyblock::museum::museum_manager::MuseumManager::new(&mut this.screen, this.left_pos, this.top_pos, this.image_width);
    }
}

/// `protected void init()` declared in the mixin class overrides `ContainerScreen`'s inherited
/// `AbstractContainerScreen.init`: the TAIL of the base `init`, for a screen that is a `ContainerScreen` - the kind
/// `setScreen` records before the new screen's `init` (`Gui.screen_kind`), so a re-init on resize sees it too.
fn init_override(ci: &mut CallbackInfo<'_>) {
    if compat::minecraft().gui.screen_kind != mcrust::net::minecraft::client::gui::gui::ScreenKind::Container {
        return;
    }
    let Some(this) = compat::detach(ci.this_mut::<mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::AbstractContainerScreen>()) else { return; };
    init(this);
}

/// @ModifyVariable x
///   java: @ModifyVariable(method = "extractBackground", at = @At("STORE"), name = "xo")
///   java: public int x(int ignored) { return this.leftPos; }
/// The STORE is the game's hooked `ContainerScreen#xo(this, xo)`; `this` is argument 0.
fn x(ci: &mut CallbackInfo<'_>) {
    let Some(this) = ci.arg::<mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::AbstractContainerScreen>(0) else { return; };
    let left_pos = this.left_pos;
    ci.set_return_value(left_pos);
}

/// @ModifyVariable y
///   java: @ModifyVariable(method = "extractBackground", at = @At("STORE"), name = "yo")
///   java: public int y(int ignored) { return this.topPos; }
fn y(ci: &mut CallbackInfo<'_>) {
    let Some(this) = ci.arg::<mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::AbstractContainerScreen>(0) else { return; };
    let top_pos = this.top_pos;
    ci.set_return_value(top_pos);
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    report(hook::register_injector("net.minecraft.client.gui.screens.inventory.AbstractContainerScreen#init", InjectOptions::at(At::Tail).kind(InjectorKind::Inject).priority(1000), Arc::new(init_override)));
    report(hook::register_injector("net.minecraft.client.gui.screens.inventory.ContainerScreen#xo", InjectOptions::at(At::Return).kind(InjectorKind::ModifyReturnValue).priority(1000), Arc::new(x)));
    report(hook::register_injector("net.minecraft.client.gui.screens.inventory.ContainerScreen#yo", InjectOptions::at(At::Return).kind(InjectorKind::ModifyReturnValue).priority(1000), Arc::new(y)));
}

//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.ContainerScreenMixin by fabric2seer.
//! @Mixin(net.minecraft.client.gui.screens.inventory.ContainerScreen)

#[allow(unused_imports)]
use crate::prelude::*;

/// @SuppressWarnings("unused")
fn init(this: &mut mcrust::net::minecraft::client::gui::screens::inventory::container_screen::ContainerScreen) {
    // TODO(fabric2seer): `super.m(..)` outside an instance method
    //   java: super.init();
    // TODO(fabric2seer): `Screen::get_title` ports `getTitle`, but the port's `ContainerScreen` holds no `Screen` (no Deref, no `base` field) to call it on
    if crate::de::hysky::skyblocker::utils::utils::Utils::is_on_skyblock() && crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().ui_and_visuals.museum_overlay && mcrust::mcrust::modding::game::with_player(|_| ()).is_some() && this.get_title().to_string().contains("Museum") {
        let mut overlay_width: i32 = crate::de::hysky::skyblocker::skyblock::museum::museum_manager::BACKGROUND_WIDTH;
        let mut spacing: i32 = crate::de::hysky::skyblocker::skyblock::museum::museum_manager::SPACING;
        // TODO(fabric2seer): `Screen.width` is a field of the port's `Screen`, which the port's `ContainerScreen` does not hold (no Deref, no `base` field)
        // rust: let mut inventory_x: i32 = (this.width.wrapping_sub(this.container.image_width)).wrapping_div(2);
        let mut inventory_x: i32 = todo!("fabric2seer: the initialiser of `inventoryX` (TODO above)");
        // TODO(fabric2seer): `Screen.width` is a field of the port's `Screen`, which the port's `ContainerScreen` does not hold (no Deref, no `base` field)
        if ((inventory_x.wrapping_add(this.container.image_width)).wrapping_add(spacing)).wrapping_add(overlay_width) > this.width {
            // TODO(fabric2seer): `Screen.width` is a field of the port's `Screen`, which the port's `ContainerScreen` does not hold (no Deref, no `base` field)
            // rust: inventory_x = this.width.wrapping_sub((this.container.image_width.wrapping_add(overlay_width)).wrapping_add(spacing));
            if inventory_x < 0 {
                inventory_x = 0;
            }
        }
        this.left_pos = inventory_x;
        crate::de::hysky::skyblocker::skyblock::museum::museum_manager::MuseumManager::new(this, this.container.left_pos, this.container.top_pos, this.container.image_width);
    }
}

/// @ModifyVariable x
///   java: @ModifyVariable(method = "extractBackground", at = @At("STORE"), name = "xo")
/// note: @ModifyVariable changes a local inside the method body; the port's hooks see only arguments, `this` and the return value (MODDING.md section 8). Hook a smaller method the local flows into, or @Overwrite and reimplement.
fn x(ci: &mut CallbackInfo<'_>) {
    // TODO(fabric2seer): @ModifyVariable is not expressible - the handler is kept for reference
    //   java: @ModifyVariable(method = "extractBackground", at = @At("STORE"), name = "xo")
    //   java: 	public int x(int ignored) {
    //   java: 		return this.leftPos;
    //   java: 	}
}

/// @ModifyVariable y
///   java: @ModifyVariable(method = "extractBackground", at = @At("STORE"), name = "yo")
/// note: @ModifyVariable changes a local inside the method body; the port's hooks see only arguments, `this` and the return value (MODDING.md section 8). Hook a smaller method the local flows into, or @Overwrite and reimplement.
fn y(ci: &mut CallbackInfo<'_>) {
    // TODO(fabric2seer): @ModifyVariable is not expressible - the handler is kept for reference
    //   java: @ModifyVariable(method = "extractBackground", at = @At("STORE"), name = "yo")
    //   java: 	public int y(int ignored) {
    //   java: 		return this.topPos;
    //   java: 	}
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    // TODO(fabric2seer): @ModifyVariable x: nothing to register - @ModifyVariable changes a local inside the method body; the port's hooks see only arguments, `this` and the return value (MODDING.md section 8). Hook a smaller method the local flows into, or @Overwrite and reimplement.
    // TODO(fabric2seer): @ModifyVariable y: nothing to register - @ModifyVariable changes a local inside the method body; the port's hooks see only arguments, `this` and the return value (MODDING.md section 8). Hook a smaller method the local flows into, or @Overwrite and reimplement.
}

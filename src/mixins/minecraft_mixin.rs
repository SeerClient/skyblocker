//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.MinecraftMixin by fabric2seer.
//! @Mixin(net.minecraft.client.Minecraft)

#[allow(unused_imports)]
use crate::prelude::*;

/// @Inject skyblocker$handleInputEvents
///   java: @Inject(method = "handleKeybinds", at = @At("HEAD"))
/// target: net.minecraft.client.Minecraft#handleKeybinds  [exists]
/// Rust: net/minecraft/client/minecraft.rs:2131 fn handle_keybinds(gfx: &mut Minecraft, g: &crate::lwjgl::glfw::Glfw, handle: *mut std::ffi::c_void, options_data: &OptionsData, player_shared: &std::sync::Arc< std::sync::Mutex<crate::net::minecraft::client::player::local_player::LocalPlayer>, >, level: &std::sync::Arc<crate::net::minecraft::client
fn skyblocker_handle_input_events(ci: &mut CallbackInfo<'_>) {
    let Some(this) = compat::detach(ci.arg_mut::<mcrust::net::minecraft::client::minecraft::Minecraft>(0)) else { return; };
    // `this`: this port's handleKeybinds is a free function; its `gfx` argument (index 0) is the Minecraft
    // screen == null prevents double-toggle when a container screen is open
    // `player != null`: the player is lent through `with_player` (Java's `player` field).
    if this.player.is_some() && this.gui.screen().is_none() && crate::de::hysky::skyblocker::utils::utils::Utils::is_on_skyblock() {
        mcrust::mcrust::modding::game::with_player(|player| {
            crate::de::hysky::skyblocker::skyblock::item::hotbar_slot_lock::HotbarSlotLock::handle_input_events(player);
            crate::de::hysky::skyblocker::skyblock::item::item_protection::ItemProtection::handle_hotbar_key_pressed(player);
        });
    }
}

/// @WrapOperation skyblocker$skyblockInventoryScreen
///   java: @WrapOperation(method = "handleKeybinds", at = @At(value = "NEW", target = "Lnet/minecraft/client/gui/screens/inventory/InventoryScreen;"))
/// note: @At("NEW") is not expressible: SeerClient hooks fire at a method's HEAD, RETURN and TAIL, and at a hooked CALLEE inside another hooked method (`within`); field access, `new`, constants and jumps are not hook points
///
/// `new InventoryScreen(player)` is asked of `container_screens::register_inventory_screen`'s factories at every
/// construction site the client has (the inventory key in `handleKeybinds`, leaving creative): the factory is this
/// wrapper's body - `Some` is `new SkyblockInventoryScreen(player)`, `None` is `original.call(player)`.
fn skyblocker_skyblock_inventory_screen() -> Option<Box<dyn mcrust::mcrust::modding::container_screens::ModContainerScreen>> {
    //   java: return Utils.isOnSkyblock() && SkyblockerConfigManager.get().uiAndVisuals.showEquipmentInInventory ? new SkyblockInventoryScreen(player) : original.call(player);
    crate::de::hysky::skyblocker::skyblock::item::skyblock_inventory_screen::SkyblockInventoryScreen::inventory_screen_factory()
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    report(hook::register_injector("net.minecraft.client.Minecraft#handleKeybinds", InjectOptions::at(At::Head).kind(InjectorKind::Inject).priority(1000), Arc::new(skyblocker_handle_input_events)));
    // @WrapOperation skyblocker$skyblockInventoryScreen (`@At("NEW") InventoryScreen`): the inventory-screen factory
    mcrust::mcrust::modding::container_screens::register_inventory_screen(skyblocker_skyblock_inventory_screen);
}

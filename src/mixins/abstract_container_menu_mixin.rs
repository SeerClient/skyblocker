//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.AbstractContainerMenuMixin by fabric2seer.
//! @Mixin(net.minecraft.world.inventory.AbstractContainerMenu)

#[allow(unused_imports)]
use crate::prelude::*;

/// @Inject beforeSetStackInSlot
///   java: @Inject(method = "setItem", at = @At("HEAD"))
/// target: net.minecraft.world.inventory.AbstractContainerMenu#setItem  [exists]
/// Rust: net/minecraft/world/inventory/abstract_container_menu.rs:690 fn set_item(&mut self, containers: &mut Containers<'_>, slot: i32, state_id: i32, item_stack: ItemStack)
fn before_set_stack_in_slot(ci: &mut CallbackInfo<'_>) {
    let Some(this) = compat::detach(ci.this_mut::<mcrust::net::minecraft::world::inventory::abstract_container_menu::AbstractContainerMenu>()) else { return; };
    let slot: i32 = ci.arg::<i32>(1).copied().unwrap_or_default();
    let revision: i32 = ci.arg::<i32>(2).copied().unwrap_or_default();
    let Some(stack) = compat::detach_ref(ci.arg::<mcrust::net::minecraft::world::item::item_stack::ItemStack>(3)) else { return; };
    crate::de::hysky::skyblocker::skyblock::item_pickup_widget::ItemPickupWidget::on_item_pickup(&crate::de::hysky::skyblocker::skyblock::item_pickup_widget::ItemPickupWidget::get_instance(), slot, stack.clone());
}

/// @Inject onSetStackInSlot
///   java: @Inject(method = "setItem", at = @At("RETURN"))
/// target: net.minecraft.world.inventory.AbstractContainerMenu#setItem  [exists]
/// Rust: net/minecraft/world/inventory/abstract_container_menu.rs:690 fn set_item(&mut self, containers: &mut Containers<'_>, slot: i32, state_id: i32, item_stack: ItemStack)
fn on_set_stack_in_slot(ci: &mut CallbackInfo<'_>) {
    let Some(this) = compat::detach(ci.this_mut::<mcrust::net::minecraft::world::inventory::abstract_container_menu::AbstractContainerMenu>()) else { return; };
    let slot: i32 = ci.arg::<i32>(1).copied().unwrap_or_default();
    let revision: i32 = ci.arg::<i32>(2).copied().unwrap_or_default();
    let Some(stack) = compat::detach_ref(ci.arg::<mcrust::net::minecraft::world::item::item_stack::ItemStack>(3)) else { return; };
    crate::de::hysky::skyblocker::utils::container::container_solver_manager::ContainerSolverManager::mark_highlights_dirty();
    if crate::de::hysky::skyblocker::skyblock::inventory_search::InventorySearch::is_searching() {
        crate::de::hysky::skyblocker::skyblock::inventory_search::InventorySearch::refresh_slot(slot);
    }
    if this.is::<compat::LiveChestMenu>() {
        if let Some(__sel1) = compat::minecraft().gui.screen().as_ref() && let Some(screen) = mcrust::mcrust::modding::game::current_mod_screen::<compat::ScreenAdapter<crate::de::hysky::skyblocker::skyblock::dungeon::partyfinder::party_finder_screen::PartyFinderScreen>, _>(|__s| __s.object().clone()) {
            crate::de::hysky::skyblocker::skyblock::dungeon::partyfinder::party_finder_screen::PartyFinderScreen::mark_dirty(&screen);
        } else if let Some(__sel3) = compat::minecraft().gui.screen().as_ref() && let Some(screen) = mcrust::mcrust::modding::container_screens::current_mod_container_screen::<compat::ContainerScreenAdapter<crate::de::hysky::skyblocker::skyblock::storageoverlay::storage_overlay_screen::StorageOverlayScreen>, _>(|__s| __s.object().clone()) {
            crate::de::hysky::skyblocker::skyblock::storageoverlay::storage_overlay_screen::StorageOverlayScreen::refresh_search(&screen);
        } else {
        }
        // TODO(fabric2seer): `broadcast_changes` takes 1 argument(s) in the port (`containers: &mut Containers<'_>`), Java passes 0
        // rust: this.broadcast_changes();
    }
}

/// @Inject initializeContents
///   java: @Inject(method = "initializeContents", at = @At("RETURN"))
/// target: net.minecraft.world.inventory.AbstractContainerMenu#initializeContents  [exists]
/// Rust: net/minecraft/world/inventory/abstract_container_menu.rs:718 fn initialize_contents(&mut self, containers: &mut Containers<'_>, state_id: i32, items: Vec<ItemStack>, carried: ItemStack)
fn initialize_contents(ci: &mut CallbackInfo<'_>) {
    let Some(this) = compat::detach(ci.this_mut::<mcrust::net::minecraft::world::inventory::abstract_container_menu::AbstractContainerMenu>()) else { return; };
    let state_id: i32 = ci.arg::<i32>(1).copied().unwrap_or_default();
    // TODO(fabric2seer): `items` is Rust argument 2: `Vec<ItemStack>` - generic or borrowed with a lifetime; read it with `unsafe { ci.arg_unchecked::<..>(2) }`
    let Some(carried) = compat::detach_ref(ci.arg::<mcrust::net::minecraft::world::item::item_stack::ItemStack>(3)) else { return; };
    crate::de::hysky::skyblocker::utils::container::container_solver_manager::ContainerSolverManager::mark_highlights_dirty();
    if this.is::<compat::LiveChestMenu>() {
        if let Some(__sel1) = compat::minecraft().gui.screen().as_ref() && let Some(screen) = mcrust::mcrust::modding::game::current_mod_screen::<compat::ScreenAdapter<crate::de::hysky::skyblocker::skyblock::dungeon::partyfinder::party_finder_screen::PartyFinderScreen>, _>(|__s| __s.object().clone()) {
            crate::de::hysky::skyblocker::skyblock::dungeon::partyfinder::party_finder_screen::PartyFinderScreen::mark_dirty(&screen);
        } else if let Some(__sel3) = compat::minecraft().gui.screen().as_ref() && let Some(screen) = mcrust::mcrust::modding::container_screens::current_mod_container_screen::<compat::ContainerScreenAdapter<crate::de::hysky::skyblocker::skyblock::storageoverlay::storage_overlay_screen::StorageOverlayScreen>, _>(|__s| __s.object().clone()) {
            crate::de::hysky::skyblocker::skyblock::storageoverlay::storage_overlay_screen::StorageOverlayScreen::refresh_search(&screen);
        } else {
        }
        // TODO(fabric2seer): `broadcast_changes` takes 1 argument(s) in the port (`containers: &mut Containers<'_>`), Java passes 0
        // rust: this.broadcast_changes();
    }
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    report(hook::register_injector("net.minecraft.world.inventory.AbstractContainerMenu#setItem", InjectOptions::at(At::Head).kind(InjectorKind::Inject).priority(1000), Arc::new(before_set_stack_in_slot)));
    report(hook::register_injector("net.minecraft.world.inventory.AbstractContainerMenu#setItem", InjectOptions::at(At::Return).kind(InjectorKind::Inject).priority(1000), Arc::new(on_set_stack_in_slot)));
    report(hook::register_injector("net.minecraft.world.inventory.AbstractContainerMenu#initializeContents", InjectOptions::at(At::Return).kind(InjectorKind::Inject).priority(1000), Arc::new(initialize_contents)));
}

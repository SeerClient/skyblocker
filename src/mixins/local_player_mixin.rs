//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.LocalPlayerMixin by fabric2seer.
//! @Mixin(net.minecraft.client.player.LocalPlayer)

#[allow(unused_imports)]
use crate::prelude::*;

/// @Inject skyblocker$dropSelectedItem
///   java: @Inject(method = "drop(Z)Z", at = @At("HEAD"), cancellable = true)
/// target: net.minecraft.client.player.LocalPlayer#drop  [exists]
/// Rust: net/minecraft/client/player/local_player.rs:7934 fn drop(&mut self, all: bool)
fn skyblocker_drop_selected_item(ci: &mut CallbackInfo<'_>) {
    let Some(this) = compat::detach(ci.this_mut::<mcrust::net::minecraft::client::player::local_player::LocalPlayer>()) else { return; };
    let item: mcrust::net::minecraft::world::item::item_stack::ItemStack = <mcrust::net::minecraft::client::player::local_player::LocalPlayer as mcrust::net::minecraft::world::entity::living_entity::LivingEntity>::get_main_hand_item(&*this).duplicate();
    if crate::de::hysky::skyblocker::utils::utils::Utils::is_on_skyblock() && (crate::de::hysky::skyblocker::skyblock::item::item_protection::ItemProtection::is_item_protected(Some(item.duplicate())) || crate::de::hysky::skyblocker::skyblock::item::hotbar_slot_lock::HotbarSlotLock::is_locked(this.base.base.get_inventory().get_selected_slot())) {
        let mut should_drop_in_dungeons: bool = (crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().dungeons.allow_dropping_protected_items && crate::de::hysky::skyblocker::skyblock::dungeon::dungeon_score::DungeonScore::is_dungeon_started());
        let mut should_drop_shining_coins: bool = (crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().hunting.haunted_biome.ignore_slot_locking_for_shining_coins && crate::de::hysky::skyblocker::skyblock::hunting::safari::safari_utils::SafariUtils::is_in_haunted_biome() && item.get_skyblock_id() == "SHINING_COIN");
        if !should_drop_in_dungeons && !should_drop_shining_coins {
            ci.set_return_value(false);
        }
    }
}

/// @Inject skyblocker$updateHealth
///   java: @Inject(method = "hurtTo", at = @At("RETURN"))
/// target: net.minecraft.client.player.LocalPlayer#hurtTo  [exists]
/// Rust: net/minecraft/client/player/local_player.rs:1522 fn hurt_to(&mut self, new_health: f32)
fn skyblocker_update_health(ci: &mut CallbackInfo<'_>) {
    let Some(this) = compat::detach(ci.this_mut::<mcrust::net::minecraft::client::player::local_player::LocalPlayer>()) else { return; };
    crate::de::hysky::skyblocker::skyblock::rift::healing_melon_indicator::HealingMelonIndicator::update_health();
}

/// @Inject skyblocker$redirectEditSignScreen
///   java: @Inject(method = "openTextEdit", at = @At("HEAD"), cancellable = true)
/// target: net.minecraft.client.player.LocalPlayer#openTextEdit  [exists]
/// Rust: net/minecraft/client/player/local_player.rs:5263 fn open_text_edit_arm(gfx: &mut crate::net::minecraft::client::minecraft::Minecraft, screen: &mut crate::net::minecraft::client::gui::screens::screen::Screen, screen_extra: &mut crate::net::minecraft::client::gui::gui::ScreenExtra, screen_kind_before_pause: &mut Option<crate::net::minecraft::client:
///
/// The port's `openTextEdit` is the client loop's arm `open_text_edit_arm`, which runs every frame and opens the
/// editor the packet drain parked in `Minecraft.pending_sign_editor` (argument 0, `gfx`): a call of Java's
/// `openTextEdit(sign, front)` is a run of the arm that finds one parked. `sign` / `front` are that editor's
/// `sign` / `is_front_text`; the open screen is the arm's `screen` (argument 1) when `screen_is_open` (argument 6).
/// `ci.cancel()` is the call not happening, so the parked editor is consumed here exactly as the arm would.
fn skyblocker_redirect_edit_sign_screen(ci: &mut CallbackInfo<'_>) {
    let Some(gfx) = compat::detach(ci.arg_mut::<mcrust::net::minecraft::client::minecraft::Minecraft>(0)) else { return; };
    let Some(pending) = gfx.pending_sign_editor.as_ref() else { return; };
    let sign = pending.sign.clone();
    let front: bool = pending.is_front_text;
    // `sign.getText(front).getMessage(3, false).getString()`
    let message_3: String = pending.sign_text.get_message(3, false).get_string();
    let screen_open: bool = ci.arg::<bool>(6).copied().unwrap_or(false);
    let screen_title: Option<String> = if screen_open { ci.arg::<mcrust::net::minecraft::client::gui::screens::screen::Screen>(1).map(|screen| screen.get_title().to_string()) } else { None };
    let cancel = |ci: &mut CallbackInfo<'_>, gfx: &mut mcrust::net::minecraft::client::minecraft::Minecraft| {
        gfx.pending_sign_editor = None;
        ci.cancel();
    };
    // Fancy Party Finder
    if !compat::get_state(&crate::de::hysky::skyblocker::skyblock::dungeon::partyfinder::party_finder_screen::IS_IN_KUUDRA_PARTY_FINDER)
        && let Some(party_finder_screen) = mcrust::mcrust::modding::game::current_mod_screen::<compat::ScreenAdapter<crate::de::hysky::skyblocker::skyblock::dungeon::partyfinder::party_finder_screen::PartyFinderScreen>, _>(|__s| __s.object().clone())
        && !crate::de::hysky::skyblocker::skyblock::dungeon::partyfinder::party_finder_screen::PartyFinderScreen::is_aborted(&party_finder_screen)
        && message_3.to_lowercase().contains("level")
    {
        crate::de::hysky::skyblocker::skyblock::dungeon::partyfinder::party_finder_screen::PartyFinderScreen::update_sign(&party_finder_screen, sign.clone(), front);
        cancel(ci, gfx);
        return;
    }
    if let Some(auction_view_screen) = mcrust::mcrust::modding::container_screens::current_mod_container_screen::<compat::ContainerScreenAdapter<crate::de::hysky::skyblocker::skyblock::auction::auction_view_screen::AuctionViewScreen>, _>(|__s| __s.object().clone()) {
        // TODO(batchD): cross-package - `EditBidPopup::new` takes a `SignBlockEntity` BY VALUE, where the sign is the
        // level's shared block entity (`Arc<dyn BlockEntityObj>`, as `PartyFinderScreen::update_sign` takes it); the
        // popup is not opened and the vanilla sign editor is left to open.
        //   java: this.minecraft.gui.setScreen(new EditBidPopup(auctionViewScreen, sign, front, auctionViewScreen.minBid));
        //   java: ci.cancel();
        let _ = auction_view_screen;
    }
    // Search Overlay
    if let Some(title) = screen_title {
        let config = &crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().ui_and_visuals.search_overlay;
        let mut is_input_sign: bool = compat::equals_ignore_case(&message_3, "enter query");
        if !is_input_sign {
            return;
        }
        let location = if config.enable_auction_house && title.contains("Auction") || config.enable_ironman_auction_house && title.contains("Cosmetics") {
            Some(crate::de::hysky::skyblocker::skyblock::searchoverlay::search_over_manager::SearchLocation::Auction)
        } else if config.enable_bazaar && title.contains("Bazaar") {
            Some(crate::de::hysky::skyblocker::skyblock::searchoverlay::search_over_manager::SearchLocation::Bazaar)
        } else if config.enable_museum && title.contains("Museum") {
            Some(crate::de::hysky::skyblocker::skyblock::searchoverlay::search_over_manager::SearchLocation::Museum)
        } else {
            None
        };
        if let Some(location) = location {
            //   java: SearchOverManager.updateSign(sign, front, location);
            crate::de::hysky::skyblocker::skyblock::searchoverlay::search_over_manager::SearchOverManager::update_sign(sign.clone(), front, location);
            //   java: minecraft.gui.setScreen(new OverlayScreen());
            mcrust::mcrust::modding::game::set_screen(Box::new(compat::ScreenAdapter::new(crate::de::hysky::skyblocker::skyblock::searchoverlay::overlay_screen::OverlayScreen::new())));
            //   java: ci.cancel();
            cancel(ci, gfx);
        }
    }
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    report(hook::register_injector("net.minecraft.client.player.LocalPlayer#drop", InjectOptions::at(At::Head).kind(InjectorKind::Inject).priority(1000), Arc::new(skyblocker_drop_selected_item)));
    report(hook::register_injector("net.minecraft.client.player.LocalPlayer#hurtTo", InjectOptions::at(At::Return).kind(InjectorKind::Inject).priority(1000), Arc::new(skyblocker_update_health)));
    report(hook::register_injector("net.minecraft.client.player.LocalPlayer#openTextEdit", InjectOptions::at(At::Head).kind(InjectorKind::Inject).priority(1000), Arc::new(skyblocker_redirect_edit_sign_screen)));
}

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
    if crate::de::hysky::skyblocker::utils::utils::Utils::is_on_skyblock() && crate::de::hysky::skyblocker::skyblock::item::item_protection::ItemProtection::is_item_protected(Some(<_ as mcrust::net::minecraft::world::entity::living_entity::LivingEntity>::get_main_hand_item(&this).clone())) || crate::de::hysky::skyblocker::skyblock::item::hotbar_slot_lock::HotbarSlotLock::is_locked(this.base.base.get_inventory().get_selected_slot()) {
        let mut should_drop_in_dungeons: bool = (crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().dungeons.allow_dropping_protected_items && crate::de::hysky::skyblocker::skyblock::dungeon::dungeon_score::DungeonScore::is_dungeon_started());
        let mut should_drop_shining_coins: bool = (crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().hunting.haunted_biome.ignore_slot_locking_for_shining_coins && crate::de::hysky::skyblocker::skyblock::hunting::safari::safari_utils::SafariUtils::is_in_haunted_biome() && (<_ as mcrust::net::minecraft::world::entity::living_entity::LivingEntity>::get_main_hand_item(&this)).get_skyblock_id() == "SHINING_COIN");
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
fn skyblocker_redirect_edit_sign_screen(ci: &mut CallbackInfo<'_>) {
    // TODO(fabric2seer): `sign`: this argument is not visible to the injector (see the notes above the handler)
    //   java: if (!PartyFinderScreen.isInKuudraPartyFinder && minecraft.gui.screen() instanceof PartyFinderScreen partyFinderScreen && !partyFinderScreen.isAborted() && sign.getText(front).getMessage(3, false).getString().toLowerCase(Locale.ENGLISH).contains("level")) {
    //   java: 			partyFinderScreen.updateSign(sign, front);
    //   java: 			ci.cancel();
    //   java: 			return;
    //   java: 		}
    if let Some(auction_view_screen) = mcrust::mcrust::modding::container_screens::current_mod_container_screen::<compat::ContainerScreenAdapter<crate::de::hysky::skyblocker::skyblock::auction::auction_view_screen::AuctionViewScreen>, _>(|__s| __s.object().clone()) {
        // TODO(fabric2seer): `this` (the hooked object) is not available in this callback
        //   java: this.minecraft.gui.setScreen(new EditBidPopup(auctionViewScreen, sign, front, auctionViewScreen.minBid));
        ci.cancel();
    }
    // TODO(fabric2seer): the condition: `this` (the hooked object) is not available in this callback
    //   java: minecraft.gui.screen() != null
    if todo!("fabric2seer: the condition (TODO above)") {
        let config = &crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().ui_and_visuals.search_overlay;
        // TODO(fabric2seer): `sign`: this argument is not visible to the injector (see the notes above the handler)
        //   java: boolean isInputSign = sign.getText(front).getMessage(3, false).getString().equalsIgnoreCase("enter query");
        let mut is_input_sign: bool = todo!("fabric2seer: the initialiser of `isInputSign` (TODO above)");
        if !is_input_sign {
            return;
        }
        // TODO(fabric2seer): `this` (the hooked object) is not available in this callback
        //   java: String title = minecraft.gui.screen().getTitle().getString();
        let mut title: String = todo!("fabric2seer: the initialiser of `title` (TODO above)");
        if config.enable_auction_house && title.contains("Auction") || config.enable_ironman_auction_house && title.contains("Cosmetics") {
            // TODO(fabric2seer): `sign`: this argument is not visible to the injector (see the notes above the handler)
            //   java: SearchOverManager.updateSign(sign, front, SearchOverManager.SearchLocation.AUCTION);
            // TODO(fabric2seer): `this` (the hooked object) is not available in this callback
            //   java: minecraft.gui.setScreen(new OverlayScreen());
            ci.cancel();
        } else if config.enable_bazaar && title.contains("Bazaar") {
            // TODO(fabric2seer): `sign`: this argument is not visible to the injector (see the notes above the handler)
            //   java: SearchOverManager.updateSign(sign, front, SearchOverManager.SearchLocation.BAZAAR);
            // TODO(fabric2seer): `this` (the hooked object) is not available in this callback
            //   java: minecraft.gui.setScreen(new OverlayScreen());
            ci.cancel();
        } else if config.enable_museum && title.contains("Museum") {
            // TODO(fabric2seer): `sign`: this argument is not visible to the injector (see the notes above the handler)
            //   java: SearchOverManager.updateSign(sign, front, SearchOverManager.SearchLocation.MUSEUM);
            // TODO(fabric2seer): `this` (the hooked object) is not available in this callback
            //   java: minecraft.gui.setScreen(new OverlayScreen());
            ci.cancel();
        }
    }
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    report(hook::register_injector("net.minecraft.client.player.LocalPlayer#drop", InjectOptions::at(At::Head).kind(InjectorKind::Inject).priority(1000), Arc::new(skyblocker_drop_selected_item)));
    report(hook::register_injector("net.minecraft.client.player.LocalPlayer#hurtTo", InjectOptions::at(At::Return).kind(InjectorKind::Inject).priority(1000), Arc::new(skyblocker_update_health)));
    report(hook::register_injector("net.minecraft.client.player.LocalPlayer#openTextEdit", InjectOptions::at(At::Head).kind(InjectorKind::Inject).priority(1000), Arc::new(skyblocker_redirect_edit_sign_screen)));
}

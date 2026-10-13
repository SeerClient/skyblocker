//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.MenuScreensConstructorMixin by fabric2seer.
//! @Mixin(net.minecraft.client.gui.screens.MenuScreens$ScreenConstructor)

#[allow(unused_imports)]
use crate::prelude::*;

/// @Inject skyblocker$open on `MenuScreens.ScreenConstructor.fromPacket` -> `container_screens::register` (the factory the port asks when the server opens a menu)
///   java: @Inject(method = "fromPacket", at = @At("HEAD"), cancellable = true)
fn skyblocker_open(args: &mcrust::mcrust::modding::container_screens::ContainerScreenArgs, menu: &mut mcrust::net::minecraft::world::entity::player::player::ContainerMenu) -> Option<mcrust::mcrust::modding::container_screens::ModContainer> {
    let mut __result = compat::ContainerResult::default();
    if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() {
        return None;
    }
    if !(crate::de::hysky::skyblocker::utils::utils::Utils::is_on_skyblock()) {
        return None;
    }
    let mut name_lowercase: String = (compat::trim(&(<_ as mcrust::net::minecraft::network::chat::component::Component>::get_string(&mcrust::net::minecraft::network::chat::component::literal(&args.title))))).to_lowercase();
    if let mcrust::net::minecraft::world::entity::player::player::ContainerMenu::Chest(_) = &mut *menu && crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().dungeons.fancy_party_finder && name_lowercase.starts_with("select tier") {
        compat::set_state(&crate::de::hysky::skyblocker::skyblock::dungeon::partyfinder::party_finder_screen::IS_IN_KUUDRA_PARTY_FINDER, true);
    } else if let mcrust::net::minecraft::world::entity::player::player::ContainerMenu::Chest(_) = &mut *menu && crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().dungeons.fancy_party_finder && name_lowercase.starts_with("catacombs") {
        compat::set_state(&crate::de::hysky::skyblocker::skyblock::dungeon::partyfinder::party_finder_screen::IS_IN_KUUDRA_PARTY_FINDER, false);
    } else if let mcrust::net::minecraft::world::entity::player::player::ContainerMenu::Chest(container_screen_handler) = &mut *menu && crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().dungeons.fancy_party_finder && crate::de::hysky::skyblocker::skyblock::dungeon::partyfinder::party_finder_screen::POSSIBLE_INVENTORY_NAMES.iter().any(|e| *e == name_lowercase.clone()) {
        if compat::minecraft().gui.screen().is_some() {
            let mut lower_case: String = compat::minecraft().gui.screen().unwrap().get_title().to_string().to_lowercase();
            if lower_case.contains("group builder") || lower_case == "training dummy" {
                return None;
            }
        }
        if compat::get_state(&crate::de::hysky::skyblocker::skyblock::dungeon::partyfinder::party_finder_screen::IS_IN_KUUDRA_PARTY_FINDER) {
            return None;
        }
        // java: player.containerMenu = the server's menu - it already is
        if let Some(__sel4) = compat::minecraft().gui.screen().as_ref() && let Some(screen) = mcrust::mcrust::modding::game::current_mod_screen::<compat::ScreenAdapter<crate::de::hysky::skyblocker::skyblock::dungeon::partyfinder::party_finder_screen::PartyFinderScreen>, _>(|__s| __s.object().clone()) {
            crate::de::hysky::skyblocker::skyblock::dungeon::partyfinder::party_finder_screen::PartyFinderScreen::update_handler(&screen, compat::LiveChestMenu::of(&*container_screen_handler), mcrust::net::minecraft::network::chat::component::literal(&args.title).clone());
        } else {
            //   java: client.gui.setScreen(new PartyFinderScreen(containerScreenHandler, player.getInventory(), title));
            __result.mod_screen(Box::new(compat::ScreenAdapter::new(crate::de::hysky::skyblocker::skyblock::dungeon::partyfinder::party_finder_screen::PartyFinderScreen::new(compat::LiveChestMenu::of(&*container_screen_handler), compat::PlayerInventory, mcrust::net::minecraft::network::chat::component::literal(&args.title).clone()))));
        }
        return __result.finish();
    } else if let mcrust::net::minecraft::world::entity::player::player::ContainerMenu::Chest(container_screen_handler) = &mut *menu && crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().ui_and_visuals.fancy_auction_house.enabled && (name_lowercase == "auctions browser" || name_lowercase.starts_with("auctions: ")) {
        let mut auction_house_screen_handler: compat::Shared<crate::de::hysky::skyblocker::skyblock::auction::auction_house_screen_handler::AuctionHouseScreenHandler> = crate::de::hysky::skyblocker::skyblock::auction::auction_house_screen_handler::AuctionHouseScreenHandler::of(container_screen_handler, false);
        __result.menu(compat::mod_menu(auction_house_screen_handler.clone()));
        if let Some(__sel7) = compat::minecraft().gui.screen().as_ref() && let Some(auction_browser_screen) = mcrust::mcrust::modding::container_screens::current_mod_container_screen::<compat::ContainerScreenAdapter<crate::de::hysky::skyblocker::skyblock::auction::auction_browser_screen::AuctionBrowserScreen>, _>(|__s| __s.object().clone()) {
            auction_browser_screen.lock().borrow_mut().change_handler(auction_house_screen_handler.clone(), container_screen_handler);
        } else {
            __result.container_screen(Box::new(compat::ContainerScreenAdapter::new(crate::de::hysky::skyblocker::skyblock::auction::auction_browser_screen::AuctionBrowserScreen::new_listening(auction_house_screen_handler.clone(), container_screen_handler))));
        }
        return __result.finish();
    } else if let mcrust::net::minecraft::world::entity::player::player::ContainerMenu::Chest(container_screen_handler) = &mut *menu && crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().ui_and_visuals.fancy_auction_house.enabled && name_lowercase.ends_with("auction view") {
        let mut auction_house_screen_handler: compat::Shared<crate::de::hysky::skyblocker::skyblock::auction::auction_house_screen_handler::AuctionHouseScreenHandler> = crate::de::hysky::skyblocker::skyblock::auction::auction_house_screen_handler::AuctionHouseScreenHandler::of(container_screen_handler, true);
        __result.menu(compat::mod_menu(auction_house_screen_handler.clone()));
        if let Some(__sel10) = compat::minecraft().gui.screen().as_ref() && let Some(auction_view_screen) = mcrust::mcrust::modding::container_screens::current_mod_container_screen::<compat::ContainerScreenAdapter<crate::de::hysky::skyblocker::skyblock::auction::auction_view_screen::AuctionViewScreen>, _>(|__s| __s.object().clone()) {
            auction_view_screen.lock().borrow_mut().change_handler(auction_house_screen_handler.clone(), container_screen_handler);
        } else {
            __result.container_screen(Box::new(compat::ContainerScreenAdapter::new(crate::de::hysky::skyblocker::skyblock::auction::auction_view_screen::AuctionViewScreen::new_listening(auction_house_screen_handler.clone(), container_screen_handler, mcrust::net::minecraft::network::chat::component::literal(&args.title).clone()))));
        }
        return __result.finish();
    } else if let mcrust::net::minecraft::world::entity::player::player::ContainerMenu::Chest(container_screen_handler) = &mut *menu && crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().ui_and_visuals.fancy_auction_house.enabled && (name_lowercase == "confirm purchase" || name_lowercase == "confirm bid") && let Some(auction_view_screen) = mcrust::mcrust::modding::container_screens::current_mod_container_screen::<compat::ContainerScreenAdapter<crate::de::hysky::skyblocker::skyblock::auction::auction_view_screen::AuctionViewScreen>, _>(|__s| __s.object().clone()) {
        //   java: client.gui.setScreen(auctionViewScreen.getConfirmPurchasePopup(name)); - shown instead of the vanilla screen
        let _ = container_screen_handler;
        __result.mod_screen(Box::new(crate::de::hysky::skyblocker::skyblock::auction::auction_view_screen::AuctionViewScreen::get_confirm_purchase_popup(&auction_view_screen, &mcrust::net::minecraft::network::chat::component::literal(&args.title))));
        // java: player.containerMenu = the server's menu - it already is
        return __result.finish();
    } else if let mcrust::net::minecraft::world::entity::player::player::ContainerMenu::Chest(container_screen_handler) = &mut *menu && crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().ui_and_visuals.fancy_crafting_table && ((<_ as mcrust::net::minecraft::network::chat::component::Component>::get_string(&mcrust::net::minecraft::network::chat::component::literal(&args.title))).to_lowercase()).contains("craft item") {
        let mut skyblock_crafting_table_screen_handler: compat::Shared<crate::de::hysky::skyblocker::skyblock::item::skyblock_crafting_table_screen_handler::SkyblockCraftingTableScreenHandler> = crate::de::hysky::skyblocker::skyblock::item::skyblock_crafting_table_screen_handler::SkyblockCraftingTableScreenHandler::new_2(container_screen_handler, compat::PlayerInventory);
        __result.menu(compat::mod_menu(skyblock_crafting_table_screen_handler.clone()));
        __result.container_screen(Box::new(compat::ContainerScreenAdapter::new(crate::de::hysky::skyblocker::skyblock::item::skyblock_crafting_table_screen::SkyblockCraftingTableScreen::new(skyblock_crafting_table_screen_handler.clone(), compat::PlayerInventory, mcrust::net::minecraft::network::chat::component::literal("Craft Item")))));
        return __result.finish();
    } else if let mcrust::net::minecraft::world::entity::player::player::ContainerMenu::Chest(container_screen_handler) = &mut *menu && compat::get_state(&crate::de::hysky::skyblocker::skyblock::tabhud::config::list::widgets_list_screen::OVERRIDE_WIDGETS_SCREEN) && (compat::lock(&crate::de::hysky::skyblocker::skyblock::tabhud::config::widgets_configuration_screen::SCREEN_TITLE_PATTERN).matcher(&name_lowercase).find() || name_lowercase == "tablist widgets" || name_lowercase.ends_with("widget settings") && !(name_lowercase.starts_with("reset")) || name_lowercase.starts_with("shown") && mcrust::mcrust::modding::game::current_screen_is::<compat::ScreenAdapter<crate::de::hysky::skyblocker::skyblock::tabhud::config::list::widgets_list_screen::WidgetsListScreen>>()) {
        // java: player.containerMenu = the server's menu - it already is
        if let Some(__sel16) = compat::minecraft().gui.screen().as_ref() && let Some(screen) = mcrust::mcrust::modding::game::current_mod_screen::<compat::ScreenAdapter<crate::de::hysky::skyblocker::skyblock::tabhud::config::list::widgets_list_screen::WidgetsListScreen>, _>(|__s| __s.object().clone()) {
            crate::de::hysky::skyblocker::skyblock::tabhud::config::list::widgets_list_screen::WidgetsListScreen::update_handler(&screen, compat::LiveChestMenu::of(&*container_screen_handler), &mcrust::net::minecraft::network::chat::component::literal(&args.title));
        } else {
            __result.mod_screen(Box::new(compat::ScreenAdapter::new(crate::de::hysky::skyblocker::skyblock::tabhud::config::list::widgets_list_screen::WidgetsListScreen::new(compat::LiveChestMenu::of(&*container_screen_handler), mcrust::net::minecraft::network::chat::component::literal(&args.title).clone()))));
        }
        return __result.finish();
    } else if let mcrust::net::minecraft::world::entity::player::player::ContainerMenu::Chest(container_screen_handler) = &mut *menu && crate::de::hysky::skyblocker::utils::utils::Utils::is_in_dungeons() && compat::read(&crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().dungeons.leap_overlay, |__t| __t.enable_leap_overlay) && name_lowercase.contains(&crate::de::hysky::skyblocker::skyblock::dungeon::leap_overlay::TITLE.to_lowercase()) {
        // java: player.containerMenu = the server's menu - it already is
        __result.mod_screen(Box::new(compat::ScreenAdapter::new(crate::de::hysky::skyblocker::skyblock::dungeon::leap_overlay::LeapOverlay::new_listening(container_screen_handler))));
        return __result.finish();
    } else if let mcrust::net::minecraft::world::entity::player::player::ContainerMenu::Chest(container_screen_handler) = &mut *menu && crate::de::hysky::skyblocker::skyblock::radial_menu::radial_menu_manager::RadialMenuManager::is_menu_exists_from_title(&name_lowercase) {
        // java: player.containerMenu = the server's menu - it already is
        let mut menu_type: compat::Shared<dyn crate::de::hysky::skyblocker::skyblock::radial_menu::radial_menu::RadialMenu> = crate::de::hysky::skyblocker::skyblock::radial_menu::radial_menu_manager::RadialMenuManager::get_menu_from_title(&name_lowercase).unwrap();
        __result.mod_screen(Box::new(compat::ScreenAdapter::new(crate::de::hysky::skyblocker::skyblock::radial_menu::radial_menu_screen::RadialMenuScreen::new_listening(container_screen_handler, menu_type.clone(), mcrust::net::minecraft::network::chat::component::literal(&args.title).clone()))));
        return __result.finish();
    } else if let mcrust::net::minecraft::world::entity::player::player::ContainerMenu::Chest(container_screen_handler) = &mut *menu && crate::de::hysky::skyblocker::skyblock::storageoverlay::storage_overlay_screen::StorageOverlayScreen::enabled(&name_lowercase) {
        let mut height: i32 = (compat::minecraft().get_window().get_gui_scaled_height()).wrapping_sub((compat::minecraft().get_window().get_gui_scaled_height()).wrapping_div(5));
        let mut is_backpack: bool = (crate::de::hysky::skyblocker::skyblock::item::tooltip::backpack_preview::BackpackPreview::get_storage_index_from_title(&name_lowercase) != -1);
        let mut storage_overlay_screen_handler: compat::Shared<crate::de::hysky::skyblocker::skyblock::storageoverlay::storage_overlay_screen_handler::StorageOverlayScreenHandler> = crate::de::hysky::skyblocker::skyblock::storageoverlay::storage_overlay_screen_handler::StorageOverlayScreenHandler::new_4(container_screen_handler, is_backpack, height, compat::PlayerInventory);
        __result.menu(compat::mod_menu(storage_overlay_screen_handler.clone()));
        __result.container_screen(Box::new(compat::ContainerScreenAdapter::new(crate::de::hysky::skyblocker::skyblock::storageoverlay::storage_overlay_screen::StorageOverlayScreen::new(storage_overlay_screen_handler.clone(), compat::LiveChestMenu::of(&*container_screen_handler), mcrust::net::minecraft::network::chat::component::literal(&args.title).clone(), compat::PlayerInventory, height))));
        return __result.finish();
    } else {
    }
    let _ = (&mut __result, args, menu);
    None
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    for __menu_type in [mcrust::net::minecraft::client::gui::screens::menu_screens::MenuType::Generic9x1, mcrust::net::minecraft::client::gui::screens::menu_screens::MenuType::Generic9x2, mcrust::net::minecraft::client::gui::screens::menu_screens::MenuType::Generic9x3, mcrust::net::minecraft::client::gui::screens::menu_screens::MenuType::Generic9x4, mcrust::net::minecraft::client::gui::screens::menu_screens::MenuType::Generic9x5, mcrust::net::minecraft::client::gui::screens::menu_screens::MenuType::Generic9x6] {
        mcrust::mcrust::modding::container_screens::register(__menu_type, skyblocker_open);
    }
}

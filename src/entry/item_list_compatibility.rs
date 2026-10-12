//! Ported from de.hysky.skyblocker.compatibility.ItemListCompatibility by fabric2seer. Entrypoint: skyblock-item-list.

#[allow(unused_imports)]
use crate::prelude::*;

pub fn register_exclusion_zones(zones: &crate::unresolved::ExclusionZoneManager) {
    // TODO(fabric2seer): `.addProvider(..)` on a ExclusionZoneManager
    //   java: zones.addProvider(ContainerScreen.class, containerScreen -> {
    //   java: 			if (!SkyblockerConfigManager.get().uiAndVisuals.museumOverlay || !containerScreen.getTitle().getString().contains("Museum")) return List.of();
    //   java: 			AbstractContainerScreenAccessor accessor = (AbstractContainerScreenAccessor) containerScreen;
    //   java: 			return List.of(new Rect2i(accessor.getX() + accessor.getImageWidth() + 4, accessor.getY(), MuseumManager.BACKGROUND_WIDTH, MuseumManager.BACKGROUND_HEIGHT));
    //   java: 		});
    // TODO(fabric2seer): `.addProvider(..)` on a ExclusionZoneManager
    //   java: zones.addProvider(InventoryScreen.class, _ -> {
    //   java: 			if (!SkyblockerConfigManager.get().farming.plotsWidget.enabled || !Utils.isInGarden() || GardenPlots.widget == null) return List.of();
    //   java: 			return List.of(new Rect2i(GardenPlots.widget.getX(), GardenPlots.widget.getY(), GardenPlots.widget.getWidth(), GardenPlots.widget.getHeight()));
    //   java: 		});
    // TODO(fabric2seer): `.addProvider(..)` on a ExclusionZoneManager
    //   java: zones.addProvider(Screen.class, _ -> {
    //   java: 			if (!VisitorHelper.shouldRender()) return List.of();
    //   java: 			return VisitorHelper.getExclusionZones();
    //   java: 		});
    // TODO(fabric2seer): `.addProvider(..)` on a ExclusionZoneManager
    //   java: zones.addProvider(AuctionBrowserScreen.class, screen -> {
    //   java: 			AbstractContainerScreenAccessor accessor = (AbstractContainerScreenAccessor) screen;
    //   java: 			return List.of(new Rect2i(accessor.getX() - 31, accessor.getY() + 2, 32, 28 * 6));
    //   java: 		});
    // TODO(fabric2seer): `.addProvider(..)` on a ExclusionZoneManager
    //   java: zones.addProvider(StorageOverlayScreen.class, screen -> List.of(
    //   java: 				screen.getMainExclusionZone(),
    //   java: 				screen.getButtonsExclusionZone()
    //   java: 		));
}

pub fn register_excluded_screens(manager: &crate::unresolved::ExcludedScreensManager) {
    // TODO(fabric2seer): `.addProvider(..)` on a ExcludedScreensManager
    //   java: manager.addProvider(StorageOverlayScreen.class, _ -> Optional.of("Skyblocker Storage Overlay"));
}

pub fn register_hovered_items(manager: &crate::unresolved::HoveredItemManager) {
    // TODO(fabric2seer): `.addProvider(..)` on a HoveredItemManager
    //   java: manager.addProvider(HoveredItemStackUtils::getHoveredItemStack);
    // TODO(fabric2seer): `.addConsumer(..)` on a HoveredItemManager
    //   java: manager.addConsumer((screen, stack, event) -> {
    //   java: 			LocalPlayer player = Minecraft.getInstance().player;
    //   java: 			if (player == null) return false;
    //   java:
    //   java: 			// Wiki Lookup
    //   java: 			if (WikiLookupManager.handleWikiLookup(screen.getTitle().getString(), Either.right(stack), player, event)) {
    //   java: 				return true;
    //   java: 			}
    //   java:
    //   java: 			// Item Price Lookup
    //   java: 			if (SkyblockerConfigManager.get().helpers.itemPrice.enableItemPriceLookup && ItemPrice.ITEM_PRICE_LOOKUP.matches(event)) {
    //   java: 				ItemPrice.itemPriceLookup(player, stack);
    //   java: 				return true;
    //   java: 			}
    //   java:
    //   java: 			return false;
    //   java: 		});
}

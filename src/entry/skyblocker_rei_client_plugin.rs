//! Ported from de.hysky.skyblocker.compatibility.rei.SkyblockerREIClientPlugin by fabric2seer. Entrypoint: rei_client.

#[allow(unused_imports)]
use crate::prelude::*;

use crate::de::hysky::skyblocker::config::configs::general_config::ItemList;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::itemlist::item_repository::ItemRepository;
use crate::de::hysky::skyblocker::utils::neu_repo_manager::NEURepoManager;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::net::minecraft::client::renderer::rect2i::Rect2i;


pub fn register_categories(category_registry: &mut crate::unresolved::CategoryRegistry) {
    if !(Utils::is_on_skyblock()) {
        return;
    }
    if !(SkyblockerConfigManager::get().general.item_list.enable_item_list) {
        return;
    }
    // TODO(fabric2seer): `.addWorkstations(..)` on a CategoryRegistry
    //   java: categoryRegistry.addWorkstations(CategoryIdentifier.of(SkyblockCraftingRecipe.ID), EntryStacks.of(Items.CRAFTING_TABLE));
    // TODO(fabric2seer): `.addWorkstations(..)` on a CategoryRegistry
    //   java: categoryRegistry.addWorkstations(CategoryIdentifier.of(SkyblockForgeRecipe.ID), EntryStacks.of(Items.ANVIL));
    // TODO(fabric2seer): `.addWorkstations(..)` on a CategoryRegistry
    //   java: categoryRegistry.addWorkstations(CategoryIdentifier.of(SkyblockNpcShopRecipe.ID), EntryStacks.of(Items.GOLD_NUGGET));
    // TODO(fabric2seer): `.addWorkstations(..)` on a CategoryRegistry
    //   java: categoryRegistry.addWorkstations(CategoryIdentifier.of(SkyblockKatUpgradeRecipe.ID), EntryStacks.of(Items.BONE));
    // TODO(fabric2seer): `.add(..)` on a CategoryRegistry
    //   java: categoryRegistry.add(new SkyblockRecipeCategory(SkyblockCraftingRecipe.ID, Component.translatable("emi.category.skyblocker.skyblock_crafting"), ItemUtils.getSkyblockerStack().getStackOrThrow(), 73));
    // TODO(fabric2seer): `.add(..)` on a CategoryRegistry
    //   java: categoryRegistry.add(new SkyblockRecipeCategory(SkyblockForgeRecipe.ID, Component.translatable("emi.category.skyblocker.skyblock_forge"), ItemUtils.getSkyblockerForgeStack().getStackOrThrow(), 84));
    // TODO(fabric2seer): `.add(..)` on a CategoryRegistry
    //   java: categoryRegistry.add(new SkyblockRecipeCategory(SkyblockNpcShopRecipe.ID, Component.translatable("emi.category.skyblocker.skyblock_npc_shop"), Items.GOLD_NUGGET.getDefaultInstance(), 73));
    // TODO(fabric2seer): `.add(..)` on a CategoryRegistry
    //   java: categoryRegistry.add(new SkyblockRecipeCategory(SkyblockKatUpgradeRecipe.ID, Component.translatable("emi.category.skyblocker.skyblock_kat_upgrade"), ItemUtils.getSkyblockerKatStack().getStackOrThrow(), 64));
    // TODO(fabric2seer): `.add(..)` on a CategoryRegistry
    //   java: categoryRegistry.add(new SkyblockInfoCategory());
}

pub fn register_displays(display_registry: &crate::unresolved::DisplayRegistry) {
    if !(Utils::is_on_skyblock()) {
        return;
    }
    if !(SkyblockerConfigManager::get().general.item_list.enable_item_list) {
        return;
    }
    // TODO(fabric2seer): the condition: `.getGlobalDisplayGenerators(..)` on a DisplayRegistry
    //   java: displayRegistry.getGlobalDisplayGenerators(..).stream(..).noneMatch(..)
    if todo!("fabric2seer: the condition (TODO above)") {
        // TODO(fabric2seer): `.registerGlobalDisplayGenerator(..)` on a DisplayRegistry
        //   java: displayRegistry.registerGlobalDisplayGenerator(new SkyblockRecipeDisplayGenerator());
    }
    // TODO(fabric2seer): the condition: `.getGlobalDisplayGenerators(..)` on a DisplayRegistry
    //   java: displayRegistry.getGlobalDisplayGenerators(..).stream(..).noneMatch(..)
    if todo!("fabric2seer: the condition (TODO above)") {
        // TODO(fabric2seer): `.registerGlobalDisplayGenerator(..)` on a DisplayRegistry
        //   java: displayRegistry.registerGlobalDisplayGenerator(new SkyblockInfoDisplayGenerator());
    }
}

pub fn register_screens(registry: &crate::unresolved::ScreenRegistry) {
    if !(Utils::is_on_skyblock()) {
        return;
    }
    // TODO(fabric2seer): `.registerFocusedStack(..)` on a ScreenRegistry
    //   java: registry.registerFocusedStack(new SkyblockerFocusedStackProvider());
}

pub fn register_entries(entry_registry: &crate::unresolved::EntryRegistry) {
    if !(Utils::is_on_skyblock()) {
        return;
    }
    if !(SkyblockerConfigManager::get().general.item_list.enable_item_list) {
        return;
    }
    // TODO(fabric2seer): `.removeEntryIf(..)` on a EntryRegistry
    //   java: entryRegistry.removeEntryIf(_ -> true);
    // TODO(fabric2seer): `.addEntries(..)` on a EntryRegistry
    //   java: entryRegistry.addEntries(ItemRepository.getItemsStream().map(FlexibleItemStack::getStackOrThrow).map(EntryStacks::of).toList());
}

pub fn register_collapsible_entries(registry: &crate::unresolved::CollapsibleEntryRegistry) {
    if !(Utils::is_on_skyblock()) {
        return;
    }
    let config = &SkyblockerConfigManager::get().general.item_list;
    if !config.enable_item_list || !config.enable_collapsible_entries {
        return;
    }
    if !(ItemRepository::files_imported()) || NEURepoManager::is_loading() {
        return;
    }
    // TODO(fabric2seer): `.getParents(..)` on a NEUConstants
    //   java: NEURepoManager.getConstants().getParents().getParents().forEach((parentId, childrenList) -> {
    //   java: 			Optional<ItemStack> parentItem = ItemRepository.getItemsStream().map(FlexibleItemStack::getStackOrThrow).filter(itemStack -> itemStack.getNeuName().equals(parentId)).findFirst();
    //   java: 			if (parentItem.isEmpty()) return;
    //   java:
    //   java: 			List<EntryStack<ItemStack>> allItems = Stream.concat(parentItem.stream(), ItemRepository.getItemsStream().map(FlexibleItemStack::getStackOrThrow).filter(itemStack -> childrenList.contains(itemStack.getNeuName())))
    //   java: 					.map(EntryStacks::of)
    //   java: 					.toList();
    //   java:
    //   java: 			String categoryPath = parentId.toLowerCase(Locale.ENGLISH).replace(";", "_");
    //   java: 			// Drop the tier at the end of the id so the category identifier remains the same even if the parent is changed to a different tier
    //   java: 			if (parentId.contains(";")) {
    //   java: 				categoryPath = categoryPath.substring(0, categoryPath.lastIndexOf("_"));
    //   java: 			}
    //   java:
    //   java: 			// For Enchanted Books, change the name of the category to the enchant name
    //   java: 			Component name;
    //   java: 			if (parentItem.get().is(Items.ENCHANTED_BOOK)) {
    //   java: 				String enchantName = EnchantedBookUtils.getEnchantNameFromLore(parentItem.get().skyblocker$getLoreStrings());
    //   java: 				// drop level
    //   java: 				int levelSeparator = enchantName.lastIndexOf(' ');
    //   java: 				enchantName = levelSeparator == -1 ? enchantName : enchantName.substring(0, levelSeparator);
    //   java: 				name = Component.literal(enchantName).withStyle(parentId.startsWith("ULTIMATE") ? ChatFormatting.LIGHT_PURPLE : ChatFormatting.BLUE);
    //   java: 			} else {
    //   java: 				name = parentItem.get().getHoverName();
    //   java: 			}
    //   java:
    //   java: 			registry.group(SkyblockerMod.id("rei_category/" + categoryPath), name, allItems);
    //   java: 		});
}

pub fn register_exclusion_zones(zones: &crate::unresolved::ExclusionZones) {
    if !(Utils::is_on_skyblock()) {
        return;
    }
    // TODO(fabric2seer): `.register(..)` on a ExclusionZones
    //   java: zones.register(ContainerScreen.class, containerScreen -> {
    //   java: 			if (!SkyblockerConfigManager.get().uiAndVisuals.museumOverlay || !containerScreen.getTitle().getString().contains("Museum")) return List.of();
    //   java: 			AbstractContainerScreenAccessor accessor = (AbstractContainerScreenAccessor) containerScreen;
    //   java: 			return List.of(new Rectangle(accessor.getX() + accessor.getImageWidth() + 4, accessor.getY(), MuseumManager.BACKGROUND_WIDTH, MuseumManager.BACKGROUND_HEIGHT));
    //   java: 		});
    // TODO(fabric2seer): `.register(..)` on a ExclusionZones
    //   java: zones.register(InventoryScreen.class, _ -> {
    //   java: 			if (!SkyblockerConfigManager.get().farming.plotsWidget.enabled || !Utils.isInGarden() || GardenPlots.widget == null) return List.of();
    //   java: 			return List.of(new Rectangle(GardenPlots.widget.getX(), GardenPlots.widget.getY(), GardenPlots.widget.getWidth(), GardenPlots.widget.getHeight()));
    //   java: 		});
    // TODO(fabric2seer): `.register(..)` on a ExclusionZones
    //   java: zones.register(Screen.class, _ -> {
    //   java: 			if (!VisitorHelper.shouldRender()) return List.of();
    //   java: 			return VisitorHelper.getExclusionZones().stream()
    //   java: 					.map(this::rectangleFromRectangle)
    //   java: 					.toList();
    //   java: 		});
    // TODO(fabric2seer): `.register(..)` on a ExclusionZones
    //   java: zones.register(StorageOverlayScreen.class, screen -> List.of(
    //   java: 				rectangleFromRectangle(screen.getMainExclusionZone()),
    //   java: 				rectangleFromRectangle(screen.getButtonsExclusionZone())
    //   java: 		));
}

pub fn rectangle_from_rectangle(rect2i: &Rect2i) -> crate::unresolved::Rectangle {
    // TODO(fabric2seer): `new Rectangle(..)`
    //   java: return new Rectangle(rect2i.getX(), rect2i.getY(), rect2i.getWidth(), rect2i.getHeight());
    todo!("fabric2seer: the value of this function is in a TODO above")
}

pub fn register_transfer_handlers(registry: &crate::unresolved::TransferHandlerRegistry) {
    if !(Utils::is_on_skyblock()) {
        return;
    }
    if !(SkyblockerConfigManager::get().general.item_list.enable_item_list) {
        return;
    }
    // TODO(fabric2seer): `.register(..)` on a TransferHandlerRegistry
    //   java: registry.register(new SkyblockTransferHandler());
}

pub fn get_priority() -> f64 {
    return ((-50) as f64) as f64;
}

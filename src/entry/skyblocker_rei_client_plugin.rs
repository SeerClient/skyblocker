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
    //   java: if (displayRegistry.getGlobalDisplayGenerators().stream().noneMatch(generator -> generator instanceof SkyblockRecipeDisplayGenerator))
    if !rei_get_global_display_generators(display_registry).iter().any(|generator| generator.is::<crate::de::hysky::skyblocker::compatibility::rei::recipe::skyblock_recipe_display_generator::SkyblockRecipeDisplayGenerator>()) {
        //   java: displayRegistry.registerGlobalDisplayGenerator(new SkyblockRecipeDisplayGenerator());
        rei_register_global_display_generator(display_registry, Box::new(crate::de::hysky::skyblocker::compatibility::rei::recipe::skyblock_recipe_display_generator::SkyblockRecipeDisplayGenerator::new()));
    }
    //   java: if (displayRegistry.getGlobalDisplayGenerators().stream().noneMatch(generator -> generator instanceof SkyblockInfoDisplayGenerator))
    if !rei_get_global_display_generators(display_registry).iter().any(|generator| generator.is::<crate::de::hysky::skyblocker::compatibility::rei::info::skyblock_info_display_generator::SkyblockInfoDisplayGenerator>()) {
        //   java: displayRegistry.registerGlobalDisplayGenerator(new SkyblockInfoDisplayGenerator());
        rei_register_global_display_generator(display_registry, Box::new(crate::de::hysky::skyblocker::compatibility::rei::info::skyblock_info_display_generator::SkyblockInfoDisplayGenerator::new()));
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
    //   java: return new Rectangle(rect2i.getX(), rect2i.getY(), rect2i.getWidth(), rect2i.getHeight());
    return rei_rectangle(rect2i.get_x(), rect2i.get_y(), rect2i.get_width(), rect2i.get_height());
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

// REI (`me.shedaniel.rei`) is a separate mod that SeerClient never loads, so REI never calls this plugin. Its API types
// are the `crate::unresolved` names; the functions below are REI's side of the boundary (as in
// `compatibility/rei/skyblock_transfer_handler.rs`).

/// `displayRegistry.getGlobalDisplayGenerators()` - the generators registered so far (none: nothing registers one).
fn rei_get_global_display_generators(display_registry: &crate::unresolved::DisplayRegistry) -> Vec<Box<dyn std::any::Any>> {
    let _ = display_registry;
    Vec::new()
}

/// `displayRegistry.registerGlobalDisplayGenerator(generator)`.
fn rei_register_global_display_generator(display_registry: &crate::unresolved::DisplayRegistry, generator: Box<dyn std::any::Any>) {
    let _ = (display_registry, generator);
}

/// `new me.shedaniel.math.Rectangle(x, y, width, height)`.
fn rei_rectangle(x: i32, y: i32, width: i32, height: i32) -> crate::unresolved::Rectangle {
    let _ = (x, y, width, height);
    crate::unresolved::Rectangle
}

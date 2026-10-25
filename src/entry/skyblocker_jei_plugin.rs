//! Ported from de.hysky.skyblocker.compatibility.jei.SkyblockerJEIPlugin by fabric2seer. Entrypoint: jei_mod_plugin.

#[allow(unused_imports)]
use crate::prelude::*;

use crate::de::hysky::skyblocker::compatibility::jei::skyblock_crafting_recipe_category::SkyblockCraftingRecipeCategory;
use crate::de::hysky::skyblocker::compatibility::jei::skyblock_forge_recipe_category::SkyblockForgeRecipeCategory;
use crate::de::hysky::skyblocker::compatibility::jei::skyblock_kat_upgrade_recipe_category::SkyblockKatUpgradeRecipeCategory;
use crate::de::hysky::skyblocker::compatibility::jei::skyblock_npc_shop_recipe_category::SkyblockNpcShopRecipeCategory;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::garden::garden_plots;
use crate::de::hysky::skyblocker::skyblock::garden::visitor::visitor_helper::VisitorHelper;
use crate::de::hysky::skyblocker::skyblock::itemlist::item_repository::ItemRepository;
use crate::de::hysky::skyblocker::skyblock::storageoverlay::storage_overlay_screen::StorageOverlayScreen;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use crate::de::hysky::skyblocker::utils::datafixer::item_stack_componentization_fixer::ItemStackComponentizationFixer;
use mcrust::net::minecraft::client::gui::screens::inventory::container_screen::ContainerScreen;
use mcrust::net::minecraft::client::gui::screens::inventory::inventory_screen::InventoryScreen;
use mcrust::net::minecraft::client::renderer::rect2i::Rect2i;
use mcrust::net::minecraft::resources::identifier::Identifier;

/// java: private SkyblockCraftingRecipeCategory skyblockCraftingRecipeCategory;
pub static SKYBLOCK_CRAFTING_RECIPE_CATEGORY: compat::MainThread<Option<SkyblockCraftingRecipeCategory>> = compat::MainThread::new(|| None);
/// java: private SkyblockForgeRecipeCategory skyblockForgeRecipeCategory;
pub static SKYBLOCK_FORGE_RECIPE_CATEGORY: compat::MainThread<Option<SkyblockForgeRecipeCategory>> = compat::MainThread::new(|| None);
/// java: private SkyblockNpcShopRecipeCategory skyblockNpcShopRecipeCategory;
pub static SKYBLOCK_NPC_SHOP_RECIPE_CATEGORY: compat::MainThread<Option<SkyblockNpcShopRecipeCategory>> = compat::MainThread::new(|| None);
/// java: private SkyblockKatUpgradeRecipeCategory skyblockKatUpgradeRecipe;
pub static SKYBLOCK_KAT_UPGRADE_RECIPE: compat::MainThread<Option<SkyblockKatUpgradeRecipeCategory>> = compat::MainThread::new(|| None);

pub fn trick_jei_into_loading_recipes() {
    // TODO(fabric2seer): static field JeiLifecycleEvents.AFTER_RECIPES_UPDATED
    //   java: JeiLifecycleEvents.AFTER_RECIPES_UPDATED.invoker().run();
}

pub fn get_plugin_uid() -> Identifier {
    return crate::entry::skyblocker_mod::id("skyblock");
}

pub fn register_item_subtypes(registration: &crate::unresolved::ISubtypeRegistration) {
    //   java: @SuppressWarnings("unused")
    //   java: 		SubtypeInterpreters interpreters = ((SubtypeRegistration) registration).getInterpreters();
    let mut interpreters: crate::unresolved::SubtypeInterpreters = jei_get_interpreters(registration);
    //   java: ItemRepository.getItemsStream().filter(stack -> !interpreters.contains(VanillaTypes.ITEM_STACK, stack.getStackOrThrow())).map(FlexibleItemStack::getStackOrThrow).distinct().forEach(item ->
    //   java: 				registration.registerSubtypeInterpreter(item.getItem(), (stack, _) -> ItemStackComponentizationFixer.componentsAsString(stack))
    //   java: 		);
    // (`distinct()` is `ItemStack`'s identity `equals`; `getStackOrThrow` answers each `FlexibleItemStack`'s one cached
    // stack, so two elements are the same stack exactly when they are the same `FlexibleItemStack`)
    let mut __seen: Vec<compat::Shared<FlexibleItemStack>> = Vec::new();
    for stack in ItemRepository::get_items_stream() {
        if jei_interpreters_contains(&interpreters, &FlexibleItemStack::get_stack_or_throw(&stack)) {
            continue;
        }
        if __seen.iter().any(|s| std::sync::Arc::ptr_eq(s, &stack)) {
            continue;
        }
        __seen.push(stack.clone());
        let item = FlexibleItemStack::get_stack_or_throw(&stack);
        jei_register_subtype_interpreter(registration, item.get_item(), Arc::new(|stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack| {
            let mut __stack = stack.clone();
            ItemStackComponentizationFixer::components_as_string(&mut __stack)
        }));
    }
}

// JEI (`mezz.jei`) is a separate mod that SeerClient never loads (`JEICompatibility.JEI_LOADED` is false), so JEI never
// calls this plugin. Its API types are the `crate::unresolved` names; the functions below are JEI's side of the
// boundary (see `compatibility/jei/abstract_skyblock_recipe_category.rs`).

/// `((SubtypeRegistration) registration).getInterpreters()`.
fn jei_get_interpreters(registration: &crate::unresolved::ISubtypeRegistration) -> crate::unresolved::SubtypeInterpreters {
    let _ = registration;
    crate::unresolved::SubtypeInterpreters
}

/// `interpreters.contains(VanillaTypes.ITEM_STACK, stack)` - no interpreter is registered by anyone.
fn jei_interpreters_contains(interpreters: &crate::unresolved::SubtypeInterpreters, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> bool {
    let _ = (interpreters, stack);
    false
}

/// `registration.registerSubtypeInterpreter(item, interpreter)`.
fn jei_register_subtype_interpreter(registration: &crate::unresolved::ISubtypeRegistration, item: Arc<mcrust::net::minecraft::world::item::item::Item>, interpreter: Arc<dyn Fn(&mcrust::net::minecraft::world::item::item_stack::ItemStack) -> String>) {
    let _ = (registration, item, interpreter);
}

pub fn register_categories(registration: &crate::unresolved::IRecipeCategoryRegistration) {
    // TODO(fabric2seer): `.getJeiHelpers(..)` on a IRecipeCategoryRegistration
    //   java: this.skyblockCraftingRecipeCategory = new SkyblockCraftingRecipeCategory(registration.getJeiHelpers().getGuiHelper());
    // TODO(fabric2seer): `.addRecipeCategories(..)` on a IRecipeCategoryRegistration
    //   java: registration.addRecipeCategories(this.skyblockCraftingRecipeCategory);
    // TODO(fabric2seer): `.getJeiHelpers(..)` on a IRecipeCategoryRegistration
    //   java: this.skyblockForgeRecipeCategory = new SkyblockForgeRecipeCategory(registration.getJeiHelpers().getGuiHelper());
    // TODO(fabric2seer): `.addRecipeCategories(..)` on a IRecipeCategoryRegistration
    //   java: registration.addRecipeCategories(this.skyblockForgeRecipeCategory);
    // TODO(fabric2seer): `.getJeiHelpers(..)` on a IRecipeCategoryRegistration
    //   java: this.skyblockNpcShopRecipeCategory = new SkyblockNpcShopRecipeCategory(registration.getJeiHelpers().getGuiHelper());
    // TODO(fabric2seer): `.addRecipeCategories(..)` on a IRecipeCategoryRegistration
    //   java: registration.addRecipeCategories(this.skyblockNpcShopRecipeCategory);
    // TODO(fabric2seer): `.getJeiHelpers(..)` on a IRecipeCategoryRegistration
    //   java: this.skyblockKatUpgradeRecipe = new SkyblockKatUpgradeRecipeCategory(registration.getJeiHelpers().getGuiHelper());
    // TODO(fabric2seer): `.addRecipeCategories(..)` on a IRecipeCategoryRegistration
    //   java: registration.addRecipeCategories(this.skyblockKatUpgradeRecipe);
}

pub fn register_gui_handlers(registration: &crate::unresolved::IGuiHandlerRegistration) {
    // TODO(fabric2seer): `.addGuiContainerHandler(..)` on a IGuiHandlerRegistration
    //   java: registration.addGuiContainerHandler(ContainerScreen.class, new GenericContainerHandler());
    // TODO(fabric2seer): `.addGuiContainerHandler(..)` on a IGuiHandlerRegistration
    //   java: registration.addGuiContainerHandler(InventoryScreen.class, new InventoryContainerHandler());
    // TODO(fabric2seer): `.addGuiContainerHandler(..)` on a IGuiHandlerRegistration
    //   java: registration.addGuiContainerHandler(StorageOverlayScreen.class, new StorageOverlayHandler());
    // TODO(fabric2seer): `.addGlobalGuiHandler(..)` on a IGuiHandlerRegistration
    //   java: registration.addGlobalGuiHandler(new GlobalHandler());
}

pub fn register_recipes(registration: &crate::unresolved::IRecipeRegistration) {
    // TODO(fabric2seer): `.getIngredientManager(..)` on a IRecipeRegistration
    //   java: registration.getIngredientManager().addIngredientsAtRuntime(VanillaTypes.ITEM_STACK, ItemRepository.getItems().stream().filter(Predicate.not(FlexibleItemStack::isEmpty)).map(FlexibleItemStack::getStackOrThrow).toList());
    // TODO(fabric2seer): `.addRecipes(..)` on a IRecipeRegistration
    //   java: registration.addRecipes(this.skyblockCraftingRecipeCategory.getRecipeType(), ItemRepository.getRecipesStream().filter(SkyblockCraftingRecipe.class::isInstance).map(SkyblockCraftingRecipe.class::cast).toList());
    // TODO(fabric2seer): `.addRecipes(..)` on a IRecipeRegistration
    //   java: registration.addRecipes(this.skyblockForgeRecipeCategory.getRecipeType(), ItemRepository.getRecipesStream().filter(SkyblockForgeRecipe.class::isInstance).map(SkyblockForgeRecipe.class::cast).toList());
    // TODO(fabric2seer): `.addRecipes(..)` on a IRecipeRegistration
    //   java: registration.addRecipes(this.skyblockNpcShopRecipeCategory.getRecipeType(), ItemRepository.getRecipesStream().filter(SkyblockNpcShopRecipe.class::isInstance).map(SkyblockNpcShopRecipe.class::cast).toList());
    // TODO(fabric2seer): `.addRecipes(..)` on a IRecipeRegistration
    //   java: registration.addRecipes(this.skyblockKatUpgradeRecipe.getRecipeType(), ItemRepository.getRecipesStream().filter(SkyblockKatUpgradeRecipe.class::isInstance).map(SkyblockKatUpgradeRecipe.class::cast).toList());
}

/// java: private static class GenericContainerHandler implements IGuiContainerHandler<ContainerScreen>
#[derive(Clone)]
pub struct GenericContainerHandler {}

impl GenericContainerHandler {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Override public List<Rect2i> getGuiExtraAreas(ContainerScreen containerScreen)
    pub fn get_gui_extra_areas(&self, container_screen: &ContainerScreen) -> Vec<Rect2i> {
        // java: containerScreen.getTitle().getString().contains("Museum")
        if !(Utils::is_on_skyblock()) || !(SkyblockerConfigManager::get().ui_and_visuals.museum_overlay) || !(container_screen.container.screen.get_title().to_string().contains("Museum")) {
            return vec![];
        }
        // java: AbstractContainerScreenAccessor accessor = (AbstractContainerScreenAccessor) containerScreen;
        let accessor = &container_screen.container;
        // java: return List.of(new Rect2i(accessor.getX() + accessor.getImageWidth() + 4, accessor.getY(), MuseumManager.BACKGROUND_WIDTH, MuseumManager.BACKGROUND_HEIGHT));
        return vec![Rect2i::new(accessor.left_pos + accessor.image_width + 4, accessor.top_pos, crate::de::hysky::skyblocker::skyblock::museum::museum_manager::BACKGROUND_WIDTH, crate::de::hysky::skyblocker::skyblock::museum::museum_manager::BACKGROUND_HEIGHT)];
    }

}

/// java: private static class InventoryContainerHandler implements IGuiContainerHandler<InventoryScreen>
#[derive(Clone)]
pub struct InventoryContainerHandler {}

impl InventoryContainerHandler {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Override public List<Rect2i> getGuiExtraAreas(InventoryScreen containerScreen)
    pub fn get_gui_extra_areas(&self, container_screen: &InventoryScreen) -> Vec<Rect2i> {
        if !(SkyblockerConfigManager::get().farming.plots_widget.enabled) || !(Utils::is_in_garden()) || garden_plots::WIDGET.get_mut().is_none() {
            return vec![];
        }
        return vec![Rect2i::new(garden_plots::WIDGET.get_mut().as_ref().unwrap().get_x(), garden_plots::WIDGET.get_mut().as_ref().unwrap().get_y(), garden_plots::WIDGET.get_mut().as_ref().unwrap().get_width(), garden_plots::WIDGET.get_mut().as_ref().unwrap().get_height())];
    }

}

/// java: private static class StorageOverlayHandler implements IGuiContainerHandler<StorageOverlayScreen>
#[derive(Clone)]
pub struct StorageOverlayHandler {}

impl StorageOverlayHandler {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Override public List<Rect2i> getGuiExtraAreas(StorageOverlayScreen screen)
    pub fn get_gui_extra_areas(&self, screen: &compat::Shared<StorageOverlayScreen>) -> Vec<Rect2i> {
        return vec![StorageOverlayScreen::get_main_exclusion_zone(screen), StorageOverlayScreen::get_buttons_exclusion_zone(screen)];
    }

}

/// java: private static class GlobalHandler implements IGlobalGuiHandler
#[derive(Clone)]
pub struct GlobalHandler {}

impl GlobalHandler {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Override public List<Rect2i> getGuiExtraAreas()
    pub fn get_gui_extra_areas(&self) -> Vec<Rect2i> {
        if !(Utils::is_on_skyblock()) || !(VisitorHelper::should_render()) {
            return vec![];
        }
        return VisitorHelper::get_exclusion_zones();
    }

}

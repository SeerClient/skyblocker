//! Ported from de.hysky.skyblocker.compatibility.emi.SkyblockerEMIPlugin by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::compatibility::emi::skyblock_emi_recipe::SkyblockEmiRecipe;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::itemlist::item_repository::ItemRepository;
use crate::de::hysky::skyblocker::skyblock::itemlist::recipes::skyblock_crafting_recipe;
use crate::de::hysky::skyblocker::skyblock::itemlist::recipes::skyblock_forge_recipe;
use mcrust::net::minecraft::resources::identifier::Identifier;

/// java: public static final Identifier SIMPLIFIED_TEXTURES = SkyblockerMod.id("textures/gui/emi_icons.png");
pub static SIMPLIFIED_TEXTURES: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("textures/gui/emi_icons.png"));
//   java: public static final EmiRecipeCategory SKYBLOCK_CRAFTING = new EmiRecipeCategory(SkyblockCraftingRecipe.ID, EmiStack.of(Items.CRAFTING_TABLE), new EmiTexture(SIMPLIFIED_TEXTURES, 0, 0, 16, 16));
/// java: public static final EmiRecipeCategory SKYBLOCK_CRAFTING = new EmiRecipeCategory(SkyblockCraftingRecipe.ID, EmiStack.of(Items.CRAFTING_TABLE), new EmiTexture(SIMPLIFIED_TEXTURES, 0, 0, 16, 16));
pub static SKYBLOCK_CRAFTING: compat::MainThread<crate::unresolved::EmiRecipeCategory> = compat::MainThread::new(|| emi_recipe_category(skyblock_crafting_recipe::ID.get_mut().clone(), emi_stack_of_item(&mcrust::net::minecraft::world::item::items::items().get("crafting_table")), emi_texture(SIMPLIFIED_TEXTURES.get_mut().clone(), 0, 0, 16, 16)));
//   java: public static final EmiRecipeCategory SKYBLOCK_FORGE = new EmiRecipeCategory(SkyblockForgeRecipe.ID, EmiStack.of(Items.LAVA_BUCKET), new EmiTexture(SIMPLIFIED_TEXTURES, 16, 0, 16, 16));
/// java: public static final EmiRecipeCategory SKYBLOCK_FORGE = new EmiRecipeCategory(SkyblockForgeRecipe.ID, EmiStack.of(Items.LAVA_BUCKET), new EmiTexture(SIMPLIFIED_TEXTURES, 16, 0, 16, 16));
pub static SKYBLOCK_FORGE: compat::MainThread<crate::unresolved::EmiRecipeCategory> = compat::MainThread::new(|| emi_recipe_category(skyblock_forge_recipe::ID.get_mut().clone(), emi_stack_of_item(&mcrust::net::minecraft::world::item::items::items().get("lava_bucket")), emi_texture(SIMPLIFIED_TEXTURES.get_mut().clone(), 16, 0, 16, 16)));
/// java: protected static final Map<Identifier, EmiRecipeCategory> IDENTIFIER_CATEGORY_MAP = Map.of( SkyblockCraftingRecipe.ID, SKYBLOCK_CRAFTING, SkyblockForgeRecipe.ID, SKYBLOCK_FORGE );
/// (the map holds the two category OBJECTS: references to the statics, as Java's map holds references)
pub static IDENTIFIER_CATEGORY_MAP: compat::MainThread<std::collections::HashMap<Identifier, &'static crate::unresolved::EmiRecipeCategory>> = compat::MainThread::new(|| std::collections::HashMap::from([(skyblock_crafting_recipe::ID.get_mut().clone(), SKYBLOCK_CRAFTING.get()), (skyblock_forge_recipe::ID.get_mut().clone(), SKYBLOCK_FORGE.get())]));

/// java: public class SkyblockerEMIPlugin implements EmiPlugin
#[derive(Clone)]
pub struct SkyblockerEMIPlugin {}

impl SkyblockerEMIPlugin {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Override public void register(EmiRegistry registry)
    pub fn register(&self, registry: &mut crate::unresolved::EmiRegistry) {
        if !(SkyblockerConfigManager::get().general.item_list.enable_item_list) {
            return;
        }
        // TODO(fabric2seer): `of` is not a method of FlexibleItemStack
        //   java: ItemRepository.getItemsStream().map(EmiStack::of).forEach(emiStack -> {
        //   java: 			registry.addEmiStack(emiStack);
        //   java: 			registry.setDefaultComparison(emiStack, Comparison.compareData(emiStack1 -> emiStack1.getItemStack().getSkyblockId()));
        //   java: 		});
        // TODO(fabric2seer): `.addCategory(..)` on a EmiRegistry
        //   java: registry.addCategory(SKYBLOCK_CRAFTING);
        // TODO(fabric2seer): `.addCategory(..)` on a EmiRegistry
        //   java: registry.addCategory(SKYBLOCK_FORGE);
        // TODO(fabric2seer): `.addWorkstation(..)` on a EmiRegistry
        //   java: registry.addWorkstation(SKYBLOCK_CRAFTING, EmiStack.of(Items.CRAFTING_TABLE));
        // TODO(fabric2seer): `.addWorkstation(..)` on a EmiRegistry
        //   java: registry.addWorkstation(SKYBLOCK_CRAFTING, EmiStack.of(Items.LAVA_BUCKET));
        for __a0 in ItemRepository::get_recipes_stream().map(|__a0| SkyblockEmiRecipe::new(__a0.clone())) {
            // TODO(fabric2seer): `.addRecipe(..)` on a EmiRegistry
            //   java: registry::addRecipe;
        }
        // TODO(fabric2seer): `.addExclusionArea(..)` on a EmiRegistry
        //   java: registry.addExclusionArea(InventoryScreen.class, (screen, consumer) -> {
        //   java: 			if (!SkyblockerConfigManager.get().farming.plotsWidget.enabled || !Utils.getLocation().equals(Location.GARDEN)) return;
        //   java: 			AbstractContainerScreenAccessor accessor = (AbstractContainerScreenAccessor) screen;
        //   java: 			consumer.accept(new Bounds(accessor.getX() + accessor.getImageWidth() + 4, accessor.getY(), 104, 127));
        //   java: 		});
    }

}

// EMI's side of the boundary (EMI is a separate mod SeerClient never loads; see `skyblock_emi_recipe.rs`).

/// `new EmiRecipeCategory(id, icon, simplified)`.
fn emi_recipe_category(id: Identifier, icon: crate::unresolved::EmiStack, simplified: EmiTexture) -> crate::unresolved::EmiRecipeCategory {
    let _ = (id, icon, simplified);
    crate::unresolved::EmiRecipeCategory
}

/// `EmiStack.of(ItemLike)`.
fn emi_stack_of_item(item: &mcrust::net::minecraft::world::item::item::Item) -> crate::unresolved::EmiStack {
    let _ = item;
    crate::unresolved::EmiStack
}

/// `dev.emi.emi.api.render.EmiTexture` - a texture region (`new EmiTexture(texture, u, v, width, height)`).
pub struct EmiTexture {
    pub texture: Identifier,
    pub u: i32,
    pub v: i32,
    pub width: i32,
    pub height: i32,
}

/// `new EmiTexture(texture, u, v, width, height)`.
fn emi_texture(texture: Identifier, u: i32, v: i32, width: i32, height: i32) -> EmiTexture {
    EmiTexture { texture, u, v, width, height }
}

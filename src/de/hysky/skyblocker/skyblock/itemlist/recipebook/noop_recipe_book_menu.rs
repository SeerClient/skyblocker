//! Ported from de.hysky.skyblocker.skyblock.itemlist.recipebook.NoopRecipeBookMenu by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::world::entity::player::inventory::Inventory;
use mcrust::net::minecraft::world::entity::player::player::Player;
use mcrust::net::minecraft::world::entity::player::stacked_item_contents::StackedItemContents;
use mcrust::net::minecraft::world::inventory::recipe_book_menu::PostPlaceAction;
use mcrust::net::minecraft::world::inventory::recipe_book_menu::RecipeBookMenu;
use mcrust::net::minecraft::world::inventory::recipe_book_type::RecipeBookType;
use mcrust::net::minecraft::world::item::crafting::recipe_holder::RecipeHolder;
use mcrust::net::minecraft::world::level::level::Level;

/// java: public class NoopRecipeBookMenu extends RecipeBookMenu
/// (`RecipeBookMenu extends AbstractContainerMenu`: the menu half is the base; the port's `RecipeBookMenu` is the trait
/// of its two client abstract members, implemented below)
pub struct NoopRecipeBookMenu {
    /// java: extends AbstractContainerMenu (through RecipeBookMenu)
    pub base: mcrust::net::minecraft::world::inventory::abstract_container_menu::AbstractContainerMenu,
}

impl NoopRecipeBookMenu {
    /// java: protected NoopRecipeBookMenu(int syncId)
    pub fn new(mut sync_id: i32) -> Self {
        // java: super(null, syncId);
        let mut this = Self {
            base: mcrust::net::minecraft::world::inventory::abstract_container_menu::AbstractContainerMenu::new(sync_id),
        };
        this
    }

    /// java: @Override public PostPlaceAction handlePlacement(boolean craftAll, boolean creative, RecipeHolder<?> recipe, ServerLevel world, Inventory inventory)
    pub fn handle_placement(&self, mut craft_all: bool, mut creative: bool, recipe: &RecipeHolder, world: &Level, inventory: &Inventory) -> PostPlaceAction {
        return PostPlaceAction::Nothing;
    }

    /// java: @Override public ItemStack quickMoveStack(Player player, int slot)
    pub fn quick_move_stack(&self, player: &dyn Player, mut slot: i32) -> mcrust::net::minecraft::world::item::item_stack::ItemStack {
        return mcrust::net::minecraft::world::item::item_stack::empty();
    }

    /// java: @Override public boolean stillValid(Player player)
    pub fn still_valid(&self, player: &dyn Player) -> bool {
        return false;
    }

}

impl RecipeBookMenu for NoopRecipeBookMenu {
    /// java: @Override public void fillCraftSlotsStackedContents(StackedItemContents finder)
    fn fill_craft_slots_stacked_contents(&self, finder: &mut StackedItemContents) {
    }

    /// java: @Override public RecipeBookType getRecipeBookType()
    fn get_recipe_book_type(&self) -> RecipeBookType {
        return RecipeBookType::Crafting;
    }
}

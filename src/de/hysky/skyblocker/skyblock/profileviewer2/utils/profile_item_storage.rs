//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.utils.ProfileItemStorage by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: public record ProfileItemStorage(List<ItemStack> inventory, List<ItemStack> armour, List<ItemStack> equipment, List<ItemStack> enderChestContents, TreeMap<Integer, Backpack> backpacks, List<ItemStack> armourSets, List<ItemStack> equipmentSe
#[derive(Clone, Debug, PartialEq)]
pub struct ProfileItemStorage {
    /// java: List<ItemStack> inventory
    pub inventory: Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack>,
    /// java: List<ItemStack> armour
    pub armour: Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack>,
    /// java: List<ItemStack> equipment
    pub equipment: Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack>,
    /// java: List<ItemStack> enderChestContents
    pub ender_chest_contents: Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack>,
    /// java: TreeMap<Integer, Backpack> backpacks
    pub backpacks: std::collections::BTreeMap<i32, Backpack>,
    /// java: List<ItemStack> armourSets
    pub armour_sets: Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack>,
    /// java: List<ItemStack> equipmentSets
    pub equipment_sets: Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack>,
    /// java: List<ItemStack> pets
    pub pets: Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack>,
    /// java: Bags bags
    pub bags: Bags,
}

impl ProfileItemStorage {
    /// java: the canonical record constructor
    pub fn new(mut inventory: Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack>, mut armour: Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack>, mut equipment: Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack>, mut ender_chest_contents: Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack>, mut backpacks: std::collections::BTreeMap<i32, Backpack>, mut armour_sets: Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack>, mut equipment_sets: Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack>, mut pets: Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack>, mut bags: Bags) -> Self {
        Self { inventory, armour, equipment, ender_chest_contents, backpacks, armour_sets, equipment_sets, pets, bags }
    }

    /// java: the record accessor `inventory()`
    pub fn inventory(&self) -> Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack> {
        self.inventory.clone()
    }

    /// java: the record accessor `armour()`
    pub fn armour(&self) -> Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack> {
        self.armour.clone()
    }

    /// java: the record accessor `equipment()`
    pub fn equipment(&self) -> Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack> {
        self.equipment.clone()
    }

    /// java: the record accessor `enderChestContents()`
    pub fn ender_chest_contents(&self) -> Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack> {
        self.ender_chest_contents.clone()
    }

    /// java: the record accessor `backpacks()`
    pub fn backpacks(&self) -> std::collections::BTreeMap<i32, Backpack> {
        self.backpacks.clone()
    }

    /// java: the record accessor `armourSets()`
    pub fn armour_sets(&self) -> Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack> {
        self.armour_sets.clone()
    }

    /// java: the record accessor `equipmentSets()`
    pub fn equipment_sets(&self) -> Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack> {
        self.equipment_sets.clone()
    }

    /// java: the record accessor `pets()`
    pub fn pets(&self) -> Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack> {
        self.pets.clone()
    }

    /// java: the record accessor `bags()`
    pub fn bags(&self) -> Bags {
        self.bags.clone()
    }

}

/// java: public record Backpack(ItemStack icon, List<ItemStack> contents)
#[derive(Clone, Debug, PartialEq)]
pub struct Backpack {
    /// java: ItemStack icon
    pub icon: mcrust::net::minecraft::world::item::item_stack::ItemStack,
    /// java: List<ItemStack> contents
    pub contents: Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack>,
}

impl Backpack {
    /// java: the canonical record constructor
    pub fn new(mut icon: mcrust::net::minecraft::world::item::item_stack::ItemStack, mut contents: Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> Self {
        Self { icon, contents }
    }

    /// java: the record accessor `icon()`
    pub fn icon(&self) -> mcrust::net::minecraft::world::item::item_stack::ItemStack {
        self.icon.clone()
    }

    /// java: the record accessor `contents()`
    pub fn contents(&self) -> Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack> {
        self.contents.clone()
    }

}

/// java: public record Bags(List<ItemStack> accessories)
#[derive(Clone, Debug, PartialEq)]
pub struct Bags {
    /// java: List<ItemStack> accessories
    pub accessories: Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack>,
}

impl Bags {
    /// java: the canonical record constructor
    pub fn new(mut accessories: Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> Self {
        Self { accessories }
    }

    /// java: the record accessor `accessories()`
    pub fn accessories(&self) -> Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack> {
        self.accessories.clone()
    }

}

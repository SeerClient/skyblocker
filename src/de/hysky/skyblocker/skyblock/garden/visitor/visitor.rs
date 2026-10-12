//! Ported from de.hysky.skyblocker.skyblock.garden.visitor.Visitor by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public record Visitor(Component name, ItemStack head, Object2IntMap<Component> requiredItems)
#[derive(Clone, Debug)]
pub struct Visitor {
    /// java: Component name
    pub name: MutableComponent,
    /// java: ItemStack head
    pub head: mcrust::net::minecraft::world::item::item_stack::ItemStack,
    /// java: Object2IntMap<Component> requiredItems
    pub required_items: std::collections::HashMap<MutableComponent, i32>,
}

impl Visitor {
    /// java: public Visitor(Component name, ItemStack head)
    pub fn new(mut name: MutableComponent, mut head: mcrust::net::minecraft::world::item::item_stack::ItemStack) -> Self {
        let mut this = Self::new(name.clone(), head.clone(), std::collections::HashMap::new());
        this
    }

    /// java: the canonical record constructor
    pub fn new(mut name: MutableComponent, mut head: mcrust::net::minecraft::world::item::item_stack::ItemStack, mut required_items: std::collections::HashMap<MutableComponent, i32>) -> Self {
        Self { name, head, required_items }
    }

    /// java: the record accessor `name()`
    pub fn name(&self) -> MutableComponent {
        self.name.clone()
    }

    /// java: the record accessor `head()`
    pub fn head(&self) -> mcrust::net::minecraft::world::item::item_stack::ItemStack {
        self.head.clone()
    }

    /// java: the record accessor `requiredItems()`
    pub fn required_items(&self) -> std::collections::HashMap<MutableComponent, i32> {
        self.required_items.clone()
    }

    /// java: public void addRequiredItem(Component item, int amount)
    pub fn add_required_item(&mut self, mut item: MutableComponent, mut amount: i32) {
        self.required_items.insert(item.clone(), (self.required_items.get(&item).cloned().unwrap_or_else(|| 0)).wrapping_add(amount));
    }

}

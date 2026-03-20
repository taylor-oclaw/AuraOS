extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct MarketplaceCatalog {
    items: Vec<Item>,
}

impl MarketplaceCatalog {
    pub fn new() -> Self {
        MarketplaceCatalog { items: Vec::new() }
    }

    pub fn add_item(&mut self, item: Item) {
        self.items.push(item);
    }

    pub fn remove_item(&mut self, id: u32) -> Option<Item> {
        let index = self.items.iter().position(|item| item.id == id)?;
        Some(self.items.remove(index))
    }

    pub fn get_item_by_id(&self, id: u32) -> Option<&Item> {
        self.items.iter().find(|item| item.id == id)
    }

    pub fn list_items(&self) -> &[Item] {
        &self.items
    }

    pub fn count_items(&self) -> usize {
        self.items.len()
    }
}

pub struct Item {
    pub id: u32,
    pub name: String,
    pub price: u64,
    pub description: String,
}

impl Item {
    pub fn new(id: u32, name: String, price: u64, description: String) -> Self {
        Item { id, name, price, description }
    }
}

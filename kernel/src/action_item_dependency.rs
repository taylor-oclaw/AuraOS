extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct ActionItem {
    id: u32,
    description: String,
    dependencies: Vec<u32>,
}

impl ActionItem {
    pub fn new(id: u32, description: &str) -> Self {
        ActionItem {
            id,
            description: String::from(description),
            dependencies: Vec::new(),
        }
    }

    pub fn add_dependency(&mut self, dependency_id: u32) {
        if !self.dependencies.contains(&dependency_id) {
            self.dependencies.push(dependency_id);
        }
    }

    pub fn remove_dependency(&mut self, dependency_id: u32) {
        self.dependencies.retain(|&id| id != dependency_id);
    }

    pub fn has_dependency(&self, dependency_id: u32) -> bool {
        self.dependencies.contains(&dependency_id)
    }

    pub fn get_dependencies(&self) -> &Vec<u32> {
        &self.dependencies
    }
}

#[derive(Debug)]
pub struct ActionItemDependencyManager {
    items: Vec<ActionItem>,
}

impl ActionItemDependencyManager {
    pub fn new() -> Self {
        ActionItemDependencyManager {
            items: Vec::new(),
        }
    }

    pub fn add_item(&mut self, item: ActionItem) {
        if !self.items.iter().any(|i| i.id == item.id) {
            self.items.push(item);
        }
    }

    pub fn remove_item(&mut self, id: u32) {
        self.items.retain(|i| i.id != id);
    }

    pub fn get_item(&self, id: u32) -> Option<&ActionItem> {
        self.items.iter().find(|&i| i.id == id)
    }

    pub fn add_dependency(&mut self, item_id: u32, dependency_id: u32) {
        if let Some(item) = self.get_item_mut(item_id) {
            item.add_dependency(dependency_id);
        }
    }

    pub fn remove_dependency(&mut self, item_id: u32, dependency_id: u32) {
        if let Some(item) = self.get_item_mut(item_id) {
            item.remove_dependency(dependency_id);
        }
    }

    pub fn has_dependency(&self, item_id: u32, dependency_id: u32) -> bool {
        if let Some(item) = self.get_item(item_id) {
            item.has_dependency(dependency_id)
        } else {
            false
        }
    }

    pub fn get_dependencies(&self, item_id: u32) -> Option<&Vec<u32>> {
        if let Some(item) = self.get_item(item_id) {
            Some(item.get_dependencies())
        } else {
            None
        }
    }

    fn get_item_mut(&mut self, id: u32) -> Option<&mut ActionItem> {
        self.items.iter_mut().find(|i| i.id == id)
    }
}

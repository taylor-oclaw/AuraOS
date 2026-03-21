extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct Chore {
    description: String,
    completed: bool,
}

impl Chore {
    pub fn new(description: &str) -> Self {
        Chore {
            description: String::from(description),
            completed: false,
        }
    }

    pub fn mark_completed(&mut self) {
        self.completed = true;
    }

    pub fn is_completed(&self) -> bool {
        self.completed
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }
}

pub struct FamilyHubChoreTracker {
    chores: Vec<Chore>,
}

impl FamilyHubChoreTracker {
    pub fn new() -> Self {
        FamilyHubChoreTracker {
            chores: Vec::new(),
        }
    }

    pub fn add_chore(&mut self, description: &str) {
        let chore = Chore::new(description);
        self.chores.push(chore);
    }

    pub fn get_chores(&self) -> &[Chore] {
        &self.chores
    }

    pub fn mark_chore_completed(&mut self, index: usize) {
        if let Some(chore) = self.chores.get_mut(index) {
            chore.mark_completed();
        }
    }

    pub fn get_completed_chores_count(&self) -> usize {
        self.chores.iter().filter(|chore| chore.is_completed()).count()
    }
}

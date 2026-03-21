extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rel_gift_idea_generate_init() {
    // Initialization code for the module
}

pub extern "C" fn rel_gift_idea_generate_exit() {
    // Cleanup code for the module
}

pub struct GiftIdeaGenerator {
    categories: Vec<String>,
    ideas: Vec<String>,
}

impl GiftIdeaGenerator {
    pub fn new(categories: Vec<&str>, ideas: Vec<&str>) -> Self {
        GiftIdeaGenerator {
            categories: categories.into_iter().map(|s| s.to_string()).collect(),
            ideas: ideas.into_iter().map(|s| s.to_string()).collect(),
        }
    }

    pub fn add_category(&mut self, category: &str) {
        self.categories.push(category.to_string());
    }

    pub fn remove_category(&mut self, category: &str) {
        self.categories.retain(|c| c != category);
    }

    pub fn add_idea(&mut self, idea: &str) {
        self.ideas.push(idea.to_string());
    }

    pub fn remove_idea(&mut self, idea: &str) {
        self.ideas.retain(|i| i != idea);
    }

    pub fn get_random_category(&self) -> Option<&String> {
        if self.categories.is_empty() {
            None
        } else {
            Some(&self.categories[0]) // Simplified for kernel module context
        }
    }

    pub fn get_random_idea(&self) -> Option<&String> {
        if self.ideas.is_empty() {
            None
        } else {
            Some(&self.ideas[0]) // Simplified for kernel module context
        }
    }
}

pub extern "C" fn rel_gift_idea_generate_new(categories: *const *const u8, ideas: *const *const u8) -> *mut GiftIdeaGenerator {
    let categories_vec = unsafe { core::slice::from_raw_parts(categories, 2) } // Example size
        .iter()
        .map(|&c| String::from_utf8_lossy(c).to_string())
        .collect();

    let ideas_vec = unsafe { core::slice::from_raw_parts(ideas, 3) } // Example size
        .iter()
        .map(|&i| String::from_utf8_lossy(i).to_string())
        .collect();

    Box::into_raw(Box::new(GiftIdeaGenerator::new(categories_vec, ideas_vec)))
}

pub extern "C" fn rel_gift_idea_generate_free(generator: *mut GiftIdeaGenerator) {
    unsafe { drop(Box::from_raw(generator)); }
}

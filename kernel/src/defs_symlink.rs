extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct Symlink {
    target: String,
    links: Vec<String>,
}

impl Symlink {
    pub fn new(target: &str) -> Self {
        Symlink {
            target: String::from(target),
            links: Vec::new(),
        }
    }

    pub fn add_link(&mut self, link_name: &str) {
        if !self.links.contains(&String::from(link_name)) {
            self.links.push(String::from(link_name));
        }
    }

    pub fn remove_link(&mut self, link_name: &str) {
        self.links.retain(|link| *link != String::from(link_name));
    }

    pub fn get_target(&self) -> &str {
        &self.target
    }

    pub fn list_links(&self) -> Vec<&str> {
        self.links.iter().map(|s| s.as_str()).collect()
    }

    pub fn has_link(&self, link_name: &str) -> bool {
        self.links.contains(&String::from(link_name))
    }
}

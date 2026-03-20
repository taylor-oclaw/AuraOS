extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct SymlinkHandler {
    symlinks: Vec<(String, String)>,
}

impl SymlinkHandler {
    pub fn new() -> Self {
        SymlinkHandler {
            symlinks: Vec::new(),
        }
    }

    pub fn add_symlink(&mut self, target: &str, link_name: &str) {
        let target = String::from(target);
        let link_name = String::from(link_name);
        self.symlinks.push((target, link_name));
    }

    pub fn remove_symlink(&mut self, link_name: &str) -> bool {
        if let Some(index) = self.symlinks.iter().position(|(_, ln)| ln == link_name) {
            self.symlinks.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_target(&self, link_name: &str) -> Option<&String> {
        self.symlinks.iter().find_map(|(target, ln)| if ln == link_name { Some(target) } else { None })
    }

    pub fn list_symlinks(&self) -> Vec<(String, String)> {
        self.symlinks.clone()
    }

    pub fn exists(&self, link_name: &str) -> bool {
        self.symlinks.iter().any(|(_, ln)| ln == link_name)
    }
}

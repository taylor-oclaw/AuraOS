extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct HardlinkManager {
    links: Vec<(String, String)>,
}

impl HardlinkManager {
    pub fn new() -> Self {
        HardlinkManager { links: Vec::new() }
    }

    pub fn add_link(&mut self, target: &str, link_name: &str) -> Result<(), &'static str> {
        if self.links.iter().any(|&(ref t, ref l)| t == target || l == link_name) {
            Err("Link or target already exists")
        } else {
            self.links.push((String::from(target), String::from(link_name)));
            Ok(())
        }
    }

    pub fn remove_link(&mut self, link_name: &str) -> Result<(), &'static str> {
        if let Some(pos) = self.links.iter().position(|&(_, ref l)| l == link_name) {
            self.links.remove(pos);
            Ok(())
        } else {
            Err("Link does not exist")
        }
    }

    pub fn get_target(&self, link_name: &str) -> Result<String, &'static str> {
        if let Some(&(ref target, _)) = self.links.iter().find(|&(_, ref l)| l == link_name) {
            Ok(target.clone())
        } else {
            Err("Link does not exist")
        }
    }

    pub fn list_links(&self) -> Vec<(String, String)> {
        self.links.clone()
    }

    pub fn exists(&self, link_name: &str) -> bool {
        self.links.iter().any(|&(_, ref l)| l == link_name)
    }
}

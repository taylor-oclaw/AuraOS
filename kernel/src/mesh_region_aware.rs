extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct MeshRegionAware {
    regions: Vec<String>,
}

impl MeshRegionAware {
    pub fn new() -> Self {
        MeshRegionAware {
            regions: Vec::new(),
        }
    }

    pub fn add_region(&mut self, region_name: &str) {
        if !self.regions.contains(&String::from(region_name)) {
            self.regions.push(String::from(region_name));
        }
    }

    pub fn remove_region(&mut self, region_name: &str) {
        self.regions.retain(|r| r != region_name);
    }

    pub fn get_regions(&self) -> Vec<String> {
        self.regions.clone()
    }

    pub fn is_region_present(&self, region_name: &str) -> bool {
        self.regions.contains(&String::from(region_name))
    }

    pub fn count_regions(&self) -> usize {
        self.regions.len()
    }
}

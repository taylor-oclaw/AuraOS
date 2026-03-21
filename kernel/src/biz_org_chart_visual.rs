extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[repr(C)]
pub struct BizOrgChartVisual {
    name: String,
    position: String,
    subordinates: Vec<BizOrgChartVisual>,
}

impl BizOrgChartVisual {
    pub fn new(name: &str, position: &str) -> Self {
        BizOrgChartVisual {
            name: String::from(name),
            position: String::from(position),
            subordinates: Vec::new(),
        }
    }

    pub fn add_subordinate(&mut self, subordinate: BizOrgChartVisual) {
        self.subordinates.push(subordinate);
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_position(&self) -> &str {
        &self.position
    }

    pub fn list_subordinates(&self) -> Vec<&BizOrgChartVisual> {
        self.subordinates.iter().collect()
    }

    pub fn find_by_name(&self, name: &str) -> Option<&BizOrgChartVisual> {
        if self.name == name {
            return Some(self);
        }
        for subordinate in &self.subordinates {
            if let Some(found) = subordinate.find_by_name(name) {
                return Some(found);
            }
        }
        None
    }
}

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct VlanManager {
    vlans: Vec<Vlan>,
}

impl VlanManager {
    pub fn new() -> Self {
        VlanManager {
            vlans: Vec::new(),
        }
    }

    pub fn add_vlan(&mut self, id: u16, name: String) -> Result<(), &'static str> {
        if self.vlans.iter().any(|vlan| vlan.id == id || vlan.name == name) {
            Err("VLAN with the same ID or name already exists")
        } else {
            self.vlans.push(Vlan { id, name });
            Ok(())
        }
    }

    pub fn remove_vlan(&mut self, id: u16) -> Result<(), &'static str> {
        if let Some(index) = self.vlans.iter().position(|vlan| vlan.id == id) {
            self.vlans.remove(index);
            Ok(())
        } else {
            Err("VLAN not found")
        }
    }

    pub fn get_vlan_by_id(&self, id: u16) -> Option<&Vlan> {
        self.vlans.iter().find(|vlan| vlan.id == id)
    }

    pub fn get_vlan_by_name(&self, name: &str) -> Option<&Vlan> {
        self.vlans.iter().find(|vlan| vlan.name == name)
    }

    pub fn list_vlans(&self) -> Vec<&Vlan> {
        self.vlans.iter().collect()
    }
}

struct Vlan {
    id: u16,
    name: String,
}

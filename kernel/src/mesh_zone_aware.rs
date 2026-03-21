extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn mesh_zone_aware_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn mesh_zone_aware_exit() {
    // Cleanup logic for the module
}

pub struct MeshZoneAware {
    zones: Vec<String>,
    active_zone: Option<usize>,
}

impl MeshZoneAware {
    pub fn new(zones: Vec<String>) -> Self {
        MeshZoneAware {
            zones,
            active_zone: None,
        }
    }

    pub fn add_zone(&mut self, zone_name: String) {
        self.zones.push(zone_name);
    }

    pub fn remove_zone(&mut self, zone_index: usize) {
        if let Some(index) = self.active_zone {
            if index == zone_index {
                self.active_zone = None;
            }
        }
        if zone_index < self.zones.len() {
            self.zones.remove(zone_index);
        }
    }

    pub fn set_active_zone(&mut self, zone_index: usize) -> bool {
        if zone_index < self.zones.len() {
            self.active_zone = Some(zone_index);
            true
        } else {
            false
        }
    }

    pub fn get_active_zone(&self) -> Option<&String> {
        self.active_zone.map(|index| &self.zones[index])
    }

    pub fn list_zones(&self) -> &[String] {
        &self.zones
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mesh_zone_aware() {
        let mut mza = MeshZoneAware::new(vec![
            String::from("zone1"),
            String::from("zone2"),
        ]);

        assert_eq!(mza.list_zones(), &["zone1", "zone2"]);
        assert_eq!(mza.get_active_zone(), None);

        assert!(mza.set_active_zone(0));
        assert_eq!(mza.get_active_zone(), Some(&String::from("zone1")));

        mza.add_zone(String::from("zone3"));
        assert_eq!(mza.list_zones(), &["zone1", "zone2", "zone3"]);

        mza.remove_zone(1);
        assert_eq!(mza.list_zones(), &["zone1", "zone3"]);
        assert_eq!(mza.get_active_zone(), Some(&String::from("zone1")));

        mza.set_active_zone(2);
        assert_eq!(mza.get_active_zone(), Some(&String::from("zone3")));
    }
}

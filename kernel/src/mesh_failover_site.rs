extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn init_module() -> i32 {
    0
}

#[no_mangle]
pub extern "C" fn cleanup_module() {}

struct MeshFailoverSite {
    site_id: u32,
    name: String,
    active: bool,
    neighbors: Vec<u32>,
    failover_count: u32,
}

impl MeshFailoverSite {
    pub fn new(site_id: u32, name: &str) -> Self {
        MeshFailoverSite {
            site_id,
            name: String::from(name),
            active: true,
            neighbors: Vec::new(),
            failover_count: 0,
        }
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }

    pub fn activate(&mut self) {
        self.active = true;
    }

    pub fn add_neighbor(&mut self, neighbor_id: u32) {
        if !self.neighbors.contains(&neighbor_id) {
            self.neighbors.push(neighbor_id);
        }
    }

    pub fn remove_neighbor(&mut self, neighbor_id: u32) {
        self.neighbors.retain(|&id| id != neighbor_id);
    }

    pub fn failover(&mut self) {
        if self.active {
            self.deactivate();
            self.failover_count += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mesh_failover_site() {
        let mut site = MeshFailoverSite::new(1, "SiteA");
        assert_eq!(site.site_id, 1);
        assert_eq!(site.name, "SiteA");
        assert!(site.active);

        site.deactivate();
        assert!(!site.active);

        site.activate();
        assert!(site.active);

        site.add_neighbor(2);
        assert_eq!(site.neighbors.len(), 1);
        assert_eq!(site.neighbors[0], 2);

        site.remove_neighbor(2);
        assert_eq!(site.neighbors.len(), 0);

        site.failover();
        assert!(!site.active);
        assert_eq!(site.failover_count, 1);
    }
}

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

#[derive(Debug)]
pub struct SmartHomeTriggerGeofence {
    name: String,
    geofences: Vec<String>,
    active_geofence: Option<String>,
}

impl SmartHomeTriggerGeofence {
    pub fn new(name: &str) -> Self {
        SmartHomeTriggerGeofence {
            name: String::from(name),
            geofences: Vec::new(),
            active_geofence: None,
        }
    }

    pub fn add_geofence(&mut self, geofence_name: &str) {
        if !self.geofences.contains(&String::from(geofence_name)) {
            self.geofences.push(String::from(geofence_name));
        }
    }

    pub fn remove_geofence(&mut self, geofence_name: &str) {
        self.geofences.retain(|g| g != geofence_name);
        if let Some(active) = &self.active_geofence {
            if active == geofence_name {
                self.active_geofence = None;
            }
        }
    }

    pub fn set_active_geofence(&mut self, geofence_name: &str) -> bool {
        if self.geofences.contains(&String::from(geofence_name)) {
            self.active_geofence = Some(String::from(geofence_name));
            true
        } else {
            false
        }
    }

    pub fn get_active_geofence(&self) -> Option<&str> {
        self.active_geofence.as_deref()
    }

    pub fn list_geofences(&self) -> Vec<&str> {
        self.geofences.iter().map(|g| g.as_str()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smart_home_trigger_geofence() {
        let mut geofence = SmartHomeTriggerGeofence::new("Home");
        assert_eq!(geofence.name, "Home");
        assert!(geofence.geofences.is_empty());
        assert_eq!(geofence.active_geofence, None);

        geofence.add_geofence("Entrance");
        geofence.add_geofence("LivingRoom");
        assert_eq!(geofence.list_geofences(), vec!["Entrance", "LivingRoom"]);

        geofence.set_active_geofence("Entrance");
        assert_eq!(geofence.get_active_geofence(), Some("Entrance"));

        geofence.remove_geofence("Entrance");
        assert_eq!(geofence.list_geofences(), vec!["LivingRoom"]);
        assert_eq!(geofence.get_active_geofence(), None);
    }
}

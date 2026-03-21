extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

mod mdm_geofence {
    use super::*;

    pub struct Geofence {
        name: String,
        coordinates: Vec<(i32, i32)>,
        radius: u32,
    }

    impl Geofence {
        pub fn new(name: &str, center: (i32, i32), radius: u32) -> Self {
            Geofence {
                name: String::from(name),
                coordinates: vec![center],
                radius,
            }
        }

        pub fn add_coordinate(&mut self, coord: (i32, i32)) {
            self.coordinates.push(coord);
        }

        pub fn remove_coordinate(&mut self, index: usize) -> Option<(i32, i32)> {
            if index < self.coordinates.len() {
                Some(self.coordinates.remove(index))
            } else {
                None
            }
        }

        pub fn is_within_radius(&self, point: (i32, i32)) -> bool {
            let center = self.coordinates[0];
            let dx = point.0 - center.0;
            let dy = point.1 - center.1;
            (dx * dx + dy * dy) as u32 <= self.radius * self.radius
        }

        pub fn get_name(&self) -> &str {
            &self.name
        }
    }
}

#[cfg(test)]
mod tests {
    use super::mdm_geofence::*;

    #[test]
    fn test_geofence_creation() {
        let geofence = Geofence::new("Home", (0, 0), 10);
        assert_eq!(geofence.get_name(), "Home");
        assert_eq!(geofence.coordinates.len(), 1);
        assert_eq!(geofence.radius, 10);
    }

    #[test]
    fn test_add_coordinate() {
        let mut geofence = Geofence::new("Work", (5, 5), 20);
        geofence.add_coordinate((10, 10));
        assert_eq!(geofence.coordinates.len(), 2);
        assert_eq!(geofence.coordinates[1], (10, 10));
    }

    #[test]
    fn test_remove_coordinate() {
        let mut geofence = Geofence::new("School", (15, 15), 30);
        geofence.add_coordinate((20, 20));
        assert_eq!(geofence.coordinates.len(), 2);
        assert!(geofence.remove_coordinate(1).is_some());
        assert_eq!(geofence.coordinates.len(), 1);
    }

    #[test]
    fn test_is_within_radius() {
        let geofence = Geofence::new("Park", (30, 30), 40);
        assert!(geofence.is_within_radius((35, 35)));
        assert!(!geofence.is_within_radius((75, 75)));
    }
}

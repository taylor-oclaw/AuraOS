extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

mod travel_emergency_info {
    extern crate alloc;
    use alloc::string::String;
    use alloc::vec::Vec;

    pub struct TravelEmergencyInfo {
        location: String,
        contact_numbers: Vec<String>,
        emergency_services: Vec<String>,
        medical_facilities: Vec<String>,
        evacuation_routes: Vec<String>,
    }

    impl TravelEmergencyInfo {
        pub fn new(location: &str) -> Self {
            TravelEmergencyInfo {
                location: String::from(location),
                contact_numbers: Vec::new(),
                emergency_services: Vec::new(),
                medical_facilities: Vec::new(),
                evacuation_routes: Vec::new(),
            }
        }

        pub fn add_contact_number(&mut self, number: &str) {
            self.contact_numbers.push(String::from(number));
        }

        pub fn add_emergency_service(&mut self, service: &str) {
            self.emergency_services.push(String::from(service));
        }

        pub fn add_medical_facility(&mut self, facility: &str) {
            self.medical_facilities.push(String::from(facility));
        }

        pub fn add_evacuation_route(&mut self, route: &str) {
            self.evacuation_routes.push(String::from(route));
        }

        pub fn get_contact_numbers(&self) -> &Vec<String> {
            &self.contact_numbers
        }

        pub fn get_emergency_services(&self) -> &Vec<String> {
            &self.emergency_services
        }

        pub fn get_medical_facilities(&self) -> &Vec<String> {
            &self.medical_facilities
        }

        pub fn get_evacuation_routes(&self) -> &Vec<String> {
            &self.evacuation_routes
        }
    }
}

#[cfg(test)]
mod tests {
    use super::travel_emergency_info::TravelEmergencyInfo;

    #[test]
    fn test_travel_emergency_info() {
        let mut info = TravelEmergencyInfo::new("New York");

        info.add_contact_number("123-456-7890");
        info.add_emergency_service("Fire Department");
        info.add_medical_facility("Hospital A");
        info.add_evacuation_route("Route 1");

        assert_eq!(info.get_location(), "New York");
        assert_eq!(info.get_contact_numbers().len(), 1);
        assert_eq!(info.get_emergency_services().len(), 1);
        assert_eq!(info.get_medical_facilities().len(), 1);
        assert_eq!(info.get_evacuation_routes().len(), 1);
    }
}

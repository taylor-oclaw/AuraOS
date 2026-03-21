extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    0
}

mod family_hub_health_record {
    use super::*;

    pub struct HealthRecord {
        name: String,
        age: u8,
        height: f32, // in meters
        weight: f32, // in kilograms
        medical_history: Vec<String>,
    }

    impl HealthRecord {
        pub fn new(name: &str, age: u8, height: f32, weight: f32) -> Self {
            HealthRecord {
                name: String::from(name),
                age,
                height,
                weight,
                medical_history: Vec::new(),
            }
        }

        pub fn add_medical_condition(&mut self, condition: &str) {
            self.medical_history.push(String::from(condition));
        }

        pub fn get_name(&self) -> &str {
            &self.name
        }

        pub fn get_age(&self) -> u8 {
            self.age
        }

        pub fn calculate_bmi(&self) -> f32 {
            if self.height == 0.0 {
                return 0.0; // Avoid division by zero
            }
            self.weight / (self.height * self.height)
        }

        pub fn get_medical_history(&self) -> &Vec<String> {
            &self.medical_history
        }
    }
}

#[cfg(test)]
mod tests {
    use super::family_hub_health_record::*;

    #[test]
    fn test_health_record() {
        let mut record = HealthRecord::new("Alice", 30, 1.65, 62.0);
        assert_eq!(record.get_name(), "Alice");
        assert_eq!(record.get_age(), 30);
        assert_eq!(record.calculate_bmi(), 22.038961);

        record.add_medical_condition("Allergies");
        assert_eq!(record.get_medical_history().len(), 1);
        assert_eq!(record.get_medical_history()[0], "Allergies");

        record.add_medical_condition("High Blood Pressure");
        assert_eq!(record.get_medical_history().len(), 2);
        assert_eq!(record.get_medical_history()[1], "High Blood Pressure");
    }
}

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

mod analytics_communication_pattern {
    use super::*;

    pub struct AnalyticsModule {
        data: Vec<String>,
        threshold: usize,
    }

    impl AnalyticsModule {
        pub fn new(threshold: usize) -> Self {
            AnalyticsModule {
                data: Vec::new(),
                threshold,
            }
        }

        pub fn add_data(&mut self, entry: String) {
            if self.data.len() < self.threshold {
                self.data.push(entry);
            } else {
                // Optionally handle overflow, e.g., log or drop the new entry
            }
        }

        pub fn get_data(&self) -> &Vec<String> {
            &self.data
        }

        pub fn clear_data(&mut self) {
            self.data.clear();
        }

        pub fn is_full(&self) -> bool {
            self.data.len() >= self.threshold
        }

        pub fn find_entry(&self, query: &str) -> Option<&String> {
            self.data.iter().find(|&&entry| entry.contains(query))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::analytics_communication_pattern::*;

    #[test]
    fn test_analytics_module() {
        let mut module = AnalyticsModule::new(3);
        assert!(!module.is_full());

        module.add_data(String::from("entry1"));
        module.add_data(String::from("entry2"));
        assert_eq!(module.get_data().len(), 2);

        module.add_data(String::from("entry3"));
        assert!(module.is_full());
        assert_eq!(module.get_data().len(), 3);

        let entry = module.find_entry("entry2");
        assert!(entry.is_some() && *entry.unwrap() == "entry2");

        module.clear_data();
        assert!(!module.is_full());
        assert_eq!(module.get_data().len(), 0);
    }
}

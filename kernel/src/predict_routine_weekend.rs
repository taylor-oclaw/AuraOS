extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub extern "C" fn predict_routine_weekend_init() {
    // Initialization logic for the module
}

pub extern "C" fn predict_routine_weekend_exit() {
    // Cleanup logic for the module
}

pub struct PredictRoutineWeekend {
    activities: Vec<String>,
    day_of_week: String,
}

impl PredictRoutineWeekend {
    pub fn new(day_of_week: &str) -> Self {
        PredictRoutineWeekend {
            activities: Vec::new(),
            day_of_week: String::from(day_of_week),
        }
    }

    pub fn add_activity(&mut self, activity: &str) {
        self.activities.push(String::from(activity));
    }

    pub fn remove_activity(&mut self, activity: &str) -> bool {
        if let Some(index) = self.activities.iter().position(|a| a == activity) {
            self.activities.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_activities(&self) -> Vec<String> {
        self.activities.clone()
    }

    pub fn is_weekend(&self) -> bool {
        self.day_of_week.to_lowercase() == "saturday" || self.day_of_week.to_lowercase() == "sunday"
    }

    pub fn predict_routine(&self) -> String {
        if self.is_weekend() {
            String::from("info")
        } else {
            String::from("Not a weekend")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_predict_routine_weekend() {
        let mut prw = PredictRoutineWeekend::new("Saturday");
        prw.add_activity("Sleep in");
        prw.add_activity("Have breakfast");
        prw.add_activity("Go for a walk");

        assert_eq!(prw.get_activities(), vec!["Sleep in", "Have breakfast", "Go for a walk"]);
        assert!(prw.remove_activity("Have breakfast"));
        assert_eq!(prw.get_activities(), vec!["Sleep in", "Go for a walk"]);
        assert_eq!(prw.predict_routine(), "Weekend routine: [\"Sleep in\", \"Go for a walk\"]");
    }
}

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
    let mut routine = predict_routine_morning::PredictRoutineMorning::new();
    routine.add_activity("Wake up at 6:00 AM");
    routine.add_activity("Drink a glass of water");
    routine.add_activity("Brush teeth and wash face");
    routine.add_activity("Prepare breakfast");
    routine.add_activity("Exercise for 30 minutes");

    let activities = routine.get_activities();
    for activity in activities {
        // Simulate logging or processing each activity
        println!("{}", activity);
    }
}

mod predict_routine_morning {
    use super::*;

    pub struct PredictRoutineMorning {
        activities: Vec<String>,
    }

    impl PredictRoutineMorning {
        pub fn new() -> Self {
            PredictRoutineMorning {
                activities: Vec::new(),
            }
        }

        pub fn add_activity(&mut self, activity: &str) {
            self.activities.push(String::from(activity));
        }

        pub fn remove_activity(&mut self, index: usize) -> Option<String> {
            if index < self.activities.len() {
                Some(self.activities.remove(index))
            } else {
                None
            }
        }

        pub fn get_activities(&self) -> &Vec<String> {
            &self.activities
        }

        pub fn clear_activities(&mut self) {
            self.activities.clear();
        }

        pub fn has_activity(&self, activity: &str) -> bool {
            self.activities.contains(&String::from(activity))
        }
    }
}

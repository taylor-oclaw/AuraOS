extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn gift_team_milestone_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn gift_team_milestone_exit() {
    // Cleanup logic for the module
}

pub struct MilestoneTracker {
    milestones: Vec<String>,
    completed: usize,
}

impl MilestoneTracker {
    pub fn new() -> Self {
        MilestoneTracker {
            milestones: Vec::new(),
            completed: 0,
        }
    }

    pub fn add_milestone(&mut self, milestone: String) {
        self.milestones.push(milestone);
    }

    pub fn complete_next_milestone(&mut self) {
        if self.completed < self.milestones.len() {
            self.completed += 1;
        }
    }

    pub fn get_completed_count(&self) -> usize {
        self.completed
    }

    pub fn get_total_milestones(&self) -> usize {
        self.milestones.len()
    }

    pub fn list_milestones(&self) -> Vec<String> {
        self.milestones.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_milestone_tracker() {
        let mut tracker = MilestoneTracker::new();
        assert_eq!(tracker.get_completed_count(), 0);
        assert_eq!(tracker.get_total_milestones(), 0);

        tracker.add_milestone(String::from("Design"));
        tracker.add_milestone(String::from("Implementation"));
        assert_eq!(tracker.get_total_milestones(), 2);

        tracker.complete_next_milestone();
        assert_eq!(tracker.get_completed_count(), 1);

        tracker.complete_next_milestone();
        assert_eq!(tracker.get_completed_count(), 2);

        let milestones = tracker.list_milestones();
        assert_eq!(milestones.len(), 2);
        assert_eq!(milestones[0], "Design");
        assert_eq!(milestones[1], "Implementation");
    }
}

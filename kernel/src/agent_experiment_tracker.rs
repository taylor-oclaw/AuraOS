extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

struct ExperimentTracker {
    experiments: Vec<String>,
}

impl ExperimentTracker {
    pub fn new() -> Self {
        ExperimentTracker {
            experiments: Vec::new(),
        }
    }

    pub fn add_experiment(&mut self, experiment_name: &str) {
        self.experiments.push(String::from(experiment_name));
    }

    pub fn remove_experiment(&mut self, experiment_name: &str) {
        if let Some(index) = self.experiments.iter().position(|e| e == experiment_name) {
            self.experiments.remove(index);
        }
    }

    pub fn list_experiments(&self) -> Vec<String> {
        self.experiments.clone()
    }

    pub fn has_experiment(&self, experiment_name: &str) -> bool {
        self.experiments.contains(&String::from(experiment_name))
    }

    pub fn count_experiments(&self) -> usize {
        self.experiments.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_experiment_tracker() {
        let mut tracker = ExperimentTracker::new();
        assert_eq!(tracker.count_experiments(), 0);

        tracker.add_experiment("Experiment1");
        assert_eq!(tracker.count_experiments(), 1);
        assert!(tracker.has_experiment("Experiment1"));

        tracker.add_experiment("Experiment2");
        assert_eq!(tracker.count_experiments(), 2);
        assert!(tracker.has_experiment("Experiment2"));

        let experiments = tracker.list_experiments();
        assert_eq!(experiments.len(), 2);
        assert!(experiments.contains(&String::from("Experiment1")));
        assert!(experiments.contains(&String::from("Experiment2")));

        tracker.remove_experiment("Experiment1");
        assert_eq!(tracker.count_experiments(), 1);
        assert!(!tracker.has_experiment("Experiment1"));

        tracker.remove_experiment("Experiment2");
        assert_eq!(tracker.count_experiments(), 0);
        assert!(!tracker.has_experiment("Experiment2"));
    }
}

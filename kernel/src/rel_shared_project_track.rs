extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub extern "C" fn rel_shared_project_track_init() {
    // Initialization logic for the module
}

pub extern "C" fn rel_shared_project_track_exit() {
    // Cleanup logic for the module
}

pub struct ProjectTracker {
    projects: Vec<String>,
}

impl ProjectTracker {
    pub fn new() -> Self {
        ProjectTracker {
            projects: Vec::new(),
        }
    }

    pub fn add_project(&mut self, project_name: &str) {
        self.projects.push(String::from(project_name));
    }

    pub fn remove_project(&mut self, project_name: &str) {
        if let Some(index) = self.projects.iter().position(|p| p == project_name) {
            self.projects.remove(index);
        }
    }

    pub fn list_projects(&self) -> Vec<String> {
        self.projects.clone()
    }

    pub fn has_project(&self, project_name: &str) -> bool {
        self.projects.contains(&String::from(project_name))
    }

    pub fn count_projects(&self) -> usize {
        self.projects.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_tracker() {
        let mut tracker = ProjectTracker::new();
        assert_eq!(tracker.count_projects(), 0);

        tracker.add_project("Project A");
        tracker.add_project("Project B");
        assert_eq!(tracker.count_projects(), 2);
        assert!(tracker.has_project("Project A"));
        assert!(tracker.has_project("Project B"));

        let projects = tracker.list_projects();
        assert_eq!(projects, vec![String::from("Project A"), String::from("Project B")]);

        tracker.remove_project("Project A");
        assert_eq!(tracker.count_projects(), 1);
        assert!(!tracker.has_project("Project A"));
        assert!(tracker.has_project("Project B"));

        tracker.remove_project("Project B");
        assert_eq!(tracker.count_projects(), 0);
        assert!(!tracker.has_project("Project B"));
    }
}

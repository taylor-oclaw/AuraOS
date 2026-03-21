extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct SmartHomeRoutine {
    name: String,
    steps: Vec<String>,
}

impl SmartHomeRoutine {
    pub fn new(name: &str) -> Self {
        SmartHomeRoutine {
            name: String::from(name),
            steps: Vec::new(),
        }
    }

    pub fn add_step(&mut self, step: &str) {
        self.steps.push(String::from(step));
    }

    pub fn remove_step(&mut self, index: usize) -> Option<String> {
        if index < self.steps.len() {
            Some(self.steps.remove(index))
        } else {
            None
        }
    }

    pub fn get_steps(&self) -> &Vec<String> {
        &self.steps
    }

    pub fn clear_steps(&mut self) {
        self.steps.clear();
    }

    pub fn execute_routine(&self) -> String {
        let mut result = String::from("info");
        for (index, step) in self.steps.iter().enumerate() {
            result.push_str(&String::from("info"));
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smart_home_routine() {
        let mut routine = SmartHomeRoutine::new("Morning Routine");
        routine.add_step("Turn on lights");
        routine.add_step("Make coffee");
        routine.add_step("Read the news");

        assert_eq!(routine.get_steps().len(), 3);
        assert_eq!(routine.execute_routine(), "Executing routine: Morning Routine\nStep 1: Turn on lights\nStep 2: Make coffee\nStep 3: Read the news");

        let removed_step = routine.remove_step(1).unwrap();
        assert_eq!(removed_step, "Make coffee");
        assert_eq!(routine.get_steps().len(), 2);

        routine.clear_steps();
        assert_eq!(routine.get_steps().len(), 0);
    }
}

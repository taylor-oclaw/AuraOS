extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct AutoTaskProgressReport {
    task_name: String,
    total_steps: usize,
    completed_steps: usize,
    start_time: u64,
    end_time: Option<u64>,
}

impl AutoTaskProgressReport {
    pub fn new(task_name: &str, total_steps: usize) -> Self {
        AutoTaskProgressReport {
            task_name: String::from(task_name),
            total_steps,
            completed_steps: 0,
            start_time: get_current_time(),
            end_time: None,
        }
    }

    pub fn complete_step(&mut self) {
        if self.completed_steps < self.total_steps {
            self.completed_steps += 1;
        }
    }

    pub fn is_completed(&self) -> bool {
        self.completed_steps == self.total_steps
    }

    pub fn get_progress_percentage(&self) -> usize {
        (self.completed_steps * 100 / self.total_steps).min(100)
    }

    pub fn finish_task(&mut self) {
        if !self.is_completed() {
            self.completed_steps = self.total_steps;
        }
        self.end_time = Some(get_current_time());
    }

    pub fn get_duration(&self) -> Option<u64> {
        match (self.start_time, self.end_time) {
            (start, Some(end)) => Some(end - start),
            _ => None,
        }
    }
}

fn get_current_time() -> u64 {
    // Placeholder for actual time retrieval logic
    0
}

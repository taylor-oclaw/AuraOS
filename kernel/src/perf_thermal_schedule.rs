extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    0
}

struct PerfThermalSchedule {
    tasks: Vec<String>,
    temperatures: Vec<u32>,
    thresholds: Vec<u32>,
    current_temp: u32,
    max_tasks: usize,
}

impl PerfThermalSchedule {
    pub fn new(max_tasks: usize) -> Self {
        PerfThermalSchedule {
            tasks: Vec::new(),
            temperatures: Vec::new(),
            thresholds: Vec::new(),
            current_temp: 0,
            max_tasks,
        }
    }

    pub fn add_task(&mut self, task_name: &str) {
        if self.tasks.len() < self.max_tasks {
            self.tasks.push(String::from(task_name));
        }
    }

    pub fn remove_task(&mut self, task_name: &str) {
        if let Some(index) = self.tasks.iter().position(|t| t == task_name) {
            self.tasks.remove(index);
        }
    }

    pub fn update_temperature(&mut self, temp: u32) {
        self.current_temp = temp;
    }

    pub fn add_threshold(&mut self, threshold: u32) {
        self.thresholds.push(threshold);
    }

    pub fn check_and_schedule(&self) -> Vec<String> {
        let mut scheduled_tasks = Vec::new();
        for &threshold in &self.thresholds {
            if self.current_temp > threshold {
                scheduled_tasks.extend(self.tasks.iter().cloned());
                break;
            }
        }
        scheduled_tasks
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perf_thermal_schedule() {
        let mut schedule = PerfThermalSchedule::new(3);
        schedule.add_task("task1");
        schedule.add_task("task2");
        schedule.add_threshold(50);

        assert_eq!(schedule.tasks.len(), 2);
        assert_eq!(schedule.thresholds.len(), 1);

        schedule.update_temperature(60);
        let scheduled_tasks = schedule.check_and_schedule();
        assert_eq!(scheduled_tasks, vec!["task1", "task2"]);

        schedule.remove_task("task1");
        assert_eq!(schedule.tasks.len(), 1);
    }
}

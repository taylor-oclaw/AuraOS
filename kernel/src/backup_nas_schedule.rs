extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
    let schedule = BackupNASSchedule::new();
    schedule.add_task("backup1", 8, 30);
    schedule.add_task("backup2", 14, 0);
    schedule.remove_task("backup1");
    schedule.list_tasks();
}

pub struct BackupNASSchedule {
    tasks: Vec<Task>,
}

impl BackupNASSchedule {
    pub fn new() -> Self {
        BackupNASSchedule { tasks: Vec::new() }
    }

    pub fn add_task(&mut self, name: &str, hour: u8, minute: u8) {
        if hour < 24 && minute < 60 {
            let task = Task {
                name: String::from(name),
                time: Time { hour, minute },
            };
            self.tasks.push(task);
        }
    }

    pub fn remove_task(&mut self, name: &str) {
        self.tasks.retain(|task| task.name != name);
    }

    pub fn list_tasks(&self) -> Vec<String> {
        let mut task_names = Vec::new();
        for task in &self.tasks {
            task_names.push(format!("{} at {}:{}", task.name, task.time.hour, task.time.minute));
        }
        task_names
    }

    pub fn get_task_by_name(&self, name: &str) -> Option<&Task> {
        self.tasks.iter().find(|task| task.name == name)
    }

    pub fn is_task_scheduled_at(&self, hour: u8, minute: u8) -> bool {
        self.tasks.iter().any(|task| task.time.hour == hour && task.time.minute == minute)
    }
}

struct Task {
    name: String,
    time: Time,
}

struct Time {
    hour: u8,
    minute: u8,
}
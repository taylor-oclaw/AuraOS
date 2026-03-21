extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> ! {
    let mut task = AutoTaskTestRun::new();
    task.initialize();
    task.run_tasks();
    loop {}
}

struct Task {
    name: String,
    status: String,
}

impl Task {
    fn new(name: &str) -> Self {
        Task {
            name: String::from(name),
            status: String::from("Pending"),
        }
    }

    fn mark_as_completed(&mut self) {
        self.status = String::from("Completed");
    }

    fn is_completed(&self) -> bool {
        self.status == "Completed"
    }

    fn get_status(&self) -> &str {
        &self.status
    }

    fn set_name(&mut self, name: &str) {
        self.name = String::from(name);
    }
}

pub struct AutoTaskTestRun {
    tasks: Vec<Task>,
}

impl AutoTaskTestRun {
    pub fn new() -> Self {
        AutoTaskTestRun { tasks: Vec::new() }
    }

    pub fn add_task(&mut self, name: &str) {
        let task = Task::new(name);
        self.tasks.push(task);
    }

    pub fn complete_task(&mut self, index: usize) {
        if let Some(task) = self.tasks.get_mut(index) {
            task.mark_as_completed();
        }
    }

    pub fn get_task_status(&self, index: usize) -> Option<&str> {
        self.tasks.get(index).map(|task| task.get_status())
    }

    pub fn list_tasks(&self) -> Vec<String> {
        self.tasks.iter().map(|task| String::from("info")).collect()
    }

    pub fn initialize(&mut self) {
        self.add_task("Task 1");
        self.add_task("Task 2");
        self.add_task("Task 3");
    }

    pub fn run_tasks(&self) {
        for (index, task) in self.tasks.iter().enumerate() {
            if !task.is_completed() {
                // Simulate task execution
                // Mark as completed after running
                let mut tasks = self.tasks.clone();
                tasks[index].mark_as_completed();
                self.tasks = tasks;
            }
        }
    }
}

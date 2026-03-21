extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_std]
mod auto_task_planner {
    use core::cmp::Ordering;

    pub struct Task {
        id: u32,
        name: String,
        priority: u8,
        duration: u32,
        completed: bool,
    }

    impl Task {
        pub fn new(id: u32, name: &str, priority: u8, duration: u32) -> Self {
            Task {
                id,
                name: String::from(name),
                priority,
                duration,
                completed: false,
            }
        }

        pub fn get_id(&self) -> u32 {
            self.id
        }

        pub fn get_name(&self) -> &str {
            &self.name
        }

        pub fn get_priority(&self) -> u8 {
            self.priority
        }

        pub fn get_duration(&self) -> u32 {
            self.duration
        }

        pub fn is_completed(&self) -> bool {
            self.completed
        }

        pub fn mark_as_completed(&mut self) {
            self.completed = true;
        }
    }

    pub struct TaskPlanner {
        tasks: Vec<Task>,
    }

    impl TaskPlanner {
        pub fn new() -> Self {
            TaskPlanner { tasks: Vec::new() }
        }

        pub fn add_task(&mut self, task: Task) {
            self.tasks.push(task);
        }

        pub fn remove_task(&mut self, id: u32) {
            self.tasks.retain(|t| t.get_id() != id);
        }

        pub fn get_tasks_by_priority(&self, priority: u8) -> Vec<&Task> {
            self.tasks.iter().filter(|t| t.get_priority() == priority).collect()
        }

        pub fn get_longest_task(&self) -> Option<&Task> {
            self.tasks.iter().max_by_key(|t| t.get_duration())
        }

        pub fn mark_task_as_completed(&mut self, id: u32) {
            if let Some(task) = self.tasks.iter_mut().find(|t| t.get_id() == id) {
                task.mark_as_completed();
            }
        }
    }
}

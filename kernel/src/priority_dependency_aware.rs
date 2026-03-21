extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[repr(C)]
pub struct PriorityDependencyAware {
    tasks: Vec<Task>,
}

impl PriorityDependencyAware {
    pub fn new() -> Self {
        PriorityDependencyAware {
            tasks: Vec::new(),
        }
    }

    pub fn add_task(&mut self, name: String, priority: u32, dependencies: Vec<String>) {
        let task = Task {
            name,
            priority,
            dependencies,
            status: TaskStatus::Pending,
        };
        self.tasks.push(task);
    }

    pub fn get_tasks_by_priority(&self, priority: u32) -> Vec<&Task> {
        self.tasks.iter().filter(|t| t.priority == priority).collect()
    }

    pub fn mark_task_completed(&mut self, task_name: &str) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.name == task_name) {
            task.status = TaskStatus::Completed;
        }
    }

    pub fn get_pending_tasks(&self) -> Vec<&Task> {
        self.tasks.iter().filter(|t| t.status == TaskStatus::Pending).collect()
    }

    pub fn resolve_dependencies(&mut self, task_name: &str) -> bool {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.name == task_name) {
            for dependency in &task.dependencies {
                if !self.is_task_completed(dependency) {
                    return false;
                }
            }
            true
        } else {
            false
        }
    }

    fn is_task_completed(&self, task_name: &str) -> bool {
        self.tasks.iter().any(|t| t.name == task_name && t.status == TaskStatus::Completed)
    }
}

#[derive(Debug)]
struct Task {
    name: String,
    priority: u32,
    dependencies: Vec<String>,
    status: TaskStatus,
}

#[derive(Debug, PartialEq)]
enum TaskStatus {
    Pending,
    Completed,
}

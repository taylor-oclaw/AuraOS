//! Cooperative Task Scheduler
//! Simple round-robin scheduler for AuraOS services.

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskState {
    Ready,
    Running,
    Blocked,
    Finished,
}

pub struct Task {
    pub id: u64,
    pub name: String,
    pub state: TaskState,
    pub priority: u8,
}

pub struct Scheduler {
    tasks: Vec<Task>,
    current: usize,
    next_id: u64,
}

impl Scheduler {
    pub fn new() -> Self {
        Scheduler {
            tasks: Vec::new(),
            current: 0,
            next_id: 1,
        }
    }

    pub fn spawn(&mut self, name: &str, priority: u8) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.tasks.push(Task {
            id,
            name: String::from(name),
            state: TaskState::Ready,
            priority,
        });
        id
    }

    pub fn kill(&mut self, id: u64) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.state = TaskState::Finished;
        }
    }

    pub fn block(&mut self, id: u64) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.state = TaskState::Blocked;
        }
    }

    pub fn unblock(&mut self, id: u64) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            if task.state == TaskState::Blocked {
                task.state = TaskState::Ready;
            }
        }
    }

    /// Pick the next ready task (highest priority, round-robin within same priority)
    pub fn next_task(&mut self) -> Option<u64> {
        // Clean up finished tasks
        self.tasks.retain(|t| t.state != TaskState::Finished);

        if self.tasks.is_empty() { return None; }

        // Find highest priority ready task
        let max_priority = self.tasks.iter()
            .filter(|t| t.state == TaskState::Ready)
            .map(|t| t.priority)
            .max();

        if let Some(prio) = max_priority {
            // Round-robin among tasks with this priority
            let start = self.current % self.tasks.len();
            for i in 0..self.tasks.len() {
                let idx = (start + i) % self.tasks.len();
                if self.tasks[idx].state == TaskState::Ready && self.tasks[idx].priority == prio {
                    self.tasks[idx].state = TaskState::Running;
                    self.current = idx + 1;
                    return Some(self.tasks[idx].id);
                }
            }
        }

        None
    }

    pub fn finish_current(&mut self, id: u64) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            if task.state == TaskState::Running {
                task.state = TaskState::Ready;
            }
        }
    }

    pub fn task_count(&self) -> usize {
        self.tasks.iter().filter(|t| t.state != TaskState::Finished).count()
    }

    pub fn list(&self) -> &[Task] {
        &self.tasks
    }
}

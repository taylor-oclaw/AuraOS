extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub enum TaskPriority {
    Critical,
    High,
    Normal,
    Low,
    Background,
}

pub enum TaskState {
    Queued,
    Assigned(u64),
    Running,
    Completed,
    Failed,
    Cancelled,
}

pub struct Task {
    pub id: u64,
    pub name: String,
    pub priority: TaskPriority,
    pub state: TaskState,
    pub created_by: u64,
    pub assigned_to: Option<u64>,
    pub payload: String,
    pub result: Option<String>,
    pub retry_count: u8,
    pub max_retries: u8,
}

pub struct TaskQueue {
    pub tasks: Vec<Task>,
    pub next_id: u64,
    pub max_queue_size: usize,
}

impl TaskQueue {
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
            next_id: 1,
            max_queue_size: 10000,
        }
    }

    pub fn enqueue(&mut self, name: &str, priority: TaskPriority, creator: u64, payload: &str) -> Option<u64> {
        if self.tasks.len() >= self.max_queue_size {
            return None;
        }
        let id = self.next_id;
        self.next_id += 1;
        self.tasks.push(Task {
            id,
            name: String::from(name),
            priority,
            state: TaskState::Queued,
            created_by: creator,
            assigned_to: None,
            payload: String::from(payload),
            result: None,
            retry_count: 0,
            max_retries: 3,
        };
        Some(id)
    }

    pub fn dequeue_highest(&mut self) -> Option<u64> {
        let idx = self.tasks.iter().position(|t| matches!(t.state, TaskState::Queued) && matches!(t.priority, TaskPriority::Critical));
        let idx = idx.or_else(|| self.tasks.iter().position(|t| matches!(t.state, TaskState::Queued) && matches!(t.priority, TaskPriority::High)));
        let idx = idx.or_else(|| self.tasks.iter().position(|t| matches!(t.state, TaskState::Queued)));
        idx.map(|i| self.tasks[i].id)
    }

    pub fn assign(&mut self, task_id: u64, agent_id: u64) {
        if let Some(t) = self.tasks.iter_mut().find(|t| t.id == task_id) {
            t.state = TaskState::Assigned(agent_id);
            t.assigned_to = Some(agent_id);
        }
    }

    pub fn complete(&mut self, task_id: u64, result: &str) {
        if let Some(t) = self.tasks.iter_mut().find(|t| t.id == task_id) {
            t.state = TaskState::Completed;
            t.result = Some(String::from(result));
        }
    }

    pub fn fail(&mut self, task_id: u64) {
        if let Some(t) = self.tasks.iter_mut().find(|t| t.id == task_id) {
            t.retry_count += 1;
            if t.retry_count >= t.max_retries {
                t.state = TaskState::Failed;
            } else {
                t.state = TaskState::Queued;
                t.assigned_to = None;
            }
        }
    }

    pub fn pending_count(&self) -> usize {
        self.tasks.iter().filter(|t| matches!(t.state, TaskState::Queued)).count()
    }

    pub fn total(&self) -> usize {
        self.tasks.len()
    }
)}

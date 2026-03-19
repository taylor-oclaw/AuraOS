extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub type AgentId = u64;
pub type TaskId = u64;

pub enum TaskStatus {
    Pending,
    Assigned(AgentId),
    InProgress(u8),
    Blocked(String),
    Complete,
    Failed(String),
}

pub struct Task {
    pub id: TaskId,
    pub description: String,
    pub assigned_to: Option<AgentId>,
    pub status: TaskStatus,
    pub priority: u8,
    pub cost_budget: u64,
    pub actual_cost: u64,
    pub dependencies: Vec<TaskId>,
    pub parent_intent: Option<String>,
    pub progress_pct: u8,
}

pub struct AgentCapability {
    pub agent_id: AgentId,
    pub name: String,
    pub cost_per_ms: u64,
    pub available: bool,
}

pub struct StatusReport {
    pub total_tasks: usize,
    pub completed: usize,
    pub in_progress: usize,
    pub blocked: usize,
    pub eta_ms: u64,
}

pub struct KernelOrchestrator {
    pub tasks: Vec<Task>,
    pub capabilities: Vec<AgentCapability>,
    pub next_task_id: TaskId,
}

impl KernelOrchestrator {
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
            capabilities: Vec::new(),
            next_task_id: 1,
        }
    }

    pub fn register_capability(&mut self, agent_id: AgentId, name: &str, cost: u64) {
        self.capabilities.push(AgentCapability {
            agent_id,
            name: String::from(name),
            cost_per_ms: cost,
            available: true,
        });
    }

    pub fn decompose_intent(&mut self, intent: &str) -> Vec<TaskId> {
        let words: Vec<&str> = intent.split(32 as char).collect();
        let mut ids = Vec::new();
        for chunk in words.chunks(3) {
            let desc = chunk.join(" ");
            let id = self.create_task(&desc, 5, 1000, &[]);
            ids.push(id);
        }
        ids
    }

    pub fn create_task(&mut self, desc: &str, priority: u8, budget: u64, deps: &[TaskId]) -> TaskId {
        let id = self.next_task_id;
        self.next_task_id += 1;
        self.tasks.push(Task {
            id,
            description: String::from(desc),
            assigned_to: None,
            status: TaskStatus::Pending,
            priority,
            cost_budget: budget,
            actual_cost: 0,
            dependencies: deps.to_vec(),
            parent_intent: None,
            progress_pct: 0,
        });
        id
    }

    pub fn assign_task(&mut self, task_id: TaskId) -> Option<AgentId> {
        let deps_met = self.tasks.iter().find(|t| t.id == task_id).map(|t| t.dependencies.iter().all(|d| self.tasks.iter().any(|t2| t2.id == *d && matches!(t2.status, TaskStatus::Complete)))).unwrap_or(false);
        if !deps_met {
            return None;
        }
        let best = self.capabilities.iter().find(|c| c.available);
        if let Some(cap) = best {
            let agent_id = cap.agent_id;
            if let Some(task) = self.tasks.iter_mut().find(|t| t.id == task_id) {
                task.assigned_to = Some(agent_id);
                task.status = TaskStatus::Assigned(agent_id);
            }
            Some(agent_id)
        } else {
            None
        }
    }

    pub fn update_progress(&mut self, task_id: TaskId, pct: u8) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == task_id) {
            task.progress_pct = pct;
            task.status = TaskStatus::InProgress(pct);
        }
    }

    pub fn complete_task(&mut self, task_id: TaskId, cost: u64) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == task_id) {
            task.status = TaskStatus::Complete;
            task.actual_cost = cost;
            task.progress_pct = 100;
            if let Some(aid) = task.assigned_to {
                if let Some(cap) = self.capabilities.iter_mut().find(|c| c.agent_id == aid) {
                    cap.available = true;
                }
            }
        }
    }

    pub fn status_report(&self) -> StatusReport {
        let completed = self.tasks.iter().filter(|t| matches!(t.status, TaskStatus::Complete)).count();
        let in_progress = self.tasks.iter().filter(|t| matches!(t.status, TaskStatus::InProgress(_))).count();
        let blocked = self.tasks.iter().filter(|t| matches!(t.status, TaskStatus::Blocked(_))).count();
        StatusReport {
            total_tasks: self.tasks.len(),
            completed,
            in_progress,
            blocked,
            eta_ms: 0,
        }
    }
}

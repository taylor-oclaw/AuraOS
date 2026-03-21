extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub type AgentId = u64;

pub enum LlmTier {
    Tiny,
    Small,
    Medium,
    Large,
    Thinking,
}

pub struct AgentNode {
    pub id: AgentId,
    pub name: String,
    pub role: AgentRole,
    pub llm_tier: LlmTier,
    pub parent_id: Option<AgentId>,
    pub children: Vec<AgentId>,
    pub status: AgentStatus,
    pub task: Option<String>,
    pub cost_spent: u64,
    pub cost_budget: u64,
}

pub enum AgentRole {
    ProjectManager,
    ScrumMaster,
    Developer,
    Reviewer,
    Researcher,
    Utility,
}

pub enum AgentStatus {
    Idle,
    Working,
    Blocked(String),
    Done,
    Failed(String),
}

pub struct SprintTask {
    pub id: u64,
    pub description: String,
    pub assigned_to: Option<AgentId>,
    pub status: AgentStatus,
    pub story_points: u8,
    pub dependencies: Vec<u64>,
}

pub struct AgentHierarchy {
    pub agents: Vec<AgentNode>,
    pub sprint_tasks: Vec<SprintTask>,
    pub next_id: AgentId,
    pub next_task_id: u64,
    pub total_budget: u64,
}

impl AgentHierarchy {
    pub fn new(budget: u64) -> Self {
        let mut h = Self {
            agents: Vec::new(),
            sprint_tasks: Vec::new(),
            next_id: 1,
            next_task_id: 1,
            total_budget: budget,
        };
        let pm = h.spawn_agent("OS-PM", AgentRole::ProjectManager, LlmTier::Medium, None);
        let sm = h.spawn_agent("OS-ScrumMaster", AgentRole::ScrumMaster, LlmTier::Small, Some(pm));
        h.spawn_agent("Worker-1", AgentRole::Developer, LlmTier::Tiny, Some(sm));
        h.spawn_agent("Worker-2", AgentRole::Developer, LlmTier::Tiny, Some(sm));
        h.spawn_agent("Reviewer", AgentRole::Reviewer, LlmTier::Small, Some(sm));
        h
    }

    pub fn spawn_agent(&mut self, name: &str, role: AgentRole, tier: LlmTier, parent: Option<AgentId>) -> AgentId {
        let id = self.next_id;
        self.next_id += 1;
        self.agents.push(AgentNode {
            id,
            name: String::from(name),
            role,
            llm_tier: tier,
            parent_id: parent,
            children: Vec::new(),
            status: AgentStatus::Idle,
            task: None,
            cost_spent: 0,
            cost_budget: self.total_budget / 10,
        });
        if let Some(pid) = parent {
            if let Some(p) = self.agents.iter_mut().find(|a| a.id == pid) {
                p.children.push(id);
            }
        }
        id
    }

    pub fn create_sprint_task(&mut self, desc: &str, points: u8, deps: &[u64]) -> u64 {
        let id = self.next_task_id;
        self.next_task_id += 1;
        self.sprint_tasks.push(SprintTask {
            id,
            description: String::from(desc),
            assigned_to: None,
            status: AgentStatus::Idle,
            story_points: points,
            dependencies: deps.to_vec(),
        });
        id
    }

    pub fn assign_cheapest(&mut self, task_id: u64) -> Option<AgentId> {
        let idle_dev = self.agents.iter().find(|a| matches!(a.role, AgentRole::Developer) && matches!(a.status, AgentStatus::Idle)).map(|a| a.id);
        if let Some(aid) = idle_dev {
            if let Some(task) = self.sprint_tasks.iter_mut().find(|t| t.id == task_id) {
                task.assigned_to = Some(aid);
                task.status = AgentStatus::Working;
            }
            if let Some(agent) = self.agents.iter_mut().find(|a| a.id == aid) {
                agent.status = AgentStatus::Working;
            }
        }
        idle_dev
    }

    pub fn total_cost(&self) -> u64 {
        self.agents.iter().map(|a| a.cost_spent).sum()
    }

    pub fn over_budget(&self) -> bool {
        self.total_cost() > self.total_budget
    }
}

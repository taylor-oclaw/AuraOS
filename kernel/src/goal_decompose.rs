extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub enum GoalStatus {
    Pending,
    Decomposed,
    InProgress,
    Completed,
    Failed,
}

pub struct SubGoal {
    pub id: u64,
    pub description: String,
    pub assigned_to: Option<u64>,
    pub status: GoalStatus,
    pub llm_tier_needed: u8,
    pub estimated_cost: u64,
    pub dependencies: Vec<u64>,
}

pub struct Goal {
    pub id: u64,
    pub intent: String,
    pub sub_goals: Vec<SubGoal>,
    pub status: GoalStatus,
    pub total_cost: u64,
    pub requester_agent: u64,
}

pub struct GoalDecomposer {
    pub goals: Vec<Goal>,
    pub next_id: u64,
    pub next_sub_id: u64,
}

impl GoalDecomposer {
    pub fn new() -> Self {
        Self {
            goals: Vec::new(),
            next_id: 1,
            next_sub_id: 1,
        }
    }

    pub fn create_goal(&mut self, intent: &str, requester: u64) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.goals.push(Goal {
            id,
            intent: String::from(intent),
            sub_goals: Vec::new(),
            status: GoalStatus::Pending,
            total_cost: 0,
            requester_agent: requester,
        };
        id
    }

    pub fn add_sub_goal(
        &mut self,
        goal_id: u64,
        desc: &str,
        tier: u8,
        cost: u64,
        deps: Vec<u64>,
    ) -> u64 {
        let sub_id = self.next_sub_id;
        self.next_sub_id += 1;
        if let Some(g) = self.goals.iter_mut().find(|g| g.id == goal_id) {
            g.sub_goals.push(SubGoal {
                id: sub_id,
                description: String::from(desc),
                assigned_to: None,
                status: GoalStatus::Pending,
                llm_tier_needed: tier,
                estimated_cost: cost,
                dependencies: deps,
            };
            g.status = GoalStatus::Decomposed;
        }
        sub_id
    }

    pub fn assign_sub_goal(&mut self, goal_id: u64, sub_id: u64, agent_id: u64) {
        if let Some(g) = self.goals.iter_mut().find(|g| g.id == goal_id) {
            if let Some(sg) = g.sub_goals.iter_mut().find(|s| s.id == sub_id) {
                sg.assigned_to = Some(agent_id);
                sg.status = GoalStatus::InProgress;
            }
        }
    }

    pub fn complete_sub_goal(&mut self, goal_id: u64, sub_id: u64, cost: u64) {
        if let Some(g) = self.goals.iter_mut().find(|g| g.id == goal_id) {
            if let Some(sg) = g.sub_goals.iter_mut().find(|s| s.id == sub_id) {
                sg.status = GoalStatus::Completed;
            }
            g.total_cost += cost;
            if g.sub_goals.iter().all(|s| matches!(s.status, GoalStatus::Completed)) {
                g.status = GoalStatus::Completed;
            }
        }
    }

    pub fn pending_goals(&self) -> Vec<&Goal> {
        self.goals
            .iter()
            .filter(|g| !matches!(g.status, GoalStatus::Completed))
            .collect()
    }
))}

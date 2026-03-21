extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub enum StepStatus {
    Pending,
    Running,
    Completed,
    Failed(String),
    Skipped,
}

pub struct WorkflowStep {
    pub id: u64,
    pub name: String,
    pub agent_id: Option<u64>,
    pub status: StepStatus,
    pub depends_on: Vec<u64>,
    pub output: Option<String>,
    pub timeout_secs: u64,
}

pub struct Workflow {
    pub id: u64,
    pub name: String,
    pub steps: Vec<WorkflowStep>,
    pub created_by: u64,
    pub next_step_id: u64,
}

pub struct WorkflowEngine {
    pub workflows: Vec<Workflow>,
    pub next_id: u64,
}

impl WorkflowEngine {
    pub fn new() -> Self {
        Self {
            workflows: Vec::new(),
            next_id: 1,
        }
    }

    pub fn create(&mut self, name: &str, created_by: u64) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.workflows.push(Workflow {
            id,
            name: String::from(name),
            steps: Vec::new(),
            created_by,
            next_step_id: 1,
        });
        id
    }

    pub fn add_step(&mut self, wf_id: u64, name: &str, deps: Vec<u64>, timeout: u64) -> Option<u64> {
        if let Some(wf) = self.workflows.iter_mut().find(|w| w.id == wf_id) {
            let sid = wf.next_step_id;
            wf.next_step_id += 1;
            wf.steps.push(WorkflowStep {
                id: sid,
                name: String::from(name),
                agent_id: None,
                status: StepStatus::Pending,
                depends_on: deps,
                output: None,
                timeout_secs: timeout,
            });
            Some(sid)
        } else {
            None
        }
    }

    pub fn ready_steps(&self, wf_id: u64) -> Vec<u64> {
        if let Some(wf) = self.workflows.iter().find(|w| w.id == wf_id) {
            wf.steps.iter()
                .filter(|s| matches!(s.status, StepStatus::Pending)
                    && s.depends_on.iter().all(|dep| wf.steps.iter().any(|ds| ds.id == *dep && matches!(ds.status, StepStatus::Completed))))
                .map(|s| s.id)
                .collect()
        } else {
            Vec::new()
        }
    }

    pub fn complete_step(&mut self, wf_id: u64, step_id: u64, output: &str) {
        if let Some(wf) = self.workflows.iter_mut().find(|w| w.id == wf_id) {
            if let Some(s) = wf.steps.iter_mut().find(|s| s.id == step_id) {
                s.status = StepStatus::Completed;
                s.output = Some(String::from(output));
            }
        }
    }

    pub fn is_complete(&self, wf_id: u64) -> bool {
        self.workflows
            .iter()
            .find(|w| w.id == wf_id)
            .map(|wf| wf.steps.iter().all(|s| matches!(s.status, StepStatus::Completed | StepStatus::Skipped)))
            .unwrap_or(false)
    }
}

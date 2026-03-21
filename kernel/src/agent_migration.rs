extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub enum MigrationState {
    Idle,
    Preparing,
    Transferring,
    Restoring,
    Complete,
    Failed(String),
}

pub struct MigrationJob {
    pub id: u64,
    pub agent_id: u64,
    pub from_node: String,
    pub to_node: String,
    pub state: MigrationState,
    pub state_size_bytes: u64,
    pub progress_pct: u8,
    pub started_at: u64,
}

pub struct AgentMigration {
    pub jobs: Vec<MigrationJob>,
    pub next_id: u64,
    pub max_concurrent: usize,
}

impl AgentMigration {
    pub fn new() -> Self {
        Self {
            jobs: Vec::new(),
            next_id: 1,
            max_concurrent: 3,
        }
    }

    pub fn start_migration(&mut self, agent_id: u64, from: &str, to: &str, state_size: u64) -> Option<u64> {
        let active = self.jobs.iter().filter(|j| matches!(j.state, MigrationState::Transferring | MigrationState::Preparing)).count();
        if active >= self.max_concurrent {
            return None;
        }
        let id = self.next_id;
        self.next_id += 1;
        self.jobs.push(MigrationJob {
            id,
            agent_id,
            from_node: String::from(from),
            to_node: String::from(to),
            state: MigrationState::Preparing,
            state_size_bytes: state_size,
            progress_pct: 0,
            started_at: 0,
        };
        Some(id)
    }

    pub fn update_progress(&mut self, job_id: u64, pct: u8) {
        if let Some(j) = self.jobs.iter_mut().find(|j| j.id == job_id) {
            j.progress_pct = pct;
            if pct >= 100 {
                j.state = MigrationState::Complete;
            } else if pct > 0 {
                j.state = MigrationState::Transferring;
            }
        }
    }

    pub fn fail(&mut self, job_id: u64, reason: &str) {
        if let Some(j) = self.jobs.iter_mut().find(|j| j.id == job_id) {
            j.state = MigrationState::Failed(String::from(reason));
        }
    }

    pub fn active_migrations(&self) -> Vec<&MigrationJob> {
        self.jobs.iter().filter(|j| !matches!(j.state, MigrationState::Complete | MigrationState::Failed(_) | MigrationState::Idle)).collect()
    }

    pub fn completed_count(&self) -> usize {
        self.jobs.iter().filter(|j| matches!(j.state, MigrationState::Complete)).count()
    }
)}

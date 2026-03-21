extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub enum ProcessState {
    Ready,
    Running,
    Blocked,
    Sleeping(u64),
    Zombie,
    Terminated,
}

pub struct AgentProcess {
    pub pid: u64,
    pub parent_pid: Option<u64>,
    pub agent_id: u64,
    pub name: String,
    pub state: ProcessState,
    pub priority: u8,
    pub cpu_time_ms: u64,
    pub memory_bytes: u64,
    pub started_at: u64,
    pub exit_code: Option<i32>,
}

pub struct ProcessTable {
    pub processes: Vec<AgentProcess>,
    pub next_pid: u64,
    pub max_processes: usize,
}

impl ProcessTable {
    pub fn new() -> Self {
        Self {
            processes: Vec::new(),
            next_pid: 1,
            max_processes: 1024,
        }
    }

    pub fn spawn(&mut self, agent_id: u64, name: &str, parent: Option<u64>, priority: u8) -> Option<u64> {
        if self.processes.len() >= self.max_processes {
            return None;
        }
        let pid = self.next_pid;
        self.next_pid += 1;
        self.processes.push(AgentProcess {
            pid,
            parent_pid: parent,
            agent_id,
            name: String::from(name),
            state: ProcessState::Ready,
            priority,
            cpu_time_ms: 0,
            memory_bytes: 0,
            started_at: 0,
            exit_code: None,
        };
        Some(pid)
    }

    pub fn kill(&mut self, pid: u64, exit_code: i32) {
        if let Some(p) = self.processes.iter_mut().find(|p| p.pid == pid) {
            p.state = ProcessState::Terminated;
            p.exit_code = Some(exit_code);
        }
    }

    pub fn schedule_next(&self) -> Option<u64> {
        self.processes
            .iter()
            .filter(|p| matches!(p.state, ProcessState::Ready))
            .min_by_key(|p| p.priority)
            .map(|p| p.pid)
    }

    pub fn set_running(&mut self, pid: u64) {
        if let Some(p) = self.processes.iter_mut().find(|p| p.pid == pid) {
            p.state = ProcessState::Running;
        }
    }

    pub fn children(&self, pid: u64) -> Vec<u64> {
        self.processes
            .iter()
            .filter(|p| p.parent_pid == Some(pid))
            .map(|p| p.pid)
            .collect()
    }

    pub fn running_count(&self) -> usize {
        self.processes
            .iter()
            .filter(|p| matches!(p.state, ProcessState::Running))
            .count()
    }

    pub fn total(&self) -> usize {
        self.processes.len()
    }
)}

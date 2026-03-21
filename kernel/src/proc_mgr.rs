extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub type Pid = u32;

pub enum ProcessState {
    Ready,
    Running,
    Blocked,
    Sleeping(u64),
    Zombie,
    Stopped,
}

pub struct Process {
    pub pid: Pid,
    pub parent_pid: Pid,
    pub name: String,
    pub state: ProcessState,
    pub priority: u8,
    pub cpu_time_ms: u64,
    pub memory_kb: u64,
    pub open_fds: Vec<u32>,
    pub exit_code: Option<i32>,
    pub created_at: u64,
}

pub struct ProcessManager {
    pub processes: Vec<Process>,
    pub next_pid: Pid,
    pub current_pid: Option<Pid>,
}

impl ProcessManager {
    pub fn new() -> Self {
        let mut pm = Self {
            processes: Vec::new(),
            next_pid: 1,
            current_pid: None,
        };
        pm.create_process(0, "init", 0);
        pm
    }

    pub fn create_process(&mut self, parent: Pid, name: &str, priority: u8) -> Pid {
        let pid = self.next_pid;
        self.next_pid += 1;
        self.processes.push(Process {
            pid,
            parent_pid: parent,
            name: String::from(name),
            state: ProcessState::Ready,
            priority,
            cpu_time_ms: 0,
            memory_kb: 0,
            open_fds: Vec::new(),
            exit_code: None,
            created_at: 0,
        };
        pid
    }

    pub fn kill_process(&mut self, pid: Pid) -> bool {
        if let Some(p) = self.processes.iter_mut().find(|p| p.pid == pid) {
            p.state = ProcessState::Zombie;
            p.exit_code = Some(-9);
            true
        } else {
            false
        }
    }

    pub fn get_process(&self, pid: Pid) -> Option<&Process> {
        self.processes.iter().find(|p| p.pid == pid)
    }

    pub fn children_of(&self, pid: Pid) -> Vec<&Process> {
        self.processes.iter().filter(|p| p.parent_pid == pid).collect()
    }

    pub fn ready_processes(&self) -> Vec<&Process> {
        self.processes.iter().filter(|p| matches!(p.state, ProcessState::Ready)).collect()
    }

    pub fn reap_zombies(&mut self) {
        self.processes.retain(|p| !matches!(p.state, ProcessState::Zombie));
    }

    pub fn process_count(&self) -> usize {
        self.processes.len()
    }
)}

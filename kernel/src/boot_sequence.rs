extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub enum BootPhase {
    HardwareProbe,
    MemoryInit,
    FilesystemMount,
    AgentRegistryInit,
    OrchestratorStart,
    DisplayServerInit,
    EventLoopStart,
    Ready,
}

pub struct BootStep {
    pub phase: BootPhase,
    pub name: String,
    pub completed: bool,
    pub duration_ms: u64,
    pub error: Option<String>,
}

pub struct BootSequence {
    pub steps: Vec<BootStep>,
    pub current_phase: usize,
    pub boot_start: u64,
    pub total_ms: u64,
}

impl BootSequence {
    pub fn new() -> Self {
        let steps = vec![
            BootStep {
                phase: BootPhase::HardwareProbe,
                name: String::from("Probing hardware devices"),
                completed: false,
                duration_ms: 0,
                error: None,
            },
            BootStep {
                phase: BootPhase::MemoryInit,
                name: String::from("Initializing memory allocator"),
                completed: false,
                duration_ms: 0,
                error: None,
            },
            BootStep {
                phase: BootPhase::FilesystemMount,
                name: String::from("Mounting DEFS root filesystem"),
                completed: false,
                duration_ms: 0,
                error: None,
            },
            BootStep {
                phase: BootPhase::AgentRegistryInit,
                name: String::from("Starting agent registry"),
                completed: false,
                duration_ms: 0,
                error: None,
            },
            BootStep {
                phase: BootPhase::OrchestratorStart,
                name: String::from("Launching kernel orchestrator"),
                completed: false,
                duration_ms: 0,
                error: None,
            },
            BootStep {
                phase: BootPhase::DisplayServerInit,
                name: String::from("Initializing display server"),
                completed: false,
                duration_ms: 0,
                error: None,
            },
            BootStep {
                phase: BootPhase::EventLoopStart,
                name: String::from("Starting event loop"),
                completed: false,
                duration_ms: 0,
                error: None,
            },
            BootStep {
                phase: BootPhase::Ready,
                name: String::from("AuraOS ready"),
                completed: false,
                duration_ms: 0,
                error: None,
            },
        ];

        Self {
            steps,
            current_phase: 0,
            boot_start: 0,
            total_ms: 0,
        }
    }

    pub fn advance(&mut self, duration_ms: u64) -> bool {
        if self.current_phase < self.steps.len() {
            self.steps[self.current_phase].completed = true;
            self.steps[self.current_phase].duration_ms = duration_ms;
            self.total_ms += duration_ms;
            self.current_phase += 1;
            true
        } else {
            false
        }
    }

    pub fn fail_current(&mut self, error: &str) {
        if self.current_phase < self.steps.len() {
            self.steps[self.current_phase].error = Some(String::from(error));
        }
    }

    pub fn is_complete(&self) -> bool {
        self.current_phase >= self.steps.len()
    }

    pub fn current_step_name(&self) -> &str {
        if self.current_phase < self.steps.len() {
            &self.steps[self.current_phase].name
        } else {
            "Boot complete"
        }
    }

    pub fn progress_pct(&self) -> u8 {
        ((self.current_phase * 100) / self.steps.len()) as u8
    }
}

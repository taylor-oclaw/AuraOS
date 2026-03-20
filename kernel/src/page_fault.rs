extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub enum FaultType { Read, Write, Execute, NotPresent, ProtectionViolation }

pub struct PageFault {
    pub address: u64,
    pub fault_type: FaultType,
    pub agent_id: Option<u64>,
    pub resolved: bool,
}

pub struct PageFaultHandler {
    pub faults: Vec<PageFault>,
    pub total_faults: u64,
    pub total_resolved: u64,
    pub oom_kills: u64,
}

impl PageFaultHandler {
    pub fn new() -> Self {
        Self { faults: Vec::new(), total_faults: 0, total_resolved: 0, oom_kills: 0 }
    }

    pub fn handle_fault(&mut self, address: u64, fault_type: FaultType, agent_id: Option<u64>) -> bool {
        self.total_faults += 1;
        let resolved = !matches!(fault_type, FaultType::ProtectionViolation);
        self.faults.push(PageFault { address, fault_type, agent_id, resolved });
        if resolved { self.total_resolved += 1; }
        resolved
    }

    pub fn unresolved_faults(&self) -> Vec<&PageFault> {
        self.faults.iter().filter(|f| !f.resolved).collect()
    }

    pub fn faults_for_agent(&self, agent_id: u64) -> usize {
        self.faults.iter().filter(|f| f.agent_id == Some(agent_id)).count()
    }

    pub fn resolution_rate(&self) -> f32 {
        if self.total_faults == 0 { 1.0 }
        else { self.total_resolved as f32 / self.total_faults as f32 }
    }

    pub fn total(&self) -> u64 { self.total_faults }
}

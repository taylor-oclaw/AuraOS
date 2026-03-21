extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub enum InterruptType {
    Timer,
    Keyboard,
    Mouse,
    Network,
    Disk,
    Agent(u64),
    Custom(String)
}

pub struct InterruptHandler {
    pub irq: u8,
    pub name: String,
    pub handler_fn: String,
    pub enabled: bool,
    pub fire_count: u64,
    pub priority: u8
}

pub struct InterruptController {
    pub handlers: Vec<InterruptHandler>,
    pub pending: Vec<u8>,
    pub masked: Vec<u8>,
    pub total_interrupts: u64,
    pub nesting_depth: u8,
    pub max_nesting: u8
}

impl InterruptController {
    pub fn new() -> Self {
        Self {
            handlers: Vec::new(),
            pending: Vec::new(),
            masked: Vec::new(),
            total_interrupts: 0,
            nesting_depth: 0,
            max_nesting: 4
        }
    }

    pub fn register(&mut self, irq: u8, name: &str, handler: &str, priority: u8) {
        self.handlers.push(InterruptHandler {
            irq,
            name: String::from(name),
            handler_fn: String::from(handler),
            enabled: true,
            fire_count: 0,
            priority
        });
    }

    pub fn fire(&mut self, irq: u8) -> bool {
        if self.masked.contains(&irq) {
            return false;
        }
        if self.nesting_depth >= self.max_nesting {
            self.pending.push(irq);
            return false;
        }
        if let Some(h) = self.handlers.iter_mut().find(|h| h.irq == irq && h.enabled) {
            h.fire_count += 1;
            self.total_interrupts += 1;
            self.nesting_depth += 1;
            true
        } else {
            false
        }
    }

    pub fn complete(&mut self) {
        if self.nesting_depth > 0 {
            self.nesting_depth -= 1;
        }
    }

    pub fn mask(&mut self, irq: u8) {
        if !self.masked.contains(&irq) {
            self.masked.push(irq);
        }
    }

    pub fn unmask(&mut self, irq: u8) {
        self.masked.retain(|i| *i != irq);
    }

    pub fn drain_pending(&mut self) -> Vec<u8> {
        let p = self.pending.clone();
        self.pending.clear();
        p
    }

    pub fn handler_count(&self) -> usize {
        self.handlers.len()
    }
}

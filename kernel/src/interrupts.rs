extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct Interrupts {
    entries: Vec<String>,
    active: bool,
}

impl Interrupts {
    pub fn new() -> Self {
        Interrupts { entries: Vec::new(), active: true }
    }
    pub fn add(&mut self, entry: &str) { self.entries.push(String::from(entry)); }
    pub fn remove(&mut self, entry: &str) { self.entries.retain(|e| e != entry); }
    pub fn count(&self) -> usize { self.entries.len() }
    pub fn is_active(&self) -> bool { self.active }
}

pub fn init_idt() {
    // IDT initialization placeholder
}

pub static PICS: PicPlaceholder = PicPlaceholder;

pub struct PicPlaceholder;
impl PicPlaceholder {
    pub fn lock(&self) -> PicGuard { PicGuard }
}
pub struct PicGuard;
impl PicGuard {
    pub fn initialize(&self) {}
}

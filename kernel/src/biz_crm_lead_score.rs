extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

pub struct LeadQualify {
    leads: Vec<String>,
    qualified_leads: Vec<String>,
}

impl LeadQualify {
    pub fn new() -> Self {
        LeadQualify {
            leads: Vec::new(),
            qualified_leads: Vec::new(),
        }
    }

    pub fn add_lead(&mut self, lead: String) {
        self.leads.push(lead);
    }

    pub fn qualify_leads(&mut self) {
        for lead in self.leads.drain(..) {
            if self.is_qualified(&lead) {
                self.qualified_leads.push(lead);
            }
        }
    }

    fn is_qualified(&self, lead: &str) -> bool {
        // Simple qualification logic
        lead.contains("qualified")
    }

    pub fn get_qualified_leads(&self) -> &[String] {
        &self.qualified_leads
    }

    pub fn clear_qualified_leads(&mut self) {
        self.qualified_leads.clear();
    }
}

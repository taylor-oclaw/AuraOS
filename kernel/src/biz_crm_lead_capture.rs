extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

struct LeadCapture {
    leads: Vec<String>,
}

impl LeadCapture {
    pub fn new() -> Self {
        LeadCapture {
            leads: Vec::new(),
        }
    }

    pub fn add_lead(&mut self, lead: String) {
        self.leads.push(lead);
    }

    pub fn get_leads_count(&self) -> usize {
        self.leads.len()
    }

    pub fn get_lead_by_index(&self, index: usize) -> Option<&String> {
        self.leads.get(index)
    }

    pub fn remove_lead_by_index(&mut self, index: usize) -> Option<String> {
        if index < self.leads.len() {
            Some(self.leads.remove(index))
        } else {
            None
        }
    }

    pub fn clear_leads(&mut self) {
        self.leads.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lead_capture() {
        let mut lead_capture = LeadCapture::new();
        assert_eq!(lead_capture.get_leads_count(), 0);

        lead_capture.add_lead(String::from("John Doe"));
        lead_capture.add_lead(String::from("Jane Smith"));
        assert_eq!(lead_capture.get_leads_count(), 2);

        assert_eq!(lead_capture.get_lead_by_index(0), Some(&String::from("John Doe")));
        assert_eq!(lead_capture.get_lead_by_index(1), Some(&String::from("Jane Smith")));

        let removed_lead = lead_capture.remove_lead_by_index(0);
        assert_eq!(removed_lead, Some(String::from("John Doe")));
        assert_eq!(lead_capture.get_leads_count(), 1);

        lead_capture.clear_leads();
        assert_eq!(lead_capture.get_leads_count(), 0);
    }
}

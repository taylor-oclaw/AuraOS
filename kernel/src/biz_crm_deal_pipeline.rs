extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let mut deal_stage = DealStage::new();
    deal_stage.add_deal("Deal 1".into(), 1000);
    deal_stage.add_deal("Deal 2".into(), 2000);
    deal_stage.update_deal_value("Deal 1", 1500);
    let total_value = deal_stage.total_value();
    let average_value = deal_stage.average_value();
    unsafe { kernel_log!("Total Value: {}", total_value) };
    unsafe { kernel_log!("Average Value: {}", average_value) };
    loop {}
}

pub struct DealStage {
    deals: Vec<(String, u32)>,
}

impl DealStage {
    pub fn new() -> Self {
        DealStage { deals: Vec::new() }
    }

    pub fn add_deal(&mut self, name: String, value: u32) {
        self.deals.push((name, value));
    }

    pub fn update_deal_value(&mut self, name: &str, new_value: u32) {
        if let Some(deal) = self.deals.iter_mut().find(|d| d.0 == name) {
            deal.1 = new_value;
        }
    }

    pub fn total_value(&self) -> u32 {
        self.deals.iter().map(|(_, value)| value).sum()
    }

    pub fn average_value(&self) -> f32 {
        if self.deals.is_empty() {
            0.0
        } else {
            self.total_value() as f32 / self.deals.len() as f32
        }
    }

    pub fn remove_deal(&mut self, name: &str) {
        self.deals.retain(|d| d.0 != name);
    }
}

// Placeholder for kernel logging function
#[no_mangle]
pub extern "C" fn kernel_log(message: &str) {
    // Implement kernel logging here
}

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let schedule = BizVendorPaymentSchedule::new();
    schedule.add_payment("VendorA", 100);
    schedule.add_payment("VendorB", 200);
    schedule.remove_payment("VendorA");
    schedule.update_payment("VendorB", 300);
    let total = schedule.total_payments();
    // Do something with the total, e.g., log it or use it in kernel operations
    loop {}
}

pub struct BizVendorPaymentSchedule {
    payments: Vec<(String, u64)>,
}

impl BizVendorPaymentSchedule {
    pub fn new() -> Self {
        BizVendorPaymentSchedule {
            payments: Vec::new(),
        }
    }

    pub fn add_payment(&mut self, vendor: &str, amount: u64) {
        let vendor_name = String::from(vendor);
        self.payments.push((vendor_name, amount));
    }

    pub fn remove_payment(&mut self, vendor: &str) {
        self.payments.retain(|(v, _)| v != vendor);
    }

    pub fn update_payment(&mut self, vendor: &str, new_amount: u64) {
        if let Some(index) = self.payments.iter().position(|(v, _)| v == vendor) {
            self.payments[index].1 = new_amount;
        }
    }

    pub fn get_payment(&self, vendor: &str) -> Option<u64> {
        self.payments.iter().find_map(|(v, amount)| {
            if v == vendor {
                Some(*amount)
            } else {
                None
            }
        }
    }

    pub fn total_payments(&self) -> u64 {
        self.payments.iter().map(|(_, amount)| *amount).sum()
    }
}

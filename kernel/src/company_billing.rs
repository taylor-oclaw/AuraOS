extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let mut billing = CompanyBilling::new();
    billing.add_invoice("Alice", 100);
    billing.add_invoice("Bob", 200);
    billing.remove_invoice("Alice");
    println!("Total revenue: {}", billing.total_revenue());
    println!("Invoices count: {}", billing.invoices_count());

    loop {}
}

pub struct CompanyBilling {
    invoices: Vec<(String, u32)>,
}

impl CompanyBilling {
    pub fn new() -> Self {
        CompanyBilling {
            invoices: Vec::new(),
        }
    }

    pub fn add_invoice(&mut self, customer_name: &str, amount: u32) {
        let name = String::from(customer_name);
        self.invoices.push((name, amount));
    }

    pub fn remove_invoice(&mut self, customer_name: &str) {
        self.invoices.retain(|(name, _)| name != customer_name);
    }

    pub fn total_revenue(&self) -> u32 {
        self.invoices.iter().map(|(_, amount)| amount).sum()
    }

    pub fn invoices_count(&self) -> usize {
        self.invoices.len()
    }

    pub fn get_invoice_amount(&self, customer_name: &str) -> Option<u32> {
        self.invoices
            .iter()
            .find(|(name, _)| name == customer_name)
            .map(|(_, amount)| *amount)
    }
}

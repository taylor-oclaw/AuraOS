extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct FinanceBillRemind {
    bills: Vec<Bill>,
}

impl FinanceBillRemind {
    pub fn new() -> Self {
        FinanceBillRemind { bills: Vec::new() }
    }

    pub fn add_bill(&mut self, bill: Bill) {
        self.bills.push(bill);
    }

    pub fn remove_bill_by_id(&mut self, id: u32) {
        self.bills.retain(|b| b.id != id);
    }

    pub fn get_bill_by_id(&self, id: u32) -> Option<&Bill> {
        self.bills.iter().find(|b| b.id == id)
    }

    pub fn list_all_bills(&self) -> &Vec<Bill> {
        &self.bills
    }

    pub fn total_amount_due(&self) -> u32 {
        self.bills.iter().map(|b| b.amount).sum()
    }
}

#[derive(Debug)]
pub struct Bill {
    id: u32,
    description: String,
    amount: u32,
    due_date: String, // Assuming date is stored as a string for simplicity
}

impl Bill {
    pub fn new(id: u32, description: String, amount: u32, due_date: String) -> Self {
        Bill { id, description, amount, due_date }
    }
}

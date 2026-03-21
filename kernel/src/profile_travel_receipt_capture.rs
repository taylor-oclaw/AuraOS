extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct TravelReceipt {
    pub id: u32,
    pub destination: String,
    pub date: String,
    pub amount: u32,
}

impl TravelReceipt {
    pub fn new(id: u32, destination: &str, date: &str, amount: u32) -> Self {
        TravelReceipt {
            id,
            destination: String::from(destination),
            date: String::from(date),
            amount,
        }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_destination(&self) -> &str {
        &self.destination
    }

    pub fn get_date(&self) -> &str {
        &self.date
    }

    pub fn get_amount(&self) -> u32 {
        self.amount
    }

    pub fn update_amount(&mut self, new_amount: u32) {
        self.amount = new_amount;
    }
}

#[derive(Debug)]
pub struct TravelReceiptCapture {
    receipts: Vec<TravelReceipt>,
}

impl TravelReceiptCapture {
    pub fn new() -> Self {
        TravelReceiptCapture {
            receipts: Vec::new(),
        }
    }

    pub fn add_receipt(&mut self, receipt: TravelReceipt) {
        self.receipts.push(receipt);
    }

    pub fn get_all_receipts(&self) -> &Vec<TravelReceipt> {
        &self.receipts
    }

    pub fn find_receipt_by_id(&self, id: u32) -> Option<&TravelReceipt> {
        self.receipts.iter().find(|r| r.id == id)
    }

    pub fn total_spent(&self) -> u32 {
        self.receipts.iter().map(|r| r.amount).sum()
    }
}

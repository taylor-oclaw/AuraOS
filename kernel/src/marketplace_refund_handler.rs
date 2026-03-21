extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct MarketplaceRefundHandler {
    refunds: Vec<Refund>,
}

impl MarketplaceRefundHandler {
    pub fn new() -> Self {
        MarketplaceRefundHandler {
            refunds: Vec::new(),
        }
    }

    pub fn add_refund(&mut self, refund: Refund) {
        self.refunds.push(refund);
    }

    pub fn get_refund_by_id(&self, id: u32) -> Option<&Refund> {
        self.refunds.iter().find(|r| r.id == id)
    }

    pub fn remove_refund_by_id(&mut self, id: u32) -> bool {
        let pos = self.refunds.iter().position(|r| r.id == id);
        if let Some(index) = pos {
            self.refunds.remove(index);
            true
        } else {
            false
        }
    }

    pub fn list_all_refunds(&self) -> Vec<&Refund> {
        self.refunds.iter().collect()
    }

    pub fn count_refunds(&self) -> usize {
        self.refunds.len()
    }
}

pub struct Refund {
    id: u32,
    amount: u64,
    reason: String,
}

impl Refund {
    pub fn new(id: u32, amount: u64, reason: &str) -> Self {
        Refund {
            id,
            amount,
            reason: String::from(reason),
        }
    }
}

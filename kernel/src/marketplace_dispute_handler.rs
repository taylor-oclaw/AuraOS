extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

struct MarketplaceDisputeHandler {
    disputes: Vec<Dispute>,
}

impl MarketplaceDisputeHandler {
    pub fn new() -> Self {
        MarketplaceDisputeHandler {
            disputes: Vec::new(),
        }
    }

    pub fn add_dispute(&mut self, dispute: Dispute) {
        self.disputes.push(dispute);
    }

    pub fn get_dispute_count(&self) -> usize {
        self.disputes.len()
    }

    pub fn resolve_dispute(&mut self, dispute_id: u32) -> bool {
        if let Some(index) = self.disputes.iter().position(|d| d.id == dispute_id) {
            self.disputes.remove(index);
            true
        } else {
            false
        }
    }

    pub fn list_disputes(&self) -> Vec<Dispute> {
        self.disputes.clone()
    }
}

struct Dispute {
    id: u32,
    buyer_id: String,
    seller_id: String,
    description: String,
    status: String,
}

impl Dispute {
    pub fn new(id: u32, buyer_id: &str, seller_id: &str, description: &str) -> Self {
        Dispute {
            id,
            buyer_id: String::from(buyer_id),
            seller_id: String::from(seller_id),
            description: String::from(description),
            status: String::from("Open"),
        }
    }

    pub fn update_status(&mut self, new_status: &str) {
        self.status = String::from(new_status);
    }
}

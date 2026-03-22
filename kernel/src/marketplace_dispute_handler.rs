#![no_std]
#![feature(allocator_api)]

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use core::alloc::{AllocError, Allocator, Global};
use core::sync::atomic::{AtomicUsize, Ordering};

struct MarketplaceDispute {
    id: usize,
    description: String,
    status: DisputeStatus,
}

enum DisputeStatus {
    Open,
    Resolved,
    Closed,
}

impl MarketplaceDispute {
    pub fn new(id: usize, description: String) -> Self {
        MarketplaceDispute {
            id,
            description,
            status: DisputeStatus::Open,
        }
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }

    pub fn resolve_dispute(&mut self) {
        self.status = DisputeStatus::Resolved;
    }

    pub fn close_dispute(&mut self) {
        self.status = DisputeStatus::Closed;
    }
}

struct MarketplaceDisputeHandler {
    disputes: Vec<MarketplaceDispute>,
    next_id: AtomicUsize,
}

impl MarketplaceDisputeHandler {
    pub fn new() -> Self {
        MarketplaceDisputeHandler {
            disputes: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }

    pub fn create_dispute(&mut self, description: String) -> Result<usize, AllocError> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let dispute = MarketplaceDispute::new(id, description);
        self.disputes.push(dispute);
        Ok(id)
    }

    pub fn get_dispute(&self, id: usize) -> Option<&MarketplaceDispute> {
        self.disputes.iter().find(|d| d.id == id)
    }

    pub fn resolve_dispute(&mut self, id: usize) -> bool {
        if let Some(dispute) = self.disputes.iter_mut().find(|d| d.id == id) {
            dispute.resolve_dispute();
            true
        } else {
            false
        }
    }

    pub fn close_dispute(&mut self, id: usize) -> bool {
        if let Some(dispute) = self.disputes.iter_mut().find(|d| d.id == id) {
            dispute.close_dispute();
            true
        } else {
            false
        }
    }
}
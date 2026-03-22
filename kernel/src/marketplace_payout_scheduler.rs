#![no_std]
#![feature(allocator_api)]
#![feature(const_mut_refs)]

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use core::alloc::{AllocError, Allocator, Global};
use core::sync::atomic::{AtomicUsize, Ordering};

struct MarketplacePayoutScheduler {
    payouts: Vec<(String, u64)>,
    total_payouts: AtomicUsize,
}

impl MarketplacePayoutScheduler {
    pub fn new() -> Self {
        MarketplacePayoutScheduler {
            payouts: Vec::new(),
            total_payouts: AtomicUsize::new(0),
        }
    }

    pub fn add_payout(&self, user: String, amount: u64) -> Result<(), AllocError> {
        self.payouts.push((user, amount));
        self.total_payouts.fetch_add(amount as usize, Ordering::SeqCst);
        Ok(())
    }

    pub fn get_total_payouts(&self) -> usize {
        self.total_payouts.load(Ordering::SeqCst)
    }

    pub fn list_payouts(&self) -> Vec<&(String, u64)> {
        self.payouts.iter().collect()
    }

    pub fn remove_payout(&self, user: &str) -> Option<(String, u64)> {
        if let Some(index) = self.payouts.iter().position(|&(ref u, _)| u == user) {
            let (user, amount) = self.payouts.remove(index);
            self.total_payouts.fetch_sub(amount as usize, Ordering::SeqCst);
            Some((user, amount))
        } else {
            None
        }
    }
}

static mut SCHEDULER: MarketplacePayoutScheduler = MarketplacePayoutScheduler::new();

pub fn init_scheduler() {
    unsafe {
        SCHEDULER = MarketplacePayoutScheduler::new();
    }
}

pub fn add_payout(user: String, amount: u64) -> Result<(), AllocError> {
    unsafe { SCHEDULER.add_payout(user, amount) }
}

pub fn get_total_payouts() -> usize {
    unsafe { SCHEDULER.get_total_payouts() }
}

pub fn list_payouts() -> Vec<&(String, u64)> {
    unsafe { SCHEDULER.list_payouts() }
}

pub fn remove_payout(user: &str) -> Option<(String, u64)> {
    unsafe { SCHEDULER.remove_payout(user) }
}
extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[repr(C)]
pub struct FinanceSubscriptionDetect {
    subscriptions: Vec<String>,
}

impl FinanceSubscriptionDetect {
    pub fn new() -> Self {
        FinanceSubscriptionDetect {
            subscriptions: Vec::new(),
        }
    }

    pub fn add_subscription(&mut self, subscription: String) {
        if !self.subscriptions.contains(&subscription) {
            self.subscriptions.push(subscription);
        }
    }

    pub fn remove_subscription(&mut self, subscription: &str) -> bool {
        let index = self.subscriptions.iter().position(|s| s == subscription);
        match index {
            Some(i) => {
                self.subscriptions.remove(i);
                true
            }
            None => false,
        }
    }

    pub fn has_subscription(&self, subscription: &str) -> bool {
        self.subscriptions.contains(&String::from(subscription))
    }

    pub fn list_subscriptions(&self) -> Vec<String> {
        self.subscriptions.clone()
    }

    pub fn count_subscriptions(&self) -> usize {
        self.subscriptions.len()
    }
}

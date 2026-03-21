extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    let mut payment = PaymentApprover::new();
    payment.add_payment("user1", 100);
    payment.add_payment("user2", 50);
    payment.approve_payment("user1");
}

pub struct PaymentApprover {
    payments: Vec<(String, u32, bool)>,
}

impl PaymentApprover {
    pub fn new() -> Self {
        PaymentApprover {
            payments: Vec::new(),
        }
    }

    pub fn add_payment(&mut self, user_id: &str, amount: u32) {
        self.payments.push((String::from(user_id), amount, false));
    }

    pub fn approve_payment(&mut self, user_id: &str) -> bool {
        for payment in self.payments.iter_mut() {
            if payment.0 == user_id && !payment.2 {
                payment.2 = true;
                return true;
            }
        }
        false
    }

    pub fn is_payment_approved(&self, user_id: &str) -> bool {
        for payment in self.payments.iter() {
            if payment.0 == user_id {
                return payment.2;
            }
        }
        false
    }

    pub fn total_payments(&self) -> u32 {
        self.payments.iter().map(|p| p.1).sum()
    }

    pub fn total_approved(&self) -> u32 {
        self.payments.iter().filter_map(|p| if p.2 { Some(p.1) } else { None }).sum()
    }
}

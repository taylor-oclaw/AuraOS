extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct FinanceCouponClipper {
    coupons: Vec<String>,
}

impl FinanceCouponClipper {
    pub fn new() -> Self {
        FinanceCouponClipper {
            coupons: Vec::new(),
        }
    }

    pub fn add_coupon(&mut self, coupon: String) {
        if !self.coupons.contains(&coupon) {
            self.coupons.push(coupon);
        }
    }

    pub fn remove_coupon(&mut self, coupon: &str) -> bool {
        let position = self.coupons.iter().position(|c| c == coupon);
        match position {
            Some(index) => {
                self.coupons.remove(index);
                true
            },
            None => false,
        }
    }

    pub fn has_coupon(&self, coupon: &str) -> bool {
        self.coupons.contains(&String::from(coupon))
    }

    pub fn list_coupons(&self) -> Vec<String> {
        self.coupons.clone()
    }

    pub fn clear_coupons(&mut self) {
        self.coupons.clear();
    }
}

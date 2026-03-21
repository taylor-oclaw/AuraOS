extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    0
}

pub struct BizClientAccountPlan {
    client_id: String,
    plan_type: String,
    features: Vec<String>,
    start_date: u64, // Unix timestamp
    end_date: u64,   // Unix timestamp
}

impl BizClientAccountPlan {
    pub fn new(client_id: &str, plan_type: &str, features: Vec<&str>, start_date: u64, end_date: u64) -> Self {
        BizClientAccountPlan {
            client_id: String::from(client_id),
            plan_type: String::from(plan_type),
            features: features.into_iter().map(String::from).collect(),
            start_date,
            end_date,
        }
    }

    pub fn get_client_id(&self) -> &str {
        &self.client_id
    }

    pub fn get_plan_type(&self) -> &str {
        &self.plan_type
    }

    pub fn has_feature(&self, feature: &str) -> bool {
        self.features.contains(&String::from(feature))
    }

    pub fn is_active(&self, current_time: u64) -> bool {
        current_time >= self.start_date && current_time <= self.end_date
    }

    pub fn extend_plan(&mut self, additional_days: u64) {
        self.end_date += additional_days * 86400; // 1 day = 86400 seconds
    }
}

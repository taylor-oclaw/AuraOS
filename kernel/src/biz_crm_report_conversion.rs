extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
    let report = BizCrmReportConversion::new();
    report.add_conversion("user123", 45.99);
    report.add_conversion("user456", 78.50);
    report.add_conversion("user123", 23.75);

    println!("Total conversions for user123: {}", report.get_total_conversions("user123"));
    println!("Average conversion value for all users: {:.2}", report.calculate_average_conversion());
    println!("Number of unique users: {}", report.count_unique_users());
    println!("Highest conversion by a single user: {:.2}", report.find_highest_conversion());
    println!("Conversion history for user456: {:?}", report.get_conversion_history("user456"));
}

pub struct BizCrmReportConversion {
    conversions: Vec<(String, f32)>,
}

impl BizCrmReportConversion {
    pub fn new() -> Self {
        BizCrmReportConversion {
            conversions: Vec::new(),
        }
    }

    pub fn add_conversion(&mut self, user_id: &str, amount: f32) {
        self.conversions.push((String::from(user_id), amount));
    }

    pub fn get_total_conversions(&self, user_id: &str) -> f32 {
        self.conversions
            .iter()
            .filter(|&&(ref id, _)| id == user_id)
            .map(|&(_, amount)| amount)
            .sum()
    }

    pub fn calculate_average_conversion(&self) -> f32 {
        if self.conversions.is_empty() {
            0.0
        } else {
            self.conversions.iter().map(|&(_, amount)| amount).sum::<f32>() / self.conversions.len() as f32
        }
    }

    pub fn count_unique_users(&self) -> usize {
        let mut unique_users = Vec::new();
        for (user_id, _) in &self.conversions {
            if !unique_users.contains(user_id) {
                unique_users.push(user_id.clone());
            }
        }
        unique_users.len()
    }

    pub fn find_highest_conversion(&self) -> f32 {
        self.conversions.iter().map(|&(_, amount)| amount).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or(0.0)
    }

    pub fn get_conversion_history(&self, user_id: &str) -> Vec<f32> {
        self.conversions
            .iter()
            .filter(|&&(ref id, _)| id == user_id)
            .map(|&(_, amount)| amount)
            .collect()
    }
}

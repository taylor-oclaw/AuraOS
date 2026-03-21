extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let mut car_insurance = CarInsuranceTrack::new();
    car_insurance.add_policy("Policy123", 500.0);
    car_insurance.update_policy("Policy123", 600.0);
    car_insurance.remove_policy("Policy456");
    car_insurance.list_policies();
    let claim_status = car_insurance.process_claim("Policy123", 300.0);

    loop {}
}

pub struct CarInsuranceTrack {
    policies: Vec<(String, f64)>,
}

impl CarInsuranceTrack {
    pub fn new() -> Self {
        CarInsuranceTrack {
            policies: Vec::new(),
        }
    }

    pub fn add_policy(&mut self, policy_id: &str, premium: f64) {
        let policy = (String::from(policy_id), premium);
        self.policies.push(policy);
    }

    pub fn update_policy(&mut self, policy_id: &str, new_premium: f64) {
        for policy in self.policies.iter_mut() {
            if policy.0 == policy_id {
                policy.1 = new_premium;
                break;
            }
        }
    }

    pub fn remove_policy(&mut self, policy_id: &str) {
        self.policies.retain(|policy| policy.0 != policy_id);
    }

    pub fn list_policies(&self) {
        for (id, premium) in &self.policies {
        }
    }

    pub fn process_claim(&mut self, policy_id: &str, claim_amount: f64) -> String {
        for policy in &mut self.policies {
            if policy.0 == policy_id {
                if claim_amount <= policy.1 {
                    return String::from("Claim approved");
                } else {
                    return String::from("Claim denied");
                }
            }
        }
        String::from("Policy not found")
    }
}

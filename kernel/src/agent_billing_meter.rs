extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn agent_billing_meter_init() {
    // Initialization logic for the module
}

pub extern "C" fn agent_billing_meter_exit() {
    // Cleanup logic for the module
}

pub struct AgentBillingMeter {
    customer_id: String,
    usage_records: Vec<(String, u32)>, // (service_name, usage_amount)
    total_charges: u32,
}

impl AgentBillingMeter {
    pub fn new(customer_id: &str) -> Self {
        AgentBillingMeter {
            customer_id: String::from(customer_id),
            usage_records: Vec::new(),
            total_charges: 0,
        }
    }

    pub fn add_usage(&mut self, service_name: &str, amount: u32) {
        self.usage_records.push((String::from(service_name), amount));
        self.update_total_charges();
    }

    pub fn get_customer_id(&self) -> &str {
        &self.customer_id
    }

    pub fn get_usage_records(&self) -> &Vec<(String, u32)> {
        &self.usage_records
    }

    pub fn get_total_charges(&self) -> u32 {
        self.total_charges
    }

    fn update_total_charges(&mut self) {
        let mut total = 0;
        for (_, amount) in &self.usage_records {
            total += amount;
        }
        self.total_charges = total;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_billing_meter() {
        let mut meter = AgentBillingMeter::new("customer123");
        assert_eq!(meter.get_customer_id(), "customer123");
        assert_eq!(meter.get_usage_records().len(), 0);
        assert_eq!(meter.get_total_charges(), 0);

        meter.add_usage("serviceA", 100);
        meter.add_usage("serviceB", 200);

        assert_eq!(meter.get_usage_records().len(), 2);
        assert_eq!(meter.get_total_charges(), 300);
    }
}

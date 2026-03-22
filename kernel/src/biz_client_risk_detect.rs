extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn init_module() -> i32 {
    0
}

#[no_mangle]
pub extern "C" fn cleanup_module() {}

struct BizClientRiskDetect {
    client_id: String,
    risk_score: u8,
    transactions: Vec<u64>,
    alerts: Vec<String>,
}

impl BizClientRiskDetect {
    pub fn new(client_id: &str) -> Self {
        BizClientRiskDetect {
            client_id: String::from(client_id),
            risk_score: 0,
            transactions: Vec::new(),
            alerts: Vec::new(),
        }
    }

    pub fn add_transaction(&mut self, amount: u64) {
        self.transactions.push(amount);
        if amount > 10000 {
            self.alerts.push(format!("Large transaction detected: {}", amount));
            self.risk_score += 5;
        } else {
            self.risk_score += 1;
        }
    }

    pub fn get_risk_score(&self) -> u8 {
        self.risk_score
    }

    pub fn get_alerts(&self) -> &Vec<String> {
        &self.alerts
    }

    pub fn reset_risk(&mut self) {
        self.risk_score = 0;
        self.alerts.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let detector = BizClientRiskDetect::new("client123");
        assert_eq!(detector.client_id, "client123");
        assert_eq!(detector.risk_score, 0);
        assert!(detector.transactions.is_empty());
        assert!(detector.alerts.is_empty());
    }

    #[test]
    fn test_add_transaction() {
        let mut detector = BizClientRiskDetect::new("client456");
        detector.add_transaction(5000);
        assert_eq!(detector.risk_score, 1);
        assert!(detector.transactions.contains(&5000));
        assert!(detector.alerts.is_empty());

        detector.add_transaction(20000);
        assert_eq!(detector.risk_score, 6);
        assert!(detector.transactions.contains(&20000));
        assert_eq!(detector.alerts.len(), 1);
    }

    #[test]
    fn test_get_risk_score() {
        let mut detector = BizClientRiskDetect::new("client789");
        detector.add_transaction(3000);
        assert_eq!(detector.get_risk_score(), 1);
    }

    #[test]
    fn test_get_alerts() {
        let mut detector = BizClientRiskDetect::new("client012");
        detector.add_transaction(40000);
        assert_eq!(detector.get_alerts().len(), 1);
        assert_eq!(detector.get_alerts()[0], "Large transaction detected: 40000");
    }

    #[test]
    fn test_reset_risk() {
        let mut detector = BizClientRiskDetect::new("client345");
        detector.add_transaction(60000);
        assert_eq!(detector.risk_score, 5);
        assert!(!detector.alerts.is_empty());

        detector.reset_risk();
        assert_eq!(detector.risk_score, 0);
        assert!(detector.alerts.is_empty());
    }
}
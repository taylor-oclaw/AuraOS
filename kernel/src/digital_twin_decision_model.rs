extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    0
}

struct DigitalTwinDecisionModel {
    data: Vec<String>,
    decisions: Vec<String>,
}

impl DigitalTwinDecisionModel {
    pub fn new() -> Self {
        DigitalTwinDecisionModel {
            data: Vec::new(),
            decisions: Vec::new(),
        }
    }

    pub fn add_data(&mut self, item: String) {
        self.data.push(item);
    }

    pub fn get_data(&self) -> &Vec<String> {
        &self.data
    }

    pub fn make_decision(&mut self, decision: String) {
        self.decisions.push(decision);
    }

    pub fn get_decisions(&self) -> &Vec<String> {
        &self.decisions
    }

    pub fn clear_data(&mut self) {
        self.data.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digital_twin_decision_model() {
        let mut model = DigitalTwinDecisionModel::new();

        model.add_data(String::from("Data1"));
        model.add_data(String::from("Data2"));

        assert_eq!(model.get_data().len(), 2);

        model.make_decision(String::from("Decision1"));
        model.make_decision(String::from("Decision2"));

        assert_eq!(model.get_decisions().len(), 2);

        model.clear_data();

        assert_eq!(model.get_data().len(), 0);
    }
}

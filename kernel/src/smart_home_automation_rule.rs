extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct SmartHomeAutomationRule {
    name: String,
    conditions: Vec<String>,
    actions: Vec<String>,
}

impl SmartHomeAutomationRule {
    pub fn new(name: &str, conditions: Vec<&str>, actions: Vec<&str>) -> Self {
        SmartHomeAutomationRule {
            name: String::from(name),
            conditions: conditions.into_iter().map(String::from).collect(),
            actions: actions.into_iter().map(String::from).collect(),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn add_condition(&mut self, condition: &str) {
        self.conditions.push(String::from(condition));
    }

    pub fn remove_condition(&mut self, condition: &str) {
        self.conditions.retain(|c| c != condition);
    }

    pub fn get_conditions(&self) -> &[String] {
        &self.conditions
    }

    pub fn add_action(&mut self, action: &str) {
        self.actions.push(String::from(action));
    }

    pub fn remove_action(&mut self, action: &str) {
        self.actions.retain(|a| a != action);
    }

    pub fn get_actions(&self) -> &[String] {
        &self.actions
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smart_home_automation_rule() {
        let mut rule = SmartHomeAutomationRule::new(
            "Turn on lights",
            vec!["Motion detected", "Time is between 6 PM and 9 PM"],
            vec!["Turn on bedroom lights", "Turn on living room lights"],
        );

        assert_eq!(rule.get_name(), "Turn on lights");
        assert_eq!(
            rule.get_conditions(),
            &[
                String::from("Motion detected"),
                String::from("Time is between 6 PM and 9 PM")
            ]
        );
        assert_eq!(
            rule.get_actions(),
            &[
                String::from("Turn on bedroom lights"),
                String::from("Turn on living room lights")
            ]
        );

        rule.add_condition("It's raining");
        assert_eq!(
            rule.get_conditions(),
            &[
                String::from("Motion detected"),
                String::from("Time is between 6 PM and 9 PM"),
                String::from("It's raining")
            ]
        );

        rule.remove_condition("Motion detected");
        assert_eq!(
            rule.get_conditions(),
            &[
                String::from("Time is between 6 PM and 9 PM"),
                String::from("It's raining")
            ]
        );

        rule.add_action("Turn on kitchen lights");
        assert_eq!(
            rule.get_actions(),
            &[
                String::from("Turn on bedroom lights"),
                String::from("Turn on living room lights"),
                String::from("Turn on kitchen lights")
            ]
        );

        rule.remove_action("Turn on bedroom lights");
        assert_eq!(
            rule.get_actions(),
            &[
                String::from("Turn on living room lights"),
                String::from("Turn on kitchen lights")
            ]
        );
    }
}

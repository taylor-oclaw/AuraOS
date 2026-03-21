extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct AIAuditDecision {
    decision_id: u32,
    explanation: String,
    confidence_level: f32,
    timestamp: u64,
    related_entities: Vec<String>,
}

impl AIAuditDecision {
    pub fn new(decision_id: u32, explanation: &str, confidence_level: f32, timestamp: u64) -> Self {
        AIAuditDecision {
            decision_id,
            explanation: String::from(explanation),
            confidence_level,
            timestamp,
            related_entities: Vec::new(),
        }
    }

    pub fn add_entity(&mut self, entity: &str) {
        self.related_entities.push(String::from(entity));
    }

    pub fn get_decision_id(&self) -> u32 {
        self.decision_id
    }

    pub fn get_explanation(&self) -> &str {
        &self.explanation
    }

    pub fn get_confidence_level(&self) -> f32 {
        self.confidence_level
    }

    pub fn get_timestamp(&self) -> u64 {
        self.timestamp
    }

    pub fn get_related_entities(&self) -> &[String] {
        &self.related_entities
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ai_audit_decision() {
        let mut decision = AIAuditDecision::new(1, "Access granted", 0.95, 1633072800);
        assert_eq!(decision.get_decision_id(), 1);
        assert_eq!(decision.get_explanation(), "Access granted");
        assert_eq!(decision.get_confidence_level(), 0.95);
        assert_eq!(decision.get_timestamp(), 1633072800);
        assert!(decision.get_related_entities().is_empty());

        decision.add_entity("user123");
        decision.add_entity("resource456");

        let entities = decision.get_related_entities();
        assert_eq!(entities.len(), 2);
        assert_eq!(entities[0], "user123");
        assert_eq!(entities[1], "resource456");
    }
}

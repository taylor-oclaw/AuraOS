extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub enum ReflectionType {
    TaskSuccess,
    TaskFailure,
    ResourceOveruse,
    SlowResponse,
    SecurityViolation,
    PatternDetected,
}

pub struct Reflection {
    pub id: u64,
    pub agent_id: u64,
    pub reflection_type: ReflectionType,
    pub description: String,
    pub lesson: String,
    pub confidence: f32,
    pub applied: bool,
    pub timestamp: u64,
}

pub struct AgentReflection {
    pub reflections: Vec<Reflection>,
    pub next_id: u64,
    pub auto_apply: bool,
}

impl AgentReflection {
    pub fn new() -> Self {
        Self {
            reflections: Vec::new(),
            next_id: 1,
            auto_apply: true,
        }
    }

    pub fn reflect(&mut self, agent_id: u64, rtype: ReflectionType, desc: &str, lesson: &str, confidence: f32) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.reflections.push(Reflection {
            id,
            agent_id,
            reflection_type: rtype,
            description: String::from(desc),
            lesson: String::from(lesson),
            confidence,
            applied: false,
            timestamp: 0,
        });
        id
    }

    pub fn apply_lesson(&mut self, id: u64) {
        if let Some(r) = self.reflections.iter_mut().find(|r| r.id == id) {
            r.applied = true;
        }
    }

    pub fn lessons_for_agent(&self, agent_id: u64) -> Vec<&Reflection> {
        self.reflections.iter().filter(|r| r.agent_id == agent_id && r.applied).collect()
    }

    pub fn unapplied_lessons(&self) -> Vec<&Reflection> {
        self.reflections.iter().filter(|r| !r.applied && r.confidence > 0.7).collect()
    }

    pub fn high_confidence_lessons(&self) -> Vec<&Reflection> {
        self.reflections.iter().filter(|r| r.confidence > 0.9).collect()
    }

    pub fn failure_count(&self, agent_id: u64) -> usize {
        self.reflections.iter().filter(|r| r.agent_id == agent_id && matches!(r.reflection_type, ReflectionType::TaskFailure)).count()
    }

    pub fn total(&self) -> usize {
        self.reflections.len()
    }
}

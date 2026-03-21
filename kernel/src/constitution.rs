extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct Rule {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub enforced: bool,
    pub violation_count: u64,
    pub severity: u8,
}

pub struct Constitution {
    pub rules: Vec<Rule>,
    pub next_id: u64,
    pub signature: [u8; 64],
    pub immutable: bool,
    pub version: u32,
}

impl Constitution {
    pub fn new() -> Self {
        let mut c = Self {
            rules: Vec::new(),
            next_id: 1,
            signature: [0; 64],
            immutable: false,
            version: 1,
        };
        c.add_rule("no_data_exfiltration", "Agents must not send user data to unauthorized endpoints", 10);
        c.add_rule("cost_limit", "No single agent may exceed daily cost budget", 8);
        c.add_rule("human_approval_destructive", "Destructive actions require human approval", 10);
        c.add_rule("no_self_modification", "Agents cannot modify the constitution", 10);
        c.add_rule("sandbox_first", "New agents must pass sandbox evaluation before production", 7);
        c.add_rule("least_privilege", "Agents receive minimum required permissions", 9);
        c.add_rule("audit_trail", "All agent actions must be logged", 8);
        c
    }

    fn add_rule(&mut self, name: &str, desc: &str, severity: u8) {
        let id = self.next_id;
        self.next_id += 1;
        self.rules.push(Rule {
            id,
            name: String::from(name),
            description: String::from(desc),
            enforced: true,
            violation_count: 0,
            severity,
        };
    }

    pub fn check_action(&mut self, action: &str) -> bool {
        let dominated = self.rules.iter().any(|r| r.enforced && r.name == action);
        if dominated {
            if let Some(r) = self.rules.iter_mut().find(|r| r.name == action) {
                r.violation_count += 1;
            }
        }
        !dominated
    }

    pub fn seal(&mut self) {
        self.immutable = true;
    }

    pub fn is_sealed(&self) -> bool {
        self.immutable
    }

    pub fn violation_count(&self) -> u64 {
        self.rules.iter().map(|r| r.violation_count).sum()
    }

    pub fn rule_count(&self) -> usize {
        self.rules.len()
    }
)}

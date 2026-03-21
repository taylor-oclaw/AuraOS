extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub enum WatchSource {
    Email,
    Calendar,
    Weather,
    StockPrice(String),
    SocialMedia(String),
    WebPage(String),
    FileChange(String),
    SystemMetric(String)
}

pub enum WatchCondition {
    Contains(String),
    GreaterThan(f64),
    LessThan(f64),
    Changed,
    NewItem,
    Matches(String)
}

pub struct WatchRule {
    pub id: u64,
    pub name: String,
    pub source: WatchSource,
    pub condition: WatchCondition,
    pub action: String,
    pub enabled: bool,
    pub triggered_count: u64,
    pub last_triggered: Option<u64>,
    pub check_interval_secs: u64
}

pub struct WatchAgent {
    pub rules: Vec<WatchRule>,
    pub next_id: u64,
    pub running: bool
}

impl WatchAgent {
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
            next_id: 1,
            running: true
        }
    }

    pub fn add_rule(&mut self, name: &str, source: WatchSource, condition: WatchCondition, action: &str, interval: u64) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.rules.push(WatchRule {
            id,
            name: String::from(name),
            source,
            condition,
            action: String::from(action),
            enabled: true,
            triggered_count: 0,
            last_triggered: None,
            check_interval_secs: interval
        });
        id
    }

    pub fn disable_rule(&mut self, id: u64) {
        if let Some(r) = self.rules.iter_mut().find(|r| r.id == id) {
            r.enabled = false;
        }
    }

    pub fn trigger(&mut self, id: u64) {
        if let Some(r) = self.rules.iter_mut().find(|r| r.id == id) {
            r.triggered_count += 1;
            r.last_triggered = Some(0);
        }
    }

    pub fn active_rules(&self) -> Vec<&WatchRule> {
        self.rules.iter().filter(|r| r.enabled).collect()
    }

    pub fn rule_count(&self) -> usize {
        self.rules.len()
    }
}

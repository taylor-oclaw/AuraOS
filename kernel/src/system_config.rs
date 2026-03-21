extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub enum ConfigValue {
    Text(String),
    Number(i64),
    Float(f32),
    Bool(bool),
    List(Vec<String>),
}

pub struct ConfigEntry {
    pub key: String,
    pub value: ConfigValue,
    pub category: String,
    pub description: String,
    pub user_modified: bool,
}

pub struct SystemConfig {
    pub entries: Vec<ConfigEntry>,
}

impl SystemConfig {
    pub fn new() -> Self {
        let mut cfg = Self {
            entries: Vec::new(),
        };
        cfg.set_default("display.resolution", ConfigValue::Text(String::from("1920x1080")), "display", "Screen resolution");
        cfg.set_default("display.theme", ConfigValue::Text(String::from("dark")), "display", "Color theme");
        cfg.set_default("display.font_size", ConfigValue::Number(14), "display", "Default font size");
        cfg.set_default("audio.volume", ConfigValue::Number(75), "audio", "System volume");
        cfg.set_default("audio.muted", ConfigValue::Bool(false), "audio", "Mute status");
        cfg.set_default("agent.max_concurrent", ConfigValue::Number(8), "agent", "Max concurrent agents");
        cfg.set_default("agent.default_llm_tier", ConfigValue::Text(String::from("small")), "agent", "Default LLM tier for agents");
        cfg.set_default("agent.cost_budget_daily", ConfigValue::Number(100), "agent", "Daily token budget");
        cfg.set_default("privacy.trust_zone_default", ConfigValue::Number(2), "privacy", "Default trust zone for new data");
        cfg.set_default("privacy.share_notifications", ConfigValue::Bool(true), "privacy", "Notify on data sharing");
        cfg.set_default("voice.wake_word", ConfigValue::Text(String::from("hey aura")), "voice", "Voice activation phrase");
        cfg.set_default("voice.enabled", ConfigValue::Bool(true), "voice", "Voice input enabled");
        cfg.set_default("network.agent_mesh", ConfigValue::Bool(true), "network", "Enable agent mesh networking");
        cfg.set_default("evolution.auto_pr", ConfigValue::Bool(true), "evolution", "Auto-create PRs for bug fixes");
        cfg.set_default("evolution.max_daily_prs", ConfigValue::Number(1), "evolution", "Max PRs per day");
        cfg
    }

    fn set_default(&mut self, key: &str, value: ConfigValue, category: &str, desc: &str) {
        self.entries.push(ConfigEntry {
            key: String::from(key),
            value,
            category: String::from(category),
            description: String::from(desc),
            user_modified: false,
        };
    }

    pub fn get(&self, key: &str) -> Option<&ConfigValue> {
        self.entries.iter().find(|e| e.key == key).map(|e| &e.value)
    }

    pub fn set(&mut self, key: &str, value: ConfigValue) {
        if let Some(entry) = self.entries.iter_mut().find(|e| e.key == key) {
            entry.value = value;
            entry.user_modified = true;
        }
    }

    pub fn by_category(&self, cat: &str) -> Vec<&ConfigEntry> {
        self.entries.iter().filter(|e| e.category == cat).collect()
    }
)}

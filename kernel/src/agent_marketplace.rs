extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct AgentSkill {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub version: String,
    pub author: String,
    pub downloads: u64,
    pub rating: f32,
    pub size_bytes: u64,
    pub installed: bool,
    pub capabilities: Vec<String>,
}

pub struct Marketplace {
    pub skills: Vec<AgentSkill>,
    pub installed: Vec<u64>,
    pub next_id: u64,
}

impl Marketplace {
    pub fn new() -> Self {
        let mut m = Self {
            skills: Vec::new(),
            installed: Vec::new(),
            next_id: 1,
        };
        m.publish("web_search", "Search the web", "1.0", "aura_team", vec!["http", "parse"]);
        m.publish("code_review", "Review code for bugs", "2.1", "aura_team", vec!["code", "analysis"]);
        m.publish("email_agent", "Send and read email", "1.3", "community", vec!["email", "imap"]);
        m.publish("calendar_sync", "Sync calendars", "1.0", "community", vec!["caldav", "schedule"]);
        m.publish("data_analyst", "Analyze CSV and data", "1.5", "aura_team", vec!["data", "stats"]);
        m
    }

    fn publish(&mut self, name: &str, desc: &str, ver: &str, author: &str, caps: Vec<&str>) {
        let id = self.next_id;
        self.next_id += 1;
        self.skills.push(AgentSkill {
            id,
            name: String::from(name),
            description: String::from(desc),
            version: String::from(ver),
            author: String::from(author),
            downloads: 0,
            rating: 0.0,
            size_bytes: 0,
            installed: false,
            capabilities: caps.iter().map(|c| String::from(*c)).collect(),
        });
    }

    pub fn install(&mut self, skill_id: u64) -> bool {
        if let Some(s) = self.skills.iter_mut().find(|s| s.id == skill_id) {
            s.installed = true;
            s.downloads += 1;
            self.installed.push(skill_id);
            true
        } else {
            false
        }
    }

    pub fn uninstall(&mut self, skill_id: u64) {
        if let Some(s) = self.skills.iter_mut().find(|s| s.id == skill_id) {
            s.installed = false;
        }
        self.installed.retain(|id| *id != skill_id);
    }

    pub fn search(&self, query: &str) -> Vec<&AgentSkill> {
        self.skills
            .iter()
            .filter(|s| s.name.contains(query) || s.description.contains(query))
            .collect()
    }

    pub fn installed_skills(&self) -> Vec<&AgentSkill> {
        self.skills.iter().filter(|s| s.installed).collect()
    }

    pub fn skill_count(&self) -> usize {
        self.skills.len()
    }
}

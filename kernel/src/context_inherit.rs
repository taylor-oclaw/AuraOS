extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct ContextEntry {
    pub key: String,
    pub value: String,
    pub source_agent: u64,
    pub inherited: bool,
    pub override_allowed: bool,
}

pub struct AgentContext {
    pub agent_id: u64,
    pub parent_id: Option<u64>,
    pub entries: Vec<ContextEntry>,
}

pub struct ContextManager {
    pub contexts: Vec<AgentContext>,
}

impl ContextManager {
    pub fn new() -> Self {
        Self {
            contexts: Vec::new(),
        }
    }

    pub fn create_context(&mut self, agent_id: u64, parent_id: Option<u64>) {
        let mut entries = Vec::new();
        if let Some(pid) = parent_id {
            if let Some(parent_ctx) = self.contexts.iter().find(|c| c.agent_id == pid) {
                for entry in &parent_ctx.entries {
                    entries.push(ContextEntry {
                        key: entry.key.clone(),
                        value: entry.value.clone(),
                        source_agent: entry.source_agent,
                        inherited: true,
                        override_allowed: true,
                    });
                }
            }
        }
        self.contexts.push(AgentContext {
            agent_id,
            parent_id,
            entries,
        });
    }

    pub fn set(&mut self, agent_id: u64, key: &str, value: &str) {
        if let Some(ctx) = self.contexts.iter_mut().find(|c| c.agent_id == agent_id) {
            if let Some(entry) = ctx.entries.iter_mut().find(|e| e.key == key) {
                if entry.override_allowed || !entry.inherited {
                    entry.value = String::from(value);
                    entry.source_agent = agent_id;
                    entry.inherited = false;
                }
            } else {
                ctx.entries.push(ContextEntry {
                    key: String::from(key),
                    value: String::from(value),
                    source_agent: agent_id,
                    inherited: false,
                    override_allowed: true,
                });
            }
        }
    }

    pub fn get(&self, agent_id: u64, key: &str) -> Option<&str> {
        self.contexts
            .iter()
            .find(|c| c.agent_id == agent_id)
            .and_then(|ctx| ctx.entries.iter().find(|e| e.key == key).map(|e| e.value.as_str()))
    }

    pub fn context_size(&self, agent_id: u64) -> usize {
        self.contexts
            .iter()
            .find(|c| c.agent_id == agent_id)
            .map(|c| c.entries.len())
            .unwrap_or(0)
    }
}

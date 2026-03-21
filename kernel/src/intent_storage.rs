extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub enum StorageIntent {
    Remember {
        data: Vec<u8>,
        hint: String,
        ttl: Option<u64>,
    },
    Forget {
        query: String,
    },
    Find {
        query: String,
    },
    Share {
        data_id: u64,
        with_agent: u64,
    },
    Replicate {
        data_id: u64,
        copies: u8,
    }
}

pub struct StoredData {
    pub id: u64,
    pub data: Vec<u8>,
    pub tags: Vec<String>,
    pub created_at: u64,
    pub last_accessed: u64,
    pub ttl: Option<u64>,
    pub owner_agent: u64,
    pub shared_with: Vec<u64>,
    pub replicas: u8,
    pub importance: f32,
}

pub struct IntentStorage {
    pub items: Vec<StoredData>,
    pub next_id: u64,
}

impl IntentStorage {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            next_id: 1,
        }
    }

    pub fn process_intent(&mut self, agent: u64, intent: StorageIntent) -> u64 {
        match intent {
            StorageIntent::Remember { data, hint, ttl } => {
                let id = self.next_id;
                self.next_id += 1;
                let tags = hint.split(32 as char).map(|s| String::from(s)).collect();
                self.items.push(StoredData {
                    id,
                    data,
                    tags,
                    created_at: 0,
                    last_accessed: 0,
                    ttl,
                    owner_agent: agent,
                    shared_with: Vec::new(),
                    replicas: 1,
                    importance: 0.5,
                };
                id
            }
            StorageIntent::Forget { query } => {
                self.items.retain(|i| !i.tags.iter().any(|t| t.contains(query.as_str())));
                0
            }
            StorageIntent::Find { query } => {
                self.items.iter().find(|i| i.tags.iter().any(|t| t.contains(query.as_str()))).map(|i| i.id).unwrap_or(0)
            }
            StorageIntent::Share { data_id, with_agent } => {
                if let Some(item) = self.items.iter_mut().find(|i| i.id == data_id) {
                    item.shared_with.push(with_agent);
                }
                data_id
            }
            StorageIntent::Replicate { data_id, copies } => {
                if let Some(item) = self.items.iter_mut().find(|i| i.id == data_id) {
                    item.replicas = copies;
                }
                data_id
            }
        }
    }

    pub fn find_by_tag(&self, tag: &str) -> Vec<&StoredData> {
        self.items.iter().filter(|i| i.tags.iter().any(|t| t == tag)).collect()
    }

    pub fn cleanup_expired(&mut self, now: u64) {
        self.items.retain(|i| i.ttl.map(|ttl| i.created_at + ttl > now).unwrap_or(true));
    }
)}

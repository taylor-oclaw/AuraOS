extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct Event {
    pub id: u64,
    pub topic: String,
    pub payload: String,
    pub source_agent: u64,
    pub timestamp: u64,
    pub delivered_to: Vec<u64>,
}

pub struct Subscription {
    pub agent_id: u64,
    pub topic: String,
    pub filter: Option<String>,
}

pub struct EventBus {
    pub events: Vec<Event>,
    pub subscriptions: Vec<Subscription>,
    pub next_id: u64,
    pub max_events: usize,
    pub total_published: u64,
    pub total_delivered: u64,
}

impl EventBus {
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
            subscriptions: Vec::new(),
            next_id: 1,
            max_events: 10000,
            total_published: 0,
            total_delivered: 0,
        }
    }

    pub fn subscribe(&mut self, agent_id: u64, topic: &str, filter: Option<&str>) {
        self.subscriptions.push(Subscription {
            agent_id,
            topic: String::from(topic),
            filter: filter.map(String::from),
        });
    }

    pub fn unsubscribe(&mut self, agent_id: u64, topic: &str) {
        self.subscriptions.retain(|s| !(s.agent_id == agent_id && s.topic == topic));
    }

    pub fn publish(&mut self, source: u64, topic: &str, payload: &str) -> u64 {
        let id = self.next_id;
        self.next_id += 1;

        let subscribers: Vec<u64> = self
            .subscriptions
            .iter()
            .filter(|s| s.topic == topic && s.agent_id != source)
            .map(|s| s.agent_id)
            .collect();

        let delivered = subscribers.len() as u64;

        self.events.push(Event {
            id,
            topic: String::from(topic),
            payload: String::from(payload),
            source_agent: source,
            timestamp: 0,
            delivered_to: subscribers,
        };

        self.total_published += 1;
        self.total_delivered += delivered;

        if self.events.len() > self.max_events {
            self.events.remove(0);
        }

        id
    }

    pub fn events_for_agent(&self, agent_id: u64) -> Vec<&Event> {
        self.events.iter().filter(|e| e.delivered_to.contains(&agent_id)).collect()
    }

    pub fn subscriber_count(&self, topic: &str) -> usize {
        self.subscriptions.iter().filter(|s| s.topic == topic).count()
    }
)}

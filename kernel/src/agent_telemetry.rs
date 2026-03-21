extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct TelemetryEvent {
    pub agent_id: u64,
    pub event_type: String,
    pub metric: String,
    pub value: f64,
    pub timestamp: u64,
    pub tags: Vec<(String, String)>,
}

pub struct AgentSpan {
    pub agent_id: u64,
    pub operation: String,
    pub start_ts: u64,
    pub end_ts: Option<u64>,
    pub parent_span: Option<u64>,
    pub status: SpanStatus,
}

pub enum SpanStatus {
    Running,
    Success,
    Error(String),
}

pub struct AgentTelemetry {
    pub events: Vec<TelemetryEvent>,
    pub spans: Vec<AgentSpan>,
    pub next_span_id: u64,
    pub enabled: bool,
    pub max_events: usize,
}

impl AgentTelemetry {
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
            spans: Vec::new(),
            next_span_id: 1,
            enabled: true,
            max_events: 100000,
        }
    }

    pub fn record(&mut self, agent_id: u64, event_type: &str, metric: &str, value: f64) {
        if !self.enabled {
            return;
        }
        self.events.push(TelemetryEvent {
            agent_id,
            event_type: String::from(event_type),
            metric: String::from(metric),
            value,
            timestamp: 0,
            tags: Vec::new(),
        });
        if self.events.len() > self.max_events {
            self.events.remove(0);
        }
    }

    pub fn start_span(&mut self, agent_id: u64, operation: &str, parent: Option<u64>) -> u64 {
        let id = self.next_span_id;
        self.next_span_id += 1;
        self.spans.push(AgentSpan {
            agent_id,
            operation: String::from(operation),
            start_ts: 0,
            end_ts: None,
            parent_span: parent,
            status: SpanStatus::Running,
        });
        id
    }

    pub fn end_span(&mut self, span_id: u64, success: bool) {
        if let Some(span) = self.spans.iter_mut().find(|s| s.agent_id == span_id) {
            span.end_ts = Some(0);
            span.status = if success {
                SpanStatus::Success
            } else {
                SpanStatus::Error(String::from("failed"))
            };
        }
    }

    pub fn events_for_agent(&self, agent_id: u64) -> Vec<&TelemetryEvent> {
        self.events.iter().filter(|e| e.agent_id == agent_id).collect()
    }

    pub fn active_spans(&self) -> Vec<&AgentSpan> {
        self.spans.iter().filter(|s| s.end_ts.is_none()).collect()
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }
}

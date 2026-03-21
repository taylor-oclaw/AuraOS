extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct PerfSample {
    pub module: String,
    pub function: String,
    pub duration_us: u64,
    pub timestamp: u64,
    pub agent_id: Option<u64>,
}

pub struct PerfStats {
    pub module: String,
    pub total_us: u64,
    pub count: u64,
    pub min_us: u64,
    pub max_us: u64,
}

pub struct PerfProfiler {
    pub samples: Vec<PerfSample>,
    pub enabled: bool,
    pub max_samples: usize,
}

impl PerfProfiler {
    pub fn new() -> Self {
        Self {
            samples: Vec::new(),
            enabled: false,
            max_samples: 50000,
        }
    }

    pub fn start(&mut self) {
        self.enabled = true;
    }

    pub fn stop(&mut self) {
        self.enabled = false;
    }

    pub fn record(&mut self, module: &str, function: &str, duration_us: u64, agent: Option<u64>) {
        if !self.enabled {
            return;
        }
        self.samples.push(PerfSample {
            module: String::from(module),
            function: String::from(function),
            duration_us,
            timestamp: 0,
            agent_id: agent,
        });
        if self.samples.len() > self.max_samples {
            self.samples.remove(0);
        }
    }

    pub fn top_modules(&self, n: usize) -> Vec<PerfStats> {
        let mut stats: Vec<PerfStats> = Vec::new();
        for s in &self.samples {
            if let Some(st) = stats.iter_mut().find(|st| st.module == s.module) {
                st.total_us += s.duration_us;
                st.count += 1;
                if s.duration_us < st.min_us {
                    st.min_us = s.duration_us;
                }
                if s.duration_us > st.max_us {
                    st.max_us = s.duration_us;
                }
            } else {
                stats.push(PerfStats {
                    module: s.module.clone(),
                    total_us: s.duration_us,
                    count: 1,
                    min_us: s.duration_us,
                    max_us: s.duration_us,
                });
            }
        }
        stats.sort_by(|a, b| b.total_us.cmp(&a.total_us));
        stats.truncate(n);
        stats
    }

    pub fn clear(&mut self) {
        self.samples.clear();
    }

    pub fn sample_count(&self) -> usize {
        self.samples.len()
    }
}

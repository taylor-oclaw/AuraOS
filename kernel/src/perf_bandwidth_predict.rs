extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

struct PerfBandwidthPredictor {
    history: Vec<u64>,
    window_size: usize,
}

impl PerfBandwidthPredictor {
    pub fn new(window_size: usize) -> Self {
        PerfBandwidthPredictor {
            history: Vec::with_capacity(window_size),
            window_size,
        }
    }

    pub fn add_sample(&mut self, sample: u64) {
        if self.history.len() >= self.window_size {
            self.history.remove(0);
        }
        self.history.push(sample);
    }

    pub fn average_bandwidth(&self) -> Option<u64> {
        if self.history.is_empty() {
            None
        } else {
            let sum: u64 = self.history.iter().sum();
            Some(sum / self.history.len() as u64)
        }
    }

    pub fn max_bandwidth(&self) -> Option<u64> {
        self.history.iter().max().copied()
    }

    pub fn min_bandwidth(&self) -> Option<u64> {
        self.history.iter().min().copied()
    }

    pub fn predict_next_bandwidth(&self) -> Option<u64> {
        if self.history.is_empty() {
            None
        } else {
            let last = *self.history.last().unwrap();
            Some(last + (last - self.min_bandwidth()?))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perf_bandwidth_predictor() {
        let mut predictor = PerfBandwidthPredictor::new(3);
        predictor.add_sample(100);
        predictor.add_sample(200);
        predictor.add_sample(300);

        assert_eq!(predictor.average_bandwidth(), Some(200));
        assert_eq!(predictor.max_bandwidth(), Some(300));
        assert_eq!(predictor.min_bandwidth(), Some(100));
        assert_eq!(predictor.predict_next_bandwidth(), Some(400));

        predictor.add_sample(400);
        assert_eq!(predictor.average_bandwidth(), Some(300));
        assert_eq!(predictor.max_bandwidth(), Some(400));
        assert_eq!(predictor.min_bandwidth(), Some(200));
        assert_eq!(predictor.predict_next_bandwidth(), Some(500));
    }
}

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct ContinuousBatcher {
    batch_size: usize,
    current_batch: Vec<String>,
}

impl ContinuousBatcher {
    pub fn new(batch_size: usize) -> Self {
        ContinuousBatcher {
            batch_size,
            current_batch: Vec::new(),
        }
    }

    pub fn add_item(&mut self, item: String) {
        if self.current_batch.len() >= self.batch_size {
            self.flush();
        }
        self.current_batch.push(item);
    }

    pub fn flush(&mut self) -> Vec<String> {
        let batch = core::mem::take(&mut self.current_batch);
        // Simulate sending the batch to a processing unit
        process_batch(batch.clone());
        batch
    }

    pub fn is_full(&self) -> bool {
        self.current_batch.len() >= self.batch_size
    }

    pub fn get_current_batch_size(&self) -> usize {
        self.current_batch.len()
    }
}

fn process_batch(batch: Vec<String>) {
    // Placeholder for actual processing logic
    for item in batch {
    }
}

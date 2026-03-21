extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_std]
mod analytics_distraction_count {
    use core::sync::atomic::{AtomicUsize, Ordering};

    pub struct DistractionCounter {
        count: AtomicUsize,
        distractions: Vec<String>,
    }

    impl DistractionCounter {
        pub fn new() -> Self {
            DistractionCounter {
                count: AtomicUsize::new(0),
                distractions: Vec::new(),
            }
        }

        pub fn increment(&self) {
            self.count.fetch_add(1, Ordering::SeqCst);
        }

        pub fn decrement(&self) {
            if self.count.load(Ordering::SeqCst) > 0 {
                self.count.fetch_sub(1, Ordering::SeqCst);
            }
        }

        pub fn get_count(&self) -> usize {
            self.count.load(Ordering::SeqCst)
        }

        pub fn add_distraction(&mut self, distraction: String) {
            self.distractions.push(distraction);
        }

        pub fn get_distractions(&self) -> &[String] {
            &self.distractions
        }
    }
}

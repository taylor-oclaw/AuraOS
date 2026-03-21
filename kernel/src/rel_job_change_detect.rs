extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct JobChangeDetector {
    previous_job: Option<String>,
    job_history: Vec<String>,
}

impl JobChangeDetector {
    pub fn new() -> Self {
        JobChangeDetector {
            previous_job: None,
            job_history: Vec::new(),
        }
    }

    pub fn record_job(&mut self, job_name: &str) {
        let job = String::from(job_name);
        if self.previous_job.as_ref() != Some(&job) {
            self.job_history.push(job.clone());
            self.previous_job = Some(job);
        }
    }

    pub fn get_previous_job(&self) -> Option<&String> {
        self.previous_job.as_ref()
    }

    pub fn get_job_history(&self) -> &Vec<String> {
        &self.job_history
    }

    pub fn clear_history(&mut self) {
        self.job_history.clear();
        self.previous_job = None;
    }

    pub fn has_changed(&self, job_name: &str) -> bool {
        match &self.previous_job {
            Some(previous) => previous != job_name,
            None => true,
        }
    }
}

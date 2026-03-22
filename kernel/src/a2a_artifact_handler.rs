extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct A2AArtifactHandler {
    artifacts: Vec<String>,
}

impl A2AArtifactHandler {
    pub fn new() -> Self {
        A2AArtifactHandler {
            artifacts: Vec::new(),
        }
    }

    pub fn add_artifact(&mut self, artifact: String) {
        if !self.artifacts.contains(&artifact) {
            self.artifacts.push(artifact);
        }
    }

    pub fn remove_artifact(&mut self, artifact: &str) -> bool {
        let index = self.artifacts.iter().position(|a| a == artifact);
        match index {
            Some(i) => {
                self.artifacts.remove(i);
                true
            },
            None => false,
        }
    }

    pub fn get_artifact(&self, index: usize) -> Option<&String> {
        self.artifacts.get(index)
    }

    pub fn list_artifacts(&self) -> &Vec<String> {
        &self.artifacts
    }

    pub fn count_artifacts(&self) -> usize {
        self.artifacts.len()
    }
}
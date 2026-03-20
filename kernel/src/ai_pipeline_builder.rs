extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

mod ai_pipeline {
    use super::*;

    pub struct AIPipelineBuilder {
        stages: Vec<String>,
    }

    impl AIPipelineBuilder {
        pub fn new() -> Self {
            AIPipelineBuilder { stages: Vec::new() }
        }

        pub fn add_stage(&mut self, stage_name: &str) {
            self.stages.push(String::from(stage_name));
        }

        pub fn remove_stage(&mut self, stage_name: &str) {
            if let Some(index) = self.stages.iter().position(|s| s == stage_name) {
                self.stages.remove(index);
            }
        }

        pub fn get_stages(&self) -> Vec<String> {
            self.stages.clone()
        }

        pub fn has_stage(&self, stage_name: &str) -> bool {
            self.stages.contains(&String::from(stage_name))
        }

        pub fn clear_stages(&mut self) {
            self.stages.clear();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ai_pipeline_builder() {
        let mut builder = ai_pipeline::AIPipelineBuilder::new();

        builder.add_stage("Preprocessing");
        builder.add_stage("FeatureExtraction");

        assert_eq!(builder.get_stages(), vec![String::from("Preprocessing"), String::from("FeatureExtraction")]);
        assert!(builder.has_stage("Preprocessing"));
        assert!(!builder.has_stage("Classification"));

        builder.remove_stage("Preprocessing");
        assert!(!builder.has_stage("Preprocessing"));

        builder.clear_stages();
        assert_eq!(builder.get_stages().len(), 0);
    }
}

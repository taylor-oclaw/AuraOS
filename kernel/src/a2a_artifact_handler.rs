extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

mod a2a_artifact_handler {
    extern crate alloc;
    use alloc::string::String;
    use alloc::vec::Vec;

    pub struct ArtifactHandler {
        artifacts: Vec<String>,
    }

    impl ArtifactHandler {
        pub fn new() -> Self {
            ArtifactHandler {
                artifacts: Vec::new(),
            }
        }

        pub fn add_artifact(&mut self, artifact_name: &str) {
            self.artifacts.push(String::from(artifact_name));
        }

        pub fn remove_artifact(&mut self, artifact_name: &str) -> bool {
            if let Some(index) = self.artifacts.iter().position(|x| x == artifact_name) {
                self.artifacts.remove(index);
                true
            } else {
                false
            }
        }

        pub fn list_artifacts(&self) -> Vec<String> {
            self.artifacts.clone()
        }

        pub fn contains_artifact(&self, artifact_name: &str) -> bool {
            self.artifacts.contains(&String::from(artifact_name))
        }

        pub fn count_artifacts(&self) -> usize {
            self.artifacts.len()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::a2a_artifact_handler::ArtifactHandler;

    #[test]
    fn test_add_remove_artifact() {
        let mut handler = ArtifactHandler::new();
        handler.add_artifact("artifact1");
        assert_eq!(handler.count_artifacts(), 1);
        assert!(handler.remove_artifact("artifact1"));
        assert_eq!(handler.count_artifacts(), 0);
    }

    #[test]
    fn test_list_artifacts() {
        let mut handler = ArtifactHandler::new();
        handler.add_artifact("artifact1");
        handler.add_artifact("artifact2");
        let artifacts = handler.list_artifacts();
        assert_eq!(artifacts.len(), 2);
        assert!(artifacts.contains(&String::from("artifact1")));
        assert!(artifacts.contains(&String::from("artifact2")));
    }

    #[test]
    fn test_contains_artifact() {
        let mut handler = ArtifactHandler::new();
        handler.add_artifact("artifact1");
        assert!(handler.contains_artifact("artifact1"));
        assert!(!handler.contains_artifact("artifact2"));
    }

    #[test]
    fn test_count_artifacts() {
        let mut handler = ArtifactHandler::new();
        assert_eq!(handler.count_artifacts(), 0);
        handler.add_artifact("artifact1");
        assert_eq!(handler.count_artifacts(), 1);
        handler.add_artifact("artifact2");
        assert_eq!(handler.count_artifacts(), 2);
    }
}

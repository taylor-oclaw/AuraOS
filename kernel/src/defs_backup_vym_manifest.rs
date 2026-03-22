extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_std]
mod defs_backup_vym_manifest {
    use core::fmt::{Debug, Formatter};
    use alloc::boxed::Box;

    #[derive(Debug)]
    pub struct Manifest {
        name: String,
        version: String,
        dependencies: Vec<String>,
        description: String,
        author: String,
    }

    impl Manifest {
        pub fn new(name: &str, version: &str, dependencies: &[&str], description: &str, author: &str) -> Self {
            Manifest {
                name: String::from(name),
                version: String::from(version),
                dependencies: dependencies.iter().map(|&s| String::from(s)).collect(),
                description: String::from(description),
                author: String::from(author),
            }
        }

        pub fn get_name(&self) -> &str {
            &self.name
        }

        pub fn get_version(&self) -> &str {
            &self.version
        }

        pub fn add_dependency(&mut self, dependency: &str) {
            self.dependencies.push(String::from(dependency));
        }

        pub fn remove_dependency(&mut self, dependency: &str) {
            self.dependencies.retain(|d| d != dependency);
        }

        pub fn get_dependencies(&self) -> &[String] {
            &self.dependencies
        }
    }

    impl Debug for Manifest {
        fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("Manifest")
                .field("name", &self.name)
                .field("version", &self.version)
                .field("dependencies", &self.dependencies)
                .field("description", &self.description)
                .field("author", &self.author)
                .finish()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_manifest_creation() {
            let manifest = Manifest::new(
                "vym",
                "1.0.0",
                &["dep1", "dep2"],
                "A sample AI-native OS kernel module",
                "AI Team",
            );
            assert_eq!(manifest.get_name(), "vym");
            assert_eq!(manifest.get_version(), "1.0.0");
            assert_eq!(manifest.get_dependencies().len(), 2);
        }

        #[test]
        fn test_add_remove_dependency() {
            let mut manifest = Manifest::new(
                "vym",
                "1.0.0",
                &["dep1"],
                "A sample AI-native OS kernel module",
                "AI Team",
            );
            assert_eq!(manifest.get_dependencies().len(), 1);

            manifest.add_dependency("dep2");
            assert_eq!(manifest.get_dependencies().len(), 2);
            assert!(manifest.get_dependencies().contains(&String::from("dep2")));

            manifest.remove_dependency("dep1");
            assert_eq!(manifest.get_dependencies().len(), 1);
            assert!(!manifest.get_dependencies().contains(&String::from("dep1")));
        }
    }
}
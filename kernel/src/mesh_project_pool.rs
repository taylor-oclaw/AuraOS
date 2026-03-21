extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

mod mesh_project_pool {
    use super::*;

    pub struct MeshProjectPool {
        projects: Vec<String>,
    }

    impl MeshProjectPool {
        pub fn new() -> Self {
            MeshProjectPool {
                projects: Vec::new(),
            }
        }

        pub fn add_project(&mut self, project_name: String) {
            if !self.projects.contains(&project_name) {
                self.projects.push(project_name);
            }
        }

        pub fn remove_project(&mut self, project_name: &str) -> bool {
            let index = self.projects.iter().position(|x| x == project_name);
            match index {
                Some(i) => {
                    self.projects.remove(i);
                    true
                }
                None => false,
            }
        }

        pub fn list_projects(&self) -> Vec<String> {
            self.projects.clone()
        }

        pub fn has_project(&self, project_name: &str) -> bool {
            self.projects.contains(&String::from(project_name))
        }

        pub fn count_projects(&self) -> usize {
            self.projects.len()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mesh_project_pool() {
        let mut pool = mesh_project_pool::MeshProjectPool::new();
        assert_eq!(pool.count_projects(), 0);

        pool.add_project(String::from("AI-OS"));
        assert_eq!(pool.count_projects(), 1);
        assert!(pool.has_project("AI-OS"));

        pool.add_project(String::from("Kernel"));
        assert_eq!(pool.count_projects(), 2);
        assert!(pool.has_project("Kernel"));

        let projects = pool.list_projects();
        assert_eq!(projects.len(), 2);
        assert!(projects.contains(&String::from("AI-OS")));
        assert!(projects.contains(&String::from("Kernel")));

        assert!(pool.remove_project("AI-OS"));
        assert_eq!(pool.count_projects(), 1);
        assert!(!pool.has_project("AI-OS"));

        assert!(!pool.remove_project("NonExistent"));
        assert_eq!(pool.count_projects(), 1);

        pool.add_project(String::from("AI-OS"));
        assert_eq!(pool.count_projects(), 2);
    }
}

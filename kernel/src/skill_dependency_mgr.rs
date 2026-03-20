extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct SkillDependencyMgr {
    skills: Vec<String>,
    dependencies: Vec<Vec<usize>>,
}

impl SkillDependencyMgr {
    pub fn new() -> Self {
        SkillDependencyMgr {
            skills: Vec::new(),
            dependencies: Vec::new(),
        }
    }

    pub fn add_skill(&mut self, skill_name: &str) -> usize {
        let index = self.skills.len();
        self.skills.push(String::from(skill_name));
        self.dependencies.push(Vec::new());
        index
    }

    pub fn add_dependency(&mut self, dependent_skill_index: usize, dependency_skill_index: usize) {
        if dependent_skill_index < self.skills.len() && dependency_skill_index < self.skills.len() {
            self.dependencies[dependent_skill_index].push(dependency_skill_index);
        }
    }

    pub fn get_dependencies(&self, skill_index: usize) -> Option<&Vec<usize>> {
        if skill_index < self.skills.len() {
            Some(&self.dependencies[skill_index])
        } else {
            None
        }
    }

    pub fn is_valid_dependency_graph(&self) -> bool {
        let mut visited = vec![false; self.skills.len()];
        let mut rec_stack = vec![false; self.skills.len()];

        for i in 0..self.skills.len() {
            if !visited[i] && self.is_cyclic_util(i, &mut visited, &mut rec_stack) {
                return false;
            }
        }

        true
    }

    fn is_cyclic_util(&self, v: usize, visited: &mut [bool], rec_stack: &mut [bool]) -> bool {
        if !visited[v] {
            visited[v] = true;
            rec_stack[v] = true;

            for i in &self.dependencies[v] {
                if !visited[*i] && self.is_cyclic_util(*i, visited, rec_stack) {
                    return true;
                } else if rec_stack[*i] {
                    return true;
                }
            }
        }

        rec_stack[v] = false;
        false
    }
}

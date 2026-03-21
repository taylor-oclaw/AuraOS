extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub struct PlaybookStep {
    pub order: u8,
    pub action: String,
    pub description: String,
    pub required_capability: Option<String>,
    pub timeout_secs: u64,
}

pub struct Playbook {
    pub id: u64,
    pub name: String,
    pub role: String,
    pub description: String,
    pub steps: Vec<PlaybookStep>,
    pub usage_count: u64,
}

pub struct PlaybookManager {
    pub playbooks: Vec<Playbook>,
    pub next_id: u64,
}

impl PlaybookManager {
    pub fn new() -> Self {
        let mut mgr = Self {
            playbooks: Vec::new(),
            next_id: 1,
        };
        mgr.add_playbook(
            "Code Review",
            "reviewer",
            "Review code changes for quality and bugs",
            vec![
                ("check_diff", "Get the git diff of changes", None, 30),
                ("analyze_logic", "Check for logical errors", Some("code"), 60),
                ("check_style", "Verify coding style", Some("code"), 30),
                ("check_security", "Scan for security issues", Some("security"), 60),
                ("write_feedback", "Write review comments", Some("code"), 30),
            ],
        );
        mgr.add_playbook(
            "Deploy Service",
            "developer",
            "Deploy a service to production",
            vec![
                ("run_tests", "Execute test suite", Some("code"), 120),
                ("build_image", "Build container image", Some("docker"), 60),
                ("push_image", "Push to registry", Some("docker"), 30),
                ("update_deploy", "Update deployment config", Some("deploy"), 30),
                ("verify_health", "Check service health", Some("monitor"), 30),
            ],
        );
        mgr
    }

    fn add_playbook(
        &mut self,
        name: &str,
        role: &str,
        desc: &str,
        steps: Vec<(&str, &str, Option<&str>, u64)>,
    ) {
        let id = self.next_id;
        self.next_id += 1;
        let pb_steps = steps
            .iter()
            .enumerate()
            .map(|(i, (action, desc, cap, timeout))| PlaybookStep {
                order: i as u8,
                action: String::from(*action),
                description: String::from(*desc),
                required_capability: cap.map(|c| String::from(c)),
                timeout_secs: *timeout,
            })
            .collect();
        self.playbooks.push(Playbook {
            id,
            name: String::from(name),
            role: String::from(role),
            description: String::from(desc),
            steps: pb_steps,
            usage_count: 0,
        });
    }

    pub fn find_for_role(&self, role: &str) -> Vec<&Playbook> {
        self.playbooks.iter().filter(|p| p.role == role).collect()
    }

    pub fn execute(&mut self, id: u64) {
        if let Some(pb) = self.playbooks.iter_mut().find(|p| p.id == id) {
            pb.usage_count += 1;
        }
    }

    pub fn count(&self) -> usize {
        self.playbooks.len()
    }
}

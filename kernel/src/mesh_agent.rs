extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct MeshAgent {
    name: String,
    mesh_id: u32,
}

impl MeshAgent {
    pub fn new(name: &str) -> Self {
        MeshAgent {
            name: String::from(name),
            mesh_id: 0,
        }
    }

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn set_mesh_id(&mut self, id: u32) {
        self.mesh_id = id;
    }

    pub fn get_mesh_id(&self) -> u32 {
        self.mesh_id
    }

    pub fn add_node(&mut self, node_name: &str) {
        let mut nodes = Vec::new();
        if let Some(nodes) = self.get_nodes() {
            nodes.push(node_name);
        } else {
            nodes.push(node_name);
        }
        self.set_nodes(nodes);
    }

    pub fn remove_node(&mut self, node_name: &str) {
        let mut nodes = Vec::new();
        if let Some(mut nodes) = self.get_nodes() {
            for node in nodes.iter() {
                if node != node_name {
                    nodes.push(node.clone());
                }
            }
            self.set_nodes(nodes);
        }
    }

    pub fn get_nodes(&self) -> Option<Vec<&str>> {
        // This is a placeholder, you would need to implement the actual logic
        // to retrieve the list of nodes from the mesh.
        None
    }

    pub fn set_nodes(&mut self, nodes: Vec<String>) {
        let mut node_names = Vec::new();
        for node in nodes.iter() {
            node_names.push(node.as_str());
        }
        self.nodes = Some(node_names);
    }

    pub fn get_mesh_info(&self) -> String {
        format!("Mesh ID: {}, Nodes: {:?}", self.mesh_id, self.get_nodes().unwrap())
    }

    pub fn set_mesh_info(&mut self, mesh_info: &str) {
        // This is a placeholder, you would need to implement the actual logic
        // to update the mesh info.
    }
}

pub struct MeshAgentList {
    agents: Vec<MeshAgent>,
}

impl MeshAgentList {
    pub fn new() -> Self {
        MeshAgentList { agents: Vec::new() }
    }

    pub fn add_agent(&mut self, agent: MeshAgent) {
        self.agents.push(agent);
    }

    pub fn get_agents(&self) -> &Vec<MeshAgent> {
        &self.agents
    }
}
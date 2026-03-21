extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub enum SwarmBehavior {
    Explore,
    Exploit,
    Converge,
    Scatter,
    Follow(u64),
}

pub struct SwarmAgent {
    pub id: u64,
    pub position: (f32, f32),
    pub velocity: (f32, f32),
    pub behavior: SwarmBehavior,
    pub fitness: f32,
    pub best_position: (f32, f32),
    pub best_fitness: f32,
}

pub struct SwarmIntelligence {
    pub agents: Vec<SwarmAgent>,
    pub global_best: (f32, f32),
    pub global_best_fitness: f32,
    pub iteration: u64,
    pub next_id: u64,
}

impl SwarmIntelligence {
    pub fn new() -> Self {
        Self {
            agents: Vec::new(),
            global_best: (0.0, 0.0),
            global_best_fitness: 0.0,
            iteration: 0,
            next_id: 1,
        }
    }

    pub fn add_agent(&mut self, x: f32, y: f32) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.agents.push(SwarmAgent {
            id,
            position: (x, y),
            velocity: (0.0, 0.0),
            behavior: SwarmBehavior::Explore,
            fitness: 0.0,
            best_position: (x, y),
            best_fitness: 0.0,
        };
        id
    }

    pub fn update_fitness(&mut self, id: u64, fitness: f32) {
        if let Some(agent) = self.agents.iter_mut().find(|a| a.id == id) {
            agent.fitness = fitness;
            if fitness > agent.best_fitness {
                agent.best_fitness = fitness;
                agent.best_position = agent.position;
            }
            if fitness > self.global_best_fitness {
                self.global_best_fitness = fitness;
                self.global_best = agent.position;
            }
        }
    }

    pub fn step(&mut self) {
        self.iteration += 1;
        for agent in &mut self.agents {
            agent.position.0 += agent.velocity.0;
            agent.position.1 += agent.velocity.1;
        }
    }

    pub fn converged(&self) -> bool {
        self.agents.iter().all(|a| {
            let dx = a.position.0 - self.global_best.0;
            let dy = a.position.1 - self.global_best.1;
            (dx * dx + dy * dy) < 0.01
        })
    }

    pub fn agent_count(&self) -> usize {
        self.agents.len()
    }
)}

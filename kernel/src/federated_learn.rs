extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct ModelGradient {
    pub agent_id: u64,
    pub layer: String,
    pub values: Vec<f32>,
    pub sample_count: u64,
}

pub struct FederatedRound {
    pub round_id: u64,
    pub gradients: Vec<ModelGradient>,
    pub aggregated: Option<Vec<f32>>,
    pub participants: Vec<u64>,
    pub completed: bool,
}

pub struct FederatedLearning {
    pub rounds: Vec<FederatedRound>,
    pub next_round: u64,
    pub min_participants: usize,
    pub privacy_noise: f32,
}

impl FederatedLearning {
    pub fn new(min_participants: usize) -> Self {
        Self {
            rounds: Vec::new(),
            next_round: 1,
            min_participants,
            privacy_noise: 0.01,
        }
    }

    pub fn start_round(&mut self) -> u64 {
        let id = self.next_round;
        self.next_round += 1;
        self.rounds.push(FederatedRound {
            round_id: id,
            gradients: Vec::new(),
            aggregated: None,
            participants: Vec::new(),
            completed: false,
        });
        id
    }

    pub fn submit_gradient(&mut self, round_id: u64, agent_id: u64, layer: &str, values: Vec<f32>, samples: u64) {
        if let Some(round) = self.rounds.iter_mut().find(|r| r.round_id == round_id && !r.completed) {
            round.gradients.push(ModelGradient {
                agent_id,
                layer: String::from(layer),
                values,
                sample_count: samples,
            });
            if !round.participants.contains(&agent_id) {
                round.participants.push(agent_id);
            }
        }
    }

    pub fn aggregate(&mut self, round_id: u64) -> bool {
        if let Some(round) = self.rounds.iter_mut().find(|r| r.round_id == round_id) {
            if round.participants.len() < 2 {
                return false;
            }
            let total_samples: u64 = round.gradients.iter().map(|g| g.sample_count).sum();
            if total_samples == 0 {
                return false;
            }
            let dim = round.gradients.first().map(|g| g.values.len()).unwrap_or(0);
            let mut avg = vec![0.0f32; dim];
            for g in &round.gradients {
                let weight = g.sample_count as f32 / total_samples as f32;
                for (i, v) in g.values.iter().enumerate() {
                    if i < dim {
                        avg[i] += v * weight;
                    }
                }
            }
            round.aggregated = Some(avg);
            round.completed = true;
            true
        } else {
            false
        }
    }

    pub fn completed_rounds(&self) -> usize {
        self.rounds.iter().filter(|r| r.completed).count()
    }

    pub fn total_rounds(&self) -> usize {
        self.rounds.len()
    }
}

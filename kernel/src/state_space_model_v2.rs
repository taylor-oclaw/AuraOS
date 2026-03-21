extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[repr(C)]
pub struct StateSpaceModel {
    state: Vec<f32>,
    transition_matrix: Vec<Vec<f32>>,
    observation_matrix: Vec<Vec<f32>>,
    initial_state_probabilities: Vec<f32>,
}

impl StateSpaceModel {
    pub fn new(
        state: Vec<f32>,
        transition_matrix: Vec<Vec<f32>>,
        observation_matrix: Vec<Vec<f32>>,
        initial_state_probabilities: Vec<f32>,
     -> Self {
        StateSpaceModel {
            state,
            transition_matrix,
            observation_matrix,
            initial_state_probabilities,
        }
    }

    pub fn predict_next_state(&self, current_state_index: usize) -> f32 {
        let mut next_state = 0.0;
        for (i, &prob) in self.transition_matrix[current_state_index].iter().enumerate() {
            next_state += prob * self.state[i];
        }
        next_state
    }

    pub fn update_observation(&mut self, observation: f32, state_index: usize) {
        if let Some(obs_vec) = self.observation_matrix.get_mut(state_index) {
            obs_vec.push(observation);
        })
    }

    pub fn get_initial_probabilities(&self) -> &Vec<f32> {
        &self.initial_state_probabilities
    }

    pub fn set_transition_probability(
        &mut self,
        from_state: usize,
        to_state: usize,
        probability: f32,
     {
        if let Some(row) = self.transition_matrix.get_mut(from_state) {
            if let Some(cell) = row.get_mut(to_state) {
                *cell = probability;
            }
        })
    }

    pub fn calculate_likelihood(&self, observations: &[f32]) -> f32 {
        let mut likelihood = 1.0;
        for &obs in observations {
            let mut obs_prob = 0.0;
            for (i, &state_prob) in self.state.iter().enumerate() {
                obs_prob += state_prob * self.observation_matrix[i][0];
            }
            likelihood *= obs_prob;
        }
        likelihood
    }
}

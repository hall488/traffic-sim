use ndarray::{Array1, Array2};
use rand::Rng;

pub struct QLearning {
    pub q_table: Array2<f64>,
    learning_rate: f64,
    discount_factor: f64,
    epsilon: f64,
    epsilon_decay: f64,
}

impl QLearning {
    pub fn new(state_size: usize, action_size: usize) -> Self {
        Self {
            q_table: Array2::zeros((state_size, action_size)),
            learning_rate: 0.1,
            discount_factor: 0.99,
            epsilon: 1.0,
            epsilon_decay: 0.995,
        }
    }

    pub fn choose_action(&self, state: &Array1<f64>) -> usize {
        let mut rng = rand::thread_rng();
        if rng.gen::<f64>() < self.epsilon {
            rng.gen_range(0..self.q_table.shape()[1])
        } else {
            let state_index = state.iter()
                                   .position(|&x| x == state.iter().cloned().fold(f64::NEG_INFINITY, f64::max))
                                   .unwrap();
            let q_values = self.q_table.row(state_index);
            q_values.iter()
                    .position(|&x| x == q_values.iter().cloned().fold(f64::NEG_INFINITY, f64::max))
                    .unwrap()
        }
    }

    pub fn update(&mut self, state: &Array1<f64>, action: usize, reward: f64, next_state: &Array1<f64>) {
        let state_index = state.iter()
                               .position(|&x| x == state.iter().cloned().fold(f64::NEG_INFINITY, f64::max))
                               .unwrap();
        let next_state_index = next_state.iter()
                                         .position(|&x| x == next_state.iter().cloned().fold(f64::NEG_INFINITY, f64::max))
                                         .unwrap();
        let max_next_q = self.q_table.row(next_state_index)
                                     .iter()
                                     .cloned()
                                     .fold(f64::NEG_INFINITY, f64::max);
        let q_value = self.q_table[(state_index, action)];
        self.q_table[(state_index, action)] = q_value + self.learning_rate * (reward + self.discount_factor * max_next_q - q_value);
        self.epsilon *= self.epsilon_decay;
    }
}


use rustc_hash::FxHashMap;

use crate::tic_tac_toe::Board;

pub struct Q<const N: usize> {
    pub alpha: f64,
    pub discount: f64,
    pub values: FxHashMap<Board<N>, FxHashMap<(usize, usize), f64>>,
}

impl<const N: usize> Q<N> {
    pub fn new() -> Self {
        Q {
            alpha: 0.5,
            discount: 0.5,
            values: FxHashMap::default(),
        }
    }

    /// Even though the `.values` field is a double nested HashMap, this method
    /// makes it flat to the user.
    /// It provides a default value of 0.0 if the entry does not exist
    pub fn get(&self, state: Board<N>, action: (usize, usize)) -> f64 {
        match self.values.get(&state) {
            None => 0.0,
            Some(action_map) => match action_map.get(&action) {
                None => 0.0,
                Some(val) => *val,
            },
        }
    }

    /// Get the action with highest reward, and the reward.
    /// If state is not yet explored, then (None, 0.0).
    pub fn max_action_for_state(&self, state: Board<N>) -> (Option<(usize, usize)>, f64) {
        if let Some(action_map) = self.values.get(&state) {
            // There is at least one action entered for this state. Get the max value
            action_map.iter().fold((None, 0.0), |accum, item| {
                if accum.1 >= *item.1 {
                    accum
                } else {
                    (Some(*item.0), *item.1)
                }
            })
        } else {
            (None, 0.0)
        }
    }

    pub fn update(
        &mut self,
        state: Board<N>,
        action: (usize, usize),
        next_state: Board<N>,
        reward: f64,
    ) {
        // The current value
        let value = self.get(state, action);

        // Get the highest known value of the `next_state`
        let (_, next_q) = self.max_action_for_state(next_state);

        let value = value + self.alpha * (reward + (self.discount * next_q) - value);

        match self.values.get_mut(&state) {
            // If None, then create an entry for this state and action with reward of 0
            None => {
                let mut new_action_map = FxHashMap::default();
                new_action_map.insert(action, 0.0);
                self.values.insert(state, new_action_map);
            }
            Some(action_map) => match action_map.get_mut(&action) {
                // If None, then create an entry for this action with reward of 0
                None => {
                    action_map.insert(action, 0.0);
                }
                Some(val) => {
                    *val = value;
                }
            },
        }
    }

    pub fn possible_minus_explored(&self) -> usize {
        // How many possible states are there? 3^(N^2)
        let n_possible_states = 3_usize
            .checked_pow((N as u32).pow(2))
            .expect("Overflow calculating possible states. Need to change to bigint");

        let n_explored_states = self.values.len();

        n_possible_states - n_explored_states
    }
}

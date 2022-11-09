use crate::tic_tac_toe::Player;
use crate::{q_matrix::Q, tic_tac_toe::Board};

use rand::distributions::{Distribution, Uniform};
use rand::seq::SliceRandom;
use rand::thread_rng;

pub struct Agent<const N: usize> {
    pub eps: f64,
    pub qlearner: Q<N>,
}

impl<const N: usize> Agent<N> {
    pub fn new() -> Self {
        Agent {
            eps: 1.0,
            qlearner: Q::new(),
        }
    }

    pub fn get_action(&self, state: Board<N>, valid_actions: &[(usize, usize)]) -> (usize, usize) {
        // If random draw from U(0, 1) < self.eps, return a random choice from valid_actions
        let mut rng = thread_rng();
        let u = Uniform::from(0.0..1.0);
        if u.sample(&mut rng) < self.eps {
            return *valid_actions
                .choose(&mut rng)
                .expect("Nothing in valid_actions to select");
        }

        // Otherwise, get the best action for this state if any. If None, return random
        // choice from valid_actions
        match self.qlearner.max_action_for_state(state) {
            (None, _) => *valid_actions
                .choose(&mut rng)
                .expect("Nothing in valid_actions to select"),
            (Some(action), _) => action,
        }
    }

    pub fn learn_one_game(&mut self) {
        let mut rng = thread_rng();
        let mut game = Board::<N>::new();
        let mut player = Player::X;
        loop {
            let state = game;
            let action = self.get_action(state, &game.get_empty_spots());
            let winner = game.make_move(player, action.0, action.1);
            player = player.next_player();

            // Update the Q matrix if the game is over
            // May want to re-think this. It rewards the agent for ties, as well as winning
            if winner.is_some() {
                self.qlearner.update(state, action, game, 100.0);
                break;
            }

            // Other player makes random action
            let valid_actions = game.get_empty_spots();
            let rand_action = valid_actions
                .choose(&mut rng)
                .expect("Failed to notice that the game was over");
            let winner = game.make_move(player, rand_action.0, rand_action.1);
            player = player.next_player();

            // If the other player won (or tied the game), update the Q matrix
            if winner.is_some() {
                self.qlearner.update(state, action, game, -100.0);
                break;
            }

            // Update Q matrix with reward of 0
            self.qlearner.update(state, action, game, 0.0);
        }
    }

    pub fn learn(&mut self, n: usize) {
        for _ in 0..n {
            self.learn_one_game();
            self.eps -= 0.00001;
        }

        // Check if all states have been visited at least once
        let n_explored_states = self.qlearner.values.len();
        if n_explored_states != 0 {
            println!("Have visited {} states at least once", n_explored_states);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_learn_one_game() {
        let mut agent = Agent::<3>::new();
        agent.learn_one_game();
    }

    #[test]
    fn test_learn() {
        let mut agent = Agent::<3>::new();
        agent.learn(1_000);
    }
}

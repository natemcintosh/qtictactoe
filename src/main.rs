use std::io;
use std::str::FromStr;

use clap::Parser;
use rand::seq::SliceRandom;
use rand::thread_rng;

mod agent;
mod q_matrix;
mod tic_tac_toe;

fn get_user_input() -> (usize, usize) {
    println!("Enter row then column. E.g. 1,0");
    let mut raw_input = String::new();
    loop {
        match io::stdin().read_line(&mut raw_input) {
            Ok(_) => break,
            Err(_) => continue,
        }
    }
    let input = raw_input.trim().to_string();

    // Parse the string
    let pieces = input.split_once(',');
    match pieces {
        None => get_user_input(),
        Some((r, c)) => {
            let row_num: usize = match usize::from_str(r) {
                Ok(val) => val,
                Err(_) => return get_user_input(),
            };

            let col_num: usize = match usize::from_str(c) {
                Ok(val) => val,
                Err(_) => return get_user_input(),
            };

            (row_num, col_num)
        }
    }
}

fn play<const N: usize>(agent: &agent::Agent<N>) {
    let mut game = tic_tac_toe::Board::<N>::new();
    let mut player = tic_tac_toe::Player::X;
    let mut action: (usize, usize);
    let mut rng = thread_rng();
    loop {
        let (new_action, _) = agent.qlearner.max_action_for_state(game);
        match new_action {
            // The agent never learned this configuration
            None => {
                let valid_actions = game.get_empty_spots();
                println!("Learner never came across this situation");
                action = *valid_actions
                    .choose(&mut rng)
                    .expect("No valid states to randomly pick from");
            }
            // The agent knows what to do
            Some(act) => {
                action = act;
            }
        }
        let winner = game.make_move(player, action.0, action.1);
        println!("{}", game);
        player = player.next_player();
        if let Some(res) = winner {
            match res {
                tic_tac_toe::GameResult::XWon => {
                    println!("=========== You Lost ===========");
                    return;
                }
                tic_tac_toe::GameResult::OWon => {
                    println!("=========== You Won ===========");
                    return;
                }
                tic_tac_toe::GameResult::Tie => {
                    println!("=========== Tie ===========");
                    return;
                }
            }
        }

        let (x, y) = get_user_input();
        let winner = game.make_move(player, x, y);
        player = player.next_player();
        println!("{}", game);
        if let Some(res) = winner {
            match res {
                tic_tac_toe::GameResult::XWon => {
                    println!("=========== You Lost ===========");
                    return;
                }
                tic_tac_toe::GameResult::OWon => {
                    println!("=========== You Won ===========");
                    return;
                }
                tic_tac_toe::GameResult::Tie => {
                    println!("=========== Tie ===========");
                    return;
                }
            }
        }
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// How many iterations to train for
    #[arg(value_parser = clap::value_parser!(usize))]
    #[arg(default_value_t = 1000000)]
    n_iters: usize,
}

fn main() {
    let cli = Cli::parse();

    let n_iters = cli.n_iters;

    let mut q_agent = agent::Agent::<4>::new();
    let start_time = std::time::Instant::now();
    println!("Learning for {n_iters} iterations");
    q_agent.learn(n_iters);
    println!("Learning took {:.2} s", start_time.elapsed().as_secs_f32());

    loop {
        println!("\nLet's play\n");
        play(&q_agent);
    }
}

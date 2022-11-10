# qtictactoe
A Q learning project for my Data Mining class. The structure comes from [this python repository](https://github.com/alirezamika/tutorials/tree/master/qtictactoe), although edits have been made to make it more generic, and fit rust's paradigms.

This repository has the following components:
- `q_matrix.rs` is the logic for storing knowledge learned
- `agent.rs` is the logic for learning over many repetitions
- `tic_tac_toe.rs` is the logic for the game of tic-tac-toe. It is (almost) generic to a game with N sides.
- `main.rs` is how you train, and then play against the agent.

## Usage
To run this, please follow these steps
1. Make sure you have the [rust compiler](https://www.rust-lang.org/tools/install) installed
1. Clone this repository
1. Set the desired board size in `src/main.rs`. Default is 3x3
1. Compile and run with `cargo run --release`

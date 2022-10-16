# qtictactoe
A Q learning project for my Data Mining class. The structure comes from [this python repository](https://github.com/alirezamika/tutorials/tree/master/qtictactoe), although edits have been made to make it more generic, and fit rust's paradigms.

This repository has several components:
- `q_matrix.rs` is the logic for storing knowledge learned
- `agent.rs` is the logic for learning over many repetitions
- `tic_tac_toe.rs` is the logic for the game of tic-tac-toe. It is (almost) generic to a game with N sides.
- `main.rs` is how you train, and then play against the agent.

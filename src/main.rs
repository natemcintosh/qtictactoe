mod agent;
mod q_matrix;
mod tic_tac_toe;

use crate::tic_tac_toe::*;

fn main() {
    let mut b = Board::<3>::new();
    b.board[2][2] = Piece::X;
    let want = [Piece::Empty, Piece::Empty, Piece::X];

    for (g, w) in b.get_lr_diag().zip(want.into_iter()) {
        dbg!(w, g);
        assert_eq!(w, g);
    }
}

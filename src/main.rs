use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Player {
    X,
    O,
}

impl Player {
    fn next_player(&self) -> Player {
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Piece {
    Empty = 0,
    X = 1,
    O = -1,
}

impl fmt::Display for Piece {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Piece::Empty => write!(f, "."),
            Piece::X => write!(f, "X"),
            Piece::O => write!(f, "O"),
        }
    }
}

#[derive(Debug, PartialEq)]
enum GameResult {
    XWon,
    OWon,
    Tie,
}

#[derive(Debug, Clone, Copy)]
struct Board<const N: usize> {
    board: [[Piece; N]; N],
}

impl<const N: usize> Board<N> {
    fn new() -> Self {
        Board {
            board: [[Piece::Empty; N]; N],
        }
    }

    /// Check if a player has won in row `row_num`
    fn row_winner(&self, row_num: usize) -> Option<Player> {
        match self.board[row_num].iter().map(|p| *p as i8).sum::<i8>() {
            3 => Some(Player::X),
            -3 => Some(Player::O),
            _ => None,
        }
    }

    /// Iterate over the items in a column
    fn get_col(&self, col_num: usize) -> impl Iterator<Item = Piece> + '_ {
        self.board.iter().map(move |&row| row[col_num])
    }

    /// Check if a player has won in column `col_num`
    fn col_winner(&self, col_num: usize) -> Option<Player> {
        match self.get_col(col_num).map(|p| p as i8).sum::<i8>() {
            3 => Some(Player::X),
            -3 => Some(Player::O),
            _ => None,
        }
    }

    /// Iterate over the diagonal from top left to bottom right
    fn get_lr_diag(&self) -> impl Iterator<Item = Piece> + '_ {
        (0_usize..self.board.len())
            .into_iter()
            .map(|idx| self.board[idx][idx])
    }

    /// Iterate over the diagonal from top right to bottom left
    fn get_rl_diag(&self) -> impl Iterator<Item = Piece> + '_ {
        let last_idx = self.board.len() - 1;
        (0_usize..self.board.len())
            .into_iter()
            .map(move |idx| self.board[idx][last_idx - idx])
    }

    /// Check if a player has won via a diagonal
    fn diagonal_winner(&self) -> Option<Player> {
        let lr_val = self.get_lr_diag().map(|p| p as i8).sum::<i8>();
        if lr_val == 3 {
            return Some(Player::X);
        } else if lr_val == -3 {
            return Some(Player::O);
        }

        let rl_val = self.get_rl_diag().map(|p| p as i8).sum::<i8>();
        if rl_val == 3 {
            return Some(Player::X);
        } else if rl_val == -3 {
            return Some(Player::O);
        }

        None
    }

    /// Return winner or tie if game over, otherwise None
    fn get_winner(&self) -> Option<GameResult> {
        // Check rows
        for row_num in 0_usize..3 {
            // If there is a winner, return a GameResult
            if let Some(winner) = self.row_winner(row_num) {
                return match winner {
                    Player::X => Some(GameResult::XWon),
                    Player::O => Some(GameResult::OWon),
                };
            }
        }

        // Check columns
        for col_num in 0_usize..3 {
            if let Some(winner) = self.col_winner(col_num) {
                return match winner {
                    Player::X => Some(GameResult::XWon),
                    Player::O => Some(GameResult::OWon),
                };
            }
        }

        // Check diagonals
        if let Some(winner) = self.diagonal_winner() {
            return match winner {
                Player::X => Some(GameResult::XWon),
                Player::O => Some(GameResult::OWon),
            };
        }

        None
    }
}

impl<const N: usize> fmt::Display for Board<N> {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.board {
            for item in row {
                write!(f, "{} ", item)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn main() {
    let mut b = Board::<3>::new();
    b.board[2][2] = Piece::X;
    let want = [Piece::Empty, Piece::Empty, Piece::X];

    for (g, w) in b.get_lr_diag().zip(want.into_iter()) {
        dbg!(w, g);
        assert_eq!(w, g)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_row_winner_all_empties() {
        let empty_board = Board::<3>::new();
        for row_num in 0..3 {
            assert_eq!(None, empty_board.row_winner(row_num))
        }
    }

    #[test]
    fn test_row_winner_mixed() {
        let mut b = Board::<3>::new();
        b.board[0][0] = Piece::X;
        b.board[1][1] = Piece::X;
        b.board[2][1] = Piece::X;
        assert_eq!(None, b.row_winner(0));
        assert_eq!(None, b.row_winner(1));
        assert_eq!(None, b.row_winner(2));
    }

    #[test]
    fn test_row_winner_x_wins_first_row() {
        let mut b = Board::<3>::new();
        b.board[0][0] = Piece::X;
        b.board[0][1] = Piece::X;
        b.board[0][2] = Piece::X;
        assert_eq!(Some(Player::X), b.row_winner(0));
    }

    #[test]
    fn test_row_winner_x_wins_second_row() {
        let mut b = Board::<3>::new();
        b.board[1][0] = Piece::X;
        b.board[1][1] = Piece::X;
        b.board[1][2] = Piece::X;
        assert_eq!(Some(Player::X), b.row_winner(1));
    }

    #[test]
    fn test_row_winner_o_wins_first_row() {
        let mut b = Board::<3>::new();
        b.board[0][0] = Piece::O;
        b.board[0][1] = Piece::O;
        b.board[0][2] = Piece::O;
        assert_eq!(Some(Player::O), b.row_winner(0));
    }

    #[test]
    fn test_row_winner_o_wins_third_row() {
        let mut b = Board::<3>::new();
        b.board[2][0] = Piece::O;
        b.board[2][1] = Piece::O;
        b.board[2][2] = Piece::O;
        assert_eq!(Some(Player::O), b.row_winner(2));
    }

    #[test]
    fn test_col_winner_all_empties() {
        let empty_board = Board::<3>::new();
        for row_num in 0..3 {
            assert_eq!(None, empty_board.col_winner(row_num))
        }
    }

    #[test]
    fn test_col_winner_mixed() {
        let mut b = Board::<3>::new();
        b.board[0][0] = Piece::X;
        b.board[1][1] = Piece::X;
        b.board[2][1] = Piece::X;
        assert_eq!(None, b.col_winner(0));
        assert_eq!(None, b.col_winner(1));
        assert_eq!(None, b.col_winner(2));
    }

    #[test]
    fn test_col_winner_x_wins_first_col() {
        let mut b = Board::<3>::new();
        b.board[0][0] = Piece::X;
        b.board[1][0] = Piece::X;
        b.board[2][0] = Piece::X;
        assert_eq!(Some(Player::X), b.col_winner(0));
    }

    #[test]
    fn test_col_winner_x_wins_second_col() {
        let mut b = Board::<3>::new();
        b.board[0][1] = Piece::X;
        b.board[1][1] = Piece::X;
        b.board[2][1] = Piece::X;
        assert_eq!(Some(Player::X), b.col_winner(1));
    }

    #[test]
    fn test_col_winner_o_wins_first_col() {
        let mut b = Board::<3>::new();
        b.board[0][0] = Piece::O;
        b.board[1][0] = Piece::O;
        b.board[2][0] = Piece::O;
        assert_eq!(Some(Player::O), b.col_winner(0));
    }

    #[test]
    fn test_col_winner_o_wins_third_col() {
        let mut b = Board::<3>::new();
        b.board[0][2] = Piece::O;
        b.board[1][2] = Piece::O;
        b.board[2][2] = Piece::O;
        assert_eq!(Some(Player::O), b.col_winner(2));
    }

    #[test]
    fn test_get_winner_x_row1() {
        let mut b = Board::<3>::new();
        b.board[0][0] = Piece::X;
        b.board[0][1] = Piece::X;
        b.board[0][2] = Piece::X;
        assert_eq!(Some(GameResult::XWon), b.get_winner())
    }

    #[test]
    fn test_get_winner_x_row2() {
        let mut b = Board::<3>::new();
        b.board[1][0] = Piece::X;
        b.board[1][1] = Piece::X;
        b.board[1][2] = Piece::X;
        assert_eq!(Some(GameResult::XWon), b.get_winner())
    }

    #[test]
    fn test_get_winner_x_row3() {
        let mut b = Board::<3>::new();
        b.board[2][0] = Piece::X;
        b.board[2][1] = Piece::X;
        b.board[2][2] = Piece::X;
        assert_eq!(Some(GameResult::XWon), b.get_winner())
    }

    #[test]
    fn test_get_winner_0_row1() {
        let mut b = Board::<3>::new();
        b.board[0][0] = Piece::O;
        b.board[0][1] = Piece::O;
        b.board[0][2] = Piece::O;
        assert_eq!(Some(GameResult::OWon), b.get_winner())
    }

    #[test]
    fn test_get_winner_0_row2() {
        let mut b = Board::<3>::new();
        b.board[1][0] = Piece::O;
        b.board[1][1] = Piece::O;
        b.board[1][2] = Piece::O;
        assert_eq!(Some(GameResult::OWon), b.get_winner())
    }

    #[test]
    fn test_get_winner_0_row3() {
        let mut b = Board::<3>::new();
        b.board[2][0] = Piece::O;
        b.board[2][1] = Piece::O;
        b.board[2][2] = Piece::O;
        assert_eq!(Some(GameResult::OWon), b.get_winner())
    }

    #[test]
    fn test_get_winner_x_col1() {
        let mut b = Board::<3>::new();
        b.board[0][0] = Piece::X;
        b.board[1][0] = Piece::X;
        b.board[2][0] = Piece::X;
        assert_eq!(Some(GameResult::XWon), b.get_winner())
    }

    #[test]
    fn test_get_winner_x_col2() {
        let mut b = Board::<3>::new();
        b.board[0][1] = Piece::X;
        b.board[1][1] = Piece::X;
        b.board[2][1] = Piece::X;
        assert_eq!(Some(GameResult::XWon), b.get_winner())
    }

    #[test]
    fn test_get_winner_x_col3() {
        let mut b = Board::<3>::new();
        b.board[0][2] = Piece::X;
        b.board[1][2] = Piece::X;
        b.board[2][2] = Piece::X;
        assert_eq!(Some(GameResult::XWon), b.get_winner())
    }

    #[test]
    fn test_get_winner_o_col1() {
        let mut b = Board::<3>::new();
        b.board[0][0] = Piece::O;
        b.board[1][0] = Piece::O;
        b.board[2][0] = Piece::O;
        assert_eq!(Some(GameResult::OWon), b.get_winner())
    }

    #[test]
    fn test_get_winner_o_col2() {
        let mut b = Board::<3>::new();
        b.board[0][1] = Piece::O;
        b.board[1][1] = Piece::O;
        b.board[2][1] = Piece::O;
        assert_eq!(Some(GameResult::OWon), b.get_winner())
    }

    #[test]
    fn test_get_winner_o_col3() {
        let mut b = Board::<3>::new();
        b.board[0][2] = Piece::O;
        b.board[1][2] = Piece::O;
        b.board[2][2] = Piece::O;
        assert_eq!(Some(GameResult::OWon), b.get_winner())
    }

    #[test]
    fn test_get_lr_diag_1() {
        let b = Board::<3>::new();
        for p in b.get_lr_diag() {
            assert_eq!(Piece::Empty, p)
        }
    }

    #[test]
    fn test_get_lr_diag_2() {
        let mut b = Board::<3>::new();
        b.board[2][2] = Piece::X;
        let want = [Piece::Empty, Piece::Empty, Piece::X];

        for (g, w) in b.get_lr_diag().zip(want.into_iter()) {
            assert_eq!(w, g)
        }
    }

    #[test]
    fn test_get_rl_diag_1() {
        let b = Board::<3>::new();
        for p in b.get_rl_diag() {
            assert_eq!(Piece::Empty, p)
        }
    }

    #[test]
    fn test_get_rl_diag_2() {
        let mut b = Board::<3>::new();
        b.board[1][1] = Piece::X;
        let want = [Piece::Empty, Piece::X, Piece::Empty];

        for (g, w) in b.get_rl_diag().zip(want.into_iter()) {
            assert_eq!(w, g)
        }
    }

    #[test]
    fn test_get_rl_diag_3() {
        let mut b = Board::<3>::new();
        b.board[1][1] = Piece::X;
        b.board[2][0] = Piece::O;
        let want = [Piece::Empty, Piece::X, Piece::O];
        println!("{}", b);

        for (g, w) in b.get_rl_diag().zip(want.into_iter()) {
            dbg!(w, g);
            assert_eq!(w, g);
        }
    }

    #[test]
    fn test_get_winner_o_lr_diag() {
        let mut b = Board::<3>::new();
        b.board[0][0] = Piece::O;
        b.board[1][1] = Piece::O;
        b.board[2][2] = Piece::O;
        println!("{}", b);
        assert_eq!(Some(GameResult::OWon), b.get_winner())
    }

    #[test]
    fn test_get_winner_o_rl_diag() {
        let mut b = Board::<3>::new();
        b.board[0][2] = Piece::O;
        b.board[1][1] = Piece::O;
        b.board[2][0] = Piece::O;
        println!("{}", b);
        assert_eq!(Some(GameResult::OWon), b.get_winner())
    }

    #[test]
    fn test_get_winner_x_lr_diag() {
        let mut b = Board::<3>::new();
        b.board[0][0] = Piece::X;
        b.board[1][1] = Piece::X;
        b.board[2][2] = Piece::X;
        println!("{}", b);
        assert_eq!(Some(GameResult::XWon), b.get_winner())
    }

    #[test]
    fn test_get_winner_x_rl_diag() {
        let mut b = Board::<3>::new();
        b.board[0][2] = Piece::X;
        b.board[1][1] = Piece::X;
        b.board[2][0] = Piece::X;
        println!("{}", b);
        assert_eq!(Some(GameResult::XWon), b.get_winner())
    }
}

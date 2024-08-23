
use std::iter::zip;

pub const BOARD_WIDTH : usize = 8;
pub const WIN_DIRECTIONS : [[i32 ; 2] ; 4 ] = 
    [[0, 1], [1, 0], [1, 1], [1, -1]];
pub const WINNING_LENGTH : i32 = 5;


#[derive(Clone, PartialEq, Eq)]
pub enum Turn {
    X,
    O
}

impl Turn {
    pub fn to_char(&self) -> char {
        match self{
            Turn::X => 'x',
            Turn::O => 'o'
        }
    }
}

#[derive(Clone)]
pub struct TicTacToeBoard {
    x_board: u64,
    o_board: u64,
    pub turn: Turn,
    pub terminated: bool,
}

impl TicTacToeBoard {
    fn get_shift(index : usize) -> usize {
        // Get BOARD_WIDTH - index - 1
        BOARD_WIDTH - index - 1
    }

    fn get_shift_from_row_col(row: usize, col: usize) -> usize {
        // Gets the shift index for given row and col
        // e.g. get_shift(0, 0) -> 63
        // e.g. get_shift(7, 7) -> 0
        BOARD_WIDTH * (BOARD_WIDTH - row - 1) + BOARD_WIDTH - col - 1
    }
    pub fn print_board(&self) {
        // Prints the state of the board
        print!("  ");
        let x_bytes = self.x_board.to_be_bytes();
        let o_bytes = self.o_board.to_be_bytes();
        let mut board = vec![vec![' '; BOARD_WIDTH]; BOARD_WIDTH];
        for (i, (x, o)) in zip(x_bytes.iter(), o_bytes.iter()).enumerate() {
            for j in 0..BOARD_WIDTH {
                let shift : usize = (BOARD_WIDTH - j - 1).try_into().unwrap();
                if (x >> shift) & 1 == 1 {
                    board[i][j] = Turn::X.to_char();
                }
                if (o >> shift) & 1 == 1{
                    board[i][j] = Turn::O.to_char();
                }
            }
        }
        // Print the board
        for i in 0..BOARD_WIDTH {
            print!("{} ", i + 1);
        }
        print!("\n");

        for (row_i, row) in board.iter().enumerate() {
            print!("{} ", row_i + 1);
            for item in row {
                print!("{item} ");
            }
            print!("\n");
        }

    }

    pub fn valid_move(&self, row: usize, col: usize) -> bool {
        let is_taken = (self.x_board & self.o_board).to_be_bytes()[row];
        return (is_taken >> Self::get_shift(col)) & 1 == 0 && !self.terminated;
    }

    pub fn make_move(&mut self, row: usize, col: usize) -> bool {
        // Places the current player's piece at (row, col) and checks for a win
        // if a win has occurred, then update the board state
        // returns whether the game is over
        if self.terminated {
            panic!("Game already terminated!");
        }

        if !self.valid_move(row, col) {
            panic!("Invalid move on {}, {}", row + 1, col + 1);
        }
        let shift = Self::get_shift_from_row_col(row, col);

        match self.turn {
            Turn::X => { self.x_board |= 1 << shift },
            Turn::O => { self.o_board |= 1 << shift }
        }

        if self.check_win(row, col) {
            self.terminated = true;
            return true;
        }

        self.turn = match self.turn {
            Turn::X => Turn::O,
            Turn::O => Turn::X
        };
        false
    }

    pub fn avaliable_moves(&self) -> Vec<(usize, usize)> {
        // Returns a vector of all possible moves
        let mut moves: Vec<(usize, usize)> = Vec::new();
        let spaces = !(self.x_board | self.o_board);
        for i in (0..u64::BITS).rev() {
            if spaces >> i & 1 == 1 {
                let index : usize = (u64::BITS - i - 1).try_into().unwrap();
                moves.push((index / BOARD_WIDTH, index % BOARD_WIDTH))
            }
        }
        return moves;
    }

    fn check_win(&self, row: usize, col: usize) -> bool {
        for [up, right] in WIN_DIRECTIONS{
            let mut count = 1;
            let last_move = self.turn.to_char();
            for direction in [1, -1] {
                for i in 1..WINNING_LENGTH {
                    let check_row: i32 = row as i32 + up * i * direction;
                    let check_col: i32 = col as i32 + right * i * direction;
                    if check_row < 0 || check_row >= BOARD_WIDTH as i32 || check_col < 0 || check_col >= BOARD_WIDTH as i32 {
                        break;
                    }
                    let check_row : usize = usize::try_from(check_row).unwrap();
                    let check_col : usize = usize::try_from(check_col).unwrap();
                    if self.get_at(check_row, check_col) == last_move {
                        count += 1;
                    } else {
                        break
                    }
                    if count >= WINNING_LENGTH {
                        return true;
                    }
                }
            }
        }
        false
    }

    pub fn get_at(&self, row: usize, col: usize) -> char {
        let x_bytes = self.x_board.to_be_bytes();
        let o_bytes = self.o_board.to_be_bytes();
        let col = BOARD_WIDTH - col - 1;
        let has_x = x_bytes[row] >> col;
        let has_o = o_bytes[row] >> col;
        if has_x & 1 == 1 {
            'x'
        } else if has_o & 1 == 1 {
            'o'
        } else {
            ' '
        }
    }
}

pub fn build_tictactoeboard() -> TicTacToeBoard {
    TicTacToeBoard {
        x_board: 0,
        o_board: 0,
        turn: Turn::X,
        terminated: false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_board() {
        let tictactoe = build_tictactoeboard();
        assert_eq!(tictactoe.avaliable_moves().len(), (BOARD_WIDTH * BOARD_WIDTH).try_into().unwrap());
    }
}
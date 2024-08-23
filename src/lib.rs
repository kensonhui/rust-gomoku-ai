use std::{cmp::{max, min, Ordering}, collections::HashMap};
use itertools::Itertools;
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
pub trait TicTacToeBot {
    fn heuristic(&self, state: &TicTacToeBoard) -> i32;
    fn turn(&self) -> &Turn;
    fn minmax(&self, 
        state: &TicTacToeBoard, depth: i32, 
        role: MinMaxNodeRole, mut alpha: i32, mut beta: i32) -> (i32, (usize, usize)) {
        if depth == 0 || state.terminated {
            return (self.heuristic(state), (BOARD_WIDTH, BOARD_WIDTH));
        }
        let possible_moves = state.avaliable_moves();
        let mut score = match role {
            MinMaxNodeRole::Maximizer => i32::MIN,
            MinMaxNodeRole::Minimizer => i32::MAX
        };
        let mut action = possible_moves[0];

        for (row, col) in possible_moves {
            let mut copy_board : TicTacToeBoard = state.clone();
            copy_board.make_move(row, col);
            let (child_score, _)  = self.minmax(
                &copy_board, depth - 1, 
                match role {
                        MinMaxNodeRole::Maximizer => MinMaxNodeRole::Minimizer,
                        MinMaxNodeRole::Minimizer => MinMaxNodeRole::Maximizer,
                    }, alpha, beta);
            match role {
               MinMaxNodeRole::Maximizer => {
                    if score < child_score {
                        action = (row, col);
                    }
                    score = max(child_score, score);
                    alpha = max(alpha, score);
                },
               MinMaxNodeRole::Minimizer => {
                    if score > child_score {
                        action = (row, col);
                    }
                    score = min(child_score, score);
                    beta = min(score, beta);
                }
            }
    
            if beta <= alpha {
                break;
            }
        }
        return (score, action);
    }
    fn make_move (&self, state: &TicTacToeBoard, depth: i32) -> (i32, (usize, usize)) {
        return self.minmax(state, depth, MinMaxNodeRole::Maximizer, i32::MIN, i32::MAX);
    }
}
pub struct SimpleBot {
    pub turn: Turn
}

fn running_count(player: char, item: char, count: &mut i32) {
    if item == player {
        *count += 1;
    } else if item == ' ' {
        *count = 0;
    } else {
        *count -= 1;
    }
}

impl TicTacToeBot for SimpleBot {
    fn turn(&self) -> &Turn {
        return &self.turn
    }
    fn heuristic(&self, state: &TicTacToeBoard) -> i32 {
        let mut three_in_a_rows = 0;
        let mut four_in_a_rows = 0;
        let player = self.turn.to_char();

        if state.terminated {
            if state.turn == self.turn {
                return i32::MAX;
            } else {
                return i32::MIN;
            }
        }

        for row in 0..BOARD_WIDTH {
            let mut count = 0;
            for col in 0..BOARD_WIDTH {
                let item = state.get_at(row, col);
                running_count(player, item, &mut count);
                if count == 3 {
                    three_in_a_rows += 1;
                } else if count == -3 {
                    three_in_a_rows -= 1;
                }
            }
        }

        for col in 0..BOARD_WIDTH {
            let col : usize = col.try_into().unwrap();
            let mut count = 0;
            for row in 0..BOARD_WIDTH {
                let row : usize = row.try_into().unwrap();
                let item : char = state.get_at(row, col);
                running_count(player, item, &mut count);
                if count == 3 {
                    three_in_a_rows += 1;
                } else if count == -3 {
                    three_in_a_rows -= 1;
                }
            }
        }

        for (row, col) in (0..BOARD_WIDTH).cartesian_product(0..1)
            .chain((0..1).cartesian_product(0..BOARD_WIDTH)) {
                let mut count = 0;
                for i in 0..min(BOARD_WIDTH - row - 1, BOARD_WIDTH- col - 1) {
                    let row : usize = row.try_into().unwrap();
                    let col : usize = col.try_into().unwrap();
                    let i : usize = i.try_into().unwrap();
                    let item = state.get_at(row + i, col + i);
                    running_count(player, item, &mut count);
                    if count == 3 {
                        three_in_a_rows += 1;
                    } else if count == -3 {
                        three_in_a_rows -= 1;
                    }
                }
        }

        for (row, col) in (BOARD_WIDTH - 1..BOARD_WIDTH)
            .cartesian_product(BOARD_WIDTH - 1..BOARD_WIDTH)
            .chain((0..BOARD_WIDTH).cartesian_product(0..1)) {
                let mut count = 0;
                for i in 0..min(row + 1, BOARD_WIDTH - col - 1) {
                    let i : usize = i.try_into().unwrap();
                    let row : usize = row.try_into().unwrap();
                    let col : usize = col.try_into().unwrap();
                    let item = state.get_at(row - i, col + i);
                    running_count(player, item, &mut count);
                    if count == 3 {
                        three_in_a_rows += 1;
                    } else if count == -3 {
                        three_in_a_rows -= 1;
                    }
                }
        }
        return three_in_a_rows;
    }
}

pub enum MinMaxNodeRole {
    Maximizer,
    Minimizer
}
pub struct MinMaxNode {
    score: i32,
    best_move: (usize, usize),
    states_evaluated: u32
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
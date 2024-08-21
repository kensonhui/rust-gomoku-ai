use std::cmp::{max, min};
use itertools::Itertools;

pub const BOARD_HEIGHT : i32 = 8;
pub const BOARD_WIDTH : i32 = 8;
pub const WIN_DIRECTIONS : [[i32 ; 2] ; 4 ] = 
    [[0, 1], [1, 0], [1, 1], [1, -1]];
pub const WINNING_LENGTH : i32 = 5;

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

pub struct TicTacToeBoard {
    pub board: Vec<Vec<char>>,
    pub turn: Turn,
    pub terminated: bool,
}

impl TicTacToeBoard {
    pub fn print_board(&self) {
        // Prints the state of the board
        print!("  ");
        for i in 0..BOARD_WIDTH {
            print!("{} ", i + 1);
        }
        print!("\n");
        for (row_i, row) in self.board.iter().enumerate() {
            print!("{} ", row_i + 1);
            for item in row {
                print!("{item} ");
            }
            print!("\n");
        }
    }

    pub fn make_move(&mut self, row: &i32, col: &i32) -> bool {
        // Places the current player's piece at (row, col) and checks for a win
        let row : usize = *row as usize;
        let col: usize = *col as usize;
        if self.board[row][col] != ' ' {
            return false
        }
        self.board[row][col] = self.turn.to_char();

        if self.check_win(row, col) {
            print!("Player {} wins!", self.turn.to_char());
            self.terminated = true;
            return true;
        }

        self.turn = match self.turn {
            Turn::X => Turn::O,
            Turn::O => Turn::X
        };
        true
    }

    fn check_win(&self, row: usize, col: usize) -> bool {
        let row : i32 = row as i32;
        let col : i32 = col as i32;
        for [up, right] in WIN_DIRECTIONS{
            let mut count = 1;
            let last_move = self.turn.to_char();
            for direction in [1, -1] {
                for i in 1..WINNING_LENGTH {
                    let check_y = row + up * i * direction;
                    let check_x = col + right * i * direction;
                    if check_y < 0 || check_y >= BOARD_HEIGHT || check_x < 0 || check_x >= BOARD_WIDTH {
                        break;
                    }
                    if self.board[usize::try_from(check_y).unwrap()][usize::try_from(check_x).unwrap()] == last_move {
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
}

pub fn build_tictactoeboard() -> TicTacToeBoard {
    TicTacToeBoard {
        board: vec![vec![' '; BOARD_WIDTH as usize]; BOARD_HEIGHT as usize],
        turn: Turn::X,
        terminated: false
    }
}
pub trait TicTacToeBot {
    fn heuristic(&self, state: &TicTacToeBoard) -> i32;
    //fn choose_move(state: &TicTacToeBoard) -> (i32, i32);
}
pub struct Simple {}

fn running_count(player: char, item: char, count: &mut i32, max_count: &mut i32, min_count: &mut i32) {
    if item == player {
        *count = max(*count, 0);
        *count += 1;
        *max_count = max(*max_count, *count);
    } else if item == ' ' {
        *count = 0;
    } else {
        *count = min(*count, 0);
        *count -= 1;
        *min_count = min(*min_count, *count);
    }
}

impl TicTacToeBot for Simple {
    fn heuristic(&self, state: &TicTacToeBoard) -> i32 {
        let mut max_count = 0;
        let mut min_count = 0;
        let player = state.turn.to_char();

        for row in &state.board {
            let mut count = 0;
            for item in row {
                let item = *item;
                running_count(player, item, &mut count, &mut max_count, &mut min_count);
            }
        }

        for col in 0..BOARD_WIDTH {
            let col : usize = col.try_into().unwrap();
            let mut count = 0;
            for row in 0..BOARD_HEIGHT {
                let row : usize = row.try_into().unwrap();
                let item : char = state.board[row][col];
                running_count(player, item, &mut count, &mut max_count, &mut min_count);
            }
        }

        for (row, col) in (0..BOARD_HEIGHT).cartesian_product(0..1)
            .chain((0..1).cartesian_product(0..BOARD_WIDTH)) {
                let mut count = 0;
                for i in 0..min(BOARD_HEIGHT - row - 1, BOARD_WIDTH- col - 1) {
                    let row : usize = row.try_into().unwrap();
                    let col : usize = col.try_into().unwrap();
                    let i : usize = i.try_into().unwrap();
                    let item = state.board[row + i][col + i];
                    running_count(player, item, &mut count, &mut max_count, &mut min_count);
                }
        }

        for (row, col) in (BOARD_HEIGHT - 1..BOARD_HEIGHT)
            .cartesian_product(BOARD_WIDTH - 1..BOARD_WIDTH)
            .chain((0..BOARD_HEIGHT).cartesian_product(0..1)) {
                let mut count = 0;
                for i in 0..min(row + 1, BOARD_WIDTH - col - 1) {
                    let i : usize = i.try_into().unwrap();
                    let row : usize = row.try_into().unwrap();
                    let col : usize = col.try_into().unwrap();
                    let item = state.board[row - i][col + i];
                    running_count(player, item, &mut count, &mut max_count, &mut min_count);
                }
        }
        print!{"Max count {max_count}, Min count {min_count}"};
        return max_count + min_count;
    }
}
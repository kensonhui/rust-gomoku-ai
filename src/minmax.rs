use std::cmp::{max, min};
use itertools::Itertools;

use crate::gomoku::{TicTacToeBoard, Turn, BOARD_WIDTH};
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
use std::{cmp::{max, min, Ordering}, collections::HashMap};
use itertools::Itertools;

pub const BOARD_HEIGHT : usize = 8;
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

    pub fn valid_move(&self, row: usize, col: usize) -> bool {
        return self.board[row][col] == ' ' && !self.terminated;
    }

    pub fn make_move(&mut self, row: usize, col: usize) -> bool {
        // Places the current player's piece at (row, col) and checks for a win
        // if a win has occurred, then update the board state
        // returns whether the game is over
        if self.terminated {
            panic!("Game already terminated!");
        }

        if self.board[row][col] != ' ' {
            panic!("Invalid move on {}, {}", row + 1, col + 1);
        }
        self.board[row][col] = self.turn.to_char();

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
        for (i, row) in self.board.iter().enumerate() {
            for (j, item) in row.iter().enumerate() {
                if *item == ' ' {
                    moves.push((i, j));
                }
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
                    let check_y: i32 = row as i32 + up * i * direction;
                    let check_x: i32 = col as i32 + right * i * direction;
                    if check_y < 0 || check_y >= BOARD_HEIGHT as i32 || check_x < 0 || check_x >= BOARD_WIDTH as i32 {
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
    fn turn(&self) -> &Turn;
    fn build_node(&self, node: & mut MinMaxNode, 
        state: &TicTacToeBoard, depth: i32, 
        role: MinMaxNodeRole) {
        if depth == 0 {
            node.score = self.heuristic(state);
            node.states_evaluated = 1;
            return;
        }
        let possible_moves = state.avaliable_moves();
        for (row, col) in possible_moves {
            let mut copy_board : TicTacToeBoard = state.clone();
            copy_board.make_move(row, col);
            let mut child_node = 
                MinMaxNode { children: HashMap::new(), 
                    score: match role {
                        MinMaxNodeRole::Maximizer => i32::MAX,
                        MinMaxNodeRole::Minimizer => i32::MIN
                    }, 
                    states_evaluated: 0,
                    best_move: (BOARD_HEIGHT, BOARD_WIDTH) };
            if copy_board.terminated {
                if copy_board.turn == *self.turn() {
                    child_node.score = i32::MAX;
                } else {
                    child_node.score = i32::MIN;
                }
            } else {
                self.build_node(&mut child_node, &copy_board, depth - 1, 
                    match role {
                            MinMaxNodeRole::Maximizer => MinMaxNodeRole::Minimizer,
                            MinMaxNodeRole::Minimizer => MinMaxNodeRole::Maximizer});
            }
            node.children.insert((row, col), child_node);
        }

        let acc_function = match role {
            MinMaxNodeRole::Maximizer => Ordering::Greater,
            _ => Ordering::Less
        };


        let min_max_value = match role {
            MinMaxNodeRole::Maximizer => i32::MAX,
            _ => i32::MIN
        };
        
        for (action, child) in node.children.iter() {
            //print!(" - ({}, {}) -> {}\n", action.0 + 1, action.1 + 1, child.score);
            let comparison = child.score.cmp(&node.score);
            if comparison == acc_function || comparison == Ordering::Equal {
                    node.score = child.score;
                    node.best_move = *action;
            }
            node.states_evaluated += child.states_evaluated;
            if child.score == min_max_value {
                break;
            }
        };

        // once node's scores are evaluated, we can free memory
        node.children.clear();
    }
    fn make_move (&self, state: &TicTacToeBoard, depth: i32) -> ((usize, usize), i32, u32) {
        let mut root_node = MinMaxNode{
            children: HashMap::new(),
            score: i32::MIN,
            best_move: (BOARD_HEIGHT, BOARD_WIDTH),
            states_evaluated: 0
        };
        self.build_node(&mut root_node, state, depth, MinMaxNodeRole::Maximizer);
        return (root_node.best_move, root_node.score, root_node.states_evaluated);
    }
}
pub struct SimpleBot {
    pub turn: Turn
}

fn running_count(player: char, item: char, count: &mut i32, 
    max_count: &mut i32, min_count: &mut i32) {
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

impl TicTacToeBot for SimpleBot {
    fn turn(&self) -> &Turn {
        return &self.turn
    }
    fn heuristic(&self, state: &TicTacToeBoard) -> i32 {
        let mut max_count = 0;
        let mut min_count = 0;
        let player = self.turn.to_char();

        if state.terminated {
            if state.turn == self.turn {
                return i32::MAX;
            } else {
                return i32::MIN;
            }
        }

        for row in &state.board {
            let mut count = 0;
            for item in row {
                let item = *item;
                running_count(player, item, &mut count, 
                    &mut max_count, &mut min_count);
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
        return max_count + min_count;
    }
}

pub enum MinMaxNodeRole {
    Maximizer,
    Minimizer
}
pub struct MinMaxNode {
    children: HashMap<(usize, usize), MinMaxNode>,
    score: i32,
    best_move: (usize, usize),
    states_evaluated: u32
}

impl MinMaxNode {
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_board() {
        let tictactoe = build_tictactoeboard();
        assert_eq!(tictactoe.avaliable_moves().len(), (BOARD_WIDTH * BOARD_HEIGHT).try_into().unwrap());
    }
}
use std::io::{self, Write};
use std::convert::TryFrom;

const BOARD_HEIGHT : i32 = 8;
const BOARD_WIDTH : i32 = 8;
const WIN_DIRECTIONS : [[i32 ; 2] ; 4 ] = 
    [[0, 1], [1, 0], [1, 1], [1, -1]];
const WINNING_LENGTH : i32 = 5;

enum Turn {
    X,
    O
}

impl Turn {
    fn to_char(&self) -> char {
        match self{
            Turn::X => 'x',
            Turn::O => 'o'
        }
    }
}

struct TicTacToeBoard {
    board: Vec<Vec<char>>,
    turn: Turn,
    terminated: bool,
}

impl TicTacToeBoard {
    fn print_board(&self) {
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

    fn make_move(&mut self, row: &i32, col: &i32) -> bool {
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

fn build_tictactoeboard() -> TicTacToeBoard {
    TicTacToeBoard {
        board: vec![
            vec![' '; BOARD_WIDTH as usize]; 
            BOARD_HEIGHT as usize
            ],
        turn: Turn::X,
        terminated: false
    }
}

fn read_char(input: &mut i32, msg: &str) {
    print!("{}", msg);
    loop {
        io::stdout().flush().unwrap();
        let mut line: String = String::new();
        io::stdin().
            read_line(&mut line)
            .expect("Read user input");

        match line.trim().parse::<i32>() {
            Ok(num) => {
                if num > BOARD_HEIGHT || num < 0 {
                    print!("Number is out of bounds, try again, got {num} \n");
                    print!("{}", msg);
                } else {
                    *input = num;
                    return;
                }
            },
            _ => {
                print!("Please type a valid number, got {line} \n");
                print!("{}", msg);
            }
        }
    }
}

fn read_move() -> (i32, i32) {
    let mut row : i32 = -1;
    let mut col : i32 = -1;
    read_char(&mut row, "Type which row to insert into\n");
    read_char(&mut col, "Type what column to insert into\n");
    return (row - 1, col - 1)

}

fn main() {
    let mut tictactoe = build_tictactoeboard();
    loop {
        tictactoe.print_board();
        let (row, col) = read_move();
        if !tictactoe.make_move(&row, &col) {
            print!("Spot has already been taken, try again!\n");
        }
        if tictactoe.terminated {
            return;
        }
    };
}
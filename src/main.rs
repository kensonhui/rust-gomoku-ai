use std::io::{self, Write};
use std::time::Instant;
use minmax_bots::minmax::TicTacToeBot;

fn read_char(msg: &str) -> usize {
    print!("{}", msg);
    loop {
        io::stdout().flush().unwrap();
        let mut line: String = String::new();
        io::stdin().
            read_line(&mut line)
            .expect("Read user input");

        match line.trim().parse::<i32>() {
            Ok(num) => {
                if num > minmax_bots::gomoku::BOARD_WIDTH as i32 || num < 0 {
                    print!("Number is out of bounds, try again, got {num} \n");
                    print!("{}", msg);
                } else {
                    return num.try_into().unwrap();
                }
            },
            _ => {
                print!("Please type a valid number, got {line} \n");
                print!("{}", msg);
            }
        }
    }
}

fn read_move() -> (usize, usize) {
    let row : usize = read_char( "Type which row to insert into\n");
    let col : usize = read_char( "Type what column to insert into\n");
    return (row - 1, col - 1);
}

fn main() {
    let mut tictactoe = minmax_bots::gomoku::build_tictactoeboard();
    let simple = minmax_bots::minmax::SimpleBot{turn: minmax_bots::gomoku::Turn::O};
    loop {
        tictactoe.print_board();
        let (row, col) = read_move();
        if !tictactoe.valid_move(row, col) {
            print!("Spot has already been taken, try again!\n");
        } else {
            if tictactoe.make_move(row, col) {
                print!("Human wins\n");
                tictactoe.print_board();
                return;
            }
        }

        let now = Instant::now();
        let (ai_score, (bot_row, bot_col)) = 
            simple.make_move(&tictactoe, 4);
        let elapsed = now.elapsed();
       
        print!("Heuristic: {}, Time: {:.2?}\n", ai_score, elapsed);
        if tictactoe.make_move(bot_row, bot_col) {
            print!("You lose!");
            tictactoe.print_board();
            return;
        }
    };
}
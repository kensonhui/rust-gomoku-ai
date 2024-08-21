use std::io::{self, Write};
use minmax_bots::{BOARD_HEIGHT, TicTacToeBot};

fn read_char(msg: &str) -> i32 {
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
                    return num;
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
    let row : i32 = read_char( "Type which row to insert into\n");
    let col : i32 = read_char( "Type what column to insert into\n");
    return (row - 1, col - 1);
}

fn main() {
    let mut tictactoe = minmax_bots::build_tictactoeboard();
    let simple = minmax_bots::SimpleBot{};
    loop {
        tictactoe.print_board();
        let (row, col) = read_move();
        if !tictactoe.make_move(&row, &col) {
            print!("Spot has already been taken, try again!\n");
        }
        print!("Simple Heuristic {}\n", simple.heuristic(&tictactoe));
        if tictactoe.terminated {
            return;
        }
    };
}
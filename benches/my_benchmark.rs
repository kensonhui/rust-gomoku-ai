use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use minmax_bots::{gomoku::build_tictactoeboard, minmax::{SimpleBot, TicTacToeBot}};


fn criterion_benchmark(c: &mut Criterion) {
    let mut board = build_tictactoeboard();
    let ai = SimpleBot{turn: minmax_bots::gomoku::Turn::O};
    board.make_move(5, 5);
    c.bench_function("First move, depth of 4", 
        |b| b.iter(|| ai.make_move(black_box(&board), black_box(4))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
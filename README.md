# rust-gomoku-ai

# Gomoku AI Bot

This is a Gomoku AI bot built with Rust. This implements alpha-beta pruning minmax, bitboard representation boards, and a simple heurestic function.

## Setup

To run, install rust here: **[Rust Installation Guide](https://www.rust-lang.org/tools/install)**

Next, you can implement your own bot by declaring a structure, then implementing the heurestic function for TicTacToeBot. Within main, you can declare the depth that you wish for the minmax algorithm.

Then run:

```
cargo run
```

## Performance enhancements

I implemented a few optimizations inspired by chess engines to reduce memory usage and the search space.

Originally I made a naive implementation with Vec<Vec<char>>. I found that
Depth of 4: Heuristic: -1, States Evaluated: 14295960, Time: 133.13s

After implementing bit boards
Depth of 4: Heuristic: -1, States Evaluated: 14295960, Time: 113.33s

After implementing alpha-beta pruning
Depth of 4: Heuristic: -1, States Evaluated: 485277, Time: 3.95s

No longer storing children minmax nodes
Depth of 4: Heuristic: -1, States Evaluated: 485277, Time: 3.59s
Depth of 5: Heuristic: 0, States Evaluated: 6916967, Time: 50.53s

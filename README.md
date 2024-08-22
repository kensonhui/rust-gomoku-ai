# rust-gomoku-ai

This is a Gomoku ai bot built with Rust.

Naive implementation with Vec<Vec<char>>
Depth of 4: Heuristic: -1, States Evaluated: 14295960, Time: 133.13s

After implementing bit boards
Depth of 4: Heuristic: -1, States Evaluated: 14295960, Time: 113.33s

After implementing alpha-beta pruning
Depth of 4: Heuristic: -1, States Evaluated: 485277, Time: 3.95s

No longer storing children minmax nodes
Depth of 4: Heuristic: -1, States Evaluated: 485277, Time: 3.59s

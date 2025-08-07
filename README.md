# Countdown Numbers Game

A Rust implementation of the Countdown Numbers Game solver and puzzle generator. This project provides tools to generate and solve puzzles from the popular TV game show Countdown.

## Game Rules

> These are the rules as I understand them - they may be incorrect.

In the Countdown Numbers Game:

* Six numbers are provided: n "large" numbers and (6-n) "small" numbers
* Large numbers are drawn from {25, 50, 75, 100}
* Small numbers are drawn from 1-10, inclusive
* Target number is any three-digit number from 101-999
* Players must use basic arithmetic operations (addition, subtraction, multiplication, division) to reach the target number

## Usage

### Generate and Solve a Puzzle

To generate a random puzzle and solve it using both the bruteforce and minimax solvers:

```bash
cargo run
```

This command will:
1. Generate a classic Countdown Numbers Game puzzle
2. Display the numbers and target
3. Solve the puzzle using both solvers
4. Compare the solutions and performance

### Run Benchmarks

To run performance benchmarks for both solvers:

```bash
cargo bench
```

This will run a series of benchmarks testing both solvers against puzzles of varying complexity:
- Simple puzzles (2 numbers)
- Medium puzzles (6 numbers)
- Complex puzzles (6 numbers with challenging targets)
- Unsolvable puzzles

## Solvers

### Bruteforce Solver

The bruteforce solver uses a recursive approach to:
1. Generate all possible permutations of the input numbers
2. Try all possible operations between adjacent numbers
3. Find the equation that produces a result closest to the target

This approach guarantees finding the optimal solution if one exists, but can be computationally expensive for complex puzzles.

### Minimax Solver

The minimax solver adapts the classic game theory algorithm to the Countdown Numbers Game by:
1. Treating the puzzle as a single-player game
2. Using a heuristic function to evaluate how close intermediate results are to the target
3. Exploring the search space efficiently using depth-limited search

While minimax is typically used for adversarial games and is not them optimal approach for this problem, it provides an interesting comparison to the bruteforce method. The minimax solver may not always find the optimal solution but can be faster in some cases (but also pretty much never is).
* Minimax solver for performance comparison

## Running

```bash
cargo build          # Build the project
cargo test           # Run tests
cargo check          # Fast compilation check
cargo clippy         # Linting
cargo fmt            # Code formatting
```
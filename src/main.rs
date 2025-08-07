use countdown_numbers_game::{BruteForceSolver, MinimaxSolver, PuzzleGenerator};
use std::time::Instant;

fn main() {
    println!("Countdown Numbers Game - Solver Comparison\n");

    let mut generator = PuzzleGenerator::new();
    let puzzle = generator.generate_classic();

    println!("Generated Puzzle:");
    println!("  Numbers: {:?}", puzzle.numbers);
    println!("  Target: {}", puzzle.target);
    println!(
        "  Large numbers: {} | Small numbers: {}",
        puzzle.large_count,
        puzzle.small_count()
    );
    println!();

    println!("Solving with Brute Force...");
    let mut brute_force = BruteForceSolver::new();
    let start = Instant::now();
    let bf_result = brute_force.solve(puzzle.target, &puzzle.numbers);
    let bf_duration = start.elapsed();

    match &bf_result {
        Some(equation) => match equation.solve() {
            Ok(value) => println!("  Solution found: {} = {}", equation.format(), value),
            Err(e) => println!("  Equation error: {}", e),
        },
        None => println!("  No solution found"),
    }
    println!("  Permutations explored: {}", brute_force.permutation_count);
    println!("  Time taken: {:?}", bf_duration);
    println!();

    println!("Solving with Minimax...");
    let mut minimax = MinimaxSolver::with_depth(8);
    let start = Instant::now();
    let mm_result = minimax.solve(puzzle.target, &puzzle.numbers);
    let mm_duration = start.elapsed();

    match &mm_result {
        Some(equation) => match equation.solve() {
            Ok(value) => println!("  Solution found: {} = {}", equation.format(), value),
            Err(e) => println!("  Equation error: {}", e),
        },
        None => println!("  No solution found"),
    }
    println!("  Nodes explored: {}", minimax.nodes_explored);
    println!("  Time taken: {:?}", mm_duration);
    println!();

    println!("Comparison:");
    match (&bf_result, &mm_result) {
        (Some(bf_eq), Some(mm_eq)) => {
            let bf_value = bf_eq.solve().unwrap_or(0);
            let mm_value = mm_eq.solve().unwrap_or(0);

            if bf_value == puzzle.target && mm_value == puzzle.target {
                println!("  Both solvers found exact solutions!");
            } else if bf_value == puzzle.target {
                println!("  Brute force found exact solution, minimax approximation");
            } else if mm_value == puzzle.target {
                println!("  Minimax found exact solution, brute force approximation");
            } else {
                println!("  Both found approximations:");
                println!(
                    "    Brute force: {} (diff: {})",
                    bf_value,
                    (puzzle.target - bf_value).abs()
                );
                println!(
                    "    Minimax: {} (diff: {})",
                    mm_value,
                    (puzzle.target - mm_value).abs()
                );
            }
        }
        (Some(bf_eq), None) => {
            let bf_value = bf_eq.solve().unwrap_or(0);
            if bf_value == puzzle.target {
                println!("  Only brute force found an exact solution");
            } else {
                let diff = (puzzle.target - bf_value).abs();
                println!(
                    "  Minimax found no solution, brute force found approximation: {} (diff: {})",
                    bf_value, diff
                );
            }
        }
        (None, Some(mm_eq)) => {
            let mm_value = mm_eq.solve().unwrap_or(0);
            if mm_value == puzzle.target {
                println!("  Only minimax found an exact solution");
            } else {
                let diff = (puzzle.target - mm_value).abs();
                println!(
                    "  Brute force found no solution, minimax found close approximation: {} (diff: {})",
                    mm_value, diff
                );
            }
        }
        (None, None) => println!("  Neither solver found a solution"),
    }

    let speed_ratio = if bf_duration.as_nanos() > 0 {
        mm_duration.as_nanos() as f64 / bf_duration.as_nanos() as f64
    } else {
        1.0
    };

    if speed_ratio < 1.0 {
        println!("  Minimax was {:.2}x faster", 1.0 / speed_ratio);
    } else if speed_ratio > 1.0 {
        println!("  Brute force was {:.2}x faster", speed_ratio);
    } else {
        println!("  Both solvers took similar time");
    }

    println!("  Exploration efficiency:");
    println!(
        "    Brute force: {} permutations",
        brute_force.permutation_count
    );
    println!("    Minimax: {} nodes", minimax.nodes_explored);
}

/*!
# Minimax-Inspired Solver for the Countdown Numbers Game

This module implements a solver based on the minimax algorithm, though it should be noted that
the Countdown Numbers game is not actually an adversarial game where minimax would naturally apply.

This implementation was created out of curiosity to explore the characteristics and behavior of
the minimax algorithm when applied to this single-player optimization problem. Rather than having
alternating players trying to maximize and minimize outcomes, this solver always maximizes the
utility function to find the best solution (closest to the target).

The algorithm explores the game tree of possible number combinations and operations, using a
utility function that rewards exact matches and penalizes distance from the target. While not
a traditional minimax scenario, it scratches my itch as to tree search behavior and
algorithm performance compared to brute force approaches.

Key differences from traditional minimax:
- No adversarial players - always maximizes utility
- Single objective - find exact solution or best approximation
- Early termination when exact match found
- Depth-limited search to manage complexity
*/

use crate::equations::{Equation, OpType, Operation};

const EXACT_MATCH_UTILITY: i32 = 1000;

#[derive(Clone, Debug)]
struct GameState {
    numbers: Vec<i32>,
    equations: std::collections::HashMap<i32, Equation>,
}

pub struct MinimaxSolver {
    pub nodes_explored: u64,
    target: i32,
    max_depth: usize,
}

impl Default for MinimaxSolver {
    fn default() -> Self {
        Self::new()
    }
}

impl MinimaxSolver {
    pub fn new() -> Self {
        Self {
            nodes_explored: 0,
            target: 0,
            max_depth: 6, // Default depth limit
        }
    }

    pub fn with_depth(max_depth: usize) -> Self {
        Self {
            nodes_explored: 0,
            target: 0,
            max_depth,
        }
    }

    pub fn solve(&mut self, target: i32, numbers: &[i32]) -> Option<Equation> {
        self.nodes_explored = 0;
        self.target = target;

        // Check if target is directly in the numbers (early termination)
        for &num in numbers {
            if num == target {
                return Some(Equation::terminate(num));
            }
        }

        let mut initial_equations = std::collections::HashMap::new();
        for &num in numbers {
            initial_equations.insert(num, Equation::terminate(num));
        }

        let initial_state = GameState {
            numbers: numbers.to_vec(),
            equations: initial_equations,
        };

        let (utility, best_equation) = self.minimax(initial_state, self.max_depth);

        // Only return equation if we found exact match or a reasonable approximation
        if utility == EXACT_MATCH_UTILITY {
            // Exact match found
            best_equation
        } else if utility > -100 {
            // Found approximation within 100 of target
            best_equation
        } else {
            // No reasonable solution found for this implementation
            None
        }
    }

    fn minimax(&mut self, state: GameState, depth: usize) -> (i32, Option<Equation>) {
        self.nodes_explored += 1;

        // Terminal conditions
        if self.is_terminal(&state) || depth == 0 {
            let utility = self.utility(&state);
            let best_equation = if state.numbers.len() == 1 {
                state.equations.get(&state.numbers[0]).cloned()
            } else {
                None
            };
            return (utility, best_equation);
        }

        let actions = self.get_actions(&state);

        // Always maximize to find the best solution (closest to target)
        let mut max_eval = i32::MIN;
        let mut best_equation = None;

        for action in actions {
            let new_state = self.apply_action(&state, action);
            let (eval, eq) = self.minimax(new_state, depth - 1);

            if eval > max_eval {
                max_eval = eval;
                best_equation = eq;
            }
        }

        // If we found an exact match, return immediately
        if max_eval == EXACT_MATCH_UTILITY {
            return (max_eval, best_equation);
        }

        (max_eval, best_equation)
    }

    fn is_terminal(&self, state: &GameState) -> bool {
        // Terminal if only one number left
        if state.numbers.len() <= 1 {
            return true;
        }

        // Check if any equation equals target
        for &num in &state.numbers {
            if let Some(eq) = state.equations.get(&num)
                && let Ok(result) = eq.solve()
                && result == self.target
            {
                return true;
            }
        }

        false
    }

    fn utility(&self, state: &GameState) -> i32 {
        let mut best_utility = i32::MIN / 2;

        // Check all equations/numbers in the current state
        for &num in &state.numbers {
            if let Some(eq) = state.equations.get(&num)
                && let Ok(result) = eq.solve()
            {
                if result == self.target {
                    return EXACT_MATCH_UTILITY; // Exact match, return high reward
                }
                // Negative distance from target (closer is better)
                let utility = -(self.target - result).abs();
                best_utility = best_utility.max(utility);
            }
        }

        best_utility
    }

    // Generate all possible pairs and operations
    fn get_actions(&self, state: &GameState) -> Vec<Action> {
        let mut actions = Vec::new();

        if state.numbers.len() < 2 {
            return actions;
        }

        for i in 0..state.numbers.len() {
            for j in 0..state.numbers.len() {
                if i == j {
                    continue;
                }

                let a = state.numbers[i];
                let b = state.numbers[j];

                // Add all basic operations
                actions.push(Action {
                    a,
                    b,
                    op_type: OpType::Add,
                });
                actions.push(Action {
                    a,
                    b,
                    op_type: OpType::Subtract,
                });
                actions.push(Action {
                    a,
                    b,
                    op_type: OpType::Multiply,
                });

                // Only add division if it results in an integer
                if b != 0 && a % b == 0 {
                    actions.push(Action {
                        a,
                        b,
                        op_type: OpType::Divide,
                    });
                }
            }
        }

        actions
    }

    fn apply_action(&self, state: &GameState, action: Action) -> GameState {
        let result = match action.op_type {
            OpType::Add => action.a + action.b,
            OpType::Subtract => action.a - action.b,
            OpType::Multiply => action.a * action.b,
            OpType::Divide => action.a / action.b,
        };

        // Create new numbers list without the used numbers
        let mut new_numbers = Vec::new();
        let mut used_a = false;
        let mut used_b = false;

        for &num in &state.numbers {
            if num == action.a && !used_a {
                used_a = true;
                continue;
            }
            if num == action.b && !used_b {
                used_b = true;
                continue;
            }
            new_numbers.push(num);
        }

        // Add the result
        new_numbers.push(result);

        // Create new equations map, copying existing ones
        let mut new_equations = state.equations.clone();

        // Remove the used equations
        new_equations.remove(&action.a);
        new_equations.remove(&action.b);

        // Get the equations for a and b
        let eq_a = state
            .equations
            .get(&action.a)
            .cloned()
            .unwrap_or_else(|| Equation::terminate(action.a));
        let eq_b = state
            .equations
            .get(&action.b)
            .cloned()
            .unwrap_or_else(|| Equation::terminate(action.b));

        // For equation building, we need to construct: eq_a op eq_b
        // But the equation structure expects: number op equation
        // So we build it as: eq_a.solve() op eq_b (if eq_a is simple) or reconstruct properly

        let combined_equation = if matches!(eq_a.operation, Operation::Terminate) {
            // Simple case: a op eq_b
            let operation = match action.op_type {
                OpType::Add => Operation::add(eq_b),
                OpType::Subtract => Operation::subtract(eq_b),
                OpType::Multiply => Operation::multiply(eq_b),
                OpType::Divide => Operation::divide(eq_b),
            };
            Equation::new(eq_a.number, operation)
        } else {
            // Complex case: need to rebuild as (eq_a) op eq_b
            // This is tricky with current equation structure - for now use a simpler approach
            let operation = match action.op_type {
                OpType::Add => Operation::add(eq_b),
                OpType::Subtract => Operation::subtract(eq_b),
                OpType::Multiply => Operation::multiply(eq_b),
                OpType::Divide => Operation::divide(eq_b),
            };
            Equation::new(
                eq_a.number,
                match eq_a.operation {
                    Operation::Op(op_type, inner) => match op_type {
                        OpType::Add => Operation::add(Equation::new(inner.number, operation)),
                        OpType::Subtract => {
                            Operation::subtract(Equation::new(inner.number, operation))
                        }
                        OpType::Multiply => {
                            Operation::multiply(Equation::new(inner.number, operation))
                        }
                        OpType::Divide => Operation::divide(Equation::new(inner.number, operation)),
                    },
                    Operation::Terminate => operation,
                },
            )
        };

        // Add the new equation for the result
        new_equations.insert(result, combined_equation);

        GameState {
            numbers: new_numbers,
            equations: new_equations,
        }
    }
}

#[derive(Clone, Debug)]
struct Action {
    a: i32,
    b: i32,
    op_type: OpType,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition_solution_exists() {
        let mut solver = MinimaxSolver::new();
        let result = solver.solve(15, &[10, 5]);

        assert!(result.is_some());
        let equation = result.unwrap();
        let value = equation.solve().unwrap();
        println!("Target: 15, Found: {}", value);

        assert_eq!(value, 15);
        println!("Minimax nodes explored: {}", solver.nodes_explored);
    }

    #[test]
    fn test_subtraction_solution_exists() {
        let mut solver = MinimaxSolver::new();
        let result = solver.solve(5, &[10, 5]);

        assert!(result.is_some());
        let equation = result.unwrap();
        assert_eq!(equation.solve(), Ok(5));
        println!("Minimax nodes explored: {}", solver.nodes_explored);
    }

    #[test]
    fn test_multiplication_solution_exists() {
        let mut solver = MinimaxSolver::new();
        let result = solver.solve(50, &[10, 5]);

        assert!(result.is_some());
        let equation = result.unwrap();
        assert_eq!(equation.solve(), Ok(50));
        println!("Minimax nodes explored: {}", solver.nodes_explored);
    }

    #[test]
    fn test_division_solution_exists() {
        let mut solver = MinimaxSolver::new();
        let result = solver.solve(2, &[10, 5]);

        assert!(result.is_some());
        let equation = result.unwrap();
        assert_eq!(equation.solve(), Ok(2));
        println!("Minimax nodes explored: {}", solver.nodes_explored);
    }

    #[test]
    fn test_no_solution_target_3() {
        let mut solver = MinimaxSolver::new();
        let result = solver.solve(3, &[10, 5]);

        if let Some(equation) = result {
            let value = equation.solve().unwrap_or(0);
            assert_ne!(value, 3);
        }
        println!("Minimax nodes explored: {}", solver.nodes_explored);
    }

    #[test]
    fn test_no_solution_target_100() {
        let mut solver = MinimaxSolver::new();
        let result = solver.solve(100, &[5, 10]);

        if let Some(equation) = result {
            let value = equation.solve().unwrap_or(0);
            assert_ne!(value, 100);
        }
        println!("Minimax nodes explored: {}", solver.nodes_explored);
    }

    #[test]
    fn test_complex_solution_1() {
        let mut solver = MinimaxSolver::with_depth(8);
        let result = solver.solve(113, &[50, 25, 3, 1, 10, 7]);

        assert!(result.is_some());
        let equation = result.unwrap();
        let value = equation.solve().unwrap();
        assert_eq!(value, 113);
        println!("Exact solution found for 113: {}", value);
        println!("Minimax nodes explored: {}", solver.nodes_explored);
    }

    #[test]
    fn test_complex_solution_2() {
        let mut solver = MinimaxSolver::with_depth(8);
        let result = solver.solve(327, &[6, 7, 7, 1, 5, 8]);

        assert!(result.is_some());
        let equation = result.unwrap();
        let value = equation.solve().unwrap();
        assert_eq!(value, 327);
        println!("Exact solution found for 327: {}", value);
        println!("Minimax nodes explored: {}", solver.nodes_explored);
    }

    #[test]
    fn test_complex_solution_3() {
        let mut solver = MinimaxSolver::with_depth(8);
        let result = solver.solve(206, &[100, 3, 1, 4, 10, 10]);

        assert!(result.is_some());
        let equation = result.unwrap();
        let value = equation.solve().unwrap();
        assert_eq!(value, 206);
        println!("Exact solution found for 206: {}", value);
        println!("Minimax nodes explored: {}", solver.nodes_explored);
    }

    #[test]
    fn test_complex_solution_4() {
        let mut solver = MinimaxSolver::with_depth(8);
        let result = solver.solve(887, &[50, 25, 100, 3, 5, 8]);

        assert!(result.is_some());
        let equation = result.unwrap();
        let value = equation.solve().unwrap();
        assert_eq!(value, 887);
        println!("Minimax solution: {} = {}", equation.format(), value);
        println!("Minimax nodes explored: {}", solver.nodes_explored);
    }

    #[test]
    fn test_unsolvable_complex() {
        let mut solver = MinimaxSolver::with_depth(100);
        let result = solver.solve(999, &[1, 2, 3, 4, 5, 6]);

        println!(
            "Minimax nodes explored for target 999: {}",
            solver.nodes_explored
        );

        if let Some(equation) = result {
            let value = equation.solve().unwrap_or(0);
            println!(
                "Minimax found approximation: {} = {}",
                equation.format(),
                value
            );
            assert_ne!(value, 999);
            assert!(value > 0);
        }
    }
}

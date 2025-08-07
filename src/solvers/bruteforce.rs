/*!
Straightforward brute-force solver. Generates all permutations of the input number of lists and then
attempts each possible operation.
*/
use crate::equations::{Equation, OpType, Operation};

pub struct BruteForceSolver {
    pub permutation_count: u64,
}

impl Default for BruteForceSolver {
    fn default() -> Self {
        Self::new()
    }
}

impl BruteForceSolver {
    pub fn new() -> Self {
        Self {
            permutation_count: 0,
        }
    }

    pub fn solve(&mut self, target: i32, numbers: &[i32]) -> Option<Equation> {
        self.permutation_count = 0;

        let mut numbers_vec = numbers.to_vec();
        self.solve_recursive(target, &mut numbers_vec)
    }

    fn solve_recursive(&mut self, target: i32, numbers: &mut [i32]) -> Option<Equation> {
        // Use a hashmap to track which numbers map to which equations
        let mut equations = std::collections::HashMap::new();
        for &num in numbers.iter() {
            equations.insert(num, Equation::terminate(num));
        }

        self.try_all_combinations_with_equations(target, numbers.to_owned(), equations)
    }

    fn try_all_combinations_with_equations(
        &mut self,
        target: i32,
        numbers: Vec<i32>,
        equations: std::collections::HashMap<i32, Equation>,
    ) -> Option<Equation> {
        self.permutation_count += 1;

        // Base case: single number
        if numbers.len() == 1 {
            if numbers[0] == target {
                let equation = equations.get(&numbers[0]).cloned()?;
                // Double-check that the equation actually evaluates to the target
                if let Ok(result) = equation.solve()
                    && result == target
                {
                    return Some(equation);
                }
            }
            return None;
        }

        // Try combining every pair of numbers with every operation
        for i in 0..numbers.len() {
            for j in 0..numbers.len() {
                if i == j {
                    continue;
                }

                let a = numbers[i];
                let b = numbers[j];

                // Try all operations
                let operations_to_try = [
                    (OpType::Add, a + b),
                    (OpType::Subtract, a - b),
                    (OpType::Multiply, a * b),
                ];

                let mut all_ops = operations_to_try.to_vec();

                // Add division if valid
                if b != 0 && a % b == 0 {
                    all_ops.push((OpType::Divide, a / b));
                }

                for (op_type, result) in all_ops {
                    // Create new numbers array with the result replacing a and b
                    let mut new_numbers = Vec::new();
                    let mut used_i = false;
                    let mut used_j = false;

                    for (idx, &num) in numbers.iter().enumerate() {
                        if idx == i && !used_i {
                            used_i = true;
                            continue;
                        }
                        if idx == j && !used_j {
                            used_j = true;
                            continue;
                        }
                        new_numbers.push(num);
                    }
                    new_numbers.push(result);

                    // Create new equations map
                    let mut new_equations = equations.clone();
                    new_equations.remove(&a);
                    new_equations.remove(&b);

                    // Get equations for a and b
                    let eq_a = equations
                        .get(&a)
                        .cloned()
                        .unwrap_or_else(|| Equation::terminate(a));
                    let eq_b = equations
                        .get(&b)
                        .cloned()
                        .unwrap_or_else(|| Equation::terminate(b));

                    // Create combined equation: eq_a op eq_b
                    let operation = match op_type {
                        OpType::Add => Operation::add(eq_b),
                        OpType::Subtract => Operation::subtract(eq_b),
                        OpType::Multiply => Operation::multiply(eq_b),
                        OpType::Divide => Operation::divide(eq_b),
                    };

                    let combined_equation = Equation::new(eq_a.number, operation);

                    // Validate that the equation evaluates to the expected result
                    if let Ok(eq_result) = combined_equation.solve() {
                        if eq_result != result {
                            continue; // Skip this combination, equation doesn't match expected result
                        }
                    } else {
                        continue; // Skip invalid equations
                    }

                    new_equations.insert(result, combined_equation);

                    // Recursively solve with new numbers and equations
                    if let Some(solution) =
                        self.try_all_combinations_with_equations(target, new_numbers, new_equations)
                    {
                        return Some(solution);
                    }
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition_solution_exists() {
        let mut solver = BruteForceSolver::new();
        let result = solver.solve(15, &[10, 5]);

        assert!(result.is_some());
        let equation = result.unwrap();
        assert_eq!(equation.solve(), Ok(15));
    }

    #[test]
    fn test_subtraction_solution_exists() {
        let mut solver = BruteForceSolver::new();
        let result = solver.solve(5, &[10, 5]);

        assert!(result.is_some());
        let equation = result.unwrap();
        assert_eq!(equation.solve(), Ok(5));
    }

    #[test]
    fn test_multiplication_solution_exists() {
        let mut solver = BruteForceSolver::new();
        let result = solver.solve(50, &[10, 5]);

        assert!(result.is_some());
        let equation = result.unwrap();
        assert_eq!(equation.solve(), Ok(50));
    }

    #[test]
    fn test_division_solution_exists() {
        let mut solver = BruteForceSolver::new();
        let result = solver.solve(2, &[10, 5]);

        assert!(result.is_some());
        let equation = result.unwrap();
        assert_eq!(equation.solve(), Ok(2));
    }

    #[test]
    fn test_no_solution_target_3() {
        let mut solver = BruteForceSolver::new();
        let result = solver.solve(3, &[10, 5]);

        assert!(result.is_none());
    }

    #[test]
    fn test_no_solution_target_100() {
        let mut solver = BruteForceSolver::new();
        let result = solver.solve(100, &[5, 10]);

        assert!(result.is_none());
    }

    #[test]
    fn test_complex_solution_1() {
        let mut solver = BruteForceSolver::new();
        let result = solver.solve(113, &[50, 25, 3, 1, 10, 7]);

        assert!(result.is_some());
        let equation = result.unwrap();
        assert_eq!(equation.solve(), Ok(113));

        println!("Permutations tried: {}", solver.permutation_count);
    }

    #[test]
    fn test_complex_solution_2() {
        let mut solver = BruteForceSolver::new();
        let result = solver.solve(327, &[6, 7, 7, 1, 5, 8]);

        assert!(result.is_some());
        let equation = result.unwrap();
        assert_eq!(equation.solve(), Ok(327));

        println!("Permutations tried for 327: {}", solver.permutation_count);
    }

    #[test]
    fn test_complex_solution_3() {
        let mut solver = BruteForceSolver::new();
        let result = solver.solve(206, &[100, 3, 1, 4, 10, 10]);

        assert!(result.is_some());
        let equation = result.unwrap();
        assert_eq!(equation.solve(), Ok(206));

        println!("Permutations tried for 206: {}", solver.permutation_count);
    }

    #[test]
    fn test_complex_solution_4() {
        let mut solver = BruteForceSolver::new();
        let result = solver.solve(887, &[50, 25, 100, 3, 5, 8]);

        assert!(result.is_some());
        let equation = result.unwrap();
        assert_eq!(equation.solve(), Ok(887));

        println!("Permutations tried for 887: {}", solver.permutation_count);
        println!("Found equation: {}", equation.format());
    }

    #[test]
    fn test_unsolvable_complex() {
        let mut solver = BruteForceSolver::new();
        let result = solver.solve(999, &[1, 2, 3, 4, 5, 6]);

        assert!(result.is_none());
        println!(
            "Permutations tried for unsolvable 999: {}",
            solver.permutation_count
        );
    }
}

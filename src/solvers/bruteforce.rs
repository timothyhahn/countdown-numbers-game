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

    fn solve_recursive(&mut self, target: i32, numbers: &mut Vec<i32>) -> Option<Equation> {
        self.generate_permutations(numbers, 0, target)
    }

    fn generate_permutations(
        &mut self,
        numbers: &mut Vec<i32>,
        start: usize,
        target: i32,
    ) -> Option<Equation> {
        if start == numbers.len() {
            // Try this permutation with all possible operation combinations
            return self.try_all_operations(numbers, target);
        }

        // Try each number in the remaining positions
        for i in start..numbers.len() {
            numbers.swap(start, i);
            if let Some(result) = self.generate_permutations(numbers, start + 1, target) {
                return Some(result);
            }
            numbers.swap(start, i); // backtrack
        }

        None
    }

    fn try_all_operations(&mut self, numbers: &[i32], target: i32) -> Option<Equation> {
        self.permutation_count += 1;

        if numbers.len() == 1 {
            return if numbers[0] == target {
                Some(Equation::terminate(numbers[0]))
            } else {
                None
            };
        }

        let num_ops = numbers.len() - 1;
        let total_combinations = 4_usize.pow(num_ops as u32);

        for combo in 0..total_combinations {
            if let Some(equation) = self.build_equation_chain(numbers, combo, target) {
                return Some(equation);
            }
        }

        None
    }

    fn build_equation_chain(
        &mut self,
        numbers: &[i32],
        op_combo: usize,
        target: i32,
    ) -> Option<Equation> {
        if numbers.len() < 2 {
            return None;
        }

        let mut operations = Vec::new();
        let mut combo = op_combo;

        for _ in 0..(numbers.len() - 1) {
            let op = match combo % 4 {
                0 => OpType::Add,
                1 => OpType::Subtract,
                2 => OpType::Multiply,
                3 => OpType::Divide,
                _ => unreachable!(),
            };
            operations.push(op);
            combo /= 4;
        }

        let mut current_eq = Equation::terminate(numbers[numbers.len() - 1]);

        for i in (0..numbers.len() - 1).rev() {
            let op_type = operations[i];
            let left_val = numbers[i];

            if op_type == OpType::Divide {
                // For division, we need to check if it results in an integer
                if let Ok(current_val) = current_eq.solve() {
                    if current_val == 0 || left_val % current_val != 0 {
                        return None; // Invalid division
                    }
                } else {
                    return None; // Can't evaluate current equation
                }
            }

            let operation = match op_type {
                OpType::Add => Operation::add(current_eq),
                OpType::Subtract => Operation::subtract(current_eq),
                OpType::Multiply => Operation::multiply(current_eq),
                OpType::Divide => Operation::divide(current_eq),
            };

            current_eq = Equation::new(left_val, operation);
        }

        // Success case
        if let Ok(result) = current_eq.solve()
            && result == target
        {
            return Some(current_eq);
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

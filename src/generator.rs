/*!
Generates a new puzzle using the countdown logic (as I understand them)
*/
use rand::prelude::*;
use rand::{Rng, rng};

const LARGE_NUMBERS: &[i32] = &[25, 50, 75, 100];
const SMALL_NUMBERS: &[i32] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

#[derive(Debug, Clone)]
pub struct Puzzle {
    pub numbers: Vec<i32>,
    pub target: i32,
    pub large_count: usize,
    pub max_numbers: usize,
}

pub struct PuzzleGenerator {
    rng: ThreadRng,
}

impl Default for PuzzleGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl PuzzleGenerator {
    pub fn new() -> Self {
        Self { rng: rng() }
    }

    pub fn generate(&mut self, large_count: usize, max_numbers: usize) -> Puzzle {
        if large_count > max_numbers {
            panic!("Cannot have more than {} large numbers", max_numbers);
        }

        let mut numbers = Vec::new();

        // Add large numbers
        let mut large_pool = LARGE_NUMBERS.to_vec();
        for _ in 0..large_count {
            if let Some(idx) = (0..large_pool.len()).choose(&mut self.rng) {
                numbers.push(large_pool[idx]);
                large_pool.remove(idx);
            }
        }

        // Add small numbers
        let small_count = max_numbers - large_count;
        for _ in 0..small_count {
            if let Some(&number) = SMALL_NUMBERS.choose(&mut self.rng) {
                numbers.push(number);
            }
        }

        // Generate target number (101-999)
        let target = self.rng.random_range(101..=999);

        Puzzle {
            numbers,
            target,
            large_count,
            max_numbers,
        }
    }

    pub fn generate_classic(&mut self) -> Puzzle {
        let large_count = self.rng.random_range(1..=4);
        self.generate(large_count, 6)
    }

    pub fn generate_with_target(
        &mut self,
        target: i32,
        large_count: usize,
        max_numbers: usize,
    ) -> Puzzle {
        if !(101..=999).contains(&target) {
            panic!("Target must be between 101 and 999");
        }

        if large_count > max_numbers {
            panic!("Cannot have more than {} large numbers", max_numbers);
        }

        let mut numbers = Vec::new();

        // Add large numbers
        let mut large_pool = LARGE_NUMBERS.to_vec();
        for _ in 0..large_count {
            if let Some(idx) = (0..large_pool.len()).choose(&mut self.rng) {
                numbers.push(large_pool[idx]);
                large_pool.remove(idx);
            }
        }

        // Add small numbers
        let small_count = max_numbers - large_count;
        for _ in 0..small_count {
            if let Some(&number) = SMALL_NUMBERS.choose(&mut self.rng) {
                numbers.push(number);
            }
        }

        Puzzle {
            numbers,
            target,
            large_count,
            max_numbers,
        }
    }
}

impl Puzzle {
    pub fn new(numbers: Vec<i32>, target: i32) -> Self {
        let large_count = numbers
            .iter()
            .filter(|&&n| LARGE_NUMBERS.contains(&n))
            .count();
        let max_numbers = numbers.len();
        Self {
            numbers,
            target,
            large_count,
            max_numbers,
        }
    }

    pub fn small_count(&self) -> usize {
        self.max_numbers - self.large_count
    }

    pub fn is_valid(&self) -> bool {
        if self.numbers.len() != self.max_numbers {
            return false;
        }
        if self.target < 101 || self.target > 999 {
            return false;
        }
        if self.large_count > 4 {
            return false;
        }

        let large_numbers: Vec<i32> = self
            .numbers
            .iter()
            .filter(|&&n| LARGE_NUMBERS.contains(&n))
            .copied()
            .collect();

        let small_numbers: Vec<i32> = self
            .numbers
            .iter()
            .filter(|&&n| SMALL_NUMBERS.contains(&n))
            .copied()
            .collect();

        large_numbers.len() == self.large_count
            && small_numbers.len() == self.small_count()
            && large_numbers.len() + small_numbers.len() == self.max_numbers
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_puzzle() {
        let mut generator = PuzzleGenerator::new();
        let puzzle = generator.generate(2, 6);

        assert_eq!(puzzle.numbers.len(), 6);
        assert_eq!(puzzle.max_numbers, 6);
        assert_eq!(puzzle.large_count, 2);
        assert_eq!(puzzle.small_count(), 4);
        assert!(puzzle.target >= 101 && puzzle.target <= 999);
        assert!(puzzle.is_valid());
    }

    #[test]
    fn test_generate_classic_puzzle() {
        let mut generator = PuzzleGenerator::new();
        let puzzle = generator.generate_classic();

        assert_eq!(puzzle.numbers.len(), 6);
        assert_eq!(puzzle.max_numbers, 6);
        assert!(puzzle.large_count >= 1 && puzzle.large_count <= 4);
        assert!(puzzle.target >= 101 && puzzle.target <= 999);
        assert!(puzzle.is_valid());
    }

    #[test]
    fn test_generate_with_target() {
        let mut generator = PuzzleGenerator::new();
        let puzzle = generator.generate_with_target(327, 3, 6);

        assert_eq!(puzzle.numbers.len(), 6);
        assert_eq!(puzzle.max_numbers, 6);
        assert_eq!(puzzle.target, 327);
        assert_eq!(puzzle.large_count, 3);
        assert_eq!(puzzle.small_count(), 3);
        assert!(puzzle.is_valid());
    }

    #[test]
    fn test_puzzle_validation() {
        let valid_puzzle = Puzzle::new(vec![25, 50, 1, 2, 3, 4], 327);
        assert!(valid_puzzle.is_valid());

        let invalid_target = Puzzle::new(vec![25, 50, 1, 2, 3, 4], 50);
        assert!(!invalid_target.is_valid());

        let too_few_numbers = Puzzle::new(vec![25, 50, 1, 2, 3], 327);
        assert!(too_few_numbers.is_valid()); // Now valid with max_numbers = 5
    }

    #[test]
    fn test_large_and_small_counts() {
        let puzzle = Puzzle::new(vec![25, 50, 75, 1, 2, 3], 456);
        assert_eq!(puzzle.large_count, 3);
        assert_eq!(puzzle.small_count(), 3);
        assert_eq!(puzzle.max_numbers, 6);
    }

    #[test]
    #[should_panic(expected = "Cannot have more than 6 large numbers")]
    fn test_too_many_large_numbers() {
        let mut generator = PuzzleGenerator::new();
        generator.generate(7, 6);
    }

    #[test]
    #[should_panic(expected = "Target must be between 101 and 999")]
    fn test_invalid_target_range() {
        let mut generator = PuzzleGenerator::new();
        generator.generate_with_target(50, 2, 6);
    }

    #[test]
    fn test_all_large_numbers() {
        let mut generator = PuzzleGenerator::new();
        let puzzle = generator.generate(4, 6);

        assert_eq!(puzzle.large_count, 4);
        assert_eq!(puzzle.small_count(), 2);
        assert_eq!(puzzle.max_numbers, 6);

        let large_count_actual = puzzle
            .numbers
            .iter()
            .filter(|&&n| LARGE_NUMBERS.contains(&n))
            .count();
        assert_eq!(large_count_actual, 4);
    }

    #[test]
    fn test_no_large_numbers() {
        let mut generator = PuzzleGenerator::new();
        let puzzle = generator.generate(0, 6);

        assert_eq!(puzzle.large_count, 0);
        assert_eq!(puzzle.small_count(), 6);
        assert_eq!(puzzle.max_numbers, 6);

        let large_count_actual = puzzle
            .numbers
            .iter()
            .filter(|&&n| LARGE_NUMBERS.contains(&n))
            .count();
        assert_eq!(large_count_actual, 0);
    }

    #[test]
    fn test_custom_max_numbers() {
        let mut generator = PuzzleGenerator::new();
        let puzzle = generator.generate(2, 8);

        assert_eq!(puzzle.numbers.len(), 8);
        assert_eq!(puzzle.max_numbers, 8);
        assert_eq!(puzzle.large_count, 2);
        assert_eq!(puzzle.small_count(), 6);
        assert!(puzzle.target >= 101 && puzzle.target <= 999);
    }

    #[test]
    fn test_small_puzzle() {
        let mut generator = PuzzleGenerator::new();
        let puzzle = generator.generate(1, 3);

        assert_eq!(puzzle.numbers.len(), 3);
        assert_eq!(puzzle.max_numbers, 3);
        assert_eq!(puzzle.large_count, 1);
        assert_eq!(puzzle.small_count(), 2);
        assert!(puzzle.target >= 101 && puzzle.target <= 999);
    }
}

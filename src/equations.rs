/*!
# Equations
Helper struct that implements the problem as a linked list and collapses the problem as needed.
*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OpType {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operation {
    Op(OpType, Box<Equation>),
    Terminate,
}

impl Operation {
    pub fn add(next: Equation) -> Self {
        Operation::Op(OpType::Add, Box::new(next))
    }

    pub fn subtract(next: Equation) -> Self {
        Operation::Op(OpType::Subtract, Box::new(next))
    }

    pub fn multiply(next: Equation) -> Self {
        Operation::Op(OpType::Multiply, Box::new(next))
    }

    pub fn divide(next: Equation) -> Self {
        Operation::Op(OpType::Divide, Box::new(next))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Equation {
    pub number: i32,
    pub operation: Operation,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SolverError {
    NonIntegerResult,
    DivisionByZero,
}

impl std::fmt::Display for SolverError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SolverError::NonIntegerResult => write!(f, "Operation resulted in non-integer value"),
            SolverError::DivisionByZero => write!(f, "Division by zero attempted"),
        }
    }
}

impl std::error::Error for SolverError {}

impl Equation {
    pub fn new(number: i32, operation: Operation) -> Self {
        Self { number, operation }
    }

    pub fn terminate(number: i32) -> Self {
        Self {
            number,
            operation: Operation::Terminate,
        }
    }

    pub fn solve(&self) -> Result<i32, SolverError> {
        match &self.operation {
            Operation::Terminate => Ok(self.number),
            Operation::Op(op_type, next) => {
                let next_result = next.solve()?;
                match op_type {
                    OpType::Add => Ok(self.number + next_result),
                    OpType::Subtract => Ok(self.number - next_result),
                    OpType::Multiply => Ok(self.number * next_result),
                    OpType::Divide => {
                        if next_result == 0 {
                            return Err(SolverError::DivisionByZero);
                        }
                        if self.number % next_result != 0 {
                            return Err(SolverError::NonIntegerResult);
                        }
                        Ok(self.number / next_result)
                    }
                }
            }
        }
    }

    pub fn format(&self) -> String {
        match &self.operation {
            Operation::Terminate => self.number.to_string(),
            Operation::Op(op_type, next) => {
                let op_str = match op_type {
                    OpType::Add => "+",
                    OpType::Subtract => "-",
                    OpType::Multiply => "*",
                    OpType::Divide => "/",
                };
                format!("({} {} {})", self.number, op_str, next.format())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_terminate() {
        let eq = Equation::terminate(42);
        assert_eq!(eq.solve(), Ok(42));
    }

    #[test]
    fn test_addition() {
        let eq = Equation::new(10, Operation::add(Equation::terminate(5)));
        assert_eq!(eq.solve(), Ok(15));
    }

    #[test]
    fn test_subtraction() {
        let eq = Equation::new(10, Operation::subtract(Equation::terminate(3)));
        assert_eq!(eq.solve(), Ok(7));
    }

    #[test]
    fn test_multiplication() {
        let eq = Equation::new(6, Operation::multiply(Equation::terminate(7)));
        assert_eq!(eq.solve(), Ok(42));
    }

    #[test]
    fn test_division() {
        let eq = Equation::new(20, Operation::divide(Equation::terminate(4)));
        assert_eq!(eq.solve(), Ok(5));
    }

    #[test]
    fn test_division_by_zero() {
        let eq = Equation::new(10, Operation::divide(Equation::terminate(0)));
        assert_eq!(eq.solve(), Err(SolverError::DivisionByZero));
    }

    #[test]
    fn test_non_integer_division() {
        let eq = Equation::new(10, Operation::divide(Equation::terminate(3)));
        assert_eq!(eq.solve(), Err(SolverError::NonIntegerResult));
    }

    #[test]
    fn test_complex_equation() {
        let eq = Equation::new(
            10,
            Operation::add(Equation::new(
                5,
                Operation::multiply(Equation::terminate(2)),
            )),
        );
        assert_eq!(eq.solve(), Ok(20));
    }

    #[test]
    fn test_chained_operations() {
        let eq = Equation::new(
            100,
            Operation::subtract(Equation::new(
                25,
                Operation::add(Equation::new(50, Operation::divide(Equation::terminate(2)))),
            )),
        );
        assert_eq!(eq.solve(), Ok(50));
    }

    #[test]
    fn test_error_propagation() {
        let eq = Equation::new(
            10,
            Operation::add(Equation::new(15, Operation::divide(Equation::terminate(0)))),
        );
        assert_eq!(eq.solve(), Err(SolverError::DivisionByZero));
    }

}

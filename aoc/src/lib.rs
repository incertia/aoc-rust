mod error;
pub mod macros;
#[cfg(test)]
mod miri;
mod solver;
mod year;

use clap::Parser;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Solution {
  Number(i64),
  Text(String),
}

impl std::fmt::Display for Solution {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Solution::Number(i) => write!(f, "{}", i),
      Solution::Text(s) => write!(f, "{}", s),
    }
  }
}

pub trait AocParser {
  fn parse(input: &[u8]) -> Self;
}

pub use error::AdventRuntimeError;
pub use solver::AdventSolver;
pub use year::AdventYearSolver;

#[derive(Parser)]
struct AocArgs {
  day: Option<i64>,
}

pub fn aoc_main(solver: &AdventYearSolver) -> Result<(), AdventRuntimeError> {
  let args = AocArgs::parse();
  let day = args.day.unwrap_or(1);
  solver.run(day)
}

#[cfg(test)]
mod test {
  use super::{AdventSolver, Solution, miri::TEST_STRING};
  type Input = Vec<u8>;

  fn solve_a(i: &Vec<u8>) -> Solution {
    Solution::Number(i.len() as i64)
  }
  fn solve_b(i: &Vec<u8>) -> Solution {
    Solution::Number(i.len() as i64)
  }

  #[test]
  fn test_erase() {
    let solver = AdventSolver::new::<Input>(1, Some(solve_a), Some(solve_b));
    solver.run(TEST_STRING);
  }
}
